use tauri::State;
use std::sync::{Arc, Mutex};
mod models;
mod database;
use database::Database;
use models::UserDB::UsersDB;

struct AuthState{
    logged_in: Mutex<bool>,
}

#[tauri::command]
fn is_logged_in(state: State<AuthState>) -> bool {
    *state.logged_in.lock().unwrap()
}

#[tauri::command]
fn register(cpf: String, email: String, password: String, state: State<AuthState>) -> bool{
    let user_db = UsersDB::new();
    user_db.funcionando();
    true
}

#[tauri::command]
fn login(email: String, password: String, state: State<AuthState>) -> bool{
    // mudar pra request de banco de dados dps
    if email == "admin@admin.com" && password == "12345"{
        *state.logged_in.lock().unwrap() = true;
        return true;
    }
    false
}

#[tauri::command]
fn logout(state: State<AuthState>) -> bool{
    *state.logged_in.lock().unwrap() = false;
    return true
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AuthState {
            logged_in: Mutex::new(false)
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![is_logged_in, login, register, logout])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
