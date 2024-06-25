//! # A collection of simple components
//! Components in this module usually do not perform any business logic and should thus only be used for simple components.
//! One such provided example is a configurable pill button (styled with Tailwind).

mod pill_button;
pub use pill_button::*;

mod mandelbrot;
pub use mandelbrot::*;

mod login_register;
pub use login_register::*;