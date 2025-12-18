use crate::commands::CommandError;
use sea_orm::{Database, DatabaseConnection};
use tauri::{AppHandle, Manager};
use tokio::fs;

pub fn get_app_data_path(app: &AppHandle) -> Result<String, CommandError> {
    if tauri::is_dev() {
        Ok(".".to_string())
    } else {
        Ok(app.path().app_data_dir()?.to_string_lossy().to_string())
    }
}

pub fn get_db_file_path(app: &AppHandle) -> Result<String, CommandError> {
    let app_data_path = get_app_data_path(app)?;
    Ok(format!("{}/PwdGenT.db", app_data_path))
}

pub async fn create_db_connection(app: &AppHandle) -> Result<DatabaseConnection, CommandError> {
    let app_data_path = get_app_data_path(app)?;
    dbg!(&app_data_path);

    fs::create_dir_all(&app_data_path).await?;

    Ok(Database::connect(format!("sqlite://{}/PwdGenT.db?mode=rwc", app_data_path)).await?)
}
