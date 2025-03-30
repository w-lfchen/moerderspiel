use actix_web::{App, HttpServer, web};
use api::{
    GameState, State,
    endpoints::{
        add_player, get_player_data, get_player_status, kill, remove_player, reset_game, start_game,
    },
};

mod api;
mod gamelogic;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State::new(GameState::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(add_player)
            .service(remove_player)
            .service(start_game)
            .service(kill)
            .service(get_player_data)
            .service(get_player_status)
            .service(reset_game)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
