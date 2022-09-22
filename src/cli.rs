use crate::point::Point;
use crate::RenderFP;
use clap::{arg_enum, AppSettings, Parser};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(
name = "Fractalgen",
about = "A rust clone of my C fractal generator.",
version = "0.1.0",
author = "Sam L. (@_tritoke)",
setting = AppSettings::AllowLeadingHyphen,
rename_all = "snake"
)]
pub struct Args {
    /// colourmap file to read colours from
    #[clap(short, long, parse(from_os_str))]
    pub mapfile: PathBuf,

    /// file to save the rendered image to
    #[clap(short, long, parse(from_os_str), default_value = "render.png")]
    pub outfile: PathBuf,

    /// width of the rendered image in pixels
    #[clap(short, long, default_value = "1000")]
    pub width: u32,

    /// the "escape limit" before a pixel is declared part of the set
    #[clap(short, long, default_value = "1000")]
    pub iterations: usize,

    /// use an algorithm to smooth between colors and reduce banding
    #[clap(short, long)]
    pub smooth: bool,

    /// print out verbose debug info
    #[clap(short, long)]
    pub verbose: bool,

    /// The type of fractal to render
    #[clap(short, long, default_value = "mandelbrot")]
    pub fractal_type: FractalType,

    /// number of threads to use when rendering
    #[clap(short, long)]
    pub threads: Option<usize>,

    /// the ratio between the width and height of the image
    #[clap(short, long, default_value = "1")]
    pub ratio: RenderFP,

    /// the length along the real "x" axis that the image width represents
    #[clap(short, long, default_value = "4")]
    pub xlen_real: RenderFP,

    /// The coordinates of the centre of the image in the complex plane
    #[clap(long, default_value = "0,0")]
    pub image_centre: Point<RenderFP, RenderFP>,

    /// c,d - the parameters of the julia set to be rendered
    #[clap(long, default_value = "-0.8,0.156")]
    pub julia_centre: Point<RenderFP, RenderFP>,
}

arg_enum! {
    #[derive(Debug)]
    pub enum FractalType {
        Mandelbrot,
        Julia,
        BurningShip
    }
}
