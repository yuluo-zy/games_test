utils::geometry::intersect::SphereIntersection::closest_intersection_distance

uint tg_math_14123b200(float *param_1)

{
  float fVar1;
  float fVar2;
  float fVar3;
  
  fVar1 = *param_1;
  fVar2 = param_1[1];
  fVar3 = fVar2;
  if (fVar1 <= fVar2) {
    fVar3 = fVar1;
  }
  return ~-(uint)NAN(fVar1) & (uint)fVar3 | -(uint)NAN(fVar1) & (uint)fVar2;
}

