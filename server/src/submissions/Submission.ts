class Submission {
    status: SubmissionStatus;
    name: string;

    creation_date: Date;
    update_date: Date;

    caps: CrudCapability;
}

enum SubmissionStatus {
    AwaitingSubmission,
    UnderReview,
    PendingChanges,
    Rejected,
    Accepted,
    Finalized,
}