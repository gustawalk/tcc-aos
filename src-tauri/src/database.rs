use mysql::prelude::Queryable;
use mysql::{Pool, PooledConn, Opts, Result, Row};
use std::sync::{Arc, Mutex};

pub struct Database {
    pool: Pool,
}


impl Database {
    pub fn new(database_url: &str) -> Arc<Mutex<Self>> {
        let pool = Pool::new(database_url).expect("Failed to create database pool");
        Arc::new(Mutex::new(Self { pool }))
    }

    pub fn get_connection(&self) -> Result<PooledConn> {
        self.pool.get_conn()
    }

    pub fn ping_database(&self) -> bool {
        match self.pool.get_conn() {
            Ok(..) => true,
            Err(_) => false,
        }
    }
}
