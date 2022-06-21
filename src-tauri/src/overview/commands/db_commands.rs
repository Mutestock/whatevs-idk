use postgres::types::ToSql;
use std::collections::HashMap;

use crate::overview::connection::postgres_connection::PostgresOptions;

// params: &[&(dyn ToSql + Sync)],

#[tauri::command]
pub fn invoke_postgres_exec_query(
    stmt: String,
    connection_options: HashMap<String, String>,
) -> String {
    let pwd = match connection_options.get("pwd") {
        Some(val) => Some(val.to_owned()),
        None => None,
    };

    let mut port: u16 = 0;

    let host = match connection_options.get("host") {
        Some(v) => v.to_owned(),
        None => "Err: No host specified".to_owned(),
    };

    if host.contains("Err") {
        return host;
    }

    let user = match connection_options.get("user") {
        Some(v) => v.to_owned(),
        None => "Err: No user specified".to_owned(),
    };

    if user.contains("Err") {
        return user;
    }

    let port_intermediate = match connection_options.get("port") {
        Some(v) => v.to_owned(),
        None => return "Err: No port specified".to_owned(),
    };

    if port_intermediate.contains("Err") {
        return port_intermediate;
    }

    match port_intermediate.parse::<u16>() {
        Ok(v) => port = v,
        Err(e) => return format!("Err: port was invalid: {}", e),
    };

    let db_name = match connection_options.get("db_name") {
        Some(v) => v.to_owned(),
        None => return "Err: No database name specified".to_owned(),
    };

    if db_name.contains("Err") {
        return db_name;
    }

    let connection = PostgresOptions::new(host, user, pwd, port, db_name);

    match connection.exec_query(stmt, &[]) {
        Ok(v) => return v,
        Err(e) => return format!("Err: Postgres returned error: {}", e),
    }
}
