pub mod logger;
pub mod events;
pub mod window;

use events::*;
use logger::*;
use window::*;

struct Application<'a> {
    window: WindowsWindow<'a, EventDispatcher>,
    dispatcher: EventDispatcher
}

impl<'a> Application<'a> {
    pub fn new (window: WindowsWindow<'_, EventDispatcher>) -> Application {
        Application {window, dispatcher: EventDispatcher {}}
    }
}

struct EventDispatcher;

// TODO: CHECK IF I CAN MAKE THE OBSERVER BE MUTABLE TO CHANGE THE FIELDS
impl EventDispatcher {
    fn on_windows_close(&self, _: &Event) -> bool{
        // self.running = false;
        // self.window.set_should_close(true);
        true
    }
}

impl Observer for EventDispatcher {
    fn update(&self, event: &mut Event) {
        event.set_handled(match event.get_event_type() {
            EventType::WindowClose => self.on_windows_close(event),
            _ => false
        });

        hades_trace(format!("{}", event.to_string()));
    }
}

fn main() {
    Logger::new().init().unwrap();

    let window: WindowsWindow<'_, EventDispatcher> = WindowsWindow::init();
    let mut app = Application::new(window);
    app.window.attach(&mut app.dispatcher);

    while !app.window.should_close() {
        app.window.on_update()
    }
}
