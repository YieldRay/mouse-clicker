//! 主窗口UI实现
//!
//! 使用egui实现连点器的主界面

use crate::config::{AppSettings, MouseButton, FunctionKey};
use crate::core::{ClickerManager, ClickerStatus, ClickerState};
use crate::core::mouse::MouseController;
use crate::utils::Result;
use egui::{Context, Ui, RichText, Color32};

/// 主窗口应用程序状态
pub struct MainWindow {
    /// 应用设置
    settings: AppSettings,
    /// 连点器管理器
    clicker_manager: Option<ClickerManager>,
    /// 当前状态
    current_status: ClickerStatus,
    /// 错误消息
    error_message: Option<String>,
    /// UI状态
    ui_state: UiState,
}

/// UI状态
#[derive(Default, Clone)]
struct UiState {
    /// 间隔输入框的文本
    interval_text: String,
    /// 点击次数输入框的文本
    count_text: String,
    /// 是否显示无限点击
    unlimited_clicks: bool,
    /// 上次热键触发时间，用于防抖
    last_hotkey_time: Option<std::time::Instant>,
}

impl MainWindow {
    /// 创建新的主窗口
    pub fn new() -> Self {
        let settings = AppSettings::default();
        let ui_state = UiState {
            interval_text: settings.interval_ms.to_string(),
            count_text: settings.click_count.map_or(String::new(), |c| c.to_string()),
            unlimited_clicks: settings.click_count.is_none(),
            last_hotkey_time: None,
        };

        Self {
            settings,
            clicker_manager: None,
            current_status: ClickerStatus::default(),
            error_message: None,
            ui_state,
        }
    }

    /// 初始化连点器管理器
    pub fn initialize_clicker(&mut self) -> Result<()> {
        match ClickerManager::new(self.settings.clone()) {
            Ok(manager) => {
                self.clicker_manager = Some(manager);
                Ok(())
            }
            Err(e) => {
                self.error_message = Some(format!("初始化连点器失败: {}", e));
                Err(e)
            }
        }
    }

    /// 更新UI
    pub fn update(&mut self, ctx: &Context) {
        // 更新连点器状态
        self.update_clicker_status();

        // 检查热键
        self.check_hotkey();

        // 绘制菜单栏
        self.draw_menu_bar(ctx);

        // 绘制主面板
        egui::CentralPanel::default().show(ctx, |ui| {
            self.draw_main_content(ui);
        });

        // 处理错误消息
        self.show_error_dialog(ctx);
    }

    /// 更新连点器状态
    fn update_clicker_status(&mut self) {
        if let Some(manager) = &self.clicker_manager {
            self.current_status = manager.get_status();
        }
    }

    /// 检查热键
    fn check_hotkey(&mut self) {
        if let Some(manager) = &mut self.clicker_manager {
            if manager.check_hotkey() {
                let now = std::time::Instant::now();
                
                // 防抖机制：如果距离上次热键触发不到500ms，则忽略
                let should_trigger = match self.ui_state.last_hotkey_time {
                    Some(last_time) => now.duration_since(last_time).as_millis() > 500,
                    None => true,
                };
                
                if should_trigger {
                    self.ui_state.last_hotkey_time = Some(now);
                    if let Err(e) = manager.toggle() {
                        self.error_message = Some(format!("热键操作失败: {}", e));
                    }
                    log::info!("热键触发，切换连点器状态");
                }
            }
        }
    }

