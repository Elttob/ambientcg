use serde::{Serialize, Deserialize};

use crate::{response, json::util};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PreviewLink {
    pub url: String,
    pub name: String,
    pub display_name: String,
    pub icon_name: Option<String>
}
impl From<PreviewLink> for response::asset::PreviewLink {
    fn from(json: PreviewLink) -> Self {
        Self {
            url: json.url,
            name: json.name,
            display_name: json.display_name,
            icon_name: util::non_empty_str(json.icon_name)
        }
    }
}