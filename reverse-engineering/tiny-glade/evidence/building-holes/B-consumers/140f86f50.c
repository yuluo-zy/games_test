
void __cdecl country_core::resources::walls::wall_holes::WallHoleStorage::clone_and_reopen(void)

{
  undefined1 auVar1 [16];
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  ulonglong uVar5;
  undefined1 *extraout_RAX;
  undefined1 *extraout_RAX_00;
  ulonglong _Size;
  undefined1 *extraout_RAX_01;
  undefined1 *extraout_RAX_02;
  undefined8 *in_RCX;
  byte *in_RDX;
  byte bVar6;
  void *pvVar7;
  ulonglong uVar8;
  undefined1 *_Dst;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  ulonglong local_98;
  undefined1 *local_90;
  ulonglong local_88;
  
  if ((*in_RDX & 1) == 0) {
    local_98 = *(ulonglong *)(in_RDX + 0x18);
    auVar2._8_8_ = 0;
    auVar2._0_8_ = local_98;
    uVar5 = SUB168(auVar2 * ZEXT816(0x14),0);
    if ((SUB168(auVar2 * ZEXT816(0x14),8) != 0) || (0x7ffffffffffffffc < uVar5)) goto LAB_140f871f0;
    pvVar7 = *(void **)(in_RDX + 0x10);
    if (uVar5 == 0) {
      local_90 = &DAT_00000004;
      local_88 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_90 = extraout_RAX_00;
      local_88 = local_98;
      if (extraout_RAX_00 == (undefined1 *)0x0) goto LAB_140f871f0;
    }
    memcpy(local_90,pvVar7,uVar5);
    uVar5 = *(ulonglong *)(in_RDX + 0x30);
    auVar4._8_8_ = 0;
    auVar4._0_8_ = uVar5;
    _Size = SUB168(auVar4 * ZEXT816(0x14),0);
    if ((SUB168(auVar4 * ZEXT816(0x14),8) != 0) || (0x7ffffffffffffffc < _Size)) {
LAB_140f87202:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    uVar9 = *(undefined4 *)(in_RDX + 0x40);
    uVar10 = *(undefined4 *)(in_RDX + 0x44);
    uVar11 = *(undefined4 *)(in_RDX + 0x48);
    uVar12 = *(undefined4 *)(in_RDX + 0x4c);
    uVar13 = *(undefined4 *)(in_RDX + 0x50);
    uVar14 = *(undefined4 *)(in_RDX + 0x54);
    uVar15 = *(undefined4 *)(in_RDX + 0x58);
    uVar16 = *(undefined4 *)(in_RDX + 0x5c);
    uVar17 = *(undefined4 *)(in_RDX + 0x38);
    bVar6 = in_RDX[0x3c];
    pvVar7 = *(void **)(in_RDX + 0x28);
    if (_Size != 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar8 = uVar5;
      _Dst = extraout_RAX_02;
      if (extraout_RAX_02 == (undefined1 *)0x0) goto LAB_140f87202;
      goto LAB_140f87184;
    }
  }
  else {
    local_98 = *(ulonglong *)(in_RDX + 0x38);
    auVar1._8_8_ = 0;
    auVar1._0_8_ = local_98;
    uVar5 = SUB168(auVar1 * ZEXT816(0x14),0);
    if ((SUB168(auVar1 * ZEXT816(0x14),8) != 0) || (0x7ffffffffffffffc < uVar5)) {
LAB_140f871f0:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    pvVar7 = *(void **)(in_RDX + 0x30);
    if (uVar5 == 0) {
      local_90 = &DAT_00000004;
      local_88 = 0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_90 = extraout_RAX;
      local_88 = local_98;
      if (extraout_RAX == (undefined1 *)0x0) goto LAB_140f871f0;
    }
    memcpy(local_90,pvVar7,uVar5);
    uVar5 = *(ulonglong *)(in_RDX + 0x18);
    auVar3._8_8_ = 0;
    auVar3._0_8_ = uVar5;
    _Size = SUB168(auVar3 * ZEXT816(0x14),0);
    if ((SUB168(auVar3 * ZEXT816(0x14),8) != 0) || (0x7ffffffffffffffc < _Size)) {
LAB_140f870d8:
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    uVar9 = *(undefined4 *)(in_RDX + 0x40);
    uVar10 = *(undefined4 *)(in_RDX + 0x44);
    uVar11 = *(undefined4 *)(in_RDX + 0x48);
    uVar12 = *(undefined4 *)(in_RDX + 0x4c);
    uVar13 = *(undefined4 *)(in_RDX + 0x50);
    uVar14 = *(undefined4 *)(in_RDX + 0x54);
    uVar15 = *(undefined4 *)(in_RDX + 0x58);
    uVar16 = *(undefined4 *)(in_RDX + 0x5c);
    uVar17 = *(undefined4 *)(in_RDX + 0x20);
    bVar6 = in_RDX[0x24];
    pvVar7 = *(void **)(in_RDX + 0x10);
    if (_Size != 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      uVar8 = uVar5;
      _Dst = extraout_RAX_01;
      if (extraout_RAX_01 != (undefined1 *)0x0) goto LAB_140f87184;
      goto LAB_140f870d8;
    }
  }
  _Dst = &DAT_00000004;
  uVar8 = 0;
LAB_140f87184:
  memcpy(_Dst,pvVar7,_Size);
  in_RCX[1] = local_88;
  in_RCX[2] = local_90;
  in_RCX[3] = local_98;
  in_RCX[4] = uVar8;
  in_RCX[5] = _Dst;
  in_RCX[6] = uVar5;
  *(undefined4 *)(in_RCX + 7) = uVar17;
  *(byte *)((longlong)in_RCX + 0x3c) = bVar6;
  *(undefined4 *)(in_RCX + 8) = uVar9;
  *(undefined4 *)((longlong)in_RCX + 0x44) = uVar10;
  *(undefined4 *)(in_RCX + 9) = uVar11;
  *(undefined4 *)((longlong)in_RCX + 0x4c) = uVar12;
  *(undefined4 *)(in_RCX + 10) = uVar13;
  *(undefined4 *)((longlong)in_RCX + 0x54) = uVar14;
  *(undefined4 *)(in_RCX + 0xb) = uVar15;
  *(undefined4 *)((longlong)in_RCX + 0x5c) = uVar16;
  *in_RCX = 0;
  return;
}

