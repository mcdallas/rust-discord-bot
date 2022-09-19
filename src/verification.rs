use ed25519_dalek::{PublicKey, Signature, SignatureError, Verifier};
use hex::FromHexError;

#[derive(Debug, thiserror::Error)]
pub(crate) enum VerificationError {
    #[error("Failed to parse from hex.")]
    ParseHexFailed(#[from] FromHexError),

    #[error("Invalid public key provided.")]
    InvalidPublicKey(#[from] SignatureError),

    #[error("Invalid signature provided.")]
    InvalidSignature(ed25519_dalek::ed25519::Error),
}

pub(crate) fn verify_signature(
    public_key: &str,
    signature: &str,
    timestamp: &str,
    body: &str,
) -> Result<(), VerificationError> {
    let public_key = &hex::decode(public_key)
        .map_err(VerificationError::ParseHexFailed)
        .and_then(|bytes| {
            PublicKey::from_bytes(&bytes).map_err(VerificationError::InvalidSignature)
        })?;

    Ok(public_key.verify(
        format!("{}{}", timestamp, body).as_bytes(),
        &hex::decode(&signature)
            .map_err(VerificationError::ParseHexFailed)
            .and_then(|bytes| {
                Signature::from_bytes(&bytes).map_err(VerificationError::InvalidSignature)
            })?,
    )?)
}