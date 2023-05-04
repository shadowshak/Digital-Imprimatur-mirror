CREATE TABLE Documents (
    doc_id    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sub_id    UUID NOT NULL REFERENCES Submissions(sub_id),

    creator   UUID NOT NULL REFERENCES Users(user_id),
    creation  TIMESTAMP NOT NULL DEFAULT NOW(),
    content   BYTEA NOT NULL
);