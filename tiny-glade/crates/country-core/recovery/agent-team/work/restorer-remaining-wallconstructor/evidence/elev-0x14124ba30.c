
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
system_wall_constructor::construct_elevation_supports::circle_pillars::construct_circle_pillars
          (void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [12];
  char extraout_AL;
  bool bVar5;
  char extraout_AL_00;
  int extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  byte *extraout_RAX_01;
  ulonglong uVar6;
  ulonglong uVar7;
  longlong in_RDX;
  ulonglong uVar8;
  longlong lVar9;
  undefined8 *puVar10;
  longlong lVar11;
  undefined8 *puVar12;
  undefined8 *in_R8;
  ulonglong uVar13;
  float extraout_XMM0_Da;
  float extraout_XMM0_Db;
  undefined1 auVar16 [12];
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar14;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float fVar15;
  float extraout_XMM0_Da_06;
  undefined1 extraout_XMM0 [16];
  undefined1 extraout_XMM0_00 [16];
  float extraout_XMM0_Db_00;
  float fVar22;
  float extraout_XMM0_Db_01;
  undefined1 extraout_var [12];
  undefined1 extraout_var_00 [12];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar21 [16];
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  float fVar29;
  float fVar30;
  undefined1 auVar31 [16];
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong *in_stack_00000060;
  float local_228;
  float fStack_224;
  undefined1 local_1f8 [96];
  undefined8 local_198;
  float fStack_190;
  undefined4 uStack_18c;
  undefined4 local_188;
  undefined4 uStack_184;
  undefined8 uStack_180;
  undefined **local_178;
  undefined8 *puStack_170;
  undefined8 *local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 local_140;
  code *local_138;
  undefined8 local_130;
  float local_128;
  float fStack_124;
  longlong local_120;
  undefined8 *puStack_118;
  longlong local_110;
  longlong local_108;
  undefined8 *local_100;
  undefined1 local_f1;
  undefined8 local_f0;
  undefined1 auVar20 [16];
  
  local_f0 = 0xfffffffffffffffe;
  if (*(char *)(in_RDX + 0x298) == '\0') {
    local_120 = 0;
    puStack_118 = (undefined8 *)&DAT_00000008;
    local_110 = 0;
    local_f1 = 1;
    local_108 = in_RDX;
    local_100 = in_R8;
    ::utils::geometry::circle::Circle2d::perimeter();
    local_f1 = 1;
    ::utils::inverse_lerp();
    fVar23 = 0.0;
    if (0.0 <= extraout_XMM0_Da) {
      fVar23 = extraout_XMM0_Da;
    }
    auVar16 = ZEXT812(0x3f800000);
    if (fVar23 <= 1.0) {
      auVar16._4_8_ = 0;
      auVar16._0_4_ = fVar23;
    }
    auVar4._4_8_ = auVar16._4_8_;
    auVar4._0_4_ = auVar16._0_4_ * 7.0 + (1.0 - auVar16._0_4_) * 4.0;
    fVar23 = (float)roundf(auVar4._0_8_);
    uVar8 = (longlong)(fVar23 - 9.223372e+18) & (longlong)fVar23 >> 0x3f | (longlong)fVar23;
    if (fVar23 < 0.0) {
      uVar8 = 0;
    }
    uVar7 = 0xffffffffffffffff;
    if (fVar23 <= 1.8446743e+19) {
      uVar7 = uVar8;
    }
    fVar23 = (float)uVar7;
    lVar11 = in_stack_00000030 * 0x50 + in_stack_00000028;
    uVar2 = *local_100;
    uVar8 = 0;
LAB_14124bbf0:
    uVar13 = uVar8;
    if (uVar13 < uVar7) {
      fVar33 = (float)uVar13 / fVar23;
      local_140 = (undefined8 *)CONCAT44(local_140._4_4_,fVar33);
      if ((fVar33 < 0.0) || (1.0 < fVar33)) {
        local_198 = &local_140;
        fStack_190 = 6.774544;
        uStack_18c = 1;
        local_178 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
        puStack_170 = (undefined8 *)0x1;
        local_158 = 0;
        local_168 = &local_198;
        uStack_160 = 1;
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (NAN(fVar33)) {
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_f1 = 1;
      fVar26 = fVar33;
      ::utils::geometry::circle::Circle2d::get_pos_at_u();
      fVar24 = *(float *)(local_108 + 0x280);
      local_f1 = 1;
      local_128 = extraout_XMM0_Da_00;
      fStack_124 = fVar26;
      country_core::resources::terrain_height::TerrainHeightsData::
      sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
      uVar8 = uVar13 + 1;
      lVar9 = in_stack_00000028;
      if (extraout_XMM0_Da_01 + 0.05 <= fVar24) {
        do {
          if (lVar9 == lVar11) {
            local_f1 = 1;
            ::utils::swizzlers::impl$1::xvy();
            local_f1 = 1;
            fVar24 = fVar33;
            ::utils::geometry::circle::Circle2d::get_normal_at_u();
            local_140 = (undefined8 *)CONCAT44(fVar24,extraout_XMM0_Da_02);
            local_f1 = 1;
            ::utils::swizzlers::impl$0::x0y();
            local_198._4_4_ = (float)*local_100;
            fStack_190 = (float)((ulonglong)*local_100 >> 0x20);
            uStack_18c = (undefined4)local_100[1];
            local_188 = (undefined4)((ulonglong)local_100[1] >> 0x20);
            local_198._0_4_ = 0.0;
            local_f1 = 1;
            rectangle_pillars::find_stone_column_start_y();
            if (extraout_EAX == 1) {
              fVar24 = *(float *)(local_108 + 0x280);
              fVar14 = ((fVar24 - extraout_XMM0_Da_03) + -5.0) / 5.0;
              fVar26 = 0.0;
              if (0.0 <= fVar14) {
                fVar26 = fVar14;
              }
              fVar14 = 1.0;
              if (fVar26 <= 1.0) {
                fVar14 = fVar26;
              }
              fVar14 = (3.0 - (fVar14 + fVar14)) * fVar14 * fVar14;
              local_f1 = 1;
              fVar26 = 0.0;
              ::utils::geometry::circle::Circle2d::get_normal_at_u();
              fVar36 = fVar14 * 1.5 + (1.0 - fVar14);
              fVar14 = local_128 + fVar36 * extraout_XMM0_Da_04 * -0.4;
              fVar34 = fStack_124 + fVar36 * fVar33 * -0.4;
              fVar35 = extraout_XMM0_Db * 0.0 * 0.0 + 0.0;
              fVar26 = fVar26 * 0.0 * 0.0 + 0.0;
              bVar5 = false;
              fVar33 = 2.5;
              lVar9 = in_stack_00000028;
              goto LAB_14124bed0;
            }
            break;
          }
          local_f1 = 1;
          country_core::resources::walls::public_walls::PublicWalls::get();
          if (extraout_RAX == 0) {
            local_f1 = 1;
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          local_f1 = 1;
          country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
          local_f1 = 1;
          country_core::resources::glade_border::GladeBorder::is_pos_inside();
          lVar9 = lVar9 + 0x50;
        } while (extraout_AL == '\0');
      }
      goto LAB_14124bbf0;
    }
    local_f1 = 1;
    minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
    local_f1 = 1;
    ::utils::rng::init_rng();
    puVar10 = puStack_118 + local_110 * 8;
    for (puVar12 = puStack_118; puVar12 != puVar10; puVar12 = puVar12 + 8) {
      local_130 = *puVar12;
      fVar23 = *(float *)(puVar12 + 6);
      uVar2 = puVar12[5];
      local_f1 = 1;
      ::utils::swizzlers::impl$1::xvy();
      local_198._0_4_ = fVar23 * 0.5;
      local_198._4_4_ = (float)((ulonglong)uVar2 >> 0x20) - (float)uVar2;
      local_f1 = 1;
      fStack_190 = (float)local_198;
      ::utils::geometry::rectangle::Rectangle3d::new();
      uStack_180 = uStack_160;
      local_198._0_4_ = SUB84(local_178,0);
      local_198._4_4_ = (float)((ulonglong)local_178 >> 0x20);
      fStack_190 = SUB84(puStack_170,0);
      uStack_18c = (undefined4)((ulonglong)puStack_170 >> 0x20);
      local_188 = SUB84(local_168,0);
      uStack_184 = (undefined4)((ulonglong)local_168 >> 0x20);
      lVar11 = in_stack_00000060[2];
      if (lVar11 == *in_stack_00000060) {
        local_f1 = 1;
        alloc::raw_vec::RawVec::
        grow_one<enum2_<toolbar::mei::config::NodeLayoutConfigAttrib>,alloc::alloc::Global>();
      }
      lVar9 = in_stack_00000060[1];
      puVar1 = (undefined8 *)(lVar9 + 0x10 + lVar11 * 0x20);
      *puVar1 = CONCAT44(uStack_184,local_188);
      puVar1[1] = uStack_180;
      puVar1 = (undefined8 *)(lVar9 + lVar11 * 0x20);
      *puVar1 = CONCAT44(local_198._4_4_,(float)local_198);
      puVar1[1] = CONCAT44(uStack_18c,fStack_190);
      in_stack_00000060[2] = lVar11 + 1;
    }
    local_f1 = 1;
    country_core::resources::walls::inner_wall_state::WallStyle::is_halftimbered();
    if (extraout_AL_00 == '\0') {
      local_168 = (undefined8 *)local_110;
      local_178 = (undefined **)local_120;
      puStack_170 = puStack_118;
      local_f1 = 0;
      util_pillar_construction::construct_brick_columns();
    }
    else {
      local_f1 = 1;
      util_pillar_construction::construct_wooden_pillar_w_stone_base();
      if (local_120 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  return;
LAB_14124bed0:
  if (lVar9 != lVar11) {
    local_f1 = 1;
    country_core::resources::walls::public_walls::PublicWalls::get();
    if (extraout_RAX_00 == 0) {
      local_f1 = 1;
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    local_f1 = 1;
    country_core::resources::walls::public_wall_state::PublicWallState::get_wall_shape();
    lVar9 = lVar9 + 0x50;
    if ((*extraout_RAX_01 & 1) == 0) goto LAB_14124bf50;
    local_f1 = 1;
    ::utils::geometry::rectangle::Rectangle2d::distance_to_closest_edge();
    if (extraout_XMM0._0_4_ < fVar33) {
      local_1f8._8_8_ = 0;
      local_1f8._0_8_ = *(ulonglong *)(extraout_RAX_01 + 4);
      fVar33 = extraout_XMM0._0_4_;
      goto LAB_14124bfa7;
    }
    goto LAB_14124bed0;
  }
  local_228 = (float)local_1f8._0_4_;
  fVar33 = (float)local_1f8._4_4_;
  if (!bVar5) {
    local_228 = (float)uVar2;
    fStack_224 = (float)((ulonglong)uVar2 >> 0x20);
    local_228 = local_228 - fVar14;
    fStack_224 = fStack_224 - fVar34;
    fVar33 = 1.0 / SQRT(fStack_224 * fStack_224 + local_228 * local_228);
    local_228 = fVar33 * local_228;
    fVar33 = fVar33 * fStack_224;
  }
  if (NAN((float)uVar8 / fVar23)) {
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar27 = fmodf((float)uVar8 / fVar23,1.0);
  local_130._0_4_ = fVar27;
  if ((fVar27 < 0.0) || (1.0 < fVar27)) {
    local_140 = &local_130;
    local_138 = core::fmt::float::impl$5::fmt;
    local_178 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    puStack_170 = (undefined8 *)0x1;
    local_158 = 0;
    local_168 = &local_140;
    uStack_160 = 1;
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar27)) {
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  local_f1 = 1;
  fVar32 = extraout_var._0_4_;
  ::utils::geometry::circle::Circle2d::get_pos_at_u();
  uVar6 = uVar13;
  if (uVar13 == 0) {
    uVar6 = uVar7;
  }
  fVar15 = (float)(uVar6 - 1) / fVar23;
  if (NAN(fVar15)) {
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar15 = fmodf(fVar15,1.0);
  local_130 = CONCAT44(local_130._4_4_,fVar15);
  if ((fVar15 < 0.0) || (1.0 < fVar15)) {
    local_140 = &local_130;
    local_138 = core::fmt::float::impl$5::fmt;
    local_178 = &anon_95101f4d2e0328d894274322bc39dc37_3_llvm_14090887955549727255;
    puStack_170 = (undefined8 *)0x1;
    local_158 = 0;
    local_168 = &local_140;
    uStack_160 = 1;
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_fmt();
  }
  if (NAN(fVar15)) {
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  local_f1 = 1;
  fVar22 = extraout_var_00._0_4_;
  ::utils::geometry::circle::Circle2d::get_pos_at_u();
  lVar9 = local_110;
  if (local_110 == local_120) {
    local_f1 = 1;
    alloc::raw_vec::RawVec::
    grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
  }
  fVar25 = extraout_XMM0_Da_05 - fVar14;
  fVar28 = fVar27 - fVar34;
  fVar29 = extraout_XMM0_Da_06 - fVar14;
  fVar30 = fVar15 - fVar34;
  auVar17._0_4_ = fVar25 * fVar25 + fVar28 * fVar28;
  auVar17._4_4_ = fVar29 * fVar29 + fVar30 * fVar30;
  auVar17._8_4_ = fVar25 * fVar25 + fVar28 * fVar28;
  auVar17._12_4_ = fVar29 * fVar29 + fVar30 * fVar30;
  auVar18 = sqrtps(auVar17,auVar17);
  auVar20._0_8_ = auVar18._0_8_;
  auVar20._8_4_ = auVar18._4_4_;
  auVar20._12_4_ = auVar18._4_4_;
  auVar19._0_4_ = auVar18._0_4_;
  auVar19._8_8_ = auVar20._8_8_;
  auVar19._4_4_ = auVar19._0_4_;
  auVar18 = divps(___xmm_3f8000003f8000003f8000003f800000,auVar19);
  auVar31._0_4_ = auVar18._0_4_ * fVar25;
  auVar31._4_4_ = auVar18._4_4_ * fVar28;
  auVar31._8_4_ = auVar18._8_4_ * fVar29;
  auVar31._12_4_ = auVar18._12_4_ * fVar30;
  auVar21._0_4_ = (fVar14 - extraout_XMM0_Da_05) * (fVar14 - extraout_XMM0_Da_05);
  auVar21._4_4_ = (fVar14 - extraout_XMM0_Da_06) * (fVar14 - extraout_XMM0_Da_06);
  auVar21._8_4_ = (fVar35 - extraout_XMM0_Db_00) * (fVar35 - extraout_XMM0_Db_00);
  auVar21._12_4_ = (fVar35 - extraout_XMM0_Db_01) * (fVar35 - extraout_XMM0_Db_01);
  auVar3._4_4_ = (fVar34 - fVar15) * (fVar34 - fVar15) + auVar21._4_4_;
  auVar3._0_4_ = (fVar34 - fVar27) * (fVar34 - fVar27) + auVar21._0_4_;
  auVar3._8_4_ = (fVar26 - fVar32) * (fVar26 - fVar32) + auVar21._8_4_;
  auVar3._12_4_ = (fVar26 - fVar22) * (fVar26 - fVar22) + auVar21._12_4_;
  auVar18 = sqrtps(auVar21,auVar3);
  puStack_118[lVar9 * 8] = CONCAT44(fVar34,fVar14);
  puStack_118[lVar9 * 8 + 1] = uVar13;
  puStack_118[lVar9 * 8 + 2] = CONCAT44(fVar33,local_228);
  *(undefined1 (*) [16])(puStack_118 + lVar9 * 8 + 3) = auVar31;
  *(float *)(puStack_118 + lVar9 * 8 + 5) = extraout_XMM0_Da_03;
  *(float *)((longlong)puStack_118 + lVar9 * 0x40 + 0x2c) = fVar24;
  *(float *)(puStack_118 + lVar9 * 8 + 6) = fVar36;
  *(longlong *)((longlong)puStack_118 + lVar9 * 0x40 + 0x34) = auVar18._0_8_;
  local_110 = lVar9 + 1;
  goto LAB_14124bbf0;
LAB_14124bf50:
  local_f1 = 1;
  ::utils::geometry::circle::Circle2d::signed_distance();
  auVar18 = extraout_XMM0_00;
  if (extraout_XMM0_00._0_4_ <= 0.0) {
    auVar18._12_4_ = 0;
    auVar18._0_12_ = extraout_XMM0_00._4_12_;
    auVar18 = auVar18 << 0x20;
  }
  if (auVar18._0_4_ < fVar33) {
    uVar6 = *(ulonglong *)(extraout_RAX_01 + 4);
    fVar33 = (float)uVar6 - fVar14;
    fVar27 = (float)(uVar6 >> 0x20) - fVar34;
    fVar32 = 1.0 / SQRT(fVar27 * fVar27 + fVar33 * fVar33);
    local_1f8._0_4_ = fVar32 * fVar33;
    local_1f8._4_4_ = fVar32 * fVar27;
    local_1f8._8_4_ = (0.0 - fVar35) * 0.0;
    local_1f8._12_4_ = (0.0 - fVar26) * 0.0;
    fVar33 = auVar18._0_4_;
LAB_14124bfa7:
    bVar5 = true;
  }
  goto LAB_14124bed0;
}

