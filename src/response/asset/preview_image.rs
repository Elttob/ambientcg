use std::{str::FromStr, convert::Infallible};

use hex_color::HexColor;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PreviewImage {
    Png {
        resolution: u32
    },
    Jpg {
        resolution: u32,
        background_colour: (u8, u8, u8)
    },
    Other(String)
}
impl FromStr for PreviewImage {
    type Err = Infallible;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let parts = str.split("-").collect::<Vec<&str>>();
        if parts.len() < 2 {
            return Ok(Self::Other(str.to_string()));
        }
        let resolution = match parts[0].parse::<u32>() {
            Ok(x) => x,
            Err(_) => return Ok(Self::Other(str.to_string()))
        };

        Ok(match parts[1].to_lowercase().as_str() {
            "png" => Self::Png {
                resolution
            },
            "jpg" => {
                if parts.len() < 3 {
                    return Ok(Self::Other(str.to_string()));
                }
                let background = parts[2].parse::<HexColor>();
                match background {
                    Ok(colour) => Self::Jpg {
                        resolution, 
                        background_colour: (colour.r, colour.g, colour.b)
                    },
                    Err(_) => return Ok(Self::Other(str.to_string()))
                }
            },
            _ => Self::Other(str.to_string())
        })
    }
}