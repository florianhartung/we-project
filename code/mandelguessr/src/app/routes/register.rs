use leptos::{component, create_effect, create_server_action, create_signal, view, Callable, Callback, IntoView, SignalGet};
use leptos_router::{ActionForm, A};

use crate::app::components::common::{SubmitButton, TextInputField};
use crate::api::auth::{SignupAction, SignupResponse};

#[component]
pub fn Register() -> impl IntoView {
    let register_action= create_server_action::<SignupAction>();

    let error_msg = move || {
        match register_action.value().get() {
            Some(Ok(SignupResponse::Error(msg))) => Some(msg),
            Some(Err(_)) => Some("Serverfehler".to_owned()),
            _ => None,
        }
    };

    view! {
        <div class="w-full flex flex-row justify-center">
            <ActionForm class="w-80 p-6 space-y-4" action=register_action>
                <h1 class="text-xl text-white">Registrierung</h1>

                <TextInputField input_name="username" label="Benutername" placeholder="mustermann123" />
                <TextInputField input_name="password" label="Passwort" is_password=true />
                <TextInputField input_name="repeat_password" label="Passwort wiederholen" is_password=true />
                <SubmitButton label="Registrieren" />
                <div class="text-red-600">{error_msg}</div>
                <A class="text-blue-700 underline" href="/login">
                    Zum Login
                </A>
                <br/>
            </ActionForm>
        </div>
    }
}
