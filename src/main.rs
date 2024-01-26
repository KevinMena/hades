pub mod logger;
pub mod events;
pub mod window;
pub mod application;

use logger::*;
use application::*;

fn main() {
    Logger::new().init().unwrap();

    let app = Application::new();
    app.main_loop()
}
