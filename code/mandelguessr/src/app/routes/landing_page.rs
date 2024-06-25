use leptos::{component, IntoView, view};
use leptos_router::A;

#[component]
pub fn LandingPage() -> impl IntoView {
    let link_class = "text-blue-700 underline";

    view! {
        <hgroup>
            <h1 class="text-8xl">"Mandelguessr"</h1>
            <p>"It's like Geoguessr except it's 💩"</p>
        </hgroup>
        <A href="/register" class=link_class>Registrieren</A>
        <br/>
        <A href="/login" class=link_class>Login</A>
        <br/>
        <A href="/docs" class=link_class>Dokumentation</A>
    }
}