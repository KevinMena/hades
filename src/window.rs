use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{Surface, SurfaceAttributesBuilder, WindowSurface},
};

use winit::{
    window::{Window, WindowBuilder},
    event_loop::EventLoop
};

use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;

use crate::logger::*;

pub struct WindowData {
    title: &'static str,
    width: u32,
    height: u32,
    vsync: bool
}

impl WindowData {
    pub fn default() -> WindowData {
        WindowData {title: "Hades Engine", width: Self::default_width(), height: Self::default_height(), vsync: true}
    }

    pub fn default_width() -> u32 {
        1280
    }

    pub fn default_height() -> u32 {
        720
    }

    pub fn get_title(&self) -> &str {
        self.title
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_vsync(&mut self, enabled: bool) {
        self.vsync = enabled
    }

    pub fn is_vsync(&self) -> bool {
        self.vsync
    }
}

pub struct WindowSystem;

impl WindowSystem {
    pub fn init_window(event_loop: &EventLoop<()>) -> (Window, Surface<WindowSurface>, PossiblyCurrentContext) {
        let window_data = WindowData::default();
        let window_builder = WindowBuilder::new()
                    .with_title(window_data.get_title())
                    .with_inner_size(winit::dpi::LogicalSize::new(window_data.get_width(), window_data.get_height()));

        // Create opengl window using glutin for setup
        let (window, cfg) = glutin_winit::DisplayBuilder::new()
            .with_window_builder(Some(window_builder))
            .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| {
                configs.next().unwrap()
            })
            .expect("Failed to create OpenGL window");

        let window = window.unwrap();

        hds_core_info!("Creating window {} ({}, {})", window_data.get_title(), window_data.get_width(), window_data.get_height());

        // Create context and surface
        let ctx_attr = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
        let context = unsafe {
            cfg.display()
                .create_context(&cfg, &ctx_attr)
                .expect("Failed to create OpenGL context")
        };

        let sur_attr = SurfaceAttributesBuilder::<WindowSurface>::new()
            .with_srgb(Some(true))
            .build(
                window.raw_window_handle(), 
                NonZeroU32::new(WindowData::default_width()).unwrap(), 
                NonZeroU32::new(WindowData::default_height()).unwrap()
            );
        
        let surface = unsafe {
            cfg.display()
                .create_window_surface(&cfg, &sur_attr)
                .expect("Failed to create OpenGL surface")
        };

        let context = context.make_current(&surface).expect("Failed to create OpenGL context");

        (window, surface, context)
    }
}