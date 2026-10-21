
void __cdecl
itertools::tuple_impl::impl$7::
next<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>,tuple_<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>
          (void)

{
  ulonglong uVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *in_RCX;
  ulonglong uVar4;
  longlong *in_RDX;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong lVar7;
  longlong lVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong local_70;
  longlong lStack_68;
  longlong lStack_60;
  
  lVar7 = in_RDX[0x13];
  lVar8 = in_RDX[0x12];
  *(undefined1 *)(in_RDX + 0x13) = 0;
  if ((lVar8 == 0) || ((char)lVar7 != '\0')) {
    uVar4 = in_RDX[0xf];
    uVar9 = in_RDX[0x10];
  }
  else {
    uVar1 = in_RDX[0xf];
    uVar9 = in_RDX[0x10];
    uVar5 = uVar9 - uVar1;
    if (uVar9 < uVar1) {
      uVar5 = 0;
    }
    if (lVar8 - 1U < uVar5) {
      uVar5 = lVar8 - 1U;
    }
    uVar4 = uVar5 + 1;
    if (uVar4 < 5) {
      lVar10 = 0;
      uVar4 = uVar1;
    }
    else {
      uVar6 = 4;
      if ((uVar4 & 3) != 0) {
        uVar6 = (ulonglong)((uint)uVar4 & 3);
      }
      lVar10 = uVar4 - uVar6;
      uVar4 = uVar1 + lVar10;
      lVar7 = 0;
      do {
        lVar7 = lVar7 + -4;
      } while (~uVar5 + uVar6 != lVar7);
      in_RDX[0xf] = uVar1 - lVar7;
    }
    lVar8 = lVar8 - lVar10;
    do {
      if (uVar9 <= uVar4) goto LAB_14129d133;
      uVar4 = uVar4 + 1;
      in_RDX[0xf] = uVar4;
      lVar8 = lVar8 + -1;
    } while (lVar8 != 0);
  }
  if (uVar4 < uVar9) {
    in_RDX[0xf] = uVar4 + 1;
    if (*in_RDX == 0) {
      core::iter::adapters::chain::impl$1::
      next<core::iter::sources::once::Once<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>,ref_mut_<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>_>_>
                ();
      if ((local_70 != 0) &&
         (core::iter::adapters::chain::impl$1::
          next<core::iter::sources::once::Once<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>,ref_mut_<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>_>_>
                    (), local_70 != 0)) {
        *in_RDX = local_70;
        in_RDX[5] = lStack_60;
        in_RDX[3] = local_70;
        in_RDX[4] = lStack_68;
        in_RDX[1] = lStack_68;
        in_RDX[2] = lStack_60;
        lVar8 = in_RDX[2];
        lVar7 = in_RDX[1];
        *in_RCX = *in_RDX;
        in_RCX[1] = lVar7;
        in_RCX[2] = lVar8;
        in_RCX[3] = local_70;
        in_RCX[4] = lStack_68;
        in_RCX[5] = lStack_60;
        return;
      }
      *in_RDX = 0;
      *in_RCX = 0;
    }
    else {
      lVar8 = in_RDX[5];
      in_RDX[5] = uVar4 * 4 + in_RDX[0xd];
      lVar7 = in_RDX[3];
      lVar10 = in_RDX[4];
      in_RDX[3] = (in_RDX[10] + uVar4) * 0xc + in_RDX[6];
      in_RDX[4] = (in_RDX[10] + uVar4) * 8 + in_RDX[8];
      *in_RDX = lVar7;
      in_RDX[1] = lVar10;
      in_RDX[2] = lVar8;
      lVar8 = in_RDX[2];
      lVar3 = in_RDX[1];
      lVar7 = in_RDX[5];
      lVar10 = in_RDX[3];
      lVar2 = in_RDX[4];
      *in_RCX = *in_RDX;
      in_RCX[1] = lVar3;
      in_RCX[2] = lVar8;
      in_RCX[3] = lVar10;
      in_RCX[4] = lVar2;
      in_RCX[5] = lVar7;
    }
  }
  else {
LAB_14129d133:
    *in_RCX = 0;
  }
  return;
}

