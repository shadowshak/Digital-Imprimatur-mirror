export abstract class HttpError extends Error {
    code: number

    constructor(message: string, code: number) {
        super(message)
        this.code = code
    }
}

export class InvalidMethodError extends HttpError {
    constructor() {
        super("Invalid method", 405)
    }
}

export class InvalidContentTypeError extends HttpError {
    constructor() {
        super("Invalid content type", 415)
    }
}

export class BadJsonError extends HttpError {
    constructor() {
        super("Bad JSON", 400)
    }
}

export class PayloadTooLargeError extends HttpError {
    constructor() {
        super("Payload too large", 413)
    }
}

export class AuthError extends HttpError {
    private constructor(message: string) {
        super(message, 403)
    }

    static NotLoggedIn = new AuthError("Not logged in")
    static WrongRole = new AuthError("Wrong role")
    static SessionExpired = new AuthError("Session expired")
    static AccessDenied = new AuthError("Access denied")
}