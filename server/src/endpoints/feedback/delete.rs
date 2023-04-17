

use axum::{Json, http::StatusCode};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, FeedbackId};

#[derive(Serialize, Deserialize)]
pub struct FeedbackDeleteRequest {
    token:          AccessToken,
    feedback_id:    FeedbackId,
}

#[derive(Serialize, Deserialize)]
pub struct FeedbackDeleteResponse;

pub async fn delete(
    Json(FeedbackDeleteRequest {
        token,
        feedback_id,
    }): Json<FeedbackDeleteRequest>)
    -> Result<Json<FeedbackDeleteResponse>, StatusCode>
{
    /*
        User must be associated with the submission the document is attached to
        User must be a publisher 
        User must have delete permissions 
    */

    let response = FeedbackDeleteResponse;

    Ok(Json(response))
}