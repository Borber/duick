#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;

use arboard::Clipboard;
use tray_item::{IconSource, TrayItem};

#[tokio::main]
async fn main() {
    let mut tray = TrayItem::new("HMTC-E DX工具", IconSource::Resource("iconName")).unwrap();

    tray.add_menu_item("Exit", || process::exit(0)).unwrap();

    let mut clipboard = Clipboard::new().unwrap();
    let mut pre_text = clipboard.get_text().unwrap();
    loop {
        let text = clipboard.get_text().unwrap();

        if text != pre_text && text.starts_with(r"C:") {
            // 如果最后的名称有后缀名, 那么就去除最后路径
            let (all, last) = text.rsplit_once('\\').unwrap();
            if last.contains('.') {
                open(all).await;
            } else {
                open(&text).await;
            }
        }
        pre_text = text;

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

// 使用系统文件浏览器打开目录
async fn open(path: &str) {
    let _ = tokio::process::Command::new("explorer").arg(path).spawn();
}
