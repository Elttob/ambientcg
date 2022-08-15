use std::{str::FromStr, convert::Infallible, fmt};

/// Represents a kind of texture map which an asset can have on ambientCG.
/// 
/// You can convert this to and from strings freely. For forwards compatibility,
/// if an unrecognised string is converted to this type, it will be stored in an
/// Unparsed enum.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Map {
    AmbientOcclusion,
    Color,
    Details,
    Displacement,
    Emission,
    Flow,
    Metalness,
    NormalDX,
    NormalGL,
    Opacity,
    Protrusion,
    Roughness,
    Snowfall,
    Soil,

    Preview,
    Unparsed(String)
}
impl FromStr for Map {
    type Err = Infallible;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.to_lowercase().as_str() {
            "ambientocclusion" => Ok(Self::AmbientOcclusion),
            "color" => Ok(Self::Color),
            "colour" => Ok(Self::Color),
            "details" => Ok(Self::Details),
            "displacement" => Ok(Self::Displacement),
            "emission" => Ok(Self::Emission),
            "flow" => Ok(Self::Flow),
            "metalness" => Ok(Self::Metalness),
            "metallness" => Ok(Self::Metalness),
            "normaldx" => Ok(Self::NormalDX),
            "normalgl" => Ok(Self::NormalGL),
            "opacity" => Ok(Self::Opacity),
            "protrusion" => Ok(Self::Protrusion),
            "roughness" => Ok(Self::Roughness),
            "snowfall" => Ok(Self::Snowfall),
            "soil" => Ok(Self::Soil),
            "preview" => Ok(Self::Preview),
            x => Ok(Self::Unparsed(x.to_string()))
        }
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::AmbientOcclusion => "AmbientOcclusion",
            Self::Color => "Color",
            Self::Details => "Details",
            Self::Displacement => "Displacement",
            Self::Emission => "Emission",
            Self::Flow => "Flow",
            Self::Metalness => "Metalness",
            Self::NormalDX => "NormalDX",
            Self::NormalGL => "NormalGL",
            Self::Opacity => "Opacity",
            Self::Protrusion => "Protrusion",
            Self::Roughness => "Roughness",
            Self::Snowfall => "Snowfall",
            Self::Soil => "Soil",
            Self::Preview => "PREVIEW",
            Self::Unparsed(x) => x
        };
        write!(f, "{}", repr)
    }
}