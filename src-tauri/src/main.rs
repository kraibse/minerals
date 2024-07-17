// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct FileItem {
    name: String,
    is_dir: bool,
    icon: String,
    // Add other file attributes as needed
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_directory(path: String) -> Result<Vec<FileItem>, String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err("Path does not exist".into());
    }

    // if !path.is_dir() {
    //     return Err("Path is not a directory".into());
    // }

    match fs::read_dir(path) {
        Ok(entries) => {
            let items: Vec<FileItem> = entries
                .filter_map(|entry| {
                    entry.ok().map(|e| {
                        let file_type = e.file_type().ok()?;

                        if file_type.is_dir() || file_type.is_file() {
                            let icon: &str = if file_type.is_dir() {
                                "folder"
                            } else if file_type.is_file() {
                                "file"
                            } else {
                                "unidentified"
                            };

                            let name = e.file_name().into_string().ok()?;
                            Some(FileItem {
                                name,
                                is_dir: file_type.is_dir(),
                                icon: icon.to_string(),
                                // Add other file attributes here
                            })
                        } else {
                            None
                        }
                    })
                })
                .flatten()
                .collect();
            Ok(items)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![read_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
