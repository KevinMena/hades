use crate::events::Event;

// TODO: Make the comparison for the traits so we can check if two objects are the same
pub trait Layer {
    fn eq(&self, _other: &dyn Layer) -> bool {
        true
    }

    fn on_attach(&self);
    fn on_detach(&self);
    fn on_update(&self);
    fn on_event(&self, event: &Event);

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

    pub fn get_layers(&self) -> &Vec<Box<dyn Layer>> {
        &self.layers
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