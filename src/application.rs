use winit::{
    keyboard::PhysicalKey,
    window::{Window as WinitWindow, WindowBuilder},
    event::{ElementState, Event as WinitEvent, KeyEvent, MouseScrollDelta, WindowEvent, MouseButton},
    event_loop::{EventLoop, ControlFlow}
};

use crate::{
    logger::*,
    events::{Event, EventType},
    window::WindowData
};

pub struct Application {
    window_data: WindowData
}

impl Application {
    pub fn new () -> Application {
        let window_data = WindowData::default();

        Application { window_data }
    }

    pub fn init(&self, event_loop: &EventLoop<()>) -> WinitWindow{
        let data = self.get_data();
        let window = WindowBuilder::new()
                    .with_title(data.get_title())
                    .with_inner_size(winit::dpi::LogicalSize::new(data.get_width(), data.get_height()))
                    .build(&event_loop).unwrap();
        
        hades_info(format!("Creating window {} ({}, {})", data.get_title(), data.get_width(), data.get_height()));
        
        window
    }

    pub fn get_data(&self) -> &WindowData {
        &self.window_data
    }

    // Application flow
    pub fn main_loop(&self) {
        let event_loop = EventLoop::new().unwrap();
        let window: WinitWindow = self.init(&event_loop);

        // Loop of winit events
        event_loop.set_control_flow(ControlFlow::Poll);
        self.process_events(event_loop, window);
    }

    // Event callbacks
    fn dispatch(&self, event: &mut Event) {
        event.set_handled(match event.get_event_type() {
            EventType::WindowClose => self.on_windows_close(event),
            _ => false
        });

        hades_trace(format!("{}", event.to_string()));
    }

    fn on_windows_close(&self, _: &Event) -> bool{
        true
    }

    // Winit events for the window
    fn process_events(&self, event_loop: EventLoop<()>, _window: WinitWindow) {
        
        event_loop.run(move |event, elwt| {
            match event {
                WinitEvent::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            let mut hades_event = Event::new(EventType::WindowClose);
                            self.dispatch(&mut hades_event);
                            elwt.exit()
                        },
                        WindowEvent::Resized(size) => {
                            let mut hades_event = Event::new(EventType::WindowResize { width: size.width, height: size.height });
                            self.dispatch(&mut hades_event);
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
                            self.dispatch(&mut hades_event);
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
                            self.dispatch(&mut hades_event);
                        },
                        WindowEvent::MouseWheel { delta, .. } => {
                            let mut hades_event = Event::new(match delta {
                                MouseScrollDelta::LineDelta(x_offset, y_offset) => EventType::MouseScrolled { x_offset, y_offset },
                                _ => EventType::None,
                            });
                            self.dispatch(&mut hades_event);
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            let mut hades_event = Event::new(EventType::MouseMoved { x: position.x as f32, y: position.y as f32 });
                            self.dispatch(&mut hades_event);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }).expect("Error with the events")
    }
}