
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs_assemble::gen_stair_floors
          (void *raw_device,void *raw_tangents_ptr,void *raw_tangents_len,void *raw_positions_ptr,
          void *raw_positions_len,void *raw_arg6,void *raw_arg7,void *raw_reset_output,
          void *raw_commands,void *raw_info28,void *raw_roof_options,void *raw_widths_ptr,
          void *raw_widths_len)

{
  float *pfVar1;
  undefined **ppuVar2;
  longlong *plVar3;
  undefined **ppuVar4;
  undefined8 uVar5;
  undefined *puVar6;
  int iVar7;
  code *pcVar8;
  int iVar9;
  float **ppfVar10;
  undefined8 uVar11;
  ulonglong uVar12;
  undefined *puVar13;
  char *pcVar14;
  undefined **ppuVar15;
  float *pfVar16;
  bool bVar17;
  undefined4 uVar18;
  uint extraout_EAX;
  char *extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *puVar19;
  undefined **extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  float *pfVar20;
  longlong lVar21;
  longlong lVar22;
  byte bVar23;
  float *pfVar24;
  float *pfVar25;
  undefined **ppuVar26;
  float *pfVar27;
  float *unaff_R14;
  float *pfVar28;
  longlong lVar29;
  bool bVar30;
  bool bVar31;
  uint uVar32;
  float fVar33;
  uint uVar34;
  float fVar35;
  uint uVar36;
  uint uVar38;
  undefined8 uVar37;
  float fVar39;
  float fVar40;
  float fVar41;
  float fVar42;
  uint uVar43;
  float fVar44;
  uint uVar45;
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float fVar51;
  float fVar52;
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
  void *local_2d0;
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
  undefined4 local_1d0;
  float *local_1c0;
  undefined8 local_1b8;
  float fStack_1b0;
  float fStack_1ac;
  ulonglong local_1a0;
  longlong local_198;
  undefined **local_190;
  longlong local_188;
  char *local_180;
  undefined8 local_178;
  float fStack_170;
  float fStack_16c;
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
  
  fVar46 = __real_3f800000;
  local_f0 = 0xfffffffffffffffe;
  local_264 = 0.0;
  local_260 = 0.0;
  if ((*(int *)raw_roof_options == 1) &&
     (local_260 = __real_3e19999a - *(float *)((longlong)raw_roof_options + 4), local_260 <= 0.0)) {
    local_260 = 0.0;
  }
  if ((*(int *)((longlong)raw_roof_options + 8) == 1) &&
     (local_264 = __real_3e19999a - *(float *)((longlong)raw_roof_options + 0xc), local_264 <= 0.0))
  {
    local_264 = 0.0;
  }
  local_268 = __real_3f800000 / (float)((longlong)raw_arg7 - 1);
  pcStack_240 = (char *)((longlong)raw_arg6 + (longlong)raw_arg7 * 0xc);
  local_248 = raw_arg6;
  local_238 = (undefined *)0x0;
  uStack_230 = &local_260;
  uStack_228 = &local_264;
  pfStack_220 = &local_268;
  local_2d8 = raw_positions_ptr;
  local_2d0 = raw_tangents_len;
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
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
    local_248 = local_388;
    local_238 = (undefined *)0x0;
    pcStack_240 = extraout_RAX;
    if (local_388 != (undefined *)0x1) {
      fVar40 = local_390[4];
      fVar39 = local_390[1];
      local_1c0 = local_390;
      fVar47 = local_390[5] - local_390[2];
      fVar49 = local_390[3] - *local_390;
      local_358 = (float)(___xmm_80000000800000008000000080000000 ^ (uint)fVar47);
      fStack_354 = fVar49;
      ::utils::swizzlers::impl$0::x0y();
      puVar13 = local_238;
      uVar45 = _UNK_1429258f4;
      uVar43 = ___xmm_80000000800000008000000080000000;
      fVar41 = fStack_2b0 * (fVar40 - fVar39) - fVar47 * fStack_2b4;
      fVar47 = local_2b8 * fVar47 - fVar49 * fStack_2b0;
      fVar39 = fVar49 * fStack_2b4 - (fVar40 - fVar39) * local_2b8;
      fVar49 = fVar46 / SQRT(fVar39 * fVar39 + fVar47 * fVar47 + fVar41 * fVar41);
      bVar17 = (int)fVar49 - 1U < 0x7fffff ||
               (int)ABS(fVar49) - 0x800000U < 0x7f000000 && -1 < (int)fVar49;
      fVar40 = __real_80000000;
      if (bVar17) {
        fVar40 = (float)((uint)fVar39 ^ ___xmm_80000000800000008000000080000000) * fVar49;
      }
      if (local_238 == local_248) {
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      uVar34 = (uint)bVar17;
      uVar32 = (int)(uVar34 << 0x1f) >> 0x1f;
      uVar34 = (int)(uVar34 << 0x1f) >> 0x1f;
      local_2c0 = &pfStack_290;
      pfVar20 = (float *)((longlong)puVar13 * 3);
      *(ulonglong *)(pcStack_240 + (longlong)puVar13 * 0xc) =
           CONCAT44(~uVar34 & __xmm_00000000000000008000000080000000._4_4_,
                    ~uVar32 & (uint)__xmm_00000000000000008000000080000000) |
           CONCAT44((uint)(fVar49 * (float)((uint)fVar47 ^ uVar45)) & uVar34,
                    (uint)(fVar49 * (float)((uint)fVar41 ^ uVar43)) & uVar32);
      *(float *)(pcStack_240 + (longlong)puVar13 * 0xc + 8) = fVar40;
      uVar32 = __xmm_00000000000000008000000080000000._4_4_;
      uVar34 = (uint)__xmm_00000000000000008000000080000000;
      fVar40 = __real_80000000;
      local_140 = local_1c0 + (longlong)local_178 * 3;
      bVar17 = true;
      pfVar27 = &local_358;
      pfVar24 = local_1c0;
      while (local_238 = puVar13 + 1, pfVar24 != local_140) {
        pfVar1 = pfVar24 + 3;
        pfVar16 = pfVar24;
        pfVar25 = pfVar1;
        pfVar28 = pfVar20;
        if (bVar17) {
          if ((pfVar1 == local_140) || (pfVar24 + 6 == local_140)) break;
          unaff_R14 = pfVar24;
          pfVar16 = pfVar24 + 6;
          pfVar25 = pfVar24 + 9;
          pfVar28 = pfVar1;
        }
        pfVar20 = pfVar16;
        fVar39 = pfVar20[1];
        fVar41 = unaff_R14[1];
        fVar49 = pfVar20[2] - unaff_R14[2];
        fVar52 = *pfVar20 - *unaff_R14;
        local_358 = (float)((uint)fVar49 ^ uVar43);
        fStack_354 = fVar52;
        ::utils::swizzlers::impl$0::x0y();
        puVar13 = local_238;
        fVar47 = fStack_2b0 * (fVar39 - fVar41) - fVar49 * fStack_2b4;
        fVar49 = local_2b8 * fVar49 - fVar52 * fStack_2b0;
        fVar41 = fVar52 * fStack_2b4 - (fVar39 - fVar41) * local_2b8;
        fVar52 = fVar46 / SQRT(fVar41 * fVar41 + fVar49 * fVar49 + fVar47 * fVar47);
        bVar30 = (int)ABS(fVar52) - 0x800000U < 0x7f000000;
        bVar31 = (int)fVar52 - 1U < 0x7fffff;
        bVar17 = bVar31 || bVar30 && -1 < (int)fVar52;
        fVar39 = fVar40;
        if (bVar17) {
          fVar39 = (float)((uint)fVar41 ^ uVar43) * fVar52;
        }
        if (local_238 == local_248) {
          alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
        }
        uVar36 = (int)((uint)(bVar31 || bVar30 && -1 < (int)fVar52) << 0x1f) >> 0x1f;
        uVar38 = (int)((uint)bVar17 << 0x1f) >> 0x1f;
        *(ulonglong *)(pcStack_240 + (longlong)puVar13 * 0xc) =
             CONCAT44(~uVar38 & uVar32,~uVar36 & uVar34) |
             CONCAT44((uint)(fVar52 * (float)((uint)fVar49 ^ uVar45)) & uVar38,
                      (uint)(fVar52 * (float)((uint)fVar47 ^ uVar43)) & uVar36);
        *(float *)(pcStack_240 + (longlong)puVar13 * 0xc + 8) = fVar39;
        bVar17 = false;
        pfVar24 = pfVar25;
        unaff_R14 = pfVar28;
      }
      fVar40 = local_140[-5];
      fVar39 = local_140[-2];
      fVar47 = local_140[-4] - local_140[-1];
      fVar49 = local_140[-6] - local_140[-3];
      local_358 = (float)(___xmm_80000000800000008000000080000000 ^ (uint)fVar47);
      fStack_354 = fVar49;
      ::utils::swizzlers::impl$0::x0y();
      puVar13 = local_238;
      fVar41 = fStack_2b0 * (fVar40 - fVar39) - fVar47 * fStack_2b4;
      fVar47 = local_2b8 * fVar47 - fVar49 * fStack_2b0;
      fVar39 = fVar49 * fStack_2b4 - (fVar40 - fVar39) * local_2b8;
      fVar46 = fVar46 / SQRT(fVar39 * fVar39 + fVar47 * fVar47 + fVar41 * fVar41);
      bVar17 = (int)fVar46 - 1U < 0x7fffff ||
               (int)ABS(fVar46) - 0x800000U < 0x7f000000 && -1 < (int)fVar46;
      fVar40 = __real_80000000;
      if (bVar17) {
        fVar40 = (float)((uint)fVar39 ^ uVar43) * fVar46;
      }
      if (local_238 == local_248) {
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      uVar34 = (uint)bVar17;
      uVar32 = (int)(uVar34 << 0x1f) >> 0x1f;
      uVar34 = (int)(uVar34 << 0x1f) >> 0x1f;
      *(ulonglong *)(pcStack_240 + (longlong)puVar13 * 0xc) =
           CONCAT44(~uVar34 & __xmm_00000000000000008000000080000000._4_4_,
                    ~uVar32 & (uint)__xmm_00000000000000008000000080000000) |
           CONCAT44((uint)(fVar46 * (float)((uint)fVar47 ^ uVar45)) & uVar34,
                    (uint)(fVar46 * (float)((uint)fVar41 ^ uVar43)) & uVar32);
      *(float *)(pcStack_240 + (longlong)puVar13 * 0xc + 8) = fVar40;
      local_180 = pcStack_240;
      local_2c8 = local_248;
      if (((undefined *)0x555555555555555 < local_178) ||
         (0x7ffffffffffffffc < (ulonglong)((longlong)local_178 * 0x30))) {
LAB_1407ddc4c:
        local_f2 = 1;
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
      if ((longlong)local_178 * 0x30 == 0) {
        lVar22 = 4;
        lVar21 = 0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        lVar21 = local_188;
        lVar22 = extraout_RAX_00;
        if (extraout_RAX_00 == 0) goto LAB_1407ddc4c;
      }
      local_150 = 0;
      local_160 = lVar21;
      local_158 = lVar22;
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
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error();
          }
          local_128 = (undefined **)0x0;
          local_119 = raw_widths_len == (void *)0x0;
          local_198 = lVar22;
          if ((bool)local_119) {
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          local_250 = local_398;
          lVar21 = (longlong)raw_positions_len << 3;
          local_2d0 = (void *)((longlong)local_2d0 << 2);
          uVar37 = 0;
          lVar29 = 0;
          local_2f8 = ZEXT816(0);
          lVar22 = 0;
          local_100 = 0;
          local_144 = 0;
          bVar23 = 0;
          local_119 = 0;
          fVar46 = 0.0;
          fVar40 = 0.0;
          fVar39 = 0.0;
          fVar41 = __real_3e19999a - *(float *)raw_widths_ptr;
          fVar47 = 0.0;
          fVar49 = 0.0;
          fVar52 = 0.0;
          local_130 = extraout_RAX_02;
          do {
            if ((((local_2d0 == (void *)lVar22) || (lVar21 == 0)) ||
                ((longlong)puVar13 * 0xc + 0xc == lVar29)) ||
               ((longlong)raw_widths_len << 2 == lVar22)) break;
            puVar19 = (undefined8 *)((longlong)local_1c0 + lVar29);
            fVar42 = (float)*puVar19;
            fVar44 = (float)((ulonglong)*puVar19 >> 0x20);
            fVar51 = *(float *)(puVar19 + 1);
            fVar48 = __real_3e19999a - *(float *)((longlong)raw_widths_ptr + lVar22);
            fStack_2b4 = *local_2d8;
            local_2b8 = (float)((uint)local_2d8[1] ^ ___xmm_80000000800000008000000080000000);
            local_258 = lVar21;
            ::utils::swizzlers::impl$0::x0y();
            ppuVar26 = local_108;
            fVar50 = *(float *)((longlong)raw_tangents_ptr + lVar22);
            fVar33 = fVar50 * SUB84(local_248,0) * (float)__xmm_00000000000000003f0000003f000000;
            fVar35 = fVar50 * (float)((ulonglong)local_248 >> 0x20) *
                     __xmm_00000000000000003f0000003f000000._4_4_;
            local_148 = pcStack_240._0_4_ * fVar50 * __real_3f000000;
            fStack_170 = 0.0 - _UNK_1429258e8 * 0.0;
            fStack_16c = 0.0 - _UNK_1429258ec * 0.0;
            local_178 = (undefined *)CONCAT44(fVar44 - fVar35,fVar42 - fVar33);
            local_188 = CONCAT44(local_188._4_4_,fVar51 - local_148);
            fStack_1b0 = _UNK_1429258e8 * 0.0 + 0.0;
            fStack_1ac = _UNK_1429258ec * 0.0 + 0.0;
            local_1b8 = (undefined **)CONCAT44(fVar35 + fVar44,fVar33 + fVar42);
            local_148 = local_148 + fVar51;
            if (0.0 < fVar41 != 0.0 < fVar48) {
              fVar41 = fVar41 / (fVar41 - fVar48);
              uVar5 = *(undefined8 *)(local_180 + lVar29);
              local_25c = *(float *)(local_180 + lVar29 + 8) * fVar41;
              iVar7 = (int)local_108;
              if ((bVar23 & 1) != 0) {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_198 = local_158;
                  local_100 = local_150;
                }
                *(int *)(local_198 + local_100 * 4) = (int)local_1a0;
                uVar18 = (undefined4)(local_1a0 >> 0x20);
                *(undefined4 *)(local_198 + 4 + local_100 * 4) = uVar18;
                *(int *)(local_198 + 8 + local_100 * 4) = iVar7;
                *(int *)(local_198 + 0xc + local_100 * 4) = iVar7;
                *(undefined4 *)(local_198 + 0x10 + local_100 * 4) = uVar18;
                *(uint *)(local_198 + 0x14 + local_100 * 4) = iVar7 + 1U;
                local_150 = local_100 + 6;
                local_100 = local_150;
              }
              ppuVar2 = local_108;
              if (__real_00000000 < fVar48) {
                local_1a0 = (ulonglong)ppuVar26 & 0xffffffff | (ulonglong)(iVar7 + 1U) << 0x20;
              }
              local_11a = __real_00000000 < fVar48;
              if (local_108 == local_118) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              fVar51 = __real_3f800000 - fVar41;
              fVar40 = fVar41 * (float)local_178 + fVar40 * fVar51;
              fVar39 = fVar41 * local_178._4_4_ + fVar39 * fVar51;
              fVar46 = fVar41 * (float)local_188 + fVar46 * fVar51;
              *(ulonglong *)(local_110 + (longlong)ppuVar2 * 0xc) = CONCAT44(fVar39,fVar40);
              *(float *)(local_110 + 8 + (longlong)ppuVar2 * 0xc) = fVar46;
              ppuVar26 = (undefined **)((longlong)ppuVar2 + 1);
              if (ppuVar26 == local_118) {
                local_108 = ppuVar26;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar4 = local_128;
              fVar50 = fVar41 * (float)local_1b8 + fVar47 * fVar51;
              fVar47 = fVar41 * local_1b8._4_4_ + fVar49 * fVar51;
              fVar49 = fVar41 * local_148 + fVar52 * fVar51;
              *(ulonglong *)(local_110 + (longlong)ppuVar26 * 0xc) = CONCAT44(fVar47,fVar50);
              *(float *)(local_110 + 8 + (longlong)ppuVar26 * 0xc) = fVar49;
              local_108 = (undefined **)((longlong)ppuVar2 + 2);
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              local_3d8 = (float)uVar37;
              fStack_3d4 = (float)((ulonglong)uVar37 >> 0x20);
              local_3c8 = (float)uVar5;
              fStack_3c4 = (float)((ulonglong)uVar5 >> 0x20);
              fVar52 = fVar41 * local_3c8 + local_3d8 * fVar51;
              fVar33 = fVar41 * fStack_3c4 + fStack_3d4 * fVar51;
              fVar41 = (float)local_2f8._0_4_ * fVar51 + local_25c;
              *(ulonglong *)(local_130 + (longlong)ppuVar4 * 0xc) = CONCAT44(fVar33,fVar52);
              *(float *)(local_130 + 8 + (longlong)ppuVar4 * 0xc) = fVar41;
              ppuVar26 = (undefined **)((longlong)ppuVar4 + 1);
              if (ppuVar26 == local_138) {
                local_128 = ppuVar26;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar2 = local_108;
              *(ulonglong *)(local_130 + (longlong)ppuVar26 * 0xc) = CONCAT44(fVar33,fVar52);
              *(float *)(local_130 + 8 + (longlong)ppuVar26 * 0xc) = fVar41;
              local_128 = (undefined **)((longlong)ppuVar4 + 2);
              iVar7 = (int)local_108;
              if ((local_144 & 1) != 0) {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_100 = local_150;
                }
                *(int *)(local_158 + local_100 * 4) = (int)pfVar27;
                uVar18 = (undefined4)((ulonglong)pfVar27 >> 0x20);
                *(undefined4 *)(local_158 + 4 + local_100 * 4) = uVar18;
                *(int *)(local_158 + 8 + local_100 * 4) = iVar7;
                *(int *)(local_158 + 0xc + local_100 * 4) = iVar7;
                *(undefined4 *)(local_158 + 0x10 + local_100 * 4) = uVar18;
                local_198 = local_158;
                *(uint *)(local_158 + 0x14 + local_100 * 4) = iVar7 + 1U;
                local_150 = local_100 + 6;
                local_100 = local_150;
              }
              ppuVar26 = local_108;
              if (__real_00000000 < fVar48) {
                pfVar27 = (float *)((ulonglong)ppuVar2 & 0xffffffff |
                                   (ulonglong)(iVar7 + 1U) << 0x20);
              }
              fVar47 = fVar47 + __real_be4ccccd;
              if (local_108 == local_118) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(float *)(local_110 + (longlong)ppuVar26 * 0xc) = fVar50;
              *(float *)(local_110 + 4 + (longlong)ppuVar26 * 0xc) = fVar47;
              *(float *)(local_110 + 8 + (longlong)ppuVar26 * 0xc) = fVar49;
              ppuVar2 = (undefined **)((longlong)ppuVar26 + 1);
              fVar39 = fVar39 + __real_be4ccccd;
              if (ppuVar2 == local_118) {
                local_108 = ppuVar2;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar4 = local_128;
              *(float *)(local_110 + (longlong)ppuVar2 * 0xc) = fVar40;
              *(float *)(local_110 + 4 + (longlong)ppuVar2 * 0xc) = fVar39;
              *(float *)(local_110 + 8 + (longlong)ppuVar2 * 0xc) = fVar46;
              local_108 = (undefined **)((longlong)ppuVar26 + 2);
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              uVar43 = (uint)fVar52 ^ ___xmm_80000000800000008000000080000000;
              uVar45 = (uint)fVar33 ^ _UNK_1429258f4;
              uVar34 = (uint)fVar41 ^ ___xmm_80000000800000008000000080000000;
              *(ulonglong *)(local_130 + (longlong)ppuVar4 * 0xc) = CONCAT44(uVar45,uVar43);
              *(uint *)(local_130 + 8 + (longlong)ppuVar4 * 0xc) = uVar34;
              ppuVar26 = (undefined **)((longlong)ppuVar4 + 1);
              if (ppuVar26 == local_138) {
                local_128 = ppuVar26;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(ulonglong *)(local_130 + (longlong)ppuVar26 * 0xc) = CONCAT44(uVar45,uVar43);
              *(uint *)(local_130 + 8 + (longlong)ppuVar26 * 0xc) = uVar34;
              local_128 = (undefined **)((longlong)ppuVar4 + 2);
              local_144 = (uint)local_11a;
              bVar23 = local_11a;
            }
            if (__real_00000000 < fVar48) {
              iVar7 = (int)local_108;
              local_2f8._0_8_ = local_108;
              if ((bVar23 & 1) == 0) {
                ppuVar26 = local_108;
                if (local_108 == local_118) goto LAB_1407dd1bf;
              }
              else {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_100 = local_150;
                }
                *(int *)(local_158 + local_100 * 4) = (int)local_1a0;
                uVar18 = (undefined4)(local_1a0 >> 0x20);
                *(undefined4 *)(local_158 + 4 + local_100 * 4) = uVar18;
                *(int *)(local_158 + 8 + local_100 * 4) = iVar7;
                *(int *)(local_158 + 0xc + local_100 * 4) = iVar7;
                *(undefined4 *)(local_158 + 0x10 + local_100 * 4) = uVar18;
                local_198 = local_158;
                *(int *)(local_158 + 0x14 + local_100 * 4) = iVar7 + 1;
                local_150 = local_100 + 6;
                ppuVar26 = local_108;
                local_100 = local_150;
                if (local_108 == local_118) {
LAB_1407dd1bf:
                  ppuVar26 = local_108;
                  alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                }
              }
              *(undefined **)(local_110 + (longlong)ppuVar26 * 0xc) = local_178;
              *(float *)(local_110 + 8 + (longlong)ppuVar26 * 0xc) = (float)local_188;
              ppuVar2 = (undefined **)((longlong)ppuVar26 + 1);
              if (ppuVar2 == local_118) {
                local_108 = ppuVar2;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar4 = local_128;
              plVar3 = (longlong *)(local_180 + lVar29);
              *(undefined ***)(local_110 + (longlong)ppuVar2 * 0xc) = local_1b8;
              *(float *)(local_110 + 8 + (longlong)ppuVar2 * 0xc) = local_148;
              local_108 = (undefined **)((longlong)ppuVar26 + 2);
              local_248 = (undefined *)*plVar3;
              pcStack_240._0_4_ = (float)(int)plVar3[1];
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              *(float *)(local_130 + 8 + (longlong)ppuVar4 * 0xc) = pcStack_240._0_4_;
              *(undefined **)(local_130 + (longlong)ppuVar4 * 0xc) = local_248;
              ppuVar26 = (undefined **)((longlong)ppuVar4 + 1);
              pcStack_240 = (char *)CONCAT44(pcStack_240._4_4_,(int)plVar3[1]);
              local_248 = (undefined *)*plVar3;
              if (ppuVar26 == local_138) {
                local_128 = ppuVar26;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar2 = local_108;
              *(float *)(local_130 + 8 + (longlong)ppuVar26 * 0xc) = pcStack_240._0_4_;
              *(undefined **)(local_130 + (longlong)ppuVar26 * 0xc) = local_248;
              local_128 = (undefined **)((longlong)ppuVar4 + 2);
              iVar9 = (int)local_108;
              if ((local_144 & 1) == 0) {
                fVar46 = local_1b8._4_4_ + __real_be4ccccd;
                ppuVar26 = local_108;
                if (local_108 == local_118) goto LAB_1407dd3dc;
              }
              else {
                if ((ulonglong)(local_160 - local_100) < 6) {
                  alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
                  local_100 = local_150;
                }
                *(int *)(local_158 + local_100 * 4) = (int)pfVar27;
                uVar18 = (undefined4)((ulonglong)pfVar27 >> 0x20);
                *(undefined4 *)(local_158 + 4 + local_100 * 4) = uVar18;
                *(int *)(local_158 + 8 + local_100 * 4) = iVar9;
                *(int *)(local_158 + 0xc + local_100 * 4) = iVar9;
                *(undefined4 *)(local_158 + 0x10 + local_100 * 4) = uVar18;
                local_198 = local_158;
                *(uint *)(local_158 + 0x14 + local_100 * 4) = iVar9 + 1U;
                local_150 = local_100 + 6;
                fVar46 = local_1b8._4_4_ + __real_be4ccccd;
                ppuVar26 = local_108;
                local_100 = local_150;
                if (local_108 == local_118) {
LAB_1407dd3dc:
                  ppuVar26 = local_108;
                  alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
                }
              }
              *(float *)(local_110 + (longlong)ppuVar26 * 0xc) = (float)local_1b8;
              *(float *)(local_110 + 4 + (longlong)ppuVar26 * 0xc) = fVar46;
              *(float *)(local_110 + 8 + (longlong)ppuVar26 * 0xc) = local_148;
              ppuVar4 = (undefined **)((longlong)ppuVar26 + 1);
              fVar46 = local_178._4_4_ + __real_be4ccccd;
              if (ppuVar4 == local_118) {
                local_108 = ppuVar4;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              ppuVar15 = local_128;
              *(float *)(local_110 + (longlong)ppuVar4 * 0xc) = (float)local_178;
              *(float *)(local_110 + 4 + (longlong)ppuVar4 * 0xc) = fVar46;
              *(float *)(local_110 + 8 + (longlong)ppuVar4 * 0xc) = (float)local_188;
              local_108 = (undefined **)((longlong)ppuVar26 + 2);
              uVar37 = *(undefined8 *)(local_180 + lVar29);
              uVar43 = *(uint *)(local_180 + lVar29 + 8);
              if (local_128 == local_138) {
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              uVar43 = uVar43 ^ ___xmm_80000000800000008000000080000000;
              *(ulonglong *)(local_130 + (longlong)ppuVar15 * 0xc) =
                   CONCAT44((uint)((ulonglong)uVar37 >> 0x20) ^ _UNK_1429258f4,
                            (uint)uVar37 ^ ___xmm_80000000800000008000000080000000);
              *(uint *)(local_130 + 8 + (longlong)ppuVar15 * 0xc) = uVar43;
              ppuVar26 = (undefined **)((longlong)ppuVar15 + 1);
              uVar37 = *(undefined8 *)(local_180 + lVar29);
              uVar43 = *(uint *)(local_180 + lVar29 + 8);
              if (ppuVar26 == local_138) {
                local_128 = ppuVar26;
                alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
              }
              local_1a0 = CONCAT44(iVar7 + 1,local_2f8._0_4_);
              pfVar27 = (float *)((ulonglong)ppuVar2 & 0xffffffff | (ulonglong)(iVar9 + 1U) << 0x20)
              ;
              uVar43 = uVar43 ^ ___xmm_80000000800000008000000080000000;
              *(ulonglong *)(local_130 + (longlong)ppuVar26 * 0xc) =
                   CONCAT44((uint)((ulonglong)uVar37 >> 0x20) ^ _UNK_1429258f4,
                            (uint)uVar37 ^ ___xmm_80000000800000008000000080000000);
              *(uint *)(local_130 + 8 + (longlong)ppuVar26 * 0xc) = uVar43;
              local_128 = (undefined **)((longlong)ppuVar15 + 2);
              bVar23 = 1;
              local_144 = 1;
            }
            local_2d8 = local_2d8 + 2;
            uVar37 = *(undefined8 *)(local_180 + lVar29);
            local_2f8 = ZEXT416(*(uint *)(local_180 + lVar29 + 8));
            lVar22 = lVar22 + 4;
            lVar29 = lVar29 + 0xc;
            lVar21 = local_258 + -8;
            fVar46 = (float)local_188;
            fVar40 = (float)local_178;
            fVar39 = local_178._4_4_;
            fVar41 = fVar48;
            fVar47 = (float)local_1b8;
            fVar49 = local_1b8._4_4_;
            fVar52 = local_148;
          } while ((float *)((longlong)puVar19 + 0xc) != local_140);
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
                    /* WARNING: Subroutine does not return */
              alloc::alloc::handle_alloc_error();
            }
            extraout_RAX_03[6] = (undefined *)local_218;
            extraout_RAX_03[4] = (undefined *)CONCAT44(uStack_228._4_4_,(undefined4)uStack_228);
            extraout_RAX_03[5] = (undefined *)pfStack_220;
            extraout_RAX_03[2] = local_238;
            extraout_RAX_03[3] = (undefined *)CONCAT44(uStack_230._4_4_,(undefined4)uStack_230);
            *extraout_RAX_03 = local_248;
            extraout_RAX_03[1] = pcStack_240;
            pcVar14 = DAT_14293e0c8;
            puVar13 = __xmm_3f800000000000000000000000000000;
            local_1b8 = extraout_RAX_03;
            local_190 = extraout_RAX_03;
            if (lStack_340 != 0) {
              plVar3 = *(longlong **)((longlong)raw_reset_output + 0xb0);
              lVar21 = 0;
              do {
                uVar37 = *(undefined8 *)((longlong)local_140 + lVar21);
                ppfVar10 = (float **)((undefined8 *)((longlong)local_140 + lVar21))[1];
                puVar19 = (undefined8 *)((longlong)local_140 + lVar21 + 0x10);
                uVar5 = *puVar19;
                uVar11 = puVar19[1];
                country_core::render::nani::trimesh::TrimeshChunk::index_count();
                uVar43 = extraout_EAX / 3 - 1;
                if (extraout_EAX / 3 == 0) {
                  uVar43 = 0;
                }
                if ((uVar43 != 0) && (iVar7 = 0x1f, uVar43 != 0)) {
                  for (; uVar43 >> iVar7 == 0; iVar7 = iVar7 + -1) {
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
                puVar6 = *local_190;
                *local_190 = *local_190 + 1;
                UNLOCK();
                if (*local_190 == (undefined *)0x0 ||
                    SCARRY8((longlong)puVar6,1) != (longlong)*local_190 < 0) {
                    /* WARNING: Does not return */
                  pcVar8 = (code *)invalidInstructionException();
                  (*pcVar8)();
                }
                local_248 = puVar13;
                pcStack_240 = pcVar14;
                uStack_230._0_4_ = 0;
                local_238 = (undefined *)0x0;
                uStack_230._4_4_ = 0x3f800000;
                uStack_228._0_4_ = 0x3f800000;
                uStack_228._4_4_ = 0x3f800000;
                local_218 = local_308;
                pcStack_210 = pcStack_300;
                local_208 = CONCAT71(local_208._1_7_,0x1a);
                local_1f8 = local_190;
                local_1d0 = raw_info28._0_4_;
                local_1f0 = uVar37;
                ppfStack_1e8 = ppfVar10;
                local_1e0 = uVar5;
                uStack_1d8 = uVar11;
                bevy_ecs::system::commands::Commands::
                spawn<tuple_<country_core::components::drawable::NaniBundle<country_core::render::nani::instance_data::StairsFloorInstanceData>,country_core::resources::stairs::visual::StairsFloorMeshChunk>_>
                          ();
                puVar6 = local_298;
                lVar22 = plVar3[2];
                if (lVar22 == *plVar3) {
                  alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
                }
                *(undefined **)(plVar3[1] + lVar22 * 8) = puVar6;
                plVar3[2] = lVar22 + 1;
                lVar21 = lVar21 + 0x20;
              } while (lStack_340 << 5 != lVar21);
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
            uVar12 = CONCAT44(fStack_2b4,local_2b8);
            lVar21 = 0;
            if (uVar12 - 9 < 2) {
              lVar21 = uVar12 - 8;
            }
            if (lVar21 == 0) {
              puVar13 = local_298;
              if (((uVar12 < 6) && ((0x32UL >> ((ulonglong)(uint)local_2b8 & 0x3f) & 1) != 0)) &&
                 (CONCAT44(uStack_2ac,fStack_2b0) != 0)) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                puVar13 = local_298;
              }
            }
            else {
              if (lVar21 == 1) {
                core::ptr::drop_in_place<backtrace::capture::Backtrace>();
                goto LAB_1407ddb33;
              }
              local_2c0 = &local_2a8;
              puVar13 = (undefined *)CONCAT44(uStack_2ac,fStack_2b0);
            }
            if (puVar13 != (undefined *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
LAB_1407ddb33:
          puVar13 = local_2c8;
          if (local_138 != (undefined **)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_118 != (undefined **)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_160 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (puVar13 != (undefined *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          return;
        }
      }
      local_f3 = 1;
      local_2e0 = lVar21;
                    /* WARNING: Subroutine does not return */
      alloc::raw_vec::handle_error();
    }
  }
  local_238 = (undefined *)0x0;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_bounds_check();
}

