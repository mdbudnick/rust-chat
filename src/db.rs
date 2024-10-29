use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    time::SystemTime,
};
use uuid::Uuid;
use crate::{models::{Conversation, NewConversation, Room, RoomResponse, User}, schema::conversations, server::Message};
type DbError = Box<dyn std::error::Error + Send + Sync>;

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

pub fn insert_new_user(conn: &mut SqliteConnection, nm: &str, _pn: &str) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;
    let new_user = User {
        id: Uuid::new_v4().to_string(),
        username: nm.to_owned(),
        created_at: iso_date(),
    };
    diesel::insert_into(users).values(&new_user).execute(conn)?;
    Ok(new_user)
}

pub fn insert_new_conversation(
    conn: &mut SqliteConnection,
    new: NewConversation,
) -> Result<Conversation, DbError> {
    use crate::schema::conversations::dsl::*;
    let new_conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        user_id: new.user_id,
        room_id: new.room_id,
        content: new.message,
        created_at: iso_date(),
    };
    diesel::insert_into(conversations)
        .values(&new_conversation)
        .execute(conn)?;
    Ok(new_conversation)
}

pub fn get_all_rooms(conn: &mut SqliteConnection) -> Result<Vec<RoomResponse>, DbError> {
    use crate::schema::rooms;
    use crate::schema::users;
    let rooms_data: Vec<Room> = rooms::table.get_results(conn)?;
    let mut ids = HashSet::new();
    let mut rooms_map = HashMap::new();
    let data = rooms_data.to_vec();
    for room in &data {
        let user_ids = room
            .participant_ids
            .split(",")
            .into_iter()
            .collect::<Vec<_>>();
        for id in user_ids.to_vec() {
            ids.insert(id.to_string());
        }
        rooms_map.insert(room.id.to_string(), user_ids.to_vec());
    }
    let ids = ids.into_iter().collect::<Vec<_>>();
    let users_data: Vec<User> = users::table
        .filter(users::id.eq_any(ids))
        .get_results(conn)?;
    let users_map: HashMap<String, User> = HashMap::from_iter(
        users_data
            .into_iter()
            .map(|item| (item.id.to_string(), item)),
    );
    let response_rooms = rooms_data.into_iter().map(|room| {
        let users = rooms_map
            .get(&room.id.to_string())
            .unwrap()
            .into_iter()
            .map(|id| users_map.get(id.to_owned()).unwrap().clone())
            .collect::<Vec<_>>();
        return RoomResponse{ room, users };
    }).collect::<Vec<_>>();
    Ok(response_rooms)
}


pub fn get_conversation_by_room_uid(
    conn: &mut SqliteConnection,
    room_id: &str
) -> Result<Option<RoomResponse>, DbError> {
    use crate::schema::rooms;
    use crate::schema::users;
    use crate::schema::conversations;
    // Get the room data
    let room: Room = rooms::table
        .filter(rooms::id.eq(room_id))
        .first(conn)?;

    // Get user IDs from the room
    let user_ids: Vec<String> = room.participant_ids
        .split(",")
        .map(|id| id.to_string())
        .collect();

    // Get user data
    let users_data: Vec<User> = users::table
        .filter(users::id.eq_any(&user_ids))
        .get_results(conn)?;

    // Create a map of user IDs to User objects
    let users_map: HashMap<String, User> = users_data
        .into_iter()
        .map(|user| (user.id.to_string(), user))
        .collect();

    // Get messages for the room
    let messages: Vec<Conversation> = conversations::table
        .filter(conversations::room_id.eq(room_id))
        .order(conversations::created_at.asc())
        .get_results(conn)?;

    // Create the RoomResponse
    let response = RoomResponse {
        room,
        users: user_ids.into_iter()
            .filter_map(|id| users_map.get(&id).cloned())
            .collect(),
        messages,
    };

    Ok(Some(response))
}
