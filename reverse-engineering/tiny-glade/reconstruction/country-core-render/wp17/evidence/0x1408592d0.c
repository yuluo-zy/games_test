// 0x1408592d0 reapply

void __cdecl country_core::resources::history::terrain_edit::TerrainEdit::reapply(void)

{
  float fVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  undefined4 uVar5;
  longlong lVar6;
  longlong lVar7;
  undefined8 uVar8;
  longlong lVar9;
  ulonglong uVar10;
  undefined8 uVar11;
  ulonglong uVar12;
  ulonglong uVar13;
  ulonglong *in_RCX;
  longlong lVar14;
  longlong *in_R8;
  longlong *in_R9;
  float extraout_XMM0_Da;
  longlong *in_stack_00000028;
  float in_stack_00000030;
  undefined8 local_c0;
  longlong local_b8;
  ulonglong local_b0;
  undefined8 local_78;
  undefined4 local_70;
  
  uVar13 = 2;
  if ((*in_RCX ^ 0x8000000000000000) < 2) {
    uVar13 = *in_RCX ^ 0x8000000000000000;
  }
  if (uVar13 == 0) {
    uVar12 = in_RCX[4];
    uVar13 = in_RCX[2];
    uVar10 = in_RCX[3];
    update_terrain_curve();
    fVar1 = *(float *)((longlong)in_RCX + 0x24);
    if (uVar10 == 0 || uVar13 + uVar10 * 0xc == 0xc) {
      local_70 = 0;
      local_78 = 0;
    }
    else {
      ::utils::swizzlers::impl$1::xvy();
    }
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar6 = *in_R8;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar7 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar9 = *(longlong *)(lVar6 + 0x28);
    *(undefined4 *)(lVar9 + lVar7 * 0x28) = 0;
    *(undefined4 *)(lVar9 + 8 + lVar7 * 0x28) = 1;
    *(float *)(lVar9 + 0xc + lVar7 * 0x28) = extraout_XMM0_Da * fVar1;
    *(undefined8 *)(lVar9 + 0x10 + lVar7 * 0x28) = local_78;
    *(undefined4 *)(lVar9 + 0x18 + lVar7 * 0x28) = local_70;
    *(int *)(lVar9 + 0x1c + lVar7 * 0x28) = (int)uVar12;
    *(undefined8 *)(lVar9 + 0x20 + lVar7 * 0x28) = uVar8;
    *(longlong *)(lVar6 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
    *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
    lVar6 = *in_R9;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar7 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar9 = *(longlong *)(lVar6 + 0x28);
    *(undefined8 *)(lVar9 + lVar7 * 0x10) = uVar8;
    *(int *)(lVar9 + 8 + lVar7 * 0x10) = (int)uVar12;
    *(longlong *)(lVar6 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  }
  else if (uVar13 == 1) {
    uVar13 = in_RCX[1];
    fVar1 = *(float *)(in_RCX + 3);
    fVar2 = *(float *)((longlong)in_RCX + 0x14);
    fVar3 = *(float *)(in_RCX + 2);
    fVar4 = *(float *)((longlong)in_RCX + 0xc);
    *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
    lVar6 = *in_R8;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar7 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                ();
    }
    lVar9 = *(longlong *)(lVar6 + 0x28);
    *(uint *)(lVar9 + lVar7 * 0x28) = -(uint)(fVar1 != fVar2) & 1;
    *(float *)(lVar9 + 4 + lVar7 * 0x28) =
         fVar1 * in_stack_00000030 + fVar2 * (1.0 - in_stack_00000030);
    *(uint *)(lVar9 + 8 + lVar7 * 0x28) = -(uint)(fVar3 != fVar4) & 1;
    *(float *)(lVar9 + 0xc + lVar7 * 0x28) =
         fVar3 * in_stack_00000030 + (1.0 - in_stack_00000030) * fVar4;
    *(undefined8 *)(lVar9 + 0x10 + lVar7 * 0x28) = 0;
    *(undefined4 *)(lVar9 + 0x18 + lVar7 * 0x28) = 0;
    *(int *)(lVar9 + 0x1c + lVar7 * 0x28) = (int)uVar13;
    *(undefined8 *)(lVar9 + 0x20 + lVar7 * 0x28) = uVar8;
    *(longlong *)(lVar6 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  }
  else {
    ::utils::easing_functions::EasingFunction::apply();
    alloc::vec::spec_from_iter::impl$0::
    from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::slice::iter::Iter<glam::f32::vec3::Vec3>,country_core::resources::history::terrain_edit::impl_1::reapply::closure_env_1>_>
              ();
    if (local_b0 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    uVar8 = *(undefined8 *)(local_b8 + (local_b0 >> 1) * 0xc);
    uVar5 = *(undefined4 *)(local_b8 + 8 + (local_b0 >> 1) * 0xc);
    uVar13 = in_RCX[4];
    *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    lVar6 = *in_stack_00000028;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar11 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar7 == *(longlong *)(lVar6 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::render_config::LastKnownGraphicsDevice,alloc::alloc::Global>();
    }
    lVar9 = *(longlong *)(lVar6 + 0x28);
    lVar14 = lVar7 * 0x30;
    *(undefined8 *)(lVar9 + lVar14) = local_c0;
    *(longlong *)(lVar9 + 8 + lVar14) = local_b8;
    *(ulonglong *)(lVar9 + 0x10 + lVar14) = local_b0;
    *(undefined8 *)(lVar9 + 0x18 + lVar14) = uVar8;
    *(undefined4 *)(lVar9 + 0x20 + lVar14) = uVar5;
    *(int *)(lVar9 + 0x24 + lVar14) = (int)uVar13;
    *(undefined8 *)(lVar9 + 0x28 + lVar14) = uVar11;
    *(longlong *)(lVar6 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  }
  return;
}

