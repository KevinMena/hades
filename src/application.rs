use winit::{
    keyboard::PhysicalKey,
    window::{Window as WinitWindow, WindowBuilder},
    event::{ElementState, Event as WinitEvent, KeyEvent, MouseScrollDelta, WindowEvent, MouseButton},
    event_loop::{EventLoop, ControlFlow}
};

use crate::{
    events::{Event, EventType}, logger::*, layers::*, window::WindowData
};

pub struct Application {
    running: bool,
    window_data: WindowData,
    layer_stack: LayerStack
}

impl Application {
    pub fn new () -> Application {
        let window_data = WindowData::default();
        let layer_stack = LayerStack::new();
        
        // Testing
        let mut app = Application { running: true, window_data, layer_stack };

        app.push_layer(Box::new(ExampleLayer { name: "Example" }));

        app
    }

    pub fn init(&self, event_loop: &EventLoop<()>) -> WinitWindow{
        let data = self.get_data();
        let window = WindowBuilder::new()
                    .with_title(data.get_title())
                    .with_inner_size(winit::dpi::LogicalSize::new(data.get_width(), data.get_height()))
                    .build(&event_loop).unwrap();
        
        hds_core_info!("Creating window {} ({}, {})", data.get_title(), data.get_width(), data.get_height());
        
        window
    }

    pub fn get_data(&self) -> &WindowData {
        &self.window_data
    }

    // Layers functions
    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        layer.on_attach();
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        overlay.on_attach();
        self.layer_stack.push_overlay(overlay);
    }

    // Application flow
    pub fn main_loop(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        let window: WinitWindow = self.init(&event_loop);

        // Loop of winit events
        event_loop.set_control_flow(ControlFlow::Poll);
        self.run(event_loop, window);
    }

    // Event callbacks
    fn on_event(&mut self, event: &mut Event) {
        event.set_handled(match event.get_event_type() {
            EventType::WindowClose => self.on_windows_close(event),
            _ => false
        });

        hds_core_trace!("{}", event.to_string());

        for layer in self.layer_stack.get_layers().iter().rev() {
            layer.on_event(event);
            if event.is_handled() {
                break;
            }
        }
    }

    fn on_windows_close(&mut self, _: &Event) -> bool{
        self.running = false;
        true
    }

    // Winit events for the window
    fn run(&mut self, event_loop: EventLoop<()>, _window: WinitWindow) {
        event_loop.run(move |event, elwt| {
            // First handle the layers (TODO: Not sure if this is called the right way)
            if self.running{
                for layer in self.layer_stack.get_layers() {
                    layer.on_update();
                }
            }

            // Handle the events from the window
            match event {
                WinitEvent::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            let mut hades_event = Event::new(EventType::WindowClose);
                            self.on_event(&mut hades_event);
                            elwt.exit()
                        },
                        WindowEvent::Resized(size) => {
                            let mut hades_event = Event::new(EventType::WindowResize { width: size.width, height: size.height });
                            self.on_event(&mut hades_event);
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
                            self.on_event(&mut hades_event);
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
                            self.on_event(&mut hades_event);
                        },
                        WindowEvent::MouseWheel { delta, .. } => {
                            let mut hades_event = Event::new(match delta {
                                MouseScrollDelta::LineDelta(x_offset, y_offset) => EventType::MouseScrolled { x_offset, y_offset },
                                _ => EventType::None,
                            });
                            self.on_event(&mut hades_event);
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            let mut hades_event = Event::new(EventType::MouseMoved { x: position.x as f32, y: position.y as f32 });
                            self.on_event(&mut hades_event);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }).expect("Error with the events")
    }
}

struct ExampleLayer {
    name: &'static str
}

impl Layer for ExampleLayer {
    fn eq(&self, _other: &dyn Layer) -> bool {
        false
    }

    fn on_attach(&self) {
        todo!()
    }

    fn on_detach(&self) {
        todo!()
    }

    fn on_update(&self) {
        hds_info!("ExampleLayer::Update");
    }

    fn on_event(&self, event: &Event) {
        hds_trace!("{}", event.to_string());
    }

    fn get_name(&self) -> &str {
        self.name
    }
}