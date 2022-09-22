use anyhow::anyhow;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub struct Point<X, Y> {
    pub x: X,
    pub y: Y,
}

impl<X, Y> From<(X, Y)> for Point<X, Y> {
    fn from(tup: (X, Y)) -> Self {
        let (x, y) = tup;
        Self { x, y }
    }
}

#[allow(unused)]
impl<X, Y> Point<X, Y> {
    pub fn new(x: X, y: Y) -> Self {
        Self { x, y }
    }

    pub fn map_x(self, f: impl FnOnce(X) -> X) -> Self {
        let Point { x, y } = self;
        Self { x: f(x), y }
    }

    pub fn map_y(self, f: impl FnOnce(Y) -> Y) -> Self {
        let Point { x, y } = self;
        Self { x, y: f(y) }
    }

    pub fn transform(self, f: impl FnOnce(X, Y) -> (X, Y)) -> Self {
        let Point { x, y } = self;
        let (x, y) = f(x, y);
        Self { x, y }
    }
}

impl<X, Y> FromStr for Point<X, Y>
where
    X: FromStr,
    <X as FromStr>::Err: std::fmt::Debug,
    Y: FromStr,
    <Y as FromStr>::Err: std::fmt::Debug,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .collect();

        let x_fromstr = coords[0]
            .parse()
            .map_err(|e| anyhow!("Error parsing X: {e:?}"))?;
        let y_fromstr = coords[1]
            .parse()
            .map_err(|e| anyhow!("Error parsing Y: {e:?}"))?;

        Ok(Self {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
