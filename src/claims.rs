use http_types::convert::Deserialize;
use openidconnect::{AdditionalClaims, EmptyExtraTokenFields, IdTokenFields, StandardErrorResponse, StandardTokenResponse};
use openidconnect::core::{CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse, CoreTokenType};
use serde::Serialize;
use serde_json::Value;

impl AdditionalClaims for Auth0Claims {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Auth0Claims {
    #[serde(alias = "https://dev-nfphi5c8x1gke8yg.us.auth0.com/app_metadata")]
    app_metadata: Value,
}


pub type Auth0IdTokenFields = IdTokenFields<
    Auth0Claims,
    EmptyExtraTokenFields,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
>;

pub type Auth0TokenResponse = StandardTokenResponse<Auth0IdTokenFields, CoreTokenType>;


/// Auth0Client is a slight modified [`openidconnect::core::CoreClient`]. Everything is the same
/// except where the AdditionalClaims is custom
pub type Auto0Client = openidconnect::Client<
    Auth0Claims,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    Auth0TokenResponse,
    CoreTokenType,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
>;


impl AdditionalClaims for Auth0Claims {}