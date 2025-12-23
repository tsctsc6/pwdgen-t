use crate::commands::CommandError;
use crate::factory::get_db_file_path;
use tokio::fs;

#[tauri::command]
pub async fn restore(app: tauri::AppHandle) -> Result<(), CommandError> {
    #[cfg(desktop)]
    {
        restore_desktop(&app).await
    }
    #[cfg(target_os = "android")]
    {
        restore_android(&app).await
    }
}

#[cfg(desktop)]
async fn restore_desktop(app: &tauri::AppHandle) -> Result<(), CommandError> {
    use crate::commands::UniversalError;
    use crate::factory::get_db_file_path;
    use tauri_plugin_dialog::DialogExt;

    let input_file_path = app
        .dialog()
        .file()
        .add_filter("SQLite file", &["db"])
        .blocking_pick_file();
    let input_file_path = match input_file_path {
        None => return Ok(()),
        Some(f) => f,
    };
    let input_file_path = match input_file_path.as_path() {
        None => Err(UniversalError {
            code: 3,
            message: "Input file path is not supported".to_string(),
        })?,
        Some(p) => p,
    };
    let file_path = get_db_file_path(&app)?;
    dbg!(&input_file_path, &file_path);

    fs::copy(&input_file_path, &file_path).await?;

    Ok(())
}

#[cfg(target_os = "android")]
async fn restore_android(app: &tauri::AppHandle) -> Result<(), CommandError> {
    use tauri_plugin_android_fs::AndroidFsExt;
    use tauri_plugin_android_fs::FileUri;
    use tauri_plugin_android_fs::PublicGeneralPurposeDir::Download;

    let api = app.android_fs_async();

    let initial_location = api
        .public_storage()
        .resolve_initial_location(None, Download, "", true)
        .await?;
    let selected_files = api
        .file_picker()
        .pick_files(
            Some(&initial_location),
            &["*/*"], // MIME type
            false,
        )
        .await?;
    let first_uri = match selected_files.first() {
        None => return Ok(()),
        Some(x) => x,
    };
    let input_file = api.open_file_readable(&first_uri).await?;

    let output_file_path = get_db_file_path(&app)?;
    let output_file_uri = FileUri::from_path(&output_file_path);
    let output_file = api.open_file_writable(&output_file_uri).await?;

    let mut output_file = fs::File::from_std(output_file);
    let mut input_file = fs::File::from_std(input_file);
    tokio::io::copy(&mut input_file, &mut output_file).await?;

    Ok(())
}
