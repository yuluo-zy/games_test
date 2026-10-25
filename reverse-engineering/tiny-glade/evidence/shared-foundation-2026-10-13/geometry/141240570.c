utils::geometry::aabb::Aabb3::intersect_segment

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_141240570(undefined4 *param_1,float *param_2,undefined8 *param_3,undefined8 *param_4)

{
  float fVar1;
  undefined4 uVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float fVar12;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  
  fVar1 = *(float *)(param_3 + 1);
  fVar6 = (float)((ulonglong)*param_3 >> 0x20);
  fVar5 = (float)*param_3;
  fVar11 = (float)*param_4 - fVar5;
  fVar12 = (float)((ulonglong)*param_4 >> 0x20) - fVar6;
  fVar13 = *(float *)(param_4 + 1) - fVar1;
  fVar7 = (param_2[4] - fVar6) / fVar12;
  fVar10 = (param_2[5] - fVar1) / fVar13;
  fVar3 = (param_2[1] - fVar6) / fVar12;
  fVar16 = (param_2[2] - fVar1) / fVar13;
  fVar17 = fVar16;
  if (fVar10 <= fVar16) {
    fVar17 = fVar10;
  }
  fVar14 = (param_2[3] - fVar5) / fVar11;
  fVar18 = (*param_2 - fVar5) / fVar11;
  fVar9 = fVar18;
  if (fVar14 <= fVar18) {
    fVar9 = fVar14;
  }
  fVar15 = fVar3;
  if (fVar7 <= fVar3) {
    fVar15 = fVar7;
  }
  fVar8 = fVar18;
  if (fVar18 <= fVar14) {
    fVar8 = fVar14;
  }
  fVar17 = (float)(~-(uint)NAN(fVar10) & (uint)fVar17 | -(uint)NAN(fVar10) & (uint)fVar16);
  fVar4 = fVar3;
  if (fVar3 <= fVar7) {
    fVar4 = fVar7;
  }
  fVar9 = (float)(~-(uint)NAN(fVar14) & (uint)fVar9 | (uint)fVar18 & -(uint)NAN(fVar14));
  fVar3 = (float)(~-(uint)NAN(fVar7) & (uint)fVar15 | -(uint)NAN(fVar7) & (uint)fVar3);
  if (fVar16 <= fVar10) {
    fVar16 = fVar10;
  }
  fVar7 = fVar17;
  if (fVar17 <= fVar3) {
    fVar7 = fVar3;
  }
  fVar17 = (float)(~-(uint)NAN(fVar3) & (uint)fVar7 | -(uint)NAN(fVar3) & (uint)fVar17);
  fVar3 = fVar17;
  if (fVar17 <= fVar9) {
    fVar3 = fVar9;
  }
  fVar3 = (float)(~-(uint)NAN(fVar9) & (uint)fVar3 | -(uint)NAN(fVar9) & (uint)fVar17);
  fVar17 = fVar16;
  if (fVar4 <= fVar16) {
    fVar17 = fVar4;
  }
  fVar17 = (float)(~-(uint)NAN(fVar4) & (uint)fVar17 | -(uint)NAN(fVar4) & (uint)fVar16);
  fVar16 = fVar17;
  if (fVar8 <= fVar17) {
    fVar16 = fVar8;
  }
  uVar2 = 0;
  if ((fVar3 <= (float)(~-(uint)NAN(fVar8) & (uint)fVar16 | -(uint)NAN(fVar8) & (uint)fVar17)) &&
     (fVar3 <= _DAT_142925904)) {
    if (0.0 <= fVar3) {
      *(ulonglong *)(param_1 + 1) = CONCAT44(fVar6 + fVar12 * fVar3,fVar5 + fVar11 * fVar3);
      param_1[3] = fVar1 + fVar13 * fVar3;
    }
    else {
      if ((((param_2[3] <= fVar5) || (param_2[4] <= fVar6)) || (param_2[5] <= fVar1)) ||
         (((fVar5 <= *param_2 || (fVar6 <= param_2[1])) || (fVar1 <= param_2[2]))))
      goto LAB_141240805;
      param_1[3] = *(undefined4 *)(param_3 + 1);
      *(undefined8 *)(param_1 + 1) = *param_3;
    }
    uVar2 = 1;
  }
LAB_141240805:
  *param_1 = uVar2;
  return;
}

