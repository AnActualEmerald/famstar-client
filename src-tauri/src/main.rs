#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

use dotenv;
use std::env;
use tauri::api::process::{Command, CommandEvent};
use tauri::{Manager, Window};

fn main() {
    dotenv::dotenv().ok();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log, get_share, get_target])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn log(msg: String) {
    println!("Frontend: {}", msg);
}

#[tauri::command]
fn get_share() -> Result<String, String> {
    env::var("SHARE").map_err(|e| e.to_string())
}

#[tauri::command]
fn get_target() -> Result<String, String> {
    env::var("SYNC_TARGET").map_err(|e| e.to_string())
}
