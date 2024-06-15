use std::sync::{atomic::AtomicU32, Arc};

use axum::extract::FromRef;
use leptos::{expect_context, use_context, LeptosOptions};
use leptos_router::RouteListing;

#[derive(Debug, Clone, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub counter: Arc<AtomicU32>,
    pub routes: Vec<RouteListing>,
}

impl AppState {
    pub fn use_from_context() -> Option<Self> {
        use_context()
    }

    pub fn expect_from_context() -> Self {
        expect_context()
    }
}
