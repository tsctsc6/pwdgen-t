use crate::commands::CommandError;
use sea_orm::{Database, DatabaseConnection};
use tauri::Manager;
use tokio::fs;

pub async fn create_db_connection(
    app: &tauri::AppHandle,
) -> Result<DatabaseConnection, CommandError> {
    let app_data_path: String;
    if tauri::is_dev() {
        app_data_path = ".".to_string();
    } else {
        app_data_path = app.path().app_data_dir()?.to_string_lossy().to_string();
    }
    dbg!(&app_data_path);

    fs::create_dir_all(&app_data_path).await?;

    Ok(Database::connect(format!("sqlite://{}/PwdGenT.db?mode=rwc", app_data_path)).await?)
}
