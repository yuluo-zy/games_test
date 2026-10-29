
void __cdecl
country_core::resources::walls::wall_trigger_volume::WallTriggerVolume::get_collision_geo(void)

{
  undefined8 *puVar1;
  uint uVar2;
  uint uVar3;
  ulonglong uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  longlong lVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined1 auVar12 [16];
  longlong lVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong lVar14;
  longlong *in_RCX;
  longlong in_RDX;
  byte *in_R8;
  longlong lVar15;
  undefined8 *puVar16;
  float fVar17;
  float fVar18;
  float fVar19;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar20;
  float fVar21;
  float fVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar27;
  undefined1 auVar26 [16];
  float fVar28;
  float fVar29;
  float fVar30;
  uint uVar31;
  uint uVar32;
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
  undefined4 uStack_340;
  undefined4 uStack_33c;
  float local_2f8;
  undefined4 uStack_2f4;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined4 local_2d8;
  undefined4 uStack_2d4;
  undefined4 uStack_2d0;
  undefined4 uStack_2cc;
  undefined4 local_2c8;
  undefined4 uStack_2c4;
  undefined4 uStack_2c0;
  undefined4 uStack_2bc;
  undefined4 uStack_2b8;
  undefined4 uStack_2b4;
  float local_2b0;
  float fStack_2ac;
  float local_2a8;
  longlong *local_2a0;
  undefined8 local_298;
  code *pcStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined4 local_278;
  undefined4 uStack_274;
  undefined4 uStack_270;
  undefined4 uStack_26c;
  undefined4 local_268;
  undefined4 uStack_264;
  undefined4 uStack_260;
  undefined4 uStack_25c;
  undefined4 uStack_258;
  undefined4 uStack_254;
  undefined8 *local_250;
  undefined8 local_248;
  float afStack_240 [2];
  undefined8 uStack_238;
  undefined8 uStack_230;
  undefined1 local_228 [4];
  undefined1 auStack_224 [8];
  undefined1 auStack_21c [8];
  undefined1 auStack_214 [8];
  undefined1 auStack_20c [8];
  undefined4 uStack_204;
  undefined8 local_200;
  undefined4 uStack_1f8;
  undefined4 uStack_1f4;
  undefined8 local_1f0 [10];
  undefined4 local_1a0;
  undefined4 uStack_19c;
  undefined4 uStack_198;
  undefined4 uStack_194;
  undefined8 uStack_190;
  longlong local_188;
  undefined8 local_178;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined4 uStack_168;
  undefined4 uStack_164;
  longlong local_158;
  longlong lStack_150;
  longlong local_148;
  longlong local_140;
  longlong lStack_138;
  longlong local_130;
  float local_128;
  float local_124;
  undefined8 local_120;
  float local_118;
  longlong local_110;
  longlong lStack_108;
  longlong local_100;
  byte local_f3;
  byte local_f2;
  byte local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  lVar15 = *(longlong *)(in_RDX + 0x10);
  if (lVar15 == 0) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  local_2a0 = in_RCX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  lVar7 = lVar15 * 2;
  local_100 = 0;
  local_110 = lVar7;
  lStack_108 = extraout_RAX;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_130 = 0;
  local_140 = lVar7;
  lStack_138 = extraout_RAX_00;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_01 == 0) {
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  local_148 = 0;
  puVar16 = *(undefined8 **)(in_RDX + 8);
  if ((in_R8[0xc] & 1) == 0) {
    fVar17 = 0.0;
  }
  else {
    fVar17 = *(float *)(in_R8 + 0x10);
  }
  local_128 = *(float *)(in_R8 + 4);
  local_124 = *(float *)(in_R8 + 8);
  local_250 = puVar16 + lVar15 * 0xf;
  local_f2 = *in_R8;
  local_f3 = in_R8[0x1c];
  local_f1 = *(byte *)(in_RDX + 0x60);
  uVar2 = *(uint *)(in_R8 + 0x14);
  auVar12._4_4_ = uVar2;
  auVar12._0_4_ = uVar2;
  uVar3 = *(uint *)(in_R8 + 0x18);
  local_158 = lVar7;
  lStack_150 = extraout_RAX_01;
  do {
    if (local_f3 == 0) {
      uStack_170 = *(undefined4 *)(puVar16 + 5);
      local_178 = puVar16[4];
      pcStack_290 = (code *)CONCAT44(pcStack_290._4_4_,*(undefined4 *)((longlong)puVar16 + 0x34));
      local_298 = *(float **)((longlong)puVar16 + 0x2c);
      afStack_240[0] = *(float *)(puVar16 + 8);
      local_248 = (undefined **)puVar16[7];
      ::utils::geometry::intersect::Triangle::new();
      uStack_2cc = *(undefined4 *)(puVar16 + 0xc);
      uStack_2c0 = *(undefined4 *)((longlong)puVar16 + 0x6c);
      afStack_240[1] = (float)*(undefined8 *)((longlong)puVar16 + 100);
      uStack_238._0_4_ = (undefined4)((ulonglong)*(undefined8 *)((longlong)puVar16 + 100) >> 0x20);
      uStack_2b4 = *(undefined4 *)(puVar16 + 0xc);
      _local_228 = CONCAT44(auStack_224._0_4_,uStack_2b4);
      uStack_230 = puVar16[0xb];
      uVar6 = *puVar16;
      uVar8 = *(undefined4 *)(puVar16 + 1);
      uVar9 = *(undefined4 *)((longlong)puVar16 + 0xc);
      uVar11 = puVar16[2];
      uVar10 = puVar16[2];
      uStack_2c4 = (undefined4)uStack_238;
      uStack_2bc = (undefined4)uStack_230;
      uStack_2b8 = (undefined4)((ulonglong)uStack_230 >> 0x20);
      uStack_2d4 = (undefined4)puVar16[0xb];
      uStack_2d0 = (undefined4)((ulonglong)puVar16[0xb] >> 0x20);
      local_2c8 = afStack_240[1];
      local_118 = *(float *)((longlong)puVar16 + 0x34);
      local_120 = *(undefined8 *)((longlong)puVar16 + 0x2c);
      uStack_170 = *(undefined4 *)((longlong)puVar16 + 0x4c);
      local_178 = *(undefined8 *)((longlong)puVar16 + 0x44);
      afStack_240[0] = *(float *)(puVar16 + 8);
      local_248 = (undefined **)puVar16[7];
      uStack_238._4_4_ = uStack_2c0;
      ::utils::geometry::intersect::Triangle::new();
      uStack_26c = *(undefined4 *)((longlong)puVar16 + 0x6c);
      uVar5 = *(undefined8 *)((longlong)puVar16 + 100);
      uStack_260 = *(undefined4 *)((longlong)puVar16 + 0x6c);
      local_1f0[7] = *(undefined8 *)((longlong)puVar16 + 100);
      afStack_240[1] = (float)local_1f0[7];
      uStack_238._0_4_ = (undefined4)((ulonglong)local_1f0[7] >> 0x20);
      uStack_254 = *(undefined4 *)(puVar16 + 0xc);
      uStack_264 = (undefined4)uStack_238;
      uStack_25c = (undefined4)puVar16[0xb];
      uStack_258 = (undefined4)((ulonglong)puVar16[0xb] >> 0x20);
      uStack_274 = (undefined4)uVar5;
      uStack_270 = (undefined4)((ulonglong)uVar5 >> 0x20);
      local_268 = afStack_240[1];
      stack0xfffffffffffffdf8 = CONCAT44(uStack_2b4,uStack_2b8);
      stack0xfffffffffffffde8 = CONCAT44(uStack_2c4,local_2c8);
      stack0xfffffffffffffdf0 = CONCAT44(uStack_2bc,uStack_2c0);
      _local_228 = CONCAT44(uStack_2d4,local_2d8);
      stack0xfffffffffffffde0 = CONCAT44(uStack_2cc,uStack_2d0);
      uStack_238._0_4_ = (undefined4)local_2e8;
      uStack_238._4_4_ = (undefined4)((ulonglong)local_2e8 >> 0x20);
      uStack_230 = uStack_2e0;
      local_248 = (undefined **)CONCAT44(uStack_2f4,local_2f8);
      afStack_240[0] = (float)uStack_2f0;
      afStack_240[1] = (float)((ulonglong)uStack_2f0 >> 0x20);
      local_1f0[9] = CONCAT44(uStack_254,uStack_258);
      local_1f0[8] = CONCAT44(uStack_25c,uStack_260);
      local_1f0[5] = CONCAT44(uStack_274,local_278);
      local_1f0[6] = CONCAT44(uStack_26c,uStack_270);
      local_1f0[3] = local_288;
      local_1f0[4] = uStack_280;
      local_1f0[1] = local_298;
      local_1f0[2] = pcStack_290;
      local_200 = uVar6;
      uStack_1f8 = uVar8;
      uStack_1f4 = uVar9;
      local_1f0[0] = uVar10;
      local_1a0 = uVar8;
      uStack_19c = uVar9;
      uStack_198 = *(undefined4 *)(puVar16 + 3);
      uStack_194 = *(undefined4 *)((longlong)puVar16 + 0x1c);
      uStack_190 = uVar11;
      lVar15 = 0;
      while (lVar7 = local_100, lVar15 != 2) {
        local_188 = lVar15 + 1;
        if (local_100 == local_110) {
          alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
        }
        lVar13 = local_130;
        lVar14 = lVar15 * 0x60;
        *(undefined4 *)(lStack_108 + 0x20 + lVar7 * 0x24) = *(undefined4 *)(local_228 + lVar14);
        uVar5 = (&local_248)[lVar15 * 0xc];
        uVar6 = *(undefined8 *)(afStack_240 + lVar15 * 0x18);
        uVar10 = *(undefined8 *)(local_228 + lVar14 + -8);
        puVar1 = (undefined8 *)(lStack_108 + 0x10 + lVar7 * 0x24);
        *puVar1 = (&uStack_238)[lVar15 * 0xc];
        puVar1[1] = uVar10;
        puVar1 = (undefined8 *)(lStack_108 + lVar7 * 0x24);
        *puVar1 = uVar5;
        puVar1[1] = uVar6;
        local_100 = lVar7 + 1;
        if (local_130 == local_140) {
          alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>();
        }
        lVar7 = local_148;
        *(undefined4 *)(lStack_138 + 0x20 + lVar13 * 0x24) = (&uStack_204)[lVar15 * 0x18];
        uVar5 = *(undefined8 *)(local_228 + lVar14 + 4);
        uVar6 = *(undefined8 *)(auStack_21c + lVar14);
        uVar10 = *(undefined8 *)(auStack_20c + lVar14);
        puVar1 = (undefined8 *)(lStack_138 + 0x10 + lVar13 * 0x24);
        *puVar1 = *(undefined8 *)(auStack_214 + lVar14);
        puVar1[1] = uVar10;
        puVar1 = (undefined8 *)(lStack_138 + lVar13 * 0x24);
        *puVar1 = uVar5;
        puVar1[1] = uVar6;
        local_130 = lVar13 + 1;
        if (local_148 == local_158) {
          alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
        }
        *(undefined8 *)(lStack_150 + 0x10 + lVar7 * 0x18) = local_1f0[lVar15 * 0xc];
        uVar5 = local_1f0[lVar15 * 0xc + -1];
        puVar1 = (undefined8 *)(lStack_150 + lVar7 * 0x18);
        *puVar1 = (&local_200)[lVar15 * 0xc];
        puVar1[1] = uVar5;
        local_148 = lVar7 + 1;
        lVar15 = lVar15 + 1;
      }
    }
    else {
      fVar22 = *(float *)(puVar16 + 10);
      fVar23 = *(float *)((longlong)puVar16 + 0x54);
      fVar24 = fVar22;
      fVar25 = fVar23;
      if ((local_f2 & 1) != 0) {
        if (local_124 < local_128) {
                    // WARNING: Subroutine does not return
          core::f32::impl$0::clamp::do_panic::runtime();
        }
        fVar25 = local_128;
        if (local_128 <= fVar22) {
          fVar25 = fVar22;
        }
        fVar24 = local_124;
        if (fVar25 <= local_124) {
          fVar24 = fVar25;
        }
        local_2f8 = fVar24;
        if ((fVar24 < 0.0) || (1.0 < fVar24)) {
          pcStack_290 = core::fmt::float::impl$5::fmt;
          local_248 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
          afStack_240[0] = 1.4013e-45;
          afStack_240[1] = 0.0;
          _local_228 = 0;
          uStack_238 = &local_298;
          uStack_230 = 1;
          local_298 = &local_2f8;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(fVar24)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
        fVar39 = local_128;
        if (local_128 <= fVar23) {
          fVar39 = fVar23;
        }
        fVar25 = local_124;
        if (fVar39 <= local_124) {
          fVar25 = fVar39;
        }
        local_2f8 = fVar25;
        if ((fVar25 < 0.0) || (1.0 < fVar25)) {
          pcStack_290 = core::fmt::float::impl$5::fmt;
          local_248 = &anon_12ce4fde5425384d5f438f5122902e1a_13_llvm_3277990281340110158;
          afStack_240[0] = 1.4013e-45;
          afStack_240[1] = 0.0;
          _local_228 = 0;
          uStack_238 = &local_298;
          uStack_230 = 1;
          local_298 = &local_2f8;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        if (NAN(fVar25)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
      if ((fVar24 != fVar25) || (NAN(fVar24) || NAN(fVar25))) {
        fVar18 = (float)puVar16[0xb] * fVar17;
        fVar20 = (float)((ulonglong)puVar16[0xb] >> 0x20) * fVar17;
        uVar4 = puVar16[0xe];
        auVar26._8_8_ = 0;
        auVar26._0_8_ = uVar4;
        fVar35 = (float)uVar4;
        fVar37 = (float)(uVar4 >> 0x20);
        auVar12._8_8_ = 0;
        auVar26 = maxps(auVar12,auVar26);
        fVar40 = (float)(~-(uint)NAN(fVar35) & auVar26._0_4_ | uVar2 & -(uint)NAN(fVar35));
        fVar38 = (float)(~-(uint)NAN(fVar37) & auVar26._4_4_ | uVar2 & -(uint)NAN(fVar37));
        fVar33 = fVar40 * 0.0;
        fVar34 = fVar38 * 0.0;
        fVar39 = fVar33 + ((float)puVar16[4] - fVar35 * 0.0) + fVar18;
        fVar40 = fVar40 + ((float)((ulonglong)puVar16[4] >> 0x20) - fVar35) + fVar20;
        fVar42 = (float)((ulonglong)puVar16[7] >> 0x20);
        fVar41 = (float)((ulonglong)*(undefined8 *)((longlong)puVar16 + 0x44) >> 0x20);
        uVar32 = (uint)(0.0 < fVar35 & local_f1);
        uVar31 = (int)(uVar32 << 0x1f) >> 0x1f;
        uVar32 = (int)(uVar32 << 0x1f) >> 0x1f;
        fVar19 = (float)(~uVar31 & uVar3 | (uint)(fVar35 - fVar42) & uVar31);
        fVar29 = (float)(~uVar32 & uVar3 | (uint)(fVar35 - fVar41) & uVar32);
        fVar28 = fVar19 * 0.0;
        fVar30 = fVar29 * 0.0;
        fVar18 = fVar28 + (float)puVar16[7] + fVar18;
        fVar20 = fVar19 + fVar42 + fVar20;
        fVar19 = (float)*(undefined8 *)((longlong)puVar16 + 100) * fVar17;
        fVar21 = (float)((ulonglong)*(undefined8 *)((longlong)puVar16 + 100) >> 0x20) * fVar17;
        fVar27 = *(float *)((longlong)puVar16 + 0x6c) * fVar17;
        fVar36 = fVar34 + ((float)*(undefined8 *)((longlong)puVar16 + 0x2c) - fVar37 * 0.0) + fVar19
        ;
        fVar38 = fVar38 + ((float)((ulonglong)*(undefined8 *)((longlong)puVar16 + 0x2c) >> 0x20) -
                          fVar37) + fVar21;
        fVar42 = (*(float *)(puVar16 + 5) - fVar35 * 0.0) + fVar33 +
                 *(float *)(puVar16 + 0xc) * fVar17;
        fVar33 = (*(float *)((longlong)puVar16 + 0x34) - fVar37 * 0.0) + fVar34 + fVar27;
        fVar19 = fVar30 + (float)*(undefined8 *)((longlong)puVar16 + 0x44) + fVar19;
        fVar21 = fVar29 + fVar41 + fVar21;
        fVar41 = *(float *)(puVar16 + 8) + fVar28 + *(float *)(puVar16 + 0xc) * fVar17;
        fVar27 = *(float *)((longlong)puVar16 + 0x4c) + fVar30 + fVar27;
        uStack_168 = (undefined4)puVar16[0xd];
        uStack_164 = (undefined4)((ulonglong)puVar16[0xd] >> 0x20);
        local_178 = puVar16[0xb];
        uStack_170 = (undefined4)puVar16[0xc];
        uStack_16c = (undefined4)((ulonglong)puVar16[0xc] >> 0x20);
        uVar10 = *puVar16;
        uVar11 = puVar16[1];
        uVar5 = puVar16[2];
        uVar6 = puVar16[3];
        if ((fVar24 != fVar22) ||
           (((NAN(fVar24) || NAN(fVar22) || (fVar25 != fVar23)) || (NAN(fVar25) || NAN(fVar23))))) {
          ::utils::inverse_lerp();
          ::utils::inverse_lerp();
          fVar28 = 1.0 - extraout_XMM0_Da;
          fVar22 = fVar36 * extraout_XMM0_Da;
          fVar23 = fVar38 * extraout_XMM0_Da;
          fVar34 = fVar33 * extraout_XMM0_Da;
          fVar30 = 1.0 - extraout_XMM0_Da_00;
          fVar36 = fVar36 * extraout_XMM0_Da_00 + fVar39 * fVar30;
          fVar38 = fVar38 * extraout_XMM0_Da_00 + fVar40 * fVar30;
          fVar33 = fVar33 * extraout_XMM0_Da_00 + fVar42 * fVar30;
          fVar24 = extraout_XMM0_Da * fVar19;
          fVar25 = extraout_XMM0_Da * fVar21;
          fVar29 = fVar30 * fVar41;
          fVar41 = extraout_XMM0_Da * fVar27 + fVar28 * fVar41;
          fVar19 = fVar19 * extraout_XMM0_Da_00 + fVar18 * fVar30;
          fVar21 = fVar21 * extraout_XMM0_Da_00 + fVar20 * fVar30;
          fVar27 = extraout_XMM0_Da_00 * fVar27 + fVar29;
          fVar39 = fVar22 + fVar39 * fVar28;
          fVar40 = fVar23 + fVar40 * fVar28;
          fVar42 = fVar34 + fVar42 * fVar28;
          fVar18 = fVar24 + fVar28 * fVar18;
          fVar20 = fVar25 + fVar28 * fVar20;
        }
        local_120 = CONCAT44(fVar40,fVar39);
        local_298 = (float *)CONCAT44(fVar38,fVar36);
        pcStack_290 = (code *)CONCAT44(pcStack_290._4_4_,fVar33);
        local_248 = (undefined **)CONCAT44(fVar20,fVar18);
        afStack_240[0] = fVar41;
        local_118 = fVar42;
        ::utils::geometry::intersect::Triangle::new();
        uStack_238._4_4_ = uStack_164;
        afStack_240[1] = (float)uStack_16c;
        uStack_238._0_4_ = uStack_168;
        _local_228 = CONCAT44(auStack_224._0_4_,uStack_170);
        uStack_230 = local_178;
        uStack_2b4 = uStack_170;
        uStack_2c4 = uStack_168;
        uStack_2c0 = uStack_164;
        uStack_2bc = (undefined4)local_178;
        uStack_2b8 = (undefined4)((ulonglong)local_178 >> 0x20);
        uStack_2cc = uStack_170;
        local_2c8 = uStack_16c;
        local_120 = CONCAT44(fVar21,fVar19);
        local_248 = (undefined **)CONCAT44(fVar20,fVar18);
        uStack_2d4 = uStack_2bc;
        uStack_2d0 = uStack_2b8;
        local_2b0 = fVar36;
        fStack_2ac = fVar38;
        local_2a8 = fVar33;
        afStack_240[0] = fVar41;
        local_118 = fVar27;
        ::utils::geometry::intersect::Triangle::new();
        uStack_254 = uStack_170;
        uStack_264 = uStack_168;
        uStack_260 = uStack_164;
        uStack_25c = (undefined4)local_178;
        uStack_258 = (undefined4)((ulonglong)local_178 >> 0x20);
        uStack_274 = uStack_16c;
        uStack_270 = uStack_168;
        uStack_26c = uStack_164;
        local_268 = uStack_16c;
        stack0xfffffffffffffdf8 = CONCAT44(uStack_2b4,uStack_2b8);
        stack0xfffffffffffffde8 = CONCAT44(uStack_2c4,local_2c8);
        stack0xfffffffffffffdf0 = CONCAT44(uStack_2bc,uStack_2c0);
        _local_228 = CONCAT44(uStack_2d4,local_2d8);
        stack0xfffffffffffffde0 = CONCAT44(uStack_2cc,uStack_2d0);
        uStack_238._0_4_ = (undefined4)local_2e8;
        uStack_238._4_4_ = (undefined4)((ulonglong)local_2e8 >> 0x20);
        uStack_230 = uStack_2e0;
        local_248 = (undefined **)CONCAT44(uStack_2f4,local_2f8);
        afStack_240[0] = (float)uStack_2f0;
        afStack_240[1] = (float)((ulonglong)uStack_2f0 >> 0x20);
        local_1f0[9] = CONCAT44(uStack_170,uStack_258);
        local_1f0[7] = CONCAT44(uStack_168,uStack_16c);
        local_1f0[8] = CONCAT44(uStack_25c,uStack_164);
        local_1f0[5] = CONCAT44(uStack_16c,local_278);
        local_1f0[6] = CONCAT44(uStack_164,uStack_168);
        local_1f0[3] = local_288;
        local_1f0[4] = uStack_280;
        local_1f0[1] = local_298;
        local_1f0[2] = pcStack_290;
        uStack_340 = (undefined4)uVar11;
        uStack_33c = (undefined4)((ulonglong)uVar11 >> 0x20);
        local_200 = uVar10;
        uStack_1f8 = uStack_340;
        uStack_1f4 = uStack_33c;
        local_1f0[0] = uVar5;
        local_1a0 = uStack_340;
        uStack_19c = uStack_33c;
        uStack_198 = (int)uVar6;
        uStack_194 = (int)((ulonglong)uVar6 >> 0x20);
        uStack_190 = uVar5;
        lVar15 = 0;
        while (lVar7 = local_100, lVar15 != 2) {
          local_188 = lVar15 + 1;
          if (local_100 == local_110) {
            alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>()
            ;
          }
          lVar13 = local_130;
          lVar14 = lVar15 * 0x60;
          *(undefined4 *)(lStack_108 + 0x20 + lVar7 * 0x24) = *(undefined4 *)(local_228 + lVar14);
          uVar5 = (&local_248)[lVar15 * 0xc];
          uVar6 = *(undefined8 *)(afStack_240 + lVar15 * 0x18);
          uVar10 = *(undefined8 *)(local_228 + lVar14 + -8);
          puVar1 = (undefined8 *)(lStack_108 + 0x10 + lVar7 * 0x24);
          *puVar1 = (&uStack_238)[lVar15 * 0xc];
          puVar1[1] = uVar10;
          puVar1 = (undefined8 *)(lStack_108 + lVar7 * 0x24);
          *puVar1 = uVar5;
          puVar1[1] = uVar6;
          local_100 = lVar7 + 1;
          if (local_130 == local_140) {
            alloc::raw_vec::RawVec::grow_one<array_<glam::f32::vec3::Vec3,3>,alloc::alloc::Global>()
            ;
          }
          lVar7 = local_148;
          *(undefined4 *)(lStack_138 + 0x20 + lVar13 * 0x24) = (&uStack_204)[lVar15 * 0x18];
          uVar5 = *(undefined8 *)(local_228 + lVar14 + 4);
          uVar6 = *(undefined8 *)(auStack_21c + lVar14);
          uVar10 = *(undefined8 *)(auStack_20c + lVar14);
          puVar1 = (undefined8 *)(lStack_138 + 0x10 + lVar13 * 0x24);
          *puVar1 = *(undefined8 *)(auStack_214 + lVar14);
          puVar1[1] = uVar10;
          puVar1 = (undefined8 *)(lStack_138 + lVar13 * 0x24);
          *puVar1 = uVar5;
          puVar1[1] = uVar6;
          local_130 = lVar13 + 1;
          if (local_148 == local_158) {
            alloc::raw_vec::RawVec::grow_one<system_critters::boids::Boid,alloc::alloc::Global>();
          }
          *(undefined8 *)(lStack_150 + 0x10 + lVar7 * 0x18) = local_1f0[lVar15 * 0xc];
          uVar5 = local_1f0[lVar15 * 0xc + -1];
          puVar1 = (undefined8 *)(lStack_150 + lVar7 * 0x18);
          *puVar1 = (&local_200)[lVar15 * 0xc];
          puVar1[1] = uVar5;
          local_148 = lVar7 + 1;
          lVar15 = lVar15 + 1;
        }
      }
    }
    puVar16 = puVar16 + 0xf;
    if (puVar16 == local_250) {
      if (local_100 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (local_130 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (local_148 == 0) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      local_2a0[2] = local_100;
      *local_2a0 = local_110;
      local_2a0[1] = lStack_108;
      local_2a0[3] = local_140;
      local_2a0[4] = lStack_138;
      local_2a0[5] = local_130;
      local_2a0[6] = local_158;
      local_2a0[7] = lStack_150;
      local_2a0[8] = local_148;
      return;
    }
  } while( true );
}


