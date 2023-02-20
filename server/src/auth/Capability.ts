import { CapabilityToken } from "./Authenticator";

// A CRUD Capability includes capabilities to read, edit, and delete
//
// For example, a document will have CRUD capabilities to read, edit, and delete
class CrudCapability {
    read: CapabilityToken;
    edit: CapabilityToken;
    delete: CapabilityToken;
}