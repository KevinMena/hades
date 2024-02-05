use imgui::{
    Context, Style, BackendFlags
};
use imgui_winit_support::WinitPlatform;
use std::time::Instant;

use crate::{
    layers::{Layer, LayerParam},
    events::{Event, EventType}
};

pub struct ImguiLayer{
    imgui: Option<Context>,
    platform: Option<WinitPlatform>,
    last_frame: Instant
}

impl ImguiLayer {
    pub fn new() -> ImguiLayer {
        ImguiLayer { imgui: None, platform: None, last_frame: Instant::now() } 
    }
}

impl Layer for ImguiLayer {
    fn on_attach(&mut self, param: LayerParam) {
        let window = match param {
            LayerParam::Window(window) => window,
        };

        let mut imgui = Context::create();
        imgui.set_ini_filename(None);
        Style::use_dark_colors(imgui.style_mut());

        imgui.io_mut().backend_flags |= BackendFlags::HAS_MOUSE_CURSORS;
        imgui.io_mut().backend_flags |= BackendFlags::HAS_SET_MOUSE_POS;

        let mut winit_platform = WinitPlatform::init(&mut imgui);
        winit_platform.attach_window(imgui.io_mut(), window, imgui_winit_support::HiDpiMode::Rounded);
        
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        imgui.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

        self.imgui = Some(imgui);
        self.platform = Some(winit_platform);
        self.last_frame = Instant::now();
    }

    fn on_detach(&mut self) {
        todo!()
    }

    fn on_update(&mut self) {
        let now = Instant::now();
        self.imgui.as_mut().unwrap().io_mut().update_delta_time(now - self.last_frame);
        self.last_frame = now;
    }

    fn on_event(&mut self, event: &Event) {
        match event.get_event_type() {
            EventType::AboutToWait(window) => {
                self.platform
                        .as_ref()
                        .unwrap()
                        .prepare_frame(self.imgui.as_mut().unwrap().io_mut(), window)
                        .expect("Error drawing the frame");
                window.request_redraw();
            },
            EventType::WindowRedrawRequest => (),
            _ => ()
        }
    }

    fn get_name(&self) -> &str {
        "ImguiLayer"
    }
}