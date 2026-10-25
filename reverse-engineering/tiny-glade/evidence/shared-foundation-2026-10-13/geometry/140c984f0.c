utils::geometry::rectangle::Rectangle3d::rotate_projective_xz

void tg_math_140c984f0(float *param_1,float *param_2)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  
  fVar5 = (float)*(undefined8 *)param_1;
  fVar6 = (float)((ulonglong)*(undefined8 *)param_1 >> 0x20);
  fVar1 = *param_2;
  fVar2 = param_2[1];
  fVar3 = param_2[2];
  fVar4 = param_2[3];
  fVar8 = (float)((ulonglong)*(undefined8 *)(param_1 + 2) >> 0x20);
  fVar7 = (float)*(undefined8 *)(param_1 + 2);
  *param_1 = fVar5 * fVar1 + fVar6 * fVar3;
  param_1[1] = fVar5 * fVar2 + fVar6 * fVar4;
  param_1[2] = fVar7 * fVar1 + fVar8 * fVar3;
  param_1[3] = fVar7 * fVar2 + fVar8 * fVar4;
  return;
}

