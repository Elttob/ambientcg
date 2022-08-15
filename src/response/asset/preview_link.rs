/// Represents information about a preview provided for an asset.
/// 
/// You can convert this to and from strings freely. For forwards compatibility,
/// if an unrecognised string is converted to this type, it will be stored in an
/// Unparsed enum.
#[derive(Debug)]
pub struct PreviewLink {
    pub url: String,
    pub name: String,
    pub display_name: String,
    pub icon_name: Option<String>
}