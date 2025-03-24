use actix_web::{
    HttpResponse, post,
    web::{Data, Json},
};

use crate::{api::GameState, gamelogic::player::NewPlayerData};

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
