// ADDRESS: 0x140b18550
// SIZE: 725
// SIGNATURE: void __cdecl look_at(void)

void __cdecl country_core::render::directional_light::DirectionalLight::look_at(void)

{
  float fVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 (*in_RCX) [16];
  undefined8 *in_RDX;
  undefined8 *in_R8;
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
  
  fVar1 = *(float *)(in_RDX + 1);
  fVar7 = fVar1 - *(float *)(in_R8 + 1);
  fVar11 = *(float *)(in_R8 + 1) - fVar1;
  fVar12 = (float)*in_R8;
  fVar14 = (float)((ulonglong)*in_R8 >> 0x20);
  fVar5 = (float)*in_RDX;
  fVar6 = (float)((ulonglong)*in_RDX >> 0x20);
  fVar9 = fVar5 - fVar12;
  fVar10 = fVar6 - fVar14;
  fVar12 = fVar12 - fVar5;
  fVar14 = fVar14 - fVar6;
  fVar8 = 1.0 / SQRT(fVar14 * fVar14 + fVar12 * fVar12 + fVar11 * fVar11);
  fVar12 = fVar8 * fVar12;
  fVar14 = fVar8 * fVar14;
  fVar8 = fVar8 * fVar11;
  fVar11 = fVar12 - fVar14 * 0.0;
  fVar15 = fVar14 * 0.0 - fVar8;
  fVar13 = fVar8 * 0.0 - fVar12 * 0.0;
  fVar17 = 1.0 / SQRT(fVar11 * fVar11 + fVar13 * fVar13 + fVar15 * fVar15);
  fVar15 = fVar15 * fVar17;
  fVar13 = fVar13 * fVar17;
  fVar11 = fVar11 * fVar17;
  fVar17 = fVar14 * fVar15 - fVar12 * fVar13;
  fVar16 = fVar8 * fVar13 - fVar14 * fVar11;
  fVar18 = fVar11 * fVar12 - fVar8 * fVar15;
  auVar4._4_4_ = fVar16;
  auVar4._0_4_ = fVar15;
  auVar4._8_4_ = -fVar12;
  auVar4._12_4_ = 0;
  *in_RCX = auVar4;
  auVar3._4_4_ = fVar18;
  auVar3._0_4_ = fVar13;
  auVar3._8_4_ = -fVar14;
  auVar3._12_4_ = 0;
  in_RCX[1] = auVar3;
  auVar2._4_4_ = fVar17;
  auVar2._0_4_ = fVar11;
  auVar2._8_4_ = -fVar8;
  auVar2._12_4_ = 0;
  in_RCX[2] = auVar2;
  *(float *)in_RCX[3] = -(fVar1 * fVar11 + fVar15 * fVar5 + fVar6 * fVar13);
  *(float *)(in_RCX[3] + 4) = -(fVar1 * fVar17 + fVar18 * fVar6 + fVar5 * fVar16);
  *(float *)(in_RCX[3] + 8) = fVar1 * fVar8 + fVar6 * fVar14 + fVar5 * fVar12;
  *(undefined4 *)(in_RCX[3] + 0xc) = 0x3f800000;
  *(ulonglong *)in_RCX[4] = CONCAT44(fVar14,fVar12);
  *(float *)(in_RCX[4] + 8) = fVar8;
  *(undefined8 *)(in_RCX[4] + 0xc) = *in_RDX;
  *(undefined4 *)(in_RCX[5] + 4) = *(undefined4 *)(in_RDX + 1);
  *(undefined8 *)(in_RCX[5] + 8) = 0x3f8000003f800000;
  *(undefined4 *)in_RCX[6] = 0x3f800000;
  *(undefined8 *)(in_RCX[6] + 4) = *in_R8;
  *(undefined4 *)(in_RCX[6] + 0xc) = *(undefined4 *)(in_R8 + 1);
  *(float *)in_RCX[7] = SQRT(fVar10 * fVar10 + fVar9 * fVar9 + fVar7 * fVar7);
  return;
}

