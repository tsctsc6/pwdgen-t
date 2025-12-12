use crate::commands::CommandError;
use crate::entities::acct_data;
use crate::entities::acct_data::Model;
use crate::factory::create_db_connection;
use sea_orm::{ActiveModelTrait, ActiveValue};

#[tauri::command]
pub async fn create_acct_data(app: tauri::AppHandle, item: Model) -> Result<(), CommandError> {
    let db = create_db_connection(&app).await?;
    let new_acct_data = acct_data::ActiveModel {
        user_name: ActiveValue::Set(item.user_name),
        platform: ActiveValue::Set(item.platform),
        remark: ActiveValue::Set(item.remark),
        skip_count: ActiveValue::Set(item.skip_count),
        use_up_letter: ActiveValue::Set(item.use_up_letter),
        use_low_letter: ActiveValue::Set(item.use_low_letter),
        use_number: ActiveValue::Set(item.use_number),
        use_sp_char: ActiveValue::Set(item.use_sp_char),
        pwd_len: ActiveValue::Set(item.pwd_len),
        updated_at: ActiveValue::Set(item.updated_at),
        ..Default::default()
    };
    new_acct_data.insert(&db).await?;
    Ok(())
}
