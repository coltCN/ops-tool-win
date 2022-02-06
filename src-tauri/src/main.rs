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
#[tauri::command]
fn list_db(path: &str) -> Vec<String> {
  let dump = Dumpfile::new(path);
  dump.list_db().unwrap()
}
#[tauri::command]
fn extract_dumpfile(path: &str, save_dir: &str, db_list: Vec<String>) {
  let dump = Dumpfile::new(path);
  dump.extract(save_dir, db_list);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      open_file,
      list_db,
      extract_dumpfile
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
