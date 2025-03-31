use actix_web::{
    HttpResponse, delete, get, post,
    web::{Data, Json, Path},
};
use serde::Deserialize;

use crate::{
    api::GameState,
    gamelogic::{
        game::Game,
        player::{NewPlayerData, PlayerID},
    },
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

/// TODO: authentication of some sort
#[post("/start")]
pub async fn start_game(state: Data<State>) -> HttpResponse {
    let gamestate = &mut *state.gamestate.write().unwrap();
    match gamestate {
        GameState::Setup(game) => {
            // TODO: maybe move this logic somewhere else
            // take the value from the old game state, leaving a new game in its place. this doesn't matter as it is never read
            // due to Vec::new() not allocating, this results in no allocation being made
            let game = std::mem::take(game);
            // start the game and construct a new gamestate from it
            *gamestate = GameState::Running(game.start());
            HttpResponse::Ok().finish()
        }
        GameState::Running(_) => HttpResponse::Conflict().body("game has already started"),
        GameState::Finished(_) => HttpResponse::Conflict().body("game is finished"),
    }
}

#[derive(Debug, Deserialize)]
pub struct KillRequest {
    killer: PlayerID,
    target: PlayerID,
}

/// serialization of [KillRequest] as body
#[post("/kill")]
pub async fn kill(state: Data<State>, Json(kill_request): Json<KillRequest>) -> HttpResponse {
    match &mut *state.gamestate.write().unwrap() {
        GameState::Setup(_) => HttpResponse::Conflict().body("game is in setup phase"),
        GameState::Running(game) => {
            match game.attempt_to_kill(&kill_request.killer, &kill_request.target) {
                Ok(next_target) => {
                    HttpResponse::Ok().body(format!("next target is: {next_target}"))
                }
                // TODO: maybe a different response code is more appropriate?
                Err(e) => HttpResponse::Conflict().body(format!("error during killing: {e}")),
            }
        }
        GameState::Finished(_) => HttpResponse::Conflict().body("game is finished"),
    }
}

#[get("/player/{id}")]
pub async fn get_player_data(path: Path<PlayerID>, state: Data<State>) -> HttpResponse {
    let player = path.into_inner();
    match &*state.gamestate.read().unwrap() {
        // TODO: better response
        GameState::Setup(_) => HttpResponse::Conflict().body("game is in setup phase"),
        GameState::Running(game) => {
            match game.get_player_data(&player) {
                Ok(data) => HttpResponse::Ok().json(data),
                // TODO: maybe a different response code is more appropriate?
                Err(e) => HttpResponse::NotFound().body(format!("something was not found: {e}")),
            }
        }
        GameState::Finished(_) => todo!(),
    }
}

#[get("/player/status/{id}")]
pub async fn get_player_status(path: Path<PlayerID>, state: Data<State>) -> HttpResponse {
    let player = path.into_inner();
    match &*state.gamestate.read().unwrap() {
        GameState::Setup(_) => HttpResponse::Conflict().body("game is in setup phase"),
        GameState::Running(game) => match game.get_status_map(&player) {
            Some(map) => HttpResponse::Ok().json(map),
            None => HttpResponse::NotFound().body("player not found"),
        },
        GameState::Finished(_) => todo!(),
    }
}

/// TODO: authentication of some sort
#[post("/finish")]
pub async fn finish_game(state: Data<State>) -> HttpResponse {
    let gamestate = &mut *state.gamestate.write().unwrap();
    match gamestate {
        GameState::Setup(_) => HttpResponse::Conflict().body("game needs to be started first"),
        GameState::Running(game) => {
            // TODO: maybe move this logic somewhere else
            // basically same logic as starting a game
            let game = std::mem::replace(game, Game::new().start());
            // start the game and construct a new gamestate from it
            *gamestate = GameState::Finished(game.finish());
            HttpResponse::Ok().finish()
        }
        GameState::Finished(_) => HttpResponse::Conflict().body("game is already finished"),
    }
}

/// TODO: authentication of some sort
/// wipe everything, new game
#[post("/reset")]
pub async fn reset_game(state: Data<State>) -> HttpResponse {
    *state.gamestate.write().unwrap() = GameState::new();
    HttpResponse::Ok().finish()
}
