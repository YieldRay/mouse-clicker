//! 应用程序主模块
//!
//! 整合所有组件，实现eframe::App trait

use crate::config::SettingsManager;
use crate::ui::MainWindow;
use crate::utils::Result;
use eframe::egui;

pub struct MouseClickerApp {
    /// 主窗口
    main_window: MainWindow,
    /// 设置管理器
    settings_manager: SettingsManager,
    /// 初始化状态
    initialized: bool,
    /// 初始化错误
    init_error: Option<String>,
}

impl MouseClickerApp {
    /// 创建新的应用程序实例
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // 设置中文字体支持
        if let Err(e) = egui_chinese_font::setup_chinese_fonts(&cc.egui_ctx) {
            log::warn!("加载中文字体失败: {}", e);
        }

        //尝试加载保存的设置
        let settings_manager = SettingsManager::new().unwrap_or_default();

        // 创建主窗口
        let main_window = MainWindow::new();

        let app = Self {
            main_window,
            settings_manager,
            initialized: false,
            init_error: None,
        };

        // 尝试从持久化存储中恢复窗口状态
        if let Some(storage) = cc.storage {
            if let Some(window_state) = storage.get_string("window_state") {
                log::info!("恢复窗口状态: {:?}", window_state);
            }
        }

        app
    }

    /// 初始化应用程序
    fn initialize(&mut self) -> Result<()> {
        // 初始化主窗口的连点器
        self.main_window.initialize_clicker()?;
        log::info!("应用程序初始化完成");
        Ok(())
    }

    /// 保存设置
    fn save_settings(&mut self) -> Result<()> {
        let settings = self.main_window.get_settings().clone();
        self.settings_manager.update(settings)?;
        self.settings_manager.save()?;
        Ok(())
    }

    /// 显示初始化错误
    fn show_init_error(&mut self, ctx: &egui::Context) {
        if let Some(error) = self.init_error.clone() {
            egui::Window::new("初始化错误")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
                .show(ctx, |ui| {
                    ui.label("应用程序初始化失败:");
                    ui.add_space(10.0);
                    ui.label(error);
                    ui.add_space(10.0);

                    if ui.button("重试").clicked() {
                        self.init_error = None;
                        self.initialized = false;
                    }

                    if ui.button("退出").clicked() {
                        std::process::exit(1);
                    }
                });
        }
    }
}

impl eframe::App for MouseClickerApp {
    /// 更新应用程序
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 如果还没有初始化，尝试初始化
        if !self.initialized && self.init_error.is_none() {
            match self.initialize() {
                Ok(_) => {
                    self.initialized = true;
                    log::info!("应用程序初始化成功");
                }
                Err(e) => {
                    self.init_error = Some(e.clone());
                    log::error!("应用程序初始化失败: {}", e);
                }
            }
        }

        // 显示初始化错误（如果有）
        if self.init_error.is_some() {
            self.show_init_error(ctx);
            return;
        }

        // 如果已初始化，更新主窗口
        if self.initialized {
            self.main_window.update(ctx);
        } else {
            // 显示加载界面
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.spinner();
                    ui.add_space(20.0);
                    ui.label("正在初始化...");
                });
            });
        }

        // 请求重绘以保持UI响应
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }

    /// 应用程序关闭时的清理工作
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // 保存设置
        if let Err(e) = self.save_settings() {
            log::error!("保存设置失败: {}", e);
        }

        log::info!("应用程序正在退出");
    }

    /// 自动保存
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // 简化存储处理
        let _settings = self.main_window.get_settings();
        log::info!("保存应用程序状态");
    }
}

/// 应用程序启动器
pub fn run_app() -> Result<()> {
    // 初始化日志系统
    env_logger::init();
    log::info!("启动应用程序 run_app");

    // 配置应用程序窗口选项
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([306.0, 308.0])
            .with_min_inner_size([306.0, 308.0])
            .with_max_inner_size([306.0, 308.0])
            .with_resizable(true)
            .with_title("Mouse Clicker"),
        ..Default::default()
    };

    // 启动应用程序
    eframe::run_native(
        "Mouse Clicker",
        options,
        Box::new(|cc| Box::new(MouseClickerApp::new(cc))),
    )
    .map_err(|e| format!("启动应用程序失败: {}", e))
}
