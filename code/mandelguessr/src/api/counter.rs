use std::sync::atomic::Ordering;

use leptos::{server, ServerFnError};

#[cfg(feature = "ssr")]
use crate::backend::state::AppState;

#[server(Increment, "/api/counter")]
pub async fn increment() -> Result<u32, ServerFnError> {
    let counter_state = AppState::expect_from_context().counter;

    let previous_value = counter_state.fetch_add(1, Ordering::SeqCst);
    let new_value = previous_value + 1;

    Ok(new_value)
}

#[server(Get, "/api/counter")]
pub async fn get() -> Result<u32, ServerFnError> {
    let counter_state = AppState::expect_from_context().counter;

    let current_value = counter_state.load(Ordering::SeqCst);

    Ok(current_value)
}
