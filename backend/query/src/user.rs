// user::new(...)


use password_hash::PasswordHashString;
use uchat_domain::ids::UserId;
use diesel::PgConnection;
use diesel::prelude::*;

use crate::QueryError;

pub fn new<T: AsRef<str>> (
    conn: &mut PgConnection,
    hash: PasswordHashString,
    handle: T,
) -> Result<UserId, QueryError> {
    use crate::schema::users::{self, columns};

    let user_id = UserId::new();

    diesel::insert_into(users::table)
        .values((
            colomns::id.eq(user_id),
            colomns::password_hash.eq(hash.as_str()),
            colomns::handle.eq(handle.as_ref()),
        ))
        .execute(conn)?;
    Ok(user_id)
}