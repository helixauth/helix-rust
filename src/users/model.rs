use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::api_error::ApiError;
use crate::database;
use crate::schema::users;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub email: String,
}

impl User {
    pub fn get(id: String, conn: database::Connection) -> Result<Self, ApiError> {
        let user = users::table
            .filter(users::id.eq(id))
            .first(&conn)?;
        Ok(user)
    }

    pub fn create(user: UserMessage, conn: database::Connection) -> Result<Self, ApiError> {
        let user = User::from(user);
        let user = diesel::insert_into(users::table)
            .values(user)
            .get_result(&conn)?;
        Ok(user)
    }
}

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserMessage {
    pub email: String,
}


impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            email: user.email,
        }
    }
}
