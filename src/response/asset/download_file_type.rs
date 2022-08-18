use std::{str::FromStr, convert::Infallible, fmt};

/// Represents a kind of texture map which an asset can have on ambientCG.
/// 
/// You can convert this to and from strings freely. For forwards compatibility,
/// if an unrecognised string is converted to this type, it will be stored in an
/// Unparsed enum.
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum DownloadFileType {
    Zip,
    Exr,
    Jpg,
    Sbsar,
    Obj,
    Unparsed(String)
}
impl FromStr for DownloadFileType {
    type Err = Infallible;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str.to_lowercase().as_str() {
            "zip" => Ok(Self::Zip),
            "exr" => Ok(Self::Exr),
            "jpg" => Ok(Self::Jpg),
            "sbsar" => Ok(Self::Sbsar),
            "obj" => Ok(Self::Obj),
            x => Ok(Self::Unparsed(x.to_string()))
        }
    }
}
impl fmt::Display for DownloadFileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::Zip => "zip",
            Self::Exr => "exr",
            Self::Jpg => "jpg",
            Self::Sbsar => "sbsar",
            Self::Obj => "obj",
            Self::Unparsed(x) => x
        };
        write!(f, "{}", repr)
    }
}