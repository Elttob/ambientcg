/// Stores a response from ambientCG's API in response to a previously submitted
/// request.
#[derive(Debug)]
pub struct Response {
    pub found_assets: Vec<super::asset::Asset>
}