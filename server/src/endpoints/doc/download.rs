

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
    let mut documents = crate::controllers::Controller::document().await;

    let response = documents.download_document(token, document_id).await
        .map_err(crate::controllers::SubmissionError::into_status_code)?;

    Ok(response)
}