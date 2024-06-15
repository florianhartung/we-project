use leptos::ev::{MouseEvent, WheelEvent};
use leptos::{
    component, create_action, create_effect, create_local_resource, create_multi_action,
    create_node_ref, create_rw_signal, create_server_action, html::Canvas, view, IntoView,
    RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith,
};
use leptos::{create_signal, spawn_local, MaybeSignal};
use mandelbrot_renderer::MandelbrotRenderer;
use std::{ops::Deref, time::Duration};

const SCROLL_SENSITIVITY: f32 = 0.02;
const MOVE_SENSITIVITY: f32 = 0.7;

#[component]
pub fn Mandelbrot(#[prop(into)] size: MaybeSignal<(u32, u32)>) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    let mandelbrot: RwSignal<Option<MandelbrotRenderer>> = create_rw_signal(None);

    // Camera signals
    let (is_mouse_down, set_is_mouse_down) = create_signal::<bool>(false);
    let (previous_mouse_position, set_previous_mouse_position) = create_signal::<Option<(f32, f32)>>(None);
    let (camera_position, set_camera_position) = create_signal::<(f32, f32)>((0.0, 0.0));
    let (camera_size_exponent, set_camera_size_exponent) = create_signal::<f32>(0.0);

    let camera_size = move || {
        let camera_size = 1.1_f32.powf(camera_size_exponent.get());
        let size = size.get();

        (camera_size, camera_size * size.1 as f32/ size.0 as f32)
    };

    // Create renderer
    #[cfg(target_arch = "wasm32")] // WTF? Why is this is necessary :(
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            leptos::logging::log!("spawing local future");
            spawn_local(async move {
                leptos::logging::log!("spawing local future");
                let canvas = canvas.deref().clone();
                let mut new_mandelbrot = MandelbrotRenderer::new_from_canvas((800, 600), canvas, true)
                    .await
                    .unwrap();

                new_mandelbrot.render(camera_position.get(), camera_size());

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
                        mandelbrot.render(camera_position.get(), camera_size());
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
            set_camera_size_exponent.update(|camera_size_exponent| {
                *camera_size_exponent += event.delta_y() as f32 * SCROLL_SENSITIVITY;
            });
        }
        on:mousedown=move |event: MouseEvent| {
            if event.button() == 0 { // Main button
                set_is_mouse_down.set(true);
            }
        }
        on:mouseup=move |event: MouseEvent| {
            if event.button() == 0 { // Main button
                set_is_mouse_down.set(false);
            }
        }
        on:mouseleave=move |_| {
            set_is_mouse_down.set(false);
        }
        on:mousemove=move |event: MouseEvent| {
            let position: (f32, f32) = (event.client_x() as f32, event.client_y() as f32);

            if is_mouse_down.get() {
                if let Some(previous_mouse_position) = previous_mouse_position.get() {
                    let mut delta = (previous_mouse_position.0 - position.0, previous_mouse_position.1 - position.1);

                    // Normalize to 0..=1
                    let size = size.get();
                    delta = (delta.0 / size.0 as f32, delta.1 / size.1 as f32);

                    // Apply necessary transformations to make up for different coordinate systems
                    delta = (delta.0 * 2.0, delta.1 * -2.0);

                    // Distance moved is proportionhal to camera size
                    let camera_size = camera_size();
                    delta = (delta.0 * camera_size.0, delta.1 * camera_size.1);

                    // Apply custom sensitivity
                    delta = (delta.0 * MOVE_SENSITIVITY, delta.1 * MOVE_SENSITIVITY);

                    set_camera_position.update(|camera_position| {
                        *camera_position = (camera_position.0 + delta.0, camera_position.1 + delta.1);
                    });
                }
            }

            set_previous_mouse_position.set(Some(position));
        }
        width="800px" height="600px"> </canvas>

        <button on:click=move |_| { }>
            Render
        </button>
    }
}
