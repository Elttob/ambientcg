use serde::{Serialize, Deserialize};

use crate::response;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    searchQuery: super::query::Query,
    foundAssets: Vec<super::asset::Asset>
}

impl From<Response> for response::Response {
    fn from(json: Response) -> Self {
        Self {
            found_assets: json.foundAssets
                .into_iter()
                .map(|x| response::asset::Asset::from(x))
                .collect()
        }
    }
}