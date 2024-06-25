use super::models::user::User;
use leptos::logging::*;

#[cfg(feature = "ssr")]
pub mod server;

use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

/// The function used to validate signup information. This will be used on the server and is also free to be used on the client.
pub fn validate_signup(
    username: String,
    password: String,
    repeat_password: String,
) -> Result<(), String> {
    if password != repeat_password {
        return Err("Die Passwörter müssen übereinstimmen.".to_owned());
    }

    User::new_validated(username, password).map(|_| ())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SignupResponse {
    Ok,
    Error(String),
}

#[cfg(feature = "ssr")]
pub async fn read_current_user_from_headers() -> Option<String> {
    use http::HeaderMap;

    leptos_axum::extract::<HeaderMap>()
        .await
        .ok()
        .and_then(|headers| {
            let cookie = server::read_auth_cookie(&headers)?;
            Some(cookie.username)
        })
}

#[server(CurrentUserAction, "/api")]
pub async fn current_user() -> Result<String, ServerFnError> {
    read_current_user_from_headers()
        .await
        .ok_or_else(|| ServerFnError::ServerError("you must be logged in".into()))
}

#[server(SignupAction, "/api/signup")]
pub async fn signup_action(
    username: String,
    password: String,
    repeat_password: String,
) -> Result<SignupResponse, ServerFnError> {
    use crate::backend::database;
    use crate::backend::database::DatabaseError;
    use crate::backend::state::AppState;
    use http::StatusCode;
    use leptos::use_context;

    let response_options = use_context::<leptos_axum::ResponseOptions>().unwrap();

    let mut conn = AppState::expect_from_context().database.get().unwrap();

    if let Err(err_msg) =
        validate_signup(username.clone(), password.clone(), repeat_password.clone())
    {
        return Ok(SignupResponse::Error(err_msg));
    };

    let user = match database::user::create_user(&mut conn, username, password) {
        Ok(user) => user,
        Err(err) => {
            warn!("got error during user creation: {err:?}");
            return Err(ServerFnError::new("Internal Server Error".to_owned()));
        }
    };

    if server::set_auth_cookie(user.username) {
        leptos_axum::redirect("/content");
        return Ok(SignupResponse::Ok)
    } else {
        response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        return Err(ServerFnError::new("Failed to set cookie".to_owned()));
    };
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoginResponse {
    Ok,
    IncorrectUserdata(()),
    InvalidUserdata(()),
}

#[server(LoginAction, "/api/login")]
pub async fn login_action(
    username: String,
    password: String,
) -> Result<LoginResponse, ServerFnError> {
    use crate::backend::database;
    use crate::backend::database::DatabaseError;
    use crate::backend::state::AppState;
    use http::StatusCode;
    use leptos::use_context;

    const INCORRECT_LOGIN_DATA: &str = "Falscher Benutzername oder Passwort.";

    if let Some(already_logged_in_username) = read_current_user_from_headers().await {
        leptos_axum::redirect("/content");
        return Err(ServerFnError::new("you are already logged in".to_owned()));
    }

    let state = leptos::use_context::<AppState>().unwrap();
    let mut conn = AppState::expect_from_context().database.get().unwrap();

    let response_options = use_context::<leptos_axum::ResponseOptions>().unwrap();

    let user = match database::user::get_user_by_name(&mut conn, username) {
        Ok(user) => user,
        Err(DatabaseError::NotFound) => return Ok(LoginResponse::IncorrectUserdata(())),
        Err(err) => {
            warn!("got error during user login check: {err:?}");
            response_options.set_status(StatusCode::FORBIDDEN);
            return Err(ServerFnError::new("Internal Server Error"));
        }
    };

    if user.password != format!("{:X}", md5::compute(password)) {
        response_options.set_status(StatusCode::FORBIDDEN);
        return Ok(LoginResponse::IncorrectUserdata(()));
    }

    if server::set_auth_cookie(user.username) {
        leptos_axum::redirect("/content");
        Ok(LoginResponse::Ok)
    } else {
        response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
        Err(ServerFnError::new("Failed to set cookie".to_owned()))
    }
}

#[server(LogoutAction, "/api/logout")]
pub async fn logout_action() -> Result<(), ServerFnError> {
    use crate::backend::database;
    use crate::backend::database::DatabaseError;
    use crate::backend::state::AppState;
    use http::StatusCode;
    use leptos::use_context;

    if let Some(logged_in_user) = read_current_user_from_headers().await {
        server::remove_auth_cookie()
            .then_some(())
            .ok_or(ServerFnError::new("error during cookie removal"))
    } else {
        return Err(ServerFnError::new("you are not logged in"));
    }
}
