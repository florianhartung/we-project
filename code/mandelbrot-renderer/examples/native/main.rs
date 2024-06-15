use std::sync::Arc;

use cgmath::{ElementWise, Vector2, Zero};
use mandelbrot_renderer::MandelbrotRenderer;
use pollster::FutureExt;
use web_time::{Duration, Instant};
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(Window::new(&event_loop).unwrap());

    let window_size = (window.inner_size().width, window.inner_size().height);
    let mut mandelbrot = MandelbrotRenderer::new(window_size, window.clone(), false)
        .block_on()
        .unwrap();

    let mut fps = FpsCounter::new();

    let mut is_mouse_down: bool = false;
    let mut previous_mouse_position: Option<Vector2<f32>> = None;

    let mut camera_position: Vector2<f32> = Vector2::zero();
    let mut camera_size_exponent = 0.0;


    event_loop
        .run(move |event, target| match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::Resized(new_size) => {
                    mandelbrot.resize((new_size.width, new_size.height));
                }
                WindowEvent::CloseRequested => {
                    target.exit();
                }
                WindowEvent::CursorMoved { position, .. } => {
                    let position = Vector2::new(position.x as f32, position.y as f32);

                    if is_mouse_down {
                        if let Some(previous_mouse_position) = previous_mouse_position {
                            let mut delta = previous_mouse_position - position;

                            // Normalize to 0..=1
                            let window_size = Vector2::new(window.inner_size().width as f32, window.inner_size().height as f32);
                            delta.div_assign_element_wise(window_size);

                            // Apply necessary transformations to make up for different coordinate systems
                            delta.mul_assign_element_wise(Vector2::new(2.0, -2.0));

                            // Distance moved is proportional to camera size
                            delta.mul_assign_element_wise(calc_camera_size(&window, camera_size_exponent));

                            camera_position += delta;
                        }
                    }

                    previous_mouse_position = Some(position);
                }
                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, delta),
                    ..
                } => {
                    camera_size_exponent -= delta;
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    if button == MouseButton::Left {
                        is_mouse_down = state == ElementState::Pressed;
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(fps) = fps.record_frame() {
                        window.set_title(&format!("FPS: {fps:.1}"));
                    }

                    let camera_size = calc_camera_size(&window, camera_size_exponent);
                    mandelbrot.render((camera_position.x, camera_position.y), (camera_size.x, camera_size.y));
                }
                _ => {}
            },
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        })
        .unwrap();

}

fn calc_camera_size(window: &Window, camera_size_exponent: f32) -> Vector2<f32> {
    let size = 1.1_f32.powf(camera_size_exponent);

    Vector2::new(size, size * window.inner_size().height as f32 / window.inner_size().width as f32)
}

/// Fps counter that calculates average FPS every second
struct FpsCounter {
    current_num_frames: usize,
    last_calculation: Instant,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            current_num_frames: 0,
            last_calculation: Instant::now(),
        }
    }

    // Returns newly calculated fps
    pub fn record_frame(&mut self) -> Option<f32> {
        self.current_num_frames += 1;

        let now = Instant::now();
        let dt = now.duration_since(self.last_calculation);

        (dt > Duration::from_secs(1)).then(|| {
            let dt_per_frame = 1.0 / (dt.as_secs_f32() * self.current_num_frames as f32);
            self.current_num_frames = 0;
            self.last_calculation = now;

            1.0 / dt_per_frame
        })
    }
}
