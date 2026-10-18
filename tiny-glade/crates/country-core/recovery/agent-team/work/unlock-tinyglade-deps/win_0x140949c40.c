
void __cdecl country_core::resources::window::AvailableMonitors::clear(void)

{
  longlong in_RCX;
  longlong lVar1;
  longlong lVar2;
  
  lVar2 = *(longlong *)(in_RCX + 0x10);
  *(undefined8 *)(in_RCX + 0x10) = 0;
  if (lVar2 != 0) {
    lVar1 = *(longlong *)(in_RCX + 8) + 0x20;
    do {
      if ((!SBORROW8(0,*(longlong *)(lVar1 + -8))) && (*(longlong *)(lVar1 + -8) != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (*(longlong *)(lVar1 + -0x20) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar1 = lVar1 + 0x40;
      lVar2 = lVar2 + -1;
    } while (lVar2 != 0);
  }
  return;
}

