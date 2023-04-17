

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, FeedbackId};

#[derive(Serialize, Deserialize)]
pub struct FeedbackDownloadRequest {
    token:          AccessToken,
    feedback_id:    FeedbackId,
}

pub async fn download(
    Json(FeedbackDownloadRequest {
        token,
        feedback_id,
    }): Json<FeedbackDownloadRequest>)
    -> Result<Vec<u8>, StatusCode>
{
    /*
        User must be associated with the submission the document is attached to 
    */

    let response = Vec::new();

    Ok(response)
}