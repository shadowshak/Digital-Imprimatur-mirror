import {
    AugmentedRequest,
    jsonResponse,
} from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";

import { BadJsonError, InvalidContentTypeError, InvalidMethodError, PayloadTooLargeError } from "../../errors.ts";
import { AccessToken } from "../../models/auth/Session.ts";
import { SubmissionId } from "../../models/submissions/Submission.ts";
import { SubReadRequest } from "./read.ts";

///
/// reads the metadata of a submission
///
export async function sub_delete(req: AugmentedRequest) {
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