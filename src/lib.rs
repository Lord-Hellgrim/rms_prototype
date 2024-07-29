#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod components;
pub mod utilities;
pub mod rms_error;
pub mod screens;
pub use app::App;
pub mod database_layer;