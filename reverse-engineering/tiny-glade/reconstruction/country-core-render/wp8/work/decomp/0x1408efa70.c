
void __cdecl country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast::closure_0(void)

{
  uint *puVar1;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  byte bVar6;
  undefined4 uVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 *puVar10;
  longlong *in_RCX;
  undefined8 *in_RDX;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  byte *in_R8;
  uint *puVar17;
  longlong unaff_R12;
  ulonglong uVar18;
  longlong lVar19;
  longlong unaff_R15;
  longlong lVar20;
  
  lVar8 = *in_RCX;
  lVar9 = in_RCX[1];
  uVar7 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar17 = *(uint **)(lVar8 + 0xf8);
  puVar1 = puVar17 + *(longlong *)(lVar8 + 0x100);
  bVar6 = *in_R8;
  if (*(char *)(lVar8 + 0x108) == '\0') {
    lVar11 = 8;
    lVar19 = 0;
    lVar16 = 0;
    lVar15 = 0;
    lVar13 = 0;
    do {
      if (lVar13 == lVar15) {
        if (puVar17 == puVar1) {
          return;
        }
        while( true ) {
          lVar15 = *(longlong *)(*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)*puVar17 * 0xa0);
          if (lVar15 != 0) break;
          puVar17 = puVar17 + 1;
          if (puVar17 == puVar1) {
            return;
          }
        }
        lVar20 = *(longlong *)(lVar9 + 0x100) + (ulonglong)*puVar17 * 0xa0;
        uVar14 = (ulonglong)*(uint *)(lVar20 + 0x94);
        lVar11 = *(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x18 + uVar14 * 0x48);
        lVar19 = *(longlong *)(*(longlong *)(lVar9 + 0x1a8) + 0x38 + uVar14 * 0x48);
        lVar16 = *(longlong *)
                  (lVar11 + 0x10 + ~*(ulonglong *)(lVar19 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30
                  );
        lVar13 = ~*(ulonglong *)(lVar19 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
        lVar19 = *(longlong *)(lVar11 + 0x10 + lVar13);
        unaff_R12 = *(longlong *)(lVar11 + 0x28 + lVar13);
        lVar11 = *(longlong *)(lVar20 + 0x50);
        puVar17 = puVar17 + 1;
        lVar13 = 0;
      }
      uVar14 = (ulonglong)*(uint *)(lVar11 + 8 + lVar13 * 0x10);
      lVar20 = uVar14 * 0x30 + lVar19;
      lVar13 = lVar13 + 1;
      if (*(char *)(lVar16 + uVar14) == '\0') {
        if ((bVar6 & 1) != 0) {
          *(undefined4 *)(unaff_R12 + uVar14 * 4) = uVar7;
          *(undefined4 *)(lVar20 + 0x18) = *(undefined4 *)(in_R8 + 0xc);
          *(undefined8 *)(lVar20 + 0x10) = *(undefined8 *)(in_R8 + 4);
        }
      }
      else {
        puVar10 = (undefined8 *)*in_RDX;
        fVar2 = *(float *)((longlong)puVar10 + 0x14);
        fVar3 = *(float *)(puVar10 + 1);
        uVar4 = *puVar10;
        uVar5 = *(undefined8 *)((longlong)puVar10 + 0xc);
        *(undefined4 *)(unaff_R12 + uVar14 * 4) = uVar7;
        *(ulonglong *)(lVar20 + 0x10) =
             CONCAT44((float)((ulonglong)uVar5 >> 0x20) * 45.0 + (float)((ulonglong)uVar4 >> 0x20),
                      (float)uVar5 * 45.0 + (float)uVar4);
        *(float *)(lVar20 + 0x18) = fVar2 * 45.0 + fVar3;
      }
    } while( true );
  }
  lVar16 = 0;
  lVar15 = 0;
  uVar14 = 0;
  uVar18 = 0;
  do {
    if (uVar18 == uVar14) {
      if (puVar17 == puVar1) {
        return;
      }
      while( true ) {
        uVar14 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)*puVar17 * 0x48);
        if (uVar14 != 0) break;
        puVar17 = puVar17 + 1;
        if (puVar17 == puVar1) {
          return;
        }
      }
      lVar15 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)*puVar17 * 0x48;
      lVar19 = *(longlong *)(lVar15 + 0x18);
      lVar16 = *(longlong *)(lVar15 + 0x38);
      lVar15 = *(longlong *)
                (lVar19 + 0x10 + ~*(ulonglong *)(lVar16 + *(longlong *)(lVar8 + 0x110) * 8) * 0x30);
      lVar11 = ~*(ulonglong *)(lVar16 + *(longlong *)(lVar8 + 0x118) * 8) * 0x30;
      lVar16 = *(longlong *)(lVar19 + 0x10 + lVar11);
      unaff_R15 = *(longlong *)(lVar19 + 0x28 + lVar11);
      puVar17 = puVar17 + 1;
      uVar18 = 0;
    }
    uVar12 = uVar18 & 0xffffffff;
    lVar19 = uVar12 * 0x30 + lVar16;
    uVar18 = uVar18 + 1;
    if (*(char *)(lVar15 + uVar12) == '\0') {
      if ((bVar6 & 1) != 0) {
        *(undefined4 *)(unaff_R15 + uVar12 * 4) = uVar7;
        *(undefined4 *)(lVar19 + 0x18) = *(undefined4 *)(in_R8 + 0xc);
        *(undefined8 *)(lVar19 + 0x10) = *(undefined8 *)(in_R8 + 4);
      }
    }
    else {
      puVar10 = (undefined8 *)*in_RDX;
      fVar2 = *(float *)((longlong)puVar10 + 0x14);
      fVar3 = *(float *)(puVar10 + 1);
      uVar4 = *puVar10;
      uVar5 = *(undefined8 *)((longlong)puVar10 + 0xc);
      *(undefined4 *)(unaff_R15 + uVar12 * 4) = uVar7;
      *(ulonglong *)(lVar19 + 0x10) =
           CONCAT44((float)((ulonglong)uVar5 >> 0x20) * 45.0 + (float)((ulonglong)uVar4 >> 0x20),
                    (float)uVar5 * 45.0 + (float)uVar4);
      *(float *)(lVar19 + 0x18) = fVar2 * 45.0 + fVar3;
    }
  } while( true );
}


