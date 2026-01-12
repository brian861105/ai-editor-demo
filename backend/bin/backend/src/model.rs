use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicResponse {
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicRequest {
    pub text: String,
}
