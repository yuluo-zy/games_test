// 0x140b463c0 write_to_disk
// signature: void __cdecl write_to_disk(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "_join", "address": "140d62810"}, {"name": "as_mut_slice", "address": "140d76ed0"}, {"name": "save_folder_path", "address": "140b46000"}, {"name": "save_session_meta_to_file<std::path::PathBuf>", "address": "1417522e0"}]


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

