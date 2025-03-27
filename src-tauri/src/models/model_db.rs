use crate::database;
use crate::database::Database;
use dotenv::dotenv;
use mysql::{PooledConn, Value, Row, Result, Opts};
use mysql::prelude::Queryable;
use std::collections::HashMap;
use std::env;

pub struct Model {
    pub tabela: &'static str,
    pub campos: Vec<&'static str>,
}

pub enum QueryResult {
    Rows(Vec<mysql::Row>),
    Empty(bool)
}

impl Model {

    pub fn exec_query(&self, query: &str, params: Vec<&str>) -> Result<QueryResult, mysql::Error> {
        dotenv().ok();
        let db = Database::new(env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env").as_str());
        let db_lock = db.lock().unwrap();
        let mut conn = db_lock.get_connection()?;

        let result = conn.exec(query, params);

        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    Ok(QueryResult::Empty(true))
                } else {
                    Ok(QueryResult::Rows(rows))
                }
            }
            Err(e) => Err(e)
        }
    }

    pub fn insert(&self, valores: Vec<&str>) -> bool {
        dotenv().ok();
        let db = Database::new(env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env").as_str());
        let db_lock = db.lock().unwrap();

        if !db_lock.ping_database(){
            println!("Database is unreachable");
            return false;
        }

        let campos_str = self.campos.join(", ");
        let placeholders = valores.iter().map(|_| "?".to_string()).collect::<Vec<String>>().join(", ");

        let query = format!(
            "INSERT INTO {} ({}) VALUES ({});",
            self.tabela, campos_str, placeholders
        );

        let mut conn = db_lock.get_connection().unwrap();

        match conn.exec_drop(&query, valores) {
            Ok(..) => true,
            Err(_) => false,
        }
    }

    pub fn select(&self, data: &str, condicao: Option<&str>, params: Vec<&str>) -> Result<Vec<mysql::Row>, mysql::Error> {
        dotenv().ok();
        let db = Database::new(env::var("DATABASE_URL").expect("DATABASE_URL not defined in .env").as_str());
        let db_lock = db.lock().unwrap();
        let mut conn = db_lock.get_connection().unwrap();

        let condicao = condicao.unwrap_or("1=1");
        let query = format!(
            "SELECT {} FROM {} WHERE {}",
            data, self.tabela, condicao
        );
        conn.exec(query, params)
    }
}
