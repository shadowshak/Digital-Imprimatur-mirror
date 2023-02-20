import * as uuid from "https://deno.land/std@0.175.0/uuid/mod.ts";

import "./Capability.ts"

export class AccessToken {
    rawToken: string;

    constructor(raw: string) {
        this.rawToken = raw;
    }
}

export function generate_access_token(): AccessToken {
    return new AccessToken(uuid.v4.generate());
}

export class UserSession {
    user_id: UserId;
    access_token: AccessToken;
    expiration: Date;
    permissions: Capabilities;
    role: Role;
}

export enum Role {
    Admin,
    Reviewer,
    Publisher,
}

export type UserId = string;