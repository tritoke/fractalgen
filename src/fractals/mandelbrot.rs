use crate::cmap::Pixel;
use crate::point::Point;
use crate::util::smooth;
use crate::RenderFP;

pub fn mandelbrot(
    start: Point<RenderFP, RenderFP>,
    escape_limit: usize,
    smooth_colours: bool,
    palette: &[Pixel],
) -> Pixel {
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
    } else if smooth_colours {
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

        smooth(mu, palette)
    } else {
        palette[iterations % palette.len()]
    }
}
