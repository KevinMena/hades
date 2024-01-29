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

        app.push_layer(Box::new(ExampleLayer { name: "Example" }));

        app
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
    pub fn run(&mut self) {
        // Winit window settings
        let event_loop = EventLoop::new().unwrap();
        let window = WindowSystem::init(&event_loop);

        // Loop of winit events
        event_loop.set_control_flow(ControlFlow::Poll);
        window.main_loop(event_loop, self)
    }

    pub fn on_update(&self) {
        for layer in self.layer_stack.get_layers() {
            layer.on_update()
        }
    }

    pub fn on_event(&mut self, mut event: Event) {
        event.set_handled(match event.get_event_type() {
            EventType::WindowClose => self.on_windows_close(&event),
            _ => false
        });

        hds_core_trace!("{}", event.to_string());

        for layer in self.layer_stack.get_layers().iter().rev() {
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

struct ExampleLayer {
    name: &'static str
}

impl Layer for ExampleLayer {
    fn eq(&self, _other: &dyn Layer) -> bool {
        false
    }

    fn on_attach(&self) {
        
    }

    fn on_detach(&self) {
        
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