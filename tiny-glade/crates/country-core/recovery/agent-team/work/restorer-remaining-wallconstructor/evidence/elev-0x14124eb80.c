
// WARNING: Removing unreachable block (ram,0x00014124f900)

void __cdecl
system_wall_constructor::construct_elevation_supports::rectangle_bottom_layer::
construct_rect_bottom_layer(void)

{
  float *pfVar1;
  float *pfVar2;
  char extraout_AL;
  int iVar3;
  undefined8 *extraout_RAX;
  longlong lVar4;
  undefined8 *extraout_RAX_00;
  ulonglong uVar5;
  ulonglong uVar6;
  longlong in_RDX;
  float *unaff_R15;
  float *pfVar7;
  bool bVar8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar9;
  float fVar10;
  longlong local_1d0;
  float *pfStack_190;
  
  country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
  if (extraout_AL == '\0') {
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      if ((extraout_RAX_00[3] != 0x3a) ||
         (iVar3 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_1_llvm_11959870551130474958,
                         (void *)extraout_RAX_00[2],0x3a), iVar3 != 0)) goto LAB_14124f94f;
      lVar4 = (**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
      if (lVar4 != 0) {
        local_1d0 = *(longlong *)(in_RDX + 0x150);
        if (local_1d0 == 0) goto LAB_14124f9d5;
        LOCK();
        bVar8 = *(char *)(local_1d0 + 0x18) == '\0';
        if (bVar8) {
          *(char *)(local_1d0 + 0x18) = '\x01';
        }
        UNLOCK();
        if (!bVar8) {
          parking_lot::raw_mutex::RawMutex::lock_slow();
        }
        parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
        if ((*(longlong *)(local_1d0 + 0x10) * 4 + *(longlong *)(local_1d0 + 8) == 4 ||
             *(longlong *)(local_1d0 + 0x10) == 0) && (*(longlong *)(lVar4 + 0x28) == 0))
        goto LAB_14124f9e1;
        fVar10 = *(float *)(lVar4 + 0x40);
        bVar8 = false;
        uVar5 = 0;
        while (uVar5 < 5) {
          uVar6 = uVar5 + 1;
          if (!bVar8) {
            if (uVar5 == 4) break;
            uVar6 = uVar5 + 2;
          }
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          fVar9 = fVar10;
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          ::utils::geometry::rectangle::Rectangle2d::get_point();
          fVar10 = SQRT((fVar10 - fVar9) * (fVar10 - fVar9) +
                        (extraout_XMM0_Da - extraout_XMM0_Da_00) *
                        (extraout_XMM0_Da - extraout_XMM0_Da_00));
          ceilf(fVar10 / 0.69);
          pfStack_190 = (float *)&DAT_00000004;
          fVar10 = 0.32 / fVar10;
          ::utils::random_splits_into();
          bVar8 = true;
          while (pfStack_190 != (float *)&DAT_00000004) {
            pfVar1 = pfStack_190 + 1;
            pfVar2 = pfVar1;
            pfVar7 = pfStack_190;
            if (bVar8) {
              if (pfVar1 == (float *)&DAT_00000004) break;
              unaff_R15 = pfStack_190;
              pfVar2 = pfStack_190 + 2;
              pfVar7 = pfVar1;
            }
            pfStack_190 = pfVar2;
            fVar10 = *unaff_R15;
            fVar9 = *pfVar7;
            ::utils::geometry::rectangle::Rectangle2d::get_point_u();
            ::utils::geometry::rectangle::Rectangle2d::get_point_u();
            if ((fVar10 < 0.0) || (1.0 < fVar10)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::lerp();
            ::utils::geometry::rectangle::Rectangle2d::perimeter();
            if ((fVar9 < 0.0) || (1.0 < fVar9)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::lerp();
            ::utils::geometry::rectangle::Rectangle2d::perimeter();
            minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
                      ();
            fVar10 = extraout_XMM0_Da_02 * fVar10 + extraout_XMM0_Da_01 * (1.0 - fVar10);
            bVar8 = false;
            circle_bottom_layer::
            add_bottom_brick<ref_mut_<system_wall_constructor::construct_elevation_supports::circle_bottom_layer::construct_circle_bottom_layer::closure_env_0>_>
                      ();
            unaff_R15 = pfVar7;
          }
          bVar8 = true;
          uVar5 = uVar6;
        }
        goto LAB_14124f424;
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    if ((extraout_RAX[3] != 0x3d) ||
       (iVar3 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_0_llvm_11959870551130474958,
                       (void *)extraout_RAX[2],0x3d), iVar3 != 0)) {
LAB_14124f94f:
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar4 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (lVar4 != 0) {
      local_1d0 = *(longlong *)(in_RDX + 0x140);
      if (local_1d0 == 0) {
LAB_14124f9d5:
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      LOCK();
      bVar8 = *(char *)(local_1d0 + 0x18) == '\0';
      if (bVar8) {
        *(char *)(local_1d0 + 0x18) = '\x01';
      }
      UNLOCK();
      if (!bVar8) {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      if ((*(longlong *)(local_1d0 + 0x10) * 4 + *(longlong *)(local_1d0 + 8) == 4 ||
           *(longlong *)(local_1d0 + 0x10) == 0) && (*(longlong *)(lVar4 + 0x28) == 0)) {
LAB_14124f9e1:
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      ::utils::geometry::rectangle::Rectangle2d::get_point();
      ::utils::swizzlers::impl$1::xvy();
      ::utils::xz_rotation_bad();
      country_core::resources::render::impl$1::from();
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
      uVar5 = 2;
      do {
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::swizzlers::impl$1::xvy();
        ::utils::xz_rotation_bad();
        country_core::resources::render::impl$1::from();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        bVar8 = uVar5 < 4;
        uVar5 = uVar5 + 1;
      } while (bVar8);
LAB_14124f424:
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar8 = *(char *)(local_1d0 + 0x18) == '\x01';
      if (bVar8) {
        *(char *)(local_1d0 + 0x18) = '\0';
      }
      UNLOCK();
      if (!bVar8) {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
      return;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

