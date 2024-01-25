use glfw::{GlfwReceiver, Action, Context, Error, WindowEvent};

use crate::events::{Event, EventType};
use crate::{logger::*, Subject, Observer};

pub trait Window {
    fn on_update(&mut self);
    
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}

struct WindowData {
    title: String,
    width: u32,
    height: u32,
    vsync: bool
}

impl WindowData {
    fn default() -> WindowData {
        WindowData {title: String::from("Hades Engine"), width: 1280, height: 720, vsync: true}
    }
}

pub struct WindowsWindow<'a, T: Observer> {
    observer: Option<&'a mut T>,
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    data: WindowData
}

impl<'a, T: Observer> Window for WindowsWindow<'a, T> {
    fn on_update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn get_width(&self) -> u32 {
        self.data.width
    }

    fn get_height(&self) -> u32 {
        self.data.height
    }

    fn set_vsync(&mut self, enabled: bool) {
        if enabled {
            self.glfw.set_swap_interval(glfw::SwapInterval::Adaptive);
        }
        else {
            self.glfw.set_swap_interval(glfw::SwapInterval::None);
        }

        self.data.vsync = enabled
    }

    fn is_vsync(&self) -> bool {
        self.data.vsync
    }
}

impl<'a, T: Observer> Subject<'a, T> for WindowsWindow<'a, T> {
    fn attach(&mut self, observer: &'a mut T) {
        self.observer = Some(observer)
    }

    fn detach(&mut self, _: &'a mut T) {
        self.observer = None
    }

    fn notify(&self, event: &mut Event) {
        match &self.observer {
            Some(observer) => observer.update(event),
            None => hades_error(String::from("Window error: application observer was not set properly."))
        }
    }
}

impl<'a, T: Observer> WindowsWindow<'a, T> {

    pub fn init() -> WindowsWindow<'a, T>{
        let data = WindowData::default();
        
        hades_info(format!("Creating window {} ({}, {})", data.title, data.width, data.height));
        
        let mut glfw = glfw::init(error_callback).unwrap();

        // TODO: Add assertion

        let (mut window, w_events) = glfw
                                    .create_window(data.width, data.height, &data.title, glfw::WindowMode::Windowed)
                                    .expect("Failed to create GLFW window");
        
        window.make_current();

        // Set the events we want glfw to poll to handle
        window.set_close_polling(true);
        window.set_size_polling(true);
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_scroll_polling(true);
        window.set_cursor_pos_polling(true);
        
        WindowsWindow { observer: None, glfw, window_handle: window, events: w_events, data}
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    fn process_events(&mut self) {
        for(_, window_event) in glfw::flush_messages(&self.events) {
            match window_event {
                glfw::WindowEvent::Close => {
                    let mut event = Event::new(EventType::WindowClose);
                    self.notify(&mut event);
                    self.window_handle.set_should_close(true);
                }
                glfw::WindowEvent::Size(width, height) => {
                    let mut event = Event::new(EventType::WindowResize { width: width as u32, height: height as u32 });
                    self.notify(&mut event);
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    let mut event = Event::new(match action {
                        Action::Press => EventType::KeyPressed { keycode: key as i32, repeat_count: 0 },
                        Action::Release => EventType::KeyReleased { keycode: key as i32 },
                        Action::Repeat => EventType::KeyPressed { keycode: key as i32, repeat_count: 1 },
                    });
                    self.notify(&mut event);
                }
                glfw::WindowEvent::MouseButton(button, action, _) => {
                    let mut event = Event::new(match action {
                        Action::Press => EventType::MouseButtonPressed { button: button as i32 },
                        Action::Release => EventType::MouseButtonReleased { button: button as i32 },
                        _ => EventType::None
                    });
                    self.notify(&mut event);
                }
                glfw::WindowEvent::Scroll(x_offset, y_offset) => {
                    let mut event = Event::new(EventType::MouseScrolled { x_offset: x_offset as f32, y_offset: y_offset as f32 });
                    self.notify(&mut event);
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let mut event = Event::new(EventType::MouseMoved { x: x as f32, y: y as f32 });
                    self.notify(&mut event);
                }
                _ => {}
            }
        }
    }
}

fn error_callback(error: Error, description: String) {
    hades_error(format!("GLFW Error ({:?}): {:?}", error, description))
}