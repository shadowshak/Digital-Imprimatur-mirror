use chrono::{DateTime, Local};

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
    pub status: SubmissionStatus,

    pub name: String,
    pub desc: String,

    pub creation: DateTime<Local>,
    pub last_update: DateTime<Local>,

    // capabilities
}