use std::ops::Range;

use leptos::{component, create_memo, create_node_ref, create_rw_signal, view, Errors, IntoView, Signal, SignalGet};
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_use::{use_element_size, UseElementSizeReturn};
use leptos::html::Div;
use leptos_router::{Route, Router, Routes, SsrMode};

use crate::app::error_template::{AppError, ErrorTemplate};
use components::common::{Mandelbrot, MandelbrotBounds};

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
    let position = create_rw_signal((0.0, 0.0));
    let zoom_exponent = create_rw_signal(0.0);


    view! {
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