use derive_more::Display;
use hex;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rs_crypto::aes_cbc;
use rs_crypto::error::CryptoError;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Debug, Clone)]
pub struct JWT {
    pub secret: String,
    pub expires_in: i64,
    pub encryption_key: [u8; 16],
}

impl JWT {
    pub fn new(secret: String, expires_in: i64, encryption_key: String) -> Result<Self, JWTError> {
        let encryption_key: [u8; 16] = hex::decode(encryption_key)?
            .try_into()
            .map_err(|_| JWTError::InvalidEncryptionKey)?;
        Ok(Self {
            secret,
            expires_in,
            encryption_key,
        })
    }

    pub fn create_claims(&self, sub: String) -> Claims {
        Claims {
            sub,
            exp: (OffsetDateTime::now_utc() + Duration::seconds(self.expires_in)).unix_timestamp(),
        }
    }

    pub fn encode(&self, claims: &Claims) -> Result<String, JWTError> {
        let mut c = claims.clone();
        let iv = aes_cbc::generate_iv();
        let cipher_text = aes_cbc::encrypt(&c.sub, self.encryption_key, iv)?;
        let iv_cipher_text = [&iv[..], &cipher_text[..]].concat();
        c.sub = hex::encode(iv_cipher_text);

        encode(
            &Header::default(),
            &c,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| JWTError::EncodeError { source: e })
    }

    pub fn decode(&self, token: &str) -> Result<TokenData<Claims>, JWTError> {
        let mut result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| JWTError::DecodeError { source: e })?;

        let iv_cipher_text = hex::decode(result.claims.sub)?;
        let iv: [u8; 16] = iv_cipher_text[..16]
            .try_into()
            .map_err(|_| JWTError::InvalidEncryptionIV)?;
        let cipher_text = iv_cipher_text[16..].to_vec();
        let plain_sub = aes_cbc::decrypt(cipher_text, self.encryption_key, iv)?;
        result.claims.sub = String::from_utf8(plain_sub)?;
        Ok(result)
    }
}

#[derive(Display, Debug)]
pub enum JWTError {
    #[display("EncodeError: {source}")]
    EncodeError { source: jsonwebtoken::errors::Error },
    #[display("DecodeError: {source}")]
    DecodeError { source: jsonwebtoken::errors::Error },
    #[display("FromUtf8Error: {source}")]
    FromUtf8Error { source: std::string::FromUtf8Error },
    #[display("InvalidEncryptionKey")]
    InvalidEncryptionKey,
    #[display("InvalidEncryptionIV")]
    InvalidEncryptionIV,
    #[display("CryptoError: {source}")]
    CryptoError { source: CryptoError },
    #[display("FromHexError: {source}")]
    FromHexError { source: hex::FromHexError },
}

impl std::error::Error for JWTError {}

impl From<CryptoError> for JWTError {
    fn from(e: CryptoError) -> Self {
        JWTError::CryptoError { source: e }
    }
}

impl From<hex::FromHexError> for JWTError {
    fn from(e: hex::FromHexError) -> Self {
        JWTError::FromHexError { source: e }
    }
}

impl From<std::string::FromUtf8Error> for JWTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        JWTError::DecodeError { source: e.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_encode_and_decode() {
        let key = "0123456789012345";
        let jwt = JWT::new("secret".into(), 3600, key.into()).unwrap();
        let claims = jwt.create_claims("test".to_string());
        let token = jwt.encode(&claims).unwrap();
        let decoded = jwt.decode(&token).unwrap();
        assert_eq!(decoded.claims.sub, "test");
    }

    #[test]
    fn test_create_encryption_key() {
        let mut rng = rand::thread_rng();
        let iv: [u8; 16] = rng.gen();
        let key = hex::encode(iv);
        println!("key: {}", key);
        assert!(true);
    }
}
