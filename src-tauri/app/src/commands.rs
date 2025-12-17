use serde::Serialize;
use thiserror::Error;

pub mod create_acct_data;
pub mod delete_acct_data;
pub mod init_migrate;
pub mod read_acct_data;
pub mod read_all_acct_data;
pub mod update_acct_data;
pub(crate) mod calculate_password;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Universal error: {0}")]
    UniversalError(#[from] UniversalError),
    #[error("Tauri error: {0}")]
    TauriError(#[from] tauri::Error),
    #[error("Std IO error: {0}")]
    StdIOError(#[from] std::io::Error),
    #[error("Database error: {0}")]
    DbError(#[from] migration::DbErr),
}

#[derive(Debug, Error, Serialize)]
#[error("[{code}] {message}")]
pub struct UniversalError {
    pub code: i32,
    pub message: String,
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
