//! # A collection of components
//! With an exception of the `common` module, components may include business logic.
//!
//! Business logic components should only expose their component function as public.
//! It can then be reexported from this module for simplicity.

pub mod common;

pub mod logout_button;
pub use logout_button::*;

pub mod game;
pub use game::*;