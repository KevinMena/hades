pub mod logger;
pub mod events;
pub mod window;

use events::*;
use logger::*;
use window::*;

struct Application {
    running: bool
}

impl Application {
    pub fn new () -> Application {
        Application {running: true}
    }

    fn on_windows_close(&self, _: &Event) {
        // self.running = false;
        // self.window.set_should_close(true);
    }
}

impl Observer for Application {
    fn update(&self, event: &Event) {
        match event.get_event_type() {
            EventType::WindowClose => self.on_windows_close(event),
            _ => {}
        }

        hades_trace(format!("{}", event.to_string()));
    }
}

fn main() {
    Logger::new().init().unwrap();

    let mut app = Application::new();
    let mut window: WindowsWindow<'_, Application> = WindowsWindow::init();
    window.attach(&mut app);

    while !window.should_close() {
        window.on_update()
    }
}
