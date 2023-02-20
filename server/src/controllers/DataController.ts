/*
Database Controller API

async StoreInFile(string file, byte[] data)
    - Tracks any filesystem errors
async LoadFromFile(string file): byte[]
    - Tracks any filesystem errors
Connect()

async Query(string query): byte[]

Disconnect()

*/

import { Buffer } from "https://deno.land/std@0.177.0/io/mod.ts";

interface DataController {
    ///
    /// Stores a file on the filesystem
    ///
    StoreInFile(
        file: string,
        data: Buffer): Promise<void>;

    ///
    /// Loads a file from the filesystem
    ///
    LoadFromFile(file: string): Promise<Buffer>;

    ///
    /// Connects to the Database
    ///
    Connect(): Promise<void>;

    ///
    /// Queries the database
    ///
    Query(query: string): Promise<object>;

    ///
    /// Disconnects from the database
    ///
    Disconnect(): Promise<void>;
}