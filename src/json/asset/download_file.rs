use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct DownloadFile {
    pub fullDownloadPath: String,
    pub downloadLink: String,
    pub fileName: String,
    pub size: u64,
    pub filetype: String,
    pub attribute: String,
    pub zipContent: Option<Vec<String>>
}