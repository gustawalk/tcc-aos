use tauri::State;
use std::sync::{Arc, Mutex};
mod models;
mod database;
use database::Database;
use models::user_db::UsersDB;
use crate::models::model_db::QueryResult;

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
    let query = user_db.insert(vec![&cpf, &email, &password]);

    if query {
        *state.logged_in.lock().unwrap() = true;
        return true;
    }

   false
}

#[tauri::command]
fn custom_query() -> String {
    let user_db = UsersDB::new();
    let query = "SELECT email_user FROM users WHERE id > ?";
    let params = vec!["1"];

    match user_db.exec_query(query, params) {
        Ok(QueryResult::Rows(rows)) => {
            let emails: Vec<String> = rows.iter().map(|row| {
                let email: String = mysql::from_row(row.clone());
                email
            }).collect();
            format!("Emails: {:?}", emails)
        }
        Ok(QueryResult::Empty(true)) => "Nenhum resultado encontrado".to_string(),
        Ok(QueryResult::Empty(false)) => "Query vazia".to_string(),
        Err(e) => format!("Erro ao executar a query: {}", e),
    }
}

#[tauri::command]
fn login(email: String, password: String, state: State<AuthState>) -> bool {
    let user_db = UsersDB::new();
    match user_db.select("*", Some("email_user = ? AND senha_user = ?"), vec![&email, &password]) {
        Ok(results) => {
            for (index, row) in results.iter().enumerate() {
                println!("Row {}: {:?}", index, row);
            }

            if !results.is_empty() {
                *state.logged_in.lock().unwrap() = true;
                true
            } else {
                false
            }
        }
        Err(e) => {
            eprintln!("Login error: {}", e);
            false
        }
    }
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
        .invoke_handler(tauri::generate_handler![is_logged_in, login, register, logout, custom_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
