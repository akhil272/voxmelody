mod models;
mod voice;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
      }
      models::commands::setup(app.handle())?;
      voice::commands::setup(app.handle())
    })
    .invoke_handler(tauri::generate_handler![
      models::commands::list_packs,
      models::commands::download_pack,
      models::commands::pause_pack,
      models::commands::remove_pack,
      models::commands::models_location,
      models::commands::change_models_location,
      models::commands::restore_models_location,
      models::commands::system_info,
      voice::commands::list_voices,
      voice::commands::speak,
      voice::commands::cancel_speech,
      voice::commands::read_take,
      voice::commands::export_take,
    ])
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}
