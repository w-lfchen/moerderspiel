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
use config::Config;
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject as _},
};

mod api;
mod config;
mod db;
mod gamelogic;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let config = Config::from_env();
    let state = web::Data::new(State::new());
    let server = HttpServer::new(move || {
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
    });

    match (config.cert_path(), config.key_path()) {
        // both certificate and key need to be set for https support
        (Some(cert_path), Some(key_path)) => {
            // ssl setup
            // https://github.com/actix/examples/blob/f0080fc31c20352dfefaa45ca0f7227606da50f3/https-tls/rustls/src/main.rs#L44
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .unwrap();

            // load TLS key/cert files
            let cert_chain = CertificateDer::pem_file_iter(cert_path)
                .unwrap()
                .flatten()
                .collect();

            let key_der = PrivateKeyDer::from_pem_file(key_path)
                .expect("Could not locate PKCS 8 private keys.");

            server.bind_rustls_0_23(
                (config.address(), config.port()),
                ServerConfig::builder()
                    .with_no_client_auth()
                    .with_single_cert(cert_chain, key_der)
                    .unwrap(),
            )
        }
        _ => server.bind((config.address(), config.port())),
    }?
    .run()
    .await
    .map_err(From::from)
}
