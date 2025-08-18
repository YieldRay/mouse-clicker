//! 跨平台全局热键监听模块
//!
//! 提供F1-F12功能键的全局监听功能

use crate::config::FunctionKey;
use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};

/// 热键管理器
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    current_hotkey: Option<HotKey>,
}

impl HotkeyManager {
    /// 创建新的热键管理器
    pub fn new() -> Result<Self, String> {
        let manager =
            GlobalHotKeyManager::new().map_err(|e| format!("初始化热键管理器失败: {}", e))?;

        Ok(Self {
            manager,
            current_hotkey: None,
        })
    }

    /// 设置热键
    pub fn set_hotkey(&mut self, key: FunctionKey) -> Result<(), String> {
        // 先注销之前的热键
        if let Some(hotkey) = &self.current_hotkey {
            let _ = self.manager.unregister(*hotkey);
        }

        // 注册新热键
        let code = self.function_key_to_code(key)?;
        let hotkey = HotKey::new(Some(Modifiers::empty()), code);

        self.manager
            .register(hotkey)
            .map_err(|e| format!("注册热键{:?} 失败: {}", key, e))?;

        self.current_hotkey = Some(hotkey);
        log::info!("成功注册热键: {:?}", key);
        Ok(())
    }

    /// 检查热键是否被按下
    pub fn check_hotkey_pressed(&self) -> bool {
        if let Ok(event) = global_hotkey::GlobalHotKeyEvent::receiver().try_recv() {
            log::debug!("热键触发: {:?}", event);
            return event.state == global_hotkey::HotKeyState::Pressed;
        }
        false
    }

    /// 将FunctionKey转换为global_hotkey的Code
    fn function_key_to_code(&self, key: FunctionKey) -> Result<Code, String> {
        let code = match key {
            FunctionKey::F1 => Code::F1,
            FunctionKey::F2 => Code::F2,
            FunctionKey::F3 => Code::F3,
            FunctionKey::F4 => Code::F4,
            FunctionKey::F5 => Code::F5,
            FunctionKey::F6 => Code::F6,
            FunctionKey::F7 => Code::F7,
            FunctionKey::F8 => Code::F8,
            FunctionKey::F9 => Code::F9,
            FunctionKey::F10 => Code::F10,
            FunctionKey::F11 => Code::F11,
            FunctionKey::F12 => Code::F12,
        };
        Ok(code)
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        if let Some(hotkey) = &self.current_hotkey {
            let _ = self.manager.unregister(*hotkey);
        }
    }
}
