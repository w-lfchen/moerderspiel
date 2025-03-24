use actix_web::{
    HttpResponse, delete, post,
    web::{Data, Json, Path},
};

use crate::{
    api::GameState,
    gamelogic::player::{NewPlayerData, PlayerID},
};

use super::State;

#[post("/player")]
pub async fn add_player(
    state: Data<State>,
    Json(player_data): Json<NewPlayerData>,
) -> HttpResponse {
    match &mut *state.gamestate.write().unwrap() {
        GameState::Setup(game) => HttpResponse::Created().json(game.add_player(player_data)),
        GameState::Running(_) => {
            HttpResponse::Conflict().body("can't add a player to a running game")
        }
        GameState::Finished(_) => {
            HttpResponse::Conflict().body("can't add a player to a finished game")
        }
    }
}

#[delete("/player/{id}")]
pub async fn remove_player(path: Path<PlayerID>, state: Data<State>) -> HttpResponse {
    let id = path.into_inner();
    match &mut *state.gamestate.write().unwrap() {
        GameState::Setup(game) => HttpResponse::Ok().json(game.remove_player(&id)),
        GameState::Running(game) => HttpResponse::Ok().json(game.remove_player(&id)),
        GameState::Finished(_) => {
            HttpResponse::Conflict().body("can't remove a player from a finished game")
        }
    }
}
