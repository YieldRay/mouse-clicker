//! 错误处理模块
//!
//! 定义应用程序中使用的错误类型

/// 应用程序结果类型的别名
pub type Result<T> = std::result::Result<T, String>;
