//! # A collection of components
//! With an exception of the `common` module, components may include business logic.
//!
//! Business logic components should only expose their component function as public.
//! It can then be reexported from this module for simplicity.

pub mod common;

mod server_counter;
pub use server_counter::*;

mod tech_stack_list;
pub use tech_stack_list::*;

mod mandelbrot;
pub use mandelbrot::*;
