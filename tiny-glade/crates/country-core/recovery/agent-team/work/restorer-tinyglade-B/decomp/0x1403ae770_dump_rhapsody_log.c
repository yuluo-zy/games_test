// dump_rhapsody_log @ 0x1403ae770
// sig: void __cdecl dump_rhapsody_log(void)


void __cdecl tiny_glade::panic_reporter::panic_dump::dump_rhapsody_log(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  longlong local_68;
  longlong local_50;
  longlong lStack_48;
  longlong local_40;
  longlong lStack_38;
  
  std::path::Path::_join();
  rhapsody::dump_log();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::write::inner();
  if (local_68 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (extraout_RAX == 0) {
    in_RCX[2] = local_40;
    in_RCX[3] = lStack_38;
    *in_RCX = local_50;
    in_RCX[1] = lStack_48;
  }
  else {
    std::backtrace::Backtrace::capture();
    anyhow::Error::construct<anyhow::error::ContextError<ref_<str_>,std::io::error::Error>_>();
    in_RCX[1] = extraout_RAX_00;
    *in_RCX = -0x8000000000000000;
    if (local_50 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

