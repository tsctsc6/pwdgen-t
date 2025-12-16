use crate::commands::{CommandError, UniversalError};
use crate::entities::acct_data;
use crate::factory::create_db_connection;
use sea_orm::{DerivePartialModel, EntityTrait};
use serde::Serialize;

#[tauri::command]
pub async fn read_acct_data(
    app: tauri::AppHandle,
    id: i32,
) -> Result<ReadAcctDataResult, CommandError> {
    let db = create_db_connection(&app).await?;
    let acct_data_to_read = acct_data::Entity::find_by_id(id)
        .into_partial_model()
        .one(&db)
        .await?;
    let acct_data_to_read = match acct_data_to_read {
        None => Err(UniversalError {
            code: 1,
            message: format!("The item which id = {} not found", id),
        })?,
        Some(x) => x,
    };
    Ok(acct_data_to_read)
}

#[derive(DerivePartialModel, Serialize)]
#[sea_orm(entity = "acct_data::Entity")]
pub struct ReadAcctDataResult {
    pub id: i32,
    pub user_name: String,
    pub platform: String,
    pub remark: String,
    pub skip_count: u32,
    pub use_up_letter: bool,
    pub use_low_letter: bool,
    pub use_number: bool,
    pub use_sp_char: bool,
    pub pwd_len: u32,
    pub updated_at: String,
}
