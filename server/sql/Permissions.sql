CREATE TABLE Permissions (
    perm_id SERIAL PRIMARY KEY,

    user_id UUID NOT NULL REFERENCES Users(user_id),
    sub_id  UUID NOT NULL REFERENCES Submissions(sub_id),

    role    VARCHAR(32) NOT NULL
);