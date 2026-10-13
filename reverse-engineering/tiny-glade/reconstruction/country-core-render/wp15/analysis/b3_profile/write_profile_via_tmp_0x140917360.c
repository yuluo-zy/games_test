// addr=0x140917360 name=write_profile_via_tmp
// signature=void __cdecl write_profile_via_tmp(void)


void __cdecl country_core::game_profile::write_profile_via_tmp(void)

{
  longlong extraout_RAX;
  undefined1 local_b0;
  undefined7 uStack_af;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined1 uStack_98;
  undefined1 local_90;
  
  paths::config();
  tempfile::util::
  create_helper<tempfile::file::NamedTempFile<std::fs::File>,tempfile::impl_1::tempfile_in::closure_env_0<ref_<std::path::PathBuf>_>_>
            ();
  if (uStack_98 == '\x02') {
    anyhow::error::impl$1::from<std::io::error::Error>();
  }
  else {
    ron::ser::impl$1::default();
    ron::options::Options::to_string_pretty<country_core::game_profile::GameProfile>();
    if (local_b0 != '2') {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    std::io::Write::write_all<std::fs::File>();
    if (extraout_RAX == 0) {
      if (uStack_a8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      paths::config();
      std::path::Path::_join();
      tempfile::file::NamedTempFile::persist<std::fs::File,std::path::PathBuf>();
      if (local_90 == '\x02') {
        CloseHandle((HANDLE)CONCAT71(uStack_af,0x32));
      }
      else {
        anyhow::error::impl$1::from<tempfile::file::PersistError<std::fs::File>_>();
      }
    }
    else {
      std::io::error::Error::kind();
      std::path::Path::to_path_buf();
      std::io::error::Error::new<tempfile::error::PathError>();
      anyhow::error::impl$1::from<std::io::error::Error>();
      if (uStack_a8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      tempfile::file::impl$6::drop();
      if (uStack_a0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      CloseHandle((HANDLE)CONCAT71(uStack_af,local_b0));
    }
  }
  return;
}


