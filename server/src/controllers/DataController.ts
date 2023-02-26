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

export interface DataController {
    ///
    /// Stores a file on the filesystem
    ///
    StoreInFile(
        file: string,
        data: Uint8Array): Promise<void>;

    ///
    /// Loads a file from the filesystem
    ///
    LoadFromFile(file: string): Promise<Uint8Array>;

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

export abstract class FsDataController implements DataController {
    StoreInFile(file: string, data: Uint8Array): Promise<void> {
        return Deno.writeFile(file, data)
    }
    LoadFromFile(file: string): Promise<Uint8Array> {
        return Deno.readFile(file)    
    }

    abstract Connect(): Promise<void>;
    abstract Query(query: string): Promise<object>;
    abstract Disconnect(): Promise<void>;

}


// Don't implement a real data controller until we figure out a dataabse
export class DummyDataController extends FsDataController {
    Connect(): Promise<void> {
        return Promise.resolve();
    }
    Query(query: string): Promise<object> {
        return Promise.resolve({});
    }
    Disconnect(): Promise<void> {
        return Promise.resolve();
    }
}