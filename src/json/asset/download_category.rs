use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadCategory {
    pub title: String,
    pub downloads: Vec<super::DownloadFile>
}