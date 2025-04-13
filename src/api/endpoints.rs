use actix_web::{
    HttpResponse, delete, get, post, put,
    web::{Data, Json, Path},
};
use serde::{Deserialize, Serialize};

use crate::{
    api::GameState,
    gamelogic::{
        NUM_CIRCLES,
        game::{Game, GameID},
        player::{NewPlayerData, Player, PlayerID},
    },
};

use super::State;

#[post("/games/{id}/players")]
pub async fn add_player(
    path: Path<GameID>,
    state: Data<State>,
    Json(player_data): Json<NewPlayerData>,
) -> HttpResponse {
    if let Some(game_state) = state.games.write().unwrap().get_mut(&path.into_inner()) {
        match game_state {
            GameState::Setup(game) => HttpResponse::Created().json(game.add_player(player_data)),
            GameState::Running(_) => {
                HttpResponse::Conflict().body("can't add a player to a running game")
            }
            GameState::Finished(_) => {
                HttpResponse::Conflict().body("can't add a player to a finished game")
            }
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

#[delete("/games/{game_id}/players/{player_id}")]
pub async fn remove_player(path: Path<(GameID, PlayerID)>, state: Data<State>) -> HttpResponse {
    let (game_id, player_id) = path.into_inner();
    if let Some(game_state) = state.games.write().unwrap().get_mut(&game_id) {
        match game_state {
            GameState::Setup(game) => {
                if let Some(player) = game.remove_player(&player_id) {
                    HttpResponse::Ok().json(player)
                } else {
                    HttpResponse::NotFound().body("player not found")
                }
            }
            GameState::Running(game) => {
                if let Some(player) = game.remove_player(&player_id) {
                    HttpResponse::Ok().json(player)
                } else {
                    HttpResponse::NotFound().body("player not found")
                }
            }
            GameState::Finished(_) => {
                HttpResponse::Conflict().body("can't remove a player from a finished game")
            }
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

/// TODO: authentication of some sort
#[post("/games/{id}/start")]
pub async fn start_game(path: Path<GameID>, state: Data<State>) -> HttpResponse {
    if let Some(game_state) = state.games.write().unwrap().get_mut(&path.into_inner()) {
        match game_state {
            GameState::Setup(game) => {
                // TODO: maybe move this logic somewhere else
                // take the value from the old game state, leaving a new game in its place. this doesn't matter as it is never read
                // due to Vec::new() not allocating, this results in no allocation being made
                let game = std::mem::take(game);
                // start the game and construct a new gamestate from it
                *game_state = GameState::Running(game.start());
                HttpResponse::Ok().finish()
            }
            GameState::Running(_) => HttpResponse::Conflict().body("game has already started"),
            GameState::Finished(_) => HttpResponse::Conflict().body("game is finished"),
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

#[derive(Debug, Deserialize)]
pub struct KillRequest {
    killer: PlayerID,
    target: PlayerID,
}

/// serialization of [KillRequest] as body
#[post("/games/{id}/kill")]
pub async fn kill(
    path: Path<GameID>,
    state: Data<State>,
    Json(kill_request): Json<KillRequest>,
) -> HttpResponse {
    if let Some(game_state) = state.games.write().unwrap().get_mut(&path.into_inner()) {
        match game_state {
            GameState::Setup(_) => HttpResponse::Conflict().body("game is in setup phase"),
            GameState::Running(game) => {
                match game.attempt_to_kill(&kill_request.killer, &kill_request.target) {
                    Ok(next_target) => HttpResponse::Ok().body(next_target.to_string()),
                    // TODO: maybe a different response code is more appropriate?
                    Err(e) => HttpResponse::Conflict().body(format!("error during killing: {e}")),
                }
            }
            GameState::Finished(_) => HttpResponse::Conflict().body("game is finished"),
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

#[get("/games/{game_id}/players/{player_id}")]
pub async fn get_player_data(path: Path<(GameID, PlayerID)>, state: Data<State>) -> HttpResponse {
    let (game_id, player_id) = path.into_inner();
    if let Some(game_state) = state.games.read().unwrap().get(&game_id) {
        match game_state {
            // TODO: better responses
            GameState::Setup(_) => HttpResponse::Conflict().body("game is in setup phase"),
            GameState::Running(game) => {
                match game.get_player_data(&player_id) {
                    Ok(data) => HttpResponse::Ok().json(data),
                    // TODO: maybe a different response code is more appropriate?
                    Err(e) => {
                        HttpResponse::NotFound().body(format!("something was not found: {e}"))
                    }
                }
            }
            GameState::Finished(_) => todo!(),
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

#[get("/games/{game_id}/players/players/{player_id}/status")]
pub async fn get_player_status(path: Path<(GameID, PlayerID)>, state: Data<State>) -> HttpResponse {
    let (game_id, player_id) = path.into_inner();
    if let Some(game_state) = state.games.read().unwrap().get(&game_id) {
        match game_state {
            GameState::Setup(_) => HttpResponse::Conflict().body("game is in setup phase"),
            GameState::Running(game) => match game.get_status_map(&player_id) {
                Some(map) => HttpResponse::Ok().json(map),
                None => HttpResponse::NotFound().body("player not found"),
            },
            GameState::Finished(_) => todo!(),
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

/// TODO: authentication of some sort
#[post("/games/{id}/finish")]
pub async fn finish_game(path: Path<GameID>, state: Data<State>) -> HttpResponse {
    if let Some(game_state) = state.games.write().unwrap().get_mut(&path.into_inner()) {
        match game_state {
            GameState::Setup(_) => HttpResponse::Conflict().body("game needs to be started first"),
            GameState::Running(game) => {
                // TODO: maybe move this logic somewhere else
                // basically same logic as starting a game
                let game = std::mem::replace(game, Game::new(String::new()).start());
                // start the game and construct a new gamestate from it
                *game_state = GameState::Finished(game.finish());
                HttpResponse::Ok().finish()
            }
            GameState::Finished(_) => HttpResponse::Conflict().body("game is already finished"),
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

/// TODO: authentication of some sort
/// wipe everything, new game
#[post("/games/{id}/reset")]
pub async fn reset_game(path: Path<GameID>, state: Data<State>) -> HttpResponse {
    if let Some(old_gamestate) = state.games.write().unwrap().get_mut(&path.into_inner()) {
        *old_gamestate =
            GameState::new_with_id(*old_gamestate.id(), String::from(old_gamestate.name()));
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

/// struct for outgoing data about a game
#[derive(Debug, Serialize)]
pub struct GameData<'a> {
    num_circles: usize,
    phase: &'a str,
    players: Vec<&'a Player>,
}

#[get("/games/{id}/data")]
pub async fn game_data(path: Path<GameID>, state: Data<State>) -> HttpResponse {
    if let Some(game_state) = state.games.read().unwrap().get(&path.into_inner()) {
        match game_state {
            GameState::Setup(game) => HttpResponse::Ok().json(GameData {
                num_circles: NUM_CIRCLES,
                phase: "setup",
                players: game.players().collect(),
            }),
            GameState::Running(game) => HttpResponse::Ok().json(GameData {
                num_circles: NUM_CIRCLES,
                phase: "running",
                players: game.players().collect(),
            }),
            GameState::Finished(_) => HttpResponse::Ok().json(GameData {
                num_circles: NUM_CIRCLES,
                phase: "finished",
                players: vec![], // TODO
            }),
        }
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

/// simply serialize the game in order to export a representation
#[get("/games/{id}/export")]
pub async fn export_game(path: Path<GameID>, state: Data<State>) -> HttpResponse {
    if let Some(game_state) = state.games.read().unwrap().get(&path.into_inner()) {
        HttpResponse::Ok().json(game_state)
    } else {
        HttpResponse::NotFound().body("game not found")
    }
}

/// TODO: authentication
/// overrides the game if present
#[put("/games/import")]
pub async fn import_game(state: Data<State>, Json(game_state): Json<GameState>) -> HttpResponse {
    state
        .games
        .write()
        .unwrap()
        .insert(*game_state.id(), game_state);
    HttpResponse::Ok().finish()
}
