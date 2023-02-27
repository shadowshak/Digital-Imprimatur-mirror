import * as bcrypt from "https://deno.land/x/bcrypt@v0.3.0/mod.ts";
import { Controller } from "./Controller.ts";
import { FsDataController } from "./DataController.ts";

type UserId = string;

///
/// Provides an abstraction over the data controller
///
/// The data controller deals with unstructured data,
/// whereas the encryption controller adds some structure
/// to the data. 
///
/// The encryption controller provides methods to check passwords,
/// encrypt data in the database, and to save files to the database.
///
export class EncryptionController {
    async SavePasswordForUser(user: UserId, password: string) {
        // Hash the password
        const hashedPassword = bcrypt.hash(password);

        // Save hashedPassword to database
        // !bug: this is vulnerable to SQL injection
        const query = `
            UPDATE users
            SET password = ${hashedPassword}
            WHERE id = ${user};
        `;

        // todo: use data controller
        await Controller.Data.Query(query);
    }

    async CheckPasswordForUser(user: UserId, password: string): Promise<boolean> {
        const query = `
            SELECT password
            FROM users
            WHERE id = ${user};
        `;

        // todo: check that the user exists
        const hashedPassword = await Controller.Data.Query(query);

        return bcrypt.compare(password, hashedPassword);
    }
}