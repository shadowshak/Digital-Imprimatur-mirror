

use axum::{Json, http::StatusCode};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::{models::{AccessToken, DocId, UserId}, controllers::{Controller, SubmissionError}};

#[derive(Serialize, Deserialize)]
pub struct DocReadRequest {
    token:          AccessToken,
    document_id:    DocId,
}

#[derive(Serialize, Deserialize)]
pub struct DocReadResponse {
    metadata:       DocumentMetadata
}

#[derive(Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub creation:       DateTime<Local>,
    pub last_update:    DateTime<Local>,
    pub creator:        UserId,
}

pub async fn read(
    Json(DocReadRequest {
        token,
        document_id,
    }): Json<DocReadRequest>)
    -> Result<Json<DocReadResponse>, StatusCode>
{
    /*
        User must be associated with the submission the document is attached to 
    */

    let mut documents = Controller::document().await;

    let metadata = documents.get_document_metadata(token, document_id).await
        .map_err(SubmissionError::into_status_code)?;

    let response = DocReadResponse {
        metadata
    };

    Ok(Json(response))
}