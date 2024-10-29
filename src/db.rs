use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};
use uuid::Uuid;
use crate::models::{Conversation, NewConversation, Room, RoomResponse, User};
type DbError = Box<dyn std::error::Error + Send + Sync>;

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str, pn: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        created_at: iso_date(),
    };
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    Ok(new_user)
}