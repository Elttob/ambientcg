use std::collections::HashMap;

/// Represents information about a downloadable folder for an ambientCG asset.
#[derive(Debug)]
pub struct DownloadFolder {
    pub title: String,
    pub download_filetype_categories: HashMap<super::DownloadFileType, super::DownloadCategory>
}