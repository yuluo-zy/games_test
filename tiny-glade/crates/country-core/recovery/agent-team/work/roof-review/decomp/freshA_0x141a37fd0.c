// 0x141a37fd0
// void __cdecl roof_record_history(void)
// roof_record_history


void __cdecl system_roof::deferred_record_roof_history::roof_record_history(void)

{
  ulonglong uVar1;
  ulonglong uVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  ulonglong uVar14;
  undefined8 *extraout_RAX;
  longlong in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  ulonglong uVar17;
  longlong lVar18;
  longlong lVar19;
  ulonglong uVar20;
  ulonglong uVar21;
  longlong lVar22;
  ulonglong uVar23;
  ulonglong uVar24;
  bool bVar25;
  bool bVar26;
  
  lVar18 = in_RDX[1];
  uVar1 = *(ulonglong *)*in_RDX;
  uVar24 = 0;
  uVar15 = uVar1 - *(ulonglong *)(lVar18 + 0x18);
  if (uVar1 < *(ulonglong *)(lVar18 + 0x18)) {
    uVar15 = uVar24;
  }
  uVar14 = uVar1 - *(ulonglong *)(lVar18 + 0x38);
  if (uVar1 < *(ulonglong *)(lVar18 + 0x38)) {
    uVar14 = uVar24;
  }
  uVar1 = *(ulonglong *)(lVar18 + 0x10);
  uVar2 = *(ulonglong *)(lVar18 + 0x30);
  bVar25 = uVar2 < uVar14;
  uVar20 = uVar2 - uVar14;
  uVar21 = uVar20;
  if (bVar25) {
    uVar21 = uVar24;
  }
  bVar26 = uVar1 < uVar15;
  uVar16 = uVar1 - uVar15;
  uVar17 = uVar16;
  if (bVar26) {
    uVar17 = uVar24;
  }
  lVar19 = *(longlong *)(lVar18 + 8);
  uVar23 = uVar21 + uVar17;
  if (!bVar26 && uVar16 != 0 || !bVar25 && uVar20 != 0) {
    uVar23 = uVar24;
  }
  lVar22 = *(longlong *)(lVar18 + 0x28);
  *(ulonglong *)*in_RDX = *(longlong *)(lVar18 + 0x40) - uVar23;
  if (!bVar26 && uVar16 != 0 || !bVar25 && uVar20 != 0) {
    lVar22 = lVar22 + uVar14 * 0x98;
    lVar19 = lVar19 + uVar15 * 0x98;
    if (uVar1 < uVar15) {
      lVar19 = 8;
    }
    lVar18 = 0;
    if (uVar15 < uVar1) {
      lVar18 = lVar19 + -0x98 + uVar17 * 0x98;
    }
    if (uVar2 < uVar14) {
      lVar22 = 8;
    }
    lVar19 = lVar22 + -0x98 + uVar21 * 0x98;
    if (uVar2 <= uVar14) {
      lVar19 = lVar18;
    }
    **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
    uVar12 = *(undefined8 *)(lVar19 + 0x10);
    uVar13 = *(undefined8 *)(lVar19 + 0x18);
    uVar4 = *(undefined4 *)(lVar19 + 0x20);
    uVar5 = *(undefined4 *)(lVar19 + 0x24);
    uVar6 = *(undefined4 *)(lVar19 + 0x28);
    uVar7 = *(undefined4 *)(lVar19 + 0x2c);
    uVar3 = *(undefined8 *)(lVar19 + 0x5c);
    uVar8 = *(undefined4 *)(lVar19 + 100);
    uVar9 = *(undefined4 *)(lVar19 + 0x68);
    uVar10 = *(undefined4 *)(lVar19 + 0x6c);
    uVar11 = *(undefined4 *)(lVar19 + 0x70);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = 1;
    extraout_RAX[1] = 1;
    *(undefined1 *)(extraout_RAX + 2) = 2;
    *(undefined4 *)((longlong)extraout_RAX + 0x14) = uVar5;
    *(undefined4 *)(extraout_RAX + 3) = uVar4;
    *(undefined4 *)((longlong)extraout_RAX + 0x1c) = uVar6;
    *(undefined4 *)(extraout_RAX + 4) = uVar7;
    *(undefined4 *)((longlong)extraout_RAX + 0x24) = uVar9;
    *(undefined4 *)(extraout_RAX + 5) = uVar8;
    *(undefined4 *)((longlong)extraout_RAX + 0x2c) = uVar10;
    *(undefined4 *)(extraout_RAX + 6) = uVar11;
    extraout_RAX[7] = uVar12;
    extraout_RAX[8] = uVar13;
    extraout_RAX[9] = uVar3;
    country_core::resources::history::History::add();
  }
  return;
}


