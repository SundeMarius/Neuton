use oxide::{AppBehavior, AppConfig, Application, Result};
use sdl2::{event::Event, pixels::Color};

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
        // Handle events here
        match event {
            Event::KeyDown { .. } => {
                // Example: print on key down
                println!("Key pressed!");
            }
            _ => {}
        }
    }

    fn on_render(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(self.color, self.color, self.color));
    }
}

fn main() -> Result<()> {
    let config = AppConfig {
        app_name: "Oxide Test".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: true,
        vsync: true,
    };

    Application::new(config, MyApp::new())?.run()
}
