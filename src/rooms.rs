use actix_web::{get, web, HttpResponse};
use serde_json::json;
use crate::{db, routes::DbPool};
use uuid::Uuid;

#[get("/conversations/{uid}")]
pub async fn get_conversation_by_id(
    pool: web::Data<DbPool>,
    uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let room_id = uid.to_owned();
    let conversations = web::block(move || {
        let mut conn = pool.get()?;
        db::get_conversation_by_room_uid(&mut conn, room_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(data) = conversations {
        Ok(HttpResponse::Ok().json(data))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No conversation with room_id: {room_id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/rooms")]
pub async fn get_rooms(
    pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
    let rooms = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_rooms(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if !rooms.is_empty() {
        Ok(HttpResponse::Ok().json(rooms))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No rooms available at the moment.",
            })
            .to_string(),
        );
        Ok(res)
    }
}
