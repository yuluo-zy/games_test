
void __cdecl country_core::systems::history::signifier_replay_active::signifier_replay_active(void)

{
  undefined4 *puVar1;
  uint *puVar2;
  char cVar3;
  undefined4 uVar4;
  int *piVar5;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  byte *pbVar9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong *in_RCX;
  undefined8 *in_RDX;
  longlong lVar13;
  longlong *in_R8;
  longlong lVar14;
  ulonglong uVar15;
  uint *puVar16;
  float *pfVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 local_170;
  float local_168;
  longlong local_160;
  ulonglong local_158;
  longlong local_128;
  longlong local_120;
  
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  piVar5 = (int *)*in_RDX;
  if (*piVar5 != 1) {
    return;
  }
  fVar29 = *(float *)(*in_stack_00000028 + 0x28) + (float)piVar5[1];
  piVar5[1] = (int)fVar29;
  fVar18 = 0.9 / *(float *)(*in_stack_00000038 + 0x28);
  if (fVar18 <= 0.5) {
    fVar18 = 0.5;
  }
  fVar29 = fVar29 / fVar18;
  fVar18 = (float)truncf(fVar29);
  fVar29 = fVar29 - fVar18;
  lVar6 = *in_R8;
  lVar7 = in_R8[1];
  uVar4 = *(undefined4 *)((longlong)in_R8 + 0x14);
  puVar16 = *(uint **)(lVar6 + 0xf8);
  puVar2 = puVar16 + *(longlong *)(lVar6 + 0x100);
  cVar3 = *(char *)(lVar6 + 0x108);
  lVar8 = *in_RCX;
  pbVar9 = (byte *)*in_stack_00000030;
  local_128 = 8;
  lVar13 = 0;
  local_160 = 0;
  uVar15 = 0;
  local_158 = 0;
  do {
    fVar18 = sinf(fVar29 * 7.853982);
    fVar19 = (float)exp2f(fVar29 * -10.0);
    fVar18 = fVar19 * fVar18 * 0.5;
    while( true ) {
      while( true ) {
        if (cVar3 == '\0') {
          if (local_158 == uVar15) {
            if (puVar16 == puVar2) {
              return;
            }
            while( true ) {
              uVar15 = *(ulonglong *)
                        (*(longlong *)(lVar7 + 0x100) + 0x58 + (ulonglong)*puVar16 * 0xa0);
              if (uVar15 != 0) break;
              puVar16 = puVar16 + 1;
              if (puVar16 == puVar2) {
                return;
              }
            }
            lVar11 = *(longlong *)(lVar7 + 0x100) + (ulonglong)*puVar16 * 0xa0;
            uVar12 = (ulonglong)*(uint *)(lVar11 + 0x94);
            lVar13 = *(longlong *)(lVar7 + 0x1a8);
            lVar10 = *(longlong *)(lVar13 + 0x18 + uVar12 * 0x48);
            lVar14 = *(longlong *)(lVar13 + 0x38 + uVar12 * 0x48);
            if ((*(ulonglong *)(lVar6 + 0x118) < *(ulonglong *)(lVar13 + 0x40 + uVar12 * 0x48)) &&
               (uVar12 = *(ulonglong *)(lVar14 + *(ulonglong *)(lVar6 + 0x118) * 8), uVar12 != 0)) {
              lVar13 = *(longlong *)(lVar10 + 0x10 + ~uVar12 * 0x30);
            }
            else {
              lVar13 = 0;
            }
            lVar14 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
            local_160 = *(longlong *)(lVar10 + 0x10 + lVar14);
            local_120 = *(longlong *)(lVar10 + 0x28 + lVar14);
            puVar16 = puVar16 + 1;
            local_128 = *(longlong *)(lVar11 + 0x50);
            local_158 = 0;
          }
          uVar12 = (ulonglong)*(uint *)(local_128 + 8 + local_158 * 0x10);
        }
        else {
          if (local_158 == uVar15) {
            if (puVar16 == puVar2) {
              return;
            }
            while( true ) {
              uVar15 = *(ulonglong *)
                        (*(longlong *)(lVar7 + 0x1a8) + 0x10 + (ulonglong)*puVar16 * 0x48);
              if (uVar15 != 0) break;
              puVar16 = puVar16 + 1;
              if (puVar16 == puVar2) {
                return;
              }
            }
            lVar13 = *(longlong *)(lVar7 + 0x1a8) + (ulonglong)*puVar16 * 0x48;
            lVar10 = *(longlong *)(lVar13 + 0x18);
            lVar14 = ~*(ulonglong *)
                       (*(longlong *)(lVar13 + 0x38) + *(longlong *)(lVar6 + 0x110) * 8) * 0x30;
            local_160 = *(longlong *)(lVar10 + 0x10 + lVar14);
            local_120 = *(longlong *)(lVar10 + 0x28 + lVar14);
            puVar16 = puVar16 + 1;
            if ((*(ulonglong *)(lVar6 + 0x118) < *(ulonglong *)(lVar13 + 0x40)) &&
               (uVar12 = *(ulonglong *)
                          (*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar6 + 0x118) * 8),
               uVar12 != 0)) {
              lVar13 = *(longlong *)(lVar10 + 0x10 + ~uVar12 * 0x30);
            }
            else {
              lVar13 = 0;
            }
            local_158 = 0;
          }
          uVar12 = local_158 & 0xffffffff;
        }
        pfVar17 = (float *)(uVar12 * 0x30 + local_160);
        puVar1 = (undefined4 *)(local_120 + uVar12 * 4);
        local_158 = local_158 + 1;
        if (*(longlong *)(lVar8 + 0x18) != 0) break;
        if ((*pbVar9 & 1) == 0) {
          local_168 = 0.0;
          local_170 = 0;
        }
        else {
          local_168 = *(float *)(pbVar9 + 0xc);
          local_170 = *(undefined8 *)(pbVar9 + 4);
        }
        *puVar1 = uVar4;
        pfVar17[6] = local_168;
        *(undefined8 *)(pfVar17 + 4) = local_170;
      }
      *puVar1 = uVar4;
      ::utils::camera::CameraMatrices::eye_position();
      ::utils::components::transform::Transform::look_at();
      if (*(longlong *)(uVar12 * 8 + lVar13) != 1) break;
      if ((*pbVar9 & 1) == 0) {
        fVar20 = 0.0;
        fVar21 = 0.0;
        fVar19 = 0.0;
      }
      else {
        fVar20 = (float)*(undefined8 *)(pbVar9 + 4);
        fVar21 = (float)((ulonglong)*(undefined8 *)(pbVar9 + 4) >> 0x20);
        fVar19 = *(float *)(pbVar9 + 0xc);
      }
      fVar24 = *pfVar17;
      fVar30 = pfVar17[1];
      fVar31 = pfVar17[2];
      fVar25 = pfVar17[3];
      fVar27 = fVar25 * fVar25;
      fVar28 = fVar31 * fVar31 + fVar30 * fVar30 + fVar24 * fVar24;
      fVar26 = fVar31 * 0.0 + fVar30 * 0.0 + fVar24 * 1.0;
      fVar26 = fVar26 + fVar26;
      fVar25 = fVar25 + fVar25;
      fVar22 = fVar25 * (fVar30 * 0.0 - fVar31 * 0.0) + fVar26 * fVar24 + (fVar27 - fVar28) * 1.0;
      fVar23 = fVar25 * (fVar31 * 1.0 - fVar24 * 0.0) + fVar26 * fVar30 + (fVar27 - fVar28) * 0.0;
      fVar24 = fVar25 * (fVar24 * 0.0 - fVar30 * 1.0) + fVar26 * fVar31 + (fVar27 - fVar28) * 0.0;
      *puVar1 = uVar4;
      *(ulonglong *)(pfVar17 + 4) =
           CONCAT44(fVar23 * -0.15 * (float)0x40333333 +
                    (fVar21 - fVar18 * fVar23 * (float)0x40333333),
                    fVar22 * -0.15 * (float)0x40333333 +
                    (fVar20 - fVar18 * fVar22 * (float)0x40333333));
      pfVar17[6] = fVar24 * -0.15 * 2.8 + (fVar19 - fVar24 * fVar18 * 2.8);
    }
    fVar18 = 0.0;
    fVar19 = 0.0;
    if (0.0 <= fVar29 + -0.05) {
      fVar19 = fVar29 + -0.05;
    }
    fVar29 = 1.0;
    if (fVar19 <= 1.0) {
      fVar29 = fVar19;
    }
    fVar19 = 0.0;
    fVar20 = 0.0;
    if ((*pbVar9 & 1) != 0) {
      fVar19 = (float)*(undefined8 *)(pbVar9 + 4);
      fVar20 = (float)((ulonglong)*(undefined8 *)(pbVar9 + 4) >> 0x20);
      fVar18 = *(float *)(pbVar9 + 0xc);
    }
    fVar21 = *pfVar17;
    fVar24 = pfVar17[1];
    fVar30 = pfVar17[2];
    fVar31 = pfVar17[3];
    fVar22 = fVar31 * fVar31;
    fVar23 = fVar30 * fVar30 + fVar24 * fVar24 + fVar21 * fVar21;
    fVar25 = fVar30 * 0.0 + fVar24 * 0.0 + fVar21 * 1.0;
    fVar25 = fVar25 + fVar25;
    fVar31 = fVar31 + fVar31;
    fVar26 = fVar31 * (fVar24 * 0.0 - fVar30 * 0.0) + fVar25 * fVar21 + (fVar22 - fVar23) * 1.0;
    fVar27 = fVar31 * (fVar30 * 1.0 - fVar21 * 0.0) + fVar25 * fVar24 + (fVar22 - fVar23) * 0.0;
    fVar30 = fVar31 * (fVar21 * 0.0 - fVar24 * 1.0) + fVar25 * fVar30 + (fVar22 - fVar23) * 0.0;
    fVar21 = sinf(fVar29 * 7.853982);
    fVar24 = (float)exp2f(fVar29 * -10.0);
    fVar21 = fVar24 * fVar21 * 0.5;
    *puVar1 = uVar4;
    *(ulonglong *)(pfVar17 + 4) =
         CONCAT44(fVar27 * 0.15 * (float)0x40333333 + (fVar20 - fVar21 * fVar27 * (float)0x40333333)
                  ,fVar26 * 0.15 * (float)0x40333333 +
                   (fVar19 - fVar21 * fVar26 * (float)0x40333333));
    pfVar17[6] = fVar30 * 0.15 * 2.8 + (fVar18 - fVar30 * fVar21 * 2.8);
  } while( true );
}


