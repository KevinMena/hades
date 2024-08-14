use glow::HasContext;
use glutin::{
    context::PossiblyCurrentContext,
    display::{GetGlDisplay, GlDisplay}};
use imgui::{
    Context, Style, BackendFlags
};

use imgui_glow_renderer::AutoRenderer;
use imgui_winit_support::WinitPlatform;
use winit::window::Window;
use std::time::Instant;

use crate::{
    events::{Event, EventType}, layers::{Layer, LayerParam}
};

pub struct ImguiLayer{
    imgui: Option<Context>,
    platform: Option<WinitPlatform>,
    renderer: Option<AutoRenderer>,
    last_frame: Instant
}

impl ImguiLayer {
    pub fn new(window: &Window, context: &PossiblyCurrentContext) -> ImguiLayer {
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

        // OpenGL glow context
        let gl = unsafe {
            glow::Context::from_loader_function_cstr(|s| context.display().get_proc_address(s).cast())
        };

        let renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui)
            .expect("Failed to create OpenGL renderer");

        ImguiLayer { imgui: Some(imgui), platform: Some(winit_platform), renderer: Some(renderer), last_frame: Instant::now() } 
    }
}

impl Layer for ImguiLayer {
    fn on_attach(&mut self, _: LayerParam) {

        // // Gather window where it's going to render
        // let window  = match param {
        //     LayerParam::Window(window) => window
        // };

        // let mut imgui = Context::create();
        // imgui.set_ini_filename(None);
        // Style::use_dark_colors(imgui.style_mut());

        // imgui.io_mut().backend_flags |= BackendFlags::HAS_MOUSE_CURSORS;
        // imgui.io_mut().backend_flags |= BackendFlags::HAS_SET_MOUSE_POS;

        // let mut winit_platform = WinitPlatform::init(&mut imgui);
        // winit_platform.attach_window(imgui.io_mut(), window, imgui_winit_support::HiDpiMode::Rounded);
        
        // imgui
        //     .fonts()
        //     .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        // imgui.io_mut().font_global_scale = (1.0 / winit_platform.hidpi_factor()) as f32;

        // self.imgui = Some(imgui);
        // self.platform = Some(winit_platform);
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

    fn on_event(&mut self, event: &Event) -> bool {
        match event.get_event_type() {
            EventType::NewEvents => {
                let now = Instant::now();

                if let Some(imgui) = &mut self.imgui {                    
                    imgui
                        .io_mut()
                        .update_delta_time(now.duration_since(self.last_frame))
                };

                self.last_frame = now;
            },
            EventType::AboutToWait(window) => {
                if let Some(platform) = &mut self.platform {
                    platform
                        .prepare_frame(self.imgui.as_mut().unwrap().io_mut(), window)
                        .unwrap();
                }
            },
            EventType::WindowRedrawRequest(window) => {
                unsafe { 
                    if let Some(renderer) = &self.renderer {
                        renderer.gl_context().clear(glow::COLOR_BUFFER_BIT)
                    }
                };

                let ui =  match &mut self.imgui {
                    Some(imgui) => imgui.frame(),
                    None => todo!(),
                };

                // For now only show the demo window in imgui
                ui.show_demo_window(&mut true);

                if let Some(platform) = &mut self.platform {
                    platform.prepare_render(ui, window)
                };

                let draw_data =  match &mut self.imgui {
                    Some(imgui) => imgui.render(),
                    None => todo!(),
                };

                if let Some(renderer) = &mut self.renderer {
                    renderer.render(draw_data)
                        .expect("Error rendering imgui");
                }
            },
            _ => ()
        }

        false
    }

    fn get_name(&self) -> &str {
        "ImguiLayer"
    }
}