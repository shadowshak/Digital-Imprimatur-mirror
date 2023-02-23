# Architecture

## MVC

This server uses a modified MVC architecture. Code is divided into

- Models
- Controllers
- Endpoints (Views)

Each of these components has a specific role.

### Models

A model class represents data stored in the database. Models contain functions to load, save, and perform operations on them.

### Controllers

Controllers exist as static classes within the server. Controllers can interact with each other, and with outside components such as the database, or endpoints.

### Endpoints

Endpoints do four things:
- Convert a request into a model
- Call the corresponding method on a controller
- Handle any errors
- Convert the controller response into an API response


Because of the modularity of the project, it would be possible to create endpoints that can be accessed from an AI tool, or from within the server.


## Layers

The server is divided into four layers, each of which depends on the layer underneath it.

### Data Layer

At the lowest level, we have the data layer. The data layer deals with physical data. The data layer is composed of the `DatabaseController`. This controller abstracts around the database and provides a common interface to access it. This layer can probably be a thin wrapper around a third-party library.

### Security Layer

Above this layer is the security layer. The security layer manages user authentication and encrypts any data added to the database. There are three controllers within the security layer: the `EncryptionController`, the `UserController`, and the `SessionController`.


#### User Controller

The `UserController` manages users for the server. This includes functions to:

- create a new user
- login a user
- get user details
- change user password
- logout a user

#### Session Controller

The `SessionController` manages user sessions for the server. It also caches some functionality of the `UserController`, so it doesn't need to go out to the database. The `SessionController` provides functions to:

- login a user
- check if a user has a capability
- check if a user has a role
- check if a session exists
- logout a user

#### Encryption Controller

The `EncryptionController` manages encryption of documents. It provides functionality to:
- Save a document under a path with a key
- Retrieve a document under a path with a key
- Adding things to a database
- Retrieving things from a database

### Business Logic Layer

#### DocumentController

The `DocumentController` depends on the `EncryptionController` and the `SessionController`. The `DocumentController` manages:

- submissions
- documents and feedback associated with submissions

### Presentation Layer

The presentation layer is how the application presents to clients (such as through the REST API).

The REST API is one component of the presentation layer. Another possible layer will be an API for bots to get a document and submit feedback associated with it.