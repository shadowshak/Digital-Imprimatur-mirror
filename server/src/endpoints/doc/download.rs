

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, DocId};

#[derive(Serialize, Deserialize)]
pub struct DocDownloadRequest {
    token:          AccessToken,
    document_id:    DocId,
}

pub async fn download(
    Json(DocDownloadRequest {
        token,
        document_id,
    }): Json<DocDownloadRequest>)
    -> Result<Vec<u8>, StatusCode>
{
    /*
        User must be associated with the submission the document is attached to 
    */

    let response = Vec::new();

    Ok(response)
}