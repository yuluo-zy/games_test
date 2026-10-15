
void __cdecl core::ptr::drop_in_place<country_core::resources::gui::DebugGui>(void)

{
  longlong in_RCX;
  
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x10))) && (*(longlong *)(in_RCX + 0x10) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x30))) && (*(longlong *)(in_RCX + 0x30) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x50))) && (*(longlong *)(in_RCX + 0x50) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x70))) && (*(longlong *)(in_RCX + 0x70) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((!SBORROW8(0,*(longlong *)(in_RCX + 0x90))) && (*(longlong *)(in_RCX + 0x90) != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((*(longlong *)(in_RCX + 0xc0) != 0) && (*(longlong *)(in_RCX + 0xc0) * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    return;
  }
  return;
}


