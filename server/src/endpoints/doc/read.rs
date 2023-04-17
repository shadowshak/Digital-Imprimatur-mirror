

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, DocId};

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

    let metadata = DocumentMetadata {

    };

    let response = DocReadResponse {
        metadata
    };

    Ok(Json(response))
}