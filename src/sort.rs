use std::{str::FromStr, convert::Infallible, fmt};

/// Represents the different sorting options available when requesting assets
/// from ambientCG.
/// 
/// You can convert this to and from strings freely. For forwards compatibility,
/// if an unrecognised string is converted to this type, it will be stored in an
/// Unparsed enum.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Sort {
    Latest,
    Popular,
    Alphabet,
    Downloads,
    Unparsed(String)
}
impl FromStr for Sort {
    type Err = Infallible;
    fn from_str(str: &str) -> Result<Self, Self::Err> {
        match str {
            "Latest" => Ok(Self::Latest),
            "Popular" => Ok(Self::Popular),
            "Alphabet" => Ok(Self::Alphabet),
            "Downloads" => Ok(Self::Downloads),
            x => Ok(Self::Unparsed(x.to_string()))
        }
    }
}
impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match self {
            Self::Latest => "Latest",
            Self::Popular => "Popular",
            Self::Alphabet => "Alphabet",
            Self::Downloads => "Downloads",
            Self::Unparsed(x) => x
        };
        write!(f, "{}", repr)
    }
}