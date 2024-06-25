use std::thread::current;

use leptos::{
    component, create_blocking_resource, create_effect, create_local_resource, create_render_effect, create_resource, html::A, view, ErrorBoundary, IntoView, ServerFnError, SignalGet, Suspense
};
use leptos_router::*;

use crate::{api, app::components::LogoutButton};
#[component]
pub fn Content() -> impl IntoView {
    let username_resource =
        create_blocking_resource(|| (), |()| async { api::auth::current_user().await });

    view! {
        <ErrorBoundary fallback = move |_| {
            leptos_router::use_navigate()("/login", Default::default());
            view! {"Du bist nicht angemeldet, Weiterleitung zur Loginseite..."}
        }>
            <Suspense fallback=move || view!{"Loading user data"}>
                "your name is " {username_resource}
                <br/>
                <LogoutButton />
                <br/>
                "todo content"
            </Suspense>
        </ErrorBoundary>
    }
}
