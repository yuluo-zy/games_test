// 1403ae2f0 void __cdecl dump_profile_file(void)

void __cdecl tiny_glade::panic_reporter::panic_dump::dump_profile_file(void)

{
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *in_RCX;
  longlong local_b0;
  longlong local_88;
  longlong lStack_80;
  longlong local_78;
  longlong lStack_70;
  
  country_core::game_profile::profile_file_path();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::file_name();
  if (extraout_RAX != 0) {
    std::path::Path::_join();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::sys::pal::windows::fs::copy();
    if (local_b0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((extraout_RAX_00 & 1) == 0) {
      in_RCX[2] = local_78;
      in_RCX[3] = lStack_70;
      *in_RCX = local_88;
      in_RCX[1] = lStack_80;
    }
    else {
      std::backtrace::Backtrace::capture();
      anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
      in_RCX[1] = extraout_RAX_01;
      *in_RCX = -0x8000000000000000;
      if (local_88 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    return;
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

