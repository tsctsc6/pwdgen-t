use crate::commands::{CommandError, UniversalError};
use crate::entities::acct_data;
use crate::entities::acct_data::Model;
use crate::factory::create_db_connection;
use sea_orm::EntityTrait;

#[tauri::command]
pub async fn read_acct_data(app: tauri::AppHandle, id: i32) -> Result<Model, CommandError> {
    let db = create_db_connection(&app).await?;
    let acct_data_to_read = acct_data::Entity::find_by_id(id).one(&db).await?;
    let acct_data_to_read = match acct_data_to_read {
        None => Err(UniversalError {
            code: 1,
            message: format!("The item which id = {} not found", id),
        })?,
        Some(x) => x,
    };
    Ok(acct_data_to_read)
}
