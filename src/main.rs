pub mod logger;
pub mod events;
pub mod window;

use events::*;
use logger::*;
use window::*;


fn main() {
    Logger::new().init().unwrap();

    let mut window = WindowsWindow::init();

    while !window.should_close() {
        window.on_update()
    }
}
