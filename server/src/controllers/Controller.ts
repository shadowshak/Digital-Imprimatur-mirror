import { DataController, DummyDataController } from "./DataController.ts";
import { EncryptionController } from "./EncryptionController.ts";

export class Controller {
    static Data: DataController = new DummyDataController();
    static Encryption: EncryptionController = new EncryptionController();    
}