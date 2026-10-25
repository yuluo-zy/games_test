
void __cdecl country_core::systems::garden::record_garden_history::record_garden_history(void)

{
  undefined4 uVar1;
  undefined1 uVar2;
  ulonglong *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong lVar7;
  void *_Src;
  longlong lVar8;
  size_t _Size;
  undefined8 uVar9;
  undefined8 *puVar10;
  undefined8 *extraout_RAX;
  undefined8 *extraout_RAX_00;
  undefined8 *extraout_RAX_01;
  longlong *in_RCX;
  undefined8 *in_RDX;
  size_t sVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  undefined4 uStack_84;
  
  puVar3 = (ulonglong *)*in_RDX;
  lVar4 = in_RDX[1];
  uVar5 = *puVar3;
  uVar14 = 0;
  uVar15 = uVar5 - *(ulonglong *)(lVar4 + 0x18);
  if (uVar5 < *(ulonglong *)(lVar4 + 0x18)) {
    uVar15 = uVar14;
  }
  uVar13 = uVar5 - *(ulonglong *)(lVar4 + 0x38);
  if (uVar5 < *(ulonglong *)(lVar4 + 0x38)) {
    uVar13 = uVar14;
  }
  uVar5 = *(ulonglong *)(lVar4 + 0x30);
  uVar16 = uVar5 - uVar13;
  if (uVar5 < uVar13) {
    uVar16 = uVar14;
  }
  uVar6 = *(ulonglong *)(lVar4 + 0x10);
  uVar12 = uVar6 - uVar15;
  if (uVar6 < uVar15) {
    uVar12 = uVar14;
  }
  uVar14 = *(ulonglong *)(lVar4 + 0x40);
  *puVar3 = uVar14 - (uVar12 + uVar16);
  if (uVar6 >= uVar15 && uVar6 - uVar15 != 0 || uVar5 >= uVar13 && uVar5 - uVar13 != 0) {
    *puVar3 = uVar14;
    lVar4 = *in_RCX;
    uVar9 = *(undefined8 *)(lVar4 + 0x1698);
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    lVar7 = *(longlong *)(lVar4 + 0x1678);
    _Src = *(void **)(lVar4 + 0x1680);
    lVar8 = *(longlong *)(lVar4 + 0x1688);
    uVar1 = *(undefined4 *)(lVar4 + 0x1690);
    uVar2 = *(undefined1 *)(lVar4 + 0x1694);
    *(undefined8 *)(lVar4 + 0x1678) = 0;
    *(undefined8 *)(lVar4 + 0x1680) = 4;
    *(undefined8 *)(lVar4 + 0x1688) = 0;
    *(undefined4 *)(lVar4 + 0x1690) = 0;
    *(undefined1 *)(lVar4 + 0x1694) = 0;
    *(undefined4 *)in_RCX[6] = *(undefined4 *)((longlong)in_RCX + 0x3c);
    _Size = lVar8 * 0xc;
    sVar11 = _Size;
    alloc::sync::arcinner_layout_for_value_layout();
    puVar10 = extraout_RAX;
    if (sVar11 != 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      puVar10 = extraout_RAX_00;
    }
    if (puVar10 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *puVar10 = 1;
    puVar10[1] = 1;
    memcpy(puVar10 + 2,_Src,_Size);
    if (lVar7 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    uStack_84 = CONCAT31(uStack_84._1_3_,uVar2);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    extraout_RAX_01[4] = lVar8;
    extraout_RAX_01[5] = CONCAT44(uStack_84,uVar1);
    extraout_RAX_01[2] = uVar9;
    extraout_RAX_01[3] = puVar10;
    *extraout_RAX_01 = 1;
    extraout_RAX_01[1] = 1;
    resources::history::History::add();
  }
  return;
}

