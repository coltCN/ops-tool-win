#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use mysql::Dumpfile;
mod mysql;
#[tauri::command]
fn open_file(path: &str) -> String {
  format!("准备打开文件:{}", path)
}
#[tauri::command(async)]
fn list_db(path: &str) -> Result<Vec<String>, String> {
  let mut dump = Dumpfile::new(path).unwrap();
  dump.list_db().map_err(|e| e.to_string())
}
#[tauri::command(async)]
fn list_extract_dbs(path: &str) -> Result<Vec<String>, String> {
  let mut dump = Dumpfile::new(path).unwrap();
  dump.list_extract_dbs().map_err(|e| e.to_string())
}
#[tauri::command]
fn extract_dumpfile(path: &str, save_dir: &str, db_list: Vec<String>) -> Result<(), String> {
  let mut dump = Dumpfile::new(path).unwrap();
  dump.extract(save_dir, db_list).map_err(|e| e.to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      open_file,
      list_db,
      extract_dumpfile,
      list_extract_dbs,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
