// 0x142318c10 smooth_critter_velocities
// SIG: void __cdecl smooth_critter_velocities(void)

void __cdecl system_critters::shared::smooth_critter_velocities(void)

{
  uint *puVar1;
  float _Y;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined4 uVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong lVar10;
  longlong lVar11;
  longlong *in_RCX;
  longlong lVar12;
  ulonglong uVar13;
  longlong *in_RDX;
  ulonglong uVar14;
  ulonglong uVar15;
  longlong lVar16;
  uint *puVar17;
  float fVar18;
  float fVar19;
  longlong local_a0;
  longlong local_98;
  longlong local_88;
  longlong local_70;
  
  _Y = *(float *)(*in_RDX + 0x28);
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar17 = *(uint **)(lVar7 + 0xf8);
  puVar1 = puVar17 + *(longlong *)(lVar7 + 0x100);
  if (*(char *)(lVar7 + 0x108) == '\0') {
    local_88 = 8;
    local_98 = 0;
    local_a0 = 0;
    lVar10 = 0;
    lVar16 = 0;
    do {
      if (lVar16 == lVar10) {
        if (puVar17 == puVar1) {
          return;
        }
        while( true ) {
          lVar10 = *(longlong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar17 * 0xa0);
          if (lVar10 != 0) break;
          puVar17 = puVar17 + 1;
          if (puVar17 == puVar1) {
            return;
          }
        }
        lVar11 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar17 * 0xa0;
        uVar13 = (ulonglong)*(uint *)(lVar11 + 0x94);
        lVar12 = *(longlong *)(lVar8 + 0x1a8);
        lVar16 = lVar12 + uVar13 * 0x48;
        lVar9 = *(longlong *)(lVar12 + 0x38 + uVar13 * 0x48);
        if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar12 + 0x40 + uVar13 * 0x48)) {
          uVar13 = *(ulonglong *)(lVar9 + *(ulonglong *)(lVar7 + 0x110) * 8);
          lVar16 = *(longlong *)(lVar16 + 0x18);
          if (uVar13 == 0) goto LAB_142318dfd;
          local_a0 = *(longlong *)(lVar16 + 0x10 + ~uVar13 * 0x30);
        }
        else {
          lVar16 = *(longlong *)(lVar16 + 0x18);
LAB_142318dfd:
          local_a0 = 0;
        }
        puVar17 = puVar17 + 1;
        lVar12 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_98 = *(longlong *)(lVar16 + 0x10 + lVar12);
        local_70 = *(longlong *)(lVar16 + 0x28 + lVar12);
        local_88 = *(longlong *)(lVar11 + 0x50);
        lVar16 = 0;
      }
      uVar13 = (ulonglong)*(uint *)(local_88 + 8 + lVar16 * 0x10);
      lVar12 = uVar13 * 0x10;
      lVar16 = lVar16 + 1;
      fVar18 = powf(*(float *)(local_a0 + 0xc + lVar12),_Y);
      fVar19 = 1.0 - fVar18;
      fVar2 = *(float *)(local_a0 + 8 + lVar12);
      fVar3 = *(float *)(local_98 + 8 + uVar13 * 0xc);
      uVar4 = *(undefined8 *)(local_a0 + lVar12);
      uVar5 = *(undefined8 *)(local_98 + uVar13 * 0xc);
      *(undefined4 *)(local_70 + uVar13 * 4) = uVar6;
      *(ulonglong *)(local_98 + uVar13 * 0xc) =
           CONCAT44((float)((ulonglong)uVar5 >> 0x20) * fVar18 +
                    (float)((ulonglong)uVar4 >> 0x20) * fVar19,
                    (float)uVar5 * fVar18 + (float)uVar4 * fVar19);
      *(float *)(local_98 + 8 + uVar13 * 0xc) = fVar18 * fVar3 + fVar19 * fVar2;
    } while( true );
  }
  local_98 = 0;
  local_a0 = 0;
  uVar13 = 0;
  uVar15 = 0;
  do {
    if (uVar15 == uVar13) {
      if (puVar17 == puVar1) {
        return;
      }
      while( true ) {
        uVar13 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar17 * 0x48);
        if (uVar13 != 0) break;
        puVar17 = puVar17 + 1;
        if (puVar17 == puVar1) {
          return;
        }
      }
      lVar10 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar17 * 0x48;
      if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar10 + 0x40)) {
        uVar15 = *(ulonglong *)(*(longlong *)(lVar10 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
        lVar16 = *(longlong *)(lVar10 + 0x18);
        if (uVar15 == 0) goto LAB_142318cbe;
        local_a0 = *(longlong *)(lVar16 + 0x10 + ~uVar15 * 0x30);
      }
      else {
        lVar16 = *(longlong *)(lVar10 + 0x18);
LAB_142318cbe:
        local_a0 = 0;
      }
      puVar17 = puVar17 + 1;
      lVar10 = ~*(ulonglong *)(*(longlong *)(lVar10 + 0x38) + *(longlong *)(lVar7 + 0x118) * 8) *
               0x30;
      local_98 = *(longlong *)(lVar16 + 0x10 + lVar10);
      local_88 = *(longlong *)(lVar16 + 0x28 + lVar10);
      uVar15 = 0;
    }
    uVar14 = uVar15 & 0xffffffff;
    lVar10 = uVar14 * 0x10;
    uVar15 = uVar15 + 1;
    fVar18 = powf(*(float *)(local_a0 + 0xc + lVar10),_Y);
    fVar19 = 1.0 - fVar18;
    fVar2 = *(float *)(local_a0 + 8 + lVar10);
    fVar3 = *(float *)(local_98 + 8 + uVar14 * 0xc);
    uVar4 = *(undefined8 *)(local_a0 + lVar10);
    uVar5 = *(undefined8 *)(local_98 + uVar14 * 0xc);
    *(undefined4 *)(local_88 + uVar14 * 4) = uVar6;
    *(ulonglong *)(local_98 + uVar14 * 0xc) =
         CONCAT44((float)((ulonglong)uVar5 >> 0x20) * fVar18 +
                  (float)((ulonglong)uVar4 >> 0x20) * fVar19,
                  (float)uVar5 * fVar18 + (float)uVar4 * fVar19);
    *(float *)(local_98 + 8 + uVar14 * 0xc) = fVar18 * fVar3 + fVar19 * fVar2;
  } while( true );
}

