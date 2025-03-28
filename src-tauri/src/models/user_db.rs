use serde::{Serialize, Deserialize};
use super::model_db::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: u32,
    pub cpf: String,
    pub email: String
}

impl User {
    pub fn user_from_row(row: mysql::Row) -> Option<Self>{
        let (id, cpf, email) = mysql::from_row_opt::<(u32, String, String)>(row).ok()?;
        Some(Self { id, cpf, email } ) 
    }
}

pub struct UsersDB;

impl UsersDB {
    pub fn new() -> Model {
        Model {
            tabela: "users",
            campos: vec!["cpf_user", "email_user", "senha_user"],
        }
    }

    pub fn get_user_by_email(email: &str) -> Option<User> {
        let result = UsersDB::new().select("id, cpf_user, email_user", Some("email_user = ?"), vec![email]);
        
        match result {
            Ok(mut rows) => rows.pop().and_then(User::user_from_row),
            Err(_) => None
        }
    }
}

