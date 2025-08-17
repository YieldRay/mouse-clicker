//! 连点器核心逻辑模块
//!
//! 实现自动点击的核心逻辑

use crate::config::AppSettings;
use crate::core::hotkey::HotkeyManager;
use crate::core::mouse::MouseController;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// 连点器状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClickerState {
    Stopped,
    Running,
}

impl Default for ClickerState {
    fn default() -> Self {
        Self::Stopped
    }
}

/// 连点器状态信息
#[derive(Debug, Clone)]
pub struct ClickerStatus {
    pub state: ClickerState,
    pub click_count: u32,
    pub target_count: Option<u32>,
    pub runtime_seconds: u64,
}

impl Default for ClickerStatus {
    fn default() -> Self {
        Self {
            state: ClickerState::Stopped,
            click_count: 0,
            target_count: None,
            runtime_seconds: 0,
        }
    }
}

/// 连点器管理器
pub struct ClickerManager {
    settings: AppSettings,
    hotkey_manager: HotkeyManager,
    is_running: Arc<AtomicBool>,
    click_count: Arc<AtomicU32>,
    start_time: Option<Instant>,
}

impl ClickerManager {
    /// 创建新的连点器管理器
    pub fn new(settings: AppSettings) -> Result<Self, String> {
        let mut hotkey_manager = HotkeyManager::new()?;
        // 注册热键
        hotkey_manager.set_hotkey(settings.hotkey)?;

        Ok(Self {
            settings,
            hotkey_manager,
            is_running: Arc::new(AtomicBool::new(false)),
            click_count: Arc::new(AtomicU32::new(0)),
            start_time: None,
        })
    }

    /// 启动连点器
    pub fn start(&mut self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Ok(());
        }

        self.is_running.store(true, Ordering::Relaxed);
        self.click_count.store(0, Ordering::Relaxed);
        self.start_time = Some(Instant::now());

        let is_running = self.is_running.clone();
        let click_count = self.click_count.clone();
        let interval = self.settings.interval_ms;
        let target_count = self.settings.click_count;
        let mouse_button = self.settings.mouse_button;

        // 在新线程中执行连点逻辑
        thread::spawn(move || {
            let mut mouse = match MouseController::new() {
                Ok(m) => m,
                Err(e) => {
                    log::error!("创建鼠标控制器失败: {}", e);
                    return;
                }
            };

            // 首次启动时等待一个间隔时间再开始点击
            log::debug!("连点器启动，等待 {}ms 后开始第一次点击", interval);
            thread::sleep(Duration::from_millis(interval));

            while is_running.load(Ordering::Relaxed) {
                let current_count = click_count.load(Ordering::Relaxed);

                // 检查是否达到目标点击次数
                if let Some(target) = target_count {
                    if current_count >= target {
                        is_running.store(false, Ordering::Relaxed);
                        break;
                    }
                }

                // 执行点击
                match mouse.click(mouse_button) {
                    Ok(_) => {
                        // 只有在点击成功时才增加计数器
                        click_count.fetch_add(1, Ordering::Relaxed);
                        log::debug!(
                            "执行点击: {:?}, 当前计数: {}",
                            mouse_button,
                            current_count + 1
                        );
                    }
                    Err(e) => {
                        log::error!("点击操作失败: {}", e);
                        // 如果是权限问题，继续尝试而不是停止
                        if e.contains("权限")
                            || e.contains("permission")
                            || e.contains("accessibility")
                        {
                            log::warn!("检测到权限问题，请在系统设置中授予辅助功能权限");
                            // 继续运行，但不增加计数器
                        } else {
                            // 其他错误则停止运行
                            is_running.store(false, Ordering::Relaxed);
                            break;
                        }
                    }
                }

                // 等待间隔时间
                thread::sleep(Duration::from_millis(interval));
            }
        });

        log::info!("连点器已启动");
        Ok(())
    }

    /// 停止连点器
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        self.start_time = None;
        log::info!("连点器已停止");
    }

    /// 检查热键是否被按下
    pub fn check_hotkey(&self) -> bool {
        self.hotkey_manager.check_hotkey_pressed()
    }

    /// 更新设置
    pub fn update_settings(&mut self, new_settings: AppSettings) -> Result<(), String> {
        // 如果热键改变了，重新注册
        if self.settings.hotkey != new_settings.hotkey {
            self.hotkey_manager.set_hotkey(new_settings.hotkey)?;
        }

        self.settings = new_settings;
        log::info!("连点器设置已更新");
        Ok(())
    }

    /// 获取当前状态
    pub fn get_status(&self) -> ClickerStatus {
        let state = if self.is_running.load(Ordering::Relaxed) {
            ClickerState::Running
        } else {
            ClickerState::Stopped
        };

        let runtime_seconds = self
            .start_time
            .map(|start| start.elapsed().as_secs())
            .unwrap_or(0);

        ClickerStatus {
            state,
            click_count: self.click_count.load(Ordering::Relaxed),
            target_count: self.settings.click_count,
            runtime_seconds,
        }
    }

    /// 切换运行状态（用于热键控制）
    pub fn toggle(&mut self) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            self.stop();
        } else {
            self.start()?;
        }
        Ok(())
    }
}
