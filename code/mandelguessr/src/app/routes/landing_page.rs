use leptos::{component, create_resource, view, IntoView};
use leptos_router::A;

#[component]
pub fn LandingPage() -> impl IntoView {
    let link_class = "text-blue-700 underline";

    view! {
        <div class="w-full h-40 p-12"> // header
            <img href="/header.png" class="w-full h-full"/>
            <hgroup class="h-full w-full absolute top-0 left-0 flex flex-col items-center justify-center">
                <h1 class="text-8xl">"Mandelguessr"</h1>
                <p>"It's like Geoguessr except it's 💩"</p>
            </hgroup>
        </div>
        // <A href="/register" class=link_class>Registrieren</A>
        // <br/>
        // <A href="/login" class=link_class>Login</A>
        // <br/>
        // <A href="/leaderboard" class=link_class>Rangliste</A>
    }
}