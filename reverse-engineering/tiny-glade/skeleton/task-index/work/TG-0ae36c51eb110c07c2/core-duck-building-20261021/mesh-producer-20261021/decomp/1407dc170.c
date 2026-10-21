
void __cdecl system_decorator::decorator_visual::stairs_assemble::gen_stair_floors(void)

{
  float *pfVar1;
  undefined **ppuVar2;
  longlong *plVar3;
  undefined **ppuVar4;
  undefined8 uVar5;
  ulonglong uVar6;
  undefined *puVar7;
  int iVar8;
  code *pcVar9;
  int iVar10;
  float **ppfVar11;
  undefined8 uVar12;
  undefined **ppuVar13;
  float *pfVar14;
  bool bVar15;
  undefined4 uVar16;
  uint extraout_EAX;
  char *extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *puVar17;
  undefined **extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  float *pfVar18;
  longlong lVar19;
  longlong in_RDX;
  longlong lVar20;
  byte bVar21;
  float *pfVar22;
  float *pfVar23;
  undefined **ppuVar24;
  longlong in_R8;
  float *in_R9;
  float *pfVar25;
  float *unaff_R14;
  float *pfVar26;
  longlong lVar27;
  bool bVar28;
  bool bVar29;
  uint uVar30;
  float fVar31;
  uint uVar32;
  float fVar33;
  undefined8 uVar34;
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
  float fVar45;
  float fVar46;
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  int *in_stack_00000058;
  float *in_stack_00000060;
  longlong in_stack_00000068;
  float local_3d8;
  float fStack_3d4;
  float local_3c8;
  float fStack_3c4;
  longlong local_398;
  float *local_390;
  undefined *local_388;
  float local_358;
  float fStack_354;
  undefined *puStack_350;
  float *local_348;
  longlong lStack_340;
  undefined *local_338;
  float *pfStack_330;
  undefined8 local_328;
  float *pfStack_320;
  float *local_318;
  undefined8 uStack_310;
  float *local_308;
  code *pcStack_300;
  undefined1 local_2f8 [16];
  longlong local_2e0;
  float *local_2d8;
  longlong local_2d0;
  undefined *local_2c8;
  float **local_2c0;
  float local_2b8;
  float fStack_2b4;
  float fStack_2b0;
  undefined4 uStack_2ac;
  float *local_2a8;
  longlong lStack_2a0;
  undefined *local_298;
  float *pfStack_290;
  undefined8 local_288;
  float *pfStack_280;
  float *local_278;
  undefined8 uStack_270;
  float local_268;
  float local_264;
  float local_260;
  float local_25c;
  longlong local_258;
  longlong local_250;
  undefined *local_248;
  char *pcStack_240;
  undefined *local_238;
  undefined8 uStack_230;
  undefined8 uStack_228;
  float *pfStack_220;
  float *local_218;
  code *pcStack_210;
  undefined8 local_208;
  undefined4 local_200;
  undefined4 local_1fc;
  undefined **local_1f8;
  undefined8 local_1f0;
  float **ppfStack_1e8;
  undefined8 local_1e0;
  undefined8 uStack_1d8;
  float *local_1c0;
  undefined8 local_1b8;
  undefined4 uStack_1b0;
  undefined4 uStack_1ac;
  ulonglong local_1a0;
  longlong local_198;
  undefined **local_190;
  longlong local_188;
  char *local_180;
  undefined8 local_178;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  longlong local_160;
  longlong local_158;
  longlong local_150;
  float local_148;
  uint local_144;
  float *local_140;
  undefined **local_138;
  longlong local_130;
  undefined **local_128;
  byte local_11a;
  undefined1 local_119;
  undefined **local_118;
  longlong local_110;
  undefined **local_108;
  longlong local_100;
  undefined1 local_f4;
  undefined1 local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_264 = 0.0;
  local_260 = 0.0;
  if ((*in_stack_00000058 == 1) &&
     (local_260 = 0.15 - (float)in_stack_00000058[1], local_260 <= 0.0)) {
    local_260 = 0.0;
  }
  if ((in_stack_00000058[2] == 1) &&
     (local_264 = 0.15 - (float)in_stack_00000058[3], local_264 <= 0.0)) {
    local_264 = 0.0;
  }
  local_268 = 1.0 / (float)(in_stack_00000038 - 1);
  pcStack_240 = (char *)(in_stack_00000030 + in_stack_00000038 * 0xc);
  local_238 = (undefined *)0x0;
  uStack_230 = &local_260;
  uStack_228 = &local_264;
  pfStack_220 = &local_268;
  local_2d8 = in_R9;
  local_2d0 = in_R8;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec3::Vec3,core::iter::adapters::map::Map<core::iter::adapters::enumerate::Enumerate<core::slice::iter::Iter<glam::f32::vec3::Vec3>_>,system_decorator::decorator_visual::stairs_assemble::gen_stair_floors::closure_env_2>_>
            ();
  local_178 = local_388;
  if (local_388 == (undefined *)0x0) {
    local_248 = (undefined *)0x0;
    pcStack_240 = (char *)0x4;
  }
  else {
    local_1b8 = (undefined **)((longlong)local_388 * 4);
    local_188 = (longlong)local_388 * 0xc;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (char *)0x0) {
      local_f1 = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    local_248 = local_388;
    local_238 = (undefined *)0x0;
    pcStack_240 = extraout_RAX;
    if (local_388 != (undefined *)0x1) {
      fVar35 = local_390[4];
      fVar40 = local_390[1];
      local_1c0 = local_390;
      fVar41 = local_390[5] - local_390[2];
      fVar43 = local_390[3] - *local_390;
      local_358 = -fVar41;
      fStack_354 = fVar43;
      ::utils::swizzlers::impl$0::x0y();
      puVar7 = local_238;
      fVar36 = fStack_2b0 * (fVar35 - fVar40) - fVar41 * fStack_2b4;
      fVar41 = local_2b8 * fVar41 - fVar43 * fStack_2b0;
      fVar35 = fVar43 * fStack_2b4 - (fVar35 - fVar40) * local_2b8;
      fVar40 = 1.0 / SQRT(fVar35 * fVar35 + fVar41 * fVar41 + fVar36 * fVar36);
      bVar15 = (int)fVar40 - 1U < 0x7fffff ||
               (int)ABS(fVar40) - 0x800000U < 0x7f000000 && -1 < (int)fVar40;
      if (bVar15) {
        fVar35 = -fVar35 * fVar40;
      }
      else {
        fVar35 = -0.0;
      }
      if (local_238 == local_248) {
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      uVar32 = (uint)bVar15;
      uVar30 = (int)(uVar32 << 0x1f) >> 0x1f;
      uVar32 = (int)(uVar32 << 0x1f) >> 0x1f;
      local_2c0 = &pfStack_290;
      pfVar18 = (float *)((longlong)puVar7 * 3);
      *(ulonglong *)(pcStack_240 + (longlong)puVar7 * 0xc) =
           CONCAT44(~uVar32,~uVar30) & 0x8000000080000000 |
           CONCAT44((uint)(fVar40 * -fVar41) & uVar32,(uint)(fVar40 * -fVar36) & uVar30);
      *(float *)(pcStack_240 + (longlong)puVar7 * 0xc + 8) = fVar35;
      local_140 = local_1c0 + (longlong)local_178 * 3;
      bVar15 = true;
      pfVar25 = &local_358;
      pfVar22 = local_1c0;
      while (local_238 = puVar7 + 1, pfVar22 != local_140) {
        pfVar1 = pfVar22 + 3;
        pfVar14 = pfVar22;
        pfVar23 = pfVar1;
        pfVar26 = pfVar18;
        if (bVar15) {
          if ((pfVar1 == local_140) || (pfVar22 + 6 == local_140)) break;
          unaff_R14 = pfVar22;
          pfVar14 = pfVar22 + 6;
          pfVar23 = pfVar22 + 9;
          pfVar26 = pfVar1;
        }
        pfVar18 = pfVar14;
        fVar35 = pfVar18[1];
        fVar40 = unaff_R14[1];
        fVar41 = pfVar18[2] - unaff_R14[2];
        fVar43 = *pfVar18 - *unaff_R14;
        local_358 = -fVar41;
        fStack_354 = fVar43;
        ::utils::swizzlers::impl$0::x0y();
        puVar7 = local_238;
        fVar36 = fStack_2b0 * (fVar35 - fVar40) - fVar41 * fStack_2b4;
        fVar41 = local_2b8 * fVar41 - fVar43 * fStack_2b0;
        fVar40 = fVar43 * fStack_2b4 - (fVar35 - fVar40) * local_2b8;
        fVar43 = 1.0 / SQRT(fVar40 * fVar40 + fVar41 * fVar41 + fVar36 * fVar36);
        bVar28 = (int)ABS(fVar43) - 0x800000U < 0x7f000000;
        bVar29 = (int)fVar43 - 1U < 0x7fffff;
        bVar15 = bVar29 || bVar28 && -1 < (int)fVar43;
        fVar35 = -0.0;
        if (bVar15) {
          fVar35 = -fVar40 * fVar43;
        }
        if (local_238 == local_248) {
          alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
        }
        uVar32 = (int)((uint)(bVar29 || bVar28 && -1 < (int)fVar43) << 0x1f) >> 0x1f;
        uVar30 = (int)((uint)bVar15 << 0x1f) >> 0x1f;
        *(ulonglong *)(pcStack_240 + (longlong)puVar7 * 0xc) =
             CONCAT44(~uVar30,~uVar32) & 0x8000000080000000 |
             CONCAT44((uint)(fVar43 * -fVar41) & uVar30,(uint)(fVar43 * -fVar36) & uVar32);
        *(float *)(pcStack_240 + (longlong)puVar7 * 0xc + 8) = fVar35;
        bVar15 = false;
        pfVar22 = pfVar23;
        unaff_R14 = pfVar26;
      }
      fVar35 = local_140[-5];
      fVar40 = local_140[-2];
      fVar41 = local_140[-4] - local_140[-1];
      fVar43 = local_140[-6] - local_140[-3];
      local_358 = -fVar41;
      fStack_354 = fVar43;
      ::utils::swizzlers::impl$0::x0y();
      puVar7 = local_238;
      fVar36 = fStack_2b0 * (fVar35 - fVar40) - fVar41 * fStack_2b4;
      fVar41 = local_2b8 * fVar41 - fVar43 * fStack_2b0;
      fVar35 = fVar43 * fStack_2b4 - (fVar35 - fVar40) * local_2b8;
      fVar40 = 1.0 / SQRT(fVar35 * fVar35 + fVar41 * fVar41 + fVar36 * fVar36);
      bVar15 = (int)fVar40 - 1U < 0x7fffff ||
               (int)ABS(fVar40) - 0x800000U < 0x7f000000 && -1 < (int)fVar40;
      if (bVar15) {
        fVar35 = -fVar35 * fVar40;
      }
      else {
        fVar35 = -0.0;
      }
      if (local_238 == local_248) {
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      uVar32 = (uint)bVar15;
      uVar30 = (int)(uVar32 << 0x1f) >> 0x1f;
      uVar32 = (int)(uVar32 << 0x1f) >> 0x1f;
      *(ulonglong *)(pcStack_240 + (longlong)puVar7 * 0xc) =
           CONCAT44(~uVar32,~uVar30) & 0x8000000080000000 |
           CONCAT44((uint)(fVar40 * -fVar41) & uVar32,(uint)(fVar40 * -fVar36) & uVar30);
      *(float *)(pcStack_240 + (longlong)puVar7 * 0xc + 8) = fVar35;
      local_180 = pcStack_240;
      local_2c8 = local_248;
      if (((undefined *)0x555555555555555 < local_178) ||
         (0x7ffffffffffffffc < (ulonglong)((longlong)local_178 * 0x30))) {
LAB_1407ddc4c:
        local_f2 = 1;
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      if ((longlong)local_178 * 0x30 == 0) {
        lVar20 = 4;
        lVar19 = 0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        lVar19 = local_188;
        lVar20 = extraout_RAX_00;
        if (extraout_RAX_00 == 0) goto LAB_1407ddc4c;
      }
      local_150 = 0;
      local_160 = lVar19;
      local_158 = lVar20;
      if (local_178 < (undefined *)0x2aaaaaaaaaaaaab) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_138 = local_1b8;
        if (extraout_RAX_01 != 0) {
          local_118 = local_1b8;
          local_108 = (undefined **)0x0;
          local_110 = extraout_RAX_01;
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX_02 == 0) {
            local_f4 = 1;
                    // WARNING: Subroutine does not return
            alloc::raw_vec::handle_error();
          }
          local_128 = (undefined **)0x0;
          local_119 = in_stack_00000068 == 0;
          local_198 = lVar20;
          if ((bool)local_119) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          local_250 = local_398;
          lVar19 = in_stack_00000028 << 3;
          local_2d0 = local_2d0 << 2;
          uVar34 = 0;
          lVar27 = 0;
          local_2f8 = ZEXT816(0);
          lVar20 = 0;
          local_100 = 0;
          local_144 = 0;
          bVar21 = 0;
          local_119 = 0;
          fVar35 = 0.0;
          fVar40 = 0.0;
          fVar36 = 0.0;
          fVar41 = 0.15 - *in_stack_00000060;
          fVar43 = 0.0;
          fVar44 = 0.0;
          fVar46 = 0.0;
          local_130 = extraout_RAX_02;
          do {
            if ((((local_2d0 == lVar20) || (lVar19 == 0)) ||
                ((longlong)puVar7 * 0xc + 0xc == lVar27)) || (in_stack_00000068 << 2 == lVar20))
            break;
            puVar17 = (undefined8 *)((longlong)local_1c0 + lVar27);
            fVar37 = (float)*puVar17;
            fVar39 = (float)((ulonglong)*puVar17 >> 0x20);
            fVar45 = *(float *)(puVar17 + 1);
            fVar42 = 0.15 - *(float *)((longlong)in_stack_00000060 + lVar20);
            fStack_2b4 = *local_2d8;
            local_2b8 = -local_2d8[1];
            local_258 = lVar19;
            ::utils::swizzlers::impl$0::x0y();
            ppuVar24 = local_108;
            fVar38 = *(float *)(in_RDX + lVar20);
            fVar31 = fVar38 * SUB84(local_248,0) * 0.5;
            fVar33 = fVar38 * (float)((ulonglong)local_248 >> 0x20) * 0.5;
            local_148 = pcStack_240._0_4_ * fVar38 * 0.5;
            uStack_170 = 0;
            uStack_16c = 0;
            local_178 = (undefined *)CONCAT44(fVar39 - fVar33,fVar37 - fVar31);
            local_188 = CONCAT44(local_188._4_4_,fVar45 - local_148);
            uStack_1b0 = 0;
            uStack_1ac = 0;
            local_1b8 = (undefined **)CONCAT44(fVar33 + fVar39,fVar31 + fVar37);
            local_148 = local_148 + fVar45;
            if (0.0 < fVar41 != 0.0 < fVar42) {
              fVar41 = fVar41 / (fVar41 - fVar42);
              uVar5 = *(undefined8 *)(local_180 + lVar27);
              local_25c = *(float *)(local_180 + lVar27 + 8) * fVar41;
              iVar8 = (int)local_108;
              if ((bVar21 & 1) != 0) {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_198 = local_158;
                  local_100 = local_150;
                }
                *(int *)(local_198 + local_100 * 4) = (int)local_1a0;
                uVar16 = (undefined4)(local_1a0 >> 0x20);
                *(undefined4 *)(local_198 + 4 + local_100 * 4) = uVar16;
                *(int *)(local_198 + 8 + local_100 * 4) = iVar8;
                *(int *)(local_198 + 0xc + local_100 * 4) = iVar8;
                *(undefined4 *)(local_198 + 0x10 + local_100 * 4) = uVar16;
                *(uint *)(local_198 + 0x14 + local_100 * 4) = iVar8 + 1U;
                local_150 = local_100 + 6;
                local_100 = local_150;
              }
              ppuVar2 = local_108;
              if (0.0 < fVar42) {
                local_1a0 = (ulonglong)ppuVar24 & 0xffffffff | (ulonglong)(iVar8 + 1U) << 0x20;
              }
              local_11a = 0.0 < fVar42;
              if (local_108 == local_118) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              fVar45 = 1.0 - fVar41;
              fVar40 = fVar41 * (float)local_178 + fVar40 * fVar45;
              fVar36 = fVar41 * local_178._4_4_ + fVar36 * fVar45;
              fVar35 = fVar41 * (float)local_188 + fVar35 * fVar45;
              *(ulonglong *)(local_110 + (longlong)ppuVar2 * 0xc) = CONCAT44(fVar36,fVar40);
              *(float *)(local_110 + 8 + (longlong)ppuVar2 * 0xc) = fVar35;
              ppuVar24 = (undefined **)((longlong)ppuVar2 + 1);
              if (ppuVar24 == local_118) {
                local_108 = ppuVar24;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar4 = local_128;
              fVar43 = fVar41 * (float)local_1b8 + fVar43 * fVar45;
              fVar44 = fVar41 * local_1b8._4_4_ + fVar44 * fVar45;
              fVar46 = fVar41 * local_148 + fVar46 * fVar45;
              *(ulonglong *)(local_110 + (longlong)ppuVar24 * 0xc) = CONCAT44(fVar44,fVar43);
              *(float *)(local_110 + 8 + (longlong)ppuVar24 * 0xc) = fVar46;
              local_108 = (undefined **)((longlong)ppuVar2 + 2);
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              local_3d8 = (float)uVar34;
              fStack_3d4 = (float)((ulonglong)uVar34 >> 0x20);
              local_3c8 = (float)uVar5;
              fStack_3c4 = (float)((ulonglong)uVar5 >> 0x20);
              fVar38 = fVar41 * local_3c8 + local_3d8 * fVar45;
              fVar31 = fVar41 * fStack_3c4 + fStack_3d4 * fVar45;
              fVar41 = (float)local_2f8._0_4_ * fVar45 + local_25c;
              *(ulonglong *)(local_130 + (longlong)ppuVar4 * 0xc) = CONCAT44(fVar31,fVar38);
              *(float *)(local_130 + 8 + (longlong)ppuVar4 * 0xc) = fVar41;
              ppuVar24 = (undefined **)((longlong)ppuVar4 + 1);
              if (ppuVar24 == local_138) {
                local_128 = ppuVar24;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar2 = local_108;
              *(ulonglong *)(local_130 + (longlong)ppuVar24 * 0xc) = CONCAT44(fVar31,fVar38);
              *(float *)(local_130 + 8 + (longlong)ppuVar24 * 0xc) = fVar41;
              local_128 = (undefined **)((longlong)ppuVar4 + 2);
              iVar8 = (int)local_108;
              if ((local_144 & 1) != 0) {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_100 = local_150;
                }
                *(int *)(local_158 + local_100 * 4) = (int)pfVar25;
                uVar16 = (undefined4)((ulonglong)pfVar25 >> 0x20);
                *(undefined4 *)(local_158 + 4 + local_100 * 4) = uVar16;
                *(int *)(local_158 + 8 + local_100 * 4) = iVar8;
                *(int *)(local_158 + 0xc + local_100 * 4) = iVar8;
                *(undefined4 *)(local_158 + 0x10 + local_100 * 4) = uVar16;
                local_198 = local_158;
                *(uint *)(local_158 + 0x14 + local_100 * 4) = iVar8 + 1U;
                local_150 = local_100 + 6;
                local_100 = local_150;
              }
              ppuVar24 = local_108;
              if (0.0 < fVar42) {
                pfVar25 = (float *)((ulonglong)ppuVar2 & 0xffffffff |
                                   (ulonglong)(iVar8 + 1U) << 0x20);
              }
              if (local_108 == local_118) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(float *)(local_110 + (longlong)ppuVar24 * 0xc) = fVar43;
              *(float *)(local_110 + 4 + (longlong)ppuVar24 * 0xc) = fVar44 + -0.2;
              *(float *)(local_110 + 8 + (longlong)ppuVar24 * 0xc) = fVar46;
              ppuVar2 = (undefined **)((longlong)ppuVar24 + 1);
              if (ppuVar2 == local_118) {
                local_108 = ppuVar2;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar4 = local_128;
              *(float *)(local_110 + (longlong)ppuVar2 * 0xc) = fVar40;
              *(float *)(local_110 + 4 + (longlong)ppuVar2 * 0xc) = fVar36 + -0.2;
              *(float *)(local_110 + 8 + (longlong)ppuVar2 * 0xc) = fVar35;
              local_108 = (undefined **)((longlong)ppuVar24 + 2);
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(ulonglong *)(local_130 + (longlong)ppuVar4 * 0xc) =
                   CONCAT44(fVar31,fVar38) ^ 0x8000000080000000;
              *(float *)(local_130 + 8 + (longlong)ppuVar4 * 0xc) = -fVar41;
              ppuVar24 = (undefined **)((longlong)ppuVar4 + 1);
              if (ppuVar24 == local_138) {
                local_128 = ppuVar24;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(ulonglong *)(local_130 + (longlong)ppuVar24 * 0xc) =
                   CONCAT44(fVar31,fVar38) ^ 0x8000000080000000;
              *(float *)(local_130 + 8 + (longlong)ppuVar24 * 0xc) = -fVar41;
              local_128 = (undefined **)((longlong)ppuVar4 + 2);
              local_144 = (uint)local_11a;
              bVar21 = local_11a;
            }
            if (0.0 < fVar42) {
              iVar8 = (int)local_108;
              local_2f8._0_8_ = local_108;
              if ((bVar21 & 1) == 0) {
                ppuVar24 = local_108;
                if (local_108 == local_118) goto LAB_1407dd1bf;
              }
              else {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_100 = local_150;
                }
                *(int *)(local_158 + local_100 * 4) = (int)local_1a0;
                uVar16 = (undefined4)(local_1a0 >> 0x20);
                *(undefined4 *)(local_158 + 4 + local_100 * 4) = uVar16;
                *(int *)(local_158 + 8 + local_100 * 4) = iVar8;
                *(int *)(local_158 + 0xc + local_100 * 4) = iVar8;
                *(undefined4 *)(local_158 + 0x10 + local_100 * 4) = uVar16;
                local_198 = local_158;
                *(int *)(local_158 + 0x14 + local_100 * 4) = iVar8 + 1;
                local_150 = local_100 + 6;
                ppuVar24 = local_108;
                local_100 = local_150;
                if (local_108 == local_118) {
LAB_1407dd1bf:
                  ppuVar24 = local_108;
                  alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                }
              }
              *(undefined **)(local_110 + (longlong)ppuVar24 * 0xc) = local_178;
              *(float *)(local_110 + 8 + (longlong)ppuVar24 * 0xc) = (float)local_188;
              ppuVar2 = (undefined **)((longlong)ppuVar24 + 1);
              if (ppuVar2 == local_118) {
                local_108 = ppuVar2;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar4 = local_128;
              plVar3 = (longlong *)(local_180 + lVar27);
              *(undefined ***)(local_110 + (longlong)ppuVar2 * 0xc) = local_1b8;
              *(float *)(local_110 + 8 + (longlong)ppuVar2 * 0xc) = local_148;
              local_108 = (undefined **)((longlong)ppuVar24 + 2);
              local_248 = (undefined *)*plVar3;
              pcStack_240._0_4_ = (float)(int)plVar3[1];
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(float *)(local_130 + 8 + (longlong)ppuVar4 * 0xc) = pcStack_240._0_4_;
              *(undefined **)(local_130 + (longlong)ppuVar4 * 0xc) = local_248;
              ppuVar24 = (undefined **)((longlong)ppuVar4 + 1);
              pcStack_240 = (char *)CONCAT44(pcStack_240._4_4_,(int)plVar3[1]);
              local_248 = (undefined *)*plVar3;
              if (ppuVar24 == local_138) {
                local_128 = ppuVar24;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar2 = local_108;
              *(float *)(local_130 + 8 + (longlong)ppuVar24 * 0xc) = pcStack_240._0_4_;
              *(undefined **)(local_130 + (longlong)ppuVar24 * 0xc) = local_248;
              local_128 = (undefined **)((longlong)ppuVar4 + 2);
              iVar10 = (int)local_108;
              if ((local_144 & 1) == 0) {
                fVar35 = local_1b8._4_4_ + -0.2;
                ppuVar24 = local_108;
                if (local_108 == local_118) goto LAB_1407dd3dc;
              }
              else {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_100 = local_150;
                }
                *(int *)(local_158 + local_100 * 4) = (int)pfVar25;
                uVar16 = (undefined4)((ulonglong)pfVar25 >> 0x20);
                *(undefined4 *)(local_158 + 4 + local_100 * 4) = uVar16;
                *(int *)(local_158 + 8 + local_100 * 4) = iVar10;
                *(int *)(local_158 + 0xc + local_100 * 4) = iVar10;
                *(undefined4 *)(local_158 + 0x10 + local_100 * 4) = uVar16;
                local_198 = local_158;
                *(uint *)(local_158 + 0x14 + local_100 * 4) = iVar10 + 1U;
                local_150 = local_100 + 6;
                fVar35 = local_1b8._4_4_ + -0.2;
                ppuVar24 = local_108;
                local_100 = local_150;
                if (local_108 == local_118) {
LAB_1407dd3dc:
                  ppuVar24 = local_108;
                  alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                }
              }
              *(float *)(local_110 + (longlong)ppuVar24 * 0xc) = (float)local_1b8;
              *(float *)(local_110 + 4 + (longlong)ppuVar24 * 0xc) = fVar35;
              *(float *)(local_110 + 8 + (longlong)ppuVar24 * 0xc) = local_148;
              ppuVar4 = (undefined **)((longlong)ppuVar24 + 1);
              fVar35 = local_178._4_4_ + -0.2;
              if (ppuVar4 == local_118) {
                local_108 = ppuVar4;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar13 = local_128;
              *(float *)(local_110 + (longlong)ppuVar4 * 0xc) = (float)local_178;
              *(float *)(local_110 + 4 + (longlong)ppuVar4 * 0xc) = fVar35;
              *(float *)(local_110 + 8 + (longlong)ppuVar4 * 0xc) = (float)local_188;
              local_108 = (undefined **)((longlong)ppuVar24 + 2);
              uVar6 = *(ulonglong *)(local_180 + lVar27);
              uVar32 = *(uint *)(local_180 + lVar27 + 8);
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(ulonglong *)(local_130 + (longlong)ppuVar13 * 0xc) = uVar6 ^ 0x8000000080000000;
              *(uint *)(local_130 + 8 + (longlong)ppuVar13 * 0xc) = uVar32 ^ 0x80000000;
              ppuVar24 = (undefined **)((longlong)ppuVar13 + 1);
              uVar6 = *(ulonglong *)(local_180 + lVar27);
              uVar32 = *(uint *)(local_180 + lVar27 + 8);
              if (ppuVar24 == local_138) {
                local_128 = ppuVar24;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              local_1a0 = CONCAT44(iVar8 + 1,local_2f8._0_4_);
              pfVar25 = (float *)((ulonglong)ppuVar2 & 0xffffffff | (ulonglong)(iVar10 + 1U) << 0x20
                                 );
              *(ulonglong *)(local_130 + (longlong)ppuVar24 * 0xc) = uVar6 ^ 0x8000000080000000;
              *(uint *)(local_130 + 8 + (longlong)ppuVar24 * 0xc) = uVar32 ^ 0x80000000;
              local_128 = (undefined **)((longlong)ppuVar13 + 2);
              bVar21 = 1;
              local_144 = 1;
            }
            local_2d8 = local_2d8 + 2;
            uVar34 = *(undefined8 *)(local_180 + lVar27);
            local_2f8 = ZEXT416(*(uint *)(local_180 + lVar27 + 8));
            lVar20 = lVar20 + 4;
            lVar27 = lVar27 + 0xc;
            lVar19 = local_258 + -8;
            fVar35 = (float)local_188;
            fVar40 = (float)local_178;
            fVar36 = local_178._4_4_;
            fVar41 = fVar42;
            fVar43 = (float)local_1b8;
            fVar44 = local_1b8._4_4_;
            fVar46 = local_148;
          } while ((float *)((longlong)puVar17 + 0xc) != local_140);
          if (local_250 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          country_core::render::nani::trimesh::Trimesh::builder();
          country_core::render::nani::trimesh::TrimeshBuilder::positions();
          country_core::render::nani::trimesh::TrimeshBuilder::attrib<glam::f32::vec3::Vec3>();
          country_core::render::nani::trimesh::TrimeshBuilder::build();
          if (local_358 == 1.54143e-44) {
            local_178 = puStack_350;
            local_140 = local_348;
            local_218 = local_318;
            uStack_228._0_4_ = (undefined4)local_328;
            uStack_228._4_4_ = (undefined4)((ulonglong)local_328 >> 0x20);
            pfStack_220 = pfStack_320;
            local_238 = local_338;
            uStack_230._0_4_ = SUB84(pfStack_330,0);
            uStack_230._4_4_ = (undefined4)((ulonglong)pfStack_330 >> 0x20);
            local_248 = (undefined *)0x1;
            pcStack_240 = (char *)0x1;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_03 == (undefined **)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            extraout_RAX_03[6] = (undefined *)local_218;
            extraout_RAX_03[4] = (undefined *)CONCAT44(uStack_228._4_4_,(undefined4)uStack_228);
            extraout_RAX_03[5] = (undefined *)pfStack_220;
            extraout_RAX_03[2] = local_238;
            extraout_RAX_03[3] = (undefined *)CONCAT44(uStack_230._4_4_,(undefined4)uStack_230);
            *extraout_RAX_03 = local_248;
            extraout_RAX_03[1] = pcStack_240;
            local_1b8 = extraout_RAX_03;
            local_190 = extraout_RAX_03;
            if (lStack_340 != 0) {
              plVar3 = *(longlong **)(in_stack_00000040 + 0xb0);
              lVar19 = 0;
              do {
                uVar34 = *(undefined8 *)((longlong)local_140 + lVar19);
                ppfVar11 = (float **)((undefined8 *)((longlong)local_140 + lVar19))[1];
                puVar17 = (undefined8 *)((longlong)local_140 + lVar19 + 0x10);
                uVar5 = *puVar17;
                uVar12 = puVar17[1];
                country_core::render::nani::trimesh::TrimeshChunk::index_count();
                uVar32 = extraout_EAX / 3 - 1;
                if (extraout_EAX / 3 == 0) {
                  uVar32 = 0;
                }
                if ((uVar32 != 0) && (iVar8 = 0x1f, uVar32 != 0)) {
                  for (; uVar32 >> iVar8 == 0; iVar8 = iVar8 + -1) {
                  }
                }
                country_core::resources::walls::decorator::DecoratorRank::new();
                country_core::components::drawable::impl$1::from();
                local_308 = (float *)CONCAT44(fStack_2b4,local_2b8);
                pcStack_300 = (code *)CONCAT44(uStack_2ac,fStack_2b0);
                bevy_app::plugin::Plugin::
                finish<country_core::systems::main_camera::MainCameraPlugin>();
                local_2b8 = SUB84(local_308,0);
                fStack_2b4 = (float)((ulonglong)local_308 >> 0x20);
                fStack_2b0 = SUB84(pcStack_300,0);
                uStack_2ac = (undefined4)((ulonglong)pcStack_300 >> 0x20);
                LOCK();
                puVar7 = *local_190;
                *local_190 = *local_190 + 1;
                UNLOCK();
                if (*local_190 == (undefined *)0x0 ||
                    SCARRY8((longlong)puVar7,1) != (longlong)*local_190 < 0) {
                    // WARNING: Does not return
                  pcVar9 = (code *)invalidInstructionException();
                  (*pcVar9)();
                }
                local_248 = (undefined *)0x0;
                pcStack_240 = (char *)0x3f80000000000000;
                uStack_230._0_4_ = 0;
                local_238 = (undefined *)0x0;
                uStack_230._4_4_ = 0x3f800000;
                uStack_228._0_4_ = 0x3f800000;
                uStack_228._4_4_ = 0x3f800000;
                local_218 = local_308;
                pcStack_210 = pcStack_300;
                local_208 = CONCAT71(local_208._1_7_,0x1a);
                local_1f8 = local_190;
                local_1f0 = uVar34;
                ppfStack_1e8 = ppfVar11;
                local_1e0 = uVar5;
                uStack_1d8 = uVar12;
                bevy_ecs::system::commands::Commands::
                spawn<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StairsFloorInstanceData>,country_core::resources::stairs::visual::StairsFloorMeshChunk>_>
                          ();
                puVar7 = local_298;
                lVar20 = plVar3[2];
                if (lVar20 == *plVar3) {
                  alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
                }
                *(undefined **)(plVar3[1] + lVar20 * 8) = puVar7;
                plVar3[2] = lVar20 + 1;
                lVar19 = lVar19 + 0x20;
              } while (lStack_340 << 5 != lVar19);
            }
            if (local_178 != (undefined *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            LOCK();
            *local_190 = *local_190 + -1;
            UNLOCK();
            if (*local_190 == (undefined *)0x0) {
              alloc::sync::Arc::drop_slow<rhapsody::raw::buffer::Buffer,alloc::alloc::Global>();
            }
          }
          else {
            local_278 = local_318;
            uStack_270 = uStack_310;
            local_288 = local_328;
            pfStack_280 = pfStack_320;
            local_298 = local_338;
            pfStack_290 = pfStack_330;
            local_2a8 = local_348;
            lStack_2a0 = lStack_340;
            local_2b8 = local_358;
            fStack_2b4 = fStack_354;
            fStack_2b0 = SUB84(puStack_350,0);
            uStack_2ac = (undefined4)((ulonglong)puStack_350 >> 0x20);
            if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
              pcStack_300 = rhapsody::raw::error::impl$1::fmt;
              local_308 = &local_2b8;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              pfStack_220 = (float *)extraout_RAX_04[1];
              local_1fc = *(undefined4 *)(extraout_RAX_04 + 2);
              local_218 = (float *)0x1;
              pcStack_210 = (code *)0x142a88570;
              local_208 = 0x33;
              local_1f8 = &PTR_s_Error_spawning_stair_mesh__142a88548;
              local_1f0 = 1;
              ppfStack_1e8 = &local_308;
              local_1e0 = 1;
              uStack_1d8 = 0;
              local_248 = (undefined *)0x0;
              pcStack_240 = "system_decorator::decorator_visual::stairs_assemble";
              local_238 = (undefined *)0x33;
              uStack_230._0_4_ = 0;
              uStack_230._4_4_ = 0;
              uStack_228._0_4_ = (undefined4)*extraout_RAX_04;
              uStack_228._4_4_ = (undefined4)((ulonglong)*extraout_RAX_04 >> 0x20);
              local_200 = 1;
              ::log::__private_api::impl$2::log();
            }
            uVar6 = CONCAT44(fStack_2b4,local_2b8);
            lVar19 = 0;
            if (uVar6 - 9 < 2) {
              lVar19 = uVar6 - 8;
            }
            if (lVar19 == 0) {
              puVar7 = local_298;
              if (((uVar6 < 6) && ((0x32UL >> ((ulonglong)(uint)local_2b8 & 0x3f) & 1) != 0)) &&
                 (CONCAT44(uStack_2ac,fStack_2b0) != 0)) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                puVar7 = local_298;
              }
            }
            else {
              if (lVar19 == 1) {
                core::ptr::drop_in_place<backtrace::capture::Backtrace>();
                goto LAB_1407ddb33;
              }
              local_2c0 = &local_2a8;
              puVar7 = (undefined *)CONCAT44(uStack_2ac,fStack_2b0);
            }
            if (puVar7 != (undefined *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
LAB_1407ddb33:
          puVar7 = local_2c8;
          if (local_138 != (undefined **)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_118 != (undefined **)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_160 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (puVar7 != (undefined *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          return;
        }
      }
      local_f3 = 1;
      local_2e0 = lVar19;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
  }
  local_238 = (undefined *)0x0;
                    // WARNING: Subroutine does not return
  core::panicking::panic_bounds_check();
}

