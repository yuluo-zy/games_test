// spawn_ducks 0x140f8f210 sig=void __cdecl spawn_ducks(void)

void __cdecl system_critters::spawn_ducks::spawn_ducks(void)

{
  undefined4 uVar1;
  longlong lVar2;
  undefined4 *puVar3;
  longlong lVar4;
  ulonglong *puVar5;
  longlong lVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong lVar9;
  char extraout_AL;
  undefined8 *extraout_RAX;
  longlong *in_RDX;
  longlong lVar10;
  longlong *in_R8;
  ulonglong uVar11;
  longlong lVar12;
  longlong *in_R9;
  ulonglong uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  float fVar17;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  longlong local_148;
  longlong local_140;
  longlong local_138;
  undefined8 local_118;
  float fStack_110;
  float local_cc;
  float fStack_c8;
  float local_c4;
  ulonglong local_b0;
  
  if ((*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') &&
     (country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted(),
     extraout_AL == '\0')) {
    lVar2 = *in_stack_00000030;
    fVar17 = *(float *)(lVar2 + 0x28);
    uVar1 = *(undefined4 *)((longlong)in_R8 + 0x1c);
    puVar3 = (undefined4 *)in_R8[2];
    *puVar3 = uVar1;
    lVar4 = *in_R8;
    *(float *)(lVar4 + 0x18) = fVar17 + *(float *)(lVar4 + 0x18);
    puVar5 = (ulonglong *)*in_stack_00000038;
    lVar6 = in_stack_00000038[1];
    uVar7 = *puVar5;
    uVar13 = 0;
    uVar14 = uVar7 - *(ulonglong *)(lVar6 + 0x18);
    if (uVar7 < *(ulonglong *)(lVar6 + 0x18)) {
      uVar14 = uVar13;
    }
    uVar11 = uVar7 - *(ulonglong *)(lVar6 + 0x38);
    if (uVar7 < *(ulonglong *)(lVar6 + 0x38)) {
      uVar11 = uVar13;
    }
    uVar7 = *(ulonglong *)(lVar6 + 0x30);
    uVar15 = uVar7 - uVar11;
    if (uVar11 > uVar7) {
      uVar15 = uVar13;
    }
    uVar8 = *(ulonglong *)(lVar6 + 0x10);
    uVar16 = uVar8 - uVar14;
    if (uVar14 > uVar8) {
      uVar16 = uVar13;
    }
    uVar13 = *(ulonglong *)(lVar6 + 0x40);
    *puVar5 = uVar13 - (uVar16 + uVar15);
    if (uVar14 <= uVar8 && uVar8 - uVar14 != 0 || uVar11 <= uVar7 && uVar7 - uVar11 != 0) {
      *puVar5 = uVar13;
      fVar17 = *(float *)(lVar2 + 0x28);
      *puVar3 = uVar1;
      fVar17 = fVar17 + fVar17 + *(float *)(lVar4 + 0x18);
      *(float *)(lVar4 + 0x18) = fVar17;
    }
    else {
      fVar17 = *(float *)(lVar4 + 0x18);
    }
    if (30.0 <= fVar17) {
      lVar2 = *in_RDX;
      lVar6 = *(longlong *)(lVar2 + 0x100);
      if (lVar6 == 0) {
        lVar10 = 0;
      }
      else if (*(char *)(lVar2 + 0x108) == '\x01') {
        lVar12 = 0;
        lVar10 = 0;
        do {
          lVar9 = *(longlong *)
                   (*(longlong *)(in_RDX[1] + 0x1a8) + 0x10 +
                   (ulonglong)*(uint *)(*(longlong *)(lVar2 + 0xf8) + lVar12) * 0x48);
          if (lVar9 != 0) {
            lVar10 = lVar10 + lVar9;
          }
          lVar12 = lVar12 + 4;
        } while (lVar6 << 2 != lVar12);
      }
      else {
        lVar12 = 0;
        lVar10 = 0;
        do {
          lVar10 = lVar10 + *(longlong *)
                             (*(longlong *)(in_RDX[1] + 0x100) + 0x58 +
                             (ulonglong)*(uint *)(*(longlong *)(lVar2 + 0xf8) + lVar12) * 0xa0);
          lVar12 = lVar12 + 4;
        } while (lVar6 << 2 != lVar12);
      }
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
      if ((ulonglong)(lVar10 + *(longlong *)(*in_R9 + 0x20)) < 0xf) {
        *puVar3 = uVar1;
        itertools::Itertools::
        sorted_by<core::slice::iter::IterMut<tuple_<glam::f32::vec2::Vec2,bool>_>,system_critters::spawn_ducks::spawn_ducks::closure_env_0>
                  ();
        do {
          do {
            while( true ) {
              if (local_148 == local_138) {
                if (local_140 == 0) {
                  return;
                }
                std::alloc::__default_lib_allocator::__rust_dealloc();
                return;
              }
              lVar2 = *(longlong *)(local_138 + -8);
              ::utils::float_grid::impl$0::
              bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                        ();
              local_138 = local_138 + -8;
              if (0.99 < extraout_XMM0_Da) break;
              *(undefined1 *)(lVar2 + 8) = 0;
            }
          } while (*(char *)(lVar2 + 8) != '\0');
          ::utils::swizzlers::impl$1::xvy();
          country_core::resources::terrain_height::TerrainHeightsData::
          sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        } while (0.0 < extraout_XMM0_Da_00);
        alloc::alloc::exchange_malloc();
        *(float *)(extraout_RAX + 1) = local_c4;
        *extraout_RAX = CONCAT44(fStack_c8,local_cc);
        for (local_b0 = 1; local_b0 < 3; local_b0 = local_b0 + 1) {
          perchance::PerchanceContext::uniform_f32();
          ::utils::golden_spiral();
          ::utils::swizzlers::impl$0::x0y();
          if (local_b0 == 1) {
            alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
          }
          *(ulonglong *)((longlong)extraout_RAX + local_b0 * 0xc) =
               CONCAT44((float)((ulonglong)local_118 >> 0x20) + fStack_c8,
                        (float)local_118 + local_cc);
          *(float *)((longlong)extraout_RAX + local_b0 * 0xc + 8) = fStack_110 + local_c4;
        }
        alloc::vec::in_place_collect::impl$1::
        from_iter<system_critters::duck::Trajectory,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<glam::f32::vec3::Vec3,alloc::alloc::Global>,system_critters::spawn_ducks::spawn_ducks::closure_env_1>_>
                  ();
        perchance::PerchanceContext::uniform_f32();
        ::utils::uniform_sample_circle();
        alloc::vec::Vec::push<system_critters::duck::TrajectoryPlan,alloc::alloc::Global>();
        *(undefined1 *)(lVar2 + 8) = 1;
        if (local_140 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        *puVar3 = uVar1;
        *(undefined4 *)(lVar4 + 0x18) = 0;
      }
    }
  }
  return;
}

