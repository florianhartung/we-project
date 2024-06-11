use std::sync::Arc;

use mandelbrot_renderer::MandelbrotExplorer;
use pollster::FutureExt;
use web_time::{Duration, Instant};
use winit::event::{ElementState, Event, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::Window;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(Window::new(&event_loop).unwrap());

    let window_size = (window.inner_size().width, window.inner_size().height);
    let mut mandelbrot = MandelbrotExplorer::new(window_size, window.clone())
        .block_on()
        .unwrap();

    let mut last_frame = Instant::now();
    let mut counter = 0;

    let mut last_inc = Instant::now();
    event_loop
        .run(|event, target| match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(new_size) => {
                        mandelbrot.resize((new_size.width, new_size.height));
                    }
                    WindowEvent::CloseRequested => {
                        target.exit();
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        mandelbrot.on_mouse_move((position.x as f32, position.y as f32));
                    },
                    WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_, delta), ..} => {
                        mandelbrot.on_mouse_wheel(delta);
                    },
                    WindowEvent::MouseInput { state, button, .. } => {
                        if button == MouseButton::Left {
                            mandelbrot.on_mouse_click(state == ElementState::Pressed);
                        }
                    },
                    WindowEvent::RedrawRequested => {
                        if last_inc.elapsed() > Duration::from_millis(16) {
                            last_inc = Instant::now();
                            mandelbrot.increment_time();
                        }

                        if counter == 200 {
                            let now = Instant::now();
                            let dt = now.duration_since(last_frame);
                            last_frame = now;
                            counter = 0;
                            window.set_title(&format!("FPS: {:.0}", 200.0 / dt.as_secs_f32()));
                        } else {
                            counter += 1;
                        }

                        mandelbrot.render();
                    },
                    _ => {},
                }
            }
            Event::AboutToWait => {
                window.request_redraw();
            }
            _ => {}
        })
        .unwrap();
}
