use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use crate::models::{AccessToken, SubId};


#[derive(Serialize, Deserialize)]
pub struct SubDeleteRequest {
    token:          AccessToken,
    submission_id:  SubId,
}

#[derive(Serialize, Deserialize)]
pub struct SubDeleteResponse;

pub async fn delete(
    Json(SubDeleteRequest {
        token,
        submission_id,
    }): Json<SubDeleteRequest>)
    -> Result<Json<SubDeleteResponse>, StatusCode>
{
    /*
        User must be a publisher 
        User must be associated with the submission User must have delete permissions 
    */


    let response = SubDeleteResponse;

    return Ok(Json(response))
}