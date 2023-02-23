import { AccessToken } from "../models/auth/Session.ts";
import { DocKind, Document, DocumentId } from "../models/docs/Document.ts";
import { SubmissionId } from "../models/submissions/Submission.ts";

class DocumentManager {
    create_submission(
        session:        AccessToken,
        name:           string,
        description:    string)
    {

    }

    get_submission(
        session:    AccessToken,
        submission: SubmissionId)
    {

    }

    list_documents(
        session:    AccessToken,
        submission: SubmissionId,
        kind:       DocKind)
    {

    }

    update_submission(
        session:    AccessToken,
        submission: SubmissionId,
        delta:      object)
    {

    }

    delete_submission(
        session:    AccessToken,
        submission: SubmissionId)
    {

    }

    create_document(
        session:    AccessToken,
        submission: SubmissionId,

        kind: DocKind,

        document: Document)
    {

    }

    get_metadata(
        session:    AccessToken,
        document:   DocumentId)
    {

    }

    get_document(
        session:    AccessToken,
        document:   DocumentId)
    {

    }

    delete_document(
        session:    AccessToken,
        document:   DocumentId)
    {

    }
}