//! single game instance apis

use actix_web::{
    HttpResponse,
    error::ErrorInternalServerError,
    post,
    web::{self, Data, Json},
};
use diesel_async::RunQueryDsl;

use crate::{
    CONFIG,
    db::{models::NewPlayer, pool::Pool, schema::players},
};

/// create player
#[post("/game/player")]
pub async fn add_player(
    pool: Data<Pool>,
    Json(player_data): Json<NewPlayer>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = pool.get().await.map_err(ErrorInternalServerError)?;
    let hashed_password =
        web::block(|| hash_password((player_data.hashed_password + CONFIG.pepper()).as_bytes()))
            .await?
            .map_err(ErrorInternalServerError)?;
    Ok(
        match diesel::insert_into(players::table)
            .values(NewPlayer {
                hashed_password,
                ..player_data
            })
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

/// hash an hashed and then peppered pw again
fn hash_password(input: &[u8]) -> Result<String, anyhow::Error> {
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };
    let salt = SaltString::generate(&mut OsRng);
    // default parameters should be fine
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(input, &salt)?.to_string();
    Ok(password_hash)
}
