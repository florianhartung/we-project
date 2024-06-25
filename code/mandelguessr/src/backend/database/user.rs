use diesel::{
    ExpressionMethods, Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl, Selectable,
    SelectableHelper,
};

use crate::api::models::user::User;

use super::DatabaseError;

#[derive(Queryable, Selectable)]
#[diesel(table_name = super::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
struct DbUser {
    username: String,
    password: String,
}

impl From<DbUser> for User {
    fn from(value: DbUser) -> Self {
        Self::new_validated(value.username, value.password).expect("a DbUser to be validated")
    }
}

pub fn create_user(
    conn: &mut PgConnection,
    username: String,
    password: String,
) -> Result<User, DatabaseError> {
    #[derive(Insertable)]
    #[diesel(table_name = super::schema::users)]
    struct DbNewUser {
        username: String,
        password: String,
    }

    let db_user = DbNewUser { username, password: format!("{:X}", md5::compute(&password)) };

    let db_user: DbUser = diesel::insert_into(super::schema::users::table)
        .values(&db_user)
        .returning(DbUser::as_returning())
        .get_result(conn)?;

    Ok(db_user.into())
}

pub fn get_user_by_name(conn: &mut PgConnection, username: String) -> Result<User, DatabaseError> {
    let db_user: DbUser = super::schema::users::table
        .filter(super::schema::users::username.eq(username.as_str()))
        .select(DbUser::as_select())
        .get_result(conn)?;

    Ok(db_user.into())
}
