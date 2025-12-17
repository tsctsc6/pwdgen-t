use crate::commands::calculate_password::calculate_password;
use crate::commands::create_acct_data::create_acct_data;
use crate::commands::delete_acct_data::delete_acct_data;
use crate::commands::init_migrate::init_migrate;
use crate::commands::read_acct_data::read_acct_data;
use crate::commands::read_all_acct_data::read_all_acct_data;
use crate::commands::update_acct_data::update_acct_data;

mod commands;
mod entities;
mod factory;
mod keystream_provider;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}));
    }

    builder
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init_migrate,
            create_acct_data,
            update_acct_data,
            delete_acct_data,
            read_acct_data,
            read_all_acct_data,
            calculate_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
