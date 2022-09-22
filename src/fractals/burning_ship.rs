use crate::cmap::Pixel;
use crate::point::Point;
use crate::util::smooth;
use crate::RenderFP;

pub fn burning_ship(
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
    } else if smooth_colours {
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

        smooth(mu, palette)
    } else {
        *palette.iter().cycle().nth(iterations).unwrap()
    }
}
