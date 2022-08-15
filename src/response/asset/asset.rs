use std::collections::{HashSet, HashMap};

use chrono::{DateTime, Utc};

use crate::errors;

/// Represents a found asset as part of a response from ambientCG's web API.
/// Most fields are optional and left out by default - you can add in the fields
/// you need by configuring what the request includes, which will cause
/// ambientCG to return values for those fields.
#[derive(Debug)]
pub struct Asset {
    pub asset_id: String,
    pub release_date: Result<DateTime<Utc>, errors::DateParseError>,
    pub early_release_date: Result<DateTime<Utc>, errors::DateParseError>,
    pub data_type: crate::DataType,
    pub creation_method: crate::CreationMethod,
    pub category: String,

    // StatisticsData
    pub download_count: Option<u32>,
    pub download_count_month: Option<u32>,
    pub download_count_week: Option<u32>,
    pub popularity_score: Option<f32>,

    // TagData
    pub tags: Option<Vec<String>>,
    
    // DisplayData
    pub data_type_name: Option<String>,
    pub data_type_description: Option<String>,
    pub creation_method_name: Option<String>,
    pub creation_method_description: Option<String>,
    pub display_name: Option<String>,
    pub custom_display_name: Option<String>,
    pub description: Option<String>,
    pub display_category: Option<String>,
    pub short_link: Option<String>,

    // DimensionsData
    pub dimension_x: Option<f32>,
    pub dimension_y: Option<f32>,
    pub dimension_z: Option<f32>,

    // RelationshipData
    pub created_using: Option<Vec<String>>,
    pub based_on_this: Option<Vec<String>>,

    // NeighbourData
    pub next_asset_id: Option<String>,
    pub previous_asset_id: Option<String>,
    
    // VariationsData
    pub variations: Option<Vec<String>>,

    // TODO: DownloadData

    // PreviewData
    pub preview_links: Option<Vec<super::PreviewLink>>,
    pub preview_type: Option<String>,

    // MapData
    pub maps: Option<HashSet<super::Map>>,

    // UsdData
    pub has_usd: Option<bool>,

    // ImageData
    pub preview_images: Option<HashMap<super::PreviewImage, String>>
}