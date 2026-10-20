// name: process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>
// addr: 0x141238d80
// size: 4680
// signature: void __cdecl process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env$0>(void)


void __cdecl
system_wall_constructor::construct_fences::process_segment::
process_segment<system_wall_constructor::construct_fences::construct_fences::closure_env_0>(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  float fVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  ulonglong uVar9;
  float fVar10;
  bool bVar11;
  char extraout_AL;
  char extraout_AL_00;
  float *pfVar12;
  float *extraout_RAX;
  ulonglong uVar13;
  undefined1 *extraout_RAX_00;
  undefined1 *puVar14;
  longlong lVar15;
  ulonglong extraout_RAX_01;
  undefined8 in_RCX;
  float *pfVar16;
  float *pfVar17;
  float *pfVar18;
  float *pfVar19;
  undefined8 *puVar20;
  undefined8 *puVar21;
  int iVar22;
  ulonglong uVar23;
  longlong lVar24;
  undefined8 *puVar25;
  undefined8 *puVar26;
  longlong in_R8;
  ulonglong in_R9;
  ulonglong uVar27;
  float extraout_XMM0_Da;
  float fVar28;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float fVar29;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  float in_XMM1_Da;
  float fVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  float fVar38;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  longlong in_stack_00000038;
  ulonglong in_stack_00000040;
  longlong in_stack_00000048;
  longlong in_stack_00000050;
  longlong local_2b0;
  float *local_2a8;
  longlong local_2a0;
  float local_278;
  float fStack_274;
  float local_258;
  float fStack_254;
  float local_248;
  float fStack_244;
  float local_238;
  float fStack_234;
  float *local_218;
  float local_1b8;
  float fStack_1b4;
  float fStack_1b0;
  undefined4 uStack_1ac;
  longlong local_1a8;
  longlong lStack_1a0;
  undefined8 local_198;
  float local_188;
  int iStack_184;
  undefined4 local_17c;
  ulonglong local_178;
  undefined8 local_170;
  undefined8 local_168;
  undefined8 *puStack_160;
  ulonglong local_158;
  longlong lStack_150;
  undefined8 local_148;
  float local_138;
  float local_130;
  float fStack_12c;
  float fStack_128;
  longlong local_120;
  ulonglong local_118;
  float *local_110;
  ulonglong local_108;
  float local_100;
  undefined8 uStack_fc;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  if (1 < in_R9) {
    split_curve_based_on_angle();
    puVar25 = (undefined8 *)0x0;
    pfVar16 = (float *)&DAT_00000048;
    auVar7._8_8_ = 0;
    auVar7._0_8_ = in_R9;
    pfVar12 = SUB168(auVar7 * ZEXT816(0x48),0);
    if ((SUB168(auVar7 * ZEXT816(0x48),8) != 0) || ((float *)0x7ffffffffffffffc < pfVar12)) {
LAB_141238e8f:
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (pfVar12 == (float *)0x0) {
      local_110 = (float *)&DAT_00000004;
      local_118 = 0;
    }
    else {
      puVar25 = (undefined8 *)&DAT_00000004;
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_110 = extraout_RAX;
      pfVar16 = pfVar12;
      local_118 = in_R9;
      if (extraout_RAX == (float *)0x0) goto LAB_141238e8f;
    }
    local_178 = in_stack_00000048;
    fVar34 = in_XMM1_Da + 0.3;
    local_108 = 0;
    pfVar12 = local_2a8 + local_2a0 * 2;
    bVar11 = true;
    local_170 = 0;
    local_120 = 0;
    pfVar18 = local_2a8;
    while (pfVar18 != pfVar12) {
      local_2a8 = pfVar18 + 2;
      pfVar17 = pfVar16;
      pfVar19 = local_2a8;
      pfVar16 = pfVar18;
      if (bVar11) {
        if (local_2a8 == pfVar12) break;
        pfVar19 = pfVar18 + 4;
        pfVar17 = pfVar18;
        pfVar16 = local_2a8;
      }
      uVar13 = *(ulonglong *)pfVar16;
      if (uVar13 == 0xffffffffffffffff) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_end_index_overflow_fail();
      }
      uVar6 = *(ulonglong *)pfVar17;
      uVar23 = (uVar13 + 1) - uVar6;
      if (uVar13 + 1 < uVar6) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_index_order_fail();
      }
      if (in_R9 <= uVar13) {
                    // WARNING: Subroutine does not return
        core::slice::index::slice_end_index_len_fail();
      }
      auVar8._8_8_ = 0;
      auVar8._0_8_ = uVar23;
      uVar13 = SUB168(auVar8 * ZEXT816(0xc),0);
      if ((SUB168(auVar8 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar13)) {
LAB_14123a009:
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if (uVar13 == 0) {
        puVar14 = &DAT_00000004;
        uVar27 = 0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        puVar14 = extraout_RAX_00;
        uVar27 = uVar23;
        if (extraout_RAX_00 == (undefined1 *)0x0) {
          local_170 = 4;
          goto LAB_14123a009;
        }
      }
      memcpy(puVar14,(void *)(in_R8 + uVar6 * 0xc),uVar13);
      local_168 = uVar27;
      puStack_160 = (undefined8 *)puVar14;
      local_158 = uVar23;
      ::utils::curve::Curve::
      try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      if (SBORROW8(0,CONCAT44(fStack_1b4,local_1b8))) {
        puStack_160 = (undefined8 *)local_1a8;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_138 = local_188;
      local_168 = CONCAT44(fStack_1b4,local_1b8);
      local_148 = local_198;
      local_158 = local_1a8;
      lStack_150 = lStack_1a0;
      ::utils::geometry::aabb::Aabb2::max_extent();
      fVar28 = (extraout_XMM0_Da + -1.0) / 5.0;
      fVar30 = 0.0;
      if (0.0 <= fVar28) {
        fVar30 = fVar28;
      }
      fVar28 = 1.0;
      if (fVar30 <= 1.0) {
        fVar28 = fVar30;
      }
      fVar30 = (3.0 - (fVar28 + fVar28)) * fVar28 * fVar28;
      ceilf(local_138 / (fVar30 * 1.6099999 + (1.0 - fVar30) * 0.7));
      ::utils::curve::Curve::
      crashy_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
      if (local_168 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (lStack_150 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar15 = local_120 + 1;
      local_168 = CONCAT44(fStack_1b4,local_1b8);
      puStack_160 = (undefined8 *)CONCAT44(uStack_1ac,fStack_1b0);
      local_158 = local_1a8;
      lStack_150 = lStack_1a0;
      puVar2 = (undefined8 *)((longlong)puStack_160 + local_1a8 * 0xc);
      bVar11 = true;
      lVar24 = 0;
      puVar20 = puStack_160;
      fVar30 = (float)local_1a8;
      while (puVar20 != puVar2) {
        puVar1 = (undefined8 *)((longlong)puVar20 + 0xc);
        puVar21 = puVar1;
        puVar26 = puVar25;
        puVar25 = puVar20;
        if (bVar11) {
          puVar25 = puVar2;
          if (puVar1 == puVar2) break;
          puVar21 = puVar20 + 3;
          puVar26 = puVar20;
          puVar25 = puVar1;
        }
        uVar4 = *puVar26;
        fVar29 = (float)uVar4;
        fVar28 = *(float *)(puVar26 + 1);
        uVar5 = *puVar25;
        fVar36 = (float)uVar5;
        fVar3 = *(float *)(puVar25 + 1);
        if (lVar24 == 0 && local_120 == 0) {
          perchance::PerchanceContext::uniform_disc_vec2();
          perchance::PerchanceContext::uniform_f32();
          fVar37 = extraout_XMM0_Da_01 * 0.6 + 0.4;
          local_100 = fVar36 - fVar29;
          fVar30 = fVar3 - fVar28;
          fVar32 = 1.0 / SQRT(fVar30 * fVar30 + local_100 * local_100);
          local_100 = local_100 * fVar32;
          uStack_fc = CONCAT44(uStack_fc._4_4_,fVar32 * fVar30);
          ::utils::swizzlers::impl$0::x0y();
          uVar13 = local_108;
          fVar33 = fStack_1b0;
          fVar40 = fStack_1b4;
          fVar32 = local_1b8;
          if (local_108 == local_118) {
            alloc::raw_vec::RawVec::
            grow_one<system_wall_constructor::construct_fences::process_segment::process_segment::FencePost,alloc::alloc::Global>
                      ();
          }
          fVar35 = extraout_XMM0_Da_00 * fVar37 * fVar37 * 0.3;
          fVar37 = 1.0 / SQRT(fVar35 * fVar35 + 1.0);
          *(undefined8 *)(local_110 + uVar13 * 0x12) = uVar4;
          local_110[(uVar13 * 9 + 1) * 2] = fVar28;
          *(ulonglong *)(local_110 + (uVar13 * 9 + 3) * 2) = CONCAT44(fStack_1b4,local_1b8);
          local_110[(uVar13 * 9 + 4) * 2] = fStack_1b0;
          local_110[(uVar13 * 9 + 5) * 2] = fVar37;
          fVar31 = fVar37 * 0.0;
          fVar35 = fVar35 * fVar37;
          fVar30 = fVar35 * fVar34 + fVar29;
          *(ulonglong *)(local_110 + uVar13 * 0x12 + 3) =
               CONCAT44(fVar37 * fVar34 + (float)((ulonglong)uVar4 >> 0x20),fVar30);
          local_110[uVar13 * 0x12 + 5] = fVar34 * fVar31 + fVar28;
          local_110[uVar13 * 0x12 + 9] = fVar35;
          local_110[uVar13 * 0x12 + 0xb] = fVar31;
          *(ulonglong *)(local_110 + (uVar13 * 9 + 6) * 2) =
               CONCAT44(fVar35 * fVar33 - fVar32 * fVar31,fVar31 * fVar40 - fVar33 * fVar37);
          local_110[(uVar13 * 9 + 7) * 2] = fVar37 * fVar32 - fVar35 * fVar40;
          local_110[uVar13 * 0x12 + 0xf] = 0.3;
          local_110[(uVar13 * 9 + 8) * 2] = fVar34;
          local_110[uVar13 * 0x12 + 0x11] = 0.3;
          local_108 = uVar13 + 1;
        }
        uVar13 = local_158;
        perchance::PerchanceContext::uniform_disc_vec2();
        perchance::PerchanceContext::uniform_f32();
        fVar40 = extraout_XMM0_Da_03 * 0.6 + 0.4;
        fVar29 = fVar36 - fVar29;
        fVar28 = fVar3 - fVar28;
        fVar32 = 1.0 / SQRT(fVar28 * fVar28 + fVar29 * fVar29);
        local_100 = fVar29 * fVar32;
        uStack_fc = CONCAT44(uStack_fc._4_4_,fVar32 * fVar28);
        ::utils::swizzlers::impl$0::x0y();
        uVar6 = local_108;
        fVar32 = fStack_1b0;
        fVar29 = fStack_1b4;
        fVar28 = local_1b8;
        if (local_108 == local_118) {
          alloc::raw_vec::RawVec::
          grow_one<system_wall_constructor::construct_fences::process_segment::process_segment::FencePost,alloc::alloc::Global>
                    ();
        }
        fVar40 = fVar40 * fVar40;
        fVar33 = 0.0;
        if (lVar24 + 2 != uVar13) {
          fVar33 = fVar30 * fVar40 * 0.3;
        }
        lVar24 = lVar24 + 1;
        fVar37 = extraout_XMM0_Da_02 * fVar40 * 0.3;
        fVar40 = 1.0 / SQRT(fVar33 * fVar33 + fVar37 * fVar37 + 1.0);
        *(undefined8 *)(local_110 + uVar6 * 0x12) = uVar5;
        local_110[(uVar6 * 9 + 1) * 2] = fVar3;
        *(ulonglong *)(local_110 + (uVar6 * 9 + 3) * 2) = CONCAT44(fStack_1b4,local_1b8);
        local_110[(uVar6 * 9 + 4) * 2] = fStack_1b0;
        local_110[(uVar6 * 9 + 5) * 2] = fVar40;
        fVar33 = fVar40 * fVar33;
        fVar37 = fVar40 * fVar37;
        fVar30 = fVar37 * fVar34 + fVar36;
        *(ulonglong *)(local_110 + uVar6 * 0x12 + 3) =
             CONCAT44(fVar40 * fVar34 + (float)((ulonglong)uVar5 >> 0x20),fVar30);
        local_110[uVar6 * 0x12 + 5] = fVar34 * fVar33 + fVar3;
        local_110[uVar6 * 0x12 + 9] = fVar37;
        local_110[uVar6 * 0x12 + 0xb] = fVar33;
        local_110[(uVar6 * 9 + 6) * 2] = fVar33 * fVar29 - fVar32 * fVar40;
        (local_110 + (uVar6 * 9 + 6) * 2)[1] = fVar37 * fVar32 - fVar28 * fVar33;
        local_110[(uVar6 * 9 + 7) * 2] = fVar40 * fVar28 - fVar37 * fVar29;
        local_110[uVar6 * 0x12 + 0xf] = 0.3;
        local_110[(uVar6 * 9 + 8) * 2] = fVar34;
        local_110[uVar6 * 0x12 + 0x11] = 0.3;
        local_108 = uVar6 + 1;
        bVar11 = false;
        puVar20 = puVar21;
      }
      if (local_168 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_2a8 = (float *)0x0;
      if (lStack_150 != 0) {
        local_2a8 = (float *)(lStack_150 << 2);
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      bVar11 = false;
      local_120 = lVar15;
      pfVar18 = pfVar19;
    }
    pfVar16 = local_110 + local_108 * 0x12;
    local_178 = local_178 - 1;
    puVar14 = *(undefined1 **)(in_stack_00000050 + 8);
    pfVar12 = local_110;
    lVar15 = 0;
    while (lVar24 = lVar15, pfVar18 = pfVar12, pfVar18 != pfVar16) {
      pfVar12 = pfVar18 + 0x12;
      lVar15 = lVar24 + 1;
      local_2a8 = (float *)CONCAT71((int7)((ulonglong)local_2a8 >> 8),lVar24 == 0);
      if ((in_stack_00000038 == 0 || lVar24 != 0) &&
         ((local_178 <= in_stack_00000040 || (lVar24 != local_108 - 1)))) {
        fVar28 = pfVar18[0x10] + 0.1;
        fVar30 = pfVar18[0x11] + 0.0;
        local_130 = (float)in_RCX;
        fStack_12c = (float)((ulonglong)in_RCX >> 0x20);
        local_100 = fVar28 * (float)*(undefined8 *)(pfVar18 + 9);
        uStack_fc = CONCAT44(pfVar18[0xb] * fVar28,
                             fVar28 * (float)((ulonglong)*(undefined8 *)(pfVar18 + 9) >> 0x20));
        local_168 = CONCAT44(fVar30 * (float)(*(ulonglong *)(pfVar18 + 0xc) >> 0x20),
                             fVar30 * (float)*(ulonglong *)(pfVar18 + 0xc));
        puStack_160 = (undefined8 *)CONCAT44(puStack_160._4_4_,pfVar18[0xe] * fVar30);
        country_core::resources::render::Affine3Packed::from_cols();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        iStack_184 = (int)lVar24;
        local_17c = 0;
        local_2a8 = &local_1b8;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        if (extraout_AL != '\0') {
          *puVar14 = 1;
        }
      }
    }
    pfVar16 = local_110 + local_108 * 0x12;
    bVar11 = true;
    iVar22 = 0;
    pfVar12 = local_110;
    while (pfVar12 != pfVar16) {
      local_218 = pfVar12 + 0x12;
      pfVar18 = pfVar12;
      if (bVar11) {
        if (local_218 == pfVar16) break;
        pfVar18 = local_218;
        local_2a8 = pfVar12;
        local_218 = pfVar12 + 0x24;
      }
      fVar30 = local_2a8[2];
      fVar28 = local_2a8[5];
      fVar3 = pfVar18[2];
      uVar13 = *(ulonglong *)local_2a8;
      uVar4 = *(undefined8 *)(local_2a8 + 3);
      uVar6 = *(ulonglong *)pfVar18;
      uVar5 = *(undefined8 *)(pfVar18 + 3);
      local_178 = CONCAT44(local_178._4_4_,pfVar18[5]);
      perchance::PerchanceContext::uniform_f32();
      perchance::PerchanceContext::uniform_f32();
      iVar22 = iVar22 + 1;
      fVar36 = extraout_XMM0_Da_04 * 0.3 + (1.0 - extraout_XMM0_Da_04) * 0.1;
      fVar29 = extraout_XMM0_Da_05 * 0.8 + (1.0 - extraout_XMM0_Da_05) * 0.6;
      local_170 = CONCAT44(local_170._4_4_,fVar36);
      fVar36 = (fVar29 - fVar36) * fVar34;
      fVar32 = floorf(fVar36 * 4.0);
      uVar23 = (longlong)(fVar32 - 9.223372e+18) & (longlong)fVar32 >> 0x3f | (longlong)fVar32;
      if (fVar32 < 0.0) {
        uVar23 = 0;
      }
      if (1.8446743e+19 < fVar32) {
        uVar23 = 0xffffffffffffffff;
      }
      uVar27 = 4;
      if (uVar23 < 4) {
        uVar27 = uVar23;
      }
      if (uVar23 == 0) {
        uVar27 = 1;
      }
      for (uVar23 = 0; uVar23 < uVar27; uVar23 = uVar23 + 1) {
        fVar32 = 0.5;
        if (uVar27 != 1) {
          fVar32 = (float)(longlong)uVar23 / (float)(longlong)(uVar27 - 1);
        }
        fVar32 = fVar32 * fVar29 + (1.0 - fVar32) * (float)local_170;
        fVar40 = 1.0 - fVar32;
        fVar44 = fVar28 * fVar32 + fVar30 * fVar40;
        local_258 = (float)uVar13;
        fStack_254 = (float)(uVar13 >> 0x20);
        local_248 = (float)uVar4;
        fStack_244 = (float)((ulonglong)uVar4 >> 0x20);
        fVar33 = local_248 * fVar32 + local_258 * fVar40;
        fVar37 = fStack_244 * fVar32 + fStack_254 * fVar40;
        local_278 = (float)uVar6;
        fStack_274 = (float)(uVar6 >> 0x20);
        local_238 = (float)uVar5;
        fStack_234 = (float)((ulonglong)uVar5 >> 0x20);
        fVar42 = fVar32 * local_238 + fVar40 * local_278;
        fVar43 = fVar32 * fStack_234 + fVar40 * fStack_274;
        fVar31 = (float)local_178 * fVar32 + fVar3 * fVar40;
        fVar32 = fVar43 - fVar37;
        fVar40 = fVar31 - fVar44;
        fVar35 = fVar42 - fVar33;
        local_130 = fVar35;
        fStack_12c = fVar32;
        fStack_128 = fVar40;
        perchance::PerchanceContext::uniform_f32();
        if (extraout_XMM0_Da_06 < 0.15) {
          perchance::PerchanceContext::uniform_f32();
          fVar38 = fVar32;
          fVar39 = fVar40;
          fVar41 = fVar35;
          fVar10 = fVar44;
          if (extraout_XMM0_Da_07 < 0.5) {
            fVar41 = -fVar35;
            fVar38 = -fVar32;
            fVar39 = -fVar40;
            uVar9 = CONCAT44(fVar40,fVar32) ^ 0x8000000080000000;
            fStack_12c = (float)uVar9;
            fStack_128 = (float)(uVar9 >> 0x20);
            local_130 = fVar41;
            fVar10 = fVar31;
          }
          local_120 = CONCAT44(local_120._4_4_,fVar10);
          perchance::PerchanceContext::uniform_f32();
          fVar38 = fVar38 - (extraout_XMM0_Da_08 * 0.5 + (1.0 - extraout_XMM0_Da_08) * 0.1) * fVar36
          ;
          local_168 = CONCAT44(fVar35,-fVar40);
          fStack_12c = fVar38;
          ::utils::swizzlers::impl$0::x0y();
          perchance::PerchanceContext::usize_less_than();
          if (1 < extraout_RAX_01) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          fVar40 = *(float *)(&DAT_142c1ed68 + extraout_RAX_01 * 4) * 0.05;
          fVar35 = fVar41 + fVar40 * local_1b8;
          fVar32 = fVar38 + fStack_1b4 * fVar40;
          fVar40 = fVar39 + fStack_1b0 * fVar40;
        }
        fStack_128 = 1.0 / SQRT(fVar40 * fVar40 + fVar35 * fVar35 + fVar32 * fVar32);
        local_130 = fVar35 * fStack_128;
        fStack_12c = fStack_128 * fVar32;
        fStack_128 = fStack_128 * fVar40;
        if ((((local_130 == 0.0) && (!NAN(local_130))) && (fStack_12c == 1.0)) &&
           (((!NAN(fStack_12c) && (fStack_128 == 0.0)) && (!NAN(fStack_128))))) {
          local_1b8 = 96.96423;
          fStack_1b4 = 1.4013e-45;
          fStack_1b0 = 1.4013e-45;
          uStack_1ac = 0;
          local_1a8 = 8;
          lStack_1a0 = 0;
          local_198 = 0;
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<glam::f32::vec3::Vec3,glam::f32::vec3::Vec3>();
        }
        fVar33 = fVar33 - fVar42;
        fVar37 = fVar37 - fVar43;
        fVar32 = SQRT((fVar44 - fVar31) * (fVar44 - fVar31) + fVar37 * fVar37 + fVar33 * fVar33);
        fVar31 = local_130 - fStack_12c * 0.0;
        fVar33 = fStack_12c * 0.0 - fStack_128;
        fVar37 = fStack_128 * 0.0 - local_130 * 0.0;
        fVar40 = 1.0 / SQRT(fVar37 * fVar37 + fVar33 * fVar33 + fVar31 * fVar31);
        local_100 = fVar32 * local_130;
        uStack_fc = CONCAT44(fVar32 * fStack_128,fVar32 * fStack_12c);
        local_168 = CONCAT44((fVar40 * fVar31 * local_130 - fVar40 * fVar33 * fStack_128) * 0.2,
                             (fVar40 * fVar37 * fStack_128 - fVar40 * fVar31 * fStack_12c) * 0.2);
        puStack_160 = (undefined8 *)
                      CONCAT44(puStack_160._4_4_,
                               (fStack_12c * fVar40 * fVar33 - fVar40 * fVar37 * local_130) * 0.2);
        country_core::resources::render::Affine3Packed::from_cols();
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        iStack_184 = (int)uVar23 + iVar22 * 4;
        local_17c = 0;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
        if (extraout_AL_00 != '\0') {
          *puVar14 = 1;
        }
      }
      bVar11 = false;
      pfVar12 = local_218;
      local_2a8 = pfVar18;
    }
    if (local_118 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_2b0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  return;
}

