
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
  float fVar10;
  longlong lVar11;
  longlong lVar12;
  longlong *in_RCX;
  longlong lVar13;
  ulonglong uVar14;
  longlong *in_RDX;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong lVar17;
  uint *puVar18;
  float fVar19;
  float fVar20;
  longlong local_a0;
  longlong local_98;
  longlong local_88;
  longlong local_70;
  
  fVar10 = __real_3f800000;
  _Y = *(float *)(*in_RDX + 0x28);
  lVar7 = *in_RCX;
  lVar8 = in_RCX[1];
  uVar6 = *(undefined4 *)((longlong)in_RCX + 0x14);
  puVar18 = *(uint **)(lVar7 + 0xf8);
  puVar1 = puVar18 + *(longlong *)(lVar7 + 0x100);
  if (*(char *)(lVar7 + 0x108) == '\0') {
    local_88 = 8;
    local_98 = 0;
    local_a0 = 0;
    lVar11 = 0;
    lVar17 = 0;
    do {
      if (lVar17 == lVar11) {
        if (puVar18 == puVar1) {
          return;
        }
        while( true ) {
          lVar11 = *(longlong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)*puVar18 * 0xa0);
          if (lVar11 != 0) break;
          puVar18 = puVar18 + 1;
          if (puVar18 == puVar1) {
            return;
          }
        }
        lVar12 = *(longlong *)(lVar8 + 0x100) + (ulonglong)*puVar18 * 0xa0;
        uVar14 = (ulonglong)*(uint *)(lVar12 + 0x94);
        lVar13 = *(longlong *)(lVar8 + 0x1a8);
        lVar17 = lVar13 + uVar14 * 0x48;
        lVar9 = *(longlong *)(lVar13 + 0x38 + uVar14 * 0x48);
        if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar13 + 0x40 + uVar14 * 0x48)) {
          uVar14 = *(ulonglong *)(lVar9 + *(ulonglong *)(lVar7 + 0x110) * 8);
          lVar17 = *(longlong *)(lVar17 + 0x18);
          if (uVar14 == 0) goto LAB_142318dfd;
          local_a0 = *(longlong *)(lVar17 + 0x10 + ~uVar14 * 0x30);
        }
        else {
          lVar17 = *(longlong *)(lVar17 + 0x18);
LAB_142318dfd:
          local_a0 = 0;
        }
        puVar18 = puVar18 + 1;
        lVar13 = ~*(ulonglong *)(lVar9 + *(longlong *)(lVar7 + 0x118) * 8) * 0x30;
        local_98 = *(longlong *)(lVar17 + 0x10 + lVar13);
        local_70 = *(longlong *)(lVar17 + 0x28 + lVar13);
        local_88 = *(longlong *)(lVar12 + 0x50);
        lVar17 = 0;
      }
      uVar14 = (ulonglong)*(uint *)(local_88 + 8 + lVar17 * 0x10);
      lVar13 = uVar14 * 0x10;
      lVar17 = lVar17 + 1;
      fVar19 = powf(*(float *)(local_a0 + 0xc + lVar13),_Y);
      fVar20 = fVar10 - fVar19;
      fVar2 = *(float *)(local_a0 + 8 + lVar13);
      fVar3 = *(float *)(local_98 + 8 + uVar14 * 0xc);
      uVar4 = *(undefined8 *)(local_a0 + lVar13);
      uVar5 = *(undefined8 *)(local_98 + uVar14 * 0xc);
      *(undefined4 *)(local_70 + uVar14 * 4) = uVar6;
      *(ulonglong *)(local_98 + uVar14 * 0xc) =
           CONCAT44((float)((ulonglong)uVar5 >> 0x20) * fVar19 +
                    (float)((ulonglong)uVar4 >> 0x20) * fVar20,
                    (float)uVar5 * fVar19 + (float)uVar4 * fVar20);
      *(float *)(local_98 + 8 + uVar14 * 0xc) = fVar19 * fVar3 + fVar20 * fVar2;
    } while( true );
  }
  local_98 = 0;
  local_a0 = 0;
  uVar14 = 0;
  uVar16 = 0;
  do {
    if (uVar16 == uVar14) {
      if (puVar18 == puVar1) {
        return;
      }
      while( true ) {
        uVar14 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)*puVar18 * 0x48);
        if (uVar14 != 0) break;
        puVar18 = puVar18 + 1;
        if (puVar18 == puVar1) {
          return;
        }
      }
      lVar11 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*puVar18 * 0x48;
      if (*(ulonglong *)(lVar7 + 0x110) < *(ulonglong *)(lVar11 + 0x40)) {
        uVar16 = *(ulonglong *)(*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(lVar7 + 0x110) * 8);
        lVar17 = *(longlong *)(lVar11 + 0x18);
        if (uVar16 == 0) goto LAB_142318cbe;
        local_a0 = *(longlong *)(lVar17 + 0x10 + ~uVar16 * 0x30);
      }
      else {
        lVar17 = *(longlong *)(lVar11 + 0x18);
LAB_142318cbe:
        local_a0 = 0;
      }
      puVar18 = puVar18 + 1;
      lVar11 = ~*(ulonglong *)(*(longlong *)(lVar11 + 0x38) + *(longlong *)(lVar7 + 0x118) * 8) *
               0x30;
      local_98 = *(longlong *)(lVar17 + 0x10 + lVar11);
      local_88 = *(longlong *)(lVar17 + 0x28 + lVar11);
      uVar16 = 0;
    }
    uVar15 = uVar16 & 0xffffffff;
    lVar11 = uVar15 * 0x10;
    uVar16 = uVar16 + 1;
    fVar19 = powf(*(float *)(local_a0 + 0xc + lVar11),_Y);
    fVar20 = fVar10 - fVar19;
    fVar2 = *(float *)(local_a0 + 8 + lVar11);
    fVar3 = *(float *)(local_98 + 8 + uVar15 * 0xc);
    uVar4 = *(undefined8 *)(local_a0 + lVar11);
    uVar5 = *(undefined8 *)(local_98 + uVar15 * 0xc);
    *(undefined4 *)(local_88 + uVar15 * 4) = uVar6;
    *(ulonglong *)(local_98 + uVar15 * 0xc) =
         CONCAT44((float)((ulonglong)uVar5 >> 0x20) * fVar19 +
                  (float)((ulonglong)uVar4 >> 0x20) * fVar20,
                  (float)uVar5 * fVar19 + (float)uVar4 * fVar20);
    *(float *)(local_98 + 8 + uVar15 * 0xc) = fVar19 * fVar3 + fVar20 * fVar2;
  } while( true );
}

