

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};
use serde_bytes_base64::Bytes;

use crate::models::{AccessToken, SubId, FeedbackId};

#[derive(Serialize, Deserialize)]
pub struct FeedbackUploadRequest {
    token:          AccessToken,
    submission_id:  SubId,
    feedback:       Bytes,
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackUploadResponse {
    feedback_id:    FeedbackId,
}

pub async fn upload(
    Json(FeedbackUploadRequest {
        token,
        submission_id,
        feedback,
    }): Json<FeedbackUploadRequest>)
    -> Result<Json<FeedbackUploadResponse>, StatusCode>
{
    /*
        User must be a publisher 
        User must be associated with the submission
    */

    let response = FeedbackUploadResponse {
        feedback_id: FeedbackId::new()
    };

    Ok(Json(response))
}