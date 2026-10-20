// name: add_wall_corners
// addr: 0x141215540
// size: 7327
// signature: void __cdecl add_wall_corners(void)


void __cdecl system_wall_constructor::utils::wall_corners::add_wall_corners(void)

{
  undefined8 *puVar1;
  longlong *plVar2;
  char cVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 uVar6;
  undefined1 auVar7 [16];
  float fVar8;
  float fVar9;
  float fVar10;
  char extraout_AL;
  char extraout_AL_00;
  undefined4 extraout_EAX;
  uint extraout_EAX_00;
  undefined4 uVar11;
  undefined4 extraout_EAX_01;
  uint extraout_EAX_02;
  ulonglong uVar12;
  longlong extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 uVar13;
  undefined8 *extraout_RAX_01;
  float *extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  longlong in_RCX;
  ulonglong *puVar14;
  longlong lVar15;
  longlong lVar16;
  ulonglong uVar17;
  undefined8 *puVar18;
  bool bVar19;
  longlong in_R8;
  ulonglong uVar20;
  ulonglong in_R9;
  undefined8 *puVar21;
  undefined8 *puVar22;
  ulonglong uVar23;
  undefined8 *unaff_R13;
  undefined8 *puVar24;
  float *pfVar25;
  float *pfVar26;
  longlong lVar27;
  undefined4 extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  float fVar28;
  float extraout_XMM0_Da_14;
  float extraout_XMM0_Da_15;
  float extraout_XMM0_Da_16;
  float extraout_XMM0_Da_17;
  float extraout_XMM0_Da_18;
  float extraout_XMM0_Da_19;
  float extraout_XMM0_Da_20;
  float fVar29;
  float extraout_XMM0_Da_21;
  float extraout_XMM0_Da_22;
  undefined4 extraout_XMM0_Da_23;
  undefined4 extraout_XMM0_Da_24;
  float extraout_XMM0_Da_25;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  uint uVar39;
  uint uVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  longlong in_stack_00000028;
  ulonglong in_stack_00000030;
  longlong *in_stack_00000038;
  char in_stack_00000048;
  undefined8 *in_stack_00000050;
  longlong local_458;
  ulonglong *puStack_450;
  longlong local_448;
  longlong lStack_440;
  undefined8 local_438;
  undefined8 uStack_430;
  float local_428;
  float fStack_424;
  float fStack_420;
  float fStack_41c;
  float local_418;
  float fStack_414;
  float fStack_410;
  float fStack_40c;
  float local_408;
  undefined4 local_398;
  undefined4 uStack_394;
  undefined4 uStack_390;
  undefined4 uStack_38c;
  undefined5 uStack_388;
  longlong local_358;
  float *local_350;
  longlong local_348;
  undefined8 local_340;
  ulonglong *puStack_338;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  longlong local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  longlong lStack_2a0;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 local_278;
  undefined8 uStack_270;
  ulonglong local_268;
  ulonglong uStack_260;
  longlong local_258;
  float *local_250;
  longlong local_248;
  undefined8 *local_240;
  ulonglong local_238;
  longlong local_230;
  ulonglong *local_220;
  longlong local_218;
  ulonglong *local_210;
  float fStack_204;
  undefined4 local_1fc;
  float local_1f8;
  undefined4 local_1f4;
  longlong local_1f0;
  longlong lStack_1e8;
  longlong local_1e0;
  ulonglong local_1d8;
  ulonglong *local_1d0;
  ulonglong local_1c8;
  float local_1bc;
  float local_1b8;
  float local_1b4;
  float local_1b0;
  undefined4 uStack_1ac;
  ulonglong local_1a8;
  undefined8 *local_1a0;
  longlong local_198;
  ulonglong uStack_190;
  undefined4 local_188;
  undefined4 local_184;
  ulonglong local_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined4 local_168;
  undefined4 uStack_164;
  undefined4 uStack_160;
  undefined4 uStack_15c;
  undefined5 uStack_158;
  char local_153;
  undefined2 local_152;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  float local_128;
  undefined4 uStack_124;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  longlong local_100;
  float local_f8;
  char local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_248 = in_R8;
  local_1d8 = in_R9;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,glam::f32::vec2::Vec2_(*)(glam::f32::vec3::Vec3)>_>
            ();
  ::utils::curve::Curve::
  try_new_from_points<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
  if (SBORROW8(0,local_148)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  local_288 = local_118;
  uStack_290 = uStack_120;
  local_2a8 = local_138;
  lStack_2a0 = uStack_130;
  local_2b8 = local_148;
  uStack_2b0 = uStack_140;
  uVar4 = *(undefined8 *)(in_RCX + 0xf8);
  country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
  puVar24 = *(undefined8 **)(in_RCX + 8);
  puVar18 = *(undefined8 **)(in_RCX + 0x10);
  local_148 = 0;
  uStack_140 = (ulonglong *)&DAT_00000008;
  local_138 = 0;
  local_188 = extraout_EAX;
  local_184 = extraout_XMM0_Da;
  if (*(int *)(in_RCX + 600) != 2) {
    if (puVar18 < (undefined8 *)0x2) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    fVar34 = (float)*puVar24;
    fVar30 = (float)puVar24[(longlong)puVar18 + -2] - fVar34;
    fVar35 = (float)((ulonglong)*puVar24 >> 0x20);
    fVar32 = (float)((ulonglong)puVar24[(longlong)puVar18 + -2] >> 0x20) - fVar35;
    fVar34 = fVar34 - (float)puVar24[1];
    fVar35 = fVar35 - (float)((ulonglong)puVar24[1] >> 0x20);
    fVar30 = (fVar32 * fVar35 + fVar30 * fVar34) /
             SQRT((fVar34 * fVar34 + fVar35 * fVar35) * (fVar32 * fVar32 + fVar30 * fVar30));
    if ((uint)ABS(fVar30) < 0x7f800000) {
      local_100 = 0;
      if ((-0.1736482 <= fVar30) && (fVar30 < 0.17364822)) {
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        *uStack_140 = 0;
        local_138 = 1;
        local_100 = 1;
      }
      goto LAB_14121572e;
    }
  }
  local_100 = 0;
LAB_14121572e:
  puVar1 = puVar24 + (longlong)puVar18;
  bVar19 = true;
  uVar23 = 0;
  fVar30 = 0.17364822;
  local_218 = local_148;
  puVar14 = uStack_140;
  while (local_148 = local_218, uStack_140 = puVar14, puVar24 != puVar1) {
    puVar21 = puVar24 + 1;
    if (bVar19) {
      if ((puVar21 == puVar1) || (puVar24 + 2 == puVar1)) break;
      unaff_R13 = puVar21;
      puVar18 = puVar24;
      puVar21 = puVar24 + 3;
      puVar24 = puVar24 + 2;
    }
    fVar32 = (float)*unaff_R13;
    fVar34 = (float)((ulonglong)*unaff_R13 >> 0x20);
    fVar35 = (float)*puVar24 - fVar32;
    fVar36 = (float)((ulonglong)*puVar24 >> 0x20) - fVar34;
    fVar32 = fVar32 - (float)*puVar18;
    fVar34 = fVar34 - (float)((ulonglong)*puVar18 >> 0x20);
    fVar32 = (fVar34 * fVar36 + fVar32 * fVar35) /
             SQRT((fVar36 * fVar36 + fVar35 * fVar35) * (fVar34 * fVar34 + fVar32 * fVar32));
    if (((0x7f7fffff < (uint)ABS(fVar32)) || (fVar32 < -0.1736482)) || (0.17364822 <= fVar32)) {
      uVar23 = uVar23 + 2;
      while( true ) {
        puVar22 = puVar21;
        if (puVar22 == puVar1) goto LAB_141215919;
        fVar32 = (float)*puVar24;
        fVar34 = (float)((ulonglong)*puVar24 >> 0x20);
        fVar35 = (float)*puVar22 - fVar32;
        fVar36 = (float)((ulonglong)*puVar22 >> 0x20) - fVar34;
        fVar32 = fVar32 - (float)*unaff_R13;
        fVar34 = fVar34 - (float)((ulonglong)*unaff_R13 >> 0x20);
        fVar32 = (fVar34 * fVar36 + fVar32 * fVar35) /
                 SQRT((fVar36 * fVar36 + fVar35 * fVar35) * (fVar34 * fVar34 + fVar32 * fVar32));
        if ((((uint)ABS(fVar32) < 0x7f800000) && (-0.1736482 <= fVar32)) && (fVar32 < 0.17364822))
        break;
        uVar23 = uVar23 + 1;
        unaff_R13 = puVar24;
        puVar21 = puVar22 + 1;
        puVar24 = puVar22;
      }
      puVar21 = puVar22 + 1;
      puVar18 = puVar24;
    }
    else {
      uVar23 = uVar23 + 1;
      puVar22 = puVar24;
      puVar18 = unaff_R13;
    }
    unaff_R13 = puVar22;
    puVar24 = puVar21;
    if (local_100 == local_218) {
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    }
    uStack_140[local_100] = uVar23;
    local_138 = local_100 + 1;
    bVar19 = false;
    local_218 = local_148;
    puVar14 = uStack_140;
    local_100 = local_138;
  }
LAB_141215919:
  local_1f0 = 0;
  lStack_1e8 = 4;
  local_1e0 = 0;
  local_220 = puVar14 + local_100;
  local_f2 = *(char *)((longlong)in_stack_00000050 + 0x14);
  local_210 = puVar14;
  do {
    local_1d0 = puVar14;
    if (puVar14 == local_220) {
      if (local_218 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_138 = local_1e0;
      local_148 = local_1f0;
      uStack_140 = (ulonglong *)lStack_1e8;
      country_core::resources::walls::wall_holes::InProgressHoleStorage::
      extend<alloc::vec::Vec<tuple_<utils::wall_space::WallSpaceRegion,country_core::resources::walls::wall_holes::HoleType>,alloc::alloc::Global>_>
                ();
      if (local_2b8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (lStack_2a0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
    uVar23 = *(ulonglong *)(in_RCX + 0x10);
    if (uVar23 == 0) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_const::panic_const_rem_by_zero();
    }
    uVar17 = *puVar14;
    uVar20 = uVar17 - 1;
    if (uVar17 == 0) {
      uVar20 = uVar23 - 2;
    }
    if (uVar23 <= uVar17) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    uVar12 = uVar17 + 1;
    if (uVar12 == uVar23) {
      uVar12 = 0;
    }
    if (uVar23 <= uVar20) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    if (*(ulonglong *)(in_RCX + 0x28) <= uVar17) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    lVar16 = *(longlong *)(in_RCX + 8);
    uVar13 = *(undefined8 *)(lVar16 + uVar17 * 8);
    fVar32 = (float)uVar13;
    fVar34 = (float)((ulonglong)uVar13 >> 0x20);
    uVar13 = *(undefined8 *)(lVar16 + uVar12 * 8);
    local_278 = *(undefined8 *)(lVar16 + uVar20 * 8);
    uStack_270 = 0;
    ::utils::curve::Curve::
    get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
              ();
    uVar23 = (ulonglong)extraout_EAX_00;
    if (local_1d8 <= uVar23) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    uVar20 = uVar23 + 1;
    if (local_1d8 <= uVar20) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    fVar35 = *(float *)(local_248 + uVar23 * 4);
    fVar35 = (*(float *)(local_248 + 4 + uVar23 * 4) - fVar35) * extraout_XMM0_Da_00 + fVar35;
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    local_100 = CONCAT44(local_100._4_4_,fVar35);
    local_1bc = extraout_XMM0_Da_01;
    if (in_stack_00000048 == '\0') {
      if (uVar17 == 0) {
        if (in_stack_00000030 <= uVar23) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (in_stack_00000030 <= uVar20) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        local_180 = CONCAT44(local_180._4_4_,*(undefined4 *)(in_stack_00000028 + uVar23 * 4));
        fVar35 = *(float *)(in_stack_00000028 + 4 + uVar23 * 4);
        ::utils::curve::Curve::
        get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                  ();
        uVar23 = (ulonglong)extraout_EAX_02;
        if (in_stack_00000030 <= uVar23) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (in_stack_00000030 <= uVar23 + 1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar37 = extraout_XMM0_Da_00 * (fVar35 - (float)local_180) + (float)local_180;
        fVar35 = *(float *)(in_stack_00000028 + uVar23 * 4);
        fVar35 = (*(float *)(in_stack_00000028 + 4 + uVar23 * 4) - fVar35) * extraout_XMM0_Da_25 +
                 fVar35;
        fVar36 = fVar35;
        if (fVar35 <= fVar37) {
          fVar36 = fVar37;
        }
        local_f8 = (float)(~-(uint)NAN(fVar37) & (uint)fVar36 | -(uint)NAN(fVar37) & (uint)fVar35);
      }
      else {
        if (in_stack_00000030 <= uVar23) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (in_stack_00000030 <= uVar20) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        local_f8 = *(float *)(in_stack_00000028 + uVar23 * 4);
        local_f8 = extraout_XMM0_Da_00 * (*(float *)(in_stack_00000028 + 4 + uVar23 * 4) - local_f8)
                   + local_f8;
      }
    }
    else {
      if (in_stack_00000030 <= uVar23) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (in_stack_00000030 <= uVar20) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      fVar35 = *(float *)(in_stack_00000028 + uVar23 * 4);
      local_f8 = extraout_XMM0_Da_00 * (*(float *)(in_stack_00000028 + 4 + uVar23 * 4) - fVar35) +
                 fVar35 + -0.46;
    }
    uVar23 = in_stack_00000030;
    egui::widgets::text_edit::text_buffer::impl$0::as_str();
    if (uVar23 == 0) {
      local_230 = 0;
    }
    else {
      lVar16 = 0;
      local_230 = 0;
      do {
        if (ABS(local_1bc - *(float *)(extraout_RAX + lVar16)) < 0.01) {
          fVar35 = *(float *)(extraout_RAX + 4 + lVar16);
          uVar20 = 0;
          if (0.0 <= fVar35) {
            uVar20 = (longlong)(fVar35 - 9.223372e+18) & (longlong)fVar35 >> 0x3f | (longlong)fVar35
            ;
          }
          if (1.8446743e+19 < fVar35) {
            uVar20 = 0xffffffffffffffff;
          }
          local_230 = local_230 + uVar20;
        }
        lVar16 = lVar16 + 0x14;
      } while (uVar23 * 0x14 != lVar16);
    }
    local_258 = uVar17 + local_230;
    fVar35 = ceilf((local_f8 - (float)local_100) / 0.69);
    uVar23 = 0;
    if (0.0 <= fVar35) {
      uVar23 = (longlong)(fVar35 - 9.223372e+18) & (longlong)fVar35 >> 0x3f | (longlong)fVar35;
    }
    if (1.8446743e+19 < fVar35) {
      uVar23 = 0xffffffffffffffff;
    }
    if (uVar23 < 3) {
      uVar23 = 2;
    }
    ::utils::random_splits();
    puVar14 = local_1d0 + 1;
    fVar41 = (float)uVar13 - fVar32;
    fVar43 = (float)((ulonglong)uVar13 >> 0x20) - fVar34;
    fVar35 = 1.0 / SQRT(fVar43 * fVar43 + fVar41 * fVar41);
    fVar32 = fVar32 - (float)local_278;
    fVar34 = fVar34 - local_278._4_4_;
    fVar37 = 0.0 - uStack_270._4_4_;
    fVar36 = 1.0 / SQRT(fVar34 * fVar34 + fVar32 * fVar32);
    local_250 = local_350 + local_348;
    uVar23 = uVar23 - 2;
    local_180 = 0;
    local_278 = 0;
    pfVar25 = local_350;
    local_1c8 = uVar23;
    while (uVar17 = local_278, pfVar25 != local_250) {
      pfVar26 = pfVar25 + 1;
      fVar44 = *pfVar25 * local_f8 + (1.0 - *pfVar25) * (float)local_100;
      fVar45 = fVar30;
      if ((local_180 & 1) == 0) {
        if (pfVar26 == local_250) break;
        pfVar26 = pfVar25 + 2;
        fVar45 = fVar44;
        fVar44 = pfVar25[1] * local_f8 + (1.0 - pfVar25[1]) * (float)local_100;
      }
      fVar30 = fVar44;
      fastrand::Rng::f32();
      fVar44 = (extraout_XMM0_Da_02 * 0.16 - (1.0 - extraout_XMM0_Da_02) * 0.16) + 0.92;
      if ((uVar17 == uVar23) && (*(int *)(in_RCX + 600) != 2)) {
        bVar19 = true;
        fVar44 = 0.76;
      }
      else {
        bVar19 = false;
      }
      local_278 = uVar17 + 1;
      local_180 = CONCAT71((int7)(local_278 >> 8),1);
      pfVar25 = pfVar26;
      if ((0.0 < fVar30 - fVar45) && (0.0 < fVar44)) {
        if (local_f2 != '\x02') {
          country_core::resources::walls::public_wall_state::PublicWallState::max_y();
        }
        ::utils::wall_space::impl$1::project_clamp();
        ::utils::wall_space::WallSpaceCoord::with_y();
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        lVar16 = 4;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_01 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        local_148 = local_340;
        uStack_140 = puStack_338;
        local_138 = 0;
        uStack_130 = 0;
        extraout_RAX_01[2] = 0;
        extraout_RAX_01[3] = 0;
        *extraout_RAX_01 = local_340;
        extraout_RAX_01[1] = puStack_338;
        *(undefined4 *)(extraout_RAX_01 + 4) = 0;
        *(undefined2 *)((longlong)extraout_RAX_01 + 0x24) = 0;
        local_198 = 0;
        uStack_190 = 1;
        local_1a8 = 1;
        if (bVar19) {
          local_f1 = 1;
          ::utils::geometry::aabb::Aabb2::from_center_dims();
          local_f1 = 1;
          egui::widgets::text_edit::text_buffer::impl$0::as_str();
          pfVar26 = extraout_RAX_02;
          do {
            if (pfVar26 == extraout_RAX_02 + lVar16 * 5) {
              break;
            }
            fVar45 = *pfVar26;
            local_f1 = 1;
            ::utils::geometry::aabb::Aabb2::from_center_dims();
            local_f1 = 1;
            ::utils::geometry::aabb::Aabb2::center();
            if (((float)local_2f8 <= extraout_XMM0_Da_03) &&
               (extraout_XMM0_Da_03 < (float)uStack_2f0)) {
              local_f1 = 1;
              ::utils::geometry::aabb::Aabb2::center();
              local_f1 = 1;
              fVar44 = fVar45;
              ::utils::geometry::aabb::Aabb2::center();
              if (fVar45 <= fVar44) {
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                local_f1 = 1;
                fVar45 = fVar44;
                ::utils::geometry::aabb::Aabb2::center();
                if (fVar44 < fVar45) {
                  fVar45 = uStack_2f0._4_4_;
                  if (uStack_2f0._4_4_ <= local_178._4_4_) {
                    fVar45 = local_178._4_4_;
                  }
                  local_178 = CONCAT44(~-(uint)NAN(local_178._4_4_) & (uint)fVar45 |
                                       -(uint)NAN(local_178._4_4_) & (uint)uStack_2f0._4_4_,
                                       (undefined4)local_178);
                }
              }
              else {
                fVar45 = local_2f8._4_4_;
                if (uStack_170._4_4_ <= local_2f8._4_4_) {
                  fVar45 = uStack_170._4_4_;
                }
                uStack_170 = (ulonglong *)
                             CONCAT44(~-(uint)NAN(uStack_170._4_4_) & (uint)fVar45 |
                                      -(uint)NAN(uStack_170._4_4_) & (uint)local_2f8._4_4_,
                                      (undefined4)uStack_170);
              }
            }
            if (*(int *)(in_RCX + 600) != 2) {
              local_f1 = 1;
              ::utils::geometry::aabb::Aabb2::center();
              local_f1 = 1;
              ::utils::geometry::aabb::Aabb2::width();
              fVar45 = extraout_XMM0_Da_04 + extraout_XMM0_Da_05 * 0.5;
              if (fVar45 < *(float *)(in_RCX + 0x30) || fVar45 == *(float *)(in_RCX + 0x30)) {
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::width();
                if (extraout_XMM0_Da_07 < extraout_XMM0_Da_08 * 0.5) {
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  fVar45 = *(float *)(in_RCX + 0x30);
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  fVar45 = extraout_XMM0_Da_09 + fVar45;
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::from_center_dims();
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  local_f1 = 1;
                  fVar44 = fVar45;
                  ::utils::geometry::aabb::Aabb2::center();
                  if (fVar44 < fVar45) goto LAB_1412160a6;
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  local_f1 = 1;
                  fVar45 = fVar44;
                  ::utils::geometry::aabb::Aabb2::center();
                  goto LAB_14121640b;
                }
              }
              else {
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                fVar45 = *(float *)(in_RCX + 0x30);
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                fVar45 = extraout_XMM0_Da_06 - fVar45;
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::from_center_dims();
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                local_f1 = 1;
                fVar44 = fVar45;
                ::utils::geometry::aabb::Aabb2::center();
                if (fVar44 < fVar45) {
LAB_1412160a6:
                  fVar45 = local_148._4_4_;
                  if (uStack_170._4_4_ <= local_148._4_4_) {
                    fVar45 = uStack_170._4_4_;
                  }
                  uStack_170 = (ulonglong *)
                               CONCAT44(~-(uint)NAN(uStack_170._4_4_) & (uint)fVar45 |
                                        -(uint)NAN(uStack_170._4_4_) & (uint)local_148._4_4_,
                                        (undefined4)uStack_170);
                }
                else {
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  local_f1 = 1;
                  fVar45 = fVar44;
                  ::utils::geometry::aabb::Aabb2::center();
LAB_14121640b:
                  if (fVar44 < fVar45) {
                    fVar45 = uStack_140._4_4_;
                    if (uStack_140._4_4_ <= local_178._4_4_) {
                      fVar45 = local_178._4_4_;
                    }
                    local_178 = CONCAT44(~-(uint)NAN(local_178._4_4_) & (uint)fVar45 |
                                         -(uint)NAN(local_178._4_4_) & (uint)uStack_140._4_4_,
                                         (undefined4)local_178);
                  }
                }
              }
            }
            pfVar26 = pfVar26 + 5;
          } while (local_178._4_4_ < uStack_170._4_4_);
          if (uStack_170._4_4_ - local_178._4_4_ <= 0.1) {
            if (local_1a8 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            local_1a8 = 0;
            local_1a0 = (undefined8 *)&DAT_00000004;
            local_198 = 0;
            uStack_190 = 0;
          }
          else {
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_04 == (undefined8 *)0x0) {
              local_f1 = 1;
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            local_148 = local_178;
            uStack_140 = uStack_170;
            local_138 = 0;
            uStack_130 = 0;
            extraout_RAX_04[2] = 0;
            extraout_RAX_04[3] = 0;
            *extraout_RAX_04 = local_178;
            extraout_RAX_04[1] = uStack_170;
            *(undefined4 *)(extraout_RAX_04 + 4) = 0;
            *(undefined2 *)((longlong)extraout_RAX_04 + 0x24) = 0;
            if (local_1a8 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            local_1a8 = 1;
            local_198 = 0;
            uStack_190 = 1;
            local_1a0 = extraout_RAX_04;
          }
        }
        else {
          local_f1 = 1;
          local_1a0 = extraout_RAX_01;
          egui::widgets::text_edit::text_buffer::impl$0::as_str();
          for (lVar27 = extraout_RAX_03; lVar27 != extraout_RAX_03 + lVar16 * 0x14;
              lVar27 = lVar27 + 0x14) {
            local_f1 = 1;
            ::utils::geometry::aabb::Aabb2::from_center_dims();
            local_f1 = 1;
            ::utils::geometry::aabb::Aabb2::intersects();
            if (extraout_AL != '\0') {
              local_f1 = 1;
              resolve_hole_overlap::resolve_hole_overlap();
            }
            if (*(int *)(in_RCX + 600) != 2) {
              local_f1 = 1;
              ::utils::geometry::aabb::Aabb2::center();
              local_f1 = 1;
              ::utils::geometry::aabb::Aabb2::width();
              fVar45 = extraout_XMM0_Da_10 + extraout_XMM0_Da_11 * 0.5;
              if (fVar45 < *(float *)(in_RCX + 0x30) || fVar45 == *(float *)(in_RCX + 0x30)) {
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::width();
                if (extraout_XMM0_Da_12 < extraout_XMM0_Da_13 * 0.5) {
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::center();
                  local_f1 = 1;
                  ::utils::geometry::aabb::Aabb2::from_center_dims();
                  local_f1 = 1;
                  resolve_hole_overlap::resolve_hole_overlap();
                }
              }
              else {
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::center();
                local_f1 = 1;
                ::utils::geometry::aabb::Aabb2::from_center_dims();
                local_f1 = 1;
                resolve_hole_overlap::resolve_hole_overlap();
              }
            }
          }
        }
        local_2f8 = &local_2b8;
        local_f1 = 1;
        ::utils::wall_space::impl$1::project_clamp();
        local_148 = CONCAT44(local_184,local_188);
        local_f1 = 1;
        ::utils::wall_space::impl$1::project_clamp();
        local_f1 = 1;
        add_wall_corners::closure_1();
        fVar45 = (float)local_148;
        fVar28 = local_148._4_4_;
        fVar8 = (float)uStack_140;
        local_f1 = 1;
        add_wall_corners::closure_1();
        fVar44 = (float)local_148;
        fVar38 = local_148._4_4_;
        fVar9 = (float)uStack_140;
        local_f1 = 1;
        add_wall_corners::closure_1();
        uVar23 = local_1c8;
        bVar19 = 0.0 < fVar45 + fVar44 + (float)local_148;
        if (bVar19) {
          fVar42 = (float)local_148 + local_148._4_4_;
        }
        else {
          fVar42 = -100.0;
        }
        fVar10 = (float)uStack_140;
        uVar13 = 0;
        uVar40 = (uint)bVar19;
        uVar39 = (int)(uVar40 << 0x1f) >> 0x1f;
        uVar40 = (int)(uVar40 << 0x1f) >> 0x1f;
        fVar45 = (float)(~uVar39 & 0xc2c80000 | (uint)(fVar28 + fVar45) & uVar39);
        fVar44 = (float)(~uVar40 & 0xc2c80000 | (uint)(fVar38 + fVar44) & uVar40);
        uStack_2f0 = uVar4;
        if (((fVar42 <= fVar30) || (fVar45 <= fVar30)) || (fVar44 <= fVar30)) {
          uStack_260 = 0;
          local_268 = CONCAT44(fVar43,fVar43 * fVar35);
          local_1b8 = fVar41 * fVar35;
          if ((uVar17 & 1) != 0) {
            local_268 = CONCAT44(fVar34,fVar34 * fVar36);
            uStack_260 = CONCAT44(fVar37,fVar37);
            local_1b8 = fVar32 * fVar36;
          }
          local_230 = local_230 + 1;
          local_1b4 = (float)uStack_140 + -0.1;
          fVar28 = fVar8 + -0.1;
          if (uStack_190 == 0) {
            if (local_1a8 != 0) goto LAB_141215d9b;
          }
          else {
            local_268 = local_268 ^ 0x8000000080000000;
            uStack_260 = uStack_260 ^ 0x8000000080000000;
            local_240 = local_1a0;
            local_238 = local_1a8;
            lVar16 = local_198;
            uVar23 = uStack_190;
            do {
              uVar17 = 0;
              if (local_238 <= lVar16 + 1U) {
                uVar17 = local_238;
              }
              uStack_388 = (undefined5)
                           ((ulonglong)*(undefined8 *)((longlong)local_240 + lVar16 * 0x28 + 0x1d)
                           >> 0x18);
              uVar13 = local_240[lVar16 * 5 + 2];
              uVar6 = (local_240 + lVar16 * 5 + 2)[1];
              uStack_390 = (undefined4)uVar6;
              uStack_38c = (undefined4)((ulonglong)uVar6 >> 0x20);
              cVar3 = *(char *)((longlong)local_240 + lVar16 * 0x28 + 0x25);
              if (cVar3 == '\x02') break;
              local_152 = *(undefined2 *)((longlong)local_240 + lVar16 * 0x28 + 0x26);
              uStack_158 = uStack_388;
              local_398 = (undefined4)uVar13;
              uStack_394 = (undefined4)((ulonglong)uVar13 >> 0x20);
              local_168 = local_398;
              uStack_164 = uStack_394;
              uStack_160 = uStack_390;
              uStack_15c = uStack_38c;
              local_180 = uVar23 - 1;
              local_178 = local_240[lVar16 * 5];
              uStack_170 = (ulonglong *)(local_240 + lVar16 * 5)[1];
              local_153 = cVar3;
              ::utils::geometry::aabb::Aabb2::width();
              if ((0.1 < extraout_XMM0_Da_14) &&
                 (::utils::geometry::aabb::Aabb2::height(), 0.1 < extraout_XMM0_Da_15)) {
                ::utils::geometry::aabb::Aabb2::center();
                ::utils::geometry::aabb::Aabb2::center();
                bevy_app::plugin::Plugin::
                finish<country_core::systems::main_camera::MainCameraPlugin>();
                local_1b0 = extraout_XMM0_Da_16;
                uStack_1ac = local_398;
                ::utils::wall_space::WallSpaceCoord::to_world_space();
                ::utils::xz_rotation_mat3_bad();
                ::utils::geometry::aabb::Aabb2::width();
                ::utils::geometry::aabb::Aabb2::height();
                local_128 = local_408 * 0.637;
                local_148 = CONCAT44(extraout_XMM0_Da_17 * fStack_424,
                                     extraout_XMM0_Da_17 * local_428);
                uStack_140 = (ulonglong *)
                             CONCAT44(extraout_XMM0_Da_18 * fStack_41c,
                                      extraout_XMM0_Da_17 * fStack_420);
                local_138 = CONCAT44(extraout_XMM0_Da_18 * fStack_414,
                                     extraout_XMM0_Da_18 * local_418);
                uStack_130 = CONCAT44(fStack_40c * 0.637,fStack_410 * 0.637);
                country_core::resources::render::Affine3Packed::from_mat3_translation();
                fVar38 = fStack_204;
                if (local_f2 == '\x02') {
LAB_141216ce8:
                  uVar11 = 4;
                  fVar31 = fVar45;
                  fVar33 = fVar44;
                  fVar29 = fVar42;
                  if (ABS(fVar38 - local_f8) < 0.75) {
                    uVar11 = 0x14;
                  }
                }
                else {
                  local_148 = *in_stack_00000050;
                  uStack_140 = (ulonglong *)in_stack_00000050[1];
                  local_138 = CONCAT17(*(undefined1 *)((longlong)in_stack_00000050 + 0x17),
                                       CONCAT25(*(undefined2 *)((longlong)in_stack_00000050 + 0x15),
                                                CONCAT14(local_f2,*(undefined4 *)
                                                                   (in_stack_00000050 + 2))));
                  country_core::resources::walls::inner_wall_state::StructureType::is_fence();
                  fVar38 = fStack_204;
                  if (extraout_AL_00 == '\0') goto LAB_141216ce8;
                  ::utils::geometry::aabb::Aabb2::height();
                  uVar11 = 0xc;
                  fVar31 = fVar28;
                  fVar33 = fVar9 + -0.1;
                  fVar29 = local_1b4;
                  if (extraout_XMM0_Da_19 * 0.5 + fVar38 <= fVar8) {
                    ::utils::geometry::aabb::Aabb2::height();
                    uVar11 = 0xc;
                    fVar29 = local_1b4;
                    if (extraout_XMM0_Da_20 * 0.5 + fVar38 <= fVar10) goto LAB_141216ce8;
                  }
                }
                local_1fc = uVar11;
                local_1f8 = fVar29;
                country_core::geometry::instanced_wall::impl$1::from();
                fVar38 = local_1b0;
                local_1f4 = extraout_EAX_01;
                ::utils::geometry::aabb::Aabb2::width();
                fVar29 = local_1b0;
                ::utils::geometry::aabb::Aabb2::width();
                local_128 = (float)local_438;
                uStack_124 = (undefined4)((ulonglong)local_438 >> 0x20);
                uStack_120 = uStack_430;
                local_138 = local_448;
                uStack_130 = lStack_440;
                local_148 = local_458;
                uStack_140 = puStack_450;
                local_118 = 0;
                uStack_110 = 0;
                lVar27 = in_stack_00000038[2];
                if (lVar27 == *in_stack_00000038) {
                  alloc::raw_vec::RawVec::
                  grow_one<country_core::geometry::instanced_wall::BrickTransformSsbo,alloc::alloc::Global>
                            ();
                }
                lVar5 = in_stack_00000038[1];
                lVar15 = lVar27 * 0x60;
                auVar7._4_4_ = uStack_124;
                auVar7._0_4_ = local_128;
                auVar7._8_8_ = uStack_120;
                puVar24 = (undefined8 *)(lVar5 + 0x30 + lVar15);
                *puVar24 = local_118;
                puVar24[1] = uStack_110;
                *(undefined1 (*) [16])(lVar5 + 0x20 + lVar15) = auVar7;
                plVar2 = (longlong *)(lVar5 + 0x10 + lVar15);
                *plVar2 = local_138;
                plVar2[1] = uStack_130;
                *(longlong *)(lVar5 + lVar15) = local_148;
                ((longlong *)(lVar5 + lVar15))[1] = (longlong)uStack_140;
                *(ulonglong *)(lVar5 + 0x40 + lVar15) = CONCAT44(fVar33,fVar31);
                *(float *)(lVar5 + 0x48 + lVar15) = local_1f8;
                *(int *)(lVar5 + 0x4c + lVar15) = (int)local_230;
                *(undefined4 *)(lVar5 + 0x50 + lVar15) = local_1f4;
                *(float *)(lVar5 + 0x54 + lVar15) = fVar38 + extraout_XMM0_Da_21 * -0.5;
                *(float *)(lVar5 + 0x58 + lVar15) = fVar29 + extraout_XMM0_Da_22 * 0.5;
                *(undefined4 *)(lVar5 + 0x5c + lVar15) = local_1fc;
                in_stack_00000038[2] = lVar27 + 1;
                ::utils::geometry::aabb::Aabb2::width();
                ::utils::geometry::aabb::Aabb2::height();
                lVar27 = local_1e0;
                uVar13 = CONCAT44(uStack_1ac,local_1b0);
                if (local_1e0 == local_1f0) {
                  alloc::raw_vec::RawVec::
                  grow_one<rhapsody::rg::resource_info::ResourceAccessInfo,alloc::alloc::Global>();
                }
                *(undefined8 *)(lStack_1e8 + lVar27 * 0x14) = uVar13;
                *(undefined4 *)(lStack_1e8 + 8 + lVar27 * 0x14) = extraout_XMM0_Da_23;
                *(undefined4 *)(lStack_1e8 + 0xc + lVar27 * 0x14) = extraout_XMM0_Da_24;
                *(undefined1 *)(lStack_1e8 + 0x10 + lVar27 * 0x14) = 2;
                local_1e0 = lVar27 + 1;
              }
              lVar16 = (lVar16 - uVar17) + 1;
              uVar23 = local_180;
            } while (local_180 != 0);
LAB_141215d9b:
            uVar23 = local_1c8;
            std::alloc::__default_lib_allocator::__rust_dealloc();
            uVar13 = extraout_RAX_00;
          }
          local_180 = CONCAT71((int7)((ulonglong)uVar13 >> 8),1);
        }
        else if (local_1a8 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
    }
    if (local_358 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  } while( true );
}

