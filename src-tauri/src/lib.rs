use tauri::State;
use std::sync::Mutex;

struct AuthState{
    logged_in: Mutex<bool>,
}

#[tauri::command]
fn is_logged_in(state: State<AuthState>) -> bool {
    *state.logged_in.lock().unwrap()
}

#[tauri::command]
fn register(cpf: String, email: String, password: String, state: State<AuthState>) -> bool{

    // tentar inserir no db depois, melhorar a checagem dos dados

    if !email.is_empty() && !cpf.is_empty() && !password.is_empty(){
        *state.logged_in.lock().unwrap() = true;
        return true;
    }
    false
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
