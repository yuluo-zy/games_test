// write_settings_file
// sig: void __cdecl write_settings_file(void)
// size: 312

void __cdecl tiny_glade::app_config::write_settings_file(void)

{
  char extraout_AL;
  longlong local_60 [4];
  longlong *local_40;
  code *local_38;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  paths::config();
  std::path::Path::_join();
  country_core::resources::walls::decorator::ClutterVersion::is_initial();
  if (extraout_AL == '\0') {
    if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_38 = std::path::impl$44::fmt;
      local_40 = local_60;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      ::log::__private_api::impl$2::log();
    }
  }
  else {
    write_game_config_to_disk();
  }
  if (local_60[0] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

