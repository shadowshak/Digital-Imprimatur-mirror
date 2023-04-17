

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, FeedbackId};

#[derive(Serialize, Deserialize)]
pub struct FeedbackReadRequest {
    token:          AccessToken,
    feedback_id:    FeedbackId,
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackReadResponse {
    metadata:       FeedbackMetadata
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackMetadata {

}

pub async fn read(
    Json(FeedbackReadRequest {
        token,
        feedback_id: document_id,
    }): Json<FeedbackReadRequest>)
    -> Result<Json<FeedbackReadResponse>, StatusCode>
{
    /*
        User must be associated with the submission the document is attached to 
    */

    let metadata = FeedbackMetadata {

    };

    let response = FeedbackReadResponse {
        metadata
    };

    Ok(Json(response))
}