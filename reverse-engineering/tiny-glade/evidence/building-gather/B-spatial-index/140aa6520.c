
void __cdecl
country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::
world_space_to_grid_coord_unclamped(void)

{
  float fVar1;
  int iVar2;
  longlong in_RCX;
  float in_XMM1_Da;
  float in_XMM2_Da;
  float fVar3;
  
  fVar1 = *(float *)(in_RCX + 0x54);
  fVar3 = __real_3f000000 * fVar1;
  iVar2 = *(int *)(in_RCX + 0x4c);
  floorf((*(float *)(in_RCX + 0x50) * __real_3f000000 + in_XMM1_Da) /
         (*(float *)(in_RCX + 0x50) / (float)*(int *)(in_RCX + 0x48)));
  floorf((fVar3 + in_XMM2_Da) / (fVar1 / (float)iVar2));
  return;
}

