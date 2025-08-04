mod app;
mod assets;
mod behavior;
mod error;
mod logger;
mod scene;
mod utils;

pub use app::{AppConfig, Application};
pub use behavior::AppBehavior;
pub use error::{NeutonError, NeutonResult};
pub use logger::init_logger;
pub use scene::{Scene, SceneStack};
pub use sdl2::event::Event;
pub use sdl2::keyboard::Keycode;
pub use sdl2::pixels::Color;
pub use sdl2::render::Canvas;
pub use sdl2::video::Window;
pub use utils::*;
