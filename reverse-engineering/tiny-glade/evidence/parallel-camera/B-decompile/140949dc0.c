
void __cdecl country_core::systems::camera_rig::normal::impl$1::constrain_to_be_above_terrain(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  longlong in_RCX;
  float extraout_XMM0_Da;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  
  fVar1 = *(float *)(in_RCX + 0x40);
  fVar2 = *(float *)(in_RCX + 0x4c);
  fVar6 = (float)*(undefined8 *)(in_RCX + 0x38);
  fVar7 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0x38) >> 0x20);
  fVar3 = *(float *)(in_RCX + 0x44);
  fVar8 = fVar7 + *(float *)(in_RCX + 0x48);
  resources::terrain_height::TerrainHeightsData::
  sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
  fVar4 = extraout_XMM0_Da + *(float *)(in_RCX + 0x68);
  fVar5 = fVar4;
  if (fVar4 <= fVar8) {
    fVar5 = fVar8;
  }
  *(ulonglong *)(in_RCX + 0x44) =
       CONCAT44((float)(~-(uint)NAN(fVar8) & (uint)fVar5 | -(uint)NAN(fVar8) & (uint)fVar4) - fVar7,
                (fVar3 + fVar6) - fVar6);
  *(float *)(in_RCX + 0x4c) = (fVar2 + fVar1) - fVar1;
  return;
}

