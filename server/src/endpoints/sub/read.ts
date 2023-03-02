import {
    AugmentedRequest,
    jsonResponse,
} from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";

import { BadJsonError, InvalidContentTypeError, InvalidMethodError, PayloadTooLargeError } from "../../errors.ts";
import { AccessToken } from "../../models/auth/Session.ts";
import { SubmissionId } from "../../models/submissions/Submission.ts";

///
/// token: AccessToken
/// submission_id: SubmissionId
///
export class SubReadRequest {
    token:  AccessToken;
    sub_id: SubmissionId;

    constructor(token: string, sub_id: string) {
        this.token = new AccessToken(token);
        this.sub_id = new SubmissionId(sub_id);
    }

    static from_json(json: any): SubReadRequest | null {
        if (!json) return null;

        if (!("token" in json)) return null;
        if (!("sub_id" in json)) return null;

        if (typeof json.token !== "string") return null;
        if (typeof json.sub_id !== "string") return null;

        // todo: verify the ids have the right length
        return new SubReadRequest(json.token, json.sub_id);
    }
}

///
/// reads the metadata of a submission
///
export async function sub_read(req: AugmentedRequest) {
    if (req.method != "POST") {
        throw new InvalidMethodError();
    }
    if (req.headers.get("Content-Type") != "application/json") {
        throw new InvalidContentTypeError();
    }

    const body = await req.json();
    const request = SubReadRequest.from_json(body);

    if (!request) {
        throw new BadJsonError();
    }

    const response = {};

    return jsonResponse(response);
}