// Relative Modules
pub mod gitlab;
pub mod github;

// External Uses
use eyre::Result;


pub type ResourceServerFn = (&'static str, fn(String, String, String) -> Result<Box<dyn OIDCHandler>>);

pub const RESOURCE_SERVERS: &'static [ResourceServerFn] = &[
    ("gitlab", gitlab::GitlabOIDC::new),
    ("github", github::GithubOIDC::new)
];


pub struct OIDCCredentials {
    id: String,
    secret: String,
}

pub trait OIDCHandler {
    /// Starts the authentication and authorization process by forming an request URL
    fn authenticate_and_authorizate(&mut self) -> Result<()>;

    fn auth_callback(&mut self) -> Result<()>;
}
