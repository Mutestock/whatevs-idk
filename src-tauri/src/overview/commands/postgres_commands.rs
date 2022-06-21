use crate::overview::connection::postgres_connection::PostgresOptions;

#[tauri::command]
pub fn invoke_postgres_exec_query(stmt: String, pg_options: PostgresOptions) -> String {
    match pg_options.exec_query(stmt, &[]) {
        Ok(v) => return v,
        Err(e) => return format!("Err: Postgres returned error: {}", e),
    }
}

#[tauri::command]
pub fn invoke_postgres_select_all_query(table_name: String, pg_options: PostgresOptions) -> String {
    match pg_options.select_all_by_table_name(table_name) {
        Ok(v) => return v,
        Err(e) => return format!("Err: Postgres returned error: {}", e),
    }
}
