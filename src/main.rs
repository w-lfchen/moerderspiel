use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use api::{
    State,
    endpoints::{
        add_player, create_game, delete_game, export_game, finish_game, get_game_data,
        get_game_data_all, get_player_data, get_player_status, import_game, kill, remove_player,
        reset_game, start_game,
    },
};
use openssl::ssl::{SslAcceptor, SslMethod};

mod api;
mod gamelogic;

// TODO: make configurable
const CERT_PATH: &str = "certs/cert.pem";
const KEY_PATH: &str = "certs/key.pem";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let state = web::Data::new(State::new());
    // ssl setup
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(KEY_PATH, openssl::ssl::SslFiletype::PEM)?;
    builder.set_certificate_chain_file(CERT_PATH)?;

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
            .service(get_game_data)
            .service(export_game)
            .service(import_game)
            .service(create_game)
            .service(get_game_data_all)
            .service(delete_game)
    })
    .bind_openssl(("127.0.0.1", 8080), builder)?
    .run()
    .await
    .map_err(From::from)
}
