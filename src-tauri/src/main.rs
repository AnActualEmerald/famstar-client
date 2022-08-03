#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate dotenv_codegen;

use std::fs;

use dotenv;
use std::env;
use tauri::api::process::{Command, CommandEvent};
use tauri::{Manager, Window};

mod model;

fn main() {
    env::set_var("SYNC_TARGET", dotenv!("SYNC_TARGET"));
    env::set_var("FAM_SHARE", dotenv!("FAM_SHARE"));

    tauri::Builder::default()
        .setup(|app| {
            let _id = app.listen_global("sync-command", |event| {
                println!("Got sync-command event: payload = {:#?}", event.payload());
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![log, start])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn log(msg: String) {
    println!("Frontend: {}", msg);
}

#[tauri::command]
fn start(window: Window) {
    println!("Setup tauri app");
    println!("Starting syncer");
    fs::create_dir_all(std::env::current_dir().unwrap().join("data"))
        .expect("Unable to create directory");
    let (mut rx, mut _child) = Command::new_sidecar("syncer")
        .expect("failed to create syncer sidecar")
        .spawn()
        .expect("failed to start syncer sidecar");
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    match serde_json::from_str::<model::SyncMessage>(&line) {
                        Ok(o) => handle_sync_msg(&window, o),
                        Err(_) => println!("Syncer: {}", line),
                    }
                }
                CommandEvent::Stderr(line) => {
                    println!("Syncer-error: {}", line);
                }
                _ => {
                    println!("Syncer-idk: {:#?}", event);
                }
            }
        }
        println!("Syncer exited");
    });
}

fn handle_sync_msg(window: &Window, msg: model::SyncMessage) {
    if let Err(e) = window.emit("sync-event", msg) {
        eprintln!("Error sending event to window: {}", e);
    }
}
