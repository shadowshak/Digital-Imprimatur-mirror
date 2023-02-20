import {
    createRouter,
    AugmentedRequest,
    createRouteMap,
    textResponse,
    jsonResponse,
  } from "https://deno.land/x/reno@v2.0.81/reno/mod.ts";
import { AuthError } from "../../models/auth/Errors.ts";
import { AccessToken } from "../../models/auth/Session.ts";
  
import { SubmissionId } from "../../models/submissions/Submission.ts";


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

export async function create(req: AugmentedRequest) {
    if (req.method != "POST") {
        return textResponse("Invalid method", 405);
    }
    if (req.headers.get("Content-Type") != "application/json") {
        return textResponse("Invalid content type", 415);
    }

    const body = await req.json();
    const request = sub_create_request(body);

    if (!request) {
        return textResponse("Bad JSON", 400);
    }

    if (check_for_size_errors(request)) {
        return textResponse("Payload too large", 413);
    }

    const response = {};

    if ("error" in response) {
        switch (response.error) {
            case AuthError.NotLoggedIn:
                return textResponse("Not logged in", 403);
            case AuthError.WrongRole:
                return textResponse("Wrong role", 403);
            case AuthError.SessionExpired:
                return textResponse("Session expired", 403);
            case AuthError.AccessDenied:
                return textResponse("Access denied", 403);
            default:
                return textResponse("Access denied", 403);
        }
    }

    return jsonResponse(response);
}

function sub_create_request(req: any): SubCreateRequest | null {
    
}

const MAX_NAME_LENGTH = 100;
const MAX_DESCRIPTION_LENGTH = 1000;

function check_for_size_errors(req: SubCreateRequest): boolean {
    if (req.name.length > MAX_NAME_LENGTH) return true;
    if (req.description.length > MAX_DESCRIPTION_LENGTH) return true;

    return false;
}