
void __cdecl
country_core::systems::collision::world::RaycastWorld::
iter_shape_overlaps<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_2,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_3>
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
  longlong *in_R9;
  longlong lVar5;
  undefined1 local_23c [28];
  undefined1 local_220 [24];
  longlong local_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined8 uStack_1f4;
  longlong *local_1e8;
  undefined8 uStack_1e0;
  longlong **local_1a8;
  longlong *local_198;
  undefined *local_190;
  undefined8 local_188;
  undefined4 local_178;
  undefined4 local_16c;
  undefined1 local_160 [16];
  longlong local_150;
  longlong **local_148;
  longlong local_140;
  undefined1 *local_138;
  longlong local_130;
  longlong local_128;
  longlong local_118;
  longlong *local_110;
  longlong local_108 [2];
  undefined8 uStack_f4;
  longlong local_e8;
  longlong local_e0;
  undefined1 *local_d8;
  undefined1 *local_d0;
  char *local_c8;
  longlong local_c0;
  longlong local_b8;
  longlong *local_b0;
  undefined1 *local_a8;
  longlong *local_a0;
  longlong local_98;
  longlong local_90;
  longlong *local_88;
  longlong local_80;
  uint local_74;
  longlong local_70;
  longlong *local_68;
  undefined8 uStack_60;
  char local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_88 = in_R9;
  local_74 = in_R8D;
  impl$42::into_transformed_shape();
  uStack_f4 = uStack_1f4;
  local_108[0] = local_208;
  local_68 = local_1e8;
  uStack_60 = uStack_1e0;
  local_70 = in_RCX;
  Rapier::subworlds();
  local_90 = CONCAT44(uStack_1fc,uStack_200);
  local_80 = local_70 + 0x3d80;
  do {
    do {
      lVar5 = local_208;
      if (lVar5 == local_90) goto LAB_14129bd7b;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_208 = lVar5 + 0x390;
    } while ((local_74 >> (extraout_AL & 0x1f) & 1) == 0);
    if (*local_88 == 0) {
      plVar4 = (longlong *)0x0;
    }
    else {
      local_c0 = local_70;
      local_b0 = local_88;
      local_a8 = &local_49;
      plVar4 = &local_c0;
      local_b8 = lVar5;
    }
    local_4a = '\0';
    local_98 = local_208;
    impl$47::as_collider_shape();
    local_190 = &DAT_142c29b08;
    local_188 = 0;
    local_178 = 0;
    local_16c = 0;
    local_e8 = local_70;
    local_d8 = local_160;
    local_d0 = &local_49;
    local_c8 = &local_4a;
    lVar1 = *(longlong *)(lVar5 + 0x138);
    lVar2 = *(longlong *)(lVar5 + 0x140);
    lVar3 = *(longlong *)(lVar2 + 0x10);
    local_198 = plVar4;
    local_e0 = lVar5;
    nalgebra::geometry::isometry::Isometry::
    inverse<f32,nalgebra::base::unit::Unit<nalgebra::geometry::quaternion::Quaternion<f32>_>,3>();
    local_130 = lVar1 + (lVar3 - 1U & 0xfffffffffffffff0) + 0x10;
    local_148 = &local_198;
    local_140 = local_80;
    local_138 = local_23c;
    local_110 = &local_e8;
    local_a0 = &local_150;
    local_150 = lVar5;
    local_128 = lVar2;
    local_118 = local_208;
    (**(code **)(local_208 + 0x110))(local_220,extraout_RAX,local_108);
    parry3d::bounding_volume::simd_aabb::SimdAabb::splat();
    local_1a8 = &local_a0;
    parry3d::partitioning::qbvh::qbvh::Qbvh::
    traverse_depth_first_node<rapier3d::geometry::collider_components::ColliderHandle,parry3d::query::visitors::bounding_volume_intersections_visitor::BoundingVolumeIntersectionsVisitor<rapier3d::geometry::collider_components::ColliderHandle,ref_mut_<ref_mut_<rapier3d::pipeline::query_pipeline::impl_7::intersections_with_shape::closure_env_0<country_core::systems::collision::world::impl_14::iter_shape_overlaps::closure_env_1<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_2,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_3>_>_>_>_>_>
              ();
    local_208 = local_98;
  } while (local_4a != '\x01');
LAB_14129bd7b:
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

