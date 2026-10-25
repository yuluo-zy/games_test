// ADDRESS: 0x140a97650
// SIZE: 87
// SIGNATURE: void __cdecl lerp(void)

void __cdecl country_core::render::atmosphere::SunSkyColors::lerp(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float *in_RCX;
  float *in_RDX;
  float *in_R8;
  float fVar7;
  float in_XMM3_Da;
  
  fVar7 = 1.0 - in_XMM3_Da;
  fVar1 = in_RDX[1];
  fVar2 = in_RDX[2];
  fVar3 = in_RDX[3];
  fVar4 = in_R8[1];
  fVar5 = in_R8[2];
  fVar6 = in_R8[3];
  *in_RCX = *in_R8 * in_XMM3_Da + *in_RDX * fVar7;
  in_RCX[1] = fVar4 * in_XMM3_Da + fVar1 * fVar7;
  in_RCX[2] = fVar5 * in_XMM3_Da + fVar2 * fVar7;
  in_RCX[3] = fVar6 * in_XMM3_Da + fVar3 * fVar7;
  fVar1 = in_RDX[5];
  fVar2 = in_RDX[6];
  fVar3 = in_RDX[7];
  fVar4 = in_R8[5];
  fVar5 = in_R8[6];
  fVar6 = in_R8[7];
  in_RCX[4] = in_R8[4] * in_XMM3_Da + in_RDX[4] * fVar7;
  in_RCX[5] = fVar4 * in_XMM3_Da + fVar1 * fVar7;
  in_RCX[6] = fVar5 * in_XMM3_Da + fVar2 * fVar7;
  in_RCX[7] = fVar6 * in_XMM3_Da + fVar3 * fVar7;
  fVar1 = in_RDX[9];
  fVar2 = in_RDX[10];
  fVar3 = in_RDX[0xb];
  fVar4 = in_R8[9];
  fVar5 = in_R8[10];
  fVar6 = in_R8[0xb];
  in_RCX[8] = in_R8[8] * in_XMM3_Da + in_RDX[8] * fVar7;
  in_RCX[9] = fVar4 * in_XMM3_Da + fVar1 * fVar7;
  in_RCX[10] = fVar5 * in_XMM3_Da + fVar2 * fVar7;
  in_RCX[0xb] = fVar6 * in_XMM3_Da + fVar3 * fVar7;
  return;
}

