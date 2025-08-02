use oxide::{AppBehavior, AppConfig, Application, OxideResult, oxide_log};
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
            oxide_log!(trace, "Key pressed, current color value: {}", self.color);
        } else if let Event::MouseButtonDown { .. } = event {
            oxide_log!(
                trace,
                "Mouse button pressed, current color value: {}",
                self.color
            );
        }
    }

    fn on_render(&mut self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(self.color / 2, self.color / 2, self.color));
        canvas.clear();
    }
}

fn main() -> OxideResult<()> {
    let config = AppConfig {
        app_name: "Sundaria".to_string(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        vsync: true,
    };

    Application::new(config, MyApp::new())?.run()
}
