pub mod logger;
pub mod events;
use events::*;
use logger::*;


fn main() {
    Logger::new().init().unwrap();

    let e = Event::new(EventType::WindowResize { width: 1280, height: 720 });

    if e.is_in_category(EventCategory::EventCategoryApplication) {
        info(e.to_string());
    }

    if !e.is_in_category(EventCategory::EventCategoryInput) {
        error(e.to_string());
    }
}
