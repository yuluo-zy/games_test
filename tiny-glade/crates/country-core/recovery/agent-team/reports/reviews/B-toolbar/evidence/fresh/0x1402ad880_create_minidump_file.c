
void __cdecl tiny_glade::crash_pickup::server::impl$1::create_minidump_file(void)

{
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong *in_RCX;
  longlong lVar1;
  longlong *local_b8;
  code *local_b0;
  longlong local_a8;
  undefined8 local_a0;
  longlong local_90;
  longlong lStack_88;
  longlong local_80;
  longlong lStack_78;
  undefined *local_70;
  undefined8 local_68;
  undefined8 **ppuStack_60;
  undefined4 local_58;
  undefined4 uStack_54;
  undefined1 local_50;
  undefined1 uStack_4f;
  undefined1 uStack_4e;
  undefined1 uStack_4d;
  undefined1 uStack_4c;
  undefined1 uStack_4b;
  undefined2 uStack_4a;
  undefined8 local_40;
  longlong local_38;
  undefined1 local_29;
  undefined8 local_28;
  
  local_28 = 0xfffffffffffffffe;
  local_70 = (undefined *)CONCAT71(local_70._1_7_,1);
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::DirBuilder::_create();
  if (extraout_RAX == 0) {
    uuid::Uuid::new_v4();
    local_b0 = uuid::fmt::impl$1::fmt;
    local_70 = &DAT_1429ad1e0;
    local_68 = 2;
    ppuStack_60 = &local_b8;
    local_58 = 1;
    uStack_54 = 0;
    local_50 = 0;
    uStack_4f = 0;
    uStack_4e = 0;
    uStack_4d = 0;
    uStack_4c = 0;
    uStack_4b = 0;
    uStack_4a = 0;
    local_b8 = &local_90;
    alloc::fmt::format::format_inner();
    local_29 = 1;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    local_38 = local_a8;
    local_40 = local_a0;
    std::path::Path::_join();
    if (local_38 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_70 = (undefined *)((ulonglong)local_70 & 0xffffffff00000000);
    local_68 = 0;
    ppuStack_60 = (undefined8 **)0x0;
    local_58 = 7;
    uStack_54 = 0;
    local_50 = 0;
    uStack_4e = 0;
    uStack_4b = 0;
    uStack_4f = 1;
    uStack_4c = 1;
    uStack_4d = 1;
    std::sys::os_str::wtf8::Buf::as_mut_slice();
    lVar1 = extraout_RAX_00;
    std::fs::OpenOptions::_open();
    if (extraout_RAX_01 == 0) {
      in_RCX[3] = local_80;
      in_RCX[4] = lStack_78;
      in_RCX[1] = local_90;
      in_RCX[2] = lStack_88;
      *in_RCX = lVar1;
    }
    else {
      *in_RCX = lVar1;
      in_RCX[1] = -0x8000000000000000;
      if (local_90 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  else {
    *in_RCX = extraout_RAX;
    in_RCX[1] = -0x8000000000000000;
  }
  return;
}

