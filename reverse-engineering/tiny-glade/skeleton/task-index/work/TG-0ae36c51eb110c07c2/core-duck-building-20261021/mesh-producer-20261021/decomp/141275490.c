
void __cdecl
core::iter::adapters::chain::impl$1::
next<core::iter::sources::once::Once<tuple_<ref_<glam::f32::vec3::Vec3>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>,ref_mut_<core::iter::adapters::step_by::StepBy<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec3::Vec3>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs_assemble::gen_stairs_collider_and_rt_geo::closure_env_1>_>_>_>
          (void)

{
  longlong *plVar1;
  ulonglong uVar2;
  longlong *in_RCX;
  int *in_RDX;
  longlong lVar3;
  ulonglong uVar4;
  ulonglong uVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  
  if (*in_RDX == 1) {
    lVar3 = *(longlong *)(in_RDX + 2);
    in_RDX[2] = 0;
    in_RDX[3] = 0;
    if (lVar3 != 0) {
      *in_RCX = lVar3;
      lVar3 = *(longlong *)(in_RDX + 6);
      in_RCX[1] = *(longlong *)(in_RDX + 4);
      in_RCX[2] = lVar3;
      return;
    }
    in_RDX[0] = 0;
    in_RDX[1] = 0;
  }
  plVar1 = *(longlong **)(in_RDX + 8);
  if (plVar1 != (longlong *)0x0) {
    lVar6 = plVar1[0xd];
    lVar3 = plVar1[0xc];
    *(undefined1 *)(plVar1 + 0xd) = 0;
    if ((lVar3 == 0) || ((char)lVar6 != '\0')) {
      uVar8 = plVar1[9];
      uVar7 = plVar1[10];
    }
    else {
      uVar2 = plVar1[9];
      uVar7 = plVar1[10];
      uVar4 = uVar7 - uVar2;
      if (uVar7 < uVar2) {
        uVar4 = 0;
      }
      if (lVar3 - 1U < uVar4) {
        uVar4 = lVar3 - 1U;
      }
      uVar8 = uVar4 + 1;
      if (uVar8 < 5) {
        lVar9 = 0;
        uVar8 = uVar2;
      }
      else {
        uVar5 = 4;
        if ((uVar8 & 3) != 0) {
          uVar5 = (ulonglong)((uint)uVar8 & 3);
        }
        lVar9 = uVar8 - uVar5;
        uVar8 = uVar2 + lVar9;
        lVar6 = 0;
        do {
          lVar6 = lVar6 + -4;
        } while (~uVar4 + uVar5 != lVar6);
        plVar1[9] = uVar2 - lVar6;
      }
      lVar3 = lVar3 - lVar9;
      do {
        if (uVar7 <= uVar8) goto LAB_1412755ae;
        uVar8 = uVar8 + 1;
        plVar1[9] = uVar8;
        lVar3 = lVar3 + -1;
      } while (lVar3 != 0);
    }
    if (uVar8 < uVar7) {
      plVar1[9] = uVar8 + 1;
      lVar3 = plVar1[4];
      lVar6 = plVar1[2];
      lVar9 = plVar1[7];
      *in_RCX = (lVar3 + uVar8) * 0xc + *plVar1;
      in_RCX[1] = (lVar3 + uVar8) * 8 + lVar6;
      in_RCX[2] = uVar8 * 4 + lVar9;
      return;
    }
  }
LAB_1412755ae:
  *in_RCX = 0;
  return;
}

