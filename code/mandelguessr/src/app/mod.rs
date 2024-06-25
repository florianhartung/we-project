use std::ops::Range;

use components::LogoutButton;
use leptos::html::Div;
use leptos::{
    component, create_effect, create_memo, create_node_ref, create_resource, create_rw_signal,
    create_server_action, create_signal, provide_context, view, Errors, IntoView, Signal,
    SignalGet, Suspense, Transition,
};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, SsrMode, A};
use leptos_use::{use_element_size, UseElementSizeReturn};
use routes::content::Content;
use routes::landing_page::LandingPage;
use routes::leaderboard::Leaderboard;
use routes::login::Login;
use routes::register::Register;

use crate::api;
use crate::api::auth::current_user;
use crate::{
    api::auth::CurrentUserAction,
    app::error_template::{AppError, ErrorTemplate},
};
use components::common::{Mandelbrot, MandelbrotBounds};

mod components;
mod error_template;
mod routes;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let user = create_resource(|| (), |()| async { current_user().await });

    let refetch_user = move || user.refetch();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mandelguessr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| "Route not found".into_view()>
            <nav class="w-full bg-[#232323] flex flex-row justify-between items-center">
                <A href="/" class="flex flex-row items-center justify-start p-2 space-x-2">
                    <img src="/icon.png" class="h-14 rounded-full"/>
                    <div class="text-white font-bold text-3xl">MandelGuessr</div>
                </A>
                {move ||
                    user.map(move |x| if x.is_ok() {
                                view! {
                                    <A href="/content">
                                        <button class="text-white bg-[#1AA404] rounded-full text-xl font-bold px-4 py-2">Spiel starten</button>
                                    </A>
                                }.into_view()
                        }else {
                            view! {}.into_view()
                        }
                    )
                }
                <Suspense fallback=|| view!{<div class="text-white font-bold text-3xl text-end">"Lade Benutzerdaten..."</div>}>
                    {
                        move || user.get()
                            .map(|user| user
                                .map(|user| view! {
                                    <div class="flex flex-row items-center justify-end p-2 space-x-2 text-white font-bold text-2xl">
                                        <div class="text-end">
                                            Angemeldet als<br/>
                                            {user}
                                        </div>

                                        <div class="font-regular text-xl"><LogoutButton on_click=move |_| refetch_user()/></div>
                                    </div>
                                })
                            .unwrap_or_else(move |_| view!{
                                <div class="text-white font-bold text-xl h-14 py-0.5 px-2 text-end">
                                    <button on:click=|_| leptos_router::use_navigate()("/login", Default::default()) class="bg-[#600070] rounded-full h-full py-1 px-8">
                                        Anmelden
                                    </button>
                                </div>
                            }))
                    }
                </Suspense>
            </nav>
            <main class="bg-[#383842] h-full">
                <Routes>
                    <Route path="register" view=move || {
                        create_effect(move |_| {
                            if let Some(Ok(_)) = user.get() {
                                leptos_router::use_navigate()("/content", Default::default());
                            }
                        });
                        view! { <Register /> }
                    }/>
                    <Route path="login" view=move || {
                        create_effect(move |_| {
                            if let Some(Ok(_)) = user.get() {
                                leptos_router::use_navigate()("/content", Default::default());
                            }
                        });
                        view! { <Login /> }
                    }/>
                    <Route path="" view=move || {
                        create_effect(move |_| {
                            user.refetch();
                        });
                        view!{ <LandingPage /> }
                    }/>
                    <Route path="content" view=move || {
                        create_effect(move |_| {
                            user.refetch();
                        });
                        view!{ <Content /> }
                    }/>
                    <Route path="leaderboard" view=move || {
                        create_effect(move |_| {
                            user.refetch();
                        });
                        view!{ <Leaderboard /> }
                    }/>
                </Routes>
            </main>
        </Router>
    }
}