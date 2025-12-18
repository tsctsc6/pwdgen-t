use crate::commands::CommandError;

#[tauri::command]
pub async fn restore() -> Result<(), CommandError> {
    Ok(())
}
