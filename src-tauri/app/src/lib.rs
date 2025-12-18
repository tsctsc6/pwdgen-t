use crate::commands::backup::backup;
use crate::commands::calculate_password::calculate_password;
use crate::commands::create_acct_data::create_acct_data;
use crate::commands::delete_acct_data::delete_acct_data;
use crate::commands::init_migrate::init_migrate;
use crate::commands::read_acct_data::read_acct_data;
use crate::commands::read_all_acct_data::read_all_acct_data;
use crate::commands::restore::restore;
use crate::commands::update_acct_data::update_acct_data;
use tauri_plugin_prevent_default::{Flags, PlatformOptions};

mod commands;
mod entities;
mod factory;
mod keystream_provider;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    #[allow(unused_mut)]
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init());

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}));
    }

    if !tauri::is_dev() {
        builder = builder.plugin(
            tauri_plugin_prevent_default::Builder::new()
                .with_flags(Flags::all())
                .platform(
                    PlatformOptions::new()
                        .dev_tools(false)
                        .general_autofill(false)
                        .password_autosave(false)
                        .default_context_menus(false)
                        .browser_accelerator_keys(false)
                        .default_script_dialogs(false),
                )
                .build(),
        );
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
            calculate_password,
            backup,
            restore,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
