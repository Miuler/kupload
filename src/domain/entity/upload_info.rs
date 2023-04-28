use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UploadInfo {
    pub id: String,
    pub path: String,
}
