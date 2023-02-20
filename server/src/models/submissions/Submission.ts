import { CrudCapability } from "../auth/Capability.ts";
import * as uuid from "https://deno.land/std@0.175.0/uuid/mod.ts";

export class SubmissionId {
    id: string;

    constructor(id: string) {
        this.id = id;
    }

    static generate(): SubmissionId {
        return new SubmissionId(uuid.v4.generate());
    }
}


export class Submission {
    status:         SubmissionStatus;
    
    name:           string;
    description:    string;

    creation_date:  Date;
    update_date:    Date;

    caps:           CrudCapability;

    constructor(
        status:         SubmissionStatus,
        name:           string,
        description:    string,

        creation_date:  Date,
        update_date:    Date,
        caps:           CrudCapability)
    {
        this.status = status;
        this.name = name;
        this.description = description;

        this.creation_date = creation_date;
        this.update_date = update_date;
        this.caps = caps;
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