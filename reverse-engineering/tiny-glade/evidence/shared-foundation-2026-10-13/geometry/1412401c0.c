utils::geometry::aabb::Aabb2::intersect_ray

uint FUN_1412401c0(float *param_1,float *param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  
  fVar5 = (param_1[2] - *param_2) / param_2[2];
  fVar2 = (param_1[3] - param_2[1]) / param_2[3];
  fVar4 = (*param_1 - *param_2) / param_2[2];
  fVar3 = (param_1[1] - param_2[1]) / param_2[3];
  fVar1 = fVar4;
  if (fVar5 <= fVar4) {
    fVar1 = fVar5;
  }
  fVar6 = fVar3;
  if (fVar2 <= fVar3) {
    fVar6 = fVar2;
  }
  fVar2 = (float)(~-(uint)NAN(fVar2) & (uint)fVar6 | -(uint)NAN(fVar2) & (uint)fVar3);
  fVar1 = (float)(~-(uint)NAN(fVar5) & (uint)fVar1 | -(uint)NAN(fVar5) & (uint)fVar4);
  fVar3 = fVar2;
  if (fVar2 <= fVar1) {
    fVar3 = fVar1;
  }
  return ~-(uint)NAN(fVar1) & (uint)fVar3 | -(uint)NAN(fVar1) & (uint)fVar2;
}

