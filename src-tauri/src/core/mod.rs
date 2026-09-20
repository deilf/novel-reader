//! 核心业务逻辑模块
//!
//! 包含网页爬取、广告清理、内容格式化等核心功能

pub mod scraper;
pub mod cleaner;
pub mod formatter;

pub use scraper::*;
pub use cleaner::*;
pub use formatter::*;
