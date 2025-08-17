//! 跨平台鼠标操作模块
//!
//! 提供统一的鼠标点击、长按和滚轮操作接口

use crate::config::MouseButton;
use enigo::{Enigo, Mouse, Settings};
use std::time::Duration;

/// 鼠标控制器
pub struct MouseController {
    enigo: Enigo,
}

impl MouseController {
    /// 创建新的鼠标控制器
    pub fn new() -> Result<Self, String> {
        let enigo =
            Enigo::new(&Settings::default()).map_err(|e| format!("初始化鼠标控制器失败: {}", e))?;
        Ok(Self { enigo })
    }

    #[cfg(target_os = "macos")]
    pub fn is_macos() -> bool {
        true
    }

    #[cfg(not(target_os = "macos"))]
    pub fn is_macos() -> bool {
        false
    }

    /// 打开macOS系统设置到隐私页面
    #[cfg(target_os = "macos")]
    pub fn open_privacy_settings() -> Result<(), String> {
        use std::process::Command;

        match Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn()
        {
            Ok(_) => {
                log::info!("已打开系统设置 - 隐私与安全性- 辅助功能");
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("无法打开系统设置: {}", e);
                log::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    /// 打开Windows系统设置到鼠标设置页面
    #[cfg(target_os = "windows")]
    pub fn open_privacy_settings() -> Result<(), String> {
        use std::process::Command;

        match Command::new("ms-settings:mousetouchpad").spawn() {
            Ok(_) => {
                log::info!("已打开Windows系统设置 - 鼠标设置");
                Ok(())
            }
            Err(e) => {
                let error_msg = format!("无法打开系统设置: {}", e);
                log::error!("{}", error_msg);
                Err(error_msg)
            }
        }
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    pub fn open_privacy_settings() -> Result<(), String> {
        Err("此功能仅在macOS和Windows上可用".to_string())
    }

    /// 执行鼠标点击操作
    pub fn click(&mut self, button: MouseButton) -> Result<(), String> {
        match button {
            MouseButton::Left => self
                .enigo
                .button(enigo::Button::Left, enigo::Direction::Click)
                .map_err(|e| format!("左键点击失败: {}", e)),
            MouseButton::Right => self
                .enigo
                .button(enigo::Button::Right, enigo::Direction::Click)
                .map_err(|e| format!("右键点击失败: {}", e)),
            MouseButton::LeftLongPress => {
                self.enigo
                    .button(enigo::Button::Left, enigo::Direction::Press)
                    .map_err(|e| format!("左键按下失败: {}", e))?;
                std::thread::sleep(Duration::from_millis(100));
                self.enigo
                    .button(enigo::Button::Left, enigo::Direction::Release)
                    .map_err(|e| format!("左键释放失败: {}", e))
            }
            MouseButton::RightLongPress => {
                self.enigo
                    .button(enigo::Button::Right, enigo::Direction::Press)
                    .map_err(|e| format!("右键按下失败: {}", e))?;
                std::thread::sleep(Duration::from_millis(100));
                self.enigo
                    .button(enigo::Button::Right, enigo::Direction::Release)
                    .map_err(|e| format!("右键释放失败: {}", e))
            }
            MouseButton::ScrollUp => self
                .enigo
                .scroll(3, enigo::Axis::Vertical)
                .map_err(|e| format!("向上滚动失败: {}", e)),
            MouseButton::ScrollDown => self
                .enigo
                .scroll(-3, enigo::Axis::Vertical)
                .map_err(|e| format!("向下滚动失败: {}", e)),
        }
    }
}