    /// 绘制菜单栏
    fn draw_menu_bar(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("设置", |ui| {
                    if MouseController::is_macos() {
                        if ui.button("打开辅助功能设置").clicked() {
                            if let Err(e) = MouseController::open_privacy_settings() {
                                self.error_message = Some(format!("无法打开系统设置: {}", e));
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                    }
                    
                    if ui.button("关于").clicked() {
                        self.error_message = Some("Mouse Clicker v0.1.0".to_string());
                        ui.close_menu();
                    }
                });
            });
        });
    }

    /// 绘制主要内容
    fn draw_main_content(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            // 标题
            ui.add_space(10.0);
            ui.heading(RichText::new("Mouse Clicker").size(20.0));
            ui.add_space(15.0);

            // 设置区域
            self.draw_settings_section(ui);
            
            ui.add_space(15.0);
            
            // 状态显示区域
            self.draw_status_section(ui);
            
            ui.add_space(15.0);
            
            // 控制按钮区域
            self.draw_control_section(ui);
        });
    }

    /// 绘制设置区域
    fn draw_settings_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            let is_enabled = self.current_status.state == ClickerState::Stopped;
            ui.add_enabled_ui(is_enabled, |ui| {
                // 点击间隔设置
                ui.horizontal(|ui| {
                        ui.label("点击间隔 (毫秒):");
                    ui.add_space(10.0);
                    
                    let response = ui.text_edit_singleline(&mut self.ui_state.interval_text);
                    if response.changed() {
                        if let Ok(interval) = self.ui_state.interval_text.parse::<u64>() {
                            if interval > 0 && interval <= 60000 {
                                self.settings.interval_ms = interval;
                                self.update_clicker_settings();
                            }
                        }
                    }
                });

                ui.add_space(8.0);

                // 鼠标按键选择
                ui.horizontal(|ui| {
                    ui.label("鼠标按键:");
                    ui.add_space(10.0);
                    
                    egui::ComboBox::from_id_source("mouse_button")
                        .selected_text(self.settings.mouse_button.to_string())
                        .show_ui(ui, |ui| {
                            let buttons = [
                                MouseButton::Left,
                                MouseButton::Right,
                                MouseButton::LeftLongPress,
                                MouseButton::RightLongPress,
                                MouseButton::ScrollUp,
                                MouseButton::ScrollDown,
                            ];
                            for &button in &buttons {
                                if ui.selectable_value(&mut self.settings.mouse_button, button, button.to_string()).changed() {
                                    self.update_clicker_settings();
                                }
                            }
                        });
                });

                ui.add_space(8.0);

                // 热键设置
                ui.horizontal(|ui| {
                    ui.label("热键:");
                    ui.add_space(10.0);
                    
                    egui::ComboBox::from_id_source("hotkey")
                        .selected_text(self.settings.hotkey.to_string())
                        .show_ui(ui, |ui| {
                            for &key in &FunctionKey::all() {
                                if ui.selectable_value(&mut self.settings.hotkey, key, key.to_string()).changed() {
                                    self.update_clicker_settings();
                                }
                            }
                        });
                });

                ui.add_space(8.0);

                // 点击次数设置
                ui.horizontal(|ui| {
                    ui.label("点击次数:");
                    ui.add_space(10.0);
                    
                    if ui.checkbox(&mut self.ui_state.unlimited_clicks, "无限制").changed() {
                        if self.ui_state.unlimited_clicks {
                            self.settings.click_count = None;
                            self.ui_state.count_text.clear();
                        } else {
                            self.settings.click_count = Some(100);
                            self.ui_state.count_text = "100".to_string();
                        }
                        self.update_clicker_settings();
                    }
                    
                    if !self.ui_state.unlimited_clicks {
                        ui.add_space(10.0);
                        let response = ui.text_edit_singleline(&mut self.ui_state.count_text);
                        if response.changed() {
                            if let Ok(count) = self.ui_state.count_text.parse::<u32>() {
                                if count > 0 && count <= 1000000 {
                                    self.settings.click_count = Some(count);
                                    self.update_clicker_settings();
                                }
                            }
                        }
                    }
                });
            });
        });
    }

    /// 绘制状态显示区域
    fn draw_status_section(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                // 状态指示器
                let (color, text) = match self.current_status.state {
                    ClickerState::Stopped => (Color32::GRAY, "已停止"),
                    ClickerState::Running => (Color32::GREEN, "运行中"),
                };
                ui.colored_label(color, format!("状态: {}", text));

                ui.add_space(20.0);
                
                // 点击计数
                ui.label(format!("点击次数: {}", self.current_status.click_count));
                if let Some(target) = self.current_status.target_count {
                    ui.label(format!("/ {}", target));
                }
                ui.add_space(20.0);
                
                // 运行时间
                if self.current_status.runtime_seconds > 0 {
                    let minutes = self.current_status.runtime_seconds / 60;
                    let seconds = self.current_status.runtime_seconds % 60;
                    ui.label(format!("运行时间: {:02}:{:02}", minutes, seconds));
                }
            });
        });
    }

    /// 绘制控制按钮区域
    fn draw_control_section(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let button_size = egui::Vec2::new(100.0, 30.0);
            match self.current_status.state {
                ClickerState::Stopped => {
                    if ui.add_sized(button_size, egui::Button::new("开始")).clicked() {
                        self.toggle_clicking();
                    }
                }
                ClickerState::Running => {
                    if ui.add_sized(button_size, egui::Button::new("停止")).clicked() {
                        self.toggle_clicking();
                    }
                }
            }
        });
    }

    /// 显示错误对话框
    fn show_error_dialog(&mut self, ctx: &Context) {
        if let Some(error) = self.error_message.clone() {
            egui::Window::new("错误")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(error);
                    ui.add_space(10.0);
                    if ui.button("确定").clicked() {
                        self.error_message = None;
                    }
                });
        }
    }

    

    /// 切换点击状态（与热键行为一致）
    fn toggle_clicking(&mut self) {
        if let Some(manager) = &mut self.clicker_manager {
            if let Err(e) = manager.toggle() {
                self.error_message = Some(format!("切换连点器状态失败: {}", e));
            }
        }
    }

    /// 更新连点器设置
    fn update_clicker_settings(&mut self) {
        if let Some(manager) = &mut self.clicker_manager {
            if let Err(e) = manager.update_settings(self.settings.clone()) {
                self.error_message = Some(format!("更新设置失败: {}", e));
            }
        }
    }

    /// 获取当前设置
    pub fn get_settings(&self) -> &AppSettings {
        &self.settings
    }
}

impl Default for MainWindow {
    fn default() -> Self {
        Self::new()
    }
}