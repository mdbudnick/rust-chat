use actix_web::{get, http::Error, post, web, HttpResponse};
use crate::models;
use serde_json::json;
use crate::routes::DbPool;
use crate::db;
use uuid::Uuid;


#[post("/users/create")]
pub async fn create_user(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewUser>,
) -> Result<HttpResponse, Error> {
    let user_result = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, &form.username)
    })
    .await;

    match user_result {
        Ok(Ok(user)) => Ok(HttpResponse::Ok().json(user)),
        Ok(Err(e)) => {
            eprintln!("Database error: {:?}", e);
            Ok(HttpResponse::UnprocessableEntity().json(json!({
                "error": "Could not create user",
                "message": e.to_string()
            })))
        },
        Err(e) => {
            eprintln!("Blocking error: {:?}", e);
            Ok(HttpResponse::InternalServerError().json(json!({
                "error": "Internal Server Error",
                "message": "An unexpected error occurred"
            })))
        }
    }
}


#[get("/users/{user_id}")]
pub async fn get_user_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_id = id.to_owned();
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::find_user_by_uid(&mut conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No user found with phone: {id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}