use std::fmt::{Display, self};
use imgui_winit_support::winit::window::Window as WinitWindow;

#[derive(Copy, Clone)]
pub enum EventType<'a> {
    None,
    WindowClose, 
    WindowResize { width: u32, height: u32 }, 
    WindowFocus, WindowLostFocus, WindowMoved, WindowRedrawRequest,
    AppTick, AppUpdate, AppRender, 
    AboutToWait(&'a WinitWindow),
    KeyPressed { keycode: i32, repeat_count: i32 }, 
    KeyReleased { keycode: i32 },
    MouseButtonPressed { button: i32 }, 
    MouseButtonReleased { button: i32 },
    MouseMoved { x: f32, y: f32 }, 
    MouseScrolled { x_offset: f32, y_offset: f32 }
}

impl Display for EventType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::None                     => write!(f, "None"),
            EventType::WindowClose              => write!(f, "WindowClose"),
            EventType::WindowResize {..}        => write!(f, "WindowResize"),
            EventType::WindowFocus              => write!(f, "WindowFocus"),
            EventType::WindowLostFocus          => write!(f, "WindowLostFocus"),
            EventType::WindowMoved              => write!(f, "WindowMoved"),
            EventType::AppTick                  => write!(f, "AppTick"),
            EventType::AppUpdate                => write!(f, "AppUpdate"),
            EventType::AppRender                => write!(f, "AppRender"),
            EventType::KeyPressed {..}          => write!(f, "KeyPressed"),
            EventType::KeyReleased {..}         => write!(f, "KeyReleased"),
            EventType::MouseButtonPressed {..}  => write!(f, "MouseButtonPressed"),
            EventType::MouseButtonReleased {..} => write!(f, "MouseButtonReleased"),
            EventType::MouseMoved {..}          => write!(f, "MouseMoved"),
            EventType::MouseScrolled {..}       => write!(f, "MouseScrolled"),
            EventType::WindowRedrawRequest      => write!(f, "WindowRedrawRequest"),
            EventType::AboutToWait(_)           => write!(f, "AboutToWait"),
        }
    }
}

pub enum EventCategory {
    None = 0,
    EventCategoryApplication = 1 << 0,
    EventCategoryInput = 1 << 1,
    EventCategoryKeyboard = 1 << 2,
    EventCategoryMouse = 1 << 3,
    EventCategoryMouseButton = 1 << 4
}

pub struct Event<'a> {
    handled: bool,
    event_type: EventType<'a>
}

impl Event<'_> {
    pub fn new(event_type: EventType) -> Event {
        Event {handled: false, event_type}
    }

    pub fn get_event_type(&self) -> EventType {
        self.event_type
    }

    pub fn get_name(&self) -> String {
        self.event_type.to_string()
    }

    pub fn set_handled(&mut self, value: bool) {
        self.handled = value
    }

    pub fn is_handled(&self) -> bool {
        self.handled
    }

    pub fn get_category_flags(&self) -> i32 {
        match self.event_type {
            EventType::None => todo!(),
            EventType::WindowClose   => EventCategory::EventCategoryApplication as i32,
            EventType::WindowResize {..} => EventCategory::EventCategoryApplication as i32,
            EventType::WindowFocus => todo!(),
            EventType::WindowLostFocus => todo!(),
            EventType::WindowMoved => todo!(),
            EventType::WindowRedrawRequest => todo!(),
            EventType::AppTick => todo!(),
            EventType::AppUpdate => todo!(),
            EventType::AppRender => todo!(),
            EventType::AboutToWait(_) => todo!(),
            EventType::KeyPressed {..} => EventCategory::EventCategoryKeyboard as i32 | EventCategory::EventCategoryInput as i32,
            EventType::KeyReleased {..} => EventCategory::EventCategoryKeyboard as i32 | EventCategory::EventCategoryInput as i32,
            EventType::MouseButtonPressed {..} => EventCategory::EventCategoryMouse as i32 | EventCategory::EventCategoryInput as i32,
            EventType::MouseButtonReleased {..} => EventCategory::EventCategoryMouse as i32 | EventCategory::EventCategoryInput as i32,
            EventType::MouseMoved {..} => EventCategory::EventCategoryMouse as i32 | EventCategory::EventCategoryInput as i32,
            EventType::MouseScrolled {..}=> EventCategory::EventCategoryMouse as i32 | EventCategory::EventCategoryInput as i32,
        }
    }

    pub fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() & category as i32 > 0
    }

    pub fn to_string(&self) -> String {
        match self.event_type {
            EventType::None => todo!(),
            EventType::WindowClose                                       => format!("WindowClose"),
            EventType::WindowResize { width, height }          => format!("WindowsResize {}, {}", width, height),
            EventType::WindowFocus                                       => todo!(),
            EventType::WindowLostFocus                                   => todo!(),
            EventType::WindowMoved                                       => todo!(),
            EventType::WindowRedrawRequest                               => format!("WindowRedrawRequest"),
            EventType::AppTick                                           => todo!(),
            EventType::AppUpdate                                         => todo!(),
            EventType::AppRender                                         => todo!(),
            EventType::AboutToWait(_)                                    => format!("AboutToWait"),
            EventType::KeyPressed { keycode, repeat_count }    => format!("KeyPressedEvent: {} ({} repeats)", keycode, repeat_count),
            EventType::KeyReleased { keycode }                      => format!("KeyReleasedEvent {}", keycode),
            EventType::MouseButtonPressed { button }                => format!("MouseButtonPressed {}", button),
            EventType::MouseButtonReleased { button }               => format!("MouseButtonReleased {}", button),
            EventType::MouseMoved { x, y }                     => format!("MouseMovedEvent ({},{})", x, y),
            EventType::MouseScrolled { x_offset, y_offset }    => format!("MouseScrolledEvent {},{}", x_offset, y_offset),
        }
    }
}