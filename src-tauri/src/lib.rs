mod audio;
mod engine;
mod library;
mod models;
mod music;
mod takes;
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
      library::commands::setup(app.handle())?;
      takes::setup(app.handle())?;
      music::commands::setup(app.handle())?;
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
      music::commands::compose,
      music::commands::cancel_music,
      music::commands::graphics_name,
      takes::read_take,
      library::commands::list_recordings,
      library::commands::read_recording,
      library::commands::rename_recording,
      library::commands::delete_recording,
      library::commands::export_recording,
      library::commands::relink_recording,
    ])
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}
