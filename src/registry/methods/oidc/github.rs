// Standard Uses

// Crate Uses
use super::OIDCHandler;

// External Uses
use eyre::Result;



pub struct GithubOIDC {
    id: String,
    secret: String,
    redirect_url: String
}

impl GithubOIDC {
    pub fn new(id: String, secret: String, redirect_url: String) -> Result<Box<dyn OIDCHandler>> {
        Box::new(Self {
            id, secret,
            redirect_url,
        })
    }
}

impl OIDCHandler for GithubOIDC {
    fn authenticate_and_authorizate(&mut self) -> Result<()> {
        todo!()
    }

    fn auth_callback(&mut self) -> Result<()> {
        todo!()
    }
}
