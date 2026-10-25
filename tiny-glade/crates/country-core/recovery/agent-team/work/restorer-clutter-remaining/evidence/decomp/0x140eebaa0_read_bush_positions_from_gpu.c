
void __cdecl
system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu(void)

{
  uint uVar1;
  int iVar2;
  uint extraout_EAX;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  undefined8 *extraout_RAX;
  undefined8 *puVar3;
  longlong *in_RCX;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  bool bVar8;
  undefined8 local_458;
  ulonglong local_450;
  undefined *local_258 [67];
  
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    local_258[0] = &anon_584c1c5eecec72721120725a2c49f1cb_0_llvm_16787586538867573953;
    local_258[1] = (undefined *)0x5c;
    if ((extraout_RAX[3] != 0x5c) ||
       (iVar2 = memcmp(&anon_584c1c5eecec72721120725a2c49f1cb_0_llvm_16787586538867573953,
                       (void *)extraout_RAX[2],0x5c), iVar2 != 0)) {
      local_458 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    puVar3 = (undefined8 *)(**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (puVar3 != (undefined8 *)0x0) {
      if (puVar3[1] == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      uVar1 = *(uint *)*puVar3;
      if ((ulonglong)puVar3[1] <= (ulonglong)uVar1) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_end_index_len_fail();
      }
      *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
      lVar7 = *in_RCX;
      lVar5 = *(longlong *)(lVar7 + 0x10);
      if (lVar5 != 0) {
        lVar6 = *(longlong *)(lVar7 + 8);
        lVar4 = 0;
        do {
          *(undefined8 *)(lVar6 + 0x10 + lVar4) = 0;
          lVar4 = lVar4 + 0x20;
        } while (lVar5 << 5 != lVar4);
      }
      if (uVar1 != 0) {
        lVar5 = 0;
        do {
          ::utils::geometry::aabb::Aabb2::from_center_radius();
          country_core::utils::dirty_tiles::DirtyTiles::
          iter_world_space_rect_cells<system_clutter::autoclutter::read_bush_positions_from_gpu::read_bush_positions_from_gpu::closure_env_1>
                    ();
          lVar5 = lVar5 + 8;
        } while ((ulonglong)uVar1 * 8 - lVar5 != 0);
      }
      local_458 = 4;
      local_450 = 0;
      ::utils::calculate_hash<ref_<slice2_<array_<u32,2>_>_>_>();
      country_core::utils::dirty_tiles::DirtyTiles::new_clear();
      if (*(longlong *)(lVar7 + 0x10) != 0) {
        lVar5 = *(longlong *)(lVar7 + 0x10) * 0x20 + *(longlong *)(lVar7 + 8);
        bVar8 = false;
        lVar7 = *(longlong *)(lVar7 + 8);
        do {
          while( true ) {
            lVar6 = lVar7 + 0x20;
            local_458 = *(undefined8 *)(lVar7 + 8);
            local_450 = *(ulonglong *)(lVar7 + 0x10);
            if (1 < local_450) {
              if (local_450 < 0x15) {
                core::slice::sort::shared::smallsort::
                insertion_sort_shift_left<array_<u32,2>,bool_(*)(ref_<array_<u32,2>_>,ref_<array_<u32,2>_>)>
                          ();
              }
              else {
                core::slice::sort::unstable::
                ipnsort<array_<u32,2>,bool_(*)(ref_<array_<u32,2>_>,ref_<array_<u32,2>_>)>();
              }
            }
            ::utils::calculate_hash<ref_<slice2_<array_<u32,2>_>_>_>();
            if (*(uint *)(lVar7 + 0x18) != (extraout_EAX_00 ^ extraout_EAX)) break;
            lVar7 = lVar6;
            if (lVar6 == lVar5) {
              if (!bVar8) {
                return;
              }
              goto LAB_140eebce3;
            }
          }
          *(uint *)(lVar7 + 0x18) = extraout_EAX_00 ^ extraout_EAX;
          country_core::resources::walls::decorator::DecoratorRank::new();
          if (0xfff < extraout_EAX_01) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          local_258[extraout_EAX_01 >> 6] =
               (undefined *)
               ((ulonglong)local_258[extraout_EAX_01 >> 6] | 1L << ((byte)extraout_EAX_01 & 0x3f));
          bVar8 = true;
          lVar7 = lVar6;
        } while (lVar6 != lVar5);
LAB_140eebce3:
        memcpy(&local_458,local_258,0x200);
        tabloid::Publisher::
        publish<country_core::utils::dirty_tiles::DirtyTiles,country_core::utils::dirty_tiles::DirtyTiles>
                  ();
      }
    }
  }
  return;
}

