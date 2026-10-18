
void __cdecl tiny_glade::setup::impl$0::from_winit(void)

{
  undefined8 *puVar1;
  code *pcVar2;
  undefined4 extraout_EAX;
  undefined4 extraout_EAX_00;
  uint extraout_EAX_01;
  void *extraout_RAX;
  void *_Dst;
  longlong extraout_RAX_00;
  size_t *in_RCX;
  ulonglong local_100;
  void *local_f8;
  size_t local_f0;
  uint local_e8;
  undefined4 uStack_e4;
  uint uStack_e0;
  undefined4 uStack_dc;
  
  winit::platform_impl::platform::monitor::get_monitor_info();
  if (local_e8 == 1) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  winit::platform_impl::platform::util::decode_wide();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::sys::os_str::wtf8::Slice::to_string_lossy();
  if ((longlong)local_f0 < 0) {
LAB_140175725:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (local_f0 == 0) {
    _Dst = (void *)0x1;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    _Dst = extraout_RAX;
    if (extraout_RAX == (void *)0x0) goto LAB_140175725;
  }
  memcpy(_Dst,local_f8,local_f0);
  if ((local_100 & 0x7fffffffffffffff) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (CONCAT44(uStack_e4,local_e8) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  winit::platform_impl::platform::monitor::get_monitor_info();
  if (((local_e8 & 1) != 0) && ((uStack_e0 & 3) == 1)) {
    puVar1 = *(undefined8 **)(CONCAT44(uStack_dc,uStack_e0) + 7);
    pcVar2 = (code *)*puVar1;
    if (pcVar2 != (code *)0x0) {
      (*pcVar2)(*(undefined8 *)(CONCAT44(uStack_dc,uStack_e0) + -1));
    }
    if (puVar1[1] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  winit::dpi::impl$5::from_f64();
  winit::dpi::impl$5::from_f64();
  winit::platform_impl::platform::monitor::get_monitor_info();
  if (local_e8 == 1) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  winit::dpi::impl$2::from_f64();
  winit::dpi::impl$2::from_f64();
  *in_RCX = local_f0;
  in_RCX[1] = (size_t)_Dst;
  in_RCX[2] = local_f0;
  in_RCX[3] = CONCAT44(extraout_EAX_00,extraout_EAX);
  in_RCX[4] = (ulonglong)extraout_EAX_01 | extraout_RAX_00 << 0x20;
  return;
}

