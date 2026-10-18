// 1403acc70 void __cdecl dump_log_file(void)

void __cdecl tiny_glade::panic_reporter::panic_dump::dump_log_file(void)

{
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  longlong local_80;
  longlong lStack_78;
  longlong local_70;
  longlong lStack_68;
  longlong local_60;
  
  std::path::Path::_join();
  paths::tmp();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::pal::windows::fs::copy();
  if (local_60 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((extraout_RAX & 1) == 0) {
    in_RCX[2] = local_70;
    in_RCX[3] = lStack_68;
    *in_RCX = local_80;
    in_RCX[1] = lStack_78;
  }
  else {
    std::backtrace::Backtrace::capture();
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
    in_RCX[1] = extraout_RAX_00;
    *in_RCX = -0x8000000000000000;
    if (local_80 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

