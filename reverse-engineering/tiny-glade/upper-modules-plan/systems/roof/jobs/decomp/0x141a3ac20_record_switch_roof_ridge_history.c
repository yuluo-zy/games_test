// name=record_switch_roof_ridge_history
// sig=void __cdecl record_switch_roof_ridge_history(void)
// size=391

void __cdecl system_roof::ui_edit_roof_ridge_dir::record_switch_roof_ridge_history(void)

{
  longlong lVar1;
  undefined1 uVar2;
  undefined1 uVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  undefined4 *puVar8;
  undefined8 uVar9;
  longlong lVar10;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  ulonglong uVar11;
  longlong lVar12;
  longlong in_RDX;
  longlong lVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  
  puVar5 = (ulonglong *)*in_RCX;
  lVar6 = in_RCX[1];
  uVar7 = *puVar5;
  uVar14 = 0;
  uVar11 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
    uVar11 = uVar14;
  }
  uVar15 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
  if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
    uVar15 = uVar14;
  }
  lVar10 = uVar11 * 0x18 + *(longlong *)(lVar6 + 8);
  uVar7 = *(ulonglong *)(lVar6 + 0x10) - uVar11;
  if (*(ulonglong *)(lVar6 + 0x10) < uVar11) {
    lVar10 = 8;
    uVar7 = uVar14;
  }
  lVar12 = uVar15 * 0x18 + *(longlong *)(lVar6 + 0x28);
  uVar11 = *(ulonglong *)(lVar6 + 0x30) - uVar15;
  if (*(ulonglong *)(lVar6 + 0x30) < uVar15) {
    lVar12 = 8;
    uVar11 = uVar14;
  }
  *puVar5 = (*(longlong *)(lVar6 + 0x40) - uVar11) - uVar7;
  lVar6 = lVar10 + uVar7 * 0x18;
  lVar1 = lVar12 + uVar11 * 0x18;
  uVar4 = *(undefined4 *)(in_RDX + 0x1c);
  puVar8 = *(undefined4 **)(in_RDX + 0x10);
  if (lVar10 == 0) goto LAB_141a3ace0;
  do {
    if (lVar10 == lVar6) goto LAB_141a3ace0;
    lVar13 = lVar10 + 0x18;
    while( true ) {
      *puVar5 = *puVar5 + 1;
      uVar9 = *(undefined8 *)(lVar10 + 8);
      uVar2 = *(undefined1 *)(lVar10 + 0x10);
      uVar3 = *(undefined1 *)(lVar10 + 0x11);
      *puVar8 = uVar4;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX = 1;
      extraout_RAX[1] = 1;
      *(undefined1 *)(extraout_RAX + 2) = 3;
      *(undefined1 *)((longlong)extraout_RAX + 0x11) = uVar2;
      *(undefined1 *)((longlong)extraout_RAX + 0x12) = uVar3;
      extraout_RAX[3] = uVar9;
      country_core::resources::history::History::add();
      lVar10 = lVar13;
      if (lVar13 != 0) break;
LAB_141a3ace0:
      lVar10 = lVar12;
      if ((lVar10 == 0) || (lVar10 == lVar1)) {
        return;
      }
      lVar12 = lVar10 + 0x18;
      lVar13 = 0;
    }
  } while( true );
}

