
void __cdecl country_core::resources::gothic_window_profile::gothic_half_dormer_profile_curve(void)

{
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *puVar1;
  undefined8 *in_RCX;
  undefined4 *puVar2;
  longlong in_RDX;
  undefined8 uVar3;
  
  if (in_RDX == 1) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 0x3fa147ae00000000;
    extraout_RAX_01[1] = 0x3f8b851f3e4ccccd;
    extraout_RAX_01[2] = 0x3f5eb8523eb33333;
    extraout_RAX_01[3] = 0x3f1eb8523ee147ae;
    extraout_RAX_01[4] = 0x3eb851ec3ef5c28f;
    *(undefined4 *)(extraout_RAX_01 + 5) = 0x3ef5c28f;
    puVar2 = (undefined4 *)((longlong)extraout_RAX_01 + 0x2c);
    uVar3 = 6;
    puVar1 = extraout_RAX_01;
  }
  else if (in_RDX == 2) {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_00 = 0x3fd9999a00000000;
    extraout_RAX_00[1] = 0x3fcccccd3e570a3d;
    extraout_RAX_00[2] = 0x3fbae1483ec7ae14;
    extraout_RAX_00[3] = 0x3fa51eb83f0a3d71;
    extraout_RAX_00[4] = 0x3f8b851f3f266666;
    extraout_RAX_00[5] = 0x3f6147ae3f3851ec;
    extraout_RAX_00[6] = 0x3f2666663f400000;
    *(undefined4 *)(extraout_RAX_00 + 7) = 0x3f400000;
    puVar2 = (undefined4 *)((longlong)extraout_RAX_00 + 0x3c);
    uVar3 = 8;
    puVar1 = extraout_RAX_00;
  }
  else {
    if (in_RDX != 3) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 0x4010000000000000;
    extraout_RAX[1] = 0x4008f5c33e570a3d;
    extraout_RAX[2] = 0x400000003ecccccd;
    extraout_RAX[3] = 0x3fe8f5c33f11eb85;
    extraout_RAX[4] = 0x3fd0a3d73f35c28f;
    extraout_RAX[5] = 0x3fb5c28f3f51eb85;
    extraout_RAX[6] = 0x3f9851ec3f666666;
    extraout_RAX[7] = 0x3f75c28f3f733333;
    extraout_RAX[8] = 0x3f3851ec3f75c28f;
    *(undefined4 *)(extraout_RAX + 9) = 0x3f75c28f;
    puVar2 = (undefined4 *)((longlong)extraout_RAX + 0x4c);
    uVar3 = 10;
    puVar1 = extraout_RAX;
  }
  *puVar2 = 0xbf733333;
  *in_RCX = uVar3;
  in_RCX[1] = puVar1;
  in_RCX[2] = uVar3;
  return;
}

