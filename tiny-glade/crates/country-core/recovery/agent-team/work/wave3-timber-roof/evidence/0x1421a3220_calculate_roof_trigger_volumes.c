// 0x1421a3220 calculate_roof_trigger_volumes
// sig: void __cdecl calculate_roof_trigger_volumes(void)
// size: 12048


void __cdecl system_roof::roof_trigger_volume::calculate_roof_trigger_volumes(void)

{
  ulonglong *puVar1;
  undefined8 *puVar2;
  ulonglong uVar3;
  ulonglong uVar4;
  int iVar5;
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined4 uVar8;
  undefined4 uVar9;
  longlong *plVar10;
  longlong **pplVar11;
  byte extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  char extraout_AL_07;
  char extraout_AL_08;
  uint extraout_EAX;
  uint extraout_EAX_00;
  uint extraout_EAX_01;
  uint extraout_EAX_02;
  uint extraout_EAX_03;
  uint extraout_EAX_04;
  uint extraout_EAX_05;
  uint extraout_EAX_06;
  uint extraout_EAX_07;
  uint extraout_EAX_08;
  longlong lVar12;
  float *pfVar13;
  ulonglong **ppuVar14;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  longlong lVar15;
  longlong lVar16;
  float *pfVar17;
  ulonglong uVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  longlong *in_RDX;
  code *pcVar21;
  ulonglong *puVar22;
  ulonglong *puVar23;
  longlong lVar24;
  uint uVar25;
  longlong lVar26;
  ulonglong uVar27;
  ulonglong uVar28;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  undefined4 extraout_XMM0_Da_06;
  undefined4 extraout_XMM0_Da_07;
  undefined4 extraout_XMM0_Da_08;
  undefined4 extraout_XMM0_Da_09;
  float extraout_XMM0_Da_10;
  float extraout_XMM0_Da_11;
  float extraout_XMM0_Da_12;
  float extraout_XMM0_Da_13;
  float extraout_XMM0_Da_14;
  float extraout_XMM0_Da_15;
  float extraout_XMM0_Da_16;
  undefined1 auVar29 [16];
  float in_XMM1_Da;
  float fVar31;
  float fVar32;
  float fVar33;
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  uint uVar40;
  undefined1 auVar39 [16];
  float fVar41;
  undefined8 uVar42;
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  float fVar55;
  float fVar56;
  float fVar57;
  float fVar58;
  float fVar59;
  float local_5a8;
  float fStack_5a4;
  float fStack_5a0;
  float fStack_59c;
  undefined4 uStack_584;
  undefined4 uStack_580;
  undefined8 local_56c;
  undefined4 local_564;
  undefined8 local_560;
  undefined4 local_558;
  undefined8 local_554;
  undefined4 local_54c;
  uint local_548;
  uint local_544;
  uint uStack_540;
  uint local_53c;
  ulonglong local_538;
  uint local_530;
  ulonglong local_52c;
  uint local_524;
  undefined8 local_520;
  undefined8 local_518;
  float local_510;
  undefined8 local_50c;
  undefined4 local_504;
  undefined8 local_500;
  float local_4f8;
  undefined8 local_4f0;
  float local_4e8;
  undefined4 uStack_4cc;
  undefined4 uStack_4c8;
  ulonglong *local_4b0;
  longlong local_4a8;
  ulonglong *local_4a0;
  ulonglong local_498;
  ulonglong local_490;
  longlong local_488;
  longlong local_480;
  longlong local_478;
  longlong local_470;
  ulonglong *local_468;
  longlong local_460;
  longlong local_458;
  ulonglong *local_450;
  float local_448;
  undefined1 local_438 [8];
  float fStack_430;
  undefined8 local_428;
  undefined8 uStack_420;
  ulonglong *local_418;
  undefined8 uStack_410;
  float local_408;
  float fStack_404;
  float fStack_400;
  float fStack_3fc;
  ulonglong *local_3f8;
  ulonglong *local_3f0;
  ulonglong local_3e8;
  undefined8 local_3dc;
  undefined4 local_3d4;
  undefined8 local_3d0;
  undefined4 local_3c8;
  undefined8 local_3c4;
  undefined4 local_3bc;
  longlong *local_3b8;
  code *pcStack_3b0;
  longlong local_3a8;
  longlong local_3a0;
  float local_370;
  float local_36c;
  undefined8 local_368;
  undefined8 uStack_360;
  float local_358;
  float local_354;
  undefined8 *local_350;
  code *local_348;
  ulonglong *local_340;
  ulonglong local_338;
  longlong local_330;
  longlong local_328;
  ulonglong *local_320;
  undefined1 local_318 [8];
  float fStack_310;
  float fStack_30c;
  undefined1 local_308 [8];
  float fStack_300;
  undefined1 local_2f8 [8];
  float fStack_2f0;
  ulonglong local_2e0;
  ulonglong local_2d8;
  ulonglong *local_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  undefined8 uStack_2b0;
  undefined1 local_2a8 [16];
  ulonglong local_298;
  ulonglong local_290;
  ulonglong local_288;
  undefined8 local_280;
  undefined1 local_278 [8];
  float fStack_270;
  float fStack_26c;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined **local_248;
  ulonglong *puStack_240;
  longlong **local_238;
  longlong local_230;
  undefined8 local_228;
  undefined1 local_208 [8];
  float fStack_200;
  float fStack_1fc;
  ulonglong *local_1f8;
  undefined4 local_1ec;
  undefined1 local_1e8 [8];
  float fStack_1e0;
  float fStack_1dc;
  ulonglong *local_1d0;
  ulonglong *local_1c8;
  longlong local_1c0;
  undefined8 local_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  undefined8 uStack_1a8;
  undefined8 uStack_1a0;
  ulonglong local_198;
  undefined8 local_188;
  float fStack_180;
  undefined4 uStack_17c;
  undefined8 uStack_178;
  undefined8 uStack_170;
  ulonglong local_168;
  ulonglong local_150;
  ulonglong *puStack_148;
  ulonglong local_140;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  ulonglong local_120;
  longlong local_118;
  longlong lStack_110;
  longlong local_108;
  longlong local_100;
  undefined1 local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  uint uVar30;
  
  local_f0 = 0xfffffffffffffffe;
  local_468 = (ulonglong *)*in_RCX;
  lVar24 = in_RCX[1];
  uVar27 = *local_468;
  uVar18 = 0;
  uVar28 = uVar27 - *(ulonglong *)(lVar24 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar24 + 0x18)) {
    uVar28 = uVar18;
  }
  uVar19 = uVar27 - *(ulonglong *)(lVar24 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar24 + 0x38)) {
    uVar19 = uVar18;
  }
  lVar12 = uVar28 * 0x20 + *(longlong *)(lVar24 + 8);
  uVar27 = *(ulonglong *)(lVar24 + 0x10) - uVar28;
  if (*(ulonglong *)(lVar24 + 0x10) < uVar28) {
    lVar12 = 8;
    uVar27 = uVar18;
  }
  lVar15 = uVar19 * 0x20 + *(longlong *)(lVar24 + 0x28);
  uVar28 = *(ulonglong *)(lVar24 + 0x30) - uVar19;
  if (*(ulonglong *)(lVar24 + 0x30) < uVar19) {
    lVar15 = 8;
    uVar28 = uVar18;
  }
  *local_468 = (*(longlong *)(lVar24 + 0x40) - uVar28) - uVar27;
  local_478 = uVar27 * 0x20 + lVar12;
  local_470 = uVar28 * 0x20 + lVar15;
  local_460 = *in_RDX;
  local_458 = in_RDX[1];
  local_1ec = *(undefined4 *)((longlong)in_RDX + 0x14);
  if (lVar12 == 0) goto LAB_1421a3380;
