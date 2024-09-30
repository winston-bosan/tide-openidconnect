use openidconnect::{AdditionalClaims, EmptyExtraTokenFields, IdTokenFields, StandardErrorResponse, StandardTokenResponse};
use openidconnect::core::{CoreAuthDisplay, CoreAuthPrompt, CoreErrorResponseType, CoreGenderClaim, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse, CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse, CoreTokenType};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Error as JsonError};

/// TEST

/// Implementing the trait to make use of our custom claim in the type params
impl AdditionalClaims for Auth0Claims {}

/// The actual body of the claim from Auth0
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Auth0Claims {
    /// App metadata from Auth0, stored as a JSON Value
    app_metadata: Value,
    my_claim: Option<Value>
}

/// The concrete type the JSON is going to populate/deserialize into
#[derive(Debug, Deserialize, Serialize)]
pub struct AppMetadata {
    /// The Stripe ID linked to the user
    pub user_linked_stripe_id: String,
}

impl Auth0Claims {
    /// Deserialize the app_metadata Value into an AppMetadata struct
    ///
    /// # Returns
    /// - `Ok(AppMetadata)` if deserialization is successful
    /// - `Err(serde_json::Error)` if deserialization fails
    pub fn deserialize_to(&self) -> Result<AppMetadata, serde_json::Error> {
        serde_json::from_value(self.app_metadata.clone())
            .map_err(|e| {
                eprintln!("Failed to deserialize app_metadata: {}", e);
                e
            })
    }
}

impl AppMetadata {
    /// Create a new AppMetadata instance
    ///
    /// # Arguments
    /// * `user_linked_stripe_id` - The Stripe ID linked to the user
    pub fn new(user_linked_stripe_id: String) -> Self {
        Self { user_linked_stripe_id }
    }
}

/// Custom IdTokenFields type for Auth0
pub type Auth0IdTokenFields = IdTokenFields<
    Auth0Claims,
    EmptyExtraTokenFields,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
>;

/// Custom TokenResponse type for Auth0
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
