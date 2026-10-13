// name: save_folder_path size: 253
// void __cdecl save_folder_path(void)

// WARNING: Removing unreachable block (ram,0x000140b460b5)

void __cdecl country_core::resources::session::SessionId::save_folder_path(void)

{
  char extraout_AL;
  
  paths::saves();
  uuid::fmt::impl$1::fmt();
  if (extraout_AL != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  std::path::Path::_join();
  return;
}


