use sdl2::{event::Event, render::Canvas, video::Window};

/// Trait for scene management in the application.
/// Implement this trait to define how your scene behaves when entered, exited, updated, on events and rendered.
pub trait Scene {
    /// Called when the scene is entered.
    fn on_enter(&mut self) {}
    /// Called when the scene is exited.
    fn on_exit(&mut self) {}
    /// Update the scene logic.
    fn on_update(&mut self, _dt: f64) {}
    /// Handle events from the SDL event pump.
    fn on_event(&mut self, _event: &Event) {}
    /// Render the scene to the canvas.
    fn on_render(&mut self, _canvas: &mut Canvas<Window>) {}
}

#[derive(Default)]
pub struct SceneStack {
    scenes: Vec<Box<dyn Scene>>,
}

#[allow(dead_code)]
impl SceneStack {
    pub fn new() -> Self {
        Self::default()
    }

    /// Push a new scene onto the stack.
    pub fn push_scene(&mut self, mut scene: Box<dyn Scene>) {
        if let Some(current_scene) = self.scenes.last_mut() {
            current_scene.on_exit();
        }
        scene.on_enter();
        self.scenes.push(scene);
    }

    /// Pop the current scene from the stack.
    pub fn pop_scene(&mut self) -> Option<Box<dyn Scene>> {
        if let Some(mut scene) = self.scenes.pop() {
            scene.on_exit();
            if let Some(next_scene) = self.scenes.last_mut() {
                next_scene.on_enter();
            }
            Some(scene)
        } else {
            None
        }
    }

    /// Update the current scene.
    pub fn update(&mut self, dt: f64) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.on_update(dt);
        }
    }

    /// Handle events for the current scene.
    pub fn handle_event(&mut self, event: &Event) {
        if let Some(scene) = self.scenes.last_mut() {
            scene.on_event(event);
        }
    }

    /// Render all scenes in the stack in order, so overlays are possible.
    pub fn render(&mut self, canvas: &mut Canvas<Window>) {
        for scene in self.scenes.iter_mut() {
            scene.on_render(canvas);
        }
    }

    /// Returns true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.scenes.is_empty()
    }

    /// Returns the number of scenes in the stack.
    pub fn len(&self) -> usize {
        self.scenes.len()
    }
}
