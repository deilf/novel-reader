//! 数据目录工具（供各命令模块复用）

use std::path::PathBuf;

/// 获取应用数据目录（novel_reader）
pub fn get_data_dir() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("novel_reader");
    path
}

/// 确保数据目录存在
pub fn ensure_data_dir() -> Result<(), String> {
    let data_dir = get_data_dir();
    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).map_err(|e| format!("创建数据目录失败: {}", e))?;
    }
    Ok(())
}
