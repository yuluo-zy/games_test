
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::normal::impl$1::rotate_yaw_pitch(void)

{
  float fVar1;
  longlong in_RCX;
  float extraout_XMM0_Da;
  float in_XMM1_Da;
  float in_XMM2_Da;
  float fVar2;
  float fVar3;
  float fVar4;
  
  fVar4 = *(float *)(in_RCX + 0x50);
  core::iter::adapters::chain::impl$1::
  fold<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref_<country_core::systems::camera_rig::normal::ActiveRotation>_>,f32,core::iter::adapters::map::map_fold::closure_env_0<ref_<country_core::systems::camera_rig::normal::ActiveRotation>,f32,f32,country_core::systems::camera_rig::normal::impl_1::rotate_yaw_pitch::closure_env_0,core::iter::traits::accum::impl_100::sum::closure_env_0<core::iter::adapters::map::Map<core::iter::adapters::chain::Chain<core::slice::iter::Iter<country_core::systems::camera_rig::normal::ActiveRotation>,core::option::IntoIter<ref_<country_core::systems::camera_rig::normal::ActiveRotation>_>_>,country_core::systems::camera_rig::normal::impl_1::rotate_yaw_pitch::closure_env_0>_>_>_>
            ();
  if (*(int *)(in_RCX + 0x18) == 1) {
    fVar4 = fVar4 + extraout_XMM0_Da;
    fVar3 = (float)((uint)(in_XMM2_Da + fVar4) ^ ___xmm_80000000800000008000000080000000);
    fVar1 = *(float *)(in_RCX + 0x5c);
    fVar2 = fVar1;
    if (fVar3 <= fVar1) {
      fVar2 = fVar3;
    }
    fVar2 = (float)(-(uint)NAN(fVar3) & (uint)fVar1 | ~-(uint)NAN(fVar3) & (uint)fVar2);
    fVar1 = *(float *)(in_RCX + 0x58);
    fVar3 = fVar1;
    if (fVar1 <= fVar2) {
      fVar3 = fVar2;
    }
    *(ulonglong *)(in_RCX + 0x24) =
         CONCAT44(in_XMM1_Da + (float)((ulonglong)*(undefined8 *)(in_RCX + 0x24) >> 0x20),
                  ((float)((-(uint)NAN(fVar2) & (uint)fVar1 | ~-(uint)NAN(fVar2) & (uint)fVar3) ^
                          ___xmm_80000000800000008000000080000000) - fVar4) +
                  (float)*(undefined8 *)(in_RCX + 0x24));
  }
  return;
}

