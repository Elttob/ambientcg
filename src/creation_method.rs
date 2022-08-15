use std::{str::FromStr, convert::Infallible, fmt};

/// Represents the different methods which are used to author assets on
/// ambientCG.
/// 
/// You can convert this to and from strings freely. For forwards compatibility,
/// if an unrecognised string is converted to this type, it will be stored in an
/// Unparsed enum.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum CreationMethod {
    PBRApproximated,
    PBRPhotogrammetry,
    PBRProcedural,
    PBRMultiAngle,
    PlainPhoto,
    ThreeDPhotogrammetry,
    HDRIStitched,
    HDRIStitchedEdited,
    UnknownOrOther,
    Unparsed(String)
}
impl FromStr for CreationMethod {
    type Err = Infallible;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "PBRApproximated" => Ok(Self::PBRApproximated),
            "PBRPhotogrammetry" => Ok(Self::PBRPhotogrammetry),
            "PBRProcedural" => Ok(Self::PBRProcedural),
            "PBRMultiAngle" => Ok(Self::PBRMultiAngle),
            "PlainPhoto" => Ok(Self::PlainPhoto),
            "3DPhotogrammetry" => Ok(Self::ThreeDPhotogrammetry),
            "HDRIStitched" => Ok(Self::HDRIStitched),
            "HDRIStitchedEdited" => Ok(Self::HDRIStitchedEdited),
            "UnknownOrOther" => Ok(Self::UnknownOrOther),
            x => Ok(Self::Unparsed(x.to_string()))
        }
    }
}
impl fmt::Display for CreationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::PBRApproximated => "PBRApproximated",
            Self::PBRPhotogrammetry => "PBRPhotogrammetry",
            Self::PBRProcedural => "PBRProcedural",
            Self::PBRMultiAngle => "PBRMultiAngle",
            Self::PlainPhoto => "PlainPhoto",
            Self::ThreeDPhotogrammetry => "3DPhotogrammetry",
            Self::HDRIStitched => "HDRIStitched",
            Self::HDRIStitchedEdited => "HDRIStitchedEdited",
            Self::UnknownOrOther => "UnknownOrOther",
            Self::Unparsed(x) => x
        };
        write!(f, "{}", repr)
    }
}