#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
pub mod video;
pub mod zmq_connector;