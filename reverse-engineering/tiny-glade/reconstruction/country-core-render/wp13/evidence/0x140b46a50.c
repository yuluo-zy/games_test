// name: loading_background_path size: 108
// void __cdecl loading_background_path(void)

void __cdecl country_core::resources::session::SavedSession::loading_background_path(void)

{
  undefined8 local_40;
  
  SessionId::save_folder_path();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  if (local_40 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


