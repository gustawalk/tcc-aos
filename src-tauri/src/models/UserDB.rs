use super::ModelDB::Model;

pub struct UsersDB;

impl UsersDB {
    pub fn new() -> Model {
        Model {
            tabela: "users",
            campos: vec!["cpf_user", "nome_user", "email_user"],
        }
    }
}

