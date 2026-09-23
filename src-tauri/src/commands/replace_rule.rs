//! 替换净化规则命令模块（replace_rules.json）

use crate::models::ReplaceRule;
use crate::utils::data_dir::{ensure_data_dir, get_data_dir};
use log::info;
use std::fs;
use std::path::PathBuf;

fn get_rules_path() -> PathBuf {
    get_data_dir().join("replace_rules.json")
}

fn read_all() -> Result<Vec<ReplaceRule>, String> {
    let path = get_rules_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let json = fs::read_to_string(&path).map_err(|e| format!("读取替换规则失败: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("解析替换规则失败: {}", e))
}

fn write_all(rules: &[ReplaceRule]) -> Result<(), String> {
    ensure_data_dir()?;
    let json =
        serde_json::to_string_pretty(rules).map_err(|e| format!("序列化替换规则失败: {}", e))?;
    fs::write(get_rules_path(), json).map_err(|e| format!("写入替换规则失败: {}", e))
}

/// 导入替换规则（按名称去重）
#[tauri::command]
pub fn import_replace_rules(rules: Vec<ReplaceRule>) -> Result<usize, String> {
    let mut all = read_all()?;
    let mut added = 0usize;
    for rule in rules {
        if rule.name.trim().is_empty() || rule.replaceRegex.trim().is_empty() {
            continue;
        }
        match all.iter_mut().find(|r| r.name == rule.name) {
            Some(old) => *old = rule,
            None => {
                all.push(rule);
                added += 1;
            }
        }
    }
    write_all(&all)?;
    info!("替换规则导入: 新增 {} 条", added);
    Ok(added)
}

/// 获取全部替换规则
#[tauri::command]
pub fn list_replace_rules() -> Result<Vec<ReplaceRule>, String> {
    Ok(read_all()?)
}

/// 更新替换规则
#[tauri::command]
pub fn update_replace_rule(rule: ReplaceRule) -> Result<(), String> {
    let mut all = read_all()?;
    match all.iter_mut().find(|r| r.name == rule.name) {
        Some(old) => *old = rule,
        None => return Err("规则不存在".to_string()),
    }
    write_all(&all)?;
    Ok(())
}

/// 删除替换规则
#[tauri::command]
pub fn delete_replace_rule(name: String) -> Result<(), String> {
    let mut all = read_all()?;
    let before = all.len();
    all.retain(|r| r.name != name);
    if all.len() == before {
        return Err("规则不存在".to_string());
    }
    write_all(&all)?;
    Ok(())
}

/// 切换启用状态
#[tauri::command]
pub fn toggle_replace_rule(name: String) -> Result<bool, String> {
    let mut all = read_all()?;
    let target = all
        .iter_mut()
        .find(|r| r.name == name)
        .ok_or_else(|| "规则不存在".to_string())?;
    target.enabled = !target.enabled;
    let state = target.enabled;
    write_all(&all)?;
    Ok(state)
}

/// 清空替换规则
#[tauri::command]
pub fn clear_replace_rules() -> Result<(), String> {
    write_all(&[])?;
    Ok(())
}
