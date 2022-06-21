use crate::overview::utils::custom_errors::InvalidSqlError;
use postgres::{types::ToSql, Client, Error, NoTls};
use serde::Deserialize;

pub struct PostgresOptions {
    host: String,
    user: String,
    pwd: Option<String>,
    port: u16,
    db_name: String,
}

impl PostgresOptions {
    pub fn new(host: String, user: String, pwd: Option<String>, port: u16, db_name: String) -> Self {
        Self {
            host,
            user,
            pwd,
            port,
            db_name,
        }
    }
    fn get_connection(&self) -> Result<Client, Error> {
        let mut conn_string = format!(
            "host={} user={} port={} dbname={}",
            self.host, self.user, self.port, self.db_name
        );
        match &self.pwd {
            Some(v) => conn_string += &format!(" password={}", v),
            None => (),
        }
        Client::connect(&conn_string, NoTls)
    }

    pub fn exec_stmt(&self, stmt: String) -> Result<(), Box<dyn std::error::Error>> {
        sql_create_table_is_valid_or_bust(stmt.as_str()).expect("Sql was invalid");

        let mut connection = self.get_connection().expect(&format!(
            "Could not create database connection for statement: {}",
            &stmt
        ));
        match connection.batch_execute(&stmt) {
            Ok(_) => Ok(()),
            Err(v) => Err(Box::new(v)),
        }
    }

    pub fn exec_query(
        &self,
        stmt: String,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<String, Error> {
        let mut connection = self.get_connection().expect(&format!(
            "Could not create database connection for statement: {}",
            &stmt
        ));
        let rows = connection.query(&stmt, params)?;
        let mut res = String::new();
        for row in rows.into_iter() {
            res += &format!("{:?}", row);
        }
        Ok(res)
    }
}

fn sql_create_table_is_valid_or_bust(sql: &str) -> Result<(), InvalidSqlError> {
    if !sql.contains("create table") {
        return Err(InvalidSqlError::MissingCreateTable(sql.to_string()));
    }
    Ok(())
}
