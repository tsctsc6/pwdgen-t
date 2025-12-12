use crate::commands::{CommandError, UniversalError};
use crate::entities::acct_data;
use crate::factory::create_db_connection;
use sea_orm::EntityTrait;

#[tauri::command]
pub async fn delete_acct_data(app: tauri::AppHandle, id: i32) -> Result<(), CommandError> {
    let db = create_db_connection(&app).await?;
    let result = acct_data::Entity::delete_by_id(id).exec(&db).await?;
    if result.rows_affected == 0 {
        Err(UniversalError {
            code: 2,
            message: "0 row affected".to_string(),
        })?
    }
    Ok(())
}
