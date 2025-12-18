use crate::commands::CommandError;

#[tauri::command]
pub async fn backup() -> Result<(), CommandError> {
    Ok(())
}
