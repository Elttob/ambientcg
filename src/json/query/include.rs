use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Include {
    statisticsData: bool,
    tagData: bool,
    displayData: bool,
    dimensionsData: bool,
    relationshipData: bool,
    neighbourData: bool,
    variationsData: bool,
    downloadData: bool,
    previewData: bool,
    mapData: bool,
    usdData: bool,
    imageData: bool
}