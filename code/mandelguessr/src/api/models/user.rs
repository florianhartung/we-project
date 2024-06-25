use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    // TODO don't send password to client
    // #[cfg_attr(feature = "hydrate", allow(dead_code))]
    // #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    pub fn new_validated(username: String, password: String) -> Result<Self, String> {
        if password.len() < 8 {
            return Err("Das Passwort muss min. 8 Zeichen lang sein.".to_owned());
        }

        Ok(Self { username, password })
    }
}
