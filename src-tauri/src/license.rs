use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const API_BASE: &str = "https://api.keymint.dev";
const CLIENT_API_KEY: &str = option_env!("KEYMINT_CLIENT_API_KEY").unwrap_or("");
const PRODUCT_ID: &str = option_env!("KEYMINT_PRODUCT_ID").unwrap_or("");

#[derive(Deserialize)]
struct ActivateRequest {
    #[serde(rename = "productId")]
    product_id: String,
    #[serde(rename = "licenseKey")]
    license_key: String,
    #[serde(rename = "hostId")]
    host_id: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    code: i32,
    message: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct LicenseState {
    license_key: String,
    activated: bool,
}

fn license_file_path() -> PathBuf {
    let dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tauri-license-starter");
    fs::create_dir_all(&dir).ok();
    dir.join("license.json")
}

fn read_state() -> Option<LicenseState> {
    let path = license_file_path();
    if path.exists() {
        let content = fs::read_to_string(&path).ok()?;
        serde_json::from_str(&content).ok()
    } else {
        None
    }
}

fn write_state(state: &LicenseState) {
    let path = license_file_path();
    if let Ok(json) = serde_json::to_string_pretty(state) {
        fs::write(path, json).ok();
    }
}

fn clear_state() {
    fs::remove_file(license_file_path()).ok();
}

fn mask_key(key: &str) -> String {
    if key.len() <= 8 {
        return "••••".to_string();
    }
    format!(
        "{}••••{}",
        &key[..4.min(key.len())],
        &key[key.len().saturating_sub(4)..]
    )
}

#[tauri::command]
pub async fn activate_license(license_key: String) -> Result<String, String> {
    let host_id = machine_uid::get().unwrap_or_else(|_| "unknown-host".to_string());

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/key/activate", API_BASE))
        .header("Authorization", format!("Bearer {}", CLIENT_API_KEY))
        .header("Content-Type", "application/json")
        .json(&ActivateRequest {
            product_id: PRODUCT_ID.to_string(),
            license_key: license_key.clone(),
            host_id,
        })
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let result: ApiResponse =
        resp.json().await.map_err(|e| format!("Invalid response: {}", e))?;

    if result.code == 0 {
        write_state(&LicenseState {
            license_key,
            activated: true,
        });
        Ok(result.message.unwrap_or_else(|| "License valid".to_string()))
    } else {
        Err(result.message.unwrap_or_else(|| "Activation failed".to_string()))
    }
}

#[tauri::command]
pub async fn deactivate_license() -> Result<String, String> {
    let state = read_state().ok_or("No license found")?;
    let host_id = machine_uid::get().unwrap_or_else(|_| "unknown-host".to_string());

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{}/key/deactivate", API_BASE))
        .header("Authorization", format!("Bearer {}", CLIENT_API_KEY))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "productId": PRODUCT_ID,
            "licenseKey": state.license_key,
            "hostId": host_id,
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    let result: ApiResponse =
        resp.json().await.map_err(|e| format!("Invalid response: {}", e))?;

    if result.code == 0 {
        clear_state();
        Ok("License deactivated".to_string())
    } else {
        Err(result.message.unwrap_or_else(|| "Deactivation failed".to_string()))
    }
}

#[tauri::command]
pub fn is_activated() -> bool {
    read_state().map(|s| s.activated).unwrap_or(false)
}

#[tauri::command]
pub fn get_license_status() -> serde_json::Value {
    match read_state() {
        Some(state) => serde_json::json!({
            "activated": state.activated,
            "licenseKey": mask_key(&state.license_key),
        }),
        None => serde_json::json!({ "activated": false, "licenseKey": null }),
    }
}
