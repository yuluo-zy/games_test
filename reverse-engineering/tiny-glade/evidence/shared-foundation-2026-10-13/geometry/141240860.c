utils::geometry::aabb::Aabb3::classify_ray_intersection

void FUN_141240860(undefined4 *param_1,float *param_2,float *param_3)

{
  float fVar1;
  float fVar2;
  float fVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
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
  
  fVar1 = *param_3;
  fVar2 = param_3[1];
  fVar3 = param_3[2];
  fVar9 = (param_2[3] - fVar1) / param_3[3];
  fVar8 = (param_2[4] - fVar2) / param_3[4];
  fVar7 = (param_2[5] - fVar3) / param_3[5];
  fVar17 = (*param_2 - fVar1) / param_3[3];
  fVar11 = (param_2[1] - fVar2) / param_3[4];
  fVar16 = (param_2[2] - fVar3) / param_3[5];
  fVar14 = fVar16;
  if (fVar7 <= fVar16) {
    fVar14 = fVar7;
  }
  fVar10 = fVar17;
  if (fVar9 <= fVar17) {
    fVar10 = fVar9;
  }
  fVar15 = fVar11;
  if (fVar8 <= fVar11) {
    fVar15 = fVar8;
  }
  fVar13 = fVar17;
  if (fVar17 <= fVar9) {
    fVar13 = fVar9;
  }
  fVar14 = (float)(~-(uint)NAN(fVar7) & (uint)fVar14 | -(uint)NAN(fVar7) & (uint)fVar16);
  fVar12 = fVar11;
  if (fVar11 <= fVar8) {
    fVar12 = fVar8;
  }
  fVar9 = (float)(~-(uint)NAN(fVar9) & (uint)fVar10 | (uint)fVar17 & -(uint)NAN(fVar9));
  fVar8 = (float)(~-(uint)NAN(fVar8) & (uint)fVar15 | -(uint)NAN(fVar8) & (uint)fVar11);
  if (fVar16 <= fVar7) {
    fVar16 = fVar7;
  }
  fVar7 = fVar14;
  if (fVar14 <= fVar8) {
    fVar7 = fVar8;
  }
  fVar14 = (float)(~-(uint)NAN(fVar8) & (uint)fVar7 | -(uint)NAN(fVar8) & (uint)fVar14);
  fVar7 = fVar14;
  if (fVar14 <= fVar9) {
    fVar7 = fVar9;
  }
  fVar7 = (float)(~-(uint)NAN(fVar9) & (uint)fVar7 | -(uint)NAN(fVar9) & (uint)fVar14);
  fVar14 = fVar16;
  if (fVar12 <= fVar16) {
    fVar14 = fVar12;
  }
  fVar14 = (float)(~-(uint)NAN(fVar12) & (uint)fVar14 | -(uint)NAN(fVar12) & (uint)fVar16);
  fVar16 = fVar14;
  if (fVar13 <= fVar14) {
    fVar16 = fVar13;
  }
  fVar14 = (float)(~-(uint)NAN(fVar13) & (uint)fVar16 | -(uint)NAN(fVar13) & (uint)fVar14);
  uVar4 = 0;
  uVar5 = 0;
  if ((fVar7 <= fVar14) &&
     ((uVar6 = 2, 0.0 <= fVar7 ||
      ((((uVar5 = uVar4, fVar1 < param_2[3] && (fVar2 < param_2[4])) && (fVar3 < param_2[5])) &&
       (((*param_2 < fVar1 && (param_2[1] < fVar2)) && (uVar6 = 1, param_2[2] < fVar3)))))))) {
    param_1[1] = fVar7;
    param_1[2] = fVar14;
    uVar5 = uVar6;
  }
  *param_1 = uVar5;
  return;
}

