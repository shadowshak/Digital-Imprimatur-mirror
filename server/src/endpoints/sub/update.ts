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
/// delta: any
///
export class SubUpdateRequest {
    token:  AccessToken;
    sub_id: SubmissionId;
    delta:  any;

    constructor(token: string, sub_id: string, delta: any) {
        this.token = new AccessToken(token);
        this.sub_id = new SubmissionId(sub_id);
        this.delta = delta;
    }

    static from_json(json: any): SubUpdateRequest | null {
        if (!json) return null;

        if (!("token" in json)) return null;
        if (!("sub_id" in json)) return null;
        if (!("delta" in json)) return null;

        if (typeof json.token !== "string") return null;
        if (typeof json.sub_id !== "string") return null;

        // todo: verify the ids have the right length
        return new SubUpdateRequest(json.token, json.sub_id, json.delta);
    }
}

///
/// reads the metadata of a submission
///
export async function sub_update(req: AugmentedRequest) {
    if (req.method != "POST") {
        throw new InvalidMethodError();
    }
    if (req.headers.get("Content-Type") != "application/json") {
        throw new InvalidContentTypeError();
    }

    const body = await req.json();
    const request = SubUpdateRequest.from_json(body);

    if (!request) {
        throw new BadJsonError();
    }

    const response = {};

    return jsonResponse(response);
}