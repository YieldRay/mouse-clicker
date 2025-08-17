//! 应用程序设置管理
//!
//! 负责配置文件的读取、保存和默认值管理

use crate::utils::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 鼠标按键类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MouseButton {
    /// 左键单击
    Left,
    /// 右键单击
    Right,
    /// 左键长按
    LeftLongPress,
    /// 右键长按
    RightLongPress,
    /// 向上滚动
    ScrollUp,
    /// 向下滚动
    ScrollDown,
}

impl Default for MouseButton {
    fn default() -> Self {
        Self::Left
    }
}

impl std::fmt::Display for MouseButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::Left => "左键单击",
            Self::Right => "右键单击",
            Self::LeftLongPress => "左键长按",
            Self::RightLongPress => "右键长按",
            Self::ScrollUp => "向上滚动",
            Self::ScrollDown => "向下滚动",
        };
        write!(f, "{}", text)
    }
}

/// 功能键类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FunctionKey {
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
}

impl Default for FunctionKey {
    fn default() -> Self {
        Self::F2
    }
}

impl std::fmt::Display for FunctionKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FunctionKey {
    /// 获取所有可用的功能键
    pub fn all() -> Vec<FunctionKey> {
        vec![
            Self::F1,
            Self::F2,
            Self::F3,
            Self::F4,
            Self::F5,
            Self::F6,
            Self::F7,
            Self::F8,
            Self::F9,
            Self::F10,
            Self::F11,
            Self::F12,
        ]
    }
}

/// 应用程序配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 点击间隔时间（毫秒）
    pub interval_ms: u64,
    ///鼠标按键类型
    pub mouse_button: MouseButton,
    /// 点击次数（None表示无限次）
    pub click_count: Option<u32>,
    /// 热键设置
    pub hotkey: FunctionKey,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            interval_ms: 1000,
            mouse_button: MouseButton::default(),
            click_count: None,
            hotkey: FunctionKey::default(),
        }
    }
}

impl AppSettings {
    /// 验证设置的有效性
    pub fn validate(&self) -> Result<()> {
        if self.interval_ms == 0 {
            return Err("点击间隔不能为0".to_string());
        }

        if self.interval_ms > 60000 {
            return Err("点击间隔不能超过60秒".to_string());
        }

        if let Some(count) = self.click_count {
            if count == 0 {
                return Err("点击次数不能为0".to_string());
            }
            if count > 1000000 {
                return Err("点击次数不能超过100万次".to_string());
            }
        }

        Ok(())
    }
}

/// 设置管理器
pub struct SettingsManager {
    config_path: PathBuf,
    settings: AppSettings,
}

impl SettingsManager {
    /// 创建新的设置管理器
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        let settings = Self::load_from_file(&config_path).unwrap_or_default();

        Ok(Self {
            config_path,
            settings,
        })
    }

    /// 获取配置文件路径
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| "无法获取配置目录".to_string())?;

        let app_config_dir = config_dir.join("mouse-clicker");
        std::fs::create_dir_all(&app_config_dir).map_err(|e| format!("创建配置目录失败: {}", e))?;

        Ok(app_config_dir.join("settings.json"))
    }

    /// 从文件加载设置
    fn load_from_file(path: &PathBuf) -> Result<AppSettings> {
        if !path.exists() {
            return Ok(AppSettings::default());
        }

        let content =
            std::fs::read_to_string(path).map_err(|e| format!("读取配置文件失败: {}", e))?;
        let settings: AppSettings =
            serde_json::from_str(&content).map_err(|e| format!("解析配置文件失败: {}", e))?;
        settings.validate()?;

        Ok(settings)
    }

    /// 获取当前设置
    #[allow(dead_code)]
    pub fn get(&self) -> &AppSettings {
        &self.settings
    }

    /// 更新设置
    pub fn update(&mut self, settings: AppSettings) -> Result<()> {
        settings.validate()?;
        self.settings = settings;
        Ok(())
    }

    /// 保存设置到文件
    pub fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.settings)
            .map_err(|e| format!("序列化设置失败: {}", e))?;
        std::fs::write(&self.config_path, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))?;
        log::info!("设置已保存到: {:?}", self.config_path);
        Ok(())
    }
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self {
            config_path: PathBuf::from("settings.json"),
            settings: AppSettings::default(),
        }
    }
}
