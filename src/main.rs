use rgb::ComponentBytes;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};
use structopt::{
    clap::{arg_enum, AppSettings},
    StructOpt,
};

mod point;
use self::point::Point;

mod cmap;
use self::cmap::{read_cmap_file, ColourMap, Pixel};

use rayon::prelude::*;

type RenderFP = f64;

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    // optionally configure rayon
    if let Some(threads) = opt.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .unwrap();
    }

    if opt.verbose {
        println!("{:#?}", opt);
    }

    let cmap: ColourMap = read_cmap_file(&opt.mapfile)?;

    let (bottom_left, top_right) = calc_boundaries(&opt.image_centre, opt.xlen_real, opt.ratio);

    let render_info = &RenderInfo {
        width: opt.width,
        height: (opt.width as RenderFP * opt.ratio) as u32,
        color_info: ColorInfo {
            smooth: opt.smooth,
            palette: &cmap,
        },
        escape_limit: opt.iterations,
        fractal_type: opt.fractal_type,
        julia_centre: opt.julia_centre,
        top_right,
        bottom_left,
    };

    let path = Path::new(&opt.outfile);
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, render_info.width, render_info.height);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut stream = writer.stream_writer();

    // test row iter
    let rows = (0..render_info.height)
        .into_par_iter()
        .map(|i| iter_row(render_info, i).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for row in rows {
        stream.write(row.as_bytes())?;
    }

    stream.finish()?;

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Fractalgen",
    about = "A rust clone of my C fractal generator.",
    version = "0.0.1",
    author = "Sam L. (@_tritoke)",
    setting = AppSettings::AllowLeadingHyphen,
    rename_all = "snake"
)]
struct Opt {
    /// colourmap file to read colours from
    #[structopt(short, long, parse(from_os_str))]
    mapfile: PathBuf,

    /// file to save the rendered image to
    #[structopt(short, long, parse(from_os_str), default_value = "render.png")]
    outfile: PathBuf,

    /// width of the rendered image in pixels
    #[structopt(short, long, default_value = "1000")]
    width: u32,

    /// the "escape limit" before a pixel is declared part of the set
    #[structopt(short, long, default_value = "1000")]
    iterations: usize,

    /// use an algorithm to smooth between colors and reduce banding
    #[structopt(short, long)]
    smooth: bool,

    /// print out verbose debug info
    #[structopt(short, long)]
    verbose: bool,

    /// The type of fractal to render
    #[structopt(short, long, default_value = "mandelbrot")]
    fractal_type: FractalType,

    /// number of threads to use when rendering
    #[structopt(short, long)]
    threads: Option<usize>,

    /// the ratio between the width and height of the image
    #[structopt(short, long, default_value = "1")]
    ratio: RenderFP,

    /// the length along the real "x" axis that the image width represents
    #[structopt(short, long, default_value = "4")]
    xlen_real: RenderFP,

    /// The coordinates of the centre of the image in the complex plane
    #[structopt(long, default_value = "0,0")]
    image_centre: Point<RenderFP>,

    /// c,d - the parameters of the julia set to be rendered
    #[structopt(long, default_value = "-0.8,0.156")]
    julia_centre: Point<RenderFP>,
}

arg_enum! {
    #[derive(Debug)]
    enum FractalType {
        Mandelbrot,
        Julia,
        BurningShip
    }
}

fn calc_boundaries(
    image_centre: &Point<RenderFP>,
    xlen_real: RenderFP,
    ratio: RenderFP,
) -> (Point<RenderFP>, Point<RenderFP>) {
    let ylen_real = xlen_real * ratio;

    let bottom_left = Point {
        x: image_centre.x - (xlen_real / 2.0),
        y: image_centre.y - (ylen_real / 2.0),
    };

    let top_right = Point {
        x: image_centre.x + (xlen_real / 2.0),
        y: image_centre.y + (ylen_real / 2.0),
    };

    (bottom_left, top_right)
}

struct RenderInfo<'a> {
    width: u32,
    height: u32,
    escape_limit: usize,
    color_info: ColorInfo<'a>,
    fractal_type: FractalType,
    julia_centre: Point<RenderFP>,
    // coordinates of render boundaries
    top_right: Point<RenderFP>,
    bottom_left: Point<RenderFP>,
}

struct ColorInfo<'a> {
    smooth: bool,
    palette: &'a Vec<Pixel>,
}

fn iter_row<'a>(render_info: &'a RenderInfo, row: u32) -> RowIter<'a> {
    // precalculate y_val for whole row
    let y_val = distribute(
        row,
        render_info.height,
        render_info.top_right.y,
        render_info.bottom_left.y,
    );

    RowIter {
        render_info,
        y_val,
        col: 0,
    }
}

struct RowIter<'a> {
    render_info: &'a RenderInfo<'a>,
    y_val: RenderFP,
    col: u32,
}

impl<'a> Iterator for RowIter<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.col < self.render_info.width {
            let x = distribute(
                self.col,
                self.render_info.width,
                self.render_info.bottom_left.x,
                self.render_info.top_right.x,
            );

            self.col += 1;

            Some(match self.render_info.fractal_type {
                FractalType::Mandelbrot => mandelbrot(
                    Point { x, y: self.y_val },
                    self.render_info.escape_limit,
                    &self.render_info.color_info,
                ),
                FractalType::Julia => julia(
                    Point { x, y: self.y_val },
                    &self.render_info.julia_centre,
                    self.render_info.escape_limit,
                    &self.render_info.color_info,
                ),
                FractalType::BurningShip => burning_ship(
                    Point {
                        x,
                        // invert y value so fractal is upright
                        y: reverse(
                            self.y_val,
                            self.render_info.top_right.y,
                            self.render_info.bottom_left.y,
                        ),
                    },
                    self.render_info.escape_limit,
                    &self.render_info.color_info,
                ),
            })
        } else {
            None
        }
    }
}

