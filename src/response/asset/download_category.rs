/// Represents a category of files which can be downloaded from ambientCG.
#[derive(Debug)]
pub struct DownloadCategory {
    pub file_type: super::DownloadFileType,
    pub downloads: Vec<super::DownloadFile>
}