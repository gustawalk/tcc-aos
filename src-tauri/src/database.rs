use mysql::{Pool, PooledConn, Opts, Result};
use mysql::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Database {
    pool: Pool,
}

impl Database {
    
    pub fn new(database_url: &str) -> Arc<Mutex<Self>> {
        let pool = Pool::new(database_url).expect("Failed to create database pool");
        Arc::new(Mutex::new(Self { pool }))
    }

    fn get_connection(&self) -> Result<PooledConn> {
        self.pool.get_conn()
    }

    pub fn insert_user(&self, email: &str, cpf: &str, senha: &str) -> Result<()> {
        let mut conn = self.get_connection()?;

        conn.exec_drop(
            "INSERT INTO users (email_user, cpf_user, senha_user) VALUES (?, ?, ?)",
            (email, cpf, senha),
        )?;
        Ok(())
    }
}
