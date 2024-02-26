// Standard Uses

// Crate Uses
use super::OIDCHandler;

// External Uses
use eyre::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use openidconnect::*;
use openidconnect::core::*;
use openidconnect::reqwest::http_client;





pub struct GitlabOIDC {
    id: ClientId,
    secret: ClientSecret,
    redirect_url: RedirectUrl
}

impl GitlabOIDC {
    pub fn new(id: String, secret: String, redirect_url: String) -> Result<Box<dyn OIDCHandler>> {
        Ok(
            Box::new(Self {
                id: ClientId::new(id), secret: ClientSecret::new(secret),
                redirect_url: RedirectUrl::new(redirect_url)?,
                }
            )
        )
    }
}


impl OIDCHandler for GitlabOIDC {
    fn authenticate_and_authorizate(&mut self) -> Result<()> {
        let issuer_url = IssuerUrl::new("https://gitlab.com".to_string())?;

        // Fetch GitLab's OpenID Connect discovery document.
        let provider_metadata = CoreProviderMetadata::discover(
            &issuer_url, http_client
        )
            .unwrap_or_else(|err| {
                handle_error(&err, "Failed to discover OpenID Provider");
                unreachable!();
            });

        // Set up the config for the GitLab OAuth2 process.
        let client = CoreClient::from_provider_metadata(
            provider_metadata,
            self.id,
            Some(self.secret),
        )
        .set_redirect_uri(self.redirect_url);

        // Generate the authorization URL to which we'll redirect the user.
        let (authorize_url, csrf_state, nonce) = client
            .authorize_url(
                AuthenticationFlow::<CoreResponseType>::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            // This example is requesting access to the the user's profile including email.
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        println!("Open this URL in your browser:\n{}\n", authorize_url);
        
        Ok(authorize_url.to_string())
    }


    fn auth_callback(&mut self) -> Result<()> {
        println!("GitLab returned the following code:\n{}\n", code.secret());
        println!(
            "GitLab returned the following state:\n{} (expected `{}`)\n",
            state.secret(),
            csrf_state.secret()
        );

        // Exchange the code with a token.
        let token_response = client
            .exchange_code(code)
            .request(http_client)
            .unwrap_or_else(|err| {
                handle_error(&err, "Failed to contact token endpoint");
                unreachable!();
            });

        println!(
            "GitLab returned access token:\n{}\n",
            token_response.access_token().secret()
        );
        println!("GitLab returned scopes: {:?}", token_response.scopes());

        let id_token_verifier = client.id_token_verifier();
        let id_token_claims: &CoreIdTokenClaims = token_response
            .extra_fields()
            .id_token()
            .expect("Server did not return an ID token")
            .claims(&id_token_verifier, &nonce)
            .unwrap_or_else(|err| {
                handle_error(&err, "Failed to verify ID token");
                unreachable!();
            });
        println!("GitLab returned ID token: {:?}\n", id_token_claims);

        let userinfo_claims: UserInfoClaims<GitLabClaims, CoreGenderClaim> = client
            .user_info(token_response.access_token().to_owned(), None)
            .unwrap_or_else(|err| {
                handle_error(&err, "No user info endpoint");
                unreachable!();
            })
            .request(http_client)
            .unwrap_or_else(|err| {
                handle_error(&err, "Failed requesting user info");
                unreachable!();
            });
        
        println!("GitLab returned UserInfo: {:?}", userinfo_claims);
        
        Ok(userinfo_claims)
    }
}


#[derive(Deserialize, Serialize)]
#[derive(Debug)]
struct GitLabClaims {
    // Deprecated and thus optional as it might be removed in the futre
    sub_legacy: Option<String>,
    groups: Vec<String>,
}
impl AdditionalClaims for GitLabClaims {}



fn handle_error<T: std::error::Error>(fail: &T, msg: &'static str) {
    let mut err_msg = format!("ERROR: {}", msg);
    let mut cur_fail: Option<&dyn std::error::Error> = Some(fail);
    
    while let Some(cause) = cur_fail {
        err_msg += &format!("\n    caused by: {}", cause);
        cur_fail = cause.source();
    }
    
    println!("{}", err_msg);
}
