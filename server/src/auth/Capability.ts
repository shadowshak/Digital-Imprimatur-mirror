// A CRUD Capability includes capabilities to read, edit, and delete
//
// For example, a document will have CRUD capabilities to read, edit, and delete
export class CrudCapability {
    read: CapabilityToken;
    edit: CapabilityToken;
    delete: CapabilityToken;
}

export type CapabilityToken = string;

export class Capabilities {
    capabilities: Set<CapabilityToken>;

    constructor() {
        this.capabilities = new Set();
    }

    has(capability: CapabilityToken) {
        return this.capabilities.has(capability);
    }
}