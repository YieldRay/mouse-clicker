//! 核心功能模块
//!
//! 包含鼠标操作、热键监听和连点逻辑等核心功能

pub mod clicker;
pub mod hotkey;
pub mod mouse;

pub use clicker::{ClickerManager, ClickerState, ClickerStatus};
