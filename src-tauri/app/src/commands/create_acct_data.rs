use crate::commands::CommandError;
use crate::commands::UniversalError;
use crate::entities::acct_data;
use crate::factory::create_db_connection;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ActiveValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub user_name: String,
    pub platform: String,
    pub remark: String,
    pub nonce_offset: u32,
    pub use_up_letter: bool,
    pub use_low_letter: bool,
    pub use_number: bool,
    pub use_sp_char: bool,
    pub pwd_len: u32,
}

pub fn validate(request: &Request) -> Result<(), CommandError> {
    if request.user_name.is_empty() {
        Err(UniversalError {
            code: 2,
            message: "user_name is empty".to_string(),
        })?;
    }

    if request.platform.is_empty() {
        Err(UniversalError {
            code: 2,
            message: "platform is empty".to_string(),
        })?;
    }

    if request.nonce_offset >= 20 {
        Err(UniversalError {
            code: 2,
            message: "nonce_offset is too big".to_string(),
        })?;
    }

    if request.pwd_len > 255 {
        Err(UniversalError {
            code: 2,
            message: "pwd_len is too long".to_string(),
        })?;
    }

    Ok(())
}

#[tauri::command]
pub async fn create_acct_data(app: tauri::AppHandle, request: Request) -> Result<(), CommandError> {
    validate(&request)?;
    let db = create_db_connection(&app).await?;
    let acct_data_to_create = acct_data::ActiveModel {
        user_name: ActiveValue::Set(request.user_name),
        platform: ActiveValue::Set(request.platform),
        remark: ActiveValue::Set(request.remark),
        nonce_offset: ActiveValue::Set(request.nonce_offset),
        use_up_letter: ActiveValue::Set(request.use_up_letter),
        use_low_letter: ActiveValue::Set(request.use_low_letter),
        use_number: ActiveValue::Set(request.use_number),
        use_sp_char: ActiveValue::Set(request.use_sp_char),
        pwd_len: ActiveValue::Set(request.pwd_len),
        updated_at: ActiveValue::Set(
            Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, false),
        ),
        ..Default::default()
    };
    acct_data_to_create.insert(&db).await?;
    Ok(())
}
