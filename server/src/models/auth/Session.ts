import * as uuid from "https://deno.land/std@0.175.0/uuid/mod.ts";

import "./Capability.ts"
import { Capabilities } from "./Capability.ts";

export class AccessToken {
    rawToken: string;

    constructor(raw: string) {
        this.rawToken = raw;
    }

    static generate(): AccessToken {
        return new AccessToken(uuid.v4.generate())
    }
}

export class UserId {
    rawId: string;

    constructor(raw: string) {
        this.rawId = raw;
    }

    static generate(): UserId {
        return new UserId(uuid.v4.generate())
    }
}

export enum Role {
    Admin,
    Reviewer,
    Publisher,
}

export class UserSession {
    user_id:        UserId;
    access_token:   AccessToken;
    expiration:     Date;
    permissions:    Capabilities;
    role:           Role;

    constructor(
        user_id:        UserId,
        access_token:   AccessToken,
        expiration:     Date,
        permissions:    Capabilities,
        role:           Role)
    {
        this.user_id = user_id;
        this.access_token = access_token;
        this.expiration = expiration;
        this.permissions = permissions;
        this.role = role;
    }
}