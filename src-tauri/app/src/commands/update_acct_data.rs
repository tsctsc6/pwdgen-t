use crate::commands::{CommandError, UniversalError};
use crate::entities::acct_data;
use crate::entities::acct_data::Model;
use crate::factory::create_db_connection;
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel, Set};

#[tauri::command]
pub async fn update_acct_data(app: tauri::AppHandle, item: Model) -> Result<(), CommandError> {
    let db = create_db_connection(&app).await?;
    let acct_data_to_update = acct_data::Entity::find_by_id(item.id).one(&db).await?;
    let acct_data_to_update = match acct_data_to_update {
        None => Err(UniversalError {
            code: 1,
            message: format!("The item which id = {} not found", item.id),
        })?,
        Some(x) => x,
    };
    let mut acct_data_to_update = acct_data_to_update.into_active_model();
    acct_data_to_update.user_name = Set((&item).user_name.clone());
    acct_data_to_update.platform = Set((&item).platform.clone());
    acct_data_to_update.remark = Set((&item).remark.clone());
    acct_data_to_update.skip_count = Set((&item).skip_count.clone());
    acct_data_to_update.use_up_letter = Set((&item).use_up_letter.clone());
    acct_data_to_update.use_low_letter = Set((&item).use_low_letter.clone());
    acct_data_to_update.use_number = Set((&item).use_number.clone());
    acct_data_to_update.use_sp_char = Set((&item).use_sp_char.clone());
    acct_data_to_update.pwd_len = Set((&item).pwd_len.clone());
    acct_data_to_update.updated_at = Set((&item).updated_at.clone());
    acct_data_to_update.update(&db).await?;
    Ok(())
}
