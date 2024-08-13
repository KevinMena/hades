pub mod logger;
pub mod events;
pub mod window;
pub mod application;
pub mod layers;

use logger::*;
use application::*;

fn main() {
    Logger::new().init().unwrap();

    let (mut app, event_loop) = Application::new();
    app.run(event_loop)
}
