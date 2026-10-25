// ADDR 0x14081b5e0
// SIG void __cdecl decorator_on_wall_height_changed(void)
// ORIG system::decorator_on_wall_height_changed


void __cdecl
system_decorator::decorator_on_wall_height_changed::decorator_on_wall_height_changed(void)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  longlong lVar3;
  ulonglong uVar4;
  undefined8 *in_RDX;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong lVar7;
  undefined8 *in_R8;
  ulonglong uVar8;
  undefined8 *in_R9;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  bool bVar15;
  bool bVar16;
  undefined8 *in_stack_00000028;
  
  puVar1 = (ulonglong *)*in_R8;
  lVar12 = in_R8[1];
  uVar10 = *puVar1;
  uVar5 = 0;
  uVar14 = uVar10 - *(ulonglong *)(lVar12 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar12 + 0x18)) {
    uVar14 = uVar5;
  }
  uVar9 = uVar10 - *(ulonglong *)(lVar12 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar12 + 0x38)) {
    uVar9 = uVar5;
  }
  bVar15 = *(ulonglong *)(lVar12 + 0x30) < uVar9;
  uVar9 = *(ulonglong *)(lVar12 + 0x30) - uVar9;
  uVar10 = uVar9;
  if (bVar15) {
    uVar10 = uVar5;
  }
  bVar16 = *(ulonglong *)(lVar12 + 0x10) < uVar14;
  uVar14 = *(ulonglong *)(lVar12 + 0x10) - uVar14;
  uVar6 = uVar14;
  if (bVar16) {
    uVar6 = uVar5;
  }
  uVar5 = *(ulonglong *)(lVar12 + 0x40);
  *puVar1 = (uVar5 - uVar10) - uVar6;
  if (!bVar16 && uVar14 != 0 || !bVar15 && uVar9 != 0) {
    *puVar1 = uVar5;
    decorator_on_wall_height_changed::closure_0();
  }
  puVar1 = (ulonglong *)*in_R9;
  lVar12 = in_R9[1];
  uVar10 = *puVar1;
  uVar6 = 0;
  uVar5 = uVar10 - *(ulonglong *)(lVar12 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar12 + 0x18)) {
    uVar5 = uVar6;
  }
  uVar4 = uVar10 - *(ulonglong *)(lVar12 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar12 + 0x38)) {
    uVar4 = uVar6;
  }
  uVar10 = *(ulonglong *)(lVar12 + 0x30);
  uVar8 = uVar10 - uVar4;
  if (uVar4 > uVar10) {
    uVar8 = uVar6;
  }
  uVar2 = *(ulonglong *)(lVar12 + 0x10);
  uVar11 = uVar2 - uVar5;
  if (uVar5 > uVar2) {
    uVar11 = uVar6;
  }
  uVar6 = *(ulonglong *)(lVar12 + 0x40);
  *puVar1 = (uVar6 - uVar8) - uVar11;
  if (uVar5 <= uVar2 && uVar2 - uVar5 != 0 || uVar4 <= uVar10 && uVar10 - uVar4 != 0) {
    *puVar1 = uVar6;
    decorator_on_wall_height_changed::closure_0();
  }
  else if ((bVar16 || uVar14 == 0) && (bVar15 || uVar9 == 0)) {
    puVar1 = (ulonglong *)*in_RDX;
    lVar12 = in_RDX[1];
    uVar10 = *puVar1;
    uVar5 = 0;
    uVar14 = uVar10 - *(ulonglong *)(lVar12 + 0x18);
    if (uVar10 < *(ulonglong *)(lVar12 + 0x18)) {
      uVar14 = uVar5;
    }
    uVar9 = uVar10 - *(ulonglong *)(lVar12 + 0x38);
    if (uVar10 < *(ulonglong *)(lVar12 + 0x38)) {
      uVar9 = uVar5;
    }
    lVar3 = uVar14 * 0x30 + *(longlong *)(lVar12 + 8);
    uVar10 = *(ulonglong *)(lVar12 + 0x10) - uVar14;
    if (*(ulonglong *)(lVar12 + 0x10) < uVar14) {
      lVar3 = 8;
      uVar10 = uVar5;
    }
    lVar7 = uVar9 * 0x30 + *(longlong *)(lVar12 + 0x28);
    uVar14 = *(ulonglong *)(lVar12 + 0x30) - uVar9;
    if (*(ulonglong *)(lVar12 + 0x30) < uVar9) {
      lVar7 = 8;
      uVar14 = uVar5;
    }
    *puVar1 = (*(longlong *)(lVar12 + 0x40) - uVar14) - uVar10;
    lVar12 = uVar10 * 0x30 + lVar3;
    lVar13 = uVar14 * 0x30 + lVar7;
    do {
      if ((lVar3 == 0) || (lVar3 == lVar12)) {
        if ((lVar7 == 0) || (lVar7 == lVar13)) break;
        lVar7 = lVar7 + 0x30;
        lVar3 = 0;
      }
      else {
        lVar3 = lVar3 + 0x30;
      }
      *puVar1 = *puVar1 + 1;
      decorator_on_wall_height_changed::closure_0();
    } while( true );
  }
  puVar1 = (ulonglong *)*in_stack_00000028;
  lVar12 = in_stack_00000028[1];
  uVar10 = *puVar1;
  uVar5 = 0;
  uVar14 = uVar10 - *(ulonglong *)(lVar12 + 0x18);
  if (uVar10 < *(ulonglong *)(lVar12 + 0x18)) {
    uVar14 = uVar5;
  }
  uVar9 = uVar10 - *(ulonglong *)(lVar12 + 0x38);
  if (uVar10 < *(ulonglong *)(lVar12 + 0x38)) {
    uVar9 = uVar5;
  }
  lVar3 = uVar14 * 0x10 + *(longlong *)(lVar12 + 8);
  uVar10 = *(ulonglong *)(lVar12 + 0x10) - uVar14;
  if (*(ulonglong *)(lVar12 + 0x10) < uVar14) {
    lVar3 = 8;
    uVar10 = uVar5;
  }
  lVar7 = uVar9 * 0x10 + *(longlong *)(lVar12 + 0x28);
  uVar14 = *(ulonglong *)(lVar12 + 0x30) - uVar9;
  if (*(ulonglong *)(lVar12 + 0x30) < uVar9) {
    lVar7 = 8;
    uVar14 = uVar5;
  }
  *puVar1 = (*(longlong *)(lVar12 + 0x40) - uVar14) - uVar10;
  lVar12 = uVar10 * 0x10 + lVar3;
  lVar13 = uVar14 * 0x10 + lVar7;
  do {
    if ((lVar3 == 0) || (lVar3 == lVar12)) {
      if ((lVar7 == 0) || (lVar7 == lVar13)) {
        return;
      }
      lVar7 = lVar7 + 0x10;
      lVar3 = 0;
    }
    else {
      lVar3 = lVar3 + 0x10;
    }
    *puVar1 = *puVar1 + 1;
    decorator_on_wall_height_changed::closure_0();
  } while( true );
}


