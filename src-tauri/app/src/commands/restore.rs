use crate::commands::{CommandError, UniversalError};
use crate::factory::get_db_file_path;
use tauri_plugin_dialog::DialogExt;
use tokio::fs;

#[tauri::command]
pub async fn restore(app: tauri::AppHandle) -> Result<(), CommandError> {
    let input_file_path = app
        .dialog()
        .file()
        .add_filter("SQLite file", &["db"])
        .blocking_pick_file();
    let input_file_path = match input_file_path {
        None => return Ok(()),
        Some(f) => f,
    };
    let input_file_path = match input_file_path.as_path() {
        None => Err(UniversalError {
            code: 3,
            message: "Input file path is not supported".to_string(),
        })?,
        Some(p) => p,
    };
    let file_path = get_db_file_path(&app)?;
    dbg!(&input_file_path, &file_path);

    fs::copy(&input_file_path, &file_path).await?;

    Ok(())
}
