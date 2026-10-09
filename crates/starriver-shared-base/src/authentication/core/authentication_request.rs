/// A request used for authentication.
///
/// * Examples:
///     * Multi-field types: username & password
///     * Single-field types: OAuth2 access token
pub trait AuthenticationRequest: Send + Sync {}
