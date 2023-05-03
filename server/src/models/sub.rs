use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

uuid_based! (SubId);

///
/// Represents the status of a submission
/// 
pub enum SubmissionStatus {
    AwaitingSubmission,
    UnderReview,
    PendingChanges,
    Rejected,
    Accepted,
    Finalized,
}

///
/// Represents a submission
/// 
pub struct Submission {
    pub status:         SubmissionStatus,

    pub name:           String,
    pub author:         String,
    pub desc:           String,

    pub creation:       DateTime<Local>,
    pub last_update:    DateTime<Local>,

    // capabilities
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionMetadata {
    pub name:           String,
    pub author:         String,
    pub description:    String,
    pub creation:       DateTime<Local>,
    pub last_update:    DateTime<Local>,
}