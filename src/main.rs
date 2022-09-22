use clap::Parser;
use rayon::prelude::*;
use rgb::ComponentBytes;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use fractalgen::cli::{Args, FractalType};
use fractalgen::cmap::{self, ColourMap, Pixel};
use fractalgen::fractals::{burning_ship, julia, mandelbrot};
use fractalgen::point::Point;
use fractalgen::util::{calc_boundaries, distribute, reverse};
use fractalgen::RenderFP;

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    // optionally configure rayon
    if let Some(threads) = args.threads {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .unwrap();
    }

    if args.verbose {
        println!("{:#?}", args);
    }

    let cmap: ColourMap = cmap::read_cmap_file(&args.mapfile)?;

    let (bottom_left, top_right) = calc_boundaries(args.image_centre, args.xlen_real, args.ratio);

    let render_info = RenderInfo {
        width: args.width,
        height: (args.width as RenderFP * args.ratio) as u32,
        smooth_colours: args.smooth,
        palette: cmap.as_slice(),
        escape_limit: args.iterations,
        fractal_type: args.fractal_type,
        julia_centre: args.julia_centre,
        top_right,
        bottom_left,
    };

    let path = Path::new(&args.outfile);
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
        .map(|i| iter_row(&render_info, i).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    for row in rows {
        stream.write_all(row.as_bytes())?;
    }

    stream.finish()?;

    Ok(())
}

struct RenderInfo<'a> {
    width: u32,
    height: u32,
    escape_limit: usize,
    smooth_colours: bool,
    palette: &'a [Pixel],
    fractal_type: FractalType,
    julia_centre: Point<RenderFP, RenderFP>,
    top_right: Point<RenderFP, RenderFP>,
    bottom_left: Point<RenderFP, RenderFP>,
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
                    Point::new(x, self.y_val),
                    self.render_info.escape_limit,
                    self.render_info.smooth_colours,
                    self.render_info.palette,
                ),
                FractalType::Julia => julia(
                    Point::new(x, self.y_val),
                    self.render_info.julia_centre,
                    self.render_info.escape_limit,
                    self.render_info.smooth_colours,
                    self.render_info.palette,
                ),
                FractalType::BurningShip => burning_ship(
                    Point::new(
                        x,
                        reverse(
                            self.y_val,
                            self.render_info.top_right.y,
                            self.render_info.bottom_left.y,
                        ),
                    ),
                    self.render_info.escape_limit,
                    self.render_info.smooth_colours,
                    self.render_info.palette,
                ),
            })
        } else {
            None
        }
    }
}
