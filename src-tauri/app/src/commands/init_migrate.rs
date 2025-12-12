use crate::commands::CommandError;
use crate::factory::create_db_connection;
use migration::{Migrator, MigratorTrait};

#[tauri::command]
pub async fn init_migrate(app: tauri::AppHandle) -> Result<(), CommandError> {
    let db = create_db_connection(&app).await?;
    Migrator::up(&db, None).await?;
    Ok(())
}
