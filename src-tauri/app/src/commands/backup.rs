use crate::commands::CommandError;
use crate::commands::UniversalError;
use crate::factory::get_db_file_path;
use tauri_plugin_dialog::DialogExt;
use tokio::fs;

#[tauri::command]
pub async fn backup(app: tauri::AppHandle) -> Result<(), CommandError> {
    let output_file_path = app
        .dialog()
        .file()
        .add_filter("SQLite file", &["db"])
        .blocking_save_file();
    let output_file_path = match output_file_path {
        None => return Ok(()),
        Some(f) => f,
    };
    let output_file_path = match output_file_path.as_path() {
        None => Err(UniversalError {
            code: 3,
            message: "Output file path is not supported".to_string(),
        })?,
        Some(p) => p,
    };
    let file_path = get_db_file_path(&app)?;
    dbg!(&file_path, &output_file_path);

    fs::copy(&file_path, &output_file_path).await?;

    Ok(())
}
