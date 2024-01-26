pub struct WindowData {
    title: &'static str,
    width: u32,
    height: u32,
    vsync: bool
}

impl WindowData {
    pub fn default() -> WindowData {
        WindowData {title: "Hades Engine", width: 1280, height: 720, vsync: true}
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