use std::str::FromStr;

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: FromStr> FromStr for Point<T> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim_matches(|c| c == '(' || c == ')')
            .split(",")
            .collect();

        let x_fromstr = coords[0].parse::<T>()?;
        let y_fromstr = coords[1].parse::<T>()?;

        Ok(Self {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}
