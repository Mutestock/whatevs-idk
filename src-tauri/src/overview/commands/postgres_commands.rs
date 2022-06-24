use rand::Rng;

use crate::overview::connection::postgres_connection::PostgresOptions;
use std::fs::File;
use std::io::{Write};

static CHARS: [char; 70] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9','a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's','t', 'u', 'v', 'w', 'x', 'y', 'z','A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S','T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!','@','#','$','%','^','&','*'];


#[tauri::command]
pub fn invoke_postgres_exec_query(stmt: String, pg_options: PostgresOptions) -> String {
    match pg_options.exec_query(stmt, &[]) {
        Ok(v) => return v,
        Err(e) => return format!("Err: Postgres returned error: {}", e),
    }
}

#[tauri::command]
pub fn invoke_postgres_select_all_query(table_name: u32) -> String {
    //pg_options
    //    .print_info()
    //    .expect("Somehow could not print pg_options info for select all query");
    "aaaaaaah".to_owned();
    //match pg_options.select_all_by_table_name(table_name) {
    //    Ok(v) => return v,
    //    Err(e) => return format!("Err: Postgres returned error: {}", e),
    //}
}

#[tauri::command]
pub fn invoke_whatever(table_name: &str) -> String{
    let path = "/home/apathy/Documents/junk/whatever_man.txt";
    let mut output = File::create(path).expect("shit aint working, man");
    write!(output, "Wrote to file:").expect("Could not write to file");
    let to_insert = format!("{}", table_name);
    write!(output, "{}", to_insert).expect("Could not write to file2");
    "derp".to_owned()
}


#[tauri::command]
pub fn generate_password(length: u32) -> String{
    let mut rng = rand::thread_rng();
    let mut result = String::new();
    for _x in 0..length {
        result.push(CHARS[rng.gen_range(0..70)]);
    }

    result
}
