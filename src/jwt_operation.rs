use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{encode, EncodingKey, Header};

mod api_authentication_claims;

pub fn generate_key(secret: &[u8], sub: String, roles: &Vec<String>) -> String {
    let iat = generate_iat();
    let our_claims = api_authentication_claims::ApiAuthenticationClaims::new(sub, iat, roles);

    let token = encode(&Header::default(), &our_claims, &EncodingKey::from_secret(secret.as_ref())).unwrap();

    token
}

fn generate_iat() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis();

    since_the_epoch.to_string()
}
