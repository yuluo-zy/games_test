// 0x141dc2170 random_vel_change
// SIG: void __cdecl random_vel_change(void)

void __cdecl system_critters::boids::Boid::random_vel_change(void)

{
  undefined8 uVar1;
  float fVar2;
  longlong in_RCX;
  longlong in_RDX;
  float fVar3;
  undefined4 extraout_XMM0_Da;
  float fVar4;
  float in_XMM2_Da;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  undefined8 local_68;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined8 local_58;
  undefined4 local_50;
  
  fVar3 = *(float *)(in_RDX + 0x30) - in_XMM2_Da;
  *(float *)(in_RDX + 0x30) = fVar3;
  if (fVar3 <= 0.0) {
    fastrand::Rng::f32();
    fastrand::Rng::f32();
    ::utils::uniform_sample_cone();
    *(undefined4 *)(in_RDX + 0x2c) = uStack_60;
    *(undefined8 *)(in_RDX + 0x24) = local_68;
    fastrand::Rng::f32();
    ::utils::lerp();
    *(undefined4 *)(in_RDX + 0x30) = extraout_XMM0_Da;
  }
  fVar3 = *(float *)(in_RDX + 0x2c);
  uVar1 = *(undefined8 *)(in_RDX + 0x24);
  ::utils::time_continuous_lerp_vec3();
  *(undefined4 *)(in_RDX + 0x20) = local_50;
  *(undefined8 *)(in_RDX + 0x18) = local_58;
  glam::f32::sse2::quat::Quat::from_rotation_arc();
  local_68._0_4_ = (float)uVar1;
  local_68._4_4_ = (float)((ulonglong)uVar1 >> 0x20);
  fVar2 = *(float *)(in_RCX + 0xc);
  fVar5 = (float)*(undefined8 *)(in_RCX + 0x10);
  fVar7 = (float)((ulonglong)*(undefined8 *)(in_RCX + 0x10) >> 0x20);
  fVar8 = uStack_5c * uStack_5c;
  fVar9 = fVar3 * fVar3 + local_68._4_4_ * local_68._4_4_ + (float)local_68 * (float)local_68;
  fVar6 = fVar3 * fVar7 + local_68._4_4_ * fVar5 + (float)local_68 * fVar2;
  fVar6 = fVar6 + fVar6;
  fVar4 = uStack_5c + uStack_5c;
  *(ulonglong *)(in_RCX + 0xc) =
       CONCAT44(fVar4 * (fVar3 * fVar2 - fVar7 * (float)local_68) +
                fVar6 * local_68._4_4_ + (fVar8 - fVar9) * fVar5,
                fVar4 * (local_68._4_4_ * fVar7 - fVar5 * fVar3) +
                fVar6 * (float)local_68 + (fVar8 - fVar9) * fVar2);
  *(float *)(in_RCX + 0x14) =
       fVar4 * ((float)local_68 * fVar5 - fVar2 * local_68._4_4_) +
       fVar6 * fVar3 + (fVar8 - fVar9) * fVar7;
  return;
}