/// takes a number in 0..n and maps it onto the range [a, b]
fn distribute(i: u32, n: u32, a: RenderFP, b: RenderFP) -> RenderFP {
    a + (i as RenderFP / (n as RenderFP / (b - a)))
}

/// takes a number in [a, b] and maps it onto the range [b, a]
fn reverse(x: RenderFP, a: RenderFP, b: RenderFP) -> RenderFP {
    b - (x - a)
}

fn mandelbrot(start: Point<RenderFP>, escape_limit: usize, color_info: &ColorInfo) -> Pixel {
    let black = Pixel {
        r: 0x00,
        g: 0x00,
        b: 0x00,
    };

    let Point { x: c, y: d } = start;
    let mut x = c;
    let mut y = d;
    let mut x2 = c.powi(2);
    let mut y2 = d.powi(2);

    // central bulb optimisation
    let cdot = x2 + y2;
    let not_in_main_bulb = (256.0 * cdot * cdot - 96.0 * cdot + 32.0 * x - 3.0 >= 0.0)
        && (16.0 * (cdot + 2.0 * x + 1.0) - 1.0 >= 0.0);

    let mut iterations = if not_in_main_bulb { 0 } else { escape_limit };
    while iterations < escape_limit && x2 + y2 < 4.0 {
        iterations += 1;
        y = 2.0 * x * y + d;
        x = x2 - y2 + c;
        x2 = x.powi(2);
        y2 = y.powi(2);
    }

    if iterations >= escape_limit - 1 {
        black
    } else if color_info.smooth {
        // get an estimate for true iteration count
        for _ in 0..3 {
            iterations += 1;
            y = 2.0 * x * y + d;
            x = x2 - y2 + c;
            x2 = x.powi(2);
            y2 = y.powi(2);
        }

        let mu = RenderFP::max(
            0.0,
            iterations as RenderFP + 1.0 - (x2 + y2).sqrt().ln().ln() / RenderFP::ln(2.0),
        );

        smooth(mu, color_info)
    } else {
        color_info.palette[iterations % color_info.palette.len()]
    }
}

fn julia(
    start: Point<RenderFP>,
    centre: &Point<RenderFP>,
    escape_limit: usize,
    color_info: &ColorInfo,
) -> Pixel {
    let black = Pixel {
        r: 0x00,
        g: 0x00,
        b: 0x00,
    };

    let Point { mut x, mut y } = start;
    let mut x2 = x.powi(2);
    let mut y2 = y.powi(2);

    let Point { x: c, y: d } = centre;

    let mut iterations = 0;
    while iterations < escape_limit && x2 + y2 < 4.0 {
        iterations += 1;
        y = 2.0 * x * y + d;
        x = x2 - y2 + c;
        x2 = x.powi(2);
        y2 = y.powi(2);
    }

    if iterations >= escape_limit - 1 {
        black
    } else if color_info.smooth {
        // get an estimate for true iteration count
        for _ in 0..3 {
            iterations += 1;
            y = 2.0 * x * y + d;
            x = x2 - y2 + c;
            x2 = x.powi(2);
            y2 = y.powi(2);
        }

        let mu = RenderFP::max(
            0.0,
            iterations as RenderFP + 1.0 - (x2 + y2).sqrt().ln().ln() / RenderFP::ln(2.0),
        );

        smooth(mu, color_info)
    } else {
        *color_info.palette.iter().cycle().nth(iterations).unwrap()
    }
}

fn burning_ship(start: Point<RenderFP>, escape_limit: usize, color_info: &ColorInfo) -> Pixel {
    let black = Pixel {
        r: 0x00,
        g: 0x00,
        b: 0x00,
    };

    let Point { x: c, y: d } = start;
    let mut x = c;
    let mut y = d;
    let mut x2 = x.powi(2);
    let mut y2 = y.powi(2);

    let mut iterations = 0;
    while iterations < escape_limit && x2 + y2 < 4.0 {
        iterations += 1;
        y = (2.0 * x * y).abs() + d;
        x = x2 - y2 + c;
        x2 = x.powi(2);
        y2 = y.powi(2);
    }

    if iterations >= escape_limit - 1 {
        black
    } else if color_info.smooth {
        // get an estimate for true iteration count
        for _ in 0..3 {
            iterations += 1;
            y = (2.0 * x * y).abs() + d;
            x = x2 - y2 + c;
            x2 = x.powi(2);
            y2 = y.powi(2);
        }

        let mu = RenderFP::max(
            0.0,
            iterations as RenderFP + 1.0 - (x2 + y2).sqrt().ln().ln() / RenderFP::ln(2.0),
        );

        smooth(mu, color_info)
    } else {
        *color_info.palette.iter().cycle().nth(iterations).unwrap()
    }
}

fn smooth(mu: RenderFP, color_info: &ColorInfo) -> Pixel {
    let estimate = mu as usize;

    /* interpolate between colours */
    let c1 = color_info.palette[estimate % color_info.palette.len()];
    let c2 = color_info.palette[(estimate + 1) % color_info.palette.len()];

    let t2 = mu - estimate as RenderFP;
    let t1 = 1.0 - t2;

    Pixel {
        r: ((c1.r as RenderFP * t1) + (c2.r as RenderFP * t2)) as u8,
        g: ((c1.g as RenderFP * t1) + (c2.g as RenderFP * t2)) as u8,
        b: ((c1.b as RenderFP * t1) + (c2.b as RenderFP * t2)) as u8,
    }
}
