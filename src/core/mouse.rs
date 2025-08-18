//! 跨平台鼠标操作模块
//!
//! 提供统一鼠标点击、长按和滚轮操作接口

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

    #[cfg(target_os = "windows")]
    pub fn is_windows() -> bool {
        true
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_windows() -> bool {
        false
    }

    /// 检查是否以管理员权限运行 (仅Windows)
    #[cfg(target_os = "windows")]
    pub fn is_admin() -> bool {
        use is_elevated::is_elevated;
        return is_elevated();
    }

    #[cfg(not(target_os = "windows"))]
    pub fn is_admin() -> bool {
        false
    }

    /// 以管理员权限重启应用程序 (仅Windows)
    #[cfg(target_os = "windows")]
    pub fn restart_as_admin() -> Result<(), String> {
        use std::env;
        use std::iter::once;
        use std::os::windows::ffi::OsStrExt;
        use std::process;
        use windows::core::{w, PCWSTR};
        use windows::Win32::UI::Shell::{ShellExecuteExW, SHELLEXECUTEINFOW};
        use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

        let exe = env::current_exe().map_err(|e| format!("获取当前程序路径失败: {}", e))?;
        let exe_w: Vec<u16> = exe.as_os_str().encode_wide().chain(once(0)).collect();

        let mut sei = SHELLEXECUTEINFOW {
            cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
            lpVerb: w!("runas"), // 使用 "runas" 动词请求管理员权限
            lpFile: PCWSTR(exe_w.as_ptr()),
            nShow: SW_SHOWNORMAL.0, // 显示新进程的窗口
            ..Default::default()
        };

        unsafe {
            ShellExecuteExW(&mut sei as *mut _)
                .map_err(|e| format!("以管理员权限重启失败: {}", e))?;

            log::info!("正在以管理员权限重启应用程序...");
            process::exit(0);
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn restart_as_admin() -> Result<(), String> {
        Err("此功能仅在Windows上可用".to_string())
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
                log::info!("已打开系统设置 - 隐私与安全性 - 辅助功能");
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
