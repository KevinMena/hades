use winit::event_loop::{EventLoop, ControlFlow};

use crate::{
    events::{Event, EventType}, logger::*, layers::*, window::WindowSystem
};

pub struct Application {
    running: bool,
    layer_stack: LayerStack
}

impl Application {
    pub fn new () -> Application {
        // Layers settings
        let layer_stack = LayerStack::new();
        
        // Testing
        let mut app = Application { running: true, layer_stack };

        // app.push_layer(Box::new(ExampleLayer { name: "Example" }));

        app
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
    pub fn run(&mut self) {
        // Winit window settings
        let event_loop = EventLoop::new().unwrap();
        let window = WindowSystem::init(&event_loop);

        // Loop of winit events
        event_loop.set_control_flow(ControlFlow::Poll);
        window.main_loop(event_loop, self)
    }

    pub fn on_update(&mut self) {
        for layer in self.layer_stack.get_layers().iter_mut() {
            layer.on_update()
        }
    }

    pub fn on_event(&mut self, mut event: Event) {
        event.set_handled(match event.get_event_type() {
            EventType::WindowClose => self.on_windows_close(&event),
            _ => false
        });

        hds_core_trace!("{}", event.to_string());

        for layer in self.layer_stack.get_layers().iter_mut().rev() {
            layer.on_event(&event);
            if event.is_handled() {
                break;
            }
        }
    }

    // Event callbacks
    fn on_windows_close(&mut self, _: &Event) -> bool{
        self.running = false;
        true
    }
}