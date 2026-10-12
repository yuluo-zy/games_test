
void __cdecl country_core::systems::collision::world::dispose_collider_mesh_instance(void)

{
  undefined8 *puVar1;
  byte bVar2;
  uint uVar3;
  int iVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined4 uVar8;
  ulonglong uVar9;
  undefined8 *in_RCX;
  longlong in_RDX;
  int *in_R8;
  char local_1c8;
  undefined7 uStack_1c7;
  undefined1 uStack_1c0;
  undefined7 uStack_1bf;
  undefined1 uStack_1b8;
  undefined7 uStack_1b7;
  undefined8 uStack_1b0;
  int local_144;
  undefined1 local_118 [16];
  longlong *local_108;
  uint local_f8;
  undefined8 local_e4;
  undefined8 uStack_dc;
  undefined8 local_d4;
  undefined4 local_cc;
  undefined4 local_68;
  undefined8 local_64;
  undefined4 local_5c;
  undefined4 local_58;
  undefined4 local_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined7 uStack_40;
  undefined8 local_30;
  
  local_30 = 0xfffffffffffffffe;
  bVar2 = *(byte *)(in_R8 + 4);
  rapier3d::geometry::collider_set::ColliderSet::remove();
  if (local_144 == 2) {
                    /* WARNING: Subroutine does not return */
    core::option::expect_failed();
  }
  memcpy(local_118,&local_1c8,0xb0);
  uVar3 = in_R8[1];
  uVar9 = (ulonglong)uVar3;
  if (((uVar9 < *(ulonglong *)(in_RDX + 0x10)) && (lVar5 = *(longlong *)(in_RDX + 8), lVar5 != 0))
     && (iVar4 = *in_R8, *(int *)(lVar5 + 0x20 + uVar9 * 0x28) == iVar4)) {
    local_1c8 = *(char *)(lVar5 + uVar9 * 0x28);
    puVar1 = (undefined8 *)(lVar5 + 0x10 + uVar9 * 0x28);
    uVar6 = *puVar1;
    uStack_1b0 = puVar1[1];
    uStack_48 = (undefined4)((ulonglong)uVar6 >> 8);
    uStack_44._0_3_ = (undefined3)((ulonglong)uVar6 >> 0x28);
    uStack_44._3_1_ = (undefined1)uStack_1b0;
    uStack_40 = (undefined7)((ulonglong)uStack_1b0 >> 8);
    puVar1 = (undefined8 *)(lVar5 + 1 + uVar9 * 0x28);
    uVar6 = *puVar1;
    uVar7 = puVar1[1];
    local_58 = (undefined4)uVar6;
    local_54 = (undefined4)((ulonglong)uVar6 >> 0x20);
    uStack_50 = (undefined4)uVar7;
    uStack_4c = (int)((ulonglong)uVar7 >> 0x20);
    uVar8 = uStack_4c;
    *(undefined4 *)(lVar5 + uVar9 * 0x28) = *(undefined4 *)(in_RDX + 0x18);
    *(uint *)(in_RDX + 0x18) = uVar3;
    *(int *)(in_RDX + 0x1c) = *(int *)(in_RDX + 0x1c) + -1;
    *(int *)(lVar5 + 0x20 + uVar9 * 0x28) = iVar4 + 1;
    if (local_1c8 != '\x11') {
      uStack_1c7 = (undefined7)uVar6;
      uStack_1c0 = (undefined1)((ulonglong)uVar6 >> 0x38);
      uStack_1bf = (undefined7)uVar7;
      uStack_4c._3_1_ = (undefined1)((ulonglong)uVar7 >> 0x38);
      uStack_1b8 = uStack_4c._3_1_;
      uStack_1b7 = (undefined7)
                   (CONCAT35((undefined3)uStack_44,CONCAT41(uStack_48,uStack_4c._3_1_)) >> 8);
      *(undefined1 *)(in_RDX + (ulonglong)bVar2 * 0x390 + 0x3a8) = 1;
      uStack_4c = uVar8;
      rapier3d::geometry::collider::Collider::compute_aabb();
      local_68 = local_58;
      local_64 = CONCAT44(uStack_4c,uStack_50);
      local_5c = uStack_44;
      country_core::utils::dirty_tiles::DirtyTiles::
      iter_world_space_rect_cells<country_core::utils::dirty_tiles::impl_0::rasterize_world_space_rect::closure_env_0>
                ();
      uStack_4c = in_R8[4];
      local_54 = (undefined4)*(undefined8 *)(in_R8 + 2);
      uStack_50 = (undefined4)((ulonglong)*(undefined8 *)(in_R8 + 2) >> 0x20);
      local_58 = 1;
      tabloid::Publisher::
      publish<country_core::systems::collision::world::ColliderDiff,enum2_<country_core::systems::collision::world::ColliderDelta>_>
                ();
      in_RCX[2] = local_d4;
      *(undefined4 *)(in_RCX + 3) = local_cc;
      in_RCX[4] = CONCAT71(uStack_1c7,local_1c8);
      in_RCX[5] = CONCAT71(uStack_1bf,uStack_1c0);
      in_RCX[6] = CONCAT71(uStack_1b7,uStack_1b8);
      in_RCX[7] = uStack_1b0;
      *in_RCX = local_e4;
      in_RCX[1] = uStack_dc;
      LOCK();
      *local_108 = *local_108 + -1;
      UNLOCK();
      if (*local_108 == 0) {
        alloc::sync::Arc::
        drop_slow<dyn_<core::convert::AsRef<slice2_<u8>_>,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
                  ();
      }
      if (1 < local_f8) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

