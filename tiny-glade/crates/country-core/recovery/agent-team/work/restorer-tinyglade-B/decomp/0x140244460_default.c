// default @ 0x140244460
// sig: void __cdecl default(void)


void __cdecl tiny_glade::aftermath::impl$0::default(void)

{
  char extraout_AL;
  undefined7 *extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar1;
  undefined1 *in_RCX;
  undefined7 *puVar2;
  longlong *local_108;
  code *local_100;
  longlong *local_f8;
  code *local_f0;
  undefined7 local_e8;
  undefined1 uStack_e1;
  undefined7 uStack_e0;
  undefined1 uStack_d9;
  undefined7 uStack_d8;
  undefined1 uStack_d1;
  undefined7 uStack_d0;
  undefined1 uStack_c9;
  undefined7 local_c8;
  undefined1 uStack_c1;
  undefined8 uStack_c0;
  undefined8 local_b8;
  char *local_b0;
  undefined8 local_a8;
  undefined4 local_a0;
  undefined4 local_9c;
  undefined *local_98;
  undefined8 local_90;
  undefined8 **local_88;
  undefined8 local_80;
  undefined8 local_78;
  longlong local_68 [4];
  longlong local_48;
  longlong local_40;
  undefined8 local_38;
  undefined8 *local_30;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  paths::tmp();
  puVar2 = extraout_RAX;
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::is_file();
  if (extraout_AL != '\0') {
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    std::sys::pal::windows::fs::unlink();
    if (extraout_RAX_00 != 0) {
      local_40 = extraout_RAX_00;
      if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
        local_100 = std::path::impl$44::fmt;
        local_f8 = &local_40;
        local_f0 = std::io::error::impl$0::fmt;
        local_108 = local_68;
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        uStack_c0 = extraout_RAX_01[1];
        local_9c = *(undefined4 *)(extraout_RAX_01 + 2);
        local_b8 = 1;
        local_b0 = "tiny_glade::aftermathNV Aftermath dump data received.";
        local_a8 = 0x15;
        local_98 = &anon_c6ad10f63d8c6b0a9569b9f7874b92f2_59_llvm_165846970094024458;
        local_90 = 2;
        local_88 = &local_108;
        local_80 = 2;
        local_78 = 0;
        local_e8 = 0;
        uStack_e1 = 0;
        uStack_e0 = 0x1429a0590;
        uStack_d9 = 0;
        uStack_d8 = 0x15;
        uStack_d1 = 0;
        uStack_d0 = 0;
        uStack_c9 = 0;
        local_c8 = (undefined7)*extraout_RAX_01;
        uStack_c1 = (undefined1)((ulonglong)*extraout_RAX_01 >> 0x38);
        local_a0 = 1;
        puVar2 = &local_e8;
        ::log::__private_api::impl$2::log();
      }
      if (((uint)local_40 & 3) == 1) {
        local_48 = local_40 + -1;
        local_38 = *(undefined8 *)(local_40 + -1);
        local_30 = *(undefined8 **)(local_40 + 7);
        if ((code *)*local_30 != (code *)0x0) {
          (*(code *)*local_30)(local_38);
        }
        if (local_30[1] != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        puVar2 = (undefined7 *)&DAT_00000018;
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_02 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_02[0] = 1;
    extraout_RAX_02[1] = 0;
    extraout_RAX_02[2] = 0;
    extraout_RAX_02[3] = 0;
    extraout_RAX_02[4] = 0;
    extraout_RAX_02[5] = 0;
    extraout_RAX_02[6] = 0;
    extraout_RAX_02[7] = 0;
    *(longlong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
    *(undefined7 **)(extraout_RAX_02 + 0x10) = puVar2;
    lVar1 = extraout_RAX_03;
  }
  else {
    lVar1 = *(longlong *)(extraout_RAX_02 + 8);
    puVar2 = *(undefined7 **)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = lVar1 + 1;
  uStack_d1 = 0;
  uStack_d0 = 0;
  uStack_c9 = 0;
  local_c8 = 0;
  uStack_e1 = 0x80;
  uStack_e0 = 0x1429a02;
  uStack_d9 = 0;
  uStack_d8 = 0;
  *in_RCX = 0;
  *(ulonglong *)(in_RCX + 1) = CONCAT17(0x80,local_e8);
  *(undefined8 *)(in_RCX + 9) = 0x1429a02;
  *(undefined8 *)(in_RCX + 0x11) = 0;
  *(undefined8 *)(in_RCX + 0x19) = 0;
  *(undefined8 *)(in_RCX + 0x20) = 0;
  *(longlong *)(in_RCX + 0x28) = lVar1;
  *(undefined7 **)(in_RCX + 0x30) = puVar2;
  if (local_68[0] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

