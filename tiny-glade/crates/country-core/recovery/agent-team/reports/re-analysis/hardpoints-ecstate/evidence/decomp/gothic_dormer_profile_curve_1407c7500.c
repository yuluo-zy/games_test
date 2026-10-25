
void __cdecl system_decorator::decorator_visual::gothic_dormer::gothic_dormer_profile_curve(void)

{
  char cVar1;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  undefined8 *puVar2;
  undefined8 *in_RCX;
  undefined4 *puVar3;
  longlong in_RDX;
  undefined8 uVar4;
  
  cVar1 = *(char *)(in_RDX + 0x44);
  if (cVar1 == '\x01') {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_01 = 0x3f6e147b00000000;
    extraout_RAX_01[1] = 0x3f47ae143e19999a;
    extraout_RAX_01[2] = 0x3f19999a3e851eb8;
    extraout_RAX_01[3] = 0x3ecccccd3ea3d70a;
    extraout_RAX_01[4] = 0x3e428f5c3eb33333;
    *(undefined4 *)(extraout_RAX_01 + 5) = 0x3eb33333;
    puVar3 = (undefined4 *)((longlong)extraout_RAX_01 + 0x2c);
    uVar4 = 6;
    puVar2 = extraout_RAX_01;
  }
  else if (cVar1 == '\x02') {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX_00 = 0x3f88f5c300000000;
    extraout_RAX_00[1] = 0x3f7333333e3851ec;
    extraout_RAX_00[2] = 0x3f4f5c293eae147b;
    extraout_RAX_00[3] = 0x3f2147ae3eeb851f;
    extraout_RAX_00[4] = 0x3ee147ae3f0a3d71;
    extraout_RAX_00[5] = 0x3e6b851f3f170a3d;
    *(undefined4 *)(extraout_RAX_00 + 6) = 0x3f170a3d;
    puVar3 = (undefined4 *)((longlong)extraout_RAX_00 + 0x34);
    uVar4 = 7;
    puVar2 = extraout_RAX_00;
  }
  else {
    if (cVar1 != '\x03') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 0x3fb9999a00000000;
    extraout_RAX[1] = 0x3fab851f3e428f5c;
    extraout_RAX[2] = 0x3f9851ec3eb851ec;
    extraout_RAX[3] = 0x3f828f5c3f000000;
    extraout_RAX[4] = 0x3f51eb853f1c28f6;
    extraout_RAX[5] = 0x3f1eb8523f30a3d7;
    extraout_RAX[6] = 0x3ecccccd3f3ae148;
    extraout_RAX[7] = 0x3e3851ec3f428f5c;
    *(undefined4 *)(extraout_RAX + 8) = 0x3f428f5c;
    puVar3 = (undefined4 *)((longlong)extraout_RAX + 0x44);
    uVar4 = 9;
    puVar2 = extraout_RAX;
  }
  *puVar3 = 0;
  *in_RCX = uVar4;
  in_RCX[1] = puVar2;
  in_RCX[2] = uVar4;
  return;
}

