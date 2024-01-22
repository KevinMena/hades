use glfw::{GlfwReceiver, Action, Context, Key, WindowEvent, fail_on_errors};

use crate::Event;
use crate::logger::*;

pub trait Window {
    fn on_update(&mut self);
    
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn set_event_callback(&mut self, callback: fn(&Event));
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}

struct WindowData {
    title: String,
    width: u32,
    height: u32,
    vsync: bool,
    event_callback: Option<fn(&Event)>
}

impl WindowData {
    fn default() -> WindowData {
        WindowData {title: String::from("Hades Engine"), width: 1280, height: 720, vsync: true, event_callback: None}
    }

    fn new(title: String, width: u32, height: u32) -> WindowData {
        WindowData {title, width, height, vsync: true, event_callback: None}
    }
}

pub struct WindowsWindow {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    data: WindowData
}

impl Window for WindowsWindow {
    fn on_update(&mut self) {
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn get_width(&self) -> u32 {
        self.data.width
    }

    fn get_height(&self) -> u32 {
        self.data.height
    }

    fn set_event_callback(&mut self, callback: fn(&Event)) {
        self.data.event_callback = Some(callback)
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

impl WindowsWindow {

    pub fn init() -> WindowsWindow{
        let data = WindowData::default();
        
        hades_info(format!("Creating window {} ({}, {})", data.title, data.width, data.height));
        
        let mut glfw = glfw::init(glfw::fail_on_errors!()).unwrap();

        // TODO: Add assertion

        let (mut window, w_events) = glfw
                                    .create_window(data.width, data.height, &data.title, glfw::WindowMode::Windowed)
                                    .expect("Failed to create GLFW window");
        
        window.make_current();
        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);
        
        WindowsWindow { glfw, window_handle: window, events: w_events, data}
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }
}