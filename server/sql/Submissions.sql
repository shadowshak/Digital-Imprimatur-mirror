CREATE TABLE Submissions (
    sub_id      UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        VARCHAR(128) NOT NULL,
    author      VARCHAR(128) NOT NULL,
    description VARCHAR(2048) NOT NULL,

    creation    TIMESTAMP NOT NULL DEFAULT NOW(),
    last_update TIMESTAMP NOT NULL DEFAULT NOW()
);