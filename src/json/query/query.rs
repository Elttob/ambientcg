use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Query {
    includeUnreleased: u8,
    forceSpecificAssetId: bool,
    category: Vec<String>,
    assetId: Vec<String>,
    absoluteDate: String,
    dataType: Vec<String>,
    creationMethod: Vec<String>,
    queryString: String,
    sort: String,
    createdUsingAssetId: String,
    basedOnAssetId: String,
    variationsOfAssetId: String,
    limit: u32,
    offset: u32,
    include: super::Include
}