use crate::commands::CommandError;
use crate::factory::get_db_file_path;
use tokio::fs;

#[tauri::command]
pub async fn backup(app: tauri::AppHandle) -> Result<(), CommandError> {
    #[cfg(desktop)]
    {
        return backup_desktop(&app).await;
    }
    #[cfg(target_os = "android")]
    {
        return backup_android(&app).await;
    }
}

#[cfg(desktop)]
async fn backup_desktop(app: &tauri::AppHandle) -> Result<(), CommandError> {
    use crate::commands::UniversalError;
    use alloc::string::ToString;
    use tauri_plugin_dialog::DialogExt;

    let output_file_path = app
        .dialog()
        .file()
        .add_filter("SQLite file", &["db"])
        .blocking_save_file();
    let output_file_path = match output_file_path {
        None => return Ok(()),
        Some(f) => f,
    };
    let output_file_path = match output_file_path.as_path() {
        None => Err(UniversalError {
            code: 3,
            message: "Output file path is not supported".to_string(),
        })?,
        Some(p) => p,
    };
    let file_path = get_db_file_path(&app)?;
    dbg!(&file_path, &output_file_path);

    fs::copy(&file_path, &output_file_path).await?;

    Ok(())
}

#[cfg(target_os = "android")]
async fn backup_android(app: &tauri::AppHandle) -> Result<(), CommandError> {
    use tauri_plugin_android_fs::AndroidFsExt;
    use tauri_plugin_android_fs::FileUri;
    use tauri_plugin_android_fs::PublicGeneralPurposeDir::Download;

    let api = app.android_fs_async();

    let initial_location = api
        .public_storage()
        .resolve_initial_location(None, Download, "", true)
        .await?;
    let selected_file = api
        .file_picker()
        .save_file(
            Some(&initial_location),
            "PwdGenT.db",
            Some("application/vnd.sqlite3"), // MIME type
            false,
        )
        .await?;
    let uri = match selected_file {
        None => return Ok(()),
        Some(x) => x,
    };
    let output_file = api.open_file_writable(&uri).await?;

    let input_file_path = get_db_file_path(&app)?;
    let input_file_uri = FileUri::from_path(&input_file_path);
    let input_file = api.open_file_readable(&input_file_uri).await?;

    let mut output_file = fs::File::from_std(output_file);
    let mut input_file = fs::File::from_std(input_file);
    tokio::io::copy(&mut input_file, &mut output_file).await?;

    Ok(())
}
