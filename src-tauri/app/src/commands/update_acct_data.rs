use crate::commands::{CommandError, UniversalError};
use crate::entities::acct_data;
use crate::factory::create_db_connection;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
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
}

#[tauri::command]
pub async fn update_acct_data(app: tauri::AppHandle, request: Request) -> Result<(), CommandError> {
    let db = create_db_connection(&app).await?;
    let acct_data_to_update = acct_data::Entity::find_by_id(request.id).one(&db).await?;
    let acct_data_to_update = match acct_data_to_update {
        None => Err(UniversalError {
            code: 1,
            message: format!("The item which id = {} not found", request.id),
        })?,
        Some(x) => x,
    };
    let mut acct_data_to_update = acct_data_to_update.into_active_model();
    acct_data_to_update.user_name = Set((&request).user_name.clone());
    acct_data_to_update.platform = Set((&request).platform.clone());
    acct_data_to_update.remark = Set((&request).remark.clone());
    acct_data_to_update.skip_count = Set((&request).skip_count.clone());
    acct_data_to_update.use_up_letter = Set((&request).use_up_letter.clone());
    acct_data_to_update.use_low_letter = Set((&request).use_low_letter.clone());
    acct_data_to_update.use_number = Set((&request).use_number.clone());
    acct_data_to_update.use_sp_char = Set((&request).use_sp_char.clone());
    acct_data_to_update.pwd_len = Set((&request).pwd_len.clone());
    acct_data_to_update.updated_at =
        Set(Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false));
    acct_data_to_update.update(&db).await?;
    Ok(())
}
