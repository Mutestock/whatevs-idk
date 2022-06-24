#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::overview::commands::postgres_commands::{
    invoke_postgres_exec_query, invoke_postgres_select_all_query, invoke_whatever, generate_password
};

mod gen;
mod overview;

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![
            invoke_postgres_exec_query,
            invoke_postgres_select_all_query,
            invoke_whatever,
            generate_password
        ])
        .run(context)
        .expect("error while running tauri application");
}
