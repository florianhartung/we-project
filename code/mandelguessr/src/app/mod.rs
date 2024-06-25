use std::ops::Range;

use leptos::html::Div;
use leptos::{
    component, create_effect, create_memo, create_node_ref, create_resource, create_rw_signal, create_server_action, provide_context, view, Errors, IntoView, Signal, SignalGet, Transition
};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, SsrMode};
use leptos_use::{use_element_size, UseElementSizeReturn};
use routes::content::Content;
use routes::landing_page::LandingPage;
use routes::login::Login;
use routes::register::Register;

use crate::api;
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


    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/mandelguessr.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| "Route not found".into_view()>
            <main>
                <Routes>
                    <Route path="register" view=move || view! {
                        <Register />
                    }/>
                    <Route path="login" view=move || view! {
                        <Login />
                    }/>
                    <Route path="" view=LandingPage />
                    <Route path="content" view=move || view!{
                        <Content />
                    }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let position = create_rw_signal((0.0, 0.0));
    let zoom_exponent = create_rw_signal(0.0);

    let user = create_server_action::<crate::api::auth::CurrentUserAction>();

    create_effect(move |_| {
        user.dispatch(CurrentUserAction {});
    });

    view! {
        {user.value()}
        <div class="rounded-md w-[800px] h-[600px]">
            <Mandelbrot
                size=(800, 600) position zoom_exponent
                position_bounds=MANDELBROT_POSITION_BOUNDS
                zoom_exponent_bounds=0.0..3.0
                class="rounded-sm shadow-sm"
                />
        </div>
    }
}

const MANDELBROT_POSITION_BOUNDS: (Range<f32>, Range<f32>) = (-2.0..0.5, -1.2..1.2);
