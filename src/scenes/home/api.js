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

    async delete() {
        const token = sessionStorage.getItem("token");

        try {
            const payload = {
                token: token,
                submission_id: this.id
            };

            await axios.post("http://localhost:3001/sub/delete", payload);

            return { };
        }
        catch({ response }) {
            console.log(response);

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

    static async create(title, author, description) {
        const token = sessionStorage.getItem("token");

        try {
            const payload = {
                token,
                name: title,
                author,
                description,
            };

            const { data } = await axios.post("http://localhost:3001/sub/create", payload);

            return new Submission(data.id, title, author, description, null, null, null);
        }
        catch ({ response }) {
            console.log(response);

            switch (response.status) {
                case 422:
                    return { error: "misformed request" }

                default:
                    return { error: "server error" }
            }
        }
    }
}