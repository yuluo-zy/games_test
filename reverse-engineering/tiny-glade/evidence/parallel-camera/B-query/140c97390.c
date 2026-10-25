
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl utils::camera::CameraProjectionParams::calc_matrices(void)

{
  float fVar1;
  undefined8 uVar2;
  undefined1 (*in_RCX) [16];
  float *in_RDX;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  
  fVar1 = in_RDX[1];
  fVar4 = in_RDX[2] * __real_3c8efa35 * __real_3f000000;
  fVar3 = cosf(fVar4);
  fVar4 = sinf(fVar4);
  fVar3 = fVar3 / fVar4;
  fVar5 = __real_3f800000 / (fVar3 / *in_RDX);
  fVar6 = __real_3f800000 / fVar3;
  fVar4 = __real_3f800000 / fVar1;
  *in_RCX = ZEXT416((uint)(fVar3 / *in_RDX));
  in_RCX[1] = ZEXT416((uint)fVar3) << 0x20;
  uVar2 = _UNK_142b2e398;
  *(undefined8 *)in_RCX[2] = ___xmm_bf800000000000000000000000000000;
  *(undefined8 *)(in_RCX[2] + 8) = uVar2;
  in_RCX[3] = ZEXT416((uint)fVar1) << 0x40;
  in_RCX[4] = ZEXT416((uint)fVar5);
  in_RCX[5] = ZEXT416((uint)fVar6) << 0x20;
  *(undefined4 *)in_RCX[6] = 0;
  *(undefined4 *)(in_RCX[6] + 4) = 0;
  *(undefined4 *)(in_RCX[6] + 8) = 0;
  *(float *)(in_RCX[6] + 0xc) = fVar4;
  uVar2 = _UNK_142a10e58;
  *(undefined8 *)in_RCX[7] = ___xmm_00000000bf8000000000000000000000;
  *(undefined8 *)(in_RCX[7] + 8) = uVar2;
  return;
}

