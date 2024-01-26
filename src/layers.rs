use crate::events::Event;

pub trait Layer {
    fn eq(&self, other: &dyn Layer) -> bool;

    fn on_attach(&self);
    fn on_detach(&self);
    fn on_update(&self);
    fn on_event(&self, event: &Event);

    fn get_name(&self) -> &str;
}

// TODO: Check way to remove properly the extra reference we have
pub struct LayerStack<'a> {
    layers: Vec<Box<&'a dyn Layer>>,
    layer_insert: usize
}

impl<'a> LayerStack<'a> {
    pub fn new() -> LayerStack<'a> {
        LayerStack { layers: vec![], layer_insert: 0 }
    }

    pub fn get_layers(&self) -> &Vec<Box<&'a dyn Layer>> {
        &self.layers
    }

    pub fn push_layer(&mut self, layer: &'a dyn Layer) {
        self.layers.insert(self.layer_insert, Box::new(layer));
        self.layer_insert += 1
    }

    pub fn push_overlay(&mut self, overlay: &'a dyn Layer) {
        self.layers.push(Box::new(overlay));
    }

    pub fn pop_layer(&mut self, layer: &'a dyn Layer) {
        self.layers.remove(self.layers.iter().position(|x| layer.eq(**x)).expect("Element not in the vector."));
        self.layer_insert -= 1;
    }

    pub fn pop_overlay(&mut self, overlay: &'a dyn Layer) {
        self.layers.remove(self.layers.iter().position(|x| overlay.eq(**x)).expect("Element not in the vector."));
    }
}