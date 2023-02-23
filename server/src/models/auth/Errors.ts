///
/// Represents the possible errors that can occur during authentication
///
export enum AuthError {
    /// Occurs when the user is not logged in
    NotLoggedIn,

    /// The user is trying to log in as a different role
    WrongRole,

    /// The user's session has expired
    SessionExpired,

    /// The user does not have the required capabilities
    AccessDenied,

    /// No error occurred
    Ok
}