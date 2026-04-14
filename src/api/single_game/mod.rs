//! single game instance apis

use actix_web::{
    HttpResponse,
    error::{ErrorInternalServerError, ErrorUnauthorized},
    get, post,
    web::{self, Data, Json},
};
use diesel::{BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl};
use diesel_async::{RunQueryDsl, pooled_connection::bb8::PooledConnection};
use rand::seq::SliceRandom;
use serde_json::json;

use crate::{
    api::single_game::logic::assert_phase,
    db::{
        models::{Game, NewOrder, Order, Phase, Player, PlayerData},
        pool::{Pool, SqliteConnWrapped},
        schema::{games, kills, orders, players},
    },
    gamelogic::NUM_CIRCLES,
};

mod logic;

/// small helper to get either a connection or an internal server error
async fn get_conn(
    pool: &'_ Pool,
) -> Result<PooledConnection<'_, SqliteConnWrapped>, actix_web::error::Error> {
    pool.get().await.map_err(ErrorInternalServerError)
}

/// create player
#[post("/game/player")]
pub async fn game_player_add(
    pool: Data<Pool>,
    Json(player_data): Json<PlayerData>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = get_conn(&pool).await?;
    assert_phase(&mut conn, Phase::Setup).await?;
    if !players::table
        .filter(
            players::name
                .eq(&player_data.name)
                .and(players::variant.eq(player_data.variant)),
        )
        .load::<Player>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .is_empty()
    {
        return Ok(HttpResponse::Conflict().body(
            "player with name and variant already exists, please choose a different variant",
        ));
    }
    // data race leading to duplicate player entry can be ignored for now
    let player_data = web::block(|| player_data.hash_password_in_place())
        .await?
        .map_err(ErrorInternalServerError)?;
    Ok(
        match diesel::insert_into(players::table)
            .values(player_data)
            .returning((players::id, players::name, players::variant))
            .get_result::<(i32, String, i32)>(&mut conn)
            .await
        {
            Ok((id, name, variant)) => HttpResponse::Ok().json(json! ({
                "id": id,
                "name": name,
                "variant": variant,
            })),
            Err(_) => HttpResponse::InternalServerError().finish(),
        },
    )
}

/// player state
#[get("/game/player")]
pub async fn game_player_status(
    pool: Data<Pool>,
    Json(player_data): Json<PlayerData>,
) -> actix_web::Result<HttpResponse> {
    let mut conn = get_conn(&pool).await?;
    // (name, variant) should be unique so we don't need to check for duplicates
    let player = players::table
        .filter(
            players::name
                .eq(&player_data.name)
                .and(players::variant.eq(player_data.variant)),
        )
        .get_result::<Player>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    web::block(move || {
        logic::verify_password(&player.hashed_password.clone(), player_data.hashed_password)
    })
    .await?
    .map_err(ErrorUnauthorized)?;
    // password was correct
    // might be able to improve these queries but need something that works now
    let orders = orders::table
        .inner_join(players::table.on(orders::executor.eq(players::id)))
        .filter(orders::target.eq(player.id))
        .get_results::<(Order, Player)>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?
        .iter()
        .map(|(order, target)| {
            json!({
                "id": order.id,
                "circle": order.circle,
                "target_id": target.id,
                "target_name": target.name,
                "target_variant": target.variant,
            })
        })
        .collect::<Vec<_>>();
    let kills = kills::table
        .filter(kills::killer.eq(player.id))
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json!({
        "id": player.id,
        "name": player.name,
        "variant": player.variant,
        "num_kills": kills,
        "orders": orders,
    })))
}

/// game info
#[get("/game")]
pub async fn game_status(pool: Data<Pool>) -> actix_web::Result<HttpResponse> {
    let mut conn = get_conn(&pool).await?;
    // no assertions about number of games
    let game = games::table
        .first::<Game>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    let num_players = players::table
        .count()
        .get_result::<i64>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(json!({
        "num_players": num_players,
        "phase": game.phase,
        "num_circles": NUM_CIRCLES,
    })))
}

#[post("/game/start")]
pub async fn game_start(pool: Data<Pool>) -> actix_web::Result<HttpResponse> {
    let mut conn = get_conn(&pool).await?;
    assert_phase(&mut conn, Phase::Setup).await?;
    // this should be a critical section
    let mut players = players::table
        .get_results::<Player>(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    let orders = (0..NUM_CIRCLES).flat_map(|circle_index| {
        players.shuffle(&mut rand::rng());
        (0..players.len())
            .map(|player_index| NewOrder {
                circle: circle_index as i32,
                executor: players[player_index].id,
                target: players[(player_index + 1) % players.len()].id,
            })
            .collect::<Vec<_>>()
    });
    // we are running into https://github.com/diesel-rs/diesel/issues/4989
    // -> no batch insert :(
    /*
    diesel::insert_into(orders::table)
        .values(&orders)
        .execute(&mut conn)
        .await
        .unwrap();
    */
    for order in orders {
        diesel::insert_into(orders::table)
            .values(order)
            .execute(&mut conn)
            .await
            .map_err(ErrorInternalServerError)?;
    }

    // no assertions about number of games, this sets every game's phase to running
    diesel::update(games::table)
        .set(games::phase.eq(Phase::Running))
        .execute(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("game started"))
}

#[post("/game/finish")]
pub async fn game_finish(pool: Data<Pool>) -> actix_web::Result<HttpResponse> {
    let mut conn = get_conn(&pool).await?;
    // no assertions about number of games, this sets every game's phase to 1
    assert_phase(&mut conn, Phase::Running).await?;
    diesel::update(games::table)
        .set(games::phase.eq(Phase::Finished))
        .execute(&mut conn)
        .await
        .map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body("game finished"))
}
