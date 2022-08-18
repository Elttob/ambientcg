/// Represents a file which can be downloaded from ambientCG.
#[derive(Debug)]
pub struct DownloadFile {
    pub full_download_path: String,
    pub download_link: String,
    pub file_name: String,
    pub size: u64,
    pub attribute: Option<String>,
    pub file_type: super::DownloadFileType,
    pub zip_content: Option<Vec<String>>
}