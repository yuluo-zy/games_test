// name: stairs_signifiers_new
// addr: 0x141223d10
// size: 5524
// signature: void __cdecl stairs_signifiers_new(void)


void __cdecl system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new(void)

{
  longlong lVar1;
  undefined8 *puVar2;
  longlong *plVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  byte bVar9;
  bool bVar10;
  ulonglong *puVar11;
  float *pfVar12;
  undefined4 *puVar13;
  bool bVar14;
  uint extraout_EAX;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  uint extraout_EAX_02;
  uint extraout_EAX_03;
  undefined4 extraout_EAX_04;
  byte *extraout_RAX;
  float *extraout_RAX_00;
  float *extraout_RAX_01;
  undefined4 *extraout_RAX_02;
  ulonglong *puVar15;
  ulonglong *puVar16;
  float *extraout_RAX_03;
  longlong lVar17;
  longlong in_RCX;
  float *pfVar18;
  longlong lVar19;
  ulonglong uVar20;
  undefined4 *puVar21;
  undefined8 *puVar22;
  float *pfVar23;
  undefined8 *puVar24;
  longlong *in_R8;
  int iVar25;
  int iVar26;
  float *in_R9;
  longlong lVar27;
  undefined4 *puVar28;
  ulonglong uVar29;
  longlong lVar30;
  bool bVar31;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar32;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  float fVar44;
  float fVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  float unaff_XMM15_Da;
  undefined8 *local_368;
  longlong lStack_360;
  longlong local_358;
  longlong lStack_350;
  undefined8 local_348;
  undefined8 uStack_340;
  undefined4 local_338;
  undefined4 uStack_334;
  float fStack_32c;
  float fStack_328;
  longlong local_2e8;
  longlong local_2d0;
  float *local_2c8;
  longlong local_2c0;
  undefined8 local_2b8;
  float local_2b0;
  undefined8 local_2ac;
  float afStack_2a4 [5];
  undefined8 local_290;
  longlong local_288;
  undefined8 local_278;
  undefined8 uStack_270;
  float local_260;
  float local_25c;
  float local_258;
  float local_254;
  float local_250;
  float local_24c;
  longlong local_248;
  ulonglong *local_240;
  ulonglong *local_238;
  longlong local_230;
  undefined8 local_228;
  undefined8 uStack_220;
  longlong local_210;
  float *local_208;
  longlong local_200;
  float local_1f8;
  float local_1f4;
  float *local_1f0;
  undefined8 *local_1e8;
  float *pfStack_1e0;
  undefined8 *local_1d8;
  longlong lStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  longlong local_1a8;
  undefined8 *local_1a0;
  longlong local_198;
  ulonglong local_190;
  ulonglong local_188;
  float *local_180;
  float *local_178;
  float *local_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  float fStack_140;
  float fStack_13c;
  undefined4 local_138;
  undefined4 uStack_134;
  float fStack_130;
  float fStack_12c;
  longlong local_120;
  ulonglong *local_118;
  longlong local_110;
  ulonglong local_108;
  longlong local_100;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_1f0 = in_R9;
  local_100 = in_RCX;
  country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape();
  if (extraout_RAX != (byte *)0x0) {
    if ((*extraout_RAX & 1) == 0) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_01 == (float *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      lVar17 = *(longlong *)(local_100 + 0x98);
      extraout_RAX_01[0] = 0.0;
      extraout_RAX_01[1] = 0.0;
      *(longlong *)(extraout_RAX_01 + 2) = lVar17 + -1;
      local_2c0 = 1;
      local_228 = 1;
      local_2c8 = extraout_RAX_01;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (float *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      extraout_RAX_00[0] = 0.0;
      extraout_RAX_00[1] = 0.0;
      local_1e8 = (undefined8 *)0x1;
      local_1d8 = (undefined8 *)0x1;
      bVar31 = false;
      uVar20 = 1;
      pfStack_1e0 = extraout_RAX_00;
      while ((puVar22 = local_1d8, !bVar31 && (uVar20 < 3))) {
        bVar31 = 1 < uVar20;
        uVar20 = uVar20 + 1;
        if (bVar31) {
          uVar20 = 2;
        }
        ::utils::geometry::rectangle::Rectangle2d::get_point();
        ::utils::curve::Curve::
        get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>
                  ();
        puVar22 = local_1d8;
        if (local_1d8 == local_1e8) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(ulonglong *)(pfStack_1e0 + (longlong)puVar22 * 2) = (ulonglong)extraout_EAX;
        local_1d8 = (undefined8 *)((longlong)puVar22 + 1);
      }
      lVar17 = *(longlong *)(local_100 + 0x98);
      if (local_1d8 == local_1e8) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      }
      *(longlong *)(pfStack_1e0 + (longlong)puVar22 * 2) = lVar17 + -1;
      local_1d8 = (undefined8 *)((longlong)puVar22 + 1);
      uStack_160 = pfStack_1e0 + (longlong)puVar22 * 2 + 2;
      local_168 = (undefined **)pfStack_1e0;
      local_158 = (undefined8 **)0x0;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<tuple_<usize,usize>,core::iter::adapters::map::Map<itertools::tuple_impl::TupleWindows<core::slice::iter::Iter<usize>,tuple_<ref_<usize>,ref_<usize>_>_>,system_wall_constructor::utils::stairs_signifiers::stairs_signifiers_new::closure_env_0>_>
                ();
      if (local_1e8 != (undefined8 *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_228 = local_2d0;
    }
    local_120 = 0;
    local_118 = (ulonglong *)&DAT_00000008;
    local_110 = 0;
    local_180 = local_2c8 + local_2c0 * 4;
    local_278 = local_2c8;
    local_178 = local_2c8;
    while (local_278 != local_180) {
      uVar20 = *(ulonglong *)(local_278 + 2);
      if (uVar20 == 0xffffffffffffffff) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_end_index_overflow_fail();
      }
      local_108 = *(ulonglong *)local_278;
      if (uVar20 + 1 < local_108) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_index_order_fail();
      }
      if (*(ulonglong *)(local_100 + 0x98) <= uVar20) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_end_index_len_fail();
      }
      local_278 = local_278 + 4;
      lVar17 = *(longlong *)(local_100 + 0x90);
      bVar14 = false;
      lVar27 = lVar17 + local_108 * 0xc;
      lVar30 = 0;
      local_188 = uVar20;
      bVar31 = true;
      while (bVar10 = bVar31, lVar19 = local_110, lVar27 != lVar17 + (uVar20 + 1) * 0xc) {
        country_core::resources::terrain_height::TerrainHeightsData::
        sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        lVar19 = lVar27 + 0xc;
        lVar1 = lVar30 + 1;
        uVar29 = lVar30 + local_108;
        lVar30 = lVar1;
        if (*(float *)(lVar27 + 4) - extraout_XMM0_Da < 0.5) {
          country_core::resources::walls::public_wall_state::PublicWallState::max_y();
          fVar34 = *(float *)(lVar27 + 4);
          if (fVar34 <= 0.55) {
            fVar34 = 0.55;
          }
          if (3.0 <= extraout_XMM0_Da_00 - fVar34) goto LAB_141224190;
          bVar14 = true;
          lVar27 = lVar19;
          bVar31 = false;
          if (bVar10) {
            local_190 = 0;
            if (uVar29 != 0) {
              local_190 = uVar29 - 1;
            }
            bVar14 = true;
            bVar31 = false;
          }
        }
        else {
LAB_141224190:
          lVar1 = local_110;
          bVar14 = false;
          lVar27 = lVar19;
          bVar31 = true;
          if (!bVar10) {
            if (local_110 == local_120) {
              alloc::raw_vec::RawVec::
              grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                        ();
            }
            local_118[lVar1 * 2] = local_190;
            local_118[lVar1 * 2 + 1] = uVar29;
            local_110 = lVar1 + 1;
            bVar14 = false;
            bVar31 = true;
          }
        }
      }
      if (bVar14) {
        if (local_110 == local_120) {
          alloc::raw_vec::RawVec::
          grow_one<alloc::boxed::Box<dyn_<minidump_unwind::symbols::SymbolProvider,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>,alloc::alloc::Global>
                    ();
        }
        local_118[lVar19 * 2] = local_190;
        local_118[lVar19 * 2 + 1] = local_188;
        local_110 = lVar19 + 1;
      }
    }
    puVar21 = (undefined4 *)0x0;
    if (local_228 != 0) {
      puVar21 = (undefined4 *)(local_228 << 4);
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lVar17 = local_120;
    if (local_110 != 0) {
      local_1a8 = 0;
      local_1a0 = (undefined8 *)&DAT_00000004;
      local_198 = 0;
      local_f1 = 1;
      country_core::resources::walls::wall_holes::FinalizedWallHoles::holes_no_duplicates();
      puVar13 = extraout_RAX_02;
LAB_1412242d3:
      while ((puVar28 = puVar13, puVar28 != puVar21 && (puVar28 != (undefined4 *)0x0))) {
        puVar13 = puVar28 + 5;
        bVar9 = *(byte *)(puVar28 + 4);
        if (bVar9 - 3 < 2) goto LAB_141224340;
        if (bVar9 == 0) goto LAB_141224303;
        if (bVar9 == 1) goto LAB_141224340;
      }
      local_248 = local_120;
      puVar15 = local_118 + local_110 * 2;
      local_230 = local_100 + 0x88;
      local_238 = local_118;
      puVar11 = local_118;
      while (puVar16 = puVar11, puVar16 != puVar15) {
        if (*(ulonglong *)(local_100 + 0xb0) <= *puVar16) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (*(ulonglong *)(local_100 + 0xb0) <= puVar16[1]) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        puVar22 = *(undefined8 **)(local_100 + 0xa8);
        fVar34 = *(float *)((longlong)puVar22 + *puVar16 * 4);
        fVar45 = *(float *)((longlong)puVar22 + puVar16[1] * 4);
        puVar11 = puVar16 + 2;
        if (0.5 <= (fVar45 - fVar34) * *(float *)(local_100 + 0xb8)) {
          local_240 = puVar16;
          if (((fVar34 != 0.0) || (NAN(fVar34))) && ((fVar34 != 1.0 || (NAN(fVar34))))) {
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                      ();
            if (*(ulonglong *)(local_100 + 0x98) <= (ulonglong)extraout_EAX_00) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (*(ulonglong *)(local_100 + 0x98) <= (ulonglong)extraout_EAX_00 + 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            puVar22 = *(undefined8 **)(local_100 + 0x90);
            ::utils::curve::Curve::
            get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
          }
          if (((fVar45 != 0.0) || (NAN(fVar45))) && ((fVar45 != 1.0 || (NAN(fVar45))))) {
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                      ();
            if (*(ulonglong *)(local_100 + 0x98) <= (ulonglong)extraout_EAX_01) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (*(ulonglong *)(local_100 + 0x98) <= (ulonglong)extraout_EAX_01 + 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            puVar22 = *(undefined8 **)(local_100 + 0x90);
            ::utils::curve::Curve::
            get_approx_u_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
          }
          ::utils::subtractive_1d_boolean::SubtractiveBoolean1D::new_range();
          local_240 = local_240 + 2;
          puVar2 = local_1a0 + local_198;
          for (puVar24 = local_1a0; puVar24 != puVar2; puVar24 = puVar24 + 1) {
            local_168 = (undefined **)*puVar24;
            uStack_160 = (float *)((ulonglong)uStack_160 & 0xffffffffffffff00);
            puVar22 = &local_168;
            ::utils::subtractive_1d_boolean::SubtractiveBoolean1D::subtract();
          }
          egui::widgets::text_edit::text_buffer::impl$0::as_str();
          local_170 = extraout_RAX_03;
          while (local_170 != extraout_RAX_03 + (longlong)puVar22 * 3) {
            fVar34 = *local_170;
            local_2b8 = CONCAT44(local_2b8._4_4_,fVar34);
            if ((fVar34 < 0.0) || (1.0 < fVar34)) {
              local_1e8 = &local_2b8;
              pfStack_1e0 = (float *)core::fmt::float::impl$5::fmt;
              local_168 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
              uStack_160 = (float *)0x1;
              local_148 = 0;
              local_158 = &local_1e8;
              uStack_150 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (NAN(fVar34)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            fVar45 = local_170[1];
            local_2b8 = CONCAT44(local_2b8._4_4_,fVar45);
            if ((fVar45 < 0.0) || (1.0 < fVar45)) {
              local_1e8 = &local_2b8;
              pfStack_1e0 = (float *)core::fmt::float::impl$5::fmt;
              local_168 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
              uStack_160 = (float *)0x1;
              local_148 = 0;
              local_158 = &local_1e8;
              uStack_150 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (NAN(fVar45)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            local_180 = (float *)CONCAT44(local_180._4_4_,fVar45);
            local_188 = CONCAT44(local_188._4_4_,fVar34);
            fVar34 = (fVar45 - fVar34) * *(float *)(local_100 + 0x30);
            country_core::resources::walls::public_wall_state::PublicWallState::max_y();
            ceilf(fVar34 / 0.69);
            local_210 = 0;
            local_208 = (float *)&DAT_00000004;
            local_200 = 0;
            ::utils::random_splits_into();
            local_170 = local_170 + 3;
            local_178 = local_208 + local_200;
            local_228 = CONCAT44(fVar34,fVar34);
            uStack_220 = 0;
            bVar31 = false;
            pfVar18 = local_208;
            pfVar23 = local_178;
            fVar34 = (float)local_188;
            fVar45 = local_180._0_4_;
            fVar32 = unaff_XMM15_Da;
            iVar26 = 0;
            while (iVar25 = iVar26, unaff_XMM15_Da = fVar32, pfVar18 != pfVar23) {
              fVar40 = *pfVar18 * fVar45 + (1.0 - *pfVar18) * fVar34;
              pfVar12 = pfVar18 + 1;
              fVar32 = fVar40;
              if (!bVar31) {
                if (pfVar18 + 1 == pfVar23) break;
                pfVar12 = pfVar18 + 2;
                fVar32 = pfVar18[1] * fVar45 + (1.0 - pfVar18[1]) * fVar34;
                unaff_XMM15_Da = fVar40;
              }
              pfVar18 = pfVar12;
              iVar26 = iVar25 + 1;
              bVar31 = true;
              if (0.1 <= *(float *)(local_100 + 0xb8) * (fVar32 - unaff_XMM15_Da)) {
                local_2b8._0_4_ = unaff_XMM15_Da;
                if ((unaff_XMM15_Da < 0.0) || (1.0 < unaff_XMM15_Da)) {
                  local_1e8 = &local_2b8;
                  pfStack_1e0 = (float *)core::fmt::float::impl$5::fmt;
                  local_168 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
                  uStack_160 = (float *)0x1;
                  local_148 = 0;
                  local_158 = &local_1e8;
                  uStack_150 = 1;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_fmt();
                }
                if (NAN(unaff_XMM15_Da)) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic();
                }
                local_190 = CONCAT44(local_190._4_4_,iVar26);
                local_108 = CONCAT44(local_108._4_4_,fVar32);
                local_1f0 = pfVar18;
                ::utils::curve::Curve::
                get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                          ();
                uVar20 = (ulonglong)extraout_EAX_02;
                if (*(ulonglong *)(local_100 + 0x10) <= uVar20) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(local_100 + 0x10) <= uVar20 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                uVar7 = *(undefined8 *)(*(longlong *)(local_100 + 8) + uVar20 * 8);
                uVar8 = *(undefined8 *)(*(longlong *)(local_100 + 8) + 8 + uVar20 * 8);
                local_168 = (undefined **)
                            CONCAT44(extraout_XMM0_Da_02 * (float)((ulonglong)uVar8 >> 0x20) +
                                     (1.0 - extraout_XMM0_Da_02) * (float)((ulonglong)uVar7 >> 0x20)
                                     ,extraout_XMM0_Da_02 * (float)uVar8 +
                                      (1.0 - extraout_XMM0_Da_02) * (float)uVar7);
                country_core::resources::walls::public_wall_state::PublicWallState::max_y();
                ::utils::swizzlers::impl$1::xvy();
                local_2b8 = CONCAT44(local_2b8._4_4_,(float)local_108);
                if (((float)local_108 < 0.0) || (1.0 < (float)local_108)) {
                  local_1e8 = &local_2b8;
                  pfStack_1e0 = (float *)core::fmt::float::impl$5::fmt;
                  local_168 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
                  uStack_160 = (float *)0x1;
                  local_148 = 0;
                  local_158 = &local_1e8;
                  uStack_150 = 1;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_fmt();
                }
                if (NAN((float)local_108)) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic();
                }
                ::utils::curve::Curve::
                get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                          ();
                uVar20 = (ulonglong)extraout_EAX_03;
                if (*(ulonglong *)(local_100 + 0x10) <= uVar20) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(local_100 + 0x10) <= uVar20 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                uVar7 = *(undefined8 *)(*(longlong *)(local_100 + 8) + uVar20 * 8);
                uVar8 = *(undefined8 *)(*(longlong *)(local_100 + 8) + 8 + uVar20 * 8);
                local_168 = (undefined **)
                            CONCAT44(extraout_XMM0_Da_03 * (float)((ulonglong)uVar8 >> 0x20) +
                                     (1.0 - extraout_XMM0_Da_03) * (float)((ulonglong)uVar7 >> 0x20)
                                     ,extraout_XMM0_Da_03 * (float)uVar8 +
                                      (1.0 - extraout_XMM0_Da_03) * (float)uVar7);
                country_core::resources::walls::public_wall_state::PublicWallState::max_y();
                ::utils::swizzlers::impl$1::xvy();
                fVar48 = local_24c;
                fVar47 = local_250;
                fVar36 = local_254;
                fVar40 = local_258;
                fVar45 = local_25c;
                fVar34 = local_260;
                local_1f4 = local_260 - local_254;
                local_1f8 = local_258 - local_24c;
                fVar35 = 1.0 / SQRT(local_1f8 * local_1f8 + local_1f4 * local_1f4);
                local_1f4 = local_1f4 * fVar35;
                local_1f8 = -fVar35 * local_1f8;
                ::utils::xz_rotation_bad();
                local_278 = (float *)local_168;
                uStack_270 = uStack_160;
                ::utils::swizzlers::impl$0::x0y();
                fVar35 = (float)local_168;
                fVar37 = local_168._4_4_;
                fVar38 = (float)uStack_160;
                ::utils::swizzlers::impl$0::x0y();
                fVar33 = (float)local_108 * 0.5 + unaff_XMM15_Da * 0.5;
                fVar44 = fVar47 - fVar45;
                fVar47 = fVar45 * 0.5 + fVar47 * 0.5;
                fVar45 = fVar48 - fVar40;
                fVar40 = fVar40 * 0.5 + fVar48 * 0.5;
                fVar48 = fVar36 - fVar34;
                fVar34 = fVar34 * 0.5 + fVar36 * 0.5;
                local_2ac._0_4_ = SQRT(fVar45 * fVar45 + fVar44 * fVar44 + fVar48 * fVar48);
                local_2b8 = CONCAT44(fVar37 * 0.3 + fVar47 + -0.79,fVar34 + fVar35 * 0.3);
                local_2b0 = fVar38 * 0.3 + fVar40;
                local_2ac._4_4_ = 0x3e4ccccd;
                afStack_2a4[0] = 0.3;
                afStack_2a4[1] = fVar34 + (float)local_168 * 0.3;
                afStack_2a4[2] = local_168._4_4_ * 0.3 + fVar47 + -0.79 + -0.25;
                afStack_2a4[3] = (float)uStack_160 * 0.3 + fVar40;
                afStack_2a4[4] = (float)local_2ac * 0.4;
                fVar38 = (float)uStack_270 + (float)uStack_270;
                fVar37 = (float)uStack_270 * fVar38;
                fVar40 = local_278._4_4_ + local_278._4_4_;
                fVar48 = (float)local_278 * ((float)local_278 + (float)local_278);
                fVar35 = local_278._4_4_ * fVar40;
                fVar34 = ((float)local_278 + (float)local_278) * uStack_270._4_4_;
                fVar36 = fVar40 * uStack_270._4_4_;
                fVar39 = uStack_270._4_4_ * fVar38;
                fVar40 = fVar40 * (float)local_278;
                fVar44 = fVar38 * (float)local_278;
                fVar38 = fVar38 * local_278._4_4_;
                fVar41 = fVar44 - fVar36;
                fVar47 = fVar34 + fVar38;
                fVar45 = 1.0 - (fVar48 + fVar35);
                fVar32 = (fVar32 - unaff_XMM15_Da) * 0.5;
                local_290 = 0x3e4ccccd3e99999a;
                fVar46 = (fVar33 - fVar32) * (float)local_228;
                fVar32 = (fVar32 + fVar33) * local_228._4_4_;
                lVar17 = 0;
                while (lVar17 != 2) {
                  local_288 = lVar17 + 1;
                  fStack_130 = (&local_2b0)[lVar17 * 6];
                  fVar33 = afStack_2a4[lVar17 * 6];
                  fStack_32c = (float)*(undefined8 *)(afStack_2a4 + lVar17 * 6 + -2);
                  fStack_328 = (float)((ulonglong)*(undefined8 *)(afStack_2a4 + lVar17 * 6 + -2) >>
                                      0x20);
                  local_338 = (undefined4)(&local_2b8)[lVar17 * 3];
                  uStack_334 = (undefined4)((ulonglong)(&local_2b8)[lVar17 * 3] >> 0x20);
                  fStack_140 = fVar33 * fVar45;
                  fStack_13c = fVar33 * fVar45;
                  local_168 = (undefined **)
                              CONCAT44(fStack_32c * (fVar40 + fVar39),
                                       fStack_32c * (1.0 - (fVar35 + fVar37)));
                  uStack_160 = (float *)CONCAT44(fStack_32c * fVar41,fStack_32c * fVar41);
                  local_158 = (undefined8 **)
                              CONCAT44(fStack_328 * (1.0 - (fVar37 + fVar48)),
                                       fStack_328 * (fVar40 - fVar39));
                  uStack_150 = CONCAT44(fStack_328 * fVar47,fStack_328 * fVar47);
                  local_148 = CONCAT44(fVar33 * (fVar38 - fVar34),fVar33 * (fVar36 + fVar44));
                  local_138 = local_338;
                  uStack_134 = uStack_334;
                  fStack_12c = fStack_130;
                  country_core::resources::render::impl$1::from();
                  country_core::geometry::instanced_wall::impl$1::from();
                  local_1e8 = local_368;
                  pfStack_1e0 = (float *)lStack_360;
                  local_1d8 = (undefined8 *)local_358;
                  lStack_1d0 = lStack_350;
                  local_1c8 = local_348;
                  uStack_1c0 = uStack_340;
                  local_1b8 = 0;
                  uStack_1b0 = 0;
                  lVar27 = in_R8[2];
                  if (lVar27 == *in_R8) {
                    alloc::raw_vec::RawVec::
                    grow_one<country_core::geometry::instanced_wall::BrickTransformSsbo,alloc::alloc::Global>
                              ();
                  }
                  lVar30 = in_R8[1];
                  lVar19 = lVar27 * 0x60;
                  puVar2 = (undefined8 *)(lVar30 + 0x30 + lVar19);
                  *puVar2 = local_1b8;
                  puVar2[1] = uStack_1b0;
                  puVar2 = (undefined8 *)(lVar30 + 0x20 + lVar19);
                  *puVar2 = local_1c8;
                  puVar2[1] = uStack_1c0;
                  plVar3 = (longlong *)(lVar30 + 0x10 + lVar19);
                  *plVar3 = (longlong)local_1d8;
                  plVar3[1] = lStack_1d0;
                  *(longlong *)(lVar30 + lVar19) = (longlong)local_1e8;
                  ((longlong *)(lVar30 + lVar19))[1] = (longlong)pfStack_1e0;
                  *(undefined8 *)(lVar30 + 0x40 + lVar19) = 0xc2c80000c2c80000;
                  *(undefined4 *)(lVar30 + 0x48 + lVar19) = 0xc2c80000;
                  *(int *)(lVar30 + 0x4c + lVar19) = iVar25;
                  *(undefined4 *)(lVar30 + 0x50 + lVar19) = extraout_EAX_04;
                  *(ulonglong *)(lVar30 + 0x54 + lVar19) = CONCAT44(fVar32,fVar46);
                  *(undefined4 *)(lVar30 + 0x5c + lVar19) = 4;
                  in_R8[2] = lVar27 + 1;
                  lVar17 = lVar17 + 1;
                }
                bVar31 = true;
                pfVar18 = local_1f0;
                pfVar23 = local_178;
                fVar34 = (float)local_188;
                fVar45 = local_180._0_4_;
                fVar32 = (float)local_108;
                iVar26 = (int)local_190;
              }
            }
            if (local_210 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
          puVar11 = local_240;
          if (local_2e8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
            puVar11 = local_240;
          }
        }
      }
      lVar17 = local_1a8;
      if (local_248 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
        lVar17 = local_1a8;
      }
    }
    if (lVar17 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
LAB_141224303:
  fVar34 = (float)puVar28[1];
  fVar45 = (float)puVar28[3];
  local_f1 = 1;
  country_core::resources::walls::public_wall_state::PublicWallState::max_y();
  fVar32 = extraout_XMM0_Da_01 + -0.79 + -0.1 + -0.3;
  if ((fVar34 <= fVar32) && (fVar32 < fVar45)) {
LAB_141224340:
    lVar17 = local_198;
    uVar4 = *puVar28;
    uVar5 = puVar28[2];
    uVar6 = *(undefined4 *)(local_100 + 0x30);
    if (local_198 == local_1a8) {
      local_f1 = 1;
      alloc::raw_vec::RawVec::
      grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                ();
    }
    auVar42._4_4_ = uVar5;
    auVar42._0_4_ = uVar4;
    auVar42._8_8_ = 0;
    auVar43._4_4_ = uVar6;
    auVar43._0_4_ = uVar6;
    auVar43._8_8_ = 0;
    auVar43 = divps(auVar42,auVar43);
    local_1a0[lVar17] = auVar43._0_8_;
    local_198 = lVar17 + 1;
  }
  goto LAB_1412242d3;
}

