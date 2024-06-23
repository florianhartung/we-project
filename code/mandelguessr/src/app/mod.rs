use std::ops::Range;

use leptos::{component, create_rw_signal, view, Errors, IntoView};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{Route, Router, Routes, SsrMode};

use crate::app::{
    components::{TechStackItem, TechStackList, Mandelbrot},
    error_template::{AppError, ErrorTemplate},
};
use components::{MandelbrotBounds, ServerCounter};

mod components;
mod error_template;

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
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage ssr=SsrMode::PartiallyBlocked/> // use PartiallyBlocked to allow certain resources to still be blocking during SSR. This could be needed for authentication?
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let tech_stack_items = vec![
        TechStackItem::new("Leptos", "https://leptos.dev/"),
        TechStackItem::new("Axum", "https://github.com/tokio-rs/axum"),
        TechStackItem::new("Diesel ORM", "https://diesel.rs/"),
        TechStackItem::new("PostgreSQL", "https://www.postgresql.org/axum"),
        TechStackItem::new("TailwindCSS", "https://tailwindcss.com/"),
    ];

    let position = create_rw_signal((0.0, 0.0));
    let zoom_exponent = create_rw_signal(0.0);

    view! {
        <div class="flex flex-col items-center space-y-8 text-lg py-4">
            <h1 class="text-3xl">"Welcome to my Rust fullstack template!👋"</h1>

            <p class="text-sm w-96 whitespace-normal">
                "This template is quite opinion based.
                It provides the fundamental parts needed for a fullstack webapp with custom components and a database connection.
                A general source code folder structure is also provided."
            </p >

            <div>
                Used technologies
                <TechStackList items=tech_stack_items />
            </div>

            <div class="flex flex-col items-center">
                This is a counter whose state is tracked by the server:
                <ServerCounter />
            </div>

            <div class="rounded-md shadow-sm overflow-hidden">
                <Mandelbrot size=(400, 300) position zoom_exponent position_bounds=MANDELBROT_POSITION_BOUNDS zoom_exponent_bounds=0.0..3.0/>
            </div>
        </div>
    }
}


const MANDELBROT_POSITION_BOUNDS: (Range<f32>, Range<f32>) = (-2.0..0.5, -1.2..1.2);