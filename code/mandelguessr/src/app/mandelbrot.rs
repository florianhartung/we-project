use leptos::ev::{MouseEvent, WheelEvent};
use leptos::{
    component, create_action, create_effect, create_local_resource, create_multi_action,
    create_node_ref, create_rw_signal, create_server_action, html::Canvas, view, IntoView,
    RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith,
};
use leptos::{create_signal, spawn_local};
use mandelbrot_renderer::MandelbrotExplorer;
use std::{ops::Deref, time::Duration};

#[component]
pub fn Mandelbrot() -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    let mandelbrot: RwSignal<Option<MandelbrotExplorer>> = create_rw_signal(None);

    // Create renderer
    #[cfg(target_arch = "wasm32")] // This is needed (idk why??)
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            leptos::logging::log!("spawing local future");
            spawn_local(async move {
                leptos::logging::log!("spawing local future");
                let canvas = canvas.deref().clone();
                let mut new_mandelbrot = MandelbrotExplorer::new_from_canvas((800, 600), canvas)
                    .await
                    .unwrap();

                new_mandelbrot.render();

                mandelbrot.set(Some(new_mandelbrot));
                leptos::logging::log!("set mandelbrot state");
            });
        } else {
            leptos::logging::log!("canvas ref not set yet");
        }
    });

    // "Game loop"
    create_effect(move |_| {
        let _handle = leptos::set_interval_with_handle(
            move || {
                mandelbrot.update(move |mandelbrot| {
                    if let Some(mandelbrot) = mandelbrot {
                        mandelbrot.render();
                    }
                })
            },
            Duration::from_millis(16),
        )
        .unwrap();
    });


    view! {
        <canvas ref=canvas_ref
        on:wheel=move |event: WheelEvent| {
            mandelbrot.update(|mandelbrot| {
                if let Some(mandelbrot) = mandelbrot {
                    mandelbrot.on_mouse_wheel(-event.delta_y() as f32 / 60.0);
                }
            })
        }
        on:mousedown=move |event: MouseEvent| {
            if event.button() == 0 { // Main button
                mandelbrot.update(|mandelbrot| {
                    if let Some(mandelbrot) = mandelbrot {
                        mandelbrot.on_mouse_click(true);
                    }
                })
            }
        }
        on:mouseup=move |event: MouseEvent| {
            if event.button() == 0 { // Main button
                mandelbrot.update(|mandelbrot| {
                    if let Some(mandelbrot) = mandelbrot {
                        mandelbrot.on_mouse_click(false);
                    }
                })
            }
        }
        on:mousemove=move |event: MouseEvent| {
            mandelbrot.update(|mandelbrot| {
                if let Some(mandelbrot) = mandelbrot {
                    mandelbrot.on_mouse_move((event.client_x() as f32, event.client_y() as f32));
                }
            })
        }
        width="800px" height="600px"> </canvas>

        <button on:click=move |_| { }>
            Render
        </button>
    }
}
