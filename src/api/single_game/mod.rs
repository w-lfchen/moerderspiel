//! single game instance apis

use actix_web::{
    HttpResponse,
    error::ErrorInternalServerError,
    post,
    web::{Data, Json},
};
use diesel_async::RunQueryDsl;

use crate::db::{models::NewPlayer, pool::Pool, schema::players};

/// create player
#[post("/game/player")]
pub async fn add_player(
    pool: Data<Pool>,
    Json(player_data): Json<NewPlayer>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = pool.get().await.map_err(ErrorInternalServerError)?;
    Ok(
        // TODO: hash password before storing into db
        match diesel::insert_into(players::table)
            .values(player_data)
            .returning((players::id, players::name, players::variant))
            .get_result::<(i32, String, i32)>(&mut conn)
            .await
        {
            Ok((id, name, variant)) => HttpResponse::Ok().json(serde_json::json! ({
                "id": id,
                "name": name,
                "variant": variant,
            })),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
    )
}
