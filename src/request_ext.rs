use std::collections::HashMap;
use std::sync::Arc;
use serde::Serialize;
use crate::redirect_strategy::RedirectStrategy;
use tide::Request;
use crate::claims::{AppMetadata, Auth0Claims};

/// TEST

/// Provides access to request-level authentication data.
pub trait OpenIdConnectRequestExt {
    /// Returns `true` if the request is authenticated, `false`
    /// otherwise.
    fn is_authenticated(&self) -> bool;

    /// Gets the Identity Provider-specific access token for the
    /// authenticated user, or `None` if the session has not been
    /// authenticated.
    fn access_token(&self) -> Option<String>;

    /// Gets the list of scopes authorized by/granted to the user, or
    /// `None` if the session has not been authenticated.
    fn scopes(&self) -> Option<Vec<String>>;

    /// Gets the Identity Provider-specific user id of the authenticated
    /// user, or `None` if the session has not been authenticated.
    fn user_id(&self) -> Option<String>;
    /// Seeing that this crate uses the auth0claim by default,
    /// this method gets the app_metadata claim that is essential for
    /// things like stripe integration and stuff
    fn provider_app_metadata(&self) -> Option<AppMetadata>;
    /// Same as above but for users
    fn user_app_metadata(&self) -> Option<HashMap<String, String>>;
}

impl<State> OpenIdConnectRequestExt for Request<State>
where
    State: Send + Sync + 'static,
{
    fn is_authenticated(&self) -> bool {
        matches!(
            self.auth_state(),
            OpenIdConnectRequestExtData::Authenticated { .. }
        )
    }

    fn access_token(&self) -> Option<String> {
        match self.auth_state() {
            OpenIdConnectRequestExtData::Authenticated { access_token, .. } => {
                Some(access_token.clone())
            }
            _ => None,
        }
    }

    fn scopes(&self) -> Option<Vec<String>> {
        match self.auth_state() {
            OpenIdConnectRequestExtData::Authenticated { scopes, .. } => Some(scopes.clone()),
            _ => None,
        }
    }

    fn user_id(&self) -> Option<String> {
        match self.auth_state() {
            OpenIdConnectRequestExtData::Authenticated { user_id, .. } => Some(user_id.clone()),
            _ => None,
        }
    }

    fn provider_app_metadata(&self) -> Option<AppMetadata> {
        match self.auth_state() {
            OpenIdConnectRequestExtData::Authenticated { auth0claims, .. } => {
                Some(auth0claims.deserialize_to().ok()?.app_metadata)
            }
            OpenIdConnectRequestExtData::Unauthenticated { .. } => None
        }
    }

    fn user_app_metadata(&self) -> Option<HashMap<String, String>> {
        todo!()
    }
}

pub(crate) enum OpenIdConnectRequestExtData {
    Unauthenticated {
        redirect_strategy: Arc<dyn RedirectStrategy>,
    },
    Authenticated {
        access_token: String,
        scopes: Vec<String>,
        user_id: String,
        auth0claims: Auth0Claims,
    },
}

pub(crate) trait OpenIdConnectRequestExtInternal {
    fn auth_state(&self) -> &OpenIdConnectRequestExtData;
}

impl<State> OpenIdConnectRequestExtInternal for Request<State>
where
    State: Send + Sync + 'static,
{
    fn auth_state(&self) -> &OpenIdConnectRequestExtData {
        self.ext()
            .expect("You must install OpenIdConnectMiddleware to access the Open ID request data.")
    }
}
