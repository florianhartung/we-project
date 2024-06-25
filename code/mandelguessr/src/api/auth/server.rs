use std::time::Duration;

use chrono::Utc;
use http::header::COOKIE;
use leptos::{logging::log, use_context};
use leptos_axum::ResponseOptions;
use serde::{Deserialize, Serialize};

const AUTH_COOKIE: &str = "token";

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub username: String,
    pub expires_ts: chrono::DateTime<Utc>,
}

pub fn read_auth_cookie(headers: &axum::http::HeaderMap) -> Option<TokenClaims> {
    let token_data = headers
        .get(COOKIE)?
        .to_str()
        .ok()?
        .split("; ")
        .find(|&x| x.starts_with(AUTH_COOKIE))?
        .split('=')
        .last()?;

    decode_token(token_data).ok()
}

fn decode_token(s: &str) -> Result<TokenClaims, ()> {
    serde_json::from_str(s).map_err(|_| ())
}

fn encode_token(token_claims: &TokenClaims) -> String {
    serde_json::to_string(token_claims).unwrap()
}

pub fn set_auth_cookie(username: String) -> bool {
    if let Some(options) = use_context::<ResponseOptions>() {
        let claims = TokenClaims {
            username,
            expires_ts: Utc::now() + Duration::from_secs(3_600),
        };

        // let secret = std::env!("JWT_SECRET");
        // let token = jsonwebtoken::encode(
        //     &jsonwebtoken::Header::default(),
        //     &claims,
        //     &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        // )
        // .unwrap();

        let token = encode_token(&claims);
        log!("Setting cookie {token}");

        options.insert_header(
            axum::http::header::SET_COOKIE,
            axum::http::HeaderValue::from_str(&format!("{}={}; path=/; HttpOnly", AUTH_COOKIE, token))
                .expect("header value couldn't be set"),
        );

        true
    } else {
        false
    }
}


pub fn remove_auth_cookie() -> bool {
    if let Some(options) = use_context::<ResponseOptions>() {
        options.insert_header(
            axum::http::header::SET_COOKIE,
            axum::http::HeaderValue::from_str(&format!("{AUTH_COOKIE}=; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT"))
                .expect("header value couldn't be set"),
        );

        true
    } else {
        false
    }
}

