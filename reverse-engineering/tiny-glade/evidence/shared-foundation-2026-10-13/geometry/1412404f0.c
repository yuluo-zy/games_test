utils::geometry::aabb::Aabb3::inclusive_contains_point

undefined4 tg_math_1412404f0(float *param_1,float *param_2)

{
  uint3 uVar1;
  uint3 uVar2;
  
  uVar1 = CONCAT12(param_1[2] <= param_2[2],CONCAT11(param_1[1] <= param_2[1],*param_1 <= *param_2))
  ;
  uVar2 = CONCAT12(param_2[2] <= param_1[5],
                   CONCAT11(param_2[1] <= param_1[4],*param_2 <= param_1[3])) & uVar1;
  return CONCAT31(uVar1 >> 8,0xffff < uVar2 & (byte)(uVar2 >> 8) & (byte)uVar2);
}

