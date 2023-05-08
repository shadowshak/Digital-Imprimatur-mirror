

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::{models::{AccessToken, DocId}, controllers::{Controller, SubmissionError}};

#[derive(Serialize, Deserialize)]
pub struct DocDeleteRequest {
    token:          AccessToken,
    document_id:    DocId,
}

#[derive(Serialize, Deserialize)]
pub struct DocDeleteResponse;

pub async fn delete(
    Json(DocDeleteRequest {
        token,
        document_id,
    }): Json<DocDeleteRequest>)
    -> Result<Json<DocDeleteResponse>, StatusCode>
{
    /*
        User must be associated with the submission the document is attached to
        User must be a publisher 
        User must have delete permissions 
    */

    let mut documents = Controller::document().await;

    documents.delete_document(token, document_id).await
        .map_err(SubmissionError::into_status_code)?;

    let response = DocDeleteResponse;

    Ok(Json(response))
}