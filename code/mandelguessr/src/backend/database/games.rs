use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

use crate::api::models::game::Game;

use super::DatabaseError;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
struct DbGame {
    id: i32,
    username: String,
    score: i32,
}

impl From<DbGame> for Game {
    fn from(value: DbGame) -> Self {
        Self {
            id: value.id,
            username: value.username,
            score: value.score as u32,
        }
    }
}

pub fn create_game(
    conn: &mut PgConnection,
    username: String,
    score: u32,
) -> Result<Game, DatabaseError> {
    #[derive(Insertable)]
    #[diesel(table_name = super::schema::games)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct DbNewGame {
        username: String,
        score: i32,
    }

    let db_game = DbNewGame { username, score: score as i32 };

    let db_game: DbGame = diesel::insert_into(super::schema::games::table)
        .values(&db_game)
        .returning(DbGame::as_returning())
        .get_result(conn)?;

    Ok(db_game.into())
}

pub fn list_games(conn: &mut PgConnection) -> Result<Vec<Game>, DatabaseError> {
    let db_games = super::schema::games::table
        .select(DbGame::as_select())
        .load(conn)?;

    let games = db_games.into_iter().map(Into::into).collect::<Vec<Game>>();

    Ok(games)
}
