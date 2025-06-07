use crate::Auth::dsl::*;
use crate::Auth::*;
use crate::{users_crud, AuthTable};
use diesel::result::Error;
use diesel::*;
use std::ops::Add;

pub fn login(
    conn: &mut SqliteConnection,
    login_email: String,
    login_password: String,
) -> Result<(String, String), Error> {
    // Get the corresponding user id if it exists
    let user_exists_id = users_crud::get_id(conn, login_email, login_password)?;

    // Get the current token information if it exists
    let auth = Auth
        .select(AuthTable::as_select())
        .filter(user_id.eq(user_exists_id.clone()))
        .first(conn)
        .optional()?;

    match auth {
        Some(auth) => Ok((auth.user_id, auth.token)),
        None => {
            let new_token = uuid::Uuid::new_v4().to_string();
            let new_expiry = chrono::Utc::now()
                .add(chrono::Duration::hours(1))
                .to_rfc3339();

            insert_into(Auth)
                .values((
                    user_id.eq(user_exists_id.clone()),
                    token.eq(new_token.clone()),
                    expiry.eq(new_expiry),
                ))
                .execute(conn)?;

            Ok((user_exists_id, new_token))
        }
    }
}
