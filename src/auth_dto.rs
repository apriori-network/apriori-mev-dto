use serde::{Deserialize, Serialize};

/// Challenge request from client
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeRequest {
    /// Client's public key (hex string with 0x prefix)
    pub pubkey: String,
    /// Requested role
    pub role: i32,
}

/// Challenge response to client
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResponse {
    /// JWT challenge token (expires in ~15s)
    pub challenge: String,
}

/// Authentication request from client
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Client's public key (hex string with 0x prefix)
    pub pubkey: String,
    /// Challenge JWT received from challenge endpoint
    pub challenge: String,
    /// Signature over keccak256(pubkey_bytes || keccak256(challenge_utf8))
    /// Hex string with 0x prefix
    pub sig: String,
}

/// Authentication response to client
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Access token (expires in ~15min)
    pub access_token: String,
    /// Refresh token (expires in ~1day)
    pub refresh_token: String,
    /// Access token expiry in seconds
    pub expires_in_secs: i64,
}

/// Refresh token request
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshRequest {
    /// Refresh token
    pub refresh_token: String,
}

/// Refresh token response
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshResponse {
    /// New access token
    pub access_token: String,
}