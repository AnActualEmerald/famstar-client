#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{fs, path::Path};

use tauri::api::process::{Command, CommandEvent};
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            println!("Setup tauri app");
            println!("Starting syncer");
            fs::create_dir_all(std::env::current_dir().unwrap().join("data"))
                .expect("Unable to create directory");
            let (mut rx, mut _child) = Command::new_sidecar("syncer")
                .expect("failed to create syncer sidecar")
                .spawn()
                .expect("failed to start syncer sidecar");
            window
                .emit("syncready", Some("syncer ready"))
                .expect("Failed to emit starting event");
            tauri::async_runtime::spawn(async move {
                while let Some(event) = rx.recv().await {
                    println!("{:#?}", event);
                    match event {
                        CommandEvent::Stdout(line) => {
                            println!("Syncer: {}", line);
                        }
                        CommandEvent::Stderr(line) => {
                            println!("Syncer-error: {}", line);
                        }
                        _ => {
                            println!("Syncer-idk: {:#?}", event);
                        }
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![log])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn log(msg: String) {
    println!("Frontend: {}", msg);
}
