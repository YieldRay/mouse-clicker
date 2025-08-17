//! 核心功能模块
//! 
//! 包含鼠标操作、热键监听和连点逻辑等核心功能

pub mod mouse;
pub mod hotkey;
pub mod clicker;

pub use clicker::{ClickerManager, ClickerStatus, ClickerState};