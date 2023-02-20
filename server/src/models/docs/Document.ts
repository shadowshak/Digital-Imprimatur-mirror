import * as uuid from "https://deno.land/std@0.175.0/uuid/mod.ts";

export type Document = string;

export class DocumentId {
    id: string;

    constructor(id: string) {
        this.id = id;
    }

    static generate(): DocumentId {
        return new DocumentId(uuid.v4.generate());
    }
}

export enum DocKind {
    Document,
    Feedback
}