use glutin::{context::PossiblyCurrentContext, surface::{Surface, WindowSurface}};
use imgui_layer::ImguiLayer;
use winit::{
    event::{ElementState, Event as WinitEvent, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent}, 
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget}, 
    keyboard::PhysicalKey, 
    window::Window
};

use crate::{
    events::{Event, EventType}, layers::*, logger::*, window::WindowSystem
};

pub struct Application {
    running: bool,
    layer_stack: LayerStack,
    window: Window,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext
}

impl Application {
    pub fn new () -> (Application, EventLoop<()>) {
        // Layers settings
        let layer_stack = LayerStack::new();

        // Winit window settings
        let event_loop = EventLoop::new().unwrap();
        let (window, surface, context) = WindowSystem::init_window(&event_loop);

        // Create layers that compound the application
        let imgui_layer = ImguiLayer::new(&window);
        
        let mut app = Application { running: true, layer_stack, window, surface, context};

        app.push_layer(Box::new(imgui_layer), LayerParam::None);

        (app, event_loop)
    }

    // Layers functions
    pub fn push_layer(&mut self, mut layer: Box<dyn Layer>, param: LayerParam) {
        layer.on_attach(param);
        self.layer_stack.push_layer(layer);
    }

    pub fn push_overlay(&mut self, mut overlay: Box<dyn Layer>, param: LayerParam) {
        overlay.on_attach(param);
        self.layer_stack.push_overlay(overlay);
    }

    // Application flow
    pub fn run(&mut self, event_loop: EventLoop<()>) {
        // Loop of winit events
        event_loop.set_control_flow(ControlFlow::Poll);
        self.main_loop(event_loop)
    }

    pub fn main_loop(&mut self, event_loop: EventLoop<()>) {

        event_loop.run(move |event, elwt| {
            // Handle the events from the window
            match event {
                WinitEvent::NewEvents(_) => {
                    self.on_update();
                },
                WinitEvent::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested => {
                            let hades_event = Event::new(EventType::WindowClose { elwt });
                            self.on_event(hades_event);
                        },
                        WindowEvent::Resized(size) => {
                            let hades_event = Event::new(EventType::WindowResize { width: size.width, height: size.height });
                            self.on_event(hades_event);
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
                            self.on_event(hades_event);
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
                            self.on_event(hades_event);
                        },
                        WindowEvent::MouseWheel { delta, .. } => {
                            let hades_event = Event::new(match delta {
                                MouseScrollDelta::LineDelta(x_offset, y_offset) => EventType::MouseScrolled { x_offset, y_offset },
                                _ => EventType::None,
                            });
                            self.on_event(hades_event);
                        },
                        WindowEvent::CursorMoved { position, .. } => {
                            let hades_event = Event::new(EventType::MouseMoved { x: position.x as f32, y: position.y as f32 });
                            self.on_event(hades_event);
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        }).expect("Error with the events")
    }

    pub fn on_update(&mut self) {
        for layer in self.layer_stack.get_layers().iter_mut() {
            layer.on_update()
        }
    }

    pub fn on_event(&mut self, mut event: Event) {
        event.set_handled(match event.get_event_type() {
            EventType::WindowClose {elwt} => self.on_windows_close(&elwt),
            _ => false
        });

        hds_core_trace!("{}", event.to_string());

        for layer in self.layer_stack.get_layers().iter_mut().rev() {
            event.set_handled(layer.on_event(&event));
            if event.is_handled() {
                break;
            }
        }
    }

    // Event callbacks
    fn on_windows_close(&mut self, elwt: &EventLoopWindowTarget<()>) -> bool{
        self.running = false;
        elwt.exit();
        true
    }
}