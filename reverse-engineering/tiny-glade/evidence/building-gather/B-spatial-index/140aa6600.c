
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws(void)

{
  undefined8 uVar1;
  float *in_RCX;
  longlong in_RDX;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined8 uVar2;
  
  uVar1 = *(undefined8 *)(in_RDX + 0x50);
  uVar2 = *(undefined8 *)(in_RDX + 0x50);
  fVar3 = (float)((ulonglong)uVar1 >> 0x20) * _UNK_142aa5d64;
  fVar4 = (float)uVar2 * _UNK_142aa5d68;
  fVar5 = (float)((ulonglong)uVar2 >> 0x20) * _UNK_142aa5d6c;
  *in_RCX = (float)uVar1 * ___xmm_3f0000003f000000bf000000bf000000;
  in_RCX[1] = fVar3;
  in_RCX[2] = fVar4;
  in_RCX[3] = fVar5;
  return;
}

