
void __cdecl country_core::resources::gothic_window_profile::gothic_door_profile_curve(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *puVar1;
  undefined8 *in_RCX;
  undefined8 uVar2;
  longlong in_RDX;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  
  if (in_RDX == 1) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
LAB_1409a4f6b:
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 0x3fd70a3d00000000;
    extraout_RAX_01[1] = 0x3fbc28f63ea3d70a;
    extraout_RAX_01[2] = 0x3f95c28f3f000000;
    extraout_RAX_01[3] = 0x3f51eb853f000000;
    uVar3 = 0x3f0a3d71;
    uVar4 = 0x3ee147ae;
    uVar5 = 0x3f0a3d71;
    uVar6 = 0xbfbd70a4;
    puVar1 = extraout_RAX_01;
  }
  else {
    if (in_RDX == 2) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_00 = 0x3fe0000000000000;
      extraout_RAX_00[1] = 0x3fc8f5c33ea3d70a;
      extraout_RAX_00[2] = 0x3fa7ae143f147ae1;
      extraout_RAX_00[3] = 0x3f8000003f4ccccd;
      extraout_RAX_00[4] = 0xbfc8f5c33f4ccccd;
      uVar2 = 5;
      puVar1 = extraout_RAX_00;
      goto LAB_1409a4f57;
    }
    if (in_RDX != 3) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) goto LAB_1409a4f6b;
    *extraout_RAX = 0x3ff3333300000000;
    extraout_RAX[1] = 0x3fe000003eae147b;
    extraout_RAX[2] = 0x3fc3d70a3f23d70a;
    extraout_RAX[3] = 0x3fa147ae3f666666;
    uVar3 = 0x3f8ccccd;
    uVar4 = 0x3f170a3d;
    uVar5 = 0x3f8ccccd;
    uVar6 = 0xbfd33333;
    puVar1 = extraout_RAX;
  }
  *(undefined4 *)(puVar1 + 4) = uVar3;
  *(undefined4 *)((longlong)puVar1 + 0x24) = uVar4;
  *(undefined4 *)(puVar1 + 5) = uVar5;
  *(undefined4 *)((longlong)puVar1 + 0x2c) = uVar6;
  uVar2 = 6;
LAB_1409a4f57:
  *in_RCX = uVar2;
  in_RCX[1] = puVar1;
  in_RCX[2] = uVar2;
  return;
}

