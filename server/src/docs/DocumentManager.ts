class DocumentManager {
    create_submission(
        session:        SessionToken,
        name:           string,
        description:    string)
    {

    }

    get_submission(
        session:    SessionToken,
        submission: SubmissionId)
    {

    }

    list_documents(
        session:    SessionToken,
        submission: SubmissionId,
        kind:       DocKind)
    {

    }

    update_submission(
        session:    SessionToken,
        submission: SubmissionId,
        delta:      object)
    {

    }

    delete_submission(
        session:    SessionToken,
        submission: SubmissionId)
    {

    }

    create_document(
        session:    SessionToken,
        submission: SubmissionId,

        kind: DocKind,

        document: Document)
    {

    }

    get_metadata(
        session:    SessionToken,
        document:   DocumentId)
    {

    }

    get_document(
        session:    SessionToken,
        document:   DocumentId)
    {

    }

    delete_document(
        session:    SessionToken,
        document:   DocumentId)
    {

    }
}

enum DocKind {
    Document,
    Feedback
}