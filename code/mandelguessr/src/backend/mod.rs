use std::sync::Arc;

use crate::{app::App, backend::state::AppState};
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::get_configuration;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tokio::net::TcpListener;
use tower_http::catch_panic::CatchPanicLayer;

mod fileserv;
pub mod state;

pub async fn start_server() {
    use tracing::level_filters::LevelFilter;

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .init();

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
    };

    // Build router
    let router = Router::new()
        .leptos_routes(&app_state, app_state.routes.clone(), App)
        .fallback(|state, request| file_and_error_handler(state, request, App))
        .layer(CatchPanicLayer::new())
        .with_state(app_state);

    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}