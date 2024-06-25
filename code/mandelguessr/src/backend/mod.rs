use std::{clone, env, str::FromStr, sync::Arc};

use crate::{app::App, backend::state::AppState};
use axum::Router;
use diesel::{r2d2::ConnectionManager, PgConnection};
use fileserv::file_and_error_handler;
use leptos::{get_configuration, provide_context};
use leptos_axum::{generate_route_list, LeptosRoutes};
use tokio::net::TcpListener;
use tower_http::catch_panic::CatchPanicLayer;
use tracing::level_filters::LevelFilter;

pub mod database;
pub mod state;
mod fileserv;

pub async fn start_server() {
    setup_logging();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Build state
    let app_state = AppState {
        leptos_options,
        counter: Arc::default(),
        routes,
        database: Arc::new(database::create_connection_pool()),
    };

    // Build router
    let cloned_app_state = app_state.clone();
    let router = Router::new()
        .leptos_routes_with_context(
            &app_state,
            app_state.routes.clone(),
            move || {
                provide_context(cloned_app_state.clone());
            },
            App,
        )
        .fallback(|state, request| file_and_error_handler(state, request, App))
        .layer(CatchPanicLayer::new())
        .with_state(app_state);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}

fn setup_logging() {
    let level_filter = env::var("RUST_LOG")
        .map(|level| {
            LevelFilter::from_str(&level).expect(&format!("Invalid RUST_LOG value `{level}`"))
        })
        .unwrap_or(LevelFilter::INFO);

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(level_filter)
        .init();
}
