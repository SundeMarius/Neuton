use crate::behavior::AppBehavior;
use crate::error::{NeutonError, NeutonResult};
use crate::log;
use sdl2::{event::Event, render::Canvas, video::Window};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};

/// Configuration for the application window and rendering.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// The window title.
    pub app_name: String,
    /// The window width in pixels.
    pub window_width: u32,
    /// The window height in pixels.
    pub window_height: u32,
    /// Whether to start in fullscreen mode.
    #[serde(skip_deserializing)]
    pub fullscreen: bool,
    /// Whether to enable vsync for the renderer.
    pub vsync: bool,
    /// Optional maximum frames per second. If None, defaults to 300 Hz.
    pub max_fps: Option<u32>,
    /// Optional directory for log files. If None, uses the home directory as default.
    pub log_directory: Option<PathBuf>,
}

/// Main application struct. Owns the SDL context, window, and user logic.
pub struct Application<B: AppBehavior> {
    /// The application configuration.
    pub config: AppConfig,
    /// The user-defined behavior implementation.
    pub behavior: B,
    sdl_context: sdl2::Sdl,
    canvas: Canvas<Window>,
    frame_rate: f64,
}

impl<B: AppBehavior> Application<B> {
    /// Create a new application with the given configuration and behavior.
    ///
    /// # Errors
    /// Returns an error if SDL2, the logger or the window/canvas cannot be initialized.
    pub fn new(config: AppConfig, behavior: B) -> NeutonResult<Self> {
        let sdl_context = sdl2::init().map_err(|e| NeutonError::Sdl2Error(e.to_string()))?;
        let video_subsystem = sdl_context
            .video()
            .map_err(|e| NeutonError::Sdl2Error(e.to_string()))?;
        let mut window_builder =
            video_subsystem.window(&config.app_name, config.window_width, config.window_height);

        window_builder
            .position_centered()
            .allow_highdpi()
            .resizable();

        if config.fullscreen {
            window_builder.fullscreen();
        }

        let window = window_builder
            .build()
            .map_err(|e| NeutonError::WindowError(e.to_string()))?;

        let mut canvas_builder = window.into_canvas();
        if config.vsync {
            canvas_builder = canvas_builder.present_vsync();
        }
        let canvas = canvas_builder
            .build()
            .map_err(|e| NeutonError::CanvasError(e.to_string()))?;

        log!(info, "'{}' initialized!", config.app_name);

        Ok(Self {
            config,
            behavior,
            sdl_context,
            canvas,
            frame_rate: 0.0,
        })
    }

    /// Run the application main loop.
    ///
    /// Handles events, updates, and rendering. Calls user hooks each frame.
    /// Returns when the window is closed or Escape is pressed.
    pub fn run(mut self) -> NeutonResult<()> {
        let mut event_pump = self
            .sdl_context
            .event_pump()
            .map_err(|e| NeutonError::Sdl2Error(e.to_string()))?;

        log!(info, "Running '{}'", self.config.app_name);

        let mut last_frame = Instant::now();

        'running: loop {
            let frame_start = Instant::now();

            // Compute delta time (clamped)
            let dt = last_frame.elapsed().as_secs_f64().min(0.1);
            self.frame_rate = 1.0 / (dt + f64::EPSILON);
            last_frame = frame_start;

            // Input
            for event in event_pump.poll_iter() {
                if Self::should_quit(&event) {
                    break 'running;
                }
                self.behavior.on_event(&event);
            }

            // Update & Render
            self.behavior.on_update(dt);
            self.canvas.clear();
            self.behavior.on_render(&mut self.canvas);
            self.canvas.present();

            // Framerate limiting
            if !self.config.vsync && let Some(max_fps) = self.config.max_fps {
                let target_frame_time = Duration::from_secs_f64(1.0 / max_fps.max(1) as f64);
                let elapsed = frame_start.elapsed();
                if elapsed < target_frame_time {
                    thread::sleep(target_frame_time - elapsed);
                }
            }
        }

        log!(info, "'{}' has exited.", self.config.app_name);
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

    /// Get the most recent frame rate (frames per second).
    pub fn frame_rate(&self) -> f64 {
        self.frame_rate
    }

    /// Helper to check if an event should quit the app.
    fn should_quit(event: &Event) -> bool {
        matches!(event, Event::Quit { .. })
    }
}
