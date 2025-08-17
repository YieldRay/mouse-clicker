//! 跨平台鼠标连点器
//! 
//! 跨平台鼠标自动点击工具
//! 支持Windows/macOS/Linux

mod app;
mod core;
mod config;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 启动应用程序
    app::run_app().map_err(|e| Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)) as Box<dyn std::error::Error>)
}