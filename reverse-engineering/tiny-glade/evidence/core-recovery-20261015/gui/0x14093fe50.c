
void __cdecl country_core::resources::gui::DebugGui::log_to_screen(void)

{
  longlong lVar1;
  void *extraout_RAX;
  longlong in_RCX;
  longlong lVar2;
  void *in_RDX;
  size_t in_R8;
  undefined4 in_R9D;
  void *_Dst;
  
  if ((longlong)in_R8 < 0) {
LAB_14093ff4e:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (in_R8 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_14093ff4e;
  }
  memcpy(_Dst,in_RDX,in_R8);
  if (4 < *(ulonglong *)(in_RCX + 0xb0)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar2 = *(ulonglong *)(in_RCX + 0xb0) * 0x20;
  lVar1 = in_RCX + lVar2;
  lVar2 = *(longlong *)(in_RCX + 0x10 + lVar2);
  if ((!SBORROW8(0,lVar2)) && (lVar2 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(size_t *)(lVar1 + 0x10) = in_R8;
  *(void **)(lVar1 + 0x18) = _Dst;
  *(size_t *)(lVar1 + 0x20) = in_R8;
  *(undefined4 *)(lVar1 + 0x28) = in_R9D;
  *(undefined4 *)(lVar1 + 0x2c) = 0x40a00000;
  *(ulonglong *)(in_RCX + 0xb0) =
       ((*(longlong *)(in_RCX + 0xb0) + 1U) / 5) * -5 + *(longlong *)(in_RCX + 0xb0) + 1;
  return;
}


