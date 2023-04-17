

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};
use serde_bytes_base64::Bytes;

use crate::models::{AccessToken, SubId, DocId};

#[derive(Serialize, Deserialize)]
pub struct DocUploadRequest {
    token:          AccessToken,
    submission_id:  SubId,
    document:       Bytes,
}

#[derive(Serialize, Deserialize)]
pub struct DocUploadResponse {
    document_id:    DocId,
}

pub async fn upload(
    Json(DocUploadRequest {
        token,
        submission_id,
        document,
    }): Json<DocUploadRequest>)
    -> Result<Json<DocUploadResponse>, StatusCode>
{
    /*
        User must be a publisher 
        User must be associated with the submission
    */

    let response = DocUploadResponse {
        document_id: DocId::new()
    };

    Ok(Json(response))
}