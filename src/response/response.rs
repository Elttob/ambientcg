#[derive(Debug)]
pub struct Response {
    pub found_assets: Vec<super::asset::Asset>
}