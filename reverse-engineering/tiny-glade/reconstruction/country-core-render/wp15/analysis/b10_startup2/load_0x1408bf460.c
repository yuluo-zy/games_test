// addr=0x1408bf460 name=load
// signature=void __cdecl load(void)


void __cdecl country_core::startup::load_glade_audio::GladeAudio::load(void)

{
  longlong *extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  longlong local_128;
  code *local_120;
  longlong lStack_118;
  longlong local_110;
  longlong lStack_108;
  longlong lStack_100;
  longlong lStack_f8;
  undefined8 *local_f0;
  longlong lStack_e8;
  code *local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  longlong lStack_80;
  undefined8 *local_78;
  longlong lStack_70;
  code **local_60;
  code *local_58;
  longlong *local_40;
  code *local_38;
  code *local_30;
  code **local_28;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  local_40 = in_RDX;
  if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::path::Path::_join();
    local_58 = std::path::impl$44::fmt;
    local_60 = &local_a8;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    lStack_108 = *extraout_RAX;
    lStack_100 = extraout_RAX[1];
    lStack_f8 = 3;
    local_f0 = &DAT_142aa0d80;
    lStack_e8 = 0x27;
    local_128 = 0;
    local_120 = (code *)&DAT_142aa0d80;
    lStack_118 = 0x27;
    local_110 = 0;
    ::log::__private_api::impl$2::log();
    if (local_a8 != (code *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (local_a8 != (code *)0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (SBORROW8(0,(longlong)local_60)) {
    if (((uint)local_58 & 3) == 1) {
      local_30 = local_58 + -1;
      local_28 = *(code ***)(local_58 + -1);
      local_38 = *(code **)(local_58 + 7);
      if (*(code **)local_38 != (code *)0x0) {
        (**(code **)local_38)(local_28);
      }
      if (*(longlong *)(local_38 + 8) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  else {
    local_28 = local_60;
    local_38 = local_58;
    serde_json::de::
    from_str<enum2_<core::option::Option<country_core::startup::load_glade_audio::GladeAudio>_>_>();
    if (local_128 != -0x7fffffffffffffff) {
      local_78 = local_f0;
      lStack_70 = lStack_e8;
      local_88 = lStack_100;
      lStack_80 = lStack_f8;
      local_98 = local_110;
      lStack_90 = lStack_108;
      local_a8 = local_120;
      lStack_a0 = lStack_118;
      if (local_28 != (code **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (*local_40 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (!SBORROW8(0,local_128)) {
        *in_RCX = local_128;
        in_RCX[1] = (longlong)local_a8;
        in_RCX[2] = lStack_a0;
        in_RCX[3] = local_98;
        in_RCX[4] = lStack_90;
        in_RCX[5] = local_88;
        in_RCX[6] = lStack_80;
        in_RCX[7] = (longlong)local_78;
        in_RCX[8] = lStack_70;
        return;
      }
      goto LAB_1408bf7f4;
    }
    local_30 = local_120;
    if (*(longlong *)local_120 == 1) {
      core::ptr::drop_in_place<std::io::error::Error>();
    }
    else if ((*(longlong *)local_120 == 0) && (*(longlong *)(local_120 + 0x10) != 0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    std::alloc::__default_lib_allocator::__rust_dealloc();
    if (local_28 != (code **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (*local_40 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_1408bf7f4:
  load_glade_audio::impl$0::default();
  return;
}


