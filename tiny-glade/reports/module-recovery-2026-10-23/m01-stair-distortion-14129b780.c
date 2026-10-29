
void __cdecl
country_core::systems::collision::world::RaycastWorld::
iter_shape_overlaps<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_0,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_1>
          (void)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  byte extraout_AL;
  undefined8 extraout_RAX;
  longlong in_RCX;
  longlong *plVar4;
  uint in_R8D;
  longlong in_R9;
  longlong lVar5;
  undefined1 local_24c [28];
  undefined1 local_230 [24];
  longlong local_218;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined8 uStack_204;
  longlong *local_1f8;
  undefined8 uStack_1f0;
  longlong **local_1b8;
  longlong *local_1a8;
  undefined *local_1a0;
  undefined8 local_198;
  undefined4 local_188;
  undefined4 local_17c;
  undefined1 local_170 [16];
  longlong local_160;
  longlong **local_158;
  longlong local_150;
  undefined1 *local_148;
  longlong local_140;
  longlong local_138;
  longlong local_128;
  longlong *local_120;
  longlong local_118 [2];
  undefined8 uStack_104;
  longlong local_f0;
  longlong local_e8;
  undefined1 *local_e0;
  undefined1 *local_d8;
  char *local_d0;
  longlong local_c8;
  longlong local_c0;
  longlong *local_b8;
  undefined1 *local_b0;
  longlong *local_a8;
  longlong local_a0 [2];
  longlong local_90;
  longlong local_88;
  longlong local_80;
  uint local_74;
  longlong local_70;
  longlong *local_68;
  undefined8 uStack_60;
  char local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_a0[0] = in_R9;
  local_74 = in_R8D;
  impl$42::into_transformed_shape();
  uStack_104 = uStack_204;
  local_118[0] = local_218;
  local_68 = local_1f8;
  uStack_60 = uStack_1f0;
  local_70 = in_RCX;
  Rapier::subworlds();
  local_88 = CONCAT44(uStack_20c,uStack_210);
  local_80 = local_70 + 0x3d80;
  do {
    do {
      lVar5 = local_218;
      if (lVar5 == local_88) goto LAB_14129ba4b;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_218 = lVar5 + 0x390;
    } while ((local_74 >> (extraout_AL & 0x1f) & 1) == 0);
    if (local_a0[0] == 0) {
      plVar4 = (longlong *)0x0;
    }
    else {
      local_c8 = local_70;
      local_b8 = local_a0;
      local_b0 = &local_49;
      plVar4 = &local_c8;
      local_c0 = lVar5;
    }
    local_4a = '\0';
    local_90 = local_218;
    impl$47::as_collider_shape();
    local_1a0 = &DAT_142c29b38;
    local_198 = 0;
    local_188 = 0;
    local_17c = 0;
    local_f0 = local_70;
    local_e0 = local_170;
    local_d8 = &local_49;
    local_d0 = &local_4a;
    lVar1 = *(longlong *)(lVar5 + 0x138);
    lVar2 = *(longlong *)(lVar5 + 0x140);
    lVar3 = *(longlong *)(lVar2 + 0x10);
    local_1a8 = plVar4;
    local_e8 = lVar5;
    nalgebra::geometry::isometry::Isometry::
    inverse<f32,nalgebra::base::unit::Unit<nalgebra::geometry::quaternion::Quaternion<f32>_>,3>();
    local_140 = lVar1 + (lVar3 - 1U & 0xfffffffffffffff0) + 0x10;
    local_158 = &local_1a8;
    local_150 = local_80;
    local_148 = local_24c;
    local_120 = &local_f0;
    local_a8 = &local_160;
    local_160 = lVar5;
    local_138 = lVar2;
    local_128 = local_218;
    (**(code **)(local_218 + 0x110))(local_230,extraout_RAX,local_118);
    parry3d::bounding_volume::simd_aabb::SimdAabb::splat();
    local_1b8 = &local_a8;
    parry3d::partitioning::qbvh::qbvh::Qbvh::
    traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut_<ref_mut_<rapier3d::pipeline::query_pipeline::impl_7::intersections_with_shape::closure_env_0<country_core::systems::collision::world::impl_14::iter_shape_overlaps::closure_env_1<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_0,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_1>_>_>_>_>_>
              ();
    local_218 = local_90;
  } while (local_4a != '\x01');
LAB_14129ba4b:
  LOCK();
  *local_68 = *local_68 + -1;
  UNLOCK();
  if (*local_68 == 0) {
    alloc::sync::Arc::
    drop_slow<dyn_<core::convert::AsRef<slice2_<u8>_>,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
              ();
  }
  return;
}

