///
/// A CRUD Capability includes capabilities to read, edit, and delete
///
/// For example, a document will have CRUD capabilities to read, edit, and delete
///
export class CrudCapability {
    read:   CapabilityToken;
    edit:   CapabilityToken;
    remove: CapabilityToken;

    constructor(
        read:   CapabilityToken,
        edit:   CapabilityToken,
        remove: CapabilityToken)
    {
        this.read = read;
        this.edit = edit;
        this.remove = remove;
    }
}

///
/// A set of capabilities owned by a user
///
/// This can be used to determine what a user can do
///
export class Capabilities {
    capabilities: Set<CapabilityToken>;

    constructor() {
        this.capabilities = new Set();
    }

    Has(capability: CapabilityToken) {
        return this.capabilities.has(capability);
    }

    Grant(capability: CapabilityToken) {
        this.capabilities.add(capability);
    }
}

///
/// A capability token is a string that represents a capability
///
export class CapabilityToken {
    capability_path: string;

    constructor(capability_path: string) {
        this.capability_path = capability_path;
    }

    Subtoken(name: string): CapabilityToken {
        return new CapabilityToken(`${this.capability_path}.${name}`);
    }
}