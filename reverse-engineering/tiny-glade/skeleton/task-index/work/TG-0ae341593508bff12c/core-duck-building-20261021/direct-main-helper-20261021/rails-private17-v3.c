
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings
          (void *raw_reset_output,void *raw_widths,void *raw_tangents,void *raw_positions,
          void *raw_rng,void *raw_info28,void *raw_extra08,void *raw_arg8,void *raw_pair_flags,
          void *raw_railing_style,void *raw_flag11,void *raw_flag12,void *raw_flag13,
          void *raw_flag14,void *raw_roof_options,void *raw_samples_ptr,void *raw_samples_len)

{
  float *pfVar1;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  bool bVar11;
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 *puVar14;
  undefined8 uVar15;
  size_t _Size;
  byte bVar16;
  uint extraout_EAX;
  undefined1 *puVar17;
  ulonglong uVar18;
  float *extraout_RAX;
  float *extraout_RAX_00;
  float *pfVar19;
  float *extraout_RAX_01;
  float *_Dst;
  undefined1 **ppuVar20;
  byte bVar21;
  undefined1 *puVar22;
  undefined1 *puVar23;
  longlong lVar24;
  float *pfVar25;
  undefined8 *puVar26;
  undefined1 *puVar27;
  ulonglong uVar28;
  uint uVar29;
  ulonglong uVar30;
  ulonglong uVar31;
  uint *puVar32;
  float *unaff_R13;
  float *pfVar33;
  float *pfVar34;
  float *pfVar35;
  bool bVar36;
  uint uVar37;
  float extraout_XMM0_Da;
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
  float fVar38;
  float fVar39;
  float fVar40;
  undefined1 extraout_XMM0 [16];
  undefined1 extraout_XMM0_00 [16];
  undefined1 extraout_XMM0_01 [16];
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  float fVar46;
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
  float unaff_XMM12_Dc;
  float fVar57;
  float fVar58;
  float fVar59;
  undefined1 *local_518;
  float *pfStack_510;
  undefined8 uStack_508;
  undefined8 uStack_500;
  undefined4 uStack_4f8;
  undefined4 uStack_4f4;
  undefined4 uStack_4f0;
  undefined4 uStack_4ec;
  undefined1 *puStack_4e8;
  undefined8 uStack_4e0;
  undefined1 local_4d8 [16];
  undefined1 *puStack_4c8;
  undefined1 **ppuStack_4c0;
  undefined1 *local_498;
  float *pfStack_490;
  longlong lStack_488;
  longlong lStack_480;
  undefined8 uStack_478;
  undefined8 uStack_470;
  undefined8 uStack_468;
  undefined8 uStack_460;
  float fStack_458;
  undefined8 *local_450;
  ulonglong local_448;
  ulonglong local_440;
  undefined1 local_438 [16];
  undefined1 auStack_428 [16];
  undefined1 *local_410;
  undefined1 *puStack_408;
  undefined1 *puStack_400;
  float local_3f4;
  float local_3f0;
  float local_3ec;
  float local_3e8;
  float local_3e4;
  float local_3e0;
  float local_3dc;
  float local_3d8;
  float local_3d4;
  undefined1 *local_3d0;
  undefined1 *puStack_3c8;
  undefined1 *puStack_3c0;
  ulonglong uStack_3b8;
  undefined1 *local_3b0;
  undefined1 *local_3a8;
  undefined8 local_3a0;
  longlong lStack_398;
  longlong lStack_390;
  undefined8 uStack_388;
  undefined8 uStack_380;
  undefined8 uStack_378;
  float local_368;
  float fStack_364;
  float fStack_360;
  float fStack_35c;
  float fStack_358;
  float fStack_354;
  float fStack_350;
  undefined8 local_348;
  float local_340;
  float local_33c;
  float fStack_338;
  float local_334;
  undefined8 local_330;
  undefined1 *local_328;
  undefined1 **ppuStack_320;
  undefined8 uStack_318;
  undefined8 uStack_310;
  undefined8 uStack_308;
  undefined4 uStack_300;
  undefined4 uStack_2fc;
  undefined8 uStack_2f4;
  uint uStack_2ec;
  float fStack_2e0;
  float fStack_2dc;
  float fStack_2d8;
  float fStack_2d4;
  float fStack_2d0;
  uint local_2cc;
  uint local_2c8;
  float fStack_2c4;
  uint uStack_2c0;
  uint uStack_2bc;
  float fStack_2b8;
  undefined4 local_2b4;
  undefined1 *puStack_2b0;
  float *pfStack_2a8;
  undefined1 *puStack_2a0;
  float *local_298;
  void *local_290;
  undefined1 *local_288;
  uint *local_280;
  undefined1 *local_278;
  float *local_270;
  undefined1 local_268 [16];
  undefined1 *puStack_258;
  undefined1 **ppuStack_250;
  undefined8 uStack_248;
  float *local_240;
  undefined8 *local_238;
  ulonglong local_230;
  undefined1 *local_228;
  size_t local_220;
  undefined8 local_218;
  undefined8 uStack_210;
  float fStack_200;
  float fStack_1fc;
  undefined1 *puStack_1f8;
  ulonglong local_1f0;
  undefined8 local_1e8;
  undefined8 local_1e0;
  undefined1 auStack_1d8 [16];
  undefined8 uStack_1c8;
  undefined8 uStack_1c0;
  undefined8 uStack_1b8;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined1 local_198 [16];
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined4 local_160;
  undefined4 local_15c;
  undefined8 local_154;
  undefined4 local_14c;
  void *local_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined1 local_128 [16];
  float local_118;
  float local_114;
  float local_110;
  uint uStack_10c;
  undefined1 *local_108;
  float *local_100;
  char cStack_f2;
  undefined1 uStack_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_348 = *(undefined1 **)((longlong)raw_positions + 0x10);
  local_290 = raw_reset_output;
  if (local_348 != raw_samples_len) {
    local_1a8 = (undefined1 *)0x0;
                    /* WARNING: Subroutine does not return */
    core::panicking::assert_failed<usize,usize>();
  }
  local_118 = 0.0;
  local_114 = 0.0;
  if ((*(int *)raw_roof_options == 1) &&
     (local_114 = __real_3e19999a - *(float *)((longlong)raw_roof_options + 4), local_114 <= 0.0)) {
    local_114 = 0.0;
  }
  if ((*(int *)((longlong)raw_roof_options + 8) == 1) &&
     (local_118 = __real_3e19999a - *(float *)((longlong)raw_roof_options + 0xc), local_118 <= 0.0))
  {
    local_118 = 0.0;
  }
  local_440 = ((ulonglong)raw_flag11 & 0xff) - 1;
  pfVar19 = (float *)0x0;
  local_1f0 = 0;
  if ((byte)((byte)raw_flag11 - 2) < 3) {
    local_1f0 = local_440;
  }
  local_448 = (ulonglong)raw_railing_style & 0xff;
  local_228 = *(undefined1 **)((longlong)raw_widths + 0x10);
  local_3b0 = (undefined1 *)((longlong)raw_samples_len + -1);
  local_110 = (float)local_3b0;
  puVar32 = *(uint **)((longlong)raw_widths + 8);
  local_140 = *(void **)((longlong)raw_positions + 8);
  local_288 = *(undefined1 **)((longlong)raw_tangents + 0x10);
  local_238 = *(undefined8 **)((longlong)raw_tangents + 8);
  puVar26 = *(undefined8 **)((longlong)raw_tangents + 8) + (longlong)(local_288 + -1);
  if (local_288 == (undefined1 *)0x0) {
    puVar26 = (undefined8 *)0x0;
  }
  local_450 = (undefined8 *)&DAT_142a8c9d0;
  if (puVar26 != (undefined8 *)0x0) {
    local_450 = puVar26;
  }
  local_2c8 = (uint)(local_1f0 == 1) << 7 | 0x40;
  local_2b4 = *(undefined4 *)(&__real_3fa893753f400000 + (ulonglong)(local_1f0 != 3) * 4);
  local_220 = (longlong)raw_samples_len * 0xc;
  puVar17 = raw_samples_len;
  if (local_228 < raw_samples_len) {
    puVar17 = local_228;
  }
  local_3d0 = (undefined1 *)((ulonglong)local_288 & 0x1fffffffffffffff);
  if (puVar17 <= (undefined1 *)((ulonglong)local_288 & 0x1fffffffffffffff)) {
    local_3d0 = puVar17;
  }
  local_2cc = (uint)((ushort)raw_pair_flags >> 8);
  local_108 = raw_samples_len;
  puVar17 = raw_samples_len;
  local_280 = puVar32;
  while( true ) {
    uVar29 = (uint)pfVar19;
    if (1 < uVar29) break;
    uVar18 = local_448;
    if ((char)raw_flag14 != '\0') {
      uVar18 = 0;
    }
    if (uVar29 != 1) {
      uVar18 = local_448;
    }
    local_230 = uVar18;
    if ((char)raw_flag13 != '\0') {
      local_230 = 0;
    }
    if (uVar29 != 0) {
      local_230 = uVar18;
    }
    if (local_1f0 == 2) {
      local_230 = 0;
    }
    local_128 = ZEXT416(__real_bd23d70a);
                    /* WARNING: Switch is manually overridden */
    switch(local_230) {
    case 1:
      local_128 = ZEXT416(___real_3e72b021);
                    /* WARNING: Switch is manually overridden */
      switch(local_1f0) {
      case 1:
        uVar37 = ___real_3e8ac083;
        goto LAB_1407f9c28;
      case 2:
        goto override_jmp_1407f9bec_case_2;
      }
      break;
    case 2:
      local_128 = ZEXT416(___real_3f147ae1);
      if (local_1f0 == 2) {
        return;
      }
      break;
    case 3:
      uVar37 = ___real_3d75c28f;
LAB_1407f9c28:
      local_128 = ZEXT416(uVar37);
    }
    if (local_228 == (undefined1 *)0x0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    auStack_428 = ZEXT416(*puVar32);
    if (puVar17 == (undefined1 *)0x0) {
      pfVar19 = (float *)&DAT_00000004;
      puVar22 = (undefined1 *)0x0;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      pfVar19 = extraout_RAX;
      puVar22 = puVar17;
      if (extraout_RAX == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::raw_vec::handle_error();
      }
    }
    uStack_10c = uVar29 + 1;
    puStack_2a0 = (undefined1 *)0x0;
    local_368 = (float)(local_128._0_4_ & ___xmm_80000000800000008000000080000000);
    fStack_364 = (float)(local_128._4_4_ & _UNK_1429258f4);
    fStack_360 = (float)(local_128._8_4_ & _UNK_1429258f8);
    fStack_35c = (float)(local_128._12_4_ & _UNK_1429258fc);
    puVar27 = (undefined1 *)0x0;
    puVar23 = (undefined1 *)0x0;
    puStack_2b0 = puVar22;
    pfStack_2a8 = pfVar19;
    while (puStack_1f8 = puVar27, puVar27 < local_3d0) {
      puVar22 = puVar23 + 1;
      fVar42 = *(float *)((longlong)local_140 + (longlong)puVar27 * 0xc + 4);
      uVar37 = -(uint)NAN(fVar42);
      fVar51 = fVar42;
      if (puVar22 < puVar17) {
        fVar51 = *(float *)((longlong)local_140 + (longlong)puVar22 * 0xc + 4);
        fVar56 = fVar51;
        if (fVar51 <= fVar42) {
          fVar56 = fVar42;
        }
        fVar51 = (float)(~uVar37 & (uint)fVar56 | uVar37 & (uint)fVar51);
      }
      fVar56 = (float)puVar32[(longlong)puVar27] + ___real_be89374c;
      if ((float)puVar32[(longlong)puVar27] + ___real_be89374c <= __real_3dcccccd) {
        fVar56 = __real_3dcccccd;
      }
      if (raw_samples_len <= puVar23) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      local_218 = (undefined1 *)(ulonglong)(uint)fVar51;
      uStack_210 = 0;
      local_138 = (float *)CONCAT44(local_138._4_4_,fVar42);
      fVar42 = *(float *)((longlong)raw_samples_ptr + (longlong)puVar23 * 4);
      fastrand::Rng::f32();
      local_348 = (undefined1 *)
                  CONCAT44(*(undefined4 *)(local_238 + (longlong)puStack_1f8),
                           *(uint *)((longlong)local_238 + (longlong)puStack_1f8 * 8 + 4) ^
                           ___xmm_80000000800000008000000080000000);
      ::utils::swizzlers::impl$0::x0y();
      pfVar19 = (float *)((longlong)local_140 + (longlong)puVar27 * 0xc);
      fVar41 = (float)local_1a8;
      fVar47 = local_1a8._4_4_;
      unaff_XMM12_Dc = 0.0;
      fVar53 = (float)uStack_1a0;
      fVar51 = *pfVar19;
      fVar55 = pfVar19[1];
      local_348 = (undefined1 *)
                  CONCAT44(*(undefined4 *)(local_238 + (longlong)puStack_1f8),
                           *(uint *)((longlong)local_238 + (longlong)puStack_1f8 * 8 + 4) ^
                           ___xmm_80000000800000008000000080000000);
      local_100._0_4_ = pfVar19[2];
      ::utils::swizzlers::impl$0::x0y();
      puVar14 = puStack_2a0;
      fVar56 = fVar56 + fVar42 * __real_3eb33333 * extraout_XMM0_Da;
      if (uVar29 == 0) {
        local_100._0_4_ = local_100._0_4_ - fVar53 * fVar56 * __real_3f000000;
        fVar55 = fVar55 - fVar47 * fVar56 * __real_3f000000;
        fVar51 = fVar51 - fVar41 * fVar56 * __real_3f000000;
      }
      else {
        fVar51 = (float)local_1a8 * fVar56 * __real_3f000000 + *pfVar19;
        fVar55 = local_1a8._4_4_ * fVar56 * __real_3f000000 + pfVar19[1];
        local_100._0_4_ = fVar56 * (float)uStack_1a0 * __real_3f000000 + pfVar19[2];
      }
      fVar42 = ((float)puVar23 / local_110) * local_118 +
               (__real_3f800000 - (float)puVar23 / local_110) * local_114;
      fVar56 = fVar55 + fVar42 + (float)local_128._0_4_;
      if (local_230 != 0) {
        fVar55 = (float)local_218;
        if ((float)local_218 <= (float)local_138) {
          fVar55 = (float)local_138;
        }
        fVar41 = (float)(~uVar37 & (uint)fVar55 | uVar37 & (uint)(float)local_218);
        fVar55 = fVar41;
        if (fVar41 <= fVar56) {
          fVar55 = fVar56;
        }
        fVar56 = (float)(~-(uint)NAN(fVar56) & (uint)fVar55 | -(uint)NAN(fVar56) & (uint)fVar41);
      }
      if (puStack_2a0 == puStack_2b0) {
        alloc::raw_vec::RawVec::grow_one<glam::f32::vec3::Vec3,alloc::alloc::Global>();
      }
      puVar27 = puStack_1f8 + 1;
      fVar42 = fVar42 * __real_00000000;
      pfStack_2a8[(longlong)puVar14 * 3] = fVar51 + fVar42 + local_368;
      pfStack_2a8[(longlong)puVar14 * 3 + 1] = fVar56;
      pfStack_2a8[(longlong)puVar14 * 3 + 2] = fVar42 + local_100._0_4_ + local_368;
      puStack_2a0 = puVar14 + 1;
      puVar23 = puVar22;
    }
    local_198._0_8_ = puStack_2a0;
    local_1a8 = puStack_2b0;
    uStack_1a0 = pfStack_2a8;
    ::utils::curve::Curve::
    try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
    if (!SBORROW8(0,(longlong)local_348)) {
      local_178 = uStack_318;
      uStack_1a0 = (float *)CONCAT44(local_33c,local_340);
      local_198._4_4_ = local_334;
      local_198._0_4_ = fStack_338;
      local_198._8_8_ = local_330;
      local_188 = local_328;
      uStack_180 = ppuStack_320;
      local_1a8 = local_348;
      ::utils::curve::Curve::try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
                ();
      fVar42 = __real_3f800000;
      if (local_1a8 != (undefined1 *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_198._8_8_ != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (!SBORROW8(0,(longlong)local_498)) {
        local_3a8 = local_498;
        local_3a0 = pfStack_490;
        lStack_398 = lStack_488;
        lStack_390 = lStack_480;
        uStack_388 = uStack_478;
        uStack_380 = uStack_470;
        uStack_378 = uStack_468;
        fStack_1fc = 0.0;
        uStack_3b8 = local_230 - 2;
        bVar36 = uStack_3b8 != 0;
        if (!bVar36) {
          fStack_1fc = __real_bd4ccccd;
        }
        fStack_2d8 = __real_bdcccccd;
        if (1 < uStack_3b8) {
          fStack_2d8 = 0.0;
        }
        uStack_2bc = 2 - bVar36;
        fStack_2b8 = *(float *)(&__real_befae148bf0a3d71 + (ulonglong)bVar36 * 4);
        fStack_2d8 = fStack_2d8 +
                     *(float *)(&__real_3e970a3f3f0d4fe0 + (ulonglong)(local_230 == 0) * 4);
        fStack_2d4 = fStack_2d8 + ___real_3e2c0831;
        auStack_428 = ZEXT416(~-(uint)(__real_3f000000 < (float)auStack_428._0_4_) & __real_bdb43958
                             );
        fStack_2d0 = fStack_2d8 * 0.0 * __real_3f000000;
        fStack_2d8 = fStack_2d8 * __real_3f000000;
        uVar29 = 0;
        while (uVar29 < uStack_2bc) {
          uStack_f1 = 1;
          ::utils::random_splits();
          puVar22 = puStack_400;
          puVar17 = puStack_408;
          local_1e8 = (undefined1 *)0x0;
          local_1e0 = (float *)&DAT_00000004;
          auVar12._8_8_ = 0;
          auVar12._0_8_ = auStack_1d8._8_8_;
          auStack_1d8 = auVar12 << 0x40;
          auVar2._8_8_ = 0;
          auVar2._0_8_ = puStack_400;
          uVar18 = SUB168(auVar2 * ZEXT816(0xc),0);
          puStack_3c8 = puStack_408;
          if ((SUB168(auVar2 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < uVar18)) {
LAB_1407fbfea:
                    /* WARNING: Subroutine does not return */
            alloc::raw_vec::handle_error();
          }
          if (uVar18 == 0) {
            pfVar19 = (float *)&DAT_00000004;
            puVar23 = (undefined1 *)0x0;
          }
          else {
            std::alloc::__default_lib_allocator::__rust_alloc();
            pfVar19 = extraout_RAX_00;
            puVar23 = puVar22;
            if (extraout_RAX_00 == (float *)0x0) goto LAB_1407fbfea;
          }
          uStack_1a0 = (float *)(puVar17 + (longlong)puVar22 * 4);
          auVar13._8_8_ = 0;
          auVar13._0_8_ = local_268._8_8_;
          local_268 = auVar13 << 0x40;
          local_198._8_8_ = raw_samples_ptr;
          local_198._0_8_ = &local_1e8;
          local_188 = local_108;
          uStack_180 = &local_3a8;
          local_1a8 = puVar17;
          local_348 = local_268;
          local_340 = 0.0;
          local_33c = 0.0;
          fStack_338 = SUB84(pfVar19,0);
          local_334 = (float)((ulonglong)pfVar19 >> 0x20);
          local_278 = puVar23;
          local_270 = pfVar19;
          core::iter::adapters::map::impl$2::
          fold<glam::f32::vec3::Vec3,core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env_4,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<glam::f32::vec3::Vec3,alloc::vec::impl_20::extend_trusted::closure_env_0<glam::f32::vec3::Vec3,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<f32>,system_decorator::decorator_visual::stairs_railings_assemble::assemble_railings::closure_env_4>_>_>_>
                    ();
          fStack_200 = __real_be19999a;
          fStack_2dc = fStack_2b8;
          if (uVar29 == 0) {
            fStack_200 = 0.0;
            fStack_2dc = fStack_1fc;
          }
          uStack_2c0 = uVar29 + 1;
          puStack_3c0 = local_278;
          uVar15 = local_268._0_8_;
          local_138 = local_270 + local_268._0_8_ * 3;
          cStack_f2 = uStack_3b8 != 0 || uVar29 != 0;
          local_240 = local_138 + -3;
          local_368 = (float)CONCAT31(local_368._1_3_,
                                      local_138 + -3 != (float *)0x0 &&
                                      (undefined1 *)local_268._0_8_ != (undefined1 *)0x0);
          local_218 = (undefined1 *)(local_268._0_8_ + -2);
          bVar36 = true;
          local_100 = local_270;
          pfVar19 = local_270;
          puVar17 = (undefined1 *)0x0;
          fVar51 = fStack_2dc;
          while (puVar22 = puVar17, puVar17 = local_108, puVar32 = local_280, uVar29 = uStack_2c0,
                pfVar19 != local_138) {
            pfVar25 = pfVar19 + 3;
            pfVar35 = pfVar25;
            pfVar33 = pfVar19;
            if (bVar36) {
              unaff_R13 = local_138;
              if (pfVar25 == local_138) break;
              unaff_R13 = pfVar19;
              pfVar35 = pfVar19 + 6;
              pfVar33 = pfVar25;
            }
            puVar17 = puVar22 + 1;
            if ((ulonglong)uVar15 <= puVar17 && local_368._0_1_ == '\0') {
                    /* WARNING: Subroutine does not return */
              core::option::unwrap_failed();
            }
            pfVar19 = local_100 + (longlong)puVar17 * 3;
            if ((ulonglong)uVar15 <= puVar17) {
              pfVar19 = local_240;
            }
            puVar23 = (undefined1 *)0x0;
            if (puVar22 != (undefined1 *)0x0) {
              puVar23 = puVar22 + -1;
            }
            if ((ulonglong)uVar15 <= puVar23) {
                    /* WARNING: Subroutine does not return */
              core::option::unwrap_failed();
            }
            fVar41 = *unaff_R13;
            fVar47 = unaff_R13[2];
            fVar53 = unaff_R13[1] + fVar51;
            fVar54 = pfVar33[1] + fVar51;
            fVar56 = *pfVar33;
            unaff_XMM12_Dc = 0.0;
            fVar55 = pfVar33[2];
            if (puVar22 == (undefined1 *)0x0) {
              if (local_288 == (undefined1 *)0x0) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_bounds_check();
              }
              fVar59 = (float)*local_238;
              fVar38 = (float)((ulonglong)*local_238 >> 0x20);
              fVar39 = fVar38 * fVar38 + fVar59 * fVar59;
            }
            else {
              fVar59 = fVar56 - local_100[(longlong)puVar23 * 3];
              fVar38 = fVar55 - local_100[(longlong)puVar23 * 3 + 2];
              fVar39 = fVar38 * fVar38 + fVar59 * fVar59;
            }
            fVar39 = fVar42 / SQRT(fVar39);
            if ((int)fVar39 - 1U < 0x7fffff ||
                (int)ABS(fVar39) - 0x800000U < 0x7f000000 && -1 < (int)fVar39) {
              fVar59 = fVar59 * fVar39;
              fVar38 = fVar38 * fVar39;
              if (puVar22 == local_218) goto LAB_1407fa6a5;
LAB_1407fa619:
              fVar39 = *pfVar19 - fVar41;
              fVar45 = pfVar19[2] - fVar47;
              fVar52 = fVar42 / SQRT(fVar45 * fVar45 + fVar39 * fVar39);
              if ((int)fVar52 - 1U < 0x7fffff ||
                  (int)ABS(fVar52) - 0x800000U < 0x7f000000 && -1 < (int)fVar52) {
                fVar39 = fVar39 * fVar52;
                fVar45 = fVar45 * fVar52;
              }
              else {
                fVar45 = 0.0;
                fVar39 = 0.0;
              }
            }
            else {
              fVar38 = 0.0;
              fVar59 = 0.0;
              if (puVar22 != local_218) goto LAB_1407fa619;
LAB_1407fa6a5:
              fVar39 = (float)*local_450;
              fVar45 = (float)((ulonglong)*local_450 >> 0x20);
              fVar52 = fVar42 / SQRT(fVar45 * fVar45 + fVar39 * fVar39);
              if ((int)fVar52 - 1U < 0x7fffff ||
                  (int)ABS(fVar52) - 0x800000U < 0x7f000000 && -1 < (int)fVar52) {
                fVar39 = fVar39 * fVar52;
                fVar45 = fVar45 * fVar52;
              }
              else {
                fVar45 = 0.0;
                fVar39 = 0.0;
              }
            }
            fVar57 = (fVar41 - fVar56) * (fVar41 - fVar56);
            fVar58 = (fVar47 - fVar55) * (fVar47 - fVar55);
            fVar52 = SQRT(fVar58 + fVar57) /
                     (float)((uint)(fVar53 - fVar54) & ___xmm_7fffffff7fffffff7fffffff7fffffff);
            bVar36 = false;
            pfVar19 = pfVar35;
            unaff_R13 = pfVar33;
            if (__real_3dcccccd <= fVar52) {
              bVar21 = puVar22 == (undefined1 *)0x0 & (byte)raw_pair_flags;
              bVar16 = puVar22 == local_218 & (byte)local_2cc;
              if (__real_3f204189 <= fVar52) {
                if (fVar52 < __real_3e800000 || (bVar21 != 0 || bVar16 != 0))
                goto code_r0x0001407fa822;
LAB_1407fa7eb:
                fVar51 = 0.0;
                fVar59 = fVar53;
                fVar38 = fVar54;
              }
              else {
                if ((__real_3e800000 <= fVar52 &&
                    __real_3f733333 <= fVar38 * fVar45 + fVar59 * fVar39) &&
                    (bVar21 == 0 && bVar16 == 0)) goto LAB_1407fa7eb;
code_r0x0001407fa822:
                fVar51 = fVar53;
                if (fVar53 <= fVar54) {
                  fVar51 = fVar54;
                }
                fVar59 = fVar53;
                if (fVar54 <= fVar53) {
                  fVar59 = fVar54;
                }
                fVar59 = (float)(~-(uint)NAN(fVar54) & (uint)fVar59 |
                                -(uint)NAN(fVar54) & (uint)fVar53);
                fVar51 = fVar51 - fVar59;
                if (((bVar21 & fVar53 < fVar54) != 0) ||
                   (fVar38 = fVar59, (bVar16 & fVar54 < fVar53) != 0)) {
                  fVar59 = fVar54 * __real_3f000000 + fVar53 * __real_3f000000;
                  fVar38 = fVar59;
                }
              }
              local_438 = ZEXT416((uint)fVar56);
              fVar56 = fVar56 - fVar41;
              fVar53 = fVar38 - fVar59;
              local_298 = (float *)CONCAT44(local_298._4_4_,fVar55);
              fVar55 = fVar55 - fVar47;
              fVar54 = fVar42 / SQRT(fVar55 * fVar55 + fVar53 * fVar53 + fVar56 * fVar56);
              fVar56 = fVar54 * fVar56;
              fVar53 = fVar54 * fVar53;
              fVar54 = fVar54 * fVar55;
              fVar42 = fVar42 / SQRT(fVar54 * fVar54 + fVar56 * fVar56);
              local_1a8._4_4_ = fVar56 * fVar42;
              local_1a8._0_4_ =
                   (float)((uint)fVar42 ^ ___xmm_80000000800000008000000080000000) * fVar54;
              fStack_2e0 = fVar47;
              ::utils::swizzlers::impl$0::x0y();
              fastrand::Rng::f32();
              fVar55 = (extraout_XMM0_Da_00 - (__real_3f800000 - extraout_XMM0_Da_00)) *
                       __real_3d4ccccd;
              fVar42 = SQRT(fVar58 + fVar57 + (fVar59 - fVar38) * (fVar59 - fVar38));
              if (fVar42 <= __real_3e4ccccd) {
                fVar42 = __real_3e4ccccd;
              }
              fastrand::Rng::f32();
              local_128 = ZEXT416((uint)(fVar51 + fStack_2d4 + fStack_200));
              fVar39 = extraout_XMM0_Da_01 * __real_3d4ccccd + __real_3e8a3d71;
              fVar59 = (fVar38 * __real_3f000000 + fVar59 * __real_3f000000) - fStack_2d8;
              fVar51 = (float)__xmm_00000000000000003f80000000000000;
              fVar47 = __xmm_00000000000000003f80000000000000._4_4_;
              if (local_230 == 0) {
                if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                fVar38 = local_1e0[(longlong)puVar22];
                fVar45 = 0.0;
                if (__real_3dcccccd < fVar38) {
                  fastrand::Rng::f32();
                  fVar57 = extraout_XMM0._0_4_ * __real_3dcccccd;
                  fastrand::Rng::f32();
                  fastrand::Rng::f32();
                  local_348 = (undefined1 *)
                              CONCAT44(extraout_XMM0_Da_03 + _UNK_142a171f4,
                                       extraout_XMM0_Da_02 + ___xmm_0000000000000000bf000000bf000000
                                      );
                  ::utils::swizzlers::impl$0::x0y();
                  fVar47 = local_1a8._4_4_;
                  fVar51 = (float)local_1a8;
                  fVar45 = (float)uStack_1a0;
                  fastrand::Rng::f32();
                  fVar52 = __real_3f800000;
                  if (fVar38 + fVar38 <= __real_3f800000) {
                    fVar52 = fVar38 + fVar38;
                  }
                  fVar59 = fVar59 + fVar57 * fVar52;
                  fVar45 = fVar45 * __real_3e99999a + __real_00000000;
                  fVar51 = fVar51 * (float)__xmm_00000000000000003e99999a3e99999a +
                           (float)__xmm_00000000000000003f80000000000000;
                  fVar47 = fVar47 * __xmm_00000000000000003e99999a3e99999a._4_4_ +
                           __xmm_00000000000000003f80000000000000._4_4_;
                  fVar52 = fVar52 * __real_3e99999a;
                  fVar38 = extraout_XMM0_Da_04;
                  goto LAB_1407fae3c;
                }
                fVar52 = 0.0;
LAB_1407fac6e:
              }
              else {
                if ((local_1f0 == 0) || (local_1f0 == 3)) {
                  if (cStack_f2 == '\0') {
                    fastrand::Rng::f32();
                    if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                      core::panicking::panic_bounds_check();
                    }
                    fVar47 = extraout_XMM0_00._0_4_ * __real_3dcccccd;
                    fVar51 = local_1e0[(longlong)puVar22];
                    fastrand::Rng::f32();
                    fastrand::Rng::f32();
                    local_348 = (undefined1 *)
                                CONCAT44(extraout_XMM0_Da_06 + _UNK_142a171f4,
                                         extraout_XMM0_Da_05 +
                                         ___xmm_0000000000000000bf000000bf000000);
                    ::utils::swizzlers::impl$0::x0y();
                    if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                      core::panicking::panic_bounds_check();
                    }
                    fVar59 = fVar59 + fVar47 * fVar51;
                    fVar38 = local_1e0[(longlong)puVar22];
                    fVar51 = fVar38 * (float)local_1a8 *
                                      (float)__xmm_00000000000000003e99999a3e99999a +
                             (float)__xmm_00000000000000003f80000000000000;
                    fVar47 = fVar38 * local_1a8._4_4_ * __xmm_00000000000000003e99999a3e99999a._4_4_
                             + __xmm_00000000000000003f80000000000000._4_4_;
                    fVar45 = (float)uStack_1a0 * __real_3e99999a * fVar38 + __real_00000000;
                  }
                  else {
                    fVar45 = 0.0;
                  }
                  if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                    core::panicking::panic_bounds_check();
                  }
                  fVar38 = local_1e0[(longlong)puVar22];
                  fastrand::Rng::f32();
                  fVar52 = fVar38 * __real_3e4ccccd * extraout_XMM0_Da_07;
                  goto LAB_1407fac6e;
                }
                if (cStack_f2 == '\0') {
                  fastrand::Rng::f32();
                  if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                    core::panicking::panic_bounds_check();
                  }
                  fVar59 = fVar59 + extraout_XMM0_Da_08 * __real_3dcccccd *
                                    local_1e0[(longlong)puVar22];
                }
                fVar45 = fStack_350;
                fVar38 = fStack_354;
                fVar47 = fStack_358;
                if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                fVar57 = local_1e0[(longlong)puVar22] * __real_3e99999a;
                fastrand::Rng::f32();
                if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                fVar40 = extraout_XMM0_01._0_4_ * __real_3dcccccd;
                fVar58 = local_1e0[(longlong)puVar22];
                fastrand::Rng::f32();
                if ((ulonglong)auStack_1d8._0_8_ <= puVar22) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                fVar52 = local_1e0[(longlong)puVar22];
                fStack_2c4 = extraout_XMM0_Da_09;
                fastrand::Rng::f32();
                fVar51 = (__real_3f800000 - fVar57) * __real_00000000;
                fVar48 = (fVar53 * fVar47 - fVar56 * fVar38) * fVar57 + fVar51;
                fVar51 = fVar51 + fVar57 * (fVar54 * fVar38 - fVar45 * fVar53);
                fVar47 = (__real_3f800000 - fVar57) + fVar57 * (fVar56 * fVar45 - fVar47 * fVar54);
                fVar45 = __real_3f800000 / SQRT(fVar48 * fVar48 + fVar47 * fVar47 + fVar51 * fVar51)
                ;
                fVar51 = fVar45 * fVar51;
                fVar47 = fVar45 * fVar47;
                fVar45 = fVar45 * fVar48;
                fVar59 = fVar59 + fVar40 * fVar58;
                local_128._0_4_ = (float)local_128._0_4_ + fStack_2c4 * ___real_bdf5c28f * fVar52;
                fVar52 = fVar52 * __real_3dcccccd;
                fVar38 = extraout_XMM0_Da_10;
LAB_1407fae3c:
                fVar52 = fVar52 * fVar38;
              }
              fVar52 = __real_3f800000 - fVar52;
              fVar42 = fVar42 * fVar52;
              local_340 = fVar52 * (fVar39 + (float)auStack_428._0_4_);
              fStack_458 = fVar54 * fVar42;
              unaff_XMM12_Dc = 0.0;
              uStack_460 = CONCAT44(fVar53 * fVar42,fVar56 * fVar42);
              fVar55 = fVar55 + (float)local_128._0_4_ * fVar52;
              local_278 = (undefined1 *)CONCAT44(fVar55 * fVar47,fVar55 * fVar51);
              local_270 = (float *)CONCAT44(local_270._4_4_,fVar45 * fVar55);
              local_348 = (undefined1 *)CONCAT44(fStack_354 * local_340,fStack_358 * local_340);
              local_340 = local_340 * fStack_350;
              local_1a8 = (undefined1 *)
                          CONCAT44(fVar59,((float)local_438._0_4_ * __real_3f000000 +
                                          fVar41 * __real_3f000000) - fStack_2d0);
              uStack_1a0 = (float *)CONCAT44(uStack_1a0._4_4_,
                                             (local_298._0_4_ * __real_3f000000 +
                                             fStack_2e0 * __real_3f000000) - fStack_2d0);
              country_core::resources::render::Affine3Packed::from_cols();
              country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
              lVar24 = *(longlong *)raw_rng;
              uVar18 = lVar24 + 0xa0761d6478bd642f;
              *(ulonglong *)raw_rng = uVar18;
              auVar3._8_8_ = 0;
              auVar3._0_8_ = uVar18 ^ 0xe7037ed1a0b428db;
              auVar7._8_8_ = 0;
              auVar7._0_8_ = uVar18;
              uVar30 = SUB168(auVar3 * auVar7,8) ^ SUB168(auVar3 * auVar7,0);
              if ((uint)((int)uVar30 * 1000) < 0x128) {
                uVar28 = lVar24 + 0x40ec3ac8f17ac85e;
                do {
                  uVar18 = uVar18 + 0xa0761d6478bd642f;
                  auVar4._8_8_ = 0;
                  auVar4._0_8_ = uVar28 ^ 0xe7037ed1a0b428db;
                  auVar8._8_8_ = 0;
                  auVar8._0_8_ = uVar28;
                  uVar30 = SUB168(auVar4 * auVar8,8) ^ SUB168(auVar4 * auVar8,0);
                  uVar28 = uVar28 + 0xa0761d6478bd642f;
                } while ((uint)((int)uVar30 * 1000) < 0x128);
                *(ulonglong *)raw_rng = uVar18;
              }
              country_core::geometry::instanced_wall::impl$3::default();
              local_328 = puStack_4c8;
              ppuStack_320 = ppuStack_4c0;
              fStack_338 = local_4d8._0_4_;
              local_334 = local_4d8._4_4_;
              local_330 = local_4d8._8_8_;
              local_348 = puStack_4e8;
              local_340 = (float)uStack_4e0;
              local_33c = (float)((ulonglong)uStack_4e0 >> 0x20);
              uStack_318 = local_178;
              uStack_310 = uStack_170;
              uStack_300 = local_160;
              uStack_308 = local_168;
              uStack_2fc = (undefined4)((uVar30 & 0xffffffff) * 1000 >> 0x20);
              uStack_2f4 = CONCAT44(__xmm_00000000000000003f80000000000000._4_4_,
                                    (float)__xmm_00000000000000003f80000000000000);
              uStack_2ec = local_2c8;
              country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
              push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
              bVar36 = false;
              fVar42 = __real_3f800000;
              fVar51 = fStack_2dc;
            }
          }
          if (puStack_3c0 != (undefined1 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_1e8 != (undefined1 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          if (local_410 != (undefined1 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
        }
        if ((int)local_230 == 3) {
          if (puVar17 == (undefined1 *)0x1) {
            uStack_f1 = 1;
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          if (puVar17 == (undefined1 *)0x0) {
            uStack_f1 = 1;
                    /* WARNING: Subroutine does not return */
            core::panicking::panic_bounds_check();
          }
          uStack_1b8 = uStack_378;
          auStack_1d8._8_8_ = lStack_390;
          auStack_1d8._0_8_ = lStack_398;
          uStack_1c8 = uStack_388;
          uStack_1c0 = uStack_380;
          local_1e8 = local_3a8;
          local_1e0 = local_3a0;
          if (((ulonglong)raw_pair_flags & 1) != 0) {
            if (lStack_398 == 1) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            if (lStack_398 == 0) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            fVar42 = local_3a0[2] * __real_3f19999a;
            fVar51 = local_3a0[5] * __real_3ecccccd;
            *(ulonglong *)local_3a0 =
                 CONCAT44((float)((ulonglong)*(undefined8 *)(local_3a0 + 3) >> 0x20) *
                          _UNK_1429cd2c4 +
                          (float)((ulonglong)*(undefined8 *)local_3a0 >> 0x20) *
                          __xmm_00000000000000003f19999a3f19999a._4_4_,
                          (float)*(undefined8 *)(local_3a0 + 3) *
                          ___xmm_00000000000000003ecccccd3ecccccd +
                          (float)*(undefined8 *)local_3a0 *
                          (float)__xmm_00000000000000003f19999a3f19999a);
            local_3a0[2] = fVar51 + fVar42;
          }
          if (((ulonglong)raw_pair_flags & 0x100) != 0) {
            lVar24 = lStack_398 + -1;
            if (lStack_398 == 0) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            if (lStack_398 == 1) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic_bounds_check();
            }
            fVar42 = local_3a0[lVar24 * 3 + 2] * __real_3f19999a;
            fVar51 = local_3a0[(lStack_398 + -2) * 3 + 2] * __real_3ecccccd;
            *(ulonglong *)(local_3a0 + lVar24 * 3) =
                 CONCAT44((float)((ulonglong)*(undefined8 *)(local_3a0 + (lStack_398 + -2) * 3) >>
                                 0x20) * _UNK_1429cd2c4 +
                          (float)((ulonglong)*(undefined8 *)(local_3a0 + lVar24 * 3) >> 0x20) *
                          __xmm_00000000000000003f19999a3f19999a._4_4_,
                          (float)*(undefined8 *)(local_3a0 + (lStack_398 + -2) * 3) *
                          ___xmm_00000000000000003ecccccd3ecccccd +
                          (float)*(undefined8 *)(local_3a0 + lVar24 * 3) *
                          (float)__xmm_00000000000000003f19999a3f19999a);
            local_3a0[lVar24 * 3 + 2] = fVar51 + fVar42;
          }
          ::utils::curve::Curve::
          try_resample<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
          if (!SBORROW8(0,(longlong)local_1a8)) {
            uStack_248 = local_178;
            puStack_258 = local_188;
            ppuStack_250 = uStack_180;
            local_268 = local_198;
            auVar5 = local_268;
            local_278 = local_1a8;
            local_270 = uStack_1a0;
            local_268._0_8_ = local_198._0_8_;
            local_218 = (undefined1 *)
                        CONCAT71(local_218._1_7_,2 < (ulonglong)local_268._0_8_ || local_1f0 != 3);
            ppuVar20 = (undefined1 **)&local_1e8;
            if (2 < (ulonglong)local_268._0_8_ || local_1f0 != 3) {
              ppuVar20 = &local_278;
            }
            local_100 = (float *)ppuVar20[2];
            local_268 = auVar5;
            if (local_100 == (float *)0x0) {
              local_138 = (float *)&DAT_00000004;
            }
            else {
              std::alloc::__default_lib_allocator::__rust_alloc_zeroed();
              local_138 = extraout_RAX_01;
              if (extraout_RAX_01 == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
                alloc::raw_vec::handle_error();
              }
            }
            _Size = local_220;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (_Dst == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
              alloc::raw_vec::handle_error();
            }
            memcpy(_Dst,local_140,_Size);
            local_198._0_8_ = puVar17;
            local_1a8 = puVar17;
            uStack_1a0 = _Dst;
            ::utils::curve::Curve::
            try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
            if (!SBORROW8(0,(longlong)local_348)) {
              uStack_1a0 = (float *)CONCAT44(local_33c,local_340);
              local_198._4_4_ = local_334;
              local_198._0_4_ = fStack_338;
              local_198._8_8_ = local_330;
              local_128._0_8_ = local_270;
              pfVar19 = local_270;
              if ((char)local_218 == '\0') {
                pfVar19 = local_1e0;
              }
              pfVar25 = pfVar19 + (longlong)ppuVar20[2] * 3;
              pfVar35 = (float *)0x0;
              local_178 = uStack_318;
              local_188 = local_328;
              uStack_180 = ppuStack_320;
              local_1a8 = local_348;
              for (; pfVar19 != pfVar25; pfVar19 = pfVar19 + 3) {
                ::utils::curve::Curve::
                get_approx_u_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>();
                ::utils::curve::Curve::
                get_coord_at_u<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,utils::curve::curve_u::CurveU<utils::curve::CurveCoordSemantic3D>_>
                          ();
                puVar17 = (undefined1 *)(ulonglong)extraout_EAX;
                if (local_108 <= puVar17) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (local_108 <= puVar17 + 1) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                if (local_100 <= pfVar35) {
                    /* WARNING: Subroutine does not return */
                  core::panicking::panic_bounds_check();
                }
                fVar42 = *(float *)((longlong)raw_samples_ptr + (longlong)puVar17 * 4);
                local_138[(longlong)pfVar35] =
                     (*(float *)((longlong)raw_samples_ptr + (longlong)puVar17 * 4 + 4) - fVar42) *
                     extraout_XMM0_Da_11 + fVar42;
                pfVar35 = (float *)((longlong)pfVar35 + 1);
              }
              if (local_1a8 != (undefined1 *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              puVar17 = local_108;
              puVar32 = local_280;
              if (local_198._8_8_ != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
            }
            construct_wooden_railing();
            if (local_100 != (float *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_278 != (undefined1 *)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (local_268._8_8_ != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
          if (local_1e8 != (undefined1 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          lVar24 = auStack_1d8._8_8_;
        }
        else {
          lVar24 = lStack_390;
          if (local_3a8 != (undefined1 *)0x0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
            lVar24 = lStack_390;
          }
        }
        if (lVar24 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
    }
    pfVar19 = (float *)(ulonglong)uStack_10c;
  }
  if ((local_440 & 0xfffffffffffffffd) == 0 || (byte)raw_flag11 < 2) {
    local_498 = (undefined1 *)0x0;
    local_410 = local_3b0;
    pfVar25 = (float *)0x0;
    local_438._0_4_ = (ushort)raw_pair_flags >> 8 & 1;
    ppuVar20 = &local_498;
    fVar42 = __real_be8a3d71;
    do {
      while( true ) {
        puVar17 = *ppuVar20;
        if (local_228 <= puVar17) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fVar51 = (float)local_280[(longlong)puVar17] + ___real_be89374c + fVar42;
        if (fVar51 <= __real_00000000) {
          fVar51 = __real_00000000;
        }
        fVar51 = fVar51 * __real_3f000000;
        local_240 = pfVar25;
        if (local_288 <= puVar17) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        local_1a8 = (undefined1 *)
                    CONCAT44(*(undefined4 *)(local_238 + (longlong)puVar17),
                             *(uint *)((longlong)local_238 + (longlong)puVar17 * 8 + 4) ^
                             ___xmm_80000000800000008000000080000000);
        ::utils::swizzlers::impl$0::x0y();
        ::utils::swizzlers::impl$0::x0y();
        if (fVar51 < __real_3d4ccccd) break;
        if (local_108 <= puVar17) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fVar42 = *(float *)(&__real_3f800000bf800000 + (ulonglong)(puVar17 == (undefined1 *)0x0) * 4
                           );
        fVar56 = *(float *)((longlong)local_140 + (longlong)puVar17 * 0xc + 4);
        fVar47 = local_3e4 * ___real_3d8f5c2a * fVar42;
        fVar59 = fVar47 + (fVar56 - fVar51 * local_3d8);
        fVar47 = fVar47 + fVar56 + fVar51 * local_3d8;
        fVar56 = *(float *)((longlong)local_140 + (longlong)puVar17 * 0xc);
        fVar55 = *(float *)((longlong)local_140 + (longlong)puVar17 * 0xc + 8);
        fVar41 = fVar42 * local_3e0 * ___xmm_00000000000000003d8f5c2a3d8f5c2a;
        fVar42 = fVar42 * local_3e8 * _UNK_142a8c5c4;
        fVar53 = fVar55 + local_3d4 * fVar51 + fVar41;
        fVar54 = fVar56 + local_3dc * fVar51 + fVar42;
        fVar41 = (fVar55 - local_3d4 * fVar51) + fVar41;
        fVar42 = (fVar56 - local_3dc * fVar51) + fVar42;
        fVar56 = SQRT((fVar47 - fVar59) * (fVar47 - fVar59) + (fVar54 - fVar42) * (fVar54 - fVar42)
                      + (fVar53 - fVar41) * (fVar53 - fVar41));
        ::utils::random_splits();
        ::utils::xz_rotation_mat3_bad();
        local_100 = (float *)CONCAT44(local_100._4_4_,__real_3ea3d70a);
        bVar36 = (puVar17 == local_3b0 & local_438[0]) == 0;
        bVar11 = (puVar17 == (undefined1 *)0x0 & (byte)raw_pair_flags) == 0;
        fVar51 = __real_3e3851ec;
        if (bVar36 && bVar11) {
          local_100 = (float *)CONCAT44(local_100._4_4_,__real_3e6147ae);
          if (!bVar36 || !bVar11) {
LAB_1407fba6c:
            fVar51 = __real_3e851eb8;
          }
        }
        else if (!bVar36 || !bVar11) goto LAB_1407fba6c;
        fVar55 = __real_3f28f5c3;
        if (bVar36 && bVar11) {
          fVar55 = __real_3ef5c290;
        }
        fVar38 = ((float)(longlong)puVar17 / local_110) * local_118 +
                 (__real_3f800000 - (float)(longlong)puVar17 / local_110) * local_114;
        pfVar25 = local_270 + local_268._0_8_;
        uStack_130 = 0;
        fVar39 = fVar38 * __real_00000000;
        local_218 = local_348;
        uStack_210 = 0;
        local_128._0_4_ = local_340;
        fVar57 = fVar55 * local_33c;
        fVar58 = fVar55 * fStack_338;
        fVar55 = fVar55 * local_334;
        fVar45 = (float)local_330;
        uVar18 = (ulonglong)local_330 >> 0x20;
        fVar52 = fVar51 * local_328._0_4_;
        bVar36 = true;
        local_368 = fVar56;
        fStack_364 = fVar56;
        pfVar35 = (float *)CONCAT44(fVar38,fVar38);
        fStack_360 = unaff_XMM12_Dc;
        fStack_35c = unaff_XMM12_Dc;
        local_298 = local_270;
        pfVar33 = local_270;
        while (local_138 = pfVar35, pfVar33 != pfVar25) {
          pfVar1 = pfVar33 + 1;
          pfVar34 = pfVar1;
          if (bVar36) {
            if (pfVar1 == pfVar25) break;
            pfVar19 = pfVar33;
            pfVar34 = pfVar33 + 2;
            pfVar33 = pfVar1;
          }
          fVar56 = *pfVar19;
          fVar48 = __real_3f800000 - fVar56;
          fVar38 = *pfVar33;
          fVar46 = __real_3f800000 - fVar38;
          local_138._0_4_ = SUB84(pfVar35,0);
          local_138._4_4_ = (float)((ulonglong)pfVar35 >> 0x20);
          uStack_130._4_4_ = (float)((ulonglong)uStack_130 >> 0x20);
          fVar43 = fVar38 * fVar59 + fVar47 * fVar46 + (float)local_138;
          fVar44 = fVar48 * fVar47 + fVar59 * fVar56 + local_138._4_4_;
          unaff_XMM12_Dc = uStack_130._4_4_ + 0.0;
          fVar50 = fVar44 - fVar43;
          fVar40 = fVar56 * fVar41 + fVar48 * fVar53 + fVar39;
          fVar56 = fVar56 * fVar42 + fVar48 * fVar54 + fVar39;
          fVar48 = fVar38 * fVar41 + fVar46 * fVar53 + fVar39;
          fVar38 = fVar38 * fVar42 + fVar46 * fVar54 + fVar39;
          fVar46 = fVar40 - fVar48;
          fVar49 = fVar56 - fVar38;
          local_3f4 = fVar38 * __real_3f000000 + fVar56 * __real_3f000000;
          local_3ec = fVar48 * __real_3f000000 + fVar40 * __real_3f000000;
          local_3f0 = (fVar43 * __real_3f000000 + fVar44 * __real_3f000000) - local_100._0_4_;
          fVar56 = SQRT(fVar49 * fVar49 + fVar50 * fVar50 + fVar46 * fVar46);
          local_218._4_4_ = (float)((ulonglong)local_218 >> 0x20);
          local_3a8 = (undefined1 *)CONCAT44(fVar56 * local_218._4_4_,fVar56 * (float)local_218);
          uVar30 = (ulonglong)local_3a0 >> 0x20;
          local_3a0 = (float *)CONCAT44((int)uVar30,(float)local_128._0_4_ * fVar56);
          local_1e8 = (undefined1 *)CONCAT44(fVar58,fVar57);
          uVar30 = (ulonglong)local_1e0 >> 0x20;
          local_1e0 = (float *)CONCAT44((int)uVar30,fVar55);
          local_1a8 = (undefined1 *)CONCAT44(fVar51 * (float)uVar18,fVar51 * fVar45);
          uStack_1a0._4_4_ = (undefined4)((ulonglong)uStack_1a0 >> 0x20);
          uStack_1a0 = (float *)CONCAT44(uStack_1a0._4_4_,fVar52);
          country_core::resources::render::Affine3Packed::from_cols();
          country_core::geometry::instanced_wall::BrickSourceId::from_wall_id();
          lVar24 = *(longlong *)raw_rng;
          uVar30 = lVar24 + 0xa0761d6478bd642f;
          *(ulonglong *)raw_rng = uVar30;
          auVar5._8_8_ = 0;
          auVar5._0_8_ = uVar30 ^ 0xe7037ed1a0b428db;
          auVar9._8_8_ = 0;
          auVar9._0_8_ = uVar30;
          uVar28 = SUB168(auVar5 * auVar9,8) ^ SUB168(auVar5 * auVar9,0);
          if ((uint)((int)uVar28 * 1000) < 0x128) {
            uVar31 = lVar24 + 0x40ec3ac8f17ac85e;
            do {
              uVar30 = uVar30 + 0xa0761d6478bd642f;
              auVar6._8_8_ = 0;
              auVar6._0_8_ = uVar31 ^ 0xe7037ed1a0b428db;
              auVar10._8_8_ = 0;
              auVar10._0_8_ = uVar31;
              uVar28 = SUB168(auVar6 * auVar10,8) ^ SUB168(auVar6 * auVar10,0);
              uVar31 = uVar31 + 0xa0761d6478bd642f;
            } while ((uint)((int)uVar28 * 1000) < 0x128);
            *(ulonglong *)raw_rng = uVar30;
          }
          local_154 = CONCAT44(*pfVar33 * fStack_364,*pfVar19 * local_368);
          local_178 = 0;
          uStack_170 = 0;
          local_188 = (undefined1 *)CONCAT44(uStack_4f4,uStack_4f8);
          uStack_180 = (undefined1 **)CONCAT44(uStack_4ec,uStack_4f0);
          local_198._8_8_ = uStack_500;
          local_198._0_8_ = uStack_508;
          local_1a8 = local_518;
          uStack_1a0 = pfStack_510;
          local_168 = __xmm_0000000000000000c2c80000c2c80000;
          local_160 = 0xc2c80000;
          local_15c = (undefined4)((uVar28 & 0xffffffff) * 1000 >> 0x20);
          local_14c = 0x40;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
          push<country_core::geometry::instanced_wall::BrickTransformSsbo>();
          bVar36 = false;
          pfVar19 = pfVar33;
          pfVar35 = local_138;
          pfVar33 = pfVar34;
        }
        if (local_278 != (undefined1 *)0x0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        ppuVar20 = &local_410;
        pfVar25 = (float *)CONCAT71((int7)((ulonglong)local_240 >> 8),1);
        fVar42 = __real_be8a3d71;
        if (((ulonglong)local_240 & 1) != 0) {
          return;
        }
      }
      ppuVar20 = &local_410;
      pfVar25 = (float *)CONCAT71((int7)((ulonglong)local_240 >> 8),1);
    } while (((ulonglong)local_240 & 1) == 0);
  }
override_jmp_1407f9bec_case_2:
  return;
}

