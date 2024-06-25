use std::borrow::Cow;

use leptos::{
    component, create_action, create_effect, create_resource, create_server_action, create_signal, Callable, server, view, Callback, ErrorBoundary, IntoView, MaybeProp, MaybeSignal, ServerFnError, SignalGet, TextProp
};
use leptos_router::{ActionForm, A};

use crate::api::auth::{self, login_action, LoginAction, LoginResponse};
use crate::app::components::common::{SubmitButton, TextInputField};

#[cfg(feature = "ssr")]
use leptos::logging::log;

#[component]
pub fn Login() -> impl IntoView {
    let login_action: leptos::Action<LoginAction, Result<LoginResponse, ServerFnError>> = create_server_action::<LoginAction>();

    let error_msg = move || {
        match login_action.value().get() {
            Some(Ok(LoginResponse::IncorrectUserdata(_))) => Some("Falscher Benutzername oder Passwort"),
            Some(Ok(LoginResponse::InvalidUserdata(_))) => Some("Das Passwort muss min. 8 Zeichen lang sein"),
            Some(Err(e)) => Some("Serverfehler"),
            _ => None,
        }
    };


    view! {

        // center form horizontally
        <div class="w-full flex flex-row justify-center">

            <ActionForm class="w-80 p-6 space-y-4" action=login_action>
                <h1 class="text-xl text-white">Anmeldung</h1>

                <TextInputField
                    input_name="username"
                    label="Benutzername"
                    placeholder="mustermann123"
                />
                <TextInputField input_name="password" label="Passwort" is_password=true/>
                <SubmitButton label="Anmelden" />
                <div class="text-red-600">{error_msg}</div>
                <A class="text-blue-700 underline" href="/register">
                    Zur Registrierung
                </A>
                <br/>
            </ActionForm>
        </div>
    }
}
