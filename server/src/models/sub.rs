use chrono::{DateTime, Local};
use postgres_types::{ToSql, FromSql};
use serde::{Serialize, Deserialize};

uuid_based! (SubId);

///
/// Represents the status of a submission
/// 
#[derive(Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SubmissionMetadata {
    pub name:           String,
    pub author:         String,
    pub description:    String,
    pub creation:       DateTime<Local>,
    pub last_update:    DateTime<Local>,
    pub status:         SubmissionStatus
}

impl Serialize for SubmissionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        let string_repr = match self {
            SubmissionStatus::AwaitingSubmission => "awaiting_submission",
            SubmissionStatus::UnderReview => "under_review",
            SubmissionStatus::PendingChanges => "pending",
            SubmissionStatus::Rejected => "rejected",
            SubmissionStatus::Accepted => "accepted",
            SubmissionStatus::Finalized => "finalized",
        };

        string_repr.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SubmissionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>
    {
        let string_repr = String::deserialize(deserializer)?;

        match string_repr.as_str() {
            "awaiting_submission" => Ok(SubmissionStatus::AwaitingSubmission),
            "under_review" => Ok(SubmissionStatus::UnderReview),
            "pending" => Ok(SubmissionStatus::PendingChanges),
            "rejected" => Ok(SubmissionStatus::Rejected),
            "accepted" => Ok(SubmissionStatus::Accepted),
            "finalized" => Ok(SubmissionStatus::Finalized),
            _ => Err(serde::de::Error::custom("invalid submission status")),
        }
    }
}

impl ToSql for SubmissionStatus {
    fn to_sql(&self, ty: &postgres_types::Type, out: &mut postgres_types::private::BytesMut) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized
    {
        let string_repr = match self {
            SubmissionStatus::AwaitingSubmission => "awaiting_submission",
            SubmissionStatus::UnderReview => "under_review",
            SubmissionStatus::PendingChanges => "pending",
            SubmissionStatus::Rejected => "rejected",
            SubmissionStatus::Accepted => "accepted",
            SubmissionStatus::Finalized => "finalized",
        }.to_string();

        string_repr.to_sql(ty, out)
    }

    fn accepts(ty: &postgres_types::Type) -> bool
    where
        Self: Sized
    {
        <String as ToSql>::accepts(ty)
    }

    fn to_sql_checked(
        &self,
        ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    {
        let string_repr = match self {
            SubmissionStatus::AwaitingSubmission => "awaiting_submission",
            SubmissionStatus::UnderReview => "under_review",
            SubmissionStatus::PendingChanges => "pending",
            SubmissionStatus::Rejected => "rejected",
            SubmissionStatus::Accepted => "accepted",
            SubmissionStatus::Finalized => "finalized",
        }.to_string();

        string_repr.to_sql_checked(ty, out)
    }
}

impl<'a> FromSql<'a> for SubmissionStatus {
    fn from_sql(ty: &postgres_types::Type, raw: &'a [u8]) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let string_repr = String::from_sql(ty, raw)?;

        match string_repr.as_str() {
            "awaiting_submission" => Ok(SubmissionStatus::AwaitingSubmission),
            "under_review" => Ok(SubmissionStatus::UnderReview),
            "pending" => Ok(SubmissionStatus::PendingChanges),
            "rejected" => Ok(SubmissionStatus::Rejected),
            "accepted" => Ok(SubmissionStatus::Accepted),
            "finalized" => Ok(SubmissionStatus::Finalized),
            _ => panic!("corrupt table"),
        }
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        <String as FromSql<'a>>::accepts(ty)
    }
}