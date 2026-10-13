// name: write_to_disk size: 124
// void __cdecl write_to_disk(void)

void __cdecl country_core::resources::session::SessionMeta::write_to_disk(void)

{
  undefined8 local_40;
  
  SessionId::save_folder_path();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  systems::save::save_game::save_session_meta_to_file<std::path::PathBuf>();
  if (local_40 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


