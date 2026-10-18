
void __cdecl
core::ops::function::impls::impl$4::
call_once<tuple_<ref_<glam::f32::vec2::Vec2>_>,system_decorator::decorator_visual::gothic_wall_window::calc_gothic_half_dormer_profile_ws::closure_0::closure_env_0>
          (void)

{
  float fVar1;
  longlong lVar2;
  float *pfVar3;
  float *pfVar4;
  bool bVar5;
  byte bVar6;
  undefined8 *in_RCX;
  longlong in_RDX;
  longlong in_R8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar7;
  float fVar8;
  float fVar9;
  float fVar10;
  float fVar11;
  float local_80;
  float fStack_7c;
  float local_78;
  
  fVar1 = *(float *)(in_R8 + 4);
  country_core::resources::walls::public_wall_state::PublicWallState::is_circle();
  fVar9 = 0.38;
  if ((local_80 != 0.0) && (**(char **)(in_RDX + 8) == '\a')) {
    bVar6 = **(char **)(in_RDX + 0x10) - 1;
    if (2 < bVar6) {
                    // WARNING: Subroutine does not return
      panicking::panic();
    }
    fVar9 = *(float *)(&DAT_142c2b180 + (ulonglong)bVar6 * 4);
  }
  lVar2 = *(longlong *)(*(longlong *)(in_RDX + 0x20) + 0x10);
  bVar5 = lVar2 * 8 + *(longlong *)(*(longlong *)(in_RDX + 0x20) + 8) == 8 || lVar2 == 0;
  if (**(int **)(in_RDX + 0x18) == 0) {
    if (bVar5) {
                    // WARNING: Subroutine does not return
      option::unwrap_failed();
    }
    ::utils::inverse_lerp();
    pfVar3 = *(float **)(in_RDX + 0x28);
    fVar7 = 1.0 - extraout_XMM0_Da_00;
    fVar10 = *pfVar3 * fVar7;
    pfVar4 = *(float **)(in_RDX + 0x30);
    fVar11 = *pfVar4 * extraout_XMM0_Da_00;
    fVar8 = extraout_XMM0_Da_00 * pfVar4[2] + fVar7 * pfVar3[2];
    fVar7 = pfVar4[1] * extraout_XMM0_Da_00 + pfVar3[1] * fVar7;
  }
  else {
    if (bVar5) {
                    // WARNING: Subroutine does not return
      option::unwrap_failed();
    }
    ::utils::inverse_lerp();
    pfVar3 = *(float **)(in_RDX + 0x28);
    fVar7 = 1.0 - extraout_XMM0_Da;
    fVar10 = *pfVar3 * fVar7;
    pfVar4 = *(float **)(in_RDX + 0x48);
    fVar11 = *pfVar4 * extraout_XMM0_Da;
    fVar8 = extraout_XMM0_Da * pfVar4[2] + fVar7 * pfVar3[2];
    fVar7 = pfVar4[1] * extraout_XMM0_Da + pfVar3[1] * fVar7;
  }
  ::utils::swizzlers::impl$0::x0y();
  *in_RCX = CONCAT44(fVar9 * fStack_7c + fVar1 + fVar7,fVar9 * local_80 + fVar11 + fVar10);
  *(float *)(in_RCX + 1) = fVar9 * local_78 + fVar8;
  return;
}

