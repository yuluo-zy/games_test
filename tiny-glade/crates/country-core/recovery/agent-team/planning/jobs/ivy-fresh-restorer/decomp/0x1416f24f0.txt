
void __cdecl country_core::systems::debug::sheep_controls::apply_sheep_snapshot(void)

{
  uint *puVar1;
  undefined8 *puVar2;
  char cVar3;
  undefined4 uVar4;
  uint uVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  longlong *in_RCX;
  longlong in_RDX;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  undefined8 *puVar15;
  ulonglong uVar16;
  uint *puVar17;
  longlong lVar18;
  ulonglong uVar19;
  undefined8 *puVar20;
  longlong local_80;
  longlong local_78;
  longlong local_70;
  longlong local_68;
  longlong local_60;
  longlong local_50;
  
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  uVar4 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar17 = *(uint **)(lVar7 + 0xf8);
  puVar1 = puVar17 + *(longlong *)(lVar7 + 0x100);
  cVar3 = *(char *)(lVar7 + 0x108);
  puVar15 = *(undefined8 **)(in_RDX + 8);
  puVar2 = puVar15 + *(longlong *)(in_RDX + 0x10) * 5;
  local_50 = 8;
  local_80 = 0;
  local_78 = 0;
  lVar14 = 0;
  uVar16 = 0;
  uVar11 = 0;
  if (cVar3 == '\0') goto LAB_1416f2640;
  do {
    if (uVar11 == uVar16) {
      do {
        if (puVar17 == puVar1) {
          return;
        }
        uVar5 = *puVar17;
        puVar17 = puVar17 + 1;
        uVar16 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar5 * 0x48);
      } while (uVar16 == 0);
      lVar14 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar5 * 0x48;
      lVar18 = *(longlong *)(lVar14 + 0x18);
      lVar14 = *(longlong *)(lVar14 + 0x38);
      lVar12 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
      local_78 = *(longlong *)(lVar18 + 0x10 + lVar12);
      local_60 = *(longlong *)(lVar18 + 0x28 + lVar12);
      lVar12 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
      local_80 = *(longlong *)(lVar18 + 0x10 + lVar12);
      local_68 = *(longlong *)(lVar18 + 0x28 + lVar12);
      lVar12 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x120) * 8) * 0x30;
      lVar14 = *(longlong *)(lVar18 + 0x10 + lVar12);
      local_70 = *(longlong *)(lVar18 + 0x28 + lVar12);
      uVar11 = 0;
    }
    uVar19 = uVar11 & 0xffffffff;
    while( true ) {
      lVar12 = uVar19 * 0x30 + local_78;
      lVar18 = uVar19 * 0x68 + local_80;
      puVar20 = (undefined8 *)(uVar19 * 0x10 + lVar14);
      if (puVar15 == puVar2) {
        return;
      }
      uVar11 = uVar11 + 1;
      uVar6 = *(undefined4 *)((longlong)puVar15 + 0x24);
      uVar9 = *(undefined8 *)((longlong)puVar15 + 0x1c);
      *(undefined4 *)(local_60 + uVar19 * 4) = uVar4;
      *(undefined4 *)(lVar12 + 0x18) = uVar6;
      *(undefined8 *)(lVar12 + 0x10) = uVar9;
      uVar9 = *puVar15;
      uVar10 = puVar15[1];
      *(undefined4 *)(local_70 + uVar19 * 4) = uVar4;
      *puVar20 = uVar9;
      puVar20[1] = uVar10;
      uVar6 = *(undefined4 *)(puVar15 + 3);
      uVar9 = puVar15[2];
      puVar15 = puVar15 + 5;
      *(undefined4 *)(local_68 + uVar19 * 4) = uVar4;
      *(undefined4 *)(lVar18 + 0x38) = uVar6;
      *(undefined8 *)(lVar18 + 0x30) = uVar9;
      if (cVar3 != '\0') break;
LAB_1416f2640:
      if (uVar11 == uVar16) {
        do {
          if (puVar17 == puVar1) {
            return;
          }
          uVar5 = *puVar17;
          puVar17 = puVar17 + 1;
          uVar16 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar5 * 0xa0);
        } while (uVar16 == 0);
        lVar12 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar5 * 0xa0;
        uVar11 = (ulonglong)*(uint *)(lVar12 + 0x94);
        lVar18 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x18 + uVar11 * 0x48);
        lVar14 = *(longlong *)(*(longlong *)(lVar8 + 0x1a8) + 0x38 + uVar11 * 0x48);
        lVar13 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x110) * 8) * 0x30;
        local_78 = *(longlong *)(lVar18 + 0x10 + lVar13);
        local_60 = *(longlong *)(lVar18 + 0x28 + lVar13);
        lVar13 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_80 = *(longlong *)(lVar18 + 0x10 + lVar13);
        local_68 = *(longlong *)(lVar18 + 0x28 + lVar13);
        lVar13 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar7 + 0x120) * 8) * 0x30;
        lVar14 = *(longlong *)(lVar18 + 0x10 + lVar13);
        local_70 = *(longlong *)(lVar18 + 0x28 + lVar13);
        local_50 = *(longlong *)(lVar12 + 0x50);
        uVar11 = 0;
      }
      uVar19 = (ulonglong)*(uint *)(local_50 + 8 + uVar11 * 0x10);
    }
  } while( true );
}

