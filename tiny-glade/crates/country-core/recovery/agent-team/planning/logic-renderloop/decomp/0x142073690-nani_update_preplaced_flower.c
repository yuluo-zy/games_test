// 0x142073690 nani_update_preplaced_flower size=3344

void __cdecl country_core::render::nani::systems::nani_update_preplaced_flower(void)

{
  byte *pbVar1;
  uint *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  char cVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  bool bVar10;
  char extraout_AL;
  uint uVar11;
  int iVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  longlong *plVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  uint uVar16;
  longlong *in_RCX;
  ulonglong uVar17;
  uint *puVar18;
  ulonglong uVar19;
  undefined *puVar20;
  ulonglong uVar21;
  longlong lVar22;
  float fVar23;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar24;
  longlong local_128;
  longlong local_f8;
  
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar10 = false;
  }
  else {
    uVar17 = 0x41;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar17 != 0) {
        if (uVar17 < 0x41) {
          if ((char)(&DAT_142fd2df1)[uVar17] < -0x40) {
LAB_142073758:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 != 0x41) goto LAB_142073758;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar17 != 0xfffffffffffffffe)) {
        if (uVar17 + 2 < 0x41) {
          if ((char)(&UNK_142fd2df3)[uVar17] < -0x40) {
LAB_1420737b6:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar17 + 2 != 0x41) goto LAB_1420737b6;
      }
    }
    puVar20 = &DAT_142fd345e;
    do {
      bVar3 = puVar20[-1];
      uVar11 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar20[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar20[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar20 + -4;
            puVar20 = puVar20 + -4;
            uVar11 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar20 = puVar20 + -3;
            uVar11 = bVar5 & 0xf;
          }
          uVar11 = bVar4 & 0x3f | uVar11 << 6;
        }
        else {
          puVar20 = puVar20 + -2;
          uVar11 = bVar4 & 0x1f;
        }
        uVar11 = (uint)(bVar3 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto LAB_142073884;
LAB_142073899:
        if (puVar20 == &UNK_142fd342f) break;
        if (puVar20 + -0x142fd342f < (undefined *)0x2e) {
          if (-0x41 < (char)puVar20[1]) break;
        }
        else if (puVar20 + -0x142fd342f == (undefined *)0x2e) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar20 = puVar20 + -1;
      if (uVar11 == 0x5c) goto LAB_142073899;
LAB_142073884:
      if (uVar11 == 0x2f) goto LAB_142073899;
    } while (puVar20 != &DAT_142fd3430);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar13 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_142074346:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar13 != 0) {
LAB_142074352:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar13 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar13 = *plVar13 + 1;
    bVar10 = true;
  }
  gpu_world::GpuWorld::owned_subset_mut();
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_03 != (undefined8 *)0x0) {
    if ((extraout_RAX_03[3] != 0x3f) ||
       (iVar12 = memcmp(&anon_107183261458dea85466985370debbb6_23_llvm_3641509820080252961,
                        (void *)extraout_RAX_03[2],0x3f), iVar12 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    plVar13 = (longlong *)(**(code **)(extraout_RAX_03[1] + 0x18))(*extraout_RAX_03);
    if (plVar13 != (longlong *)0x0) {
      lVar7 = *plVar13;
      uVar17 = plVar13[1];
      lVar8 = *in_RCX;
      lVar9 = in_RCX[1];
      puVar18 = *(uint **)(lVar8 + 0xf8);
      puVar2 = puVar18 + *(longlong *)(lVar8 + 0x100);
      cVar6 = *(char *)(lVar8 + 0x108);
      local_128 = 8;
      local_f8 = 0;
      uVar19 = 0;
      uVar21 = 0;
LAB_142073b72:
      do {
        if (cVar6 == '\0') {
          if (uVar21 == uVar19) {
            do {
              if (puVar18 == puVar2) goto LAB_142074230;
              uVar11 = *puVar18;
              puVar18 = puVar18 + 1;
              uVar19 = *(ulonglong *)
                        (*(longlong *)(lVar9 + 0x100) + 0x58 + (ulonglong)uVar11 * 0xa0);
            } while (uVar19 == 0);
            lVar14 = *(longlong *)(lVar9 + 0x100) + (ulonglong)uVar11 * 0xa0;
            uVar21 = (ulonglong)*(uint *)(lVar14 + 0x94);
            lVar22 = *(longlong *)(lVar9 + 0x1a8) + uVar21 * 0x48;
            uVar21 = *(ulonglong *)(*(longlong *)(lVar9 + 0x1a8) + 0x40 + uVar21 * 0x48);
            if ((*(ulonglong *)(lVar8 + 0x110) < uVar21) &&
               (*(longlong *)(*(longlong *)(lVar22 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0
               )) {
              uVar15 = *(ulonglong *)(lVar8 + 0x118);
              if (uVar21 <= uVar15) goto LAB_142073d6f;
LAB_142073d32:
              uVar21 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar15 * 8);
              if (uVar21 == 0) goto LAB_142073d6f;
              local_f8 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar21 * 0x30);
            }
            else {
              uVar15 = *(ulonglong *)(lVar8 + 0x118);
              if (uVar15 < uVar21) goto LAB_142073d32;
LAB_142073d6f:
              local_f8 = 0;
            }
            local_128 = *(longlong *)(lVar14 + 0x50);
            uVar21 = 0;
          }
          uVar15 = (ulonglong)*(uint *)(local_128 + 8 + uVar21 * 0x10);
        }
        else {
          if (uVar21 == uVar19) {
            do {
              if (puVar18 == puVar2) goto LAB_142074230;
              uVar11 = *puVar18;
              puVar18 = puVar18 + 1;
              uVar19 = *(ulonglong *)
                        (*(longlong *)(lVar9 + 0x1a8) + 0x10 + (ulonglong)uVar11 * 0x48);
            } while (uVar19 == 0);
            lVar22 = *(longlong *)(lVar9 + 0x1a8) + (ulonglong)uVar11 * 0x48;
            uVar21 = *(ulonglong *)(lVar22 + 0x40);
            if ((*(ulonglong *)(lVar8 + 0x110) < uVar21) &&
               (*(longlong *)(*(longlong *)(lVar22 + 0x38) + *(ulonglong *)(lVar8 + 0x110) * 8) != 0
               )) {
              uVar15 = *(ulonglong *)(lVar8 + 0x118);
              if (uVar21 <= uVar15) goto LAB_142073d5c;
LAB_142073cea:
              uVar21 = *(ulonglong *)(*(longlong *)(lVar22 + 0x38) + uVar15 * 8);
              if (uVar21 == 0) goto LAB_142073d5c;
              local_f8 = *(longlong *)(*(longlong *)(lVar22 + 0x18) + 0x10 + ~uVar21 * 0x30);
            }
            else {
              uVar15 = *(ulonglong *)(lVar8 + 0x118);
              if (uVar15 < uVar21) goto LAB_142073cea;
LAB_142073d5c:
              local_f8 = 0;
            }
            uVar21 = 0;
          }
          uVar15 = uVar21 & 0xffffffff;
        }
        lVar22 = uVar15 * 0x30 + local_f8;
        fVar23 = *(float *)(lVar22 + 0x10) / 130.0 + 0.5;
        fVar24 = *(float *)(lVar22 + 0x18) / 130.0 + 0.5;
        if (fVar23 <= 0.0) {
          fVar23 = 0.0;
        }
        if (fVar24 <= 0.0) {
          fVar24 = 0.0;
        }
        if (1.0 <= fVar23) {
          fVar23 = 1.0;
        }
        if (1.0 <= fVar24) {
          fVar24 = 1.0;
        }
        fVar23 = fVar23 * 512.0;
        uVar11 = (uint)(longlong)fVar23;
        if (fVar23 < 0.0) {
          uVar11 = 0;
        }
        fVar24 = fVar24 * 512.0;
        if (4.294967e+09 < fVar23) {
          uVar11 = 0xffffffff;
        }
        uVar16 = (uint)(longlong)fVar24;
        if (fVar24 < 0.0) {
          uVar16 = 0;
        }
        if (4.294967e+09 < fVar24) {
          uVar16 = 0xffffffff;
        }
        if (0x1fe < uVar11) {
          uVar11 = 0x1ff;
        }
        if (0x1fe < uVar16) {
          uVar16 = 0x1ff;
        }
        uVar15 = (ulonglong)((uVar16 << 9 | uVar11) >> 5);
        if (uVar17 <= uVar15) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        uVar21 = uVar21 + 1;
        if (((*(uint *)(lVar7 + uVar15 * 4) >> (uVar11 & 0x1f) & 1) == 0) &&
           (::utils::float_grid::impl$0::
            bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                      (), extraout_XMM0_Da <= 0.01)) {
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          ::utils::float_grid::impl$0::
          bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>
                    ();
          fVar24 = extraout_XMM0_Da_00 + extraout_XMM0_Da_01 + extraout_XMM0_Da_02 +
                   extraout_XMM0_Da_03;
          fVar23 = 0.0;
          if (0.0 <= fVar24) {
            fVar23 = fVar24;
          }
          fVar24 = 1.0;
          if (fVar23 <= 1.0) {
            fVar24 = fVar23;
          }
          if (fVar24 <= 0.1) {
            gpu_world::OwnedGpuWorldSubset::
            update_instance_data<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
                      ();
            goto LAB_142073b72;
          }
        }
        ::utils::components::transform::impl$1::default();
        gpu_world::OwnedGpuWorldSubset::
        update_instance_data<country_core::render::nani::instance_data::PreplacedFlowerInstanceData>
                  ();
      } while( true );
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_142074230:
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  lVar7 = *extraout_RAX_02;
  if (lVar7 == 8) {
    *extraout_RAX_02 = 0;
  }
  UNLOCK();
  if (lVar7 != 8) {
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  if (bVar10) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar13 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_142074346;
    if (*plVar13 != 0) goto LAB_142074352;
    *plVar13 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar13 = *plVar13 + 1;
  }
  return;
}

