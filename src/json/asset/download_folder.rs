use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFolder {
    pub title: String,
    pub downloadFiletypeCategories: HashMap<String, super::DownloadCategory>
}