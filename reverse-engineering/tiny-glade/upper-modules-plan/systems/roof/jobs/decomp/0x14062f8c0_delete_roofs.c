// name=delete_roofs
// sig=void __cdecl delete_roofs(void)
// size=1006

void __cdecl system_roof::delete_roof::delete_roofs(void)

{
  uint *puVar1;
  float fVar2;
  uint uVar3;
  longlong lVar4;
  longlong lVar5;
  bool bVar6;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  longlong *in_RCX;
  longlong lVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong uVar13;
  bool bVar14;
  longlong lVar15;
  longlong lVar16;
  uint *puVar17;
  longlong local_b8;
  
  lVar4 = *in_RCX;
  lVar5 = in_RCX[1];
  puVar17 = *(uint **)(lVar4 + 0xf8);
  puVar1 = puVar17 + *(longlong *)(lVar4 + 0x100);
  if (*(char *)(lVar4 + 0x108) == '\0') {
    lVar12 = 8;
    lVar16 = 0;
    lVar15 = 0;
    bVar14 = false;
    local_b8 = 0;
    lVar10 = 0;
LAB_14062fad3:
    do {
      lVar9 = lVar10;
      if (lVar9 != lVar16) goto LAB_14062fb80;
      do {
        if (puVar17 == puVar1) {
          return;
        }
        uVar3 = *puVar17;
        puVar17 = puVar17 + 1;
        lVar16 = *(longlong *)(*(longlong *)(lVar5 + 0x100) + 0x58 + (ulonglong)uVar3 * 0xa0);
      } while (lVar16 == 0);
      lVar8 = *(longlong *)(lVar5 + 0x100) + (ulonglong)uVar3 * 0xa0;
      uVar11 = (ulonglong)*(uint *)(lVar8 + 0x94);
      lVar12 = *(longlong *)(lVar5 + 0x1a8);
      lVar10 = lVar12 + uVar11 * 0x48;
      lVar9 = *(longlong *)(lVar12 + 0x38 + uVar11 * 0x48);
      uVar11 = *(ulonglong *)(lVar12 + 0x40 + uVar11 * 0x48);
      uVar13 = *(ulonglong *)(lVar4 + 0x118);
      if (uVar13 < *(ulonglong *)(lVar8 + 0x88)) {
        if (*(longlong *)(*(longlong *)(lVar8 + 0x80) + uVar13 * 8) == 0) {
          uVar13 = *(ulonglong *)(lVar4 + 0x120);
          goto joined_r0x00014062fb9e;
        }
        bVar14 = true;
        if ((uVar13 < uVar11) && (uVar13 = *(ulonglong *)(lVar9 + uVar13 * 8), uVar13 != 0)) {
          lVar15 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar13 * 0x30);
          uVar13 = *(ulonglong *)(lVar4 + 0x120);
          goto joined_r0x00014062fb9e;
        }
        lVar15 = 0;
        uVar13 = *(ulonglong *)(lVar4 + 0x120);
        if (uVar13 < uVar11) goto LAB_14062fbc4;
LAB_14062fbe8:
        local_b8 = 0;
      }
      else {
        uVar13 = *(ulonglong *)(lVar4 + 0x120);
joined_r0x00014062fb9e:
        bVar14 = false;
joined_r0x00014062fb9e:
        if (uVar11 <= uVar13) goto LAB_14062fbe8;
LAB_14062fbc4:
        uVar11 = *(ulonglong *)(lVar9 + uVar13 * 8);
        if (uVar11 == 0) goto LAB_14062fbe8;
        local_b8 = *(longlong *)(*(longlong *)(lVar10 + 0x18) + 0x10 + ~uVar11 * 0x30);
      }
      lVar12 = *(longlong *)(lVar8 + 0x50);
      lVar9 = 0;
      lVar10 = 1;
    } while (!bVar14);
    goto LAB_14062fc05;
  }
  uVar13 = 0;
  lVar10 = 0;
  bVar14 = false;
  lVar16 = 0;
  uVar11 = 0;
LAB_14062f94b:
  do {
    uVar7 = uVar11;
    if (uVar7 != uVar13) goto LAB_14062f9c0;
    do {
      if (puVar17 == puVar1) {
        return;
      }
      uVar3 = *puVar17;
      puVar17 = puVar17 + 1;
      uVar13 = *(ulonglong *)(*(longlong *)(lVar5 + 0x1a8) + 0x10 + (ulonglong)uVar3 * 0x48);
    } while (uVar13 == 0);
    lVar16 = *(longlong *)(lVar5 + 0x1a8) + (ulonglong)uVar3 * 0x48;
    uVar11 = *(ulonglong *)(lVar16 + 0x40);
    if ((*(ulonglong *)(lVar4 + 0x118) < uVar11) &&
       (uVar7 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + *(ulonglong *)(lVar4 + 0x118) * 8),
       uVar7 != 0)) {
      lVar10 = *(longlong *)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar7 * 0x30);
      bVar14 = true;
      uVar7 = *(ulonglong *)(lVar4 + 0x120);
      bVar6 = bVar14;
      if (uVar11 <= uVar7) goto LAB_14062f9b9;
LAB_14062f9e0:
      uVar11 = *(ulonglong *)(*(longlong *)(lVar16 + 0x38) + uVar7 * 8);
      if (uVar11 == 0) {
        lVar16 = 0;
      }
      else {
        lVar16 = *(longlong *)(*(longlong *)(lVar16 + 0x18) + 0x10 + ~uVar11 * 0x30);
      }
    }
    else {
      bVar14 = false;
      uVar7 = *(ulonglong *)(lVar4 + 0x120);
      bVar6 = false;
      if (uVar7 < uVar11) goto LAB_14062f9e0;
LAB_14062f9b9:
      bVar14 = bVar6;
      lVar16 = 0;
    }
    uVar7 = 0;
    uVar11 = 1;
  } while (!bVar14);
  goto LAB_14062fa1f;
LAB_14062fb80:
  lVar10 = lVar9 + 1;
  if (bVar14) {
LAB_14062fc05:
    if (((lVar15 != 0) &&
        (fVar2 = *(float *)(local_b8 + (ulonglong)*(uint *)(lVar12 + lVar9 * 0x10 + 8) * 8),
        fVar2 == 0.0)) && (!NAN(fVar2))) {
      bevy_ecs::system::commands::EntityCommands::despawn();
    }
  }
  goto LAB_14062fad3;
LAB_14062f9c0:
  uVar11 = uVar7 + 1;
  if (bVar14) {
LAB_14062fa1f:
    if (((lVar10 != 0) && (fVar2 = *(float *)(lVar16 + (uVar7 & 0xffffffff) * 8), fVar2 == 0.0)) &&
       (!NAN(fVar2))) {
      bevy_ecs::system::commands::EntityCommands::despawn();
    }
  }
  goto LAB_14062f94b;
}

