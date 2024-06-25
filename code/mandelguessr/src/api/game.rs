use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

use super::models::game::Game;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GameInfo {
    pub position: (f32, f32),
    pub zoom_exponent: f32,
}

#[server(StartGameAction, "/api/start_game")]
pub async fn start_game() -> Result<GameInfo, ServerFnError> {
    use super::auth::read_current_user_from_headers;
    use crate::backend::state::AppState;

    let mut conn = AppState::expect_from_context().database.get().unwrap();

    let current_user = read_current_user_from_headers().await;
    let Some(_current_user) = current_user else {
        return Err(ServerFnError::new("Unauthorized"));
    };

    let position = (
        2.0 * rand::random::<f32>() - 1.0,
        0.8 * rand::random::<f32>(),
    );
    let zoom_exponent: f32 = 1.5 * rand::random::<f32>() + 1.5;

    Ok(GameInfo {
        position,
        zoom_exponent,
    })
}

#[server(EndGameAction, "/api/end_game")]
pub async fn end_game(score: u32) -> Result<(), ServerFnError> {
    use super::auth::read_current_user_from_headers;
    use crate::backend::database;
    use crate::backend::state::AppState;

    let mut conn = AppState::expect_from_context().database.get().unwrap();

    let current_user = read_current_user_from_headers()
        .await
        .ok_or(ServerFnError::new("Unauthenticated"))?;

    let _ = database::games::create_game(&mut conn, current_user, score)
        .map_err(|_| ServerFnError::new("Database error"))?;

    Ok(())
}



#[server(ListGamesAction, "/api/list_games")]
pub async fn list_games() -> Result<Vec<Game>, ServerFnError> {
    use super::auth::read_current_user_from_headers;
    use crate::backend::database;
    use crate::backend::state::AppState;

    let mut conn = AppState::expect_from_context().database.get().unwrap();

    let games = database::games::list_games(&mut conn)
        .map_err(|_| ServerFnError::new("Database error"))?;

    Ok(games)
}