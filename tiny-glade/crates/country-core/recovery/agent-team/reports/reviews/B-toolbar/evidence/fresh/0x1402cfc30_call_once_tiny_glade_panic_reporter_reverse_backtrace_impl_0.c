
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::panic_reporter::reverse_backtrace::impl_0::fmt::closure_env_0,tuple_<ref_mut_<core::fmt::Formatter>,enum2_<backtrace::types::BytesOrWideString>_>_>
          (void)

{
  longlong *in_RCX;
  
  tiny_glade::panic_reporter::reverse_backtrace::impl$0::fmt::closure_0();
  if (SBORROW8(0,*in_RCX)) {
    ptr::drop_in_place<std::io::error::Error>();
  }
  else if (*in_RCX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

