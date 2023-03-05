#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::RoverGUI;
pub mod video;
pub mod zmq_connector;