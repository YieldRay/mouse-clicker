/// 检测系统暗色模式（跨平台）
pub fn detect_system_dark_mode() -> bool {
    if let Ok(mode) = dark_light::detect() {
        match mode {
            dark_light::Mode::Dark => true,
            dark_light::Mode::Light => false,
            dark_light::Mode::Unspecified => false,
        }
    } else {
        false
    }
}
