import * as uuid from "https://deno.land/std@0.175.0/uuid/mod.ts";

export type Document = string;

///
/// Unique identifier for a document
///
export class DocumentId {
    id: string;

    constructor(id: string) {
        this.id = id;
    }

    static Generate(): DocumentId {
        return new DocumentId(uuid.v4.generate());
    }
}

///
/// Represents what kind of document this is
///
export enum DocKind {
    Document,
    Feedback
}