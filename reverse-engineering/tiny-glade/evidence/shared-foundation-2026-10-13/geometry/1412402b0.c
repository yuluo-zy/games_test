utils::geometry::aabb::Aabb2::max_extent

uint tg_math_1412402b0(float *param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  
  fVar1 = param_1[2] - *param_1;
  fVar2 = param_1[3] - param_1[1];
  fVar3 = fVar2;
  if (fVar2 <= fVar1) {
    fVar3 = fVar1;
  }
  return ~-(uint)NAN(fVar1) & (uint)fVar3 | -(uint)NAN(fVar1) & (uint)fVar2;
}

