use std::{str::FromStr, convert::Infallible, fmt};

/// Represents the different kinds of asset which ambientCG can host.
/// 
/// You can convert this to and from strings freely. For forwards compatibility,
/// if an unrecognised string is converted to this type, it will be stored in an
/// Unparsed enum.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum DataType {
    ThreeDModel,
    Atlas,
    Brush,
    Decal,
    HDRI,
    Material,
    PlainTexture,
    Substance,
    Terrain,
    Unparsed(String)
}
impl FromStr for DataType {
    type Err = Infallible;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "3DModel" => Ok(Self::ThreeDModel),
            "Atlas" => Ok(Self::Atlas),
            "Brush" => Ok(Self::Brush),
            "Decal" => Ok(Self::Decal),
            "HDRI" => Ok(Self::HDRI),
            "Material" => Ok(Self::Material),
            "PlainTexture" => Ok(Self::PlainTexture),
            "Substance" => Ok(Self::Substance),
            "Terrain" => Ok(Self::Terrain),
            x => Ok(Self::Unparsed(x.to_string()))
        }
    }
}
impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::ThreeDModel => "3DModel",
            Self::Atlas => "Atlas",
            Self::Brush => "Brush",
            Self::Decal => "Decal",
            Self::HDRI => "HDRI",
            Self::Material => "Material",
            Self::PlainTexture => "PlainTexture",
            Self::Substance => "Substance",
            Self::Terrain => "Terrain",
            Self::Unparsed(x) => x
        };
        write!(f, "{}", repr)
    }
}