LAB_1421a3368:
  if (lVar12 == local_478) goto LAB_1421a3380;
  lVar24 = lVar12 + 0x20;
  do {
    local_4a8 = lVar12 + 8;
    *local_468 = *local_468 + 1;
    uVar27 = *(ulonglong *)(lVar12 + 8) & 0xffffffff;
    if ((*(ulonglong *)(local_458 + 0x10) <= uVar27) ||
       (*(int *)(*(longlong *)(local_458 + 8) + uVar27 * 0x14) !=
        (int)(*(ulonglong *)(lVar12 + 8) >> 0x20))) {
LAB_1421a6136:
      local_3b8 = &local_4a8;
      pcStack_3b0 = core::fmt::impl$74::
                    fmt<country_core::resources::roofs::commands::RecalculateRoofCmd>;
      local_248 = &PTR_s_Couldn_t_recalcualte_roof_volume_142ff6ae8;
      puStack_240 = (ulonglong *)0x1;
      local_228 = 0;
      local_238 = &local_3b8;
      local_230 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    lVar12 = *(longlong *)(local_458 + 8) + uVar27 * 0x14;
    uVar27 = (ulonglong)*(uint *)(lVar12 + 4);
    if ((uVar27 == 0xffffffff) ||
       ((*(ulonglong *)(local_460 + 0x38) <= uVar27 ||
        ((*(ulonglong *)
           (*(longlong *)(local_460 + 0x28) + (ulonglong)(*(uint *)(lVar12 + 4) >> 6) * 8) >>
          (uVar27 & 0x3f) & 1) == 0)))) goto LAB_1421a6136;
    local_338 = (ulonglong)*(uint *)(lVar12 + 0x10);
    lVar26 = *(longlong *)
              (*(longlong *)(local_458 + 0x1a8) + 0x18 + (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48);
    lVar12 = *(longlong *)
              (*(longlong *)(local_458 + 0x1a8) + 0x38 + (ulonglong)*(uint *)(lVar12 + 0xc) * 0x48);
    lVar16 = ~*(ulonglong *)(lVar12 + *(longlong *)(local_460 + 0x110) * 8) * 0x30;
    local_328 = *(longlong *)(lVar26 + 0x28 + lVar16);
    lVar12 = *(longlong *)
              (lVar26 + 0x10 + ~*(ulonglong *)(lVar12 + *(longlong *)(local_460 + 0x118) * 8) * 0x30
              );
    local_330 = local_338 * 0x48 + *(longlong *)(lVar26 + 0x10 + lVar16);
    lVar26 = local_338 * 0x58;
    local_488 = lVar15;
    local_480 = lVar24;
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    local_2d8 = (ulonglong)(extraout_AL ^ 1) * 5 + 4;
    local_118 = 0;
    lStack_110 = 4;
    local_108 = 0;
    local_138 = 0;
    lStack_130 = 4;
    local_128 = 0;
    local_150 = 0;
    puStack_148 = (ulonglong *)&DAT_00000004;
    local_140 = 0;
    lVar24 = lVar12 + lVar26;
    local_100 = lVar12 + lVar26;
    if ((*(byte *)(lVar12 + 8 + lVar26) & 1) != 0) {
      local_258 = *(undefined8 *)(lVar24 + 0x1c);
      local_268 = *(undefined8 *)(lVar24 + 0xc);
      uStack_260 = *(undefined8 *)(lVar24 + 0x14);
      local_f1 = 1;
      local_f2 = 1;
      local_f3 = 1;
      country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom();
      local_f1 = 1;
      local_f2 = 1;
      local_f3 = 1;
      country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_rectangle();
      pcVar21 = pcStack_3b0;
      plVar10 = local_3b8;
      auVar43._8_8_ = pcStack_3b0;
      auVar43._0_8_ = local_3b8;
      uVar27 = (ulonglong)pcStack_3b0 >> 0x20;
      local_f1 = 1;
      local_f2 = 1;
      local_f3 = 1;
      country_core::resources::roofs::roof_shape::Roof::is_flat();
      if (extraout_AL_00 == '\0') {
        auVar7._4_4_ = SUB84(pcVar21,0) + 0.4;
        auVar7._0_4_ = (float)((ulonglong)plVar10 >> 0x20) + 0.4;
        auVar7._8_4_ = (float)uVar27 + 0.4;
        auVar7._12_4_ = 0;
        auVar43 = auVar7 << 0x20;
      }
      local_f1 = 1;
      local_f2 = 1;
      local_f3 = 1;
      country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized();
      ::utils::geometry::rectangle::Rectangle2d::new_centered_axis_aligned();
      lVar24 = local_100;
      country_core::resources::roofs::roof_ridge::calc_roof_tip_center_ls();
      in_XMM1_Da = auVar43._8_4_;
      ::utils::geometry::rectangle::Rectangle2d::new_axis_aligned();
      ::utils::geometry::rectangle::Rectangle2d::orientation_mat2();
      country_core::resources::roofs::roof_shape::Roof::is_gable();
      if (extraout_AL_01 == '\0') {
        local_298 = 0;
      }
      else {
        if (*(char *)(lVar24 + 0x3c) == '\0') {
          country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws();
          pfVar17 = &local_358;
          pfVar13 = &local_370;
          fVar31 = extraout_XMM0_Da_10;
        }
        else {
          country_core::resources::roofs::roof_shape::RoofShapeParams::eave_length_ws();
          pfVar17 = &local_354;
          pfVar13 = &local_36c;
          fVar31 = extraout_XMM0_Da;
        }
        in_XMM1_Da = *pfVar17 - (fVar31 + 0.56);
        *pfVar17 = in_XMM1_Da;
        *pfVar13 = *pfVar13 + -0.56;
        country_core::resources::roofs::roof_shape::Roof::gable_sides();
      }
      ::utils::geometry::rectangle::Rectangle2d::as_curve2();
      ::utils::geometry::rectangle::Rectangle2d::as_curve2();
      local_340 = (ulonglong *)((longlong)pcStack_3b0 + local_3a8 * 8);
      local_2e0 = local_3a8 - 1;
      if (local_3a8 == 0) {
        local_2e0 = 0;
      }
      local_1c0 = (longlong)local_238 - 1;
      if (local_238 == (longlong **)0x0) {
        local_1c0 = 0;
      }
      local_408 = (float)(int)local_2d8;
      local_308 = (undefined1  [8])local_238;
      local_320 = puStack_240 + (longlong)local_238;
      local_120 = 1;
      local_3e8 = 0;
      local_2f8 = (undefined1  [8])puStack_240;
      uVar27 = 1;
      local_450 = (ulonglong *)pcStack_3b0;
      local_1d0 = puStack_240;
      local_1f8 = (ulonglong *)pcStack_3b0;
      local_2d0 = (ulonglong *)pcStack_3b0;
      pcVar21 = pcStack_3b0;
      puVar22 = puStack_240;
      if (local_2e0 != 0) goto LAB_1421a47c3;
LAB_1421a4970:
      uVar28 = 0;
      lVar24 = local_1c0;
LAB_1421a4999:
      do {
        ppuVar14 = &local_1c8;
        uVar18 = local_120;
        puVar23 = puVar22;
        puVar22 = (ulonglong *)0x0;
        while( true ) {
          *ppuVar14 = puVar22;
          local_428 = local_1c8;
          if (local_1c8 == (ulonglong *)0x0) {
            if (local_248 != (undefined **)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_230 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_3b8 != (longlong *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_3a0 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            lVar24 = uStack_170;
            if (local_188 != (undefined **)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
              lVar24 = uStack_170;
            }
            goto joined_r0x0001421a59e4;
          }
          local_418 = local_4b0;
          local_2e0 = uVar28;
          local_1c0 = lVar24;
          local_120 = uVar18;
          if (((local_298 & 1) == 0) ||
             ((puVar22 = puVar23, pcVar21 = (code *)local_2d0, local_290 != local_490 &&
              (local_288 != local_490)))) {
            local_448 = (float)local_490 * 0.25;
            uVar28 = 0;
            local_438._0_4_ = ((float)local_490 + 1.0) * 0.25;
            local_4a0 = puVar23;
            local_498 = uVar27;
            while (uVar28 < local_2d8) {
              fVar31 = (float)(longlong)uVar28 / local_408;
              local_280 = CONCAT44(local_280._4_4_,fVar31);
              if ((fVar31 < 0.0) || (1.0 < fVar31)) {
                local_350 = &local_280;
                local_348 = core::fmt::float::impl$5::fmt;
                local_1b8._0_4_ = 127.63519;
                local_1b8._4_4_ = 1.4013e-45;
                uStack_1b0 = 1;
                uStack_1ac = 0;
                local_198 = 0;
                uStack_1a8 = &local_350;
                uStack_1a0 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              if (NAN(fVar31)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              local_278._0_4_ = fVar31;
              ::utils::curve::Curve::
              get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                        ();
              uVar27 = CONCAT44(uStack_178._4_4_,(float)uStack_178);
              uVar18 = (ulonglong)extraout_EAX_05;
              if (uVar27 <= uVar18) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              if (uVar27 <= uVar18 + 1) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              lVar24 = CONCAT44(uStack_17c,fStack_180);
              fVar31 = *(float *)(lVar24 + uVar18 * 8);
              fVar53 = *(float *)(lVar24 + 8 + uVar18 * 8);
              uVar18 = *local_428;
              _fStack_1e0 = 0;
              local_1e8 = (undefined1  [8])*local_3f0;
              _fStack_310 = 0;
              local_318 = (undefined1  [8])*local_3f8;
              _fStack_200 = 0;
              local_208 = (undefined1  [8])*local_418;
              country_core::resources::roofs::roof_shape::Roof::is_flat();
              local_2b8 = CONCAT44(local_2b8._4_4_,0xbf3d70a4);
              if (extraout_AL_06 == '\0') {
                country_core::resources::roofs::roof_shape::Roof::height();
                ::utils::curve::Curve::
                get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                          ();
                uVar19 = (ulonglong)extraout_EAX_06;
                if (uVar27 <= uVar19) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar27 <= uVar19 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                fVar32 = *(float *)(lVar24 + 0xc + uVar19 * 8) * extraout_XMM0_Da_13 +
                         (1.0 - extraout_XMM0_Da_13) * *(float *)(lVar24 + 4 + uVar19 * 8);
                local_2b8 = CONCAT44(local_2b8._4_4_,
                                     extraout_XMM0_Da_12 * fVar32 + (1.0 - fVar32) * 0.0);
              }
              fVar32 = ((float)(longlong)uVar28 + 1.0) / local_408;
              local_280 = CONCAT44(local_280._4_4_,fVar32);
              if ((fVar32 < 0.0) || (1.0 < fVar32)) {
                local_350 = &local_280;
                local_348 = core::fmt::float::impl$5::fmt;
                local_1b8._0_4_ = 127.63519;
                local_1b8._4_4_ = 1.4013e-45;
                uStack_1b0 = 1;
                uStack_1ac = 0;
                local_198 = 0;
                uStack_1a8 = &local_350;
                uStack_1a0 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              if (NAN(fVar32)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              ::utils::curve::Curve::
              get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                        ();
              uVar19 = (ulonglong)extraout_EAX_07;
              if (uVar27 <= uVar19) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              if (uVar27 <= uVar19 + 1) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              fVar41 = *(float *)(lVar24 + uVar19 * 8);
              fVar54 = *(float *)(lVar24 + 8 + uVar19 * 8);
              uVar19 = *local_428;
              uVar3 = *local_3f0;
              uVar4 = *local_3f8;
              local_2c8 = *local_418;
              uStack_2c0 = 0;
              country_core::resources::roofs::roof_shape::Roof::is_flat();
              local_2a8._0_4_ = (float)local_2b8;
              if (extraout_AL_07 == '\0') {
                country_core::resources::roofs::roof_shape::Roof::height();
                ::utils::curve::Curve::
                get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                          ();
                uVar20 = (ulonglong)extraout_EAX_08;
                if (uVar27 <= uVar20) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar27 <= uVar20 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                fVar33 = *(float *)(lVar24 + 0xc + uVar20 * 8) * extraout_XMM0_Da_16 +
                         (1.0 - extraout_XMM0_Da_16) * *(float *)(lVar24 + 4 + uVar20 * 8);
                local_2a8._0_4_ = extraout_XMM0_Da_15 * fVar33 + (1.0 - fVar33) * 0.0;
              }
              uVar28 = uVar28 + 1;
              fVar53 = (1.0 - extraout_XMM0_Da_11) * fVar31 + fVar53 * extraout_XMM0_Da_11;
              fVar31 = 1.0 - fVar53;
              local_1e8._4_4_ = (float)local_1e8._4_4_ * fVar53 + (float)(uVar18 >> 0x20) * fVar31;
              local_1e8._0_4_ = (float)local_1e8._0_4_ * fVar53 + (float)uVar18 * fVar31;
              fStack_1e0 = fStack_1e0 * 0.0 + 0.0;
              fStack_1dc = fStack_1dc * 0.0 + 0.0;
              local_208._4_4_ = (float)local_208._4_4_ * fVar53 + (float)local_318._4_4_ * fVar31;
              local_208._0_4_ = (float)local_208._0_4_ * fVar53 + (float)local_318._0_4_ * fVar31;
              fStack_200 = fStack_200 * 0.0 + fStack_310 * 0.0;
              fStack_1fc = fStack_1fc * 0.0 + fStack_30c * 0.0;
              fVar53 = (1.0 - extraout_XMM0_Da_14) * fVar41 + fVar54 * extraout_XMM0_Da_14;
              fVar31 = 1.0 - fVar53;
              local_318._4_4_ = (float)(uVar3 >> 0x20) * fVar53 + (float)(uVar19 >> 0x20) * fVar31;
              local_318._0_4_ = (float)uVar3 * fVar53 + (float)uVar19 * fVar31;
              _fStack_310 = 0;
              local_2c8 = CONCAT44(local_2c8._4_4_ * fVar53 + (float)(uVar4 >> 0x20) * fVar31,
                                   (float)local_2c8 * fVar53 + (float)uVar4 * fVar31);
              uStack_2c0 = CONCAT44(uStack_2c0._4_4_ * 0.0 + 0.0,(float)uStack_2c0 * 0.0 + 0.0);
              for (uVar25 = 0; uVar25 < 4; uVar25 = uVar25 + 1) {
                fVar54 = (float)(int)uVar25 * 0.25;
                fVar33 = 1.0 - fVar54;
                fVar31 = (float)local_208._0_4_ * fVar54 + (float)local_1e8._0_4_ * fVar33;
                fVar53 = (float)local_208._4_4_ * fVar54 + (float)local_1e8._4_4_ * fVar33;
                local_1b8._0_4_ = (float)uStack_260 + fVar53 * fStack_5a0 + fVar31 * local_5a8;
                local_1b8._4_4_ =
                     (float)((ulonglong)uStack_260 >> 0x20) +
                     fVar53 * fStack_59c + fVar31 * fStack_5a4;
                ::utils::swizzlers::impl$1::xvy();
                fVar41 = ((float)(int)uVar25 + 1.0) * 0.25;
                fVar56 = 1.0 - fVar41;
                fVar31 = (float)local_1e8._0_4_ * fVar56 + (float)local_208._0_4_ * fVar41;
                fVar53 = (float)local_1e8._4_4_ * fVar56 + (float)local_208._4_4_ * fVar41;
                local_1b8._0_4_ = (float)uStack_260 + fVar53 * fStack_5a0 + fVar31 * local_5a8;
                local_1b8._4_4_ =
                     (float)((ulonglong)uStack_260 >> 0x20) +
                     fVar53 * fStack_59c + fVar31 * fStack_5a4;
                ::utils::swizzlers::impl$1::xvy();
                fVar31 = fVar54 * (float)local_2c8 + fVar33 * (float)local_318._0_4_;
                fVar53 = fVar54 * local_2c8._4_4_ + fVar33 * (float)local_318._4_4_;
                local_1b8._0_4_ = (float)uStack_260 + fVar53 * fStack_5a0 + fVar31 * local_5a8;
                local_1b8._4_4_ =
                     (float)((ulonglong)uStack_260 >> 0x20) +
                     fVar53 * fStack_59c + fVar31 * fStack_5a4;
                ::utils::swizzlers::impl$1::xvy();
                fVar31 = fVar41 * (float)local_2c8 + fVar56 * (float)local_318._0_4_;
                fVar53 = fVar41 * local_2c8._4_4_ + fVar56 * (float)local_318._4_4_;
                local_1b8._0_4_ = (float)uStack_260 + fVar53 * fStack_5a0 + fVar31 * local_5a8;
                local_1b8._4_4_ =
                     (float)((ulonglong)uStack_260 >> 0x20) +
                     fVar53 * fStack_59c + fVar31 * fStack_5a4;
                ::utils::swizzlers::impl$1::xvy();
                uVar9 = local_3bc;
                uVar8 = local_3c8;
                uVar42 = local_3d0;
                uStack_584 = (undefined4)local_3c4;
                uStack_580 = (undefined4)((ulonglong)local_3c4 >> 0x20);
                ::utils::geometry::intersect::Triangle::normal();
                lVar24 = local_108;
                local_198 = CONCAT44(local_198._4_4_,uVar8);
                uStack_1a8._0_4_ = (float)uStack_580;
                uStack_1a8._4_4_ = (float)uVar9;
                uStack_1a0 = uVar42;
                local_1b8._0_4_ = (float)local_50c;
                local_1b8._4_4_ = (float)((ulonglong)local_50c >> 0x20);
                uStack_1b0 = local_504;
                uStack_1ac = uStack_584;
                if (local_108 == local_118) {
                  alloc::raw_vec::RawVec::
                  grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
                }
                lVar12 = local_128;
                *(undefined4 *)(lStack_110 + 0x20 + lVar24 * 0x24) = (undefined4)local_198;
                puVar2 = (undefined8 *)(lStack_110 + 0x10 + lVar24 * 0x24);
                *puVar2 = CONCAT44(uStack_1a8._4_4_,(float)uStack_1a8);
                puVar2[1] = uStack_1a0;
                puVar2 = (undefined8 *)(lStack_110 + lVar24 * 0x24);
                *puVar2 = CONCAT44(local_1b8._4_4_,(float)local_1b8);
                puVar2[1] = CONCAT44(uStack_1ac,uStack_1b0);
                local_108 = lVar24 + 1;
                if (local_128 == local_138) {
                  alloc::raw_vec::RawVec::
                  grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
                }
                uVar27 = local_140;
                uVar8 = local_3d4;
                uVar42 = local_3dc;
                fVar53 = fVar54 * (float)local_438._0_4_ + fVar33 * local_448;
                fVar31 = fVar41 * (float)local_438._0_4_ + fVar56 * local_448;
                *(float *)(lStack_130 + lVar12 * 0x18) = fVar53;
                *(undefined4 *)(lStack_130 + 4 + lVar12 * 0x18) = local_278._0_4_;
                *(float *)(lStack_130 + 8 + lVar12 * 0x18) = fVar31;
                *(undefined4 *)(lStack_130 + 0xc + lVar12 * 0x18) = local_278._0_4_;
                *(float *)(lStack_130 + 0x10 + lVar12 * 0x18) = fVar53;
                *(float *)(lStack_130 + 0x14 + lVar12 * 0x18) = fVar32;
                local_128 = lVar12 + 1;
                uStack_4cc = (undefined4)local_3dc;
                uStack_4c8 = (undefined4)((ulonglong)local_3dc >> 0x20);
                if (local_140 == local_150) {
                  alloc::raw_vec::RawVec::
                  grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
                }
                lVar24 = local_108;
                *(undefined4 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x20) = uVar8;
                puVar2 = (undefined8 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x10);
                *puVar2 = CONCAT44(uVar8,uStack_4c8);
                puVar2[1] = uVar42;
                puVar2 = (undefined8 *)((longlong)puStack_148 + uVar27 * 0x24);
                *puVar2 = uVar42;
                puVar2[1] = CONCAT44(uStack_4cc,uVar8);
                local_140 = uVar27 + 1;
                uStack_1b0 = local_3bc;
                local_1b8._0_4_ = (float)local_3c4;
                local_1b8._4_4_ = (float)((ulonglong)local_3c4 >> 0x20);
                uStack_1a8._4_4_ = local_510;
                uStack_1ac = (undefined4)local_518;
                uStack_1a8._0_4_ = (float)((ulonglong)local_518 >> 0x20);
                local_198 = CONCAT44(local_198._4_4_,local_3c8);
                uStack_1a0 = local_3d0;
                if (local_108 == local_118) {
                  alloc::raw_vec::RawVec::
                  grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
                }
                lVar12 = local_128;
                *(undefined4 *)(lStack_110 + 0x20 + lVar24 * 0x24) = (undefined4)local_198;
                puVar2 = (undefined8 *)(lStack_110 + 0x10 + lVar24 * 0x24);
                *puVar2 = CONCAT44(uStack_1a8._4_4_,(float)uStack_1a8);
                puVar2[1] = uStack_1a0;
                puVar2 = (undefined8 *)(lStack_110 + lVar24 * 0x24);
                *puVar2 = CONCAT44(local_1b8._4_4_,(float)local_1b8);
                puVar2[1] = CONCAT44(uStack_1ac,uStack_1b0);
                local_108 = lVar24 + 1;
                if (local_128 == local_138) {
                  alloc::raw_vec::RawVec::
                  grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
                }
                uVar27 = local_140;
                *(float *)(lStack_130 + lVar12 * 0x18) = fVar31;
                *(undefined4 *)(lStack_130 + 4 + lVar12 * 0x18) = local_278._0_4_;
                *(float *)(lStack_130 + 8 + lVar12 * 0x18) = fVar31;
                *(float *)(lStack_130 + 0xc + lVar12 * 0x18) = fVar32;
                *(float *)(lStack_130 + 0x10 + lVar12 * 0x18) = fVar53;
                *(float *)(lStack_130 + 0x14 + lVar12 * 0x18) = fVar32;
                local_128 = lVar12 + 1;
                if (local_140 == local_150) {
                  alloc::raw_vec::RawVec::
                  grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
                }
                *(undefined4 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x20) = uVar8;
                puVar2 = (undefined8 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x10);
                *puVar2 = CONCAT44(uVar8,uStack_4c8);
                puVar2[1] = uVar42;
                puVar2 = (undefined8 *)((longlong)puStack_148 + uVar27 * 0x24);
                *puVar2 = uVar42;
                puVar2[1] = CONCAT44(uStack_4cc,uVar8);
                local_140 = uVar27 + 1;
              }
            }
            country_core::resources::roofs::roof_shape::Roof::is_flat();
            if (extraout_AL_08 == '\0') {
              country_core::resources::roofs::roof_shape::Roof::height();
            }
            if (local_308 == (undefined1  [8])0x0) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            ::utils::swizzlers::impl$1::xvy();
            if (local_308 == (undefined1  [8])0x1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar53 = (float)((ulonglong)*(undefined8 *)((longlong)local_2f8 + 8) >> 0x20);
            fVar31 = (float)*(undefined8 *)((longlong)local_2f8 + 8);
            local_280 = CONCAT44((float)((ulonglong)uStack_260 >> 0x20) +
                                 fVar53 * fStack_59c + fVar31 * fStack_5a4,
                                 (float)uStack_260 + fVar53 * fStack_5a0 + fVar31 * local_5a8);
            ::utils::swizzlers::impl$1::xvy();
            if ((ulonglong)local_308 < 3) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar53 = (float)((ulonglong)*(undefined8 *)((longlong)local_2f8 + 0x10) >> 0x20);
            fVar31 = (float)*(undefined8 *)((longlong)local_2f8 + 0x10);
            local_350 = (undefined8 *)
                        CONCAT44((float)((ulonglong)uStack_260 >> 0x20) +
                                 fVar53 * fStack_59c + fVar31 * fStack_5a4,
                                 (float)uStack_260 + fVar53 * fStack_5a0 + fVar31 * local_5a8);
            ::utils::swizzlers::impl$1::xvy();
            if (local_308 == (undefined1  [8])0x3) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            fVar53 = (float)((ulonglong)*(undefined8 *)((longlong)local_2f8 + 0x18) >> 0x20);
            fVar31 = (float)*(undefined8 *)((longlong)local_2f8 + 0x18);
            local_1b8._0_4_ = (float)uStack_260 + fStack_5a0 * fVar53 + local_5a8 * fVar31;
            local_1b8._4_4_ =
                 (float)((ulonglong)uStack_260 >> 0x20) + fStack_59c * fVar53 + fStack_5a4 * fVar31;
            ::utils::swizzlers::impl$1::xvy();
            auVar34._8_8_ = 0;
            auVar34._0_8_ = local_538;
            auVar36._8_8_ = 0;
            auVar36._0_8_ = local_52c;
            iVar5 = *(int *)(local_100 + 0x3c);
            uVar25 = (iVar5 << 0x1f) >> 0x1f;
            uVar30 = (iVar5 << 0x1f) >> 0x1f;
            auVar29._0_8_ = CONCAT44(uVar30,uVar25);
            auVar29._8_8_ = 0;
            uVar40 = (uint)((ulonglong)local_520 >> 0x20);
            auVar39._0_4_ = (uint)local_520 & uVar25;
            auVar39._4_4_ = uVar40 & uVar30;
            auVar39._8_8_ = 0;
            auVar39 = auVar39 | ~auVar29 & auVar36;
            auVar37._8_8_ = 0;
            auVar37._0_8_ = local_52c & auVar29._0_8_;
            auVar37 = ~auVar29 & auVar34 | auVar37;
            auVar44._0_4_ = (iVar5 << 0x1f) >> 0x1f;
            auVar44._4_4_ = (iVar5 << 0x1f) >> 0x1f;
            auVar44._8_4_ = (iVar5 << 0x1f) >> 0x1f;
            auVar44._12_4_ = (iVar5 << 0x1f) >> 0x1f;
            auVar38._4_4_ = local_544;
            auVar38._0_4_ = local_548;
            auVar38._8_4_ = uStack_540;
            auVar38._12_4_ = local_53c;
            auVar46._0_4_ = local_53c & auVar44._0_4_;
            auVar46._4_4_ = (uint)local_538 & auVar44._4_4_;
            auVar46._8_4_ = (uint)(local_538 >> 0x20) & auVar44._8_4_;
            auVar46._12_4_ = local_530 & auVar44._12_4_;
            auVar46 = auVar46 | ~auVar44 & auVar38;
            auVar35._0_4_ = local_548 & auVar44._0_4_;
            auVar35._4_4_ = local_544 & auVar44._4_4_;
            auVar35._8_4_ = uStack_540 & auVar44._8_4_;
            auVar35._12_4_ = local_524 & auVar44._12_4_;
            auVar45._0_4_ = ~auVar44._0_4_ & local_524;
            auVar45._4_4_ = ~auVar44._4_4_ & (uint)local_520;
            auVar45._8_4_ = ~auVar44._8_4_ & uVar40;
            auVar45._12_4_ = ~auVar44._12_4_ & local_530;
            auVar45 = auVar45 | auVar35;
            fVar32 = auVar46._12_4_;
            fStack_1e0 = auVar46._8_4_;
            local_1e8 = (undefined1  [8])auVar46._8_8_;
            fStack_1dc = fVar32;
            fVar53 = auVar45._12_4_;
            fStack_200 = auVar45._8_4_;
            local_208 = (undefined1  [8])auVar45._8_8_;
            fStack_1fc = fVar53;
            fVar31 = auVar37._4_4_;
            local_2c8 = CONCAT44(fVar31,fVar31);
            uStack_2c0 = CONCAT44(auVar37._12_4_,auVar37._12_4_);
            local_278._0_4_ = auVar39._0_4_;
            fStack_270 = auVar39._4_4_;
            local_2b8 = CONCAT44(fStack_270,fStack_270);
            uStack_2b0 = CONCAT44(auVar39._12_4_,auVar39._12_4_);
            local_278._4_4_ = fVar53;
            fStack_26c = (float)local_278._0_4_;
            in_XMM1_Da = (float)local_278._0_4_;
            for (uVar25 = 0; lVar24 = local_108, puVar22 = local_4a0, uVar27 = local_498,
                pcVar21 = (code *)local_2d0, uVar25 < 4; uVar25 = uVar25 + 1) {
              fVar41 = ((float)(int)uVar25 + 1.0) * 0.25;
              fVar54 = (float)(int)uVar25 * 0.25;
              if (local_108 == local_118) {
                alloc::raw_vec::RawVec::
                grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
              }
              fVar33 = 1.0 - fVar41;
              fVar56 = 1.0 - fVar54;
              fVar47 = fVar33 * (float)local_278._0_4_;
              fVar50 = fVar33 * fStack_270;
              fVar48 = fVar56 * (float)local_278._4_4_ + fVar54 * fVar32;
              fVar51 = fVar56 * fStack_26c + fVar54 * auVar37._0_4_;
              fVar52 = (float)local_2c8 * fVar54 + (float)local_2b8 * fVar56;
              fVar57 = fVar41 * auVar46._0_4_ + fVar33 * auVar45._0_4_;
              fVar58 = fVar41 * auVar46._4_4_ + fVar33 * auVar45._4_4_;
              fVar59 = fVar41 * auVar46._8_4_ + fVar33 * auVar45._8_4_;
              *(ulonglong *)(lStack_110 + lVar24 * 0x24) =
                   CONCAT44(auVar46._4_4_ * fVar54 + auVar45._4_4_ * fVar56,
                            auVar46._0_4_ * fVar54 + auVar45._0_4_ * fVar56);
              *(float *)(lStack_110 + 8 + lVar24 * 0x24) =
                   (float)local_1e8._0_4_ * fVar54 + (float)local_208._0_4_ * fVar56;
              pfVar17 = (float *)(lStack_110 + 0xc + lVar24 * 0x24);
              *pfVar17 = fVar57;
              pfVar17[1] = fVar58;
              pfVar17[2] = fVar59;
              pfVar17[3] = fVar48;
              *(float *)(lStack_110 + 0x1c + lVar24 * 0x24) = fVar51;
              *(float *)(lStack_110 + 0x20 + lVar24 * 0x24) = fVar52;
              lVar12 = lVar24 + 1;
              if (lVar12 == local_118) {
                local_108 = lVar12;
                alloc::raw_vec::RawVec::
                grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
              }
              lVar15 = local_128;
              pfVar17 = (float *)(lStack_110 + lVar12 * 0x24);
              *pfVar17 = fVar57;
              pfVar17[1] = fVar58;
              pfVar17[2] = fVar59;
              pfVar17[3] = fVar41 * fVar32 + fVar33 * fVar53;
              pfVar17 = (float *)(lStack_110 + 0x10 + lVar12 * 0x24);
              *pfVar17 = fVar47 + fVar41 * auVar37._0_4_;
              pfVar17[1] = fVar50 + fVar41 * fVar31;
              pfVar17[2] = fVar48;
              pfVar17[3] = fVar51;
              *(float *)(lStack_110 + 0x20 + lVar12 * 0x24) = fVar52;
              local_108 = lVar24 + 2;
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>
                          ();
              }
              fVar54 = fVar54 + fVar56 * 0.0;
              fVar41 = fVar41 + fVar33 * 0.0;
              *(float *)(lStack_130 + lVar15 * 0x18) = fVar54;
              *(undefined4 *)(lStack_130 + 4 + lVar15 * 0x18) = 0x40000000;
              *(float *)(lStack_130 + 8 + lVar15 * 0x18) = fVar41;
              *(undefined4 *)(lStack_130 + 0xc + lVar15 * 0x18) = 0x40000000;
              *(float *)(lStack_130 + 0x10 + lVar15 * 0x18) = fVar54;
              *(undefined4 *)(lStack_130 + 0x14 + lVar15 * 0x18) = 0x40066666;
              lVar24 = lVar15 + 1;
              if (lVar24 == local_138) {
                local_128 = lVar24;
                alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>
                          ();
              }
              uVar27 = local_140;
              *(float *)(lStack_130 + lVar24 * 0x18) = fVar41;
              *(undefined4 *)(lStack_130 + 4 + lVar24 * 0x18) = 0x40000000;
              *(float *)(lStack_130 + 8 + lVar24 * 0x18) = fVar41;
              *(undefined4 *)(lStack_130 + 0xc + lVar24 * 0x18) = 0x40066666;
              *(float *)(lStack_130 + 0x10 + lVar24 * 0x18) = fVar54;
              *(undefined4 *)(lStack_130 + 0x14 + lVar24 * 0x18) = 0x40066666;
              local_128 = lVar15 + 2;
              local_1b8._0_4_ = 0.0;
              local_1b8._4_4_ = 1.0;
              uStack_1b0 = 0;
              uStack_1ac = 0;
              uStack_1a8._0_4_ = 1.0;
              uStack_1a8._4_4_ = 0.0;
              uStack_1a0 = 0x3f80000000000000;
              local_198 = local_198 & 0xffffffff00000000;
              if (local_140 == local_150) {
                alloc::raw_vec::RawVec::
                grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
              }
              *(undefined4 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x20) = (undefined4)local_198;
              puVar2 = (undefined8 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x10);
              *puVar2 = CONCAT44(uStack_1a8._4_4_,(float)uStack_1a8);
              puVar2[1] = uStack_1a0;
              puVar2 = (undefined8 *)((longlong)puStack_148 + uVar27 * 0x24);
              *puVar2 = CONCAT44(local_1b8._4_4_,(float)local_1b8);
              puVar2[1] = CONCAT44(uStack_1ac,uStack_1b0);
              uVar28 = uVar27 + 1;
              local_1b8._0_4_ = 0.0;
              local_1b8._4_4_ = 1.0;
              uStack_1b0 = 0;
              uStack_1ac = 0;
              uStack_1a8._0_4_ = 1.0;
              uStack_1a8._4_4_ = 0.0;
              uStack_1a0 = 0x3f80000000000000;
              local_198 = local_198 & 0xffffffff00000000;
              if (uVar28 == local_150) {
                local_140 = uVar28;
                alloc::raw_vec::RawVec::
                grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
              }
              *(undefined4 *)((longlong)puStack_148 + uVar28 * 0x24 + 0x20) = (undefined4)local_198;
              auVar6._4_4_ = local_1b8._4_4_;
              auVar6._0_4_ = (float)local_1b8;
              auVar6._8_4_ = uStack_1b0;
              auVar6._12_4_ = uStack_1ac;
              pfVar17 = (float *)((longlong)puStack_148 + uVar28 * 0x24 + 0x10);
              *pfVar17 = (float)uStack_1a8;
              pfVar17[1] = uStack_1a8._4_4_;
              pfVar17[2] = (float)uStack_1a0;
              pfVar17[3] = uStack_1a0._4_4_;
              *(undefined1 (*) [16])((longlong)puStack_148 + uVar28 * 0x24) = auVar6;
              local_140 = uVar27 + 2;
              in_XMM1_Da = (float)uStack_1a8;
            }
          }
          local_2d0 = (ulonglong *)pcVar21;
          if (local_2e0 == 0) goto LAB_1421a4970;
LAB_1421a47c3:
          uVar28 = local_2e0 - 1;
          lVar24 = local_1c0;
          if (pcVar21 == (code *)local_340) {
            local_2d0 = local_340;
            goto LAB_1421a4999;
          }
          local_2d0 = (ulonglong *)((longlong)pcVar21 + 8);
          if (uVar27 == 0) {
            if (local_1f8 == local_340) {
              uVar27 = 0;
              local_1f8 = local_340;
              goto LAB_1421a4999;
            }
          }
          else {
            if ((ulonglong)((longlong)local_340 - (longlong)local_1f8) >> 3 <= uVar27) {
              uVar27 = 0;
              local_1f8 = local_340;
              goto LAB_1421a4999;
            }
            local_1f8 = local_1f8 + uVar27;
          }
          puVar1 = local_1f8 + 1;
          if (local_1c0 == 0) {
            lVar24 = 0;
            goto LAB_1421a48e9;
          }
          lVar24 = local_1c0 + -1;
          if (local_1d0 == local_320) break;
          if (local_120 == 0) {
            if (puVar22 != local_320) goto LAB_1421a4853;
LAB_1421a4900:
            uVar18 = 0;
            uVar27 = 0;
            ppuVar14 = &local_1c8;
            puVar22 = (ulonglong *)0x0;
            puVar23 = local_320;
            local_1f8 = puVar1;
            local_1d0 = local_1d0 + 1;
          }
          else {
            if ((ulonglong)((longlong)local_320 - (longlong)puVar22) >> 3 <= local_120)
            goto LAB_1421a4900;
            puVar22 = puVar22 + local_120;
LAB_1421a4853:
            local_490 = local_3e8;
            local_3e8 = local_3e8 + 1;
            uVar18 = 0;
            uVar27 = 0;
            ppuVar14 = &local_4b0;
            puVar23 = puVar22 + 1;
            local_3f8 = local_1f8;
            local_3f0 = local_1d0;
            local_1f8 = puVar1;
            local_1d0 = local_1d0 + 1;
            local_1c8 = (ulonglong *)pcVar21;
          }
        }
        local_1d0 = local_320;
LAB_1421a48e9:
        uVar27 = 0;
        local_1f8 = puVar1;
      } while( true );
    }
    local_368 = *(undefined8 *)(lVar24 + 0xc);
    uStack_360 = *(undefined8 *)(lVar24 + 0x14);
    local_f1 = 1;
    local_f2 = 1;
    local_f3 = 1;
    country_core::resources::roofs::roof_visual_shape::get_roof_expanded_top_and_bottom();
    local_f1 = 1;
    local_f2 = 1;
    local_f3 = 1;
    country_core::resources::roofs::roof_visual_shape::RoofBoundsDims::as_circle();
    local_120 = CONCAT44(local_120._4_4_,in_XMM1_Da);
    local_f1 = 1;
    local_f2 = 1;
    local_f3 = 1;
    country_core::resources::roofs::roof_shape::Roof::is_flat();
    if (extraout_AL_02 == '\0') {
      local_120 = CONCAT44(local_120._4_4_,(float)local_120 + 0.13);
    }
    local_f1 = 1;
    local_f2 = 1;
    local_f3 = 1;
    country_core::resources::roofs::roof_shape::Roof::profile_curve_normalized();
    country_core::resources::roofs::roof_shape::Roof::height();
    country_core::resources::roofs::roof_shape::profile_curve_ws();
    local_1c0 = CONCAT44(local_1c0._4_4_,(float)(int)local_2d8);
    uVar27 = 0;
    while (uStack_178 = &local_1b8, uVar27 < local_2d8) {
      fVar31 = (float)(longlong)uVar27 / (float)local_1c0;
      local_298 = CONCAT44(local_298._4_4_,fVar31);
      if ((fVar31 < 0.0) || (1.0 < fVar31)) {
        local_1b8 = &local_298;
        uStack_1b0 = 0x40d8c910;
        uStack_1ac = 1;
        local_188 = &anon_6eb2c319c8ba7444a890b1dfc3e489f9_1_llvm_17463645933293978377;
        fStack_180 = 1.4013e-45;
        uStack_17c = 0;
        local_168 = 0;
        uStack_170 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar31)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      _local_278 = ZEXT416((uint)fVar31);
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                ();
      pplVar11 = local_238;
      puVar22 = puStack_240;
      if (local_238 <= (ulonglong)extraout_EAX) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (local_238 <= (ulonglong)extraout_EAX + 1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      country_core::resources::roofs::roof_shape::Roof::is_flat();
      if (extraout_AL_03 == '\0') {
        ::utils::curve::Curve::
        get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                  ();
        uVar28 = (ulonglong)extraout_EAX_00;
        if (pplVar11 <= uVar28) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (pplVar11 <= uVar28 + 1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar31 = (1.0 - extraout_XMM0_Da_00) * *(float *)((longlong)puVar22 + uVar28 * 8 + 4);
      }
      uVar42 = local_368;
      country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz();
      fVar53 = (float)uVar42;
      fVar32 = (float)((ulonglong)uVar42 >> 0x20);
      local_188 = (undefined **)
                  CONCAT44(fVar31 * (float)local_278._0_4_ + fVar32,
                           extraout_XMM0_Da_01 * (float)local_278._0_4_ + fVar53);
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::circle::Circle2d::new_rotated();
      fVar31 = ((float)(longlong)uVar27 + 1.0) / (float)local_1c0;
      local_298 = CONCAT44(local_298._4_4_,fVar31);
      if ((fVar31 < 0.0) || (1.0 < fVar31)) {
        local_1b8 = &local_298;
        uStack_1b0 = 0x40d8c910;
        uStack_1ac = 1;
        local_188 = &anon_6eb2c319c8ba7444a890b1dfc3e489f9_1_llvm_17463645933293978377;
        fStack_180 = 1.4013e-45;
        uStack_17c = 0;
        local_168 = 0;
        uStack_170 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar31)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_2a8 = ZEXT416((uint)fVar31);
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                ();
      if (pplVar11 <= (ulonglong)extraout_EAX_01) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      if (pplVar11 <= (ulonglong)extraout_EAX_01 + 1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      country_core::resources::roofs::roof_shape::Roof::is_flat();
      if (extraout_AL_04 == '\0') {
        ::utils::curve::Curve::
        get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                  ();
        uVar28 = (ulonglong)extraout_EAX_02;
        if (pplVar11 <= uVar28) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (pplVar11 <= uVar28 + 1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar31 = (1.0 - extraout_XMM0_Da_02) * *(float *)((longlong)puVar22 + uVar28 * 8 + 4);
      }
      country_core::resources::roofs::roof_shape::Roof::roof_tip_offset_xz();
      local_188 = (undefined **)
                  CONCAT44(fVar32 + fVar31 * (float)local_2a8._0_4_,
                           fVar53 + extraout_XMM0_Da_03 * (float)local_2a8._0_4_);
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::circle::Circle2d::new_rotated();
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                ();
      uVar28 = (ulonglong)extraout_EAX_03;
      if (pplVar11 <= uVar28 + 1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      fVar31 = *(float *)(puVar22 + uVar28 + 1) - *(float *)(puVar22 + uVar28);
      fVar53 = *(float *)((longlong)puVar22 + uVar28 * 8 + 0xc) -
               *(float *)((longlong)puVar22 + uVar28 * 8 + 4);
      ::utils::curve::Curve::
      get_coord_at_u<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic2D>_>
                ();
      uVar28 = (ulonglong)extraout_EAX_04;
      if (pplVar11 <= uVar28 + 1) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      local_2e0 = uVar27 + 1;
      fVar32 = 1.0 / SQRT(fVar53 * fVar53 + fVar31 * fVar31);
      local_2f8._0_4_ = fVar53 * fVar32;
      fVar53 = (float)puVar22[uVar28 + 1] - (float)puVar22[uVar28];
      fVar54 = (float)(puVar22[uVar28 + 1] >> 0x20) - (float)(puVar22[uVar28] >> 0x20);
      fVar41 = 1.0 / SQRT(fVar54 * fVar54 + fVar53 * fVar53);
      fVar32 = -fVar31 * fVar32;
      local_318._0_4_ = fVar54 * fVar41;
      local_318._4_4_ = -fVar53 * fVar41;
      local_2f8._4_4_ = fVar32;
      fStack_2f0 = -0.0;
      register0x0000130c = 0x80000000;
      local_308._4_4_ = local_2f8._0_4_;
      local_308._0_4_ = 0x80000000;
      fStack_300 = fVar32;
      register0x0000120c = 0;
      fStack_310 = -0.0;
      fStack_30c = -0.0;
      local_438._4_4_ = local_318._0_4_;
      local_438._0_4_ = 0x80000000;
      fStack_430 = (float)local_318._4_4_;
      register0x0000128c = 0;
      for (uVar28 = 0; uVar27 = local_2e0, uVar28 < 0xf; uVar28 = uVar28 + 1) {
        fVar31 = (float)(longlong)uVar28 / 15.0;
        local_268 = CONCAT44(local_268._4_4_,fVar31);
        if (1.0 < fVar31) {
          local_1b8 = &local_268;
          uStack_1b0 = 0x40d8c910;
          uStack_1ac = 1;
          local_188 = &anon_6eb2c319c8ba7444a890b1dfc3e489f9_1_llvm_17463645933293978377;
          fStack_180 = 1.4013e-45;
          uStack_17c = 0;
          local_168 = 0;
          uStack_170 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        fVar53 = ((float)(longlong)uVar28 + 1.0) / 15.0;
        local_268 = CONCAT44(local_268._4_4_,fVar53);
        if (1.0 < fVar53) {
          local_1b8 = &local_268;
          uStack_1b0 = 0x40d8c910;
          uStack_1ac = 1;
          local_188 = &anon_6eb2c319c8ba7444a890b1dfc3e489f9_1_llvm_17463645933293978377;
          fStack_180 = 1.4013e-45;
          uStack_17c = 0;
          local_168 = 0;
          uStack_170 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        ::utils::circular_f32::CircularF32::as_f32();
        fVar32 = (fVar31 * 6.2831855 - extraout_XMM0_Da_04) * 0.5;
        fVar41 = sinf(fVar32);
        fVar54 = cosf(fVar32);
        fVar32 = (fVar53 * 6.2831855 - extraout_XMM0_Da_04) * 0.5;
        local_448 = sinf(fVar32);
        fVar33 = cosf(fVar32);
        fVar32 = fVar31;
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        local_1b8._4_4_ = fVar32;
        ::utils::swizzlers::impl$0::x0y();
        fVar48 = (float)((ulonglong)local_4f0 >> 0x20);
        fVar56 = SUB84(local_188,0);
        uVar27 = (ulonglong)local_188 >> 0x20;
        local_1e8._0_4_ = local_4e8;
        fVar47 = fStack_180 + local_4e8;
        fVar32 = fVar53;
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        local_1b8._4_4_ = fVar32;
        ::utils::swizzlers::impl$0::x0y();
        fVar50 = SUB84(local_188,0);
        uVar18 = (ulonglong)local_188 >> 0x20;
        local_1e8._0_4_ = (float)local_1e8._0_4_ + fStack_180;
        fVar32 = fVar31;
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        local_1b8._4_4_ = fVar32;
        ::utils::swizzlers::impl$0::x0y();
        local_2b8 = local_500;
        uStack_2b0 = 0;
        fVar51 = SUB84(local_188,0);
        uVar19 = (ulonglong)local_188 >> 0x20;
        local_208._0_4_ = local_4f8;
        local_2c8 = CONCAT44(local_2c8._4_4_,fStack_180 + local_4f8);
        fVar32 = fVar53;
        ::utils::geometry::circle::Circle2d::get_pos_at_u();
        local_1b8._0_4_ = extraout_XMM0_Da_05;
        local_1b8._4_4_ = fVar32;
        ::utils::swizzlers::impl$0::x0y();
        lVar24 = local_108;
        local_208._0_4_ = (float)local_208._0_4_ + fStack_180;
        local_428 = (ulonglong *)local_188;
        uStack_420 = 0;
        if (local_108 == local_118) {
          alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
        }
        lVar12 = local_128;
        fVar50 = fVar50 + (float)local_4f0;
        fVar32 = (float)uVar18 + fVar48;
        local_408 = fVar51 + (float)local_2b8;
        fStack_404 = (float)uVar19 + local_2b8._4_4_;
        fStack_400 = (float)uStack_2b0 + 0.0;
        fStack_3fc = uStack_2b0._4_4_ + 0.0;
        *(ulonglong *)(lStack_110 + lVar24 * 0x24) =
             CONCAT44((float)uVar27 + fVar48,fVar56 + (float)local_4f0);
        *(float *)(lStack_110 + 8 + lVar24 * 0x24) = fVar47;
        local_418 = (ulonglong *)CONCAT44(fVar32,fVar50);
        uStack_410 = 0;
        *(ulonglong *)(lStack_110 + 0xc + lVar24 * 0x24) = CONCAT44(fVar32,fVar50);
        *(undefined4 *)(lStack_110 + 0x14 + lVar24 * 0x24) = local_1e8._0_4_;
        *(ulonglong *)(lStack_110 + 0x18 + lVar24 * 0x24) = CONCAT44(fStack_404,local_408);
        *(float *)(lStack_110 + 0x20 + lVar24 * 0x24) = (float)local_2c8;
        local_108 = lVar24 + 1;
        if (local_128 == local_138) {
          alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
        }
        uVar27 = local_140;
        *(float *)(lStack_130 + lVar12 * 0x18) = fVar31;
        *(undefined4 *)(lStack_130 + 4 + lVar12 * 0x18) = local_278._0_4_;
        *(float *)(lStack_130 + 8 + lVar12 * 0x18) = fVar53;
        *(undefined4 *)(lStack_130 + 0xc + lVar12 * 0x18) = local_278._0_4_;
        *(float *)(lStack_130 + 0x10 + lVar12 * 0x18) = fVar31;
        *(undefined4 *)(lStack_130 + 0x14 + lVar12 * 0x18) = local_2a8._0_4_;
        local_128 = lVar12 + 1;
        if (local_140 == local_150) {
          alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
        }
        lVar24 = local_108;
        fVar32 = local_448;
        fVar56 = fVar41 * 0.0;
        fVar47 = fVar54 * fVar54;
        fVar48 = fVar56 * fVar56 + fVar41 * fVar41 + fVar56 * fVar56;
        fVar50 = fStack_2f0 * fVar56 +
                 (float)local_2f8._4_4_ * fVar41 + (float)local_2f8._0_4_ * fVar56;
        fVar50 = fVar50 + fVar50;
        fVar54 = fVar54 + fVar54;
        fVar51 = local_448 * 0.0;
        fVar55 = fVar33 * fVar33;
        fVar57 = fVar51 * fVar51 + local_448 * local_448 + fVar51 * fVar51;
        fVar52 = fStack_2f0 * fVar51 +
                 (float)local_2f8._4_4_ * local_448 + (float)local_2f8._0_4_ * fVar51;
        fVar52 = fVar52 + fVar52;
        fVar33 = fVar33 + fVar33;
        fVar58 = (fStack_2f0 * local_448 - fStack_300 * fVar51) * fVar33 +
                 fVar52 * fVar51 + (float)local_2f8._0_4_ * (fVar55 - fVar57);
        fVar59 = ((float)local_2f8._0_4_ * fVar51 - (float)local_308._0_4_ * fVar51) * fVar33 +
                 fVar52 * local_448 + (float)local_2f8._4_4_ * (fVar55 - fVar57);
        fVar49 = ((float)local_2f8._4_4_ * fVar51 - (float)local_308._4_4_ * local_448) * fVar33 +
                 fVar52 * fVar51 + fStack_2f0 * (fVar55 - fVar57);
        fVar52 = fStack_310 * fVar56 +
                 (float)local_318._4_4_ * fVar41 + (float)local_318._0_4_ * fVar56;
        fVar52 = fVar52 + fVar52;
        uVar42 = CONCAT44((fVar56 * (float)local_318._0_4_ - fVar56 * (float)local_438._0_4_) *
                          fVar54 + fVar52 * fVar41 + (fVar47 - fVar48) * (float)local_318._4_4_,
                          (fVar41 * fStack_310 - fVar56 * fStack_430) * fVar54 +
                          fVar52 * fVar56 + (fVar47 - fVar48) * (float)local_318._0_4_);
        fVar52 = (fVar56 * (float)local_318._4_4_ - fVar41 * (float)local_438._4_4_) * fVar54 +
                 fVar52 * fVar56 + (fVar47 - fVar48) * fStack_310;
        *(ulonglong *)((longlong)puStack_148 + uVar27 * 0x24) =
             CONCAT44(((float)local_2f8._0_4_ * fVar56 - (float)local_308._0_4_ * fVar56) * fVar54 +
                      fVar50 * fVar41 + (float)local_2f8._4_4_ * (fVar47 - fVar48),
                      (fStack_2f0 * fVar41 - fStack_300 * fVar56) * fVar54 +
                      fVar50 * fVar56 + (float)local_2f8._0_4_ * (fVar47 - fVar48));
        *(float *)((longlong)puStack_148 + uVar27 * 0x24 + 8) =
             ((float)local_2f8._4_4_ * fVar56 - (float)local_308._4_4_ * fVar41) * fVar54 +
             fVar50 * fVar56 + fStack_2f0 * (fVar47 - fVar48);
        *(ulonglong *)((longlong)puStack_148 + uVar27 * 0x24 + 0xc) = CONCAT44(fVar59,fVar58);
        *(float *)((longlong)puStack_148 + uVar27 * 0x24 + 0x14) = fVar49;
        *(undefined8 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x18) = uVar42;
        *(float *)((longlong)puStack_148 + uVar27 * 0x24 + 0x20) = fVar52;
        local_140 = uVar27 + 1;
        if (local_108 == local_118) {
          alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
        }
        lVar12 = local_128;
        *(ulonglong **)(lStack_110 + lVar24 * 0x24) = local_418;
        *(undefined4 *)(lStack_110 + 8 + lVar24 * 0x24) = local_1e8._0_4_;
        *(ulonglong *)(lStack_110 + 0xc + lVar24 * 0x24) =
             CONCAT44(local_2b8._4_4_ + local_428._4_4_,(float)local_2b8 + (float)local_428);
        *(undefined4 *)(lStack_110 + 0x14 + lVar24 * 0x24) = local_208._0_4_;
        *(ulonglong *)(lStack_110 + 0x18 + lVar24 * 0x24) = CONCAT44(fStack_404,local_408);
        *(float *)(lStack_110 + 0x20 + lVar24 * 0x24) = (float)local_2c8;
        local_108 = lVar24 + 1;
        if (local_128 == local_138) {
          alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
        }
        uVar27 = local_140;
        *(float *)(lStack_130 + lVar12 * 0x18) = fVar53;
        *(undefined4 *)(lStack_130 + 4 + lVar12 * 0x18) = local_278._0_4_;
        *(float *)(lStack_130 + 8 + lVar12 * 0x18) = fVar53;
        *(undefined4 *)(lStack_130 + 0xc + lVar12 * 0x18) = local_2a8._0_4_;
        *(float *)(lStack_130 + 0x10 + lVar12 * 0x18) = fVar31;
        *(undefined4 *)(lStack_130 + 0x14 + lVar12 * 0x18) = local_2a8._0_4_;
        local_128 = lVar12 + 1;
        if (local_140 == local_150) {
          alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
        }
        fVar31 = fStack_310 * fVar51 +
                 (float)local_318._4_4_ * fVar32 + (float)local_318._0_4_ * fVar51;
        fVar31 = fVar31 + fVar31;
        *(ulonglong *)((longlong)puStack_148 + uVar27 * 0x24) = CONCAT44(fVar59,fVar58);
        *(float *)((longlong)puStack_148 + uVar27 * 0x24 + 8) = fVar49;
        *(ulonglong *)((longlong)puStack_148 + uVar27 * 0x24 + 0xc) =
             CONCAT44(fVar33 * (fVar51 * (float)local_318._0_4_ - fVar51 * (float)local_438._0_4_) +
                      fVar31 * fVar32 + (fVar55 - fVar57) * (float)local_318._4_4_,
                      fVar33 * (fVar32 * fStack_310 - fVar51 * fStack_430) +
                      fVar31 * fVar51 + (fVar55 - fVar57) * (float)local_318._0_4_);
        *(float *)((longlong)puStack_148 + uVar27 * 0x24 + 0x14) =
             fVar33 * (fVar51 * (float)local_318._4_4_ - fVar32 * (float)local_438._4_4_) +
             fVar31 * fVar51 + (fVar55 - fVar57) * fStack_310;
        *(undefined8 *)((longlong)puStack_148 + uVar27 * 0x24 + 0x18) = uVar42;
        *(float *)((longlong)puStack_148 + uVar27 * 0x24 + 0x20) = fVar52;
        local_140 = uVar27 + 1;
      }
    }
    ::utils::geometry::circle::Circle2d::new_rotated();
    in_XMM1_Da = 0.05;
    ::utils::geometry::circle::Circle2d::new_rotated();
    for (uVar27 = 0; uVar27 < 0xf; uVar27 = uVar27 + 1) {
      fVar31 = (float)(longlong)uVar27 / 15.0;
      local_268 = CONCAT44(local_268._4_4_,fVar31);
      if (1.0 < fVar31) {
        local_1b8 = &local_268;
        uStack_1b0 = 0x40d8c910;
        uStack_1ac = 1;
        local_188 = &anon_6eb2c319c8ba7444a890b1dfc3e489f9_1_llvm_17463645933293978377;
        fStack_180 = 1.4013e-45;
        uStack_17c = 0;
        local_168 = 0;
        uStack_170 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      fVar53 = ((float)(longlong)uVar27 + 1.0) / 15.0;
      local_268 = CONCAT44(local_268._4_4_,fVar53);
      if (1.0 < fVar53) {
        local_1b8 = &local_268;
        uStack_1b0 = 0x40d8c910;
        uStack_1ac = 1;
        local_188 = &anon_6eb2c319c8ba7444a890b1dfc3e489f9_1_llvm_17463645933293978377;
        fStack_180 = 1.4013e-45;
        uStack_17c = 0;
        local_168 = 0;
        uStack_170 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      country_core::resources::roofs::roof_shape::Roof::is_flat();
      if (extraout_AL_05 == '\0') {
        country_core::resources::roofs::roof_shape::Roof::height();
      }
      fVar32 = fVar31;
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      local_188 = (undefined **)CONCAT44(fVar32,extraout_XMM0_Da_06);
      ::utils::swizzlers::impl$1::xvy();
      fVar32 = fVar53;
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      local_188 = (undefined **)CONCAT44(fVar32,extraout_XMM0_Da_07);
      ::utils::swizzlers::impl$1::xvy();
      ::utils::swizzlers::impl$1::xvy();
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      lVar24 = local_108;
      uVar28 = local_298;
      fStack_180 = (float)local_54c;
      local_188 = (undefined **)local_554;
      uStack_178._4_4_ = (float)local_558;
      uStack_17c = (undefined4)local_560;
      uStack_178._0_4_ = (float)((ulonglong)local_560 >> 0x20);
      local_168 = CONCAT44(local_168._4_4_,local_564);
      uStack_170 = local_56c;
      if (local_108 == local_118) {
        alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
      }
      lVar12 = local_128;
      *(undefined4 *)(lStack_110 + 0x20 + lVar24 * 0x24) = (undefined4)local_168;
      puVar2 = (undefined8 *)(lStack_110 + 0x10 + lVar24 * 0x24);
      *puVar2 = CONCAT44(uStack_178._4_4_,(float)uStack_178);
      puVar2[1] = uStack_170;
      puVar2 = (undefined8 *)(lStack_110 + lVar24 * 0x24);
      *puVar2 = local_188;
      puVar2[1] = CONCAT44(uStack_17c,fStack_180);
      local_108 = lVar24 + 1;
      if (local_128 == local_138) {
        alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
      }
      uVar18 = local_140;
      *(undefined4 *)(lStack_130 + lVar12 * 0x18) = extraout_XMM0_Da_08;
      *(float *)(lStack_130 + 4 + lVar12 * 0x18) = fVar31;
      *(undefined4 *)(lStack_130 + 8 + lVar12 * 0x18) = extraout_XMM0_Da_09;
      *(float *)(lStack_130 + 0xc + lVar12 * 0x18) = fVar53;
      *(ulonglong *)(lStack_130 + 0x10 + lVar12 * 0x18) = uVar28;
      local_128 = lVar12 + 1;
      local_188 = (undefined **)0x3f80000000000000;
      fStack_180 = 0.0;
      uStack_17c = 0;
      uStack_178._0_4_ = 1.0;
      uStack_178._4_4_ = 0.0;
      uStack_170 = 0x3f80000000000000;
      local_168 = local_168 & 0xffffffff00000000;
      if (local_140 == local_150) {
        alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
      }
      *(undefined4 *)((longlong)puStack_148 + uVar18 * 0x24 + 0x20) = (undefined4)local_168;
      pfVar17 = (float *)((longlong)puStack_148 + uVar18 * 0x24 + 0x10);
      *pfVar17 = (float)uStack_178;
      pfVar17[1] = uStack_178._4_4_;
      pfVar17[2] = (float)uStack_170;
      pfVar17[3] = uStack_170._4_4_;
      puVar2 = (undefined8 *)((longlong)puStack_148 + uVar18 * 0x24);
      *puVar2 = local_188;
      puVar2[1] = CONCAT44(uStack_17c,fStack_180);
      local_140 = uVar18 + 1;
      in_XMM1_Da = (float)uStack_178;
    }
    if (local_248 != (undefined **)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_230 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    lVar24 = local_3a0;
    if (local_3b8 != (longlong *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      lVar24 = local_3a0;
    }
joined_r0x0001421a59e4:
    if (lVar24 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_1e8 = (undefined1  [8])(lStack_110 + local_108 * 0x24);
    lVar24 = lStack_110;
    while ((undefined1  [8])lVar24 != local_1e8) {
      local_f1 = 1;
      local_f2 = 1;
      local_f3 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      lVar24 = lVar24 + 0x24;
      for (lVar12 = 0; lVar12 != 0x24; lVar12 = lVar12 + 0xc) {
        in_XMM1_Da = *(float *)(extraout_RAX + 4 + lVar12) - *(float *)(local_100 + 0x4c);
        local_f1 = 1;
        local_f2 = 1;
        local_f3 = 1;
        country_core::resources::roofs::roof_noise::apply_roof_noise();
      }
    }
    *(undefined4 *)(local_328 + local_338 * 4) = local_1ec;
    local_238 = (longlong **)local_108;
    local_248 = (undefined **)local_118;
    puStack_240 = (ulonglong *)lStack_110;
    local_f1 = 1;
    local_f2 = 1;
    local_f3 = 0;
    country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::set_triangles();
    *(undefined4 *)(local_328 + local_338 * 4) = local_1ec;
    local_238 = (longlong **)local_128;
    local_248 = (undefined **)local_138;
    puStack_240 = (ulonglong *)lStack_130;
    local_f1 = 1;
    local_f2 = 0;
    local_f3 = 0;
    country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::set_uvs();
    *(undefined4 *)(local_328 + local_338 * 4) = local_1ec;
    local_238 = (longlong **)local_140;
    local_248 = (undefined **)local_150;
    puStack_240 = puStack_148;
    local_f1 = 0;
    local_f2 = 0;
    local_f3 = 0;
    country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::set_normals();
    lVar12 = local_480;
    lVar15 = local_488;
    if (local_480 != 0) goto LAB_1421a3368;
LAB_1421a3380:
    lVar12 = lVar15;
    if ((lVar12 == 0) || (lVar12 == local_470)) {
      return;
    }
    lVar15 = lVar12 + 0x20;
    lVar24 = 0;
  } while( true );
}

