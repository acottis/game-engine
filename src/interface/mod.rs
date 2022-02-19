//! This module handles all things user interfact and graphical
//! using our [crate::gfx] module to provide GPU support and [winit] 
//! to provide a window from the OS 
//! 
pub mod gfx;
pub mod app;

pub use app::{init_gfx, init_window, handle_events};