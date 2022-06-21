use std::collections::HashMap;

use crate::overview::connection::postgres_connection::PostgresOptions;

// params: &[&(dyn ToSql + Sync)],

#[tauri::command]
pub fn invoke_postgres_exec_query(
    stmt: String,
    pg_options: PostgresOptions,
) -> String {

    match pg_options.exec_query(stmt, &[]) {
        Ok(v) => return v,
        Err(e) => return format!("Err: Postgres returned error: {}", e),
    }
}
