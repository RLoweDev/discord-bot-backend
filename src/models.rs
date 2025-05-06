use serde::Deserialize;

#[derive(Deserialize)]
pub struct ClockInRequest {
    pub access_key: String,
    pub project_id: i32,
}