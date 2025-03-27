use super::model_db::Model;

pub struct UsersDB;

impl UsersDB {
    pub fn new() -> Model {
        Model {
            tabela: "users",
            campos: vec!["cpf_user", "email_user", "senha_user"],
        }
    }
}

