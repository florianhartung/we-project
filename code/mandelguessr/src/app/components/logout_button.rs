use leptos::{
    component, create_action, create_effect, html::Button, use_context, view, IntoView, SignalGet,
};
use leptos_router::{use_navigate, ActionForm, Form};

use crate::{api};

#[component]
pub fn LogoutButton() -> impl IntoView {
    let logout_action = create_action(|&()| async move {
            api::auth::logout_action().await
        });

    let navigate = leptos_router::use_navigate();

    // Redirect to landing page when logout was successfull
    let should_redirect = move || logout_action.value().get() == Some(Ok(()));
    create_effect(move |_| {
        if should_redirect() {
            navigate("/", Default::default());
        }
    });
    view! {
        <button class="border-2 border-black rounded-sm" on:click=move |_| {
            logout_action.dispatch(());
        }>
            "Logout"
        </button>
    }
}
