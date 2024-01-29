use winit::{
    keyboard::PhysicalKey,
    window::{Window as WinitWindow, WindowBuilder},
    event::{ElementState, Event as WinitEvent, KeyEvent, MouseScrollDelta, WindowEvent, MouseButton},
    event_loop::EventLoop
};

use crate::{
    events::{Event, EventType}, logger::*, Application
};

pub struct WindowData {
    title: &'static str,
    width: u32,
    height: u32,
    vsync: bool
}

impl WindowData {
    pub fn default() -> WindowData {
        WindowData {title: "Hades Engine", width: 1280, height: 720, vsync: true}
    }

    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled
    }

    pub fn is_vsync(&self) -> bool {
        self.vsync
    }
}

pub struct WindowSystem {
    window_data: WindowData,
    window: WinitWindow,
}

impl WindowSystem {
    pub fn init(event_loop: &EventLoop<()>) -> WindowSystem {
        let window_data = WindowData::default();
        let window = WindowBuilder::new()
                    .with_title(window_data.get_title())
                    .with_inner_size(winit::dpi::LogicalSize::new(window_data.get_width(), window_data.get_height()))
                    .build(&event_loop).unwrap();

        hds_core_info!("Creating window {} ({}, {})", window_data.get_title(), window_data.get_width(), window_data.get_height());

        WindowSystem { window_data, window }
    }

    pub fn main_loop(&self, event_loop: EventLoop<()>, app: &mut Application) {

        event_loop.run(move |event, elwt| {
            // Handle the events from the window
            match event {
                WinitEvent::NewEvents(_) => {
                    app.on_update();
                },
                WinitEvent::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            let hades_event = Event::new(EventType::WindowClose);
                            app.on_event(hades_event);
                            elwt.exit()
                        },
                        WindowEvent::Resized(size) => {
                            let hades_event = Event::new(EventType::WindowResize { width: size.width, height: size.height });
                            app.on_event(hades_event);
                        },
                        WindowEvent::KeyboardInput { event: KeyEvent { physical_key, state, repeat, ..}, .. } => {
                            let hades_event = Event::new(match physical_key {
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
                            app.on_event(hades_event);
                        },
                        WindowEvent::MouseInput { button, state, .. } => {
                            let hades_event = Event::new(match state {
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
                            app.on_event(hades_event);
                        },
                        WindowEvent::MouseWheel { delta, .. } => {
                            let hades_event = Event::new(match delta {
                                MouseScrollDelta::LineDelta(x_offset, y_offset) => EventType::MouseScrolled { x_offset, y_offset },
                                _ => EventType::None,
                            });
                            app.on_event(hades_event);
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            let hades_event = Event::new(EventType::MouseMoved { x: position.x as f32, y: position.y as f32 });
                            app.on_event(hades_event);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }).expect("Error with the events")
    }
}