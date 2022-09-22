use crate::cmap::Pixel;
use crate::point::Point;
use crate::RenderFP;

pub fn calc_boundaries(
    image_centre: Point<RenderFP, RenderFP>,
    xlen_real: RenderFP,
    ratio: RenderFP,
) -> (Point<RenderFP, RenderFP>, Point<RenderFP, RenderFP>) {
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

/// takes a number in 0..n and maps it onto the range [a, b]
pub fn distribute(i: u32, n: u32, a: RenderFP, b: RenderFP) -> RenderFP {
    a + (i as RenderFP / (n as RenderFP / (b - a)))
}

/// takes a number in [a, b] and maps it onto the range [b, a]
pub fn reverse(x: RenderFP, a: RenderFP, b: RenderFP) -> RenderFP {
    b - (x - a)
}

/// linear interpolation
#[inline(always)]
fn lerp(c1: u8, c2: u8, f: f64) -> u8 {
    ((c1 as RenderFP) * (1.0 - f) + (c2 as RenderFP) * f) as u8
}

/// LERP between two colours based on an estimate of how far between two pixels a value is
pub fn smooth(mu: RenderFP, palette: &[Pixel]) -> Pixel {
    let estimate = mu as usize;

    let c1 = palette[estimate % palette.len()];
    let c2 = palette[(estimate + 1) % palette.len()];

    let f = mu.fract().abs();
    Pixel {
        r: lerp(c1.r, c2.r, f),
        g: lerp(c1.g, c2.g, f),
        b: lerp(c1.b, c2.b, f),
    }
}
