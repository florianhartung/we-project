use leptos::ev::{scroll, MouseEvent, WheelEvent};
use leptos::{
    component, create_action, create_effect, create_local_resource, create_multi_action,
    create_node_ref, create_rw_signal, create_server_action, html::Canvas, view, IntoView,
    RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith,
};
use leptos::{create_signal, spawn_local, HtmlElement, MaybeSignal, SignalGetUntracked};
use mandelbrot_renderer::MandelbrotRenderer;
use tailwind_fuse::tw_join;
use std::ops::RangeBounds;
use std::{ops::Deref, time::Duration};

// Arguments needed:
// How to specify position & size? position: (f32, f32), size as zoom: f32
// Initial position + size
// Zoom limit: Option<Range<f32>>
// Position limits: (Option<Range<f32>>, Option<Range<f32>>)


pub trait MandelbrotBounds: RangeBounds<f32> + Clone + 'static {
    fn limit_value(&self, mut value: f32) -> f32 {
        value = match self.start_bound() {
            std::ops::Bound::Included(&start) => value.max(start),
            std::ops::Bound::Excluded(&start) => value.max(start + f32::EPSILON),
            std::ops::Bound::Unbounded => value,
        };

        value = match self.end_bound() {
            std::ops::Bound::Included(&start) => value.min(start),
            std::ops::Bound::Excluded(&start) => value.min(start - f32::EPSILON),
            std::ops::Bound::Unbounded => value,
        };

        value
    }
}

impl<T> MandelbrotBounds for T where T: RangeBounds<f32> + Clone + 'static { }

#[component]
pub fn Mandelbrot<RX: MandelbrotBounds, RY: MandelbrotBounds, RZ: MandelbrotBounds>(
    // The size of the canvas element.
    #[prop(into)] size: MaybeSignal<(u32, u32)>,
    // This is the center of the canvas
    position: RwSignal<(f32, f32)>,
    // The zoom level exponent. The mandelbrot coordinate multiplier will essentially be 10^zoom_exponent.
    zoom_exponent: RwSignal<f32>,
    #[prop(into)]
    position_bounds: MaybeSignal<(RX, RY)>,
    #[prop(into)]
    zoom_exponent_bounds: MaybeSignal<RZ>,
    #[prop(optional, into)]
    scroll_sensitivity: Option<f32>,
    #[prop(optional)]
    class: &'static str,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<Canvas>();

    let scroll_sensitivity = scroll_sensitivity.unwrap_or(0.001);

    let mandelbrot: RwSignal<Option<MandelbrotRenderer>> = create_rw_signal(None);

    // Camera signals
    let (is_mouse_down, set_is_mouse_down) = create_signal::<bool>(false);
    let (previous_mouse_position, set_previous_mouse_position) = create_signal::<Option<(f32, f32)>>(None);

    let (camera_position, set_camera_position) = (position.read_only(), position.write_only());
    let (camera_size_exponent, set_camera_size_exponent) = (zoom_exponent.read_only(), zoom_exponent.write_only());

    let camera_size = move || {
        let camera_size = 10.0_f32.powf(-camera_size_exponent.get());
        let size = size.get();

        (camera_size, camera_size * size.1 as f32 / size.0 as f32)
    };

    // Create renderer
    #[cfg(target_arch = "wasm32")] // WTF? Why is this is necessary :(
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            leptos::logging::log!("spawing local future");
            spawn_local(async move {
                leptos::logging::log!("spawing local future");
                let canvas = canvas.deref().clone();
                let size = size.get_untracked();
                let mut new_mandelbrot = MandelbrotRenderer::new_from_canvas((size.0, size.1), canvas, true)
                    .await
                    .unwrap();

                new_mandelbrot.render(camera_position.get_untracked(), camera_size());

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

    create_effect(move |_| {
        let size = size.get();
        mandelbrot.update(|renderer| {
            if let Some(renderer) = renderer {
                renderer.resize((size.0, size.1));
            }
        });
    });

    #[allow(unused)]
    let mut non_passive_wheel = leptos::ev::Custom::<leptos::ev::WheelEvent>::new("wheel");
    #[cfg(feature = "hydrate")] {
        let options = non_passive_wheel.options_mut();
        options.passive(false);
        canvas_ref.on_load(move |canvas: HtmlElement<leptos::html::Canvas>| {
            let _ = canvas.on(non_passive_wheel, move |event| {
                set_camera_size_exponent.update(|camera_size_exponent| {
                    let mut new_camera_size_exponent = *camera_size_exponent;
                    new_camera_size_exponent -= event.delta_y() as f32 * scroll_sensitivity;
                    new_camera_size_exponent = zoom_exponent_bounds.get().limit_value(new_camera_size_exponent);

                    *camera_size_exponent = new_camera_size_exponent;
                });
                event.prevent_default();
                event.stop_propagation();
            });
        });
    }


    let class = move || {
        let size = size.get();
        tw_join!(class, "cursor-pointer active:cursor-move")
    };

    view! {
        <canvas ref=canvas_ref class=class
        on:mousedown=move |event: MouseEvent| {
            if event.button() == 0 { // Main button
                set_is_mouse_down.set(true);
                // event.prevent_default();
            }
        }
        on:mouseup=move |event: MouseEvent| {
            if event.button() == 0 { // Main button
                set_is_mouse_down.set(false);
                // event.prevent_default();
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
                    delta = (2.0 * delta.0, -2.0 * delta.1);

                    // Distance moved is proportionhal to camera size
                    let camera_size = camera_size();
                    delta = (delta.0 * camera_size.0, delta.1 * camera_size.1);

                    set_camera_position.update(|camera_position| {
                        // Add delta
                        *camera_position = (camera_position.0 + delta.0, camera_position.1 + delta.1);

                        // Limit camera position to be inside specified bounds
                        let position_bounds = position_bounds.get();
                        *camera_position = (position_bounds.0.limit_value(camera_position.0), position_bounds.1.limit_value(camera_position.1));
                    });
                }
            }

            set_previous_mouse_position.set(Some(position));
        }
        width=move || format!("{}px", size.get().0) height=move || format!("{}px", size.get().1)> </canvas>
    }
}
