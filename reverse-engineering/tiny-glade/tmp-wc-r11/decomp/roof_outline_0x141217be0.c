// name: roof_outline
// addr: 0x141217be0
// size: 3968
// signature: void __cdecl roof_outline(void)


void __cdecl system_wall_constructor::utils::half_timber::roof_outline(void)

{
  undefined8 uVar1;
  longlong lVar2;
  ulonglong uVar3;
  undefined4 uVar4;
  int iVar5;
  uint extraout_EAX;
  uint extraout_EAX_00;
  int *extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong lVar6;
  int *piVar7;
  longlong in_RCX;
  ulonglong uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  bool bVar11;
  float extraout_XMM0_Da;
  float fVar12;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float fVar13;
  float fVar14;
  float fVar15;
  float fVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  float fVar31;
  longlong in_stack_00000028;
  float afStack_330 [4];
  float fStack_320;
  float fStack_31c;
  undefined1 local_318 [8];
  float fStack_310;
  float local_308;
  float fStack_304;
  float fStack_300;
  undefined4 uStack_2fc;
  undefined1 local_2f8 [16];
  undefined1 local_2e8 [16];
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  longlong local_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  longlong local_298;
  undefined8 local_290;
  undefined8 uStack_288;
  longlong local_280;
  undefined8 local_278;
  undefined8 uStack_270;
  int local_268;
  float afStack_250 [2];
  undefined8 local_248;
  float fStack_240;
  undefined8 uStack_23c;
  float fStack_234;
  longlong local_228;
  longlong local_220;
  ulonglong local_218;
  ulonglong local_208;
  int *local_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  longlong local_1e8;
  longlong local_1e0;
  ulonglong local_1d8;
  longlong local_1d0;
  undefined8 local_1b0;
  float fStack_1a8;
  float fStack_1a4;
  ulonglong local_1a0;
  ulonglong uStack_198;
  undefined8 local_190;
  undefined8 uStack_188;
  int *local_180;
  int *local_178;
  ulonglong local_170;
  undefined8 local_168;
  float fStack_160;
  undefined8 uStack_15c;
  undefined4 uStack_154;
  float local_144;
  undefined8 local_140;
  undefined8 local_138;
  float fStack_130;
  float fStack_12c;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  float local_108;
  float fStack_104;
  float fStack_100;
  float fStack_fc;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  country_core::resources::walls::public_wall_state::PublicWallState::try_get_wall_shape();
  if ((extraout_RAX == (int *)0x0) || (*extraout_RAX != 1)) {
    return;
  }
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_00 != (undefined8 *)0x0) {
    local_138._0_4_ = 96.83087;
    local_138._4_4_ = 1.4013e-45;
    fStack_130 = 8.54792e-44;
    fStack_12c = 0.0;
    if ((extraout_RAX_00[3] != 0x3d) ||
       (iVar5 = memcmp(&anon_11a7c4262c3a602e5b20874ec0ef09d1_0_llvm_11959870551130474958,
                       (void *)extraout_RAX_00[2],0x3d), iVar5 != 0)) {
      local_2b8 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar6 = (**(code **)(extraout_RAX_00[1] + 0x18))(*extraout_RAX_00);
    if (lVar6 != 0) {
      lVar2 = *(longlong *)(in_RCX + 0x180);
      if (lVar2 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      LOCK();
      bVar11 = *(char *)(lVar2 + 0x18) == '\0';
      if (bVar11) {
        *(char *)(lVar2 + 0x18) = '\x01';
      }
      UNLOCK();
      if (!bVar11) {
        parking_lot::raw_mutex::RawMutex::lock_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      piVar7 = (int *)(*(longlong *)(lVar2 + 0x10) * 4 + *(longlong *)(lVar2 + 8) + -4);
      if (piVar7 == (int *)0x0 || *(longlong *)(lVar2 + 0x10) == 0) {
        local_2b0 = *(longlong *)(lVar6 + 0x28);
        if (local_2b0 == 0) {
                    // WARNING: Subroutine does not return
          core::option::unwrap_failed();
        }
        local_268 = (int)local_2b0 + -1;
      }
      else {
        local_268 = *piVar7;
        local_2b0 = *(longlong *)(lVar6 + 0x28);
      }
      local_2b8 = *(undefined8 *)(lVar6 + 0x20);
      local_2a8 = *(undefined8 *)(lVar6 + 0x30);
      uStack_2a0 = *(undefined8 *)(lVar6 + 0x38);
      local_290 = *(undefined8 *)(lVar6 + 0x40);
      uStack_288 = *(undefined8 *)(lVar6 + 0x48);
      local_278 = *(undefined8 *)(lVar6 + 0x50);
      uStack_270 = *(undefined8 *)(lVar6 + 0x58);
      local_298 = lVar2;
      local_280 = lVar6;
      country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom();
      country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle();
      country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized();
      ::utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned();
      country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls();
      ::utils::geometry::rectangle::Rectangle2d::new_axis_aligned();
      local_208 = (ulonglong)*(byte *)(in_stack_00000028 + 0x3c);
      afStack_330[local_208] = afStack_250[local_208];
      country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
      country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
      ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
      local_180 = extraout_RAX + 1;
      fVar24 = *(float *)(in_stack_00000028 + 0x4c);
      fVar13 = fVar24 * 0.0;
      local_178 = extraout_RAX + 2;
      local_2f8 = ZEXT416((uint)fVar24);
      local_2d8 = CONCAT44(fVar24,fVar13);
      uStack_2d0 = 0;
      local_2e8 = ZEXT416((uint)fVar13);
      local_2c8 = CONCAT44(fVar13,fVar13);
      uStack_2c0 = 0;
      uVar9 = 0;
      local_200 = extraout_RAX;
      do {
        piVar7 = local_200;
        if (3 < uVar9) {
          if (local_1e8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_1d0 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          lVar6 = local_298;
          parking_lot_core::parking_lot::deadlock_impl::release_resource();
          LOCK();
          bVar11 = *(char *)(lVar6 + 0x18) == '\x01';
          if (bVar11) {
            *(char *)(lVar6 + 0x18) = '\0';
          }
          UNLOCK();
          if (bVar11) {
            return;
          }
          parking_lot::raw_mutex::RawMutex::unlock_slow();
          return;
        }
        country_core::resources::roofs::roof_shape::Roof::height();
        ::utils::geometry::rectangle::Rectangle2d::as_points2();
        fVar12 = fStack_31c;
        fVar25 = fStack_320;
        fVar13 = afStack_330[3];
        fVar24 = afStack_330[2];
        fVar14 = (float)(&local_1b0)[uVar9];
        fVar31 = (float)((ulonglong)(&local_1b0)[uVar9] >> 0x20);
        uVar1 = *(undefined8 *)(piVar7 + 3);
        ::utils::geometry::rectangle::Rectangle2d::as_points2();
        fVar15 = (float)(&local_138)[uVar9];
        fVar19 = (float)((ulonglong)(&local_138)[uVar9] >> 0x20);
        local_140 = CONCAT44(((float)((ulonglong)uVar1 >> 0x20) + fVar31 * fVar12 + fVar14 * fVar13)
                             - ((float)((ulonglong)*(undefined8 *)(piVar7 + 3) >> 0x20) +
                               fVar19 * fVar12 + fVar15 * fVar13),
                             ((float)uVar1 + fVar31 * fVar25 + fVar14 * fVar24) -
                             ((float)*(undefined8 *)(piVar7 + 3) + fVar19 * fVar25 + fVar15 * fVar24
                             ));
        ::utils::swizzlers::impl$0::x0y();
        fVar14 = fStack_130;
        fVar19 = local_138._4_4_;
        fVar15 = (float)local_138;
        ::utils::swizzlers::impl$0::x0y();
        uVar4 = (float)local_138;
        local_1f8 = CONCAT44(fStack_130,local_138._4_4_);
        uStack_1f0 = 0;
        ::utils::geometry::rectangle::Rectangle2d::as_points2();
        fVar31 = (float)(&local_138)[uVar9];
        fVar16 = (float)((ulonglong)(&local_138)[uVar9] >> 0x20);
        local_168 = CONCAT44((float)((ulonglong)*(undefined8 *)(piVar7 + 3) >> 0x20) +
                             fVar12 * fVar16 + fVar13 * fVar31,
                             (float)*(undefined8 *)(piVar7 + 3) + fVar25 * fVar16 + fVar24 * fVar31)
        ;
        ::utils::swizzlers::impl$0::x0y();
        fVar25 = fStack_1a8;
        fVar13 = local_1b0._4_4_;
        fVar24 = (float)local_1b0;
        fVar31 = extraout_XMM0_Da * 0.0;
        country_core::resources::roofs::roof_shape::profile_curve_ws();
        fVar12 = local_108;
        if (local_138 != (float *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        if (uStack_120 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        local_144 = (float)roundf(fVar12);
        if (local_144 <= 2.0) {
          local_144 = 2.0;
        }
        local_128 = (undefined8 *)0;
        if (0.0 <= local_144) {
          local_128 = (undefined8 *)
                      ((longlong)(local_144 - 9.223372e+18) & (longlong)local_144 >> 0x3f |
                      (longlong)local_144);
        }
        if (1.8446743e+19 < local_144) {
          local_128 = (undefined8 *)0xffffffffffffffff;
        }
        local_138 = &local_144;
        fStack_130 = 0.0;
        fStack_12c = 0.0;
        uStack_120 = uStack_120 & 0xffffffffffffff00;
        alloc::vec::spec_from_iter::impl$0::
        from_iter<f32,core::iter::adapters::map::Map<core::ops::range::RangeInclusive<usize>,system_wall_constructor::utils::half_timber::roof_outline::closure_env_2>_>
                  ();
        uVar3 = local_218;
        uVar10 = local_218 - 1;
        local_170 = uVar9;
        if (local_218 != 1) {
          fVar12 = extraout_XMM0_Da * fVar14 - fVar31 * fVar19;
          fVar14 = fVar31 * fVar15 - fVar14 * fVar31;
          fVar15 = fVar31 * fVar19 - fVar15 * extraout_XMM0_Da;
          fVar19 = 1.0 / SQRT(fVar15 * fVar15 + fVar12 * fVar12 + fVar14 * fVar14);
          local_318._4_4_ = (undefined4)local_1f8;
          local_318._0_4_ = uVar4;
          fStack_310 = (float)local_1f8._4_4_;
          unique0x10000867 = 0;
          local_308 = fVar24;
          fStack_304 = fVar13;
          fStack_300 = fVar25;
          uStack_2fc = 0x3f800000;
          fVar24 = fVar12 * fVar19 * 0.0;
          fVar13 = fVar19 * fVar14 * 0.0;
          fVar25 = fVar19 * fVar15 * 0.0;
          uVar9 = 0;
          local_1f8 = local_220;
          do {
            if (uVar3 <= uVar9) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar12 = *(float *)(local_1f8 + uVar9 * 4);
            local_168 = CONCAT44(local_168._4_4_,fVar12);
            if ((fVar12 < 0.0) || (1.0 < fVar12)) {
              local_1b0 = &local_168;
              fStack_1a8 = 6.774544;
              fStack_1a4 = 1.4013e-45;
              local_138._0_4_ = 96.82391;
              local_138._4_4_ = 1.4013e-45;
              fStack_130 = 1.4013e-45;
              fStack_12c = 0.0;
              local_118 = 0;
              uStack_120 = 1;
              local_128 = &local_1b0;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (NAN(fVar12)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            uVar9 = uVar9 + 1;
            if (uVar3 <= uVar9) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar12 = *(float *)(local_1f8 + uVar9 * 4);
            local_168 = CONCAT44(local_168._4_4_,fVar12);
            if ((fVar12 < 0.0) || (1.0 < fVar12)) {
              local_1b0 = &local_168;
              fStack_1a8 = 6.774544;
              fStack_1a4 = 1.4013e-45;
              local_138._0_4_ = 96.82391;
              local_138._4_4_ = 1.4013e-45;
              fStack_130 = 1.4013e-45;
              fStack_12c = 0.0;
              local_118 = 0;
              uStack_120 = 1;
              local_128 = &local_1b0;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (NAN(fVar12)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                      ();
            uVar8 = (ulonglong)extraout_EAX;
            if (local_1d8 <= uVar8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (local_1d8 <= uVar8 + 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar12 = *(float *)(local_1e0 + uVar8 * 8);
            fVar15 = *(float *)(local_1e0 + 4 + uVar8 * 8);
            fVar19 = *(float *)(local_1e0 + 8 + uVar8 * 8);
            fVar14 = *(float *)(local_1e0 + 0xc + uVar8 * 8);
            ::utils::curve::Curve::
            get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                      ();
            uVar8 = (ulonglong)extraout_EAX_00;
            if (local_1d8 <= uVar8) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (local_1d8 <= uVar8 + 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar19 = fVar12 * (1.0 - extraout_XMM0_Da_00) + fVar19 * extraout_XMM0_Da_00;
            fVar15 = (1.0 - extraout_XMM0_Da_00) * fVar15 + fVar14 * extraout_XMM0_Da_00;
            fVar14 = *(float *)(local_1e0 + 8 + uVar8 * 8) * extraout_XMM0_Da_01 +
                     *(float *)(local_1e0 + uVar8 * 8) * (1.0 - extraout_XMM0_Da_01);
            fVar12 = *(float *)(local_1e0 + 0xc + uVar8 * 8) * extraout_XMM0_Da_01 +
                     (1.0 - extraout_XMM0_Da_01) * *(float *)(local_1e0 + 4 + uVar8 * 8);
            fVar26 = (float)local_2d8 +
                     fVar19 * (float)local_318._0_4_ + fVar15 * fVar31 + fVar24 + local_308;
            fVar27 = local_2d8._4_4_ +
                     fVar19 * (float)local_318._4_4_ + fVar15 * extraout_XMM0_Da + fVar13 +
                     fStack_304;
            fVar28 = fVar19 * fStack_310 + fVar15 * fVar31 + fVar25 + fStack_300 +
                     (float)local_2e8._0_4_;
            fVar15 = fVar14 * (float)local_318._4_4_ + fVar12 * extraout_XMM0_Da + fVar13 +
                     fStack_304 + (float)local_2f8._0_4_;
            fVar29 = fVar14 * (float)local_318._0_4_ + fVar12 * fVar31 + fVar24 + local_308 +
                     (float)local_2c8;
            fVar30 = fVar14 * fStack_310 + fVar12 * fVar31 + fVar25 + fStack_300 + local_2c8._4_4_;
            fVar12 = fVar28 - fVar30;
            local_1b0._0_4_ = fVar26 - fVar29;
            local_1b0._4_4_ = fVar27 - fVar15;
            fVar19 = SQRT(local_1b0._4_4_ * local_1b0._4_4_ + (float)local_1b0 * (float)local_1b0 +
                          fVar12 * fVar12);
            fStack_1a8 = 1.0 / fVar19;
            local_1b0._0_4_ = fStack_1a8 * (float)local_1b0;
            local_1b0._4_4_ = fStack_1a8 * local_1b0._4_4_;
            fStack_1a8 = fStack_1a8 * fVar12;
            ::utils::rotation_from_normal();
            fStack_240 = (fVar30 + fVar28) * 0.5;
            fStack_234 = fVar19 + 0.05;
            local_108 = (fVar29 + fVar26) * 0.5;
            fStack_104 = (fVar15 + fVar27) * 0.5;
            local_248 = CONCAT44(fStack_104,local_108);
            uStack_23c = 0x3e8000003ecccccd;
            fVar23 = fStack_130 + fStack_130;
            fVar15 = fStack_130 * fVar23;
            fVar16 = local_138._4_4_ + local_138._4_4_;
            fVar12 = fVar16 * (float)local_138;
            fVar21 = fVar23 * (float)local_138;
            fVar22 = fVar23 * local_138._4_4_;
            fVar20 = (float)local_138 * ((float)local_138 + (float)local_138);
            fVar19 = ((float)local_138 + (float)local_138) * fStack_12c;
            fVar17 = fVar16 * fStack_12c;
            fVar23 = fStack_12c * fVar23;
            fStack_12c = fVar21 - fVar17;
            fVar18 = fVar19 + fVar22;
            fVar14 = 1.0 - (fVar20 + local_138._4_4_ * fVar16);
            local_138._0_4_ = (1.0 - (local_138._4_4_ * fVar16 + fVar15)) * 0.4;
            local_138._4_4_ = (fVar12 + fVar23) * 0.4;
            fStack_130 = fStack_12c * 0.4;
            fStack_12c = fStack_12c * 0.4;
            local_128 = (undefined8 *)
                        CONCAT44((1.0 - (fVar15 + fVar20)) * 0.25,(fVar12 - fVar23) * 0.25);
            uStack_120 = CONCAT44(fVar18 * 0.25,fVar18 * 0.25);
            local_118 = CONCAT44(fStack_234 * (fVar22 - fVar19),fStack_234 * (fVar17 + fVar21));
            uStack_110 = CONCAT44(fStack_234 * fVar14,fStack_234 * fVar14);
            fStack_100 = fStack_240;
            fStack_fc = fStack_240;
            country_core::resources::render::impl$1::from();
            minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
                      ();
            minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
                      ();
            local_118 = local_190;
            uStack_110 = uStack_188;
            local_128 = (undefined8 *)local_1a0;
            uStack_120 = uStack_198;
            local_138._0_4_ = (float)local_1b0;
            local_138._4_4_ = local_1b0._4_4_;
            fStack_130 = fStack_1a8;
            fStack_12c = fStack_1a4;
            local_108 = 0.0;
            fStack_fc = 1.4013e-45;
            country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
            push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
            fVar29 = fVar29 - fVar26;
            fVar30 = fVar30 - fVar28;
            local_1b0._4_4_ = 1.0 / SQRT(fVar30 * fVar30 + fVar29 * fVar29);
            local_1b0._0_4_ = local_1b0._4_4_ * fVar29;
            local_1b0._4_4_ = local_1b0._4_4_ * fVar30;
            ::utils::swizzlers::impl$0::x0y();
            fVar19 = fStack_130;
            fVar15 = local_138._4_4_;
            fVar12 = (float)local_138;
            ::utils::xz_rotation_bad();
            fStack_160 = fVar28 + fVar19 * 0.25;
            local_108 = fVar26 + fVar12 * 0.25;
            fStack_104 = fVar27 + fVar15 * 0.25;
            local_168 = CONCAT44(fStack_104,local_108);
            uStack_15c = 0x3e4000003e400000;
            uStack_154 = 0x3f333333;
            fVar23 = fStack_130 + fStack_130;
            fVar15 = fStack_130 * fVar23;
            fVar16 = local_138._4_4_ + local_138._4_4_;
            fVar12 = fVar16 * (float)local_138;
            fVar21 = fVar23 * (float)local_138;
            fVar22 = fVar23 * local_138._4_4_;
            fVar20 = (float)local_138 * ((float)local_138 + (float)local_138);
            fVar14 = ((float)local_138 + (float)local_138) * fStack_12c;
            fVar17 = fVar16 * fStack_12c;
            fVar23 = fStack_12c * fVar23;
            fStack_12c = fVar21 - fVar17;
            fVar18 = fVar14 + fVar22;
            fVar19 = 1.0 - (fVar20 + local_138._4_4_ * fVar16);
            local_138._0_4_ = (1.0 - (local_138._4_4_ * fVar16 + fVar15)) * 0.1875;
            local_138._4_4_ = (fVar12 + fVar23) * 0.1875;
            fStack_130 = fStack_12c * 0.1875;
            fStack_12c = fStack_12c * 0.1875;
            local_128 = (undefined8 *)
                        CONCAT44((1.0 - (fVar15 + fVar20)) * 0.1875,(fVar12 - fVar23) * 0.1875);
            uStack_120 = CONCAT44(fVar18 * 0.1875,fVar18 * 0.1875);
            local_118 = CONCAT44((fVar22 - fVar14) * 0.7,(fVar17 + fVar21) * 0.7);
            uStack_110 = CONCAT44(fVar19 * 0.7,fVar19 * 0.7);
            fStack_100 = fStack_160;
            fStack_fc = fStack_160;
            country_core::resources::render::impl$1::from();
            minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
                      ();
            minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction
                      ();
            local_118 = local_190;
            uStack_110 = uStack_188;
            local_128 = (undefined8 *)local_1a0;
            uStack_120 = uStack_198;
            local_138._0_4_ = (float)local_1b0;
            local_138._4_4_ = local_1b0._4_4_;
            fStack_130 = fStack_1a8;
            fStack_12c = fStack_1a4;
            local_108 = 0.0;
            fStack_fc = 1.4013e-45;
            country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
            push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
          } while (uVar9 < uVar10);
        }
        if (local_228 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        uVar9 = local_170 + 1;
      } while( true );
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

