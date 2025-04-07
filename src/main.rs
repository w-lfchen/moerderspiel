use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use api::{
    GameState, State,
    endpoints::{
        add_player, export_game, finish_game, game_data, get_player_data, get_player_status,
        import_game, kill, remove_player, reset_game, start_game,
    },
};

mod api;
mod gamelogic;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State::new(GameState::new()));

    HttpServer::new(move || {
        App::new()
            // TODO: :warning: DO NOT SHIP :warning:
            .wrap(Cors::permissive())
            .app_data(state.clone())
            .service(add_player)
            .service(remove_player)
            .service(start_game)
            .service(kill)
            .service(get_player_data)
            .service(get_player_status)
            .service(finish_game)
            .service(reset_game)
            .service(game_data)
            .service(export_game)
            .service(import_game)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
