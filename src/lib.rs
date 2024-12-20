pub mod randomusert_lib;



pub struct Window {
    title: String,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
        }
    }

    pub fn show(&self) {
        //platform::create_window(&self.title, self.width, self.height);
        eprintln!("disbaled, because issues!");
    }
}

// Import platform-specific implementations
#[cfg(target_os = "windows")]
mod platform;
#[cfg(target_os = "linux")]
mod platform;
#[cfg(target_os = "macos")]
eprintln!("not implemented!");