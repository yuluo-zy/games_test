
void __cdecl utils::geometry::aabb::Aabb2::expanded(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float *in_RCX;
  float *in_RDX;
  float in_XMM2_Da;
  float fVar4;
  float fVar5;
  float in_XMM3_Da;
  
  fVar4 = in_XMM2_Da * (float)__xmm_00000000000000003f0000003f000000;
  fVar5 = in_XMM3_Da * __xmm_00000000000000003f0000003f000000._4_4_;
  fVar1 = in_RDX[1];
  fVar2 = in_RDX[2];
  fVar3 = in_RDX[3];
  *in_RCX = *in_RDX - fVar4;
  in_RCX[1] = fVar1 - fVar5;
  in_RCX[2] = fVar4 + fVar2;
  in_RCX[3] = fVar5 + fVar3;
  return;
}

