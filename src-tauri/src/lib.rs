mod audio;
mod clones;
mod engine;
mod library;
mod mcp;
mod mix;
mod models;
mod music;
mod preferences;
mod takes;
mod voice;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let assistant = mcp::is_requested();
  let mut context = tauri::generate_context!();
  if assistant {
    context.config_mut().app.windows.clear();
  }
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_opener::init())
    .setup(move |app| {
      // The log uses stdout, which belongs to the assistant.
      if cfg!(debug_assertions) && !assistant {
        app.handle().plugin(tauri_plugin_log::Builder::default().level(log::LevelFilter::Info).build())?;
      }
      models::commands::setup(app.handle())?;
      library::commands::setup(app.handle())?;
      takes::setup(app.handle())?;
      clones::commands::setup(app.handle())?;
      music::commands::setup(app.handle())?;
      voice::commands::setup(app.handle())?;
      if assistant {
        #[cfg(target_os = "macos")]
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
        mcp::serve(app.handle().clone());
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      preferences::read_preferences,
      preferences::save_preferences,
      models::commands::list_models,
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
      mix::commands::create_mix,
      clones::commands::list_clones,
      clones::commands::check_sample,
      clones::commands::read_sample,
      clones::commands::discard_sample,
      clones::commands::create_clone,
      clones::commands::read_clone,
      clones::commands::rename_clone,
      clones::commands::delete_clone,
      mcp::commands::assistant_command,
      takes::read_take,
      library::commands::list_recordings,
      library::commands::read_recording,
      library::commands::rename_recording,
      library::commands::delete_recording,
      library::commands::export_recording,
      library::commands::relink_recording,
      library::commands::save_folder,
      library::commands::change_save_folder,
      library::commands::restore_save_folder,
    ])
    .run(context)
    .expect("error while building tauri application");
}
