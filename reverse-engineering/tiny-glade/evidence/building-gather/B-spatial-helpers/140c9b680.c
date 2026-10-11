
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::geometry::rectangle::Rectangle2d::aabb(void)

{
  uint uVar1;
  float *in_RCX;
  uint *in_RDX;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  
  fVar2 = (float)*(undefined8 *)(in_RDX + 4) * (float)__xmm_00000000000000003f0000003f000000;
  fVar3 = (float)((ulonglong)*(undefined8 *)(in_RDX + 4) >> 0x20) *
          __xmm_00000000000000003f0000003f000000._4_4_;
  uVar1 = *in_RDX;
  fVar6 = (float)(uVar1 & _UNK_14292e984);
  fVar7 = (float)(uVar1 & _UNK_14292e988);
  fVar9 = (float)(uVar1 & _UNK_14292e98c);
  fVar4 = (float)(in_RDX[1] & ___xmm_7fffffff7fffffff7fffffff7fffffff) * fVar2;
  fVar5 = (float)(in_RDX[1] & _UNK_14292e984) * fVar3;
  fVar8 = (float)*(undefined8 *)(in_RDX + 2);
  fVar10 = (float)((ulonglong)*(undefined8 *)(in_RDX + 2) >> 0x20);
  *in_RCX = fVar8 - (fVar5 + fVar2 * (float)(uVar1 & ___xmm_7fffffff7fffffff7fffffff7fffffff));
  in_RCX[1] = fVar10 - (fVar4 + fVar3 * fVar6);
  in_RCX[2] = fVar8 + fVar5 + fVar2 * fVar7;
  in_RCX[3] = fVar10 + fVar4 + fVar3 * fVar9;
  return;
}

