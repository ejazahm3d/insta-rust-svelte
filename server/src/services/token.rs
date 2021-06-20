use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use uuid::Uuid;
pub struct Token;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

impl Token {
    pub fn sign(claims: Claims) -> Result<String, Error> {
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret("secret".as_ref()),
        )
    }

    pub fn verify(token: &str) -> Result<TokenData<Claims>, Error> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        )
    }
}
