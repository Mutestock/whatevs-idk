#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::overview::commands::db_commands::invoke_postgres_exec_query;

mod gen;
mod overview;

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .invoke_handler(tauri::generate_handler![invoke_postgres_exec_query])
    .run(context)
    .expect("error while running tauri application");
}
