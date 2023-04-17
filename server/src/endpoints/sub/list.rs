use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, SubId, DocId, FeedbackId};

#[derive(Serialize, Deserialize)]
pub struct SubReadDocRequest {
    token:          AccessToken,
    submission_id:  SubId,
}

#[derive(Serialize, Deserialize)]
pub struct SubReadDocResponse {
    document_ids:   Vec<DocId>,
}

#[derive(Serialize, Deserialize)]
pub struct SubReadFeedbackRequest {
    token:          AccessToken,
    submission_id:  SubId,
}

#[derive(Serialize, Deserialize)]
pub struct SubReadFeedbackResponse {
    feedback_ids:   Vec<FeedbackId>,
}

pub async fn read_doc(
    Json(SubReadDocRequest {
        token,
        submission_id,
    }): Json<SubReadDocRequest>)
    -> Result<Json<SubReadDocResponse>, StatusCode>
{
    /*
        The user must be associated with the submission 
     */
    
    let response = SubReadDocResponse {
        document_ids: Vec::new(),
    };

    return Ok(Json(response))
}

pub async fn read_feedback(
    Json(SubReadFeedbackRequest {
        token,
        submission_id,
    }): Json<SubReadFeedbackRequest>)
    -> Result<Json<SubReadFeedbackResponse>, StatusCode>
{
    /*
        The user must be associated with the submission 
     */

    let response = SubReadFeedbackResponse {
        feedback_ids: Vec::new(),
    };

    return Ok(Json(response))
}