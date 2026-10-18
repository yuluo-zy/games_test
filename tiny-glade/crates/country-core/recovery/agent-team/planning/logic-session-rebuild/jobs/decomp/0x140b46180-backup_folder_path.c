// 0x140b46180 backup_folder_path
// signature: void __cdecl backup_folder_path(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "_join", "address": "140d62810"}, {"name": "backup_saves", "address": "140b9db90"}, {"name": "fmt", "address": "140c10290"}, {"name": "unwrap_failed", "address": "1428d9760"}]


// WARNING: Removing unreachable block (ram,0x000140b46235)

void __cdecl country_core::resources::session::SessionId::backup_folder_path(void)

{
  char extraout_AL;
  
  paths::backup_saves();
  uuid::fmt::impl$1::fmt();
  if (extraout_AL != '\0') {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  std::path::Path::_join();
  return;
}

