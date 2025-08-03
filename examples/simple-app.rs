use std::path::PathBuf;

use oxide::{AppBehavior, AppConfig, Application, OxideResult, init_logger, log};
use sdl2::{event::Event, pixels::Color, render::Canvas, video::Window};

struct MyApp {
    color: u8,
}

impl MyApp {
    fn new() -> Self {
        Self { color: 0 }
    }
}

impl AppBehavior for MyApp {
    fn on_update(&mut self, _dt: f64) {
        self.color = self.color.wrapping_add(1);
    }

    fn on_event(&mut self, event: &Event) {
        if let Event::KeyDown { .. } = event {
            log!(trace, "Key pressed, current color value: {}", self.color);
        } else if let Event::MouseButtonDown { .. } = event {
            log!(
                trace,
                "Mouse button pressed, current color value: {}",
                self.color
            );
        }
    }

    fn on_render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(self.color / 2, self.color / 2, self.color));
    }
}

fn main() -> OxideResult<()> {
    let config = AppConfig {
        app_name: "Example App".to_string(),
        window_width: 3560,
        window_height: 1440,
        fullscreen: false,
        vsync: true,
        max_fps: Some(60),
        log_directory: Some(PathBuf::from("/tmp")),
    };

    init_logger(config.log_directory.as_deref())?;

    Application::new(config, MyApp::new())?.run()
}
