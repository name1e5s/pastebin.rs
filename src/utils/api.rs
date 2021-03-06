use serde::Serialize;
use serde_json;
use tide::IntoResponse;
use tide::Response;

pub struct APIResult {
    pub data: serde_json::Value,
}

/// Form a new API result by a Serializable struct
pub fn new_api_result(data: &impl Serialize) -> APIResult {
    APIResult {
        data: serde_json::to_value(data).unwrap().into(),
    }
}

/// Convert a `APIResult` type into `tide::Response`.
impl IntoResponse for APIResult {
    fn into_response(self) -> Response {
        Response::new(200).body_json(&self.data).unwrap()
    }
}
