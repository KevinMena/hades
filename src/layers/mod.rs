pub mod imgui_layer;

use imgui_winit_support::winit::window::Window as WinitWindow;

use crate::events::Event;

// TODO: Make the comparison for the traits so we can check if two objects are the same
pub trait Layer {
    fn eq(&self, _other: &dyn Layer) -> bool {
        true
    }

    fn on_attach(&mut self, param: LayerParam);
    fn on_detach(&mut self);
    fn on_update(&mut self);
    fn on_event(&mut self, event: &Event);

    fn get_name(&self) -> &str;
}

pub struct LayerStack {
    layers: Vec<Box<dyn Layer>>,
    layer_insert: usize
}

impl LayerStack {
    pub fn new() -> LayerStack {
        LayerStack { layers: vec![], layer_insert: 0 }
    }

    pub fn get_layers(&mut self) -> &mut Vec<Box<dyn Layer>> {
        &mut self.layers
    }

    pub fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.layer_insert, layer);
        self.layer_insert += 1
    }

    pub fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    pub fn pop_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.remove(self.layers.iter().position(|x| layer.eq(&**x)).expect("Element not in the vector."));
        self.layer_insert -= 1;
    }

    pub fn pop_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.remove(self.layers.iter().position(|x| overlay.eq(&**x)).expect("Element not in the vector."));
    }
}

pub enum LayerParam<'a> {
    Window(&'a WinitWindow)
}