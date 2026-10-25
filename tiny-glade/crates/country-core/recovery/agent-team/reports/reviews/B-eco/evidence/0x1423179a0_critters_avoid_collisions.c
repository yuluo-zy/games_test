
void __cdecl system_critters::shared::critters_avoid_collisions(void)

{
  byte *pbVar1;
  undefined4 *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined8 uVar6;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar7;
  int iVar8;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *puVar9;
  longlong extraout_RAX_03;
  longlong lVar10;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *in_RCX;
  longlong lVar11;
  longlong *plVar12;
  undefined8 *in_RDX;
  ulonglong uVar13;
  longlong lVar14;
  longlong lVar15;
  int *piVar16;
  undefined8 *in_R8;
  undefined *puVar17;
  uint *in_R9;
  ulonglong uVar18;
  ulonglong uVar19;
  undefined8 *puVar20;
  longlong lVar21;
  uint *puVar22;
  longlong lVar23;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar24;
  float fVar25;
  float fVar26;
  undefined8 *in_stack_00000038;
  undefined8 local_288 [3];
  float local_270;
  float fStack_26c;
  float fStack_268;
  int local_258;
  undefined *local_248;
  undefined8 local_240;
  undefined8 local_208;
  undefined8 local_200;
  undefined8 local_1e8;
  float local_1e0;
  undefined8 local_1d8;
  undefined8 local_1d0;
  undefined8 *local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 local_1a8;
  undefined4 local_1a0;
  undefined8 local_198;
  undefined4 local_190;
  undefined8 local_18c;
  float local_184;
  longlong local_180;
  uint *local_170;
  longlong local_168;
  longlong local_160;
  longlong local_158;
  longlong local_150;
  longlong local_148;
  longlong local_140;
  undefined4 *local_138;
  float local_130;
  float local_12c;
  longlong *local_128;
  longlong local_120;
  longlong local_118;
  longlong local_110;
  uint *local_108;
  undefined4 local_100;
  uint local_fc;
  char local_f5;
  uint local_f4;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_180 = 0;
  }
  else {
    uVar13 = 0x32;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar13 != 0) {
        if (uVar13 < 0x32) {
          if ((char)(&DAT_14304af00)[uVar13] < -0x40) {
LAB_142317a68:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 != 0x32) goto LAB_142317a68;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar13 != 0xfffffffffffffffe)) {
        if (uVar13 + 2 < 0x32) {
          if ((char)(&UNK_14304af02)[uVar13] < -0x40) {
LAB_142317ac6:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar13 + 2 != 0x32) goto LAB_142317ac6;
      }
    }
    puVar17 = &DAT_14304b04d;
    do {
      bVar3 = puVar17[-1];
      uVar7 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar17[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar17[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar17 + -4;
            puVar17 = puVar17 + -4;
            uVar7 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar17 = puVar17 + -3;
            uVar7 = bVar5 & 0xf;
          }
          uVar7 = bVar4 & 0x3f | uVar7 << 6;
        }
        else {
          puVar17 = puVar17 + -2;
          uVar7 = bVar4 & 0x1f;
        }
        uVar7 = (uint)(bVar3 & 0x3f) | uVar7 << 6;
        if (uVar7 != 0x5c) goto LAB_142317b94;
LAB_142317bab:
        if (puVar17 == &UNK_14304b027) break;
        if (puVar17 + -0x14304b027 < (undefined *)0x25) {
          if (-0x41 < (char)puVar17[1]) break;
        }
        else if (puVar17 + -0x14304b027 == (undefined *)0x25) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar17 = puVar17 + -1;
      if (uVar7 == 0x5c) goto LAB_142317bab;
LAB_142317b94:
      if (uVar7 == 0x2f) goto LAB_142317bab;
    } while (puVar17 != &DAT_14304b028);
    local_108 = in_R9;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar12 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_142318905:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar12 != 0) {
LAB_142318911:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar12 = -1;
    local_128 = plVar12;
    puffin::ThreadProfiler::begin_scope();
    *local_128 = *local_128 + 1;
    local_180 = 1;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_02 != (undefined8 *)0x0) {
    local_248 = &anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650;
    local_240 = 0x3f;
    if ((extraout_RAX_02[3] != 0x3f) ||
       (iVar8 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                       (void *)extraout_RAX_02[2],0x3f), iVar8 != 0)) {
      local_288[0] = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    puVar9 = (undefined8 *)(**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (puVar9 != (undefined8 *)0x0) {
      local_1d0 = *puVar9;
      local_1c8 = (undefined8 *)puVar9[1];
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      puVar9 = (undefined8 *)(extraout_RAX_03 + 0x20);
      puVar20 = (undefined8 *)(extraout_RAX_03 + 0x28);
      if (*(char *)(extraout_RAX_03 + 0x298) == '\0') {
        puVar20 = (undefined8 *)(extraout_RAX_03 + 0x10);
        puVar9 = (undefined8 *)(extraout_RAX_03 + 8);
      }
      local_200 = *puVar20;
      local_208 = *puVar9;
      country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
      local_148 = *in_RCX;
      local_140 = in_RCX[1];
      local_100 = *(undefined4 *)((longlong)in_RCX + 0x14);
      puVar22 = *(uint **)(local_148 + 0xf8);
      local_170 = puVar22 + *(longlong *)(local_148 + 0x100);
      local_f5 = *(char *)(local_148 + 0x108);
      local_1b8 = *in_R8;
      local_1d8 = *in_RDX;
      local_1b0 = *in_stack_00000038;
      local_168 = 8;
      local_150 = 0;
      local_118 = 0;
      local_f4 = 0;
      local_120 = 0;
      local_128 = (longlong *)0x0;
      plVar12 = (longlong *)0x0;
      local_160 = 8;
      local_110 = 0;
      local_fc = 0;
      lVar21 = local_140;
      lVar23 = local_148;
LAB_142317e4f:
      do {
        if (local_f5 == '\0') {
          if (local_128 == plVar12) {
            do {
              if (puVar22 == local_170) goto LAB_14231881c;
              uVar7 = *puVar22;
              puVar22 = puVar22 + 1;
              plVar12 = *(longlong **)
                         (*(longlong *)(lVar21 + 0x100) + 0x58 + (ulonglong)uVar7 * 0xa0);
            } while (plVar12 == (longlong *)0x0);
            lVar10 = *(longlong *)(lVar21 + 0x100) + (ulonglong)uVar7 * 0xa0;
            uVar13 = (ulonglong)*(uint *)(lVar10 + 0x94);
            lVar11 = *(longlong *)(lVar21 + 0x1a8);
            lVar15 = lVar11 + uVar13 * 0x48;
            lVar14 = *(longlong *)(lVar11 + 0x38 + uVar13 * 0x48);
            uVar13 = *(ulonglong *)(lVar11 + 0x40 + uVar13 * 0x48);
            if (*(ulonglong *)(lVar23 + 0x110) < uVar13) {
              uVar18 = *(ulonglong *)(lVar14 + *(ulonglong *)(lVar23 + 0x110) * 8);
              lVar15 = *(longlong *)(lVar15 + 0x18);
              if (uVar18 == 0) {
                uVar18 = *(ulonglong *)(lVar23 + 0x120);
                goto joined_r0x000142317fed;
              }
              local_118 = *(longlong *)(lVar15 + 0x10 + ~uVar18 * 0x30);
              uVar18 = *(ulonglong *)(lVar23 + 0x120);
              if (uVar18 < uVar13) goto LAB_142318122;
LAB_142318157:
              local_110 = 0;
              uVar18 = *(ulonglong *)(lVar23 + 0x128);
              uVar19 = *(ulonglong *)(lVar10 + 0x88);
              if (uVar19 <= uVar18) goto LAB_1423181c0;
LAB_142318175:
              if (*(longlong *)(*(longlong *)(lVar10 + 0x80) + uVar18 * 8) == 0) goto LAB_1423181c0;
              local_f4 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar10 + 0x80) >> 8),1);
              if ((uVar13 <= uVar18) || (uVar13 = *(ulonglong *)(lVar14 + uVar18 * 8), uVar13 == 0))
              {
                local_120 = 0;
                uVar13 = *(ulonglong *)(lVar23 + 0x130);
                if (uVar13 < uVar19) goto LAB_1423181d6;
                goto LAB_142318206;
              }
              local_120 = *(longlong *)(lVar15 + 0x10 + ~uVar13 * 0x30);
              uVar13 = *(ulonglong *)(lVar23 + 0x130);
              if (uVar19 <= uVar13) goto LAB_142318206;
LAB_1423181d6:
              local_fc = (uint)CONCAT71((int7)(uVar13 >> 8),
                                        *(longlong *)(*(longlong *)(lVar10 + 0x80) + uVar13 * 8) !=
                                        0);
            }
            else {
              lVar15 = *(longlong *)(lVar15 + 0x18);
              uVar18 = *(ulonglong *)(lVar23 + 0x120);
joined_r0x000142317fed:
              local_118 = 0;
              if (uVar13 <= uVar18) goto LAB_142318157;
LAB_142318122:
              uVar18 = *(ulonglong *)(lVar14 + uVar18 * 8);
              if (uVar18 == 0) goto LAB_142318157;
              local_110 = *(longlong *)(lVar15 + 0x10 + ~uVar18 * 0x30);
              uVar18 = *(ulonglong *)(lVar23 + 0x128);
              uVar19 = *(ulonglong *)(lVar10 + 0x88);
              if (uVar18 < uVar19) goto LAB_142318175;
LAB_1423181c0:
              local_f4 = 0;
              uVar13 = *(ulonglong *)(lVar23 + 0x130);
              if (uVar13 < uVar19) goto LAB_1423181d6;
LAB_142318206:
              local_fc = 0;
            }
            lVar11 = ~*(ulonglong *)(lVar14 + *(longlong *)(lVar23 + 0x118) * 8) * 0x30;
            local_150 = *(longlong *)(lVar15 + 0x10 + lVar11);
            local_158 = *(longlong *)(lVar15 + 0x28 + lVar11);
            local_168 = *(longlong *)(lVar10 + 0x50);
            local_128 = (longlong *)0x0;
          }
          puVar9 = (undefined8 *)(local_168 + (longlong)local_128 * 0x10);
          uVar13 = (ulonglong)*(uint *)(local_168 + 8 + (longlong)local_128 * 0x10);
        }
        else {
          if (local_128 == plVar12) {
            do {
              if (puVar22 == local_170) goto LAB_14231881c;
              uVar7 = *puVar22;
              puVar22 = puVar22 + 1;
              plVar12 = *(longlong **)
                         (*(longlong *)(lVar21 + 0x1a8) + 0x10 + (ulonglong)uVar7 * 0x48);
            } while (plVar12 == (longlong *)0x0);
            lVar15 = *(longlong *)(lVar21 + 0x1a8) + (ulonglong)uVar7 * 0x48;
            lVar11 = *(longlong *)(lVar15 + 0x38);
            uVar13 = *(ulonglong *)(lVar15 + 0x40);
            if (*(ulonglong *)(lVar23 + 0x110) < uVar13) {
              uVar18 = *(ulonglong *)(lVar11 + *(ulonglong *)(lVar23 + 0x110) * 8);
              lVar14 = *(longlong *)(lVar15 + 0x18);
              if (uVar18 == 0) {
                uVar18 = *(ulonglong *)(lVar23 + 0x120);
                goto joined_r0x000142317fd0;
              }
              local_118 = *(longlong *)(lVar14 + 0x10 + ~uVar18 * 0x30);
              uVar18 = *(ulonglong *)(lVar23 + 0x120);
              if (uVar18 < uVar13) goto LAB_14231800f;
LAB_14231803d:
              local_110 = 0;
              uVar18 = *(ulonglong *)(lVar23 + 0x128);
              if (uVar13 <= uVar18) goto LAB_142318096;
LAB_142318054:
              uVar18 = *(ulonglong *)(lVar11 + uVar18 * 8);
              if (uVar18 == 0) goto LAB_142318096;
              local_120 = *(longlong *)(lVar14 + 0x10 + ~uVar18 * 0x30);
              local_f4 = (uint)CONCAT71((int7)((ulonglong)local_120 >> 8),1);
              uVar18 = *(ulonglong *)(lVar23 + 0x130);
              if (uVar13 <= uVar18) goto LAB_14231808a;
LAB_1423180ac:
              local_fc = (uint)CONCAT71((int7)(uVar13 >> 8),*(longlong *)(lVar11 + uVar18 * 8) != 0)
              ;
            }
            else {
              lVar14 = *(longlong *)(lVar15 + 0x18);
              uVar18 = *(ulonglong *)(lVar23 + 0x120);
joined_r0x000142317fd0:
              local_118 = 0;
              if (uVar13 <= uVar18) goto LAB_14231803d;
LAB_14231800f:
              uVar18 = *(ulonglong *)(lVar11 + uVar18 * 8);
              if (uVar18 == 0) goto LAB_14231803d;
              local_110 = *(longlong *)(lVar14 + 0x10 + ~uVar18 * 0x30);
              uVar18 = *(ulonglong *)(lVar23 + 0x128);
              if (uVar18 < uVar13) goto LAB_142318054;
LAB_142318096:
              local_f4 = 0;
              uVar18 = *(ulonglong *)(lVar23 + 0x130);
              if (uVar18 < uVar13) goto LAB_1423180ac;
LAB_14231808a:
              local_fc = 0;
            }
            lVar11 = ~*(ulonglong *)(lVar11 + *(longlong *)(lVar23 + 0x118) * 8) * 0x30;
            local_150 = *(longlong *)(lVar14 + 0x10 + lVar11);
            local_158 = *(longlong *)(lVar14 + 0x28 + lVar11);
            local_160 = *(longlong *)(lVar15 + 8);
            local_128 = (longlong *)0x0;
          }
          puVar9 = (undefined8 *)(local_160 + (longlong)local_128 * 8);
          uVar13 = (ulonglong)local_128 & 0xffffffff;
        }
        lVar15 = uVar13 * 0x30 + local_118;
        puVar2 = (undefined4 *)(local_158 + uVar13 * 4);
        local_108 = (uint *)(local_110 + uVar13 * 8);
        piVar16 = (int *)(uVar13 * 0x10 + local_150);
        lVar11 = uVar13 * 0x68 + local_120;
        if ((local_f4 & 1) == 0) {
          lVar11 = 0;
        }
        uVar6 = *puVar9;
        local_128 = (longlong *)((longlong)local_128 + 1);
        *puVar2 = local_100;
      } while (*piVar16 != 1);
      local_130 = (float)piVar16[1] - *(float *)(lVar15 + 0x10);
      local_12c = (float)piVar16[3] - *(float *)(lVar15 + 0x18);
      fVar25 = 1.0 / SQRT(local_12c * local_12c + local_130 * local_130);
      if ((int)fVar25 - 1U < 0x7fffff ||
          (int)ABS(fVar25) - 0x800000U < 0x7f000000 && -1 < (int)fVar25) {
        local_130 = local_130 * fVar25;
        local_12c = local_12c * fVar25;
      }
      else {
        local_12c = 0.0;
        local_130 = 0.0;
      }
      local_1c0 = uVar6;
      local_138 = puVar2;
      ::utils::geometry::circle::Circle2d::new_rotated();
      puVar9 = local_288;
      ::utils::geometry::line_relation::LineInterval::line_segment();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if ((*local_108 != 2) ||
         (puVar9 = local_1c8,
         country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz(),
         extraout_XMM0_Da <= 0.0 && extraout_AL_00 == '\0')) {
        if ((local_fc & 1) == 0) {
          if (lVar11 != 0) goto LAB_1423185c6;
          country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query();
          lVar11 = extraout_RAX_04;
          do {
            do {
              lVar21 = local_140;
              lVar23 = local_148;
              if (lVar11 == (longlong)puVar9 * 0x38 + extraout_RAX_04) goto LAB_142317e4f;
              country_core::resources::walls::public_walls::PublicWalls::get();
              if (extraout_RAX_05 == 0) {
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              lVar11 = lVar11 + 0x38;
            } while ((*local_108 != 3) &&
                    (country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
                               (), extraout_XMM0_Da_04 < *(float *)(lVar15 + 0x14)));
            ::utils::geometry::circle::Circle2d::new_rotated();
            ::utils::geometry::line_relation::LineInterval::line_segment();
            ::utils::geometry::line_relation::LineInterval::relate();
          } while ((int)local_198 != 0);
        }
        else {
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
          fVar26 = extraout_XMM0_Da_00 + extraout_XMM0_Da_01 + extraout_XMM0_Da_02 +
                   extraout_XMM0_Da_03;
          fVar25 = 0.0;
          if (0.0 <= fVar26) {
            fVar25 = fVar26;
          }
          fVar26 = 1.0;
          if (fVar25 <= 1.0) {
            fVar26 = fVar25;
          }
          if (fVar26 <= 0.0) {
LAB_1423185c6:
            if ((*local_108 < 7) &&
               (lVar21 = local_140, lVar23 = local_148, (99U >> (*local_108 & 0x1f) & 1) != 0))
            goto LAB_142317e4f;
            local_1a0 = *(undefined4 *)(lVar15 + 0x18);
            local_1a8 = *(undefined8 *)(lVar15 + 0x10);
            ::utils::swizzlers::impl$0::x0y();
            fVar25 = (float)local_1e8;
            fVar26 = (float)((ulonglong)local_1e8 >> 0x20);
            local_184 = 1.0 / SQRT(fVar26 * fVar26 + fVar25 * fVar25 + local_1e0 * local_1e0);
            local_18c = CONCAT44(local_184 * fVar26,local_184 * fVar25);
            local_184 = local_184 * local_1e0;
            local_198 = local_1a8;
            local_190 = local_1a0;
            country_core::systems::collision::world::RaycastWorld::legacy_spherecast();
            lVar21 = local_140;
            lVar23 = local_148;
            if ((local_258 == 2) ||
               (fVar25 = fStack_26c - *(float *)(lVar15 + 0x14),
               fVar26 = local_270 - *(float *)(lVar15 + 0x10),
               fVar24 = fStack_268 - *(float *)(lVar15 + 0x18),
               fVar25 = SQRT(fVar24 * fVar24 + fVar25 * fVar25 + fVar26 * fVar26), 1.0 <= fVar25))
            goto LAB_142317e4f;
            if ((fVar25 < 0.4) && ((local_fc & 1) != 0)) {
              bevy_ecs::event::writer::EventWriter::
              write<system_critters::spawn_ducks::DespawnDuckCmd>();
            }
          }
        }
      }
      *local_138 = local_100;
      *piVar16 = 0;
      lVar21 = local_140;
      lVar23 = local_148;
      goto LAB_142317e4f;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
LAB_14231881c:
  if (local_180 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 1) {
      plVar12 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar12 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_142318905;
    if (*plVar12 != 0) goto LAB_142318911;
    *plVar12 = -1;
    local_108 = (uint *)plVar12;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_108 = *(longlong *)local_108 + 1;
  }
  return;
}

