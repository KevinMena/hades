use winit::{
    keyboard::PhysicalKey,
    window::{Window as WinitWindow, WindowBuilder},
    event::{ElementState, Event as WinitEvent, KeyEvent, MouseScrollDelta, WindowEvent, MouseButton},
    event_loop::EventLoop
};

use crate::events::{Event, EventType};
use crate::{logger::*, Subject, Observer};

pub trait Window {
    fn on_update(&mut self, event_loop: EventLoop<()>, window: WinitWindow);
    
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;

    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;
}

pub struct WindowData {
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

// TODO: SIMPLIFY THIS TO JUST BE A DATA SENDER, OBSERVER PATTERN IS NOT SUTABLE
pub struct WindowsWindow<'a, T: Observer> {
    observer: Option<&'a mut T>,
    data: WindowData
}

impl<'a, T: Observer> Window for WindowsWindow<'a, T> {
    fn on_update(&mut self, event_loop: EventLoop<()>, window: WinitWindow) {
        self.process_events(event_loop, window);
    }

    fn get_width(&self) -> u32 {
        self.data.width
    }

    fn get_height(&self) -> u32 {
        self.data.height
    }

    fn set_vsync(&mut self, enabled: bool) {
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

    pub fn new() -> WindowsWindow<'a, T> {
        let data = WindowData::default();

        WindowsWindow { observer: None, data}
    }

    pub fn init(event_loop: &EventLoop<()>, data: &WindowData) -> WinitWindow{
        let window = WindowBuilder::new()
                    .with_title(&data.title)
                    .with_inner_size(winit::dpi::LogicalSize::new(data.width, data.height))
                    .build(&event_loop).unwrap();
        
        hades_info(format!("Creating window {} ({}, {})", &data.title, &data.width, &data.height));
        
        window
    }

    pub fn get_data(&self) -> &WindowData {
        &self.data
    }

    fn process_events(&self, event_loop: EventLoop<()>, window: WinitWindow) {
        
        event_loop.run(move |event, elwt| {
            match event {
                WinitEvent::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            let mut hades_event = Event::new(EventType::WindowClose);
                            self.notify(&mut hades_event);
                            elwt.exit()
                        },
                        WindowEvent::Resized(size) => {
                            let mut hades_event = Event::new(EventType::WindowResize { width: size.width, height: size.height });
                            self.notify(&mut hades_event);
                        },
                        WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, repeat, ..}, .. } => {
                            let mut hades_event = Event::new(match physical_key {
                                PhysicalKey::Code(keycode) => match state {
                                    ElementState::Pressed => if repeat {
                                        EventType::KeyPressed { keycode: keycode as i32, repeat_count: 1 }
                                    }
                                    else {
                                        EventType::KeyPressed { keycode: keycode as i32, repeat_count: 0 }
                                    },
                                    ElementState::Released => EventType::KeyReleased { keycode: keycode as i32 },
                                },
                                PhysicalKey::Unidentified(_) => EventType::None,
                            });
                            self.notify(&mut hades_event);
                        },
                        WindowEvent::MouseInput { button, state, .. } => {
                            let mut hades_event = Event::new(match state {
                                ElementState::Pressed => EventType::MouseButtonPressed { button: match button {
                                    MouseButton::Left => 0,
                                    MouseButton::Right => 1,
                                    _ => -1
                                } },
                                ElementState::Released => EventType::MouseButtonReleased { button: match button {
                                    MouseButton::Left => 0,
                                    MouseButton::Right => 1,
                                    _ => -1
                                } }
                            });
                            self.notify(&mut hades_event);
                        },
                        WindowEvent::MouseWheel { delta, .. } => {
                            let mut hades_event = Event::new(match delta {
                                MouseScrollDelta::LineDelta(x_offset, y_offset) => EventType::MouseScrolled { x_offset, y_offset },
                                _ => EventType::None,
                            });
                            self.notify(&mut hades_event);
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            let mut hades_event = Event::new(EventType::MouseMoved { x: position.x as f32, y: position.y as f32 });
                            self.notify(&mut hades_event);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }).expect("Error with the events")
    }
}