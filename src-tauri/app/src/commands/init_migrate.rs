use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use tauri::Manager;
use tokio::fs;

#[tauri::command]
pub async fn init_migrate(app: tauri::AppHandle) {
    let app_data_path = app
        .path()
        .app_data_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();
    // println!("{}", app_data_path);

    fs::create_dir_all(&app_data_path).await.unwrap();

    let db: DatabaseConnection =
        Database::connect(format!("sqlite://{}/PwdGenT.db?mode=rwc", app_data_path))
            .await
            .unwrap();
    Migrator::up(&db, None).await.unwrap();
}
