use sdl2::{event::Event, render::Canvas, video::Window};

/// Trait for user application logic.
///
/// Implement this trait to define how your application updates, handles events, and renders.
pub trait AppBehavior {
    /// Called on each update frame.
    /// `dt` is the delta time since the last update in seconds.
    fn on_update(&mut self, dt: f64);

    /// Handle events from the SDL event pump.
    fn on_event(&mut self, event: &Event);

    /// Render the application state to the canvas.
    fn on_render(&mut self, canvas: &mut Canvas<Window>);
}
