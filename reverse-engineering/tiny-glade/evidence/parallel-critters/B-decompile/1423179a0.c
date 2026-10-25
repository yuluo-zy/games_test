
void __cdecl system_critters::shared::critters_avoid_collisions(void)

{
  byte *pbVar1;
  undefined4 *puVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  undefined8 uVar6;
  float fVar7;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar8;
  int iVar9;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *extraout_RAX_02;
  undefined8 *puVar10;
  longlong extraout_RAX_03;
  longlong lVar11;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *in_RCX;
  longlong lVar12;
  longlong *plVar13;
  undefined8 *in_RDX;
  ulonglong uVar14;
  longlong lVar15;
  longlong lVar16;
  int *piVar17;
  undefined8 *in_R8;
  undefined *puVar18;
  uint *in_R9;
  ulonglong uVar19;
  ulonglong uVar20;
  undefined8 *puVar21;
  longlong lVar22;
  uint *puVar23;
  longlong lVar24;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float fVar25;
  float fVar26;
  float fVar27;
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
    uVar14 = 0x32;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar14 != 0) {
        if (uVar14 < 0x32) {
          if ((char)(&DAT_14304af00)[uVar14] < -0x40) {
LAB_142317a68:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar14 != 0x32) goto LAB_142317a68;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar14 != 0xfffffffffffffffe)) {
        if (uVar14 + 2 < 0x32) {
          if ((char)(&UNK_14304af02)[uVar14] < -0x40) {
LAB_142317ac6:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar14 + 2 != 0x32) goto LAB_142317ac6;
      }
    }
    puVar18 = &DAT_14304b04d;
    do {
      bVar3 = puVar18[-1];
      uVar8 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar18[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar18[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar18 + -4;
            puVar18 = puVar18 + -4;
            uVar8 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar18 = puVar18 + -3;
            uVar8 = bVar5 & 0xf;
          }
          uVar8 = bVar4 & 0x3f | uVar8 << 6;
        }
        else {
          puVar18 = puVar18 + -2;
          uVar8 = bVar4 & 0x1f;
        }
        uVar8 = (uint)(bVar3 & 0x3f) | uVar8 << 6;
        if (uVar8 != 0x5c) goto LAB_142317b94;
LAB_142317bab:
        if (puVar18 == &UNK_14304b027) break;
        if (puVar18 + -0x14304b027 < (undefined *)0x25) {
          if (-0x41 < (char)puVar18[1]) break;
        }
        else if (puVar18 + -0x14304b027 == (undefined *)0x25) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar18 = puVar18 + -1;
      if (uVar8 == 0x5c) goto LAB_142317bab;
LAB_142317b94:
      if (uVar8 == 0x2f) goto LAB_142317bab;
    } while (puVar18 != &DAT_14304b028);
    local_108 = in_R9;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar13 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_142318905:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar13 != 0) {
LAB_142318911:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar13 = -1;
    local_128 = plVar13;
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
       (iVar9 = memcmp(&anon_75d1013106e08612ad9ec328065f3af1_1_llvm_9468065370954387650,
                       (void *)extraout_RAX_02[2],0x3f), iVar9 != 0)) {
      local_288[0] = 0;
                    /* WARNING: Subroutine does not return */
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    puVar10 = (undefined8 *)(**(code **)(extraout_RAX_02[1] + 0x18))(*extraout_RAX_02);
    if (puVar10 != (undefined8 *)0x0) {
      local_1d0 = *puVar10;
      local_1c8 = (undefined8 *)puVar10[1];
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      puVar10 = (undefined8 *)(extraout_RAX_03 + 0x20);
      puVar21 = (undefined8 *)(extraout_RAX_03 + 0x28);
      if (*(char *)(extraout_RAX_03 + 0x298) == '\0') {
        puVar21 = (undefined8 *)(extraout_RAX_03 + 0x10);
        puVar10 = (undefined8 *)(extraout_RAX_03 + 8);
      }
      local_200 = *puVar21;
      local_208 = *puVar10;
      country_core::systems::paths::draw_path::PathRastersResources::as_float_grid();
      fVar7 = __real_3f800000;
      local_148 = *in_RCX;
      local_140 = in_RCX[1];
      local_100 = *(undefined4 *)((longlong)in_RCX + 0x14);
      puVar23 = *(uint **)(local_148 + 0xf8);
      local_170 = puVar23 + *(longlong *)(local_148 + 0x100);
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
      plVar13 = (longlong *)0x0;
      local_160 = 8;
      local_110 = 0;
      local_fc = 0;
      lVar22 = local_140;
      lVar24 = local_148;
LAB_142317e4f:
      do {
        if (local_f5 == '\0') {
          if (local_128 == plVar13) {
            do {
              if (puVar23 == local_170) goto LAB_14231881c;
              uVar8 = *puVar23;
              puVar23 = puVar23 + 1;
              plVar13 = *(longlong **)
                         (*(longlong *)(lVar22 + 0x100) + 0x58 + (ulonglong)uVar8 * 0xa0);
            } while (plVar13 == (longlong *)0x0);
            lVar11 = *(longlong *)(lVar22 + 0x100) + (ulonglong)uVar8 * 0xa0;
            uVar14 = (ulonglong)*(uint *)(lVar11 + 0x94);
            lVar12 = *(longlong *)(lVar22 + 0x1a8);
            lVar16 = lVar12 + uVar14 * 0x48;
            lVar15 = *(longlong *)(lVar12 + 0x38 + uVar14 * 0x48);
            uVar14 = *(ulonglong *)(lVar12 + 0x40 + uVar14 * 0x48);
            if (*(ulonglong *)(lVar24 + 0x110) < uVar14) {
              uVar19 = *(ulonglong *)(lVar15 + *(ulonglong *)(lVar24 + 0x110) * 8);
              lVar16 = *(longlong *)(lVar16 + 0x18);
              if (uVar19 == 0) {
                uVar19 = *(ulonglong *)(lVar24 + 0x120);
                goto joined_r0x000142317fed;
              }
              local_118 = *(longlong *)(lVar16 + 0x10 + ~uVar19 * 0x30);
              uVar19 = *(ulonglong *)(lVar24 + 0x120);
              if (uVar19 < uVar14) goto LAB_142318122;
LAB_142318157:
              local_110 = 0;
              uVar19 = *(ulonglong *)(lVar24 + 0x128);
              uVar20 = *(ulonglong *)(lVar11 + 0x88);
              if (uVar20 <= uVar19) goto LAB_1423181c0;
LAB_142318175:
              if (*(longlong *)(*(longlong *)(lVar11 + 0x80) + uVar19 * 8) == 0) goto LAB_1423181c0;
              local_f4 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar11 + 0x80) >> 8),1);
              if ((uVar14 <= uVar19) || (uVar14 = *(ulonglong *)(lVar15 + uVar19 * 8), uVar14 == 0))
              {
                local_120 = 0;
                uVar14 = *(ulonglong *)(lVar24 + 0x130);
                if (uVar14 < uVar20) goto LAB_1423181d6;
                goto LAB_142318206;
              }
              local_120 = *(longlong *)(lVar16 + 0x10 + ~uVar14 * 0x30);
              uVar14 = *(ulonglong *)(lVar24 + 0x130);
              if (uVar20 <= uVar14) goto LAB_142318206;
LAB_1423181d6:
              local_fc = (uint)CONCAT71((int7)(uVar14 >> 8),
                                        *(longlong *)(*(longlong *)(lVar11 + 0x80) + uVar14 * 8) !=
                                        0);
            }
            else {
              lVar16 = *(longlong *)(lVar16 + 0x18);
              uVar19 = *(ulonglong *)(lVar24 + 0x120);
joined_r0x000142317fed:
              local_118 = 0;
              if (uVar14 <= uVar19) goto LAB_142318157;
LAB_142318122:
              uVar19 = *(ulonglong *)(lVar15 + uVar19 * 8);
              if (uVar19 == 0) goto LAB_142318157;
              local_110 = *(longlong *)(lVar16 + 0x10 + ~uVar19 * 0x30);
              uVar19 = *(ulonglong *)(lVar24 + 0x128);
              uVar20 = *(ulonglong *)(lVar11 + 0x88);
              if (uVar19 < uVar20) goto LAB_142318175;
LAB_1423181c0:
              local_f4 = 0;
              uVar14 = *(ulonglong *)(lVar24 + 0x130);
              if (uVar14 < uVar20) goto LAB_1423181d6;
LAB_142318206:
              local_fc = 0;
            }
            lVar12 = ~*(ulonglong *)(lVar15 + *(longlong *)(lVar24 + 0x118) * 8) * 0x30;
            local_150 = *(longlong *)(lVar16 + 0x10 + lVar12);
            local_158 = *(longlong *)(lVar16 + 0x28 + lVar12);
            local_168 = *(longlong *)(lVar11 + 0x50);
            local_128 = (longlong *)0x0;
          }
          puVar10 = (undefined8 *)(local_168 + (longlong)local_128 * 0x10);
          uVar14 = (ulonglong)*(uint *)(local_168 + 8 + (longlong)local_128 * 0x10);
        }
        else {
          if (local_128 == plVar13) {
            do {
              if (puVar23 == local_170) goto LAB_14231881c;
              uVar8 = *puVar23;
              puVar23 = puVar23 + 1;
              plVar13 = *(longlong **)
                         (*(longlong *)(lVar22 + 0x1a8) + 0x10 + (ulonglong)uVar8 * 0x48);
            } while (plVar13 == (longlong *)0x0);
            lVar16 = *(longlong *)(lVar22 + 0x1a8) + (ulonglong)uVar8 * 0x48;
            lVar12 = *(longlong *)(lVar16 + 0x38);
            uVar14 = *(ulonglong *)(lVar16 + 0x40);
            if (*(ulonglong *)(lVar24 + 0x110) < uVar14) {
              uVar19 = *(ulonglong *)(lVar12 + *(ulonglong *)(lVar24 + 0x110) * 8);
              lVar15 = *(longlong *)(lVar16 + 0x18);
              if (uVar19 == 0) {
                uVar19 = *(ulonglong *)(lVar24 + 0x120);
                goto joined_r0x000142317fd0;
              }
              local_118 = *(longlong *)(lVar15 + 0x10 + ~uVar19 * 0x30);
              uVar19 = *(ulonglong *)(lVar24 + 0x120);
              if (uVar19 < uVar14) goto LAB_14231800f;
LAB_14231803d:
              local_110 = 0;
              uVar19 = *(ulonglong *)(lVar24 + 0x128);
              if (uVar14 <= uVar19) goto LAB_142318096;
LAB_142318054:
              uVar19 = *(ulonglong *)(lVar12 + uVar19 * 8);
              if (uVar19 == 0) goto LAB_142318096;
              local_120 = *(longlong *)(lVar15 + 0x10 + ~uVar19 * 0x30);
              local_f4 = (uint)CONCAT71((int7)((ulonglong)local_120 >> 8),1);
              uVar19 = *(ulonglong *)(lVar24 + 0x130);
              if (uVar14 <= uVar19) goto LAB_14231808a;
LAB_1423180ac:
              local_fc = (uint)CONCAT71((int7)(uVar14 >> 8),*(longlong *)(lVar12 + uVar19 * 8) != 0)
              ;
            }
            else {
              lVar15 = *(longlong *)(lVar16 + 0x18);
              uVar19 = *(ulonglong *)(lVar24 + 0x120);
joined_r0x000142317fd0:
              local_118 = 0;
              if (uVar14 <= uVar19) goto LAB_14231803d;
LAB_14231800f:
              uVar19 = *(ulonglong *)(lVar12 + uVar19 * 8);
              if (uVar19 == 0) goto LAB_14231803d;
              local_110 = *(longlong *)(lVar15 + 0x10 + ~uVar19 * 0x30);
              uVar19 = *(ulonglong *)(lVar24 + 0x128);
              if (uVar19 < uVar14) goto LAB_142318054;
LAB_142318096:
              local_f4 = 0;
              uVar19 = *(ulonglong *)(lVar24 + 0x130);
              if (uVar19 < uVar14) goto LAB_1423180ac;
LAB_14231808a:
              local_fc = 0;
            }
            lVar12 = ~*(ulonglong *)(lVar12 + *(longlong *)(lVar24 + 0x118) * 8) * 0x30;
            local_150 = *(longlong *)(lVar15 + 0x10 + lVar12);
            local_158 = *(longlong *)(lVar15 + 0x28 + lVar12);
            local_160 = *(longlong *)(lVar16 + 8);
            local_128 = (longlong *)0x0;
          }
          puVar10 = (undefined8 *)(local_160 + (longlong)local_128 * 8);
          uVar14 = (ulonglong)local_128 & 0xffffffff;
        }
        lVar16 = uVar14 * 0x30 + local_118;
        puVar2 = (undefined4 *)(local_158 + uVar14 * 4);
        local_108 = (uint *)(local_110 + uVar14 * 8);
        piVar17 = (int *)(uVar14 * 0x10 + local_150);
        lVar12 = uVar14 * 0x68 + local_120;
        if ((local_f4 & 1) == 0) {
          lVar12 = 0;
        }
        uVar6 = *puVar10;
        local_128 = (longlong *)((longlong)local_128 + 1);
        *puVar2 = local_100;
      } while (*piVar17 != 1);
      local_130 = (float)piVar17[1] - *(float *)(lVar16 + 0x10);
      local_12c = (float)piVar17[3] - *(float *)(lVar16 + 0x18);
      fVar26 = fVar7 / SQRT(local_12c * local_12c + local_130 * local_130);
      if ((int)fVar26 - 1U < 0x7fffff ||
          (int)ABS(fVar26) - 0x800000U < 0x7f000000 && -1 < (int)fVar26) {
        local_130 = local_130 * fVar26;
        local_12c = local_12c * fVar26;
      }
      else {
        local_12c = 0.0;
        local_130 = 0.0;
      }
      local_1c0 = uVar6;
      local_138 = puVar2;
      ::utils::geometry::circle::Circle2d::new_rotated();
      puVar10 = local_288;
      ::utils::geometry::line_relation::LineInterval::line_segment();
      ::utils::float_grid::impl$0::
      bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
      if ((*local_108 != 2) ||
         (puVar10 = local_1c8,
         country_core::resources::terrain_rock_mask::impl$1::sample_at_world_xz(),
         extraout_XMM0_Da <= 0.0 && extraout_AL_00 == '\0')) {
        if ((local_fc & 1) == 0) {
          if (lVar12 != 0) goto LAB_1423185c6;
          country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::query();
          lVar12 = extraout_RAX_04;
          do {
            do {
              lVar22 = local_140;
              lVar24 = local_148;
              if (lVar12 == (longlong)puVar10 * 0x38 + extraout_RAX_04) goto LAB_142317e4f;
              country_core::resources::walls::public_walls::PublicWalls::get();
              if (extraout_RAX_05 == 0) {
                    /* WARNING: Subroutine does not return */
                core::option::unwrap_failed();
              }
              lVar12 = lVar12 + 0x38;
            } while ((*local_108 != 3) &&
                    (country_core::resources::walls::public_wall_state::PublicWallState::max_y_gable
                               (), extraout_XMM0_Da_04 < *(float *)(lVar16 + 0x14)));
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
          fVar27 = extraout_XMM0_Da_00 + extraout_XMM0_Da_01 + extraout_XMM0_Da_02 +
                   extraout_XMM0_Da_03;
          fVar26 = 0.0;
          if (0.0 <= fVar27) {
            fVar26 = fVar27;
          }
          fVar27 = fVar7;
          if (fVar26 <= fVar7) {
            fVar27 = fVar26;
          }
          if (fVar27 <= 0.0) {
LAB_1423185c6:
            if ((*local_108 < 7) &&
               (lVar22 = local_140, lVar24 = local_148, (99U >> (*local_108 & 0x1f) & 1) != 0))
            goto LAB_142317e4f;
            local_1a0 = *(undefined4 *)(lVar16 + 0x18);
            local_1a8 = *(undefined8 *)(lVar16 + 0x10);
            ::utils::swizzlers::impl$0::x0y();
            fVar26 = (float)local_1e8;
            fVar27 = (float)((ulonglong)local_1e8 >> 0x20);
            local_184 = fVar7 / SQRT(fVar27 * fVar27 + fVar26 * fVar26 + local_1e0 * local_1e0);
            local_18c = CONCAT44(local_184 * fVar27,local_184 * fVar26);
            local_184 = local_184 * local_1e0;
            local_198 = local_1a8;
            local_190 = local_1a0;
            country_core::systems::collision::world::RaycastWorld::legacy_spherecast();
            lVar22 = local_140;
            lVar24 = local_148;
            if ((local_258 == 2) ||
               (fVar26 = fStack_26c - *(float *)(lVar16 + 0x14),
               fVar27 = local_270 - *(float *)(lVar16 + 0x10),
               fVar25 = fStack_268 - *(float *)(lVar16 + 0x18),
               fVar26 = SQRT(fVar25 * fVar25 + fVar26 * fVar26 + fVar27 * fVar27), fVar7 <= fVar26))
            goto LAB_142317e4f;
            if ((fVar26 < __real_3ecccccd) && ((local_fc & 1) != 0)) {
              bevy_ecs::event::writer::EventWriter::
              write<system_critters::spawn_ducks::DespawnDuckCmd>();
            }
          }
        }
      }
      *local_138 = local_100;
      *piVar17 = 0;
      lVar22 = local_140;
      lVar24 = local_148;
      goto LAB_142317e4f;
    }
  }
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
LAB_14231881c:
  if (local_180 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 1) {
      plVar13 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar13 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_142318905;
    if (*plVar13 != 0) goto LAB_142318911;
    *plVar13 = -1;
    local_108 = (uint *)plVar13;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_108 = *(longlong *)local_108 + 1;
  }
  return;
}

