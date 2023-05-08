import axios from "axios";


export class Submission {
    constructor(id, title, author, description, creation, last_update, status) {
        this.id = id;
        this.title = title;
        this.description = description;
        this.author = author;
        this.creation = creation;
        this.last_update = last_update;
        this.status = status;
    }

    async delete(token) {
        try {
            const payload = {
                token,
                sub_id: this.id
            };

            await axios.post("http://localhost:3001/user/login", payload);

            return { };
        }
        catch({ response }) {
            switch (response.status) {
                case 403:
                    // invalid access token
                    return { error: "invalid access token" }
                case 401:
                    // invalid permissions
                    return { error: "invalid permissions" }
                default:
                    return { error: "internal"}
            }
        }
    }
}