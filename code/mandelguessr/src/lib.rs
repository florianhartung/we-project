#[cfg(feature = "ssr")]
pub mod backend;

pub mod api;
pub mod app;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    leptos::mount_to_body(App);
}
