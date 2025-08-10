use neuton::*;

struct MainMenu;

impl Scene for MainMenu {
    fn on_update(&mut self, _dt: f64) {}

    fn on_event(&mut self, _event: &Event) {}

    fn on_render(&mut self, canvas: &mut Canvas<Window>) {
       canvas.set_draw_color(Color::GRAY); 
    }
}

struct GameScene;

impl Scene for GameScene {
    fn on_update(&mut self, _dt: f64) {}

    fn on_event(&mut self, _event: &Event) {}

    fn on_render(&mut self, canvas: &mut Canvas<Window>) {
       canvas.set_draw_color(Color::GREEN); 
    }
}

enum GameState {
    MainMenu,
    InGame,
}

struct App {
    scenes: SceneStack,
    state: GameState,
}

impl App {
    fn new() -> Self {
        let mut scenes = SceneStack::new();
        scenes.push_scene(Box::new(MainMenu));
        Self {
            scenes,
            state: GameState::MainMenu,
        }
    }
}

impl AppBehavior for App {
    fn on_update(&mut self, dt: f64) {
        self.scenes.update(dt);
    }

    fn on_event(&mut self, event: &Event) {
        match self.state {
            GameState::MainMenu => {
                if keyboard_pressed(event, Keycode::Return) {
                    self.state = GameState::InGame;
                    self.scenes.push_scene(Box::new(GameScene));
                }
            }
            GameState::InGame => {
                if keyboard_pressed(event, Keycode::Escape) {
                    self.state = GameState::MainMenu;
                    self.scenes.pop_scene();
                }
            }
        }
        self.scenes.handle_event(event);
    }

    fn on_render(&mut self, canvas: &mut Canvas<Window>) {
        self.scenes.render(canvas);
    }
}

fn main() -> NeutonResult<()> {
    let config = AppConfig {
        app_name: "Simple Scenes Example".to_string(),
        window_width: 800,
        window_height: 600,
        fullscreen: false,
        vsync: true,
        max_fps: None,
        log_directory: None,
    };

    init_logger(config.log_directory.as_deref())?;

    Application::new(config, App::new())?.run()
}