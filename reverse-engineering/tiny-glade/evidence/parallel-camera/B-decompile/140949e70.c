
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::camera_rig::normal::impl$1::translate_screen_space(void)

{
  longlong in_RCX;
  float *in_RDX;
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  float local_5c;
  uint local_50;
  
  ::utils::camera::CameraMatrices::eye_position();
  ::utils::camera::CameraMatrices::eye_direction();
  fVar9 = local_5c / (float)(local_50 ^ ___xmm_80000000800000008000000080000000);
  if (fVar9 <= __real_3f800000) {
    fVar9 = __real_3f800000;
  }
  fVar7 = *(float *)(in_RCX + 0x54) * __real_3c8efa35 * __real_3f000000;
  fVar1 = sinf(fVar7);
  fVar2 = cosf(fVar7);
  fVar9 = fVar9 / __real_42480000;
  fVar7 = *in_RDX;
  fVar5 = (float)*(undefined8 *)(in_RDX + 1);
  fVar6 = (float)((ulonglong)*(undefined8 *)(in_RDX + 1) >> 0x20);
  fVar4 = fVar2 * fVar2;
  fVar8 = fVar1 * fVar1 + 0.0 + 0.0;
  fVar3 = fVar6 * 0.0 + fVar5 * fVar1 + fVar7 * 0.0;
  fVar3 = fVar3 + fVar3;
  fVar2 = fVar2 + fVar2;
  *(ulonglong *)(in_RCX + 0x44) =
       CONCAT44((float)((ulonglong)*(undefined8 *)(in_RCX + 0x44) >> 0x20) +
                fVar9 * (fVar2 * (fVar7 * 0.0 - fVar6 * 0.0) +
                        fVar3 * fVar1 + (fVar4 - fVar8) * fVar5),
                (float)*(undefined8 *)(in_RCX + 0x44) +
                fVar9 * (fVar2 * (fVar1 * fVar6 - fVar5 * 0.0) +
                        fVar3 * 0.0 + (fVar4 - fVar8) * fVar7));
  *(float *)(in_RCX + 0x4c) =
       (fVar2 * (fVar5 * 0.0 - fVar7 * fVar1) + fVar3 * 0.0 + (fVar4 - fVar8) * fVar6) * fVar9 +
       *(float *)(in_RCX + 0x4c);
  return;
}

