// shader_debug_info @ 0x140245340 size=500
// sig: void __cdecl shader_debug_info(void)


void __cdecl tiny_glade::aftermath::impl$1::shader_debug_info(void)

{
  char cVar1;
  undefined8 *extraout_RAX;
  void *extraout_RAX_00;
  char *in_RCX;
  void *in_RDX;
  void *_Dst;
  size_t in_R8;
  byte local_110 [8];
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e0;
  char *local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_b0;
  char *local_a8;
  undefined8 local_a0;
  undefined4 local_98;
  undefined4 local_94;
  undefined **local_90;
  undefined8 local_88;
  undefined1 **local_80;
  undefined8 local_78;
  undefined8 local_70;
  byte *local_60;
  code *local_58;
  char *local_48;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  aftermath_rs::get_shader_debug_info_identifier();
  if (4 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    local_58 = core::option::impl$46::
               fmt<aftermath_rs::sys::GFSDK_Aftermath_ShaderDebugInfoIdentifier>;
    local_60 = local_110;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_c0 = *extraout_RAX;
    uStack_b8 = extraout_RAX[1];
    local_94 = *(undefined4 *)(extraout_RAX + 2);
    local_b0 = 5;
    local_a8 = "tiny_glade::aftermathNV Aftermath dump data received.";
    local_a0 = 0x15;
    local_90 = &PTR_s_Got_NV_shader_debug_info__Ident__1429a06b0;
    local_88 = 1;
    local_80 = &local_60;
    local_78 = 1;
    local_70 = 0;
    local_e0 = 0;
    local_d8 = "tiny_glade::aftermathNV Aftermath dump data received.";
    local_d0 = 0x15;
    local_c8 = 0;
    local_98 = 1;
    ::log::__private_api::impl$2::log();
  }
  if ((local_110[0] & 1) != 0) {
    local_f8 = local_108;
    uStack_f0 = uStack_100;
    LOCK();
    cVar1 = *in_RCX;
    if (cVar1 == '\0') {
      *in_RCX = '\x01';
    }
    UNLOCK();
    if (cVar1 != '\0') {
      parking_lot::raw_mutex::RawMutex::lock_slow();
    }
    parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
    if ((longlong)in_R8 < 0) {
LAB_140245522:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_48 = in_RCX;
    if (in_R8 == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX_00;
      if (extraout_RAX_00 == (void *)0x0) goto LAB_140245522;
    }
    memcpy(_Dst,in_RDX,in_R8);
    local_d8 = _Dst;
    hashbrown::map::HashMap::
    insert<aftermath_rs::sys::GFSDK_Aftermath_ShaderDebugInfoIdentifier,alloc::vec::Vec<u8,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    if ((!SBORROW8(0,(longlong)local_60)) && (local_60 != (byte *)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    cVar1 = *local_48;
    if (cVar1 == '\x01') {
      *local_48 = '\0';
    }
    UNLOCK();
    if (cVar1 != '\x01') {
      parking_lot::raw_mutex::RawMutex::unlock_slow();
    }
  }
  return;
}


