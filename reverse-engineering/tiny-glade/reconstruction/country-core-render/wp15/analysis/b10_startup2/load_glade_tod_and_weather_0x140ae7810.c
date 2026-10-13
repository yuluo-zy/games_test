// addr=0x140ae7810 name=load_glade_tod_and_weather
// signature=void __cdecl load_glade_tod_and_weather(void)


void __cdecl country_core::startup::load_glade_tod_and_weather::load_glade_tod_and_weather(void)

{
  undefined4 uVar1;
  longlong lVar2;
  undefined8 *puVar3;
  code *pcVar4;
  longlong *plVar5;
  undefined4 *puVar6;
  longlong lVar7;
  longlong *in_RCX;
  undefined8 *in_RDX;
  longlong *in_R8;
  longlong *in_R9;
  undefined4 uVar8;
  longlong local_c0;
  longlong local_b8;
  longlong local_88;
  longlong lStack_80;
  longlong local_78;
  longlong local_68;
  longlong lStack_60;
  longlong local_58;
  
  lVar2 = *in_R9;
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::path::Path::_join();
  std::sys::os_str::wtf8::Buf::as_mut_slice();
  std::fs::read_to_string::inner();
  if (local_68 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (SBORROW8(0,local_c0)) {
    if (((uint)local_b8 & 3) == 1) {
      puVar3 = *(undefined8 **)(local_b8 + 7);
      pcVar4 = (code *)*puVar3;
      if (pcVar4 != (code *)0x0) {
        (*pcVar4)(*(undefined8 *)(local_b8 + -1));
      }
      if (puVar3[1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_88 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    std::fs::read_to_string::inner();
    if (SBORROW8(0,local_88)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    serde_json::de::from_str<country_core::resources::time_of_day::TodSequence>();
    if (SBORROW8(0,local_88)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    plVar5 = (longlong *)*in_RCX;
    if (*plVar5 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar5[2] = local_78;
    *plVar5 = local_88;
    plVar5[1] = lStack_80;
  }
  else {
    serde_json::de::from_str<country_core::resources::time_of_day::TodSequence>();
    if (SBORROW8(0,local_68)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    plVar5 = (longlong *)*in_RCX;
    if (*plVar5 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    plVar5[2] = local_58;
    *plVar5 = local_68;
    plVar5[1] = lStack_60;
    if (local_c0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (local_88 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uVar1 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar6 = (undefined4 *)in_RDX[2];
  *puVar6 = uVar1;
  puVar3 = (undefined8 *)*in_RDX;
  *puVar3 = 0x3f80000000000000;
  puVar3[1] = 0;
  lVar7 = *in_R8;
  uVar8 = *(undefined4 *)(lVar7 + 8);
  *puVar6 = uVar1;
  *(undefined4 *)puVar3 = uVar8;
  *(undefined4 *)((longlong)puVar3 + 4) = *(undefined4 *)(lVar7 + 0xc);
  *puVar6 = uVar1;
  *(undefined4 *)(puVar3 + 1) = 0;
  if (*(char *)(lVar2 + 0x58) == '\x04') {
    uVar8 = 0x3f800000;
  }
  else {
    uVar8 = 0;
  }
  *puVar6 = uVar1;
  *(undefined4 *)((longlong)puVar3 + 0xc) = uVar8;
  return;
}


