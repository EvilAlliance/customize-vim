#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fc;

#[tauri::command]
fn open_folder(folder_path: &str) -> String{
    let file_tree = fc::read_directory(folder_path);
    let file_tree_str = match serde_json::to_string(&file_tree) {
        Ok(str) => str,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    file_tree_str
}

fn main() {
    use tauri::Manager;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_folder])
        .setup(|app| {
        {
        let window = app.get_window("main").unwrap();
        window.open_devtools();
        window.close_devtools();
        }
        Ok(())
        }).run(tauri::generate_context!())
        .expect("error while running tauri application");
}
