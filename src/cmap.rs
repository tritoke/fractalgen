use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub type Pixel = rgb::RGB<u8>;
pub type ColourMap = Vec<Pixel>;

pub fn read_cmap_file(map_file: &Path) -> std::io::Result<ColourMap> {
    let file = File::open(map_file)?;
    let reader = BufReader::new(file);
    let cmap: ColourMap = reader
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| line.starts_with('#'))
        .map(|line| {
            // anything after a space is a comment
            let parse_to = line.find(' ').unwrap_or_else(|| line.len());
            let hex_parsed = u32::from_str_radix(line.get(1..parse_to).unwrap(), 16).unwrap();
            match parse_to {
                // short hexcode
                4 => {
                    let r: u8 = (hex_parsed >> 8 & 0xF) as u8;
                    let g: u8 = (hex_parsed >> 4 & 0xF) as u8;
                    let b: u8 = (hex_parsed & 0xF) as u8;

                    Pixel {
                        r: r << 4 | r,
                        g: g << 4 | g,
                        b: b << 4 | b,
                    }
                }
                7 => {
                    let r: u8 = (hex_parsed >> 16 & 0xFF) as u8;
                    let g: u8 = (hex_parsed >> 8 & 0xFF) as u8;
                    let b: u8 = (hex_parsed & 0xFF) as u8;

                    Pixel { r, g, b }
                }
                _ => panic!("Unrecognised hexcode: \"{}\", parse_to: {}", line, parse_to),
            }
        })
        .collect();

    if cmap.len() > 0 {
        Ok(cmap)
    } else {
        panic!("Colourmap file must contain at least one colour.")
    }
}
