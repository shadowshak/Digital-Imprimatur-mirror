import {
    AugmentedRequest,
    jsonResponse,
} from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";

import { BadJsonError, InvalidContentTypeError, InvalidMethodError, PayloadTooLargeError } from "../../errors.ts";
import { AccessToken } from "../../models/auth/Session.ts";


class SubCreateRequest {
    token: AccessToken;
    name: string;
    description: string;

    constructor(token: string, name: string, description: string) {
        this.token = new AccessToken(token);
        this.name = name;
        this.description = description;
    }

    static from_json(json: any): SubCreateRequest | null {
        if (!json) return null;

        if (!("token" in json)) return null;
        if (!("name" in json)) return null;
        if (!("description" in json)) return null;

        if (typeof json.token !== "string") return null;
        if (typeof json.name !== "string") return null;
        if (typeof json.description !== "string") return null;

        return new SubCreateRequest(json.token, json.name, json.description);
    }
}

export async function sub_create(req: AugmentedRequest) {
    if (req.method != "POST") {
        throw new InvalidMethodError();
    }
    if (req.headers.get("Content-Type") != "application/json") {
        throw new InvalidContentTypeError();
    }

    const body = await req.json();
    const request = SubCreateRequest.from_json(body);

    if (!request) {
        throw new BadJsonError();
    }

    check_for_size_errors(request);

    const response = {};

    return jsonResponse(response);
}

const MAX_NAME_LENGTH = 100;
const MAX_DESCRIPTION_LENGTH = 1000;

function check_for_size_errors(req: SubCreateRequest) {
    if (
        req.name.length > MAX_NAME_LENGTH ||
        req.description.length > MAX_DESCRIPTION_LENGTH
    ) {
        throw new PayloadTooLargeError();
    }
}