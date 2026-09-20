// 预导入库
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // 初始化日志系统
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    log::info!("小说阅读器应用启动中...");

    // 运行Tauri应用
    novel_reader_lib::run();
}
