use sdl2::{event::Event, keyboard::Keycode, render::Canvas, video::Window};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Configuration for the application window and rendering.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub window_width: u32,
    pub window_height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

/// Trait for user application logic.
pub trait AppBehavior {
    /// Called on each update frame.
    /// `dt` is the delta time since the last update in seconds.
    fn on_update(&mut self, dt: f64);

    /// Handle events from the SDL event pump.
    fn on_event(&mut self, event: &Event);

    /// Render the application state to the canvas.
    fn on_render(&mut self, canvas: &mut Canvas<Window>);
}

/// Main application struct.
pub struct Application<B: AppBehavior> {
    pub config: AppConfig,
    pub behavior: B,
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
}

impl<B: AppBehavior> Application<B> {
    /// Create a new application.
    pub fn new(config: AppConfig, behavior: B) -> Result<Self> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let mut window_builder =
            video_subsystem.window(&config.app_name, config.window_width, config.window_height);

        window_builder
            .position_centered()
            .allow_highdpi()
            .resizable();

        if config.fullscreen {
            window_builder.fullscreen();
        }

        let mut canvas_builder = window_builder.build()?.into_canvas();
        if config.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let canvas = canvas_builder.build()?;

        Ok(Application {
            config,
            behavior,
            sdl_context,
            canvas,
        })
    }

    /// Run the application main loop.
    pub fn run(mut self) -> Result<()> {
        let mut event_pump = self.sdl_context.event_pump()?;

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => self.behavior.on_event(&event),
                }
            }

            let dt = 1.0 / 60.0;
            self.behavior.on_update(dt);
            self.canvas.clear();
            self.behavior.on_render(&mut self.canvas);
            self.canvas.present();
        }

        Ok(())
    }

    /// Get a reference to the window.
    pub fn window(&self) -> &Window {
        self.canvas.window()
    }

    /// Get a mutable reference to the canvas.
    pub fn canvas(&mut self) -> &mut Canvas<Window> {
        &mut self.canvas
    }
}
