#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::State;
use base64::{Engine as _, engine::general_purpose};

// Device state
struct AppState {
    device: Mutex<Option<flowx_core::platforms::android::AndroidClient>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ScriptInfo {
    name: String,
    path: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceInfo {
    id: String,
    name: String,
    platform: String,
    status: String,
    resolution: (u32, u32),
}

#[tauri::command]
fn connect_device(uri: String, state: State<AppState>) -> Result<String, String> {
    eprintln!("========================================");
    eprintln!("connect_device command called!");
    eprintln!("URI: {}", uri);
    eprintln!("========================================");

    let client = flowx_core::platforms::android::AndroidClient::new(uri.clone());

    eprintln!("AndroidClient created, testing connection...");

    // Test connection
    let size = client.get_screen_size().map_err(|e| {
        eprintln!("Connection FAILED: {:?}", e);
        format!("Connection failed: {:?}", e)
    })?;

    eprintln!("Connection SUCCESS! Screen size: {}x{}", size.0, size.1);

    *state.device.lock().unwrap() = Some(client);

    Ok(format!("Connected: {}x{}", size.0, size.1))
}

#[tauri::command]
fn get_devices(state: State<AppState>) -> Result<Vec<DeviceInfo>, String> {
    let device = state.device.lock().unwrap();

    if let Some(ref client) = *device {
        if let Ok(size) = client.get_screen_size() {
            return Ok(vec![DeviceInfo {
                id: "android-1".to_string(),
                name: "Android Device".to_string(),
                platform: "android".to_string(),
                status: "connected".to_string(),
                resolution: size,
            }]);
        }
    }

    Ok(vec![])
}

#[tauri::command]
fn take_screenshot(state: State<AppState>) -> Result<String, String> {
    let device = state.device.lock().unwrap();

    if let Some(ref client) = *device {
        let data = client.screenshot().map_err(|e| format!("Screenshot failed: {:?}", e))?;
        Ok(general_purpose::STANDARD.encode(&data))
    } else {
        Err("No device connected".to_string())
    }
}

#[tauri::command]
fn get_scripts() -> Result<Vec<ScriptInfo>, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scripts_dir = format!("{}/.flowx/scripts", home);

    std::fs::create_dir_all(&scripts_dir).ok();

    let mut scripts = vec![];

    if let Ok(entries) = std::fs::read_dir(&scripts_dir) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if name.ends_with(".py") {
                    let path = entry.path();
                    let content = std::fs::read_to_string(&path).unwrap_or_default();

                    scripts.push(ScriptInfo {
                        name: name.to_string(),
                        path: path.display().to_string(),
                        content,
                    });
                }
            }
        }
    }

    Ok(scripts)
}

#[tauri::command]
fn create_script(name: String) -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scripts_dir = format!("{}/.flowx/scripts", home);
    let path = format!("{}/{}", scripts_dir, name);

    std::fs::create_dir_all(&scripts_dir).ok();

    let template = "# FlowX Script\n\n# device.click(x, y)\n# device.swipe(x1, y1, x2, y2, duration_ms)\n# device.input_text(\"text\")\n";

    std::fs::write(&path, template).map_err(|e| format!("Failed to create script: {}", e))?;

    Ok(path)
}

#[tauri::command]
fn load_script(name: String) -> Result<ScriptInfo, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scripts_dir = format!("{}/.flowx/scripts", home);
    let path = format!("{}/{}", scripts_dir, name);

    let content = std::fs::read_to_string(&path).map_err(|e| format!("Failed to load: {}", e))?;

    Ok(ScriptInfo {
        name,
        path,
        content,
    })
}

#[tauri::command]
fn save_script(name: String, content: String) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scripts_dir = format!("{}/.flowx/scripts", home);
    let path = format!("{}/{}", scripts_dir, name);

    std::fs::write(&path, content).map_err(|e| format!("Failed to save: {}", e))?;

    Ok(())
}

#[tauri::command]
fn delete_script(name: String) -> Result<(), String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scripts_dir = format!("{}/.flowx/scripts", home);
    let path = format!("{}/{}", scripts_dir, name);

    std::fs::remove_file(&path).map_err(|e| format!("Failed to delete: {}", e))?;

    Ok(())
}

#[tauri::command]
fn run_script(name: String, state: State<AppState>) -> Result<String, String> {
    eprintln!("========================================");
    eprintln!("run_script called: {}", name);
    eprintln!("========================================");

    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let scripts_dir = format!("{}/.flowx/scripts", home);
    let path = format!("{}/{}", scripts_dir, name);

    eprintln!("Script path: {}", path);

    // 检查设备是否连接
    let device = state.device.lock().unwrap();
    if device.is_none() {
        return Err("No device connected. Please connect a device first.".to_string());
    }
    drop(device);

    // 运行 Python 脚本
    let output = std::process::Command::new("python3")
        .arg(&path)
        .env("FLOWX_DEVICE_URI", "android://localhost:6789")
        .output()
        .map_err(|e| format!("Failed to run script: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    eprintln!("Script stdout: {}", stdout);
    eprintln!("Script stderr: {}", stderr);

    if output.status.success() {
        Ok(format!("[Output]\n{}\n[Completed successfully]", stdout))
    } else {
        Ok(format!("[Output]\n{}\n[Error]\n{}", stdout, stderr))
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(AppState {
            device: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            connect_device,
            get_devices,
            take_screenshot,
            get_scripts,
            create_script,
            load_script,
            save_script,
            delete_script,
            run_script,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
