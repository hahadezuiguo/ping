#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
use dialoguer::Confirm;
mod ping;
mod tool;
// mod say;
use ferris_says::say;
use std::io::{stdout, BufWriter};
#[tauri::command]
fn greet_action() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    println!("大家晚上好 !{}", width);
    say(&message, width, &mut writer).unwrap();
    // format!("你好, {}! 我已经录入了!", name)
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn hello(name: &str) -> String {
    format!("你好, {}! 我已经录入了!", name)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn log(content: &str) {
    println!("log content : {}", content);
    // println!("大家晚上好 !{}", width);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            hello,
            greet,
            greet_action,
            tool::service,
            ping::ping_ip,
            ping::ping_ip_list,
            log,
            tool::select_local_file,
            tool::select_local_ip_list,
            tool::install_harmony_package,
            ping::test_terminal,
            tool::is_ip_reachable,
            tool::ip_list_reachable,
            // show_dialog
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
fn show_dialog(content: &str ) -> bool{
    let answer = Confirm::new()
        .with_prompt(content)
        .default(false)
        .show_default(true)
        .interact()
        .unwrap();
    answer
}