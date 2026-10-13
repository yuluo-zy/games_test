// flattened  0x140b00b20
// signature: void __cdecl flattened(void)
// callees: [{"name": "__rust_alloc", "address": "140613c10"}, {"name": "clone", "address": "140d7bd90"}, {"name": "handle_error", "address": "1428d8fe3"}]


void __cdecl country_core::resources::grading_lut::ColorGradingState::flattened(void)

{
  undefined4 *extraout_RAX;
  undefined8 *in_RCX;
  longlong in_RDX;
  undefined4 uVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  
  if (*(char *)(in_RDX + 0x28) == '\0') {
    alloc::string::impl$6::clone();
    uVar1 = *(undefined4 *)(in_RDX + 0x18);
    uVar2 = *(undefined4 *)(in_RDX + 0x1c);
    uVar3 = *(undefined4 *)(in_RDX + 0x20);
    uVar4 = *(undefined4 *)(in_RDX + 0x24);
    *(undefined1 *)(in_RCX + 5) = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined4 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    *(undefined4 *)((longlong)extraout_RAX + 3) = 0x746c7561;
    *extraout_RAX = 0x61666564;
    *(undefined1 *)(in_RCX + 5) = 0;
    *in_RCX = 7;
    in_RCX[1] = extraout_RAX;
    in_RCX[2] = 7;
    uVar1 = 0x3f800000;
    uVar2 = 0;
    uVar3 = 0x3f800000;
    uVar4 = 0x3f800000;
  }
  *(undefined4 *)(in_RCX + 3) = uVar1;
  *(undefined4 *)((longlong)in_RCX + 0x1c) = uVar2;
  *(undefined4 *)(in_RCX + 4) = uVar3;
  *(undefined4 *)((longlong)in_RCX + 0x24) = uVar4;
  return;
}

