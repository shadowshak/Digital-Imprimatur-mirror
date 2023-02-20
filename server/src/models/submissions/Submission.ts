import { CrudCapability } from "../auth/Capability.ts";

export class SubmissionId {
    id: string;

    constructor(id: string) {
        this.id = id;
    }
}


export class Submission {
    status: SubmissionStatus;
    
    name: string;
    description: string;

    creation_date: Date;
    update_date: Date;

    caps: CrudCapability;

    constructor() {

    }
}

enum SubmissionStatus {
    AwaitingSubmission,
    UnderReview,
    PendingChanges,
    Rejected,
    Accepted,
    Finalized,
}