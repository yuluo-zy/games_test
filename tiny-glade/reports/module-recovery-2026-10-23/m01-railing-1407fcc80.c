
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::railings_tips
          (void *p1,void *p2,void *p3,void *p4,uint p5,uint p6,uchar p7,void *p8,float *p9,
          ulonglong p10,void *p11,ulonglong p12,void *p13,ulonglong p14,void *p15,void *p16,
          void *p17,ushort p18)

{
  float *pfVar1;
  uint second;
  undefined1 auVar2 [16];
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  float *pfVar6;
  undefined4 uVar7;
  float *pfVar8;
  float *extraout_RAX;
  float *extraout_RAX_00;
  float *pfVar9;
  bool bVar10;
  longlong lVar11;
  ulonglong uVar12;
  uint uVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  float *pfVar16;
  float *pfVar17;
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
  float fVar32;
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  float fVar37;
  longlong local_348;
  float *pfStack_340;
  longlong local_338;
  longlong lStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  float local_318;
  undefined4 uStack_314;
  undefined4 uStack_310;
  undefined4 uStack_30c;
  ushort *local_308;
  void *local_300;
  void *local_2f8;
  void *local_2f0;
  void *local_2e8;
  ulonglong local_2e0;
  void *local_2d8;
  ulonglong local_2d0;
  float *local_2c8;
  ulonglong local_2c0;
  void *local_2b8;
  char *local_2b0;
  ulonglong local_2a8;
  undefined4 local_2a0;
  undefined4 uStack_29c;
  ulonglong local_298;
  longlong local_290;
  ulonglong local_270;
  undefined4 local_268;
  undefined8 local_264;
  float local_25c;
  undefined8 local_258;
  float local_250;
  longlong local_248;
  float *pfStack_240;
  longlong local_238;
  longlong lStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 local_210;
  float local_208;
  undefined8 local_200;
  float local_1f8;
  ulonglong local_1f0;
  float local_1e8;
  ulonglong local_1e0;
  undefined1 local_1d8 [16];
  undefined8 local_1c8;
  float fStack_1c0;
  float fStack_1bc;
  undefined8 local_1b8;
  float *pfStack_1b0;
  longlong local_1a8;
  longlong lStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined4 local_170;
  undefined4 local_16c;
  uint local_168;
  undefined8 local_164;
  undefined4 local_15c;
  float *local_150;
  ulonglong local_148;
  longlong local_140;
  undefined1 local_138 [16];
  uint local_11c;
  undefined8 local_118;
  undefined8 uStack_110;
  float *local_100;
  uint local_f8;
  char local_f2 [178];
  
  local_f2[2] = -2;
  local_f2[3] = -1;
  local_f2[4] = -1;
  local_f2[5] = -1;
  local_f2[6] = -1;
  local_f2[7] = -1;
  local_f2[8] = -1;
  local_f2[9] = -1;
  local_308 = &p18;
  local_2f0 = p16;
  local_f2[0] = '\0';
  local_f2[1] = '\0';
  local_2e8 = p13;
  local_2e0 = p14;
  local_2d8 = p11;
  local_2d0 = p12;
  local_2c8 = p9;
  local_2c0 = p10;
  local_2b8 = p15;
  local_2b0 = local_f2;
  uVar13 = *(uint *)p3;
  second = *(uint *)((longlong)p3 + 4);
  local_300 = p2;
  local_2f8 = p1;
  railings_tips::tip_endpoint((ulonglong *)&local_308,uVar13,second,0,0);
  local_1e0 = p14 - 1;
  railings_tips::tip_endpoint((ulonglong *)&local_308,second,uVar13,local_1e0,1);
  local_11c = (uint)p7;
  lVar11 = 0;
  pfVar17 = (float *)p14;
  do {
    if (local_f2[lVar11] == '\x01') {
      uVar14 = local_1e0;
      if (lVar11 == 0) {
        uVar14 = 0;
      }
      if (p14 <= uVar14) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      local_140 = lVar11;
      if (p12 <= uVar14) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      fVar28 = *(float *)(&__real_3f800000bf800000 + (ulonglong)(lVar11 == 0) * 4);
      fVar29 = *(float *)((longlong)p13 + uVar14 * 0xc);
      fVar26 = *(float *)((longlong)p13 + uVar14 * 0xc + 4);
      fVar27 = *(float *)((longlong)p13 + uVar14 * 0xc + 8);
      ::utils::swizzlers::impl$0::x0y(&local_1b8,(float *)((longlong)p11 + uVar14 * 8));
      fVar32 = fVar28 * local_1b8._4_4_ * ___real_3dc7ae14;
      fVar33 = pfStack_1b0._0_4_ * fVar28 * ___xmm_00000000000000003dc7ae143dc7ae14;
      fVar35 = (float)local_1b8 * fVar28 * _UNK_142a8c644;
      fVar28 = *(float *)((longlong)p11 + uVar14 * 8);
      fVar19 = *(float *)((longlong)p11 + uVar14 * 8 + 4);
      fVar30 = __real_3f800000 / SQRT(fVar28 * fVar28 + fVar19 * fVar19);
      local_1b8 = CONCAT44(fVar28 * fVar30,
                           (float)((uint)fVar19 ^ ___xmm_80000000800000008000000080000000) * fVar30)
      ;
      ::utils::swizzlers::impl$0::x0y(&local_1f0,(float *)&local_1b8);
      if (p10 <= uVar14) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      fVar26 = fVar26 + fVar32;
      fVar27 = fVar27 + fVar33;
      fVar29 = fVar29 + fVar35;
      fVar31 = p9[uVar14] + __real_bf000000;
      fVar32 = (float)(local_1f0 >> 0x20) * fVar31 * __real_3f000000;
      fVar37 = fVar26 + fVar32;
      fVar32 = fVar26 - fVar32;
      local_138 = ZEXT416((uint)local_1e8);
      local_118 = local_1f0;
      uStack_110 = 0;
      fVar35 = local_1e8 * fVar31 * (float)__xmm_00000000000000003f0000003f000000;
      fVar20 = (float)local_1f0 * fVar31 * __xmm_00000000000000003f0000003f000000._4_4_;
      fVar34 = fVar27 + fVar35;
      fVar36 = fVar29 + fVar20;
      fVar35 = fVar27 - fVar35;
      fVar20 = fVar29 - fVar20;
      fVar33 = SQRT((fVar37 - fVar32) * (fVar37 - fVar32) + (fVar36 - fVar20) * (fVar36 - fVar20) +
                    (fVar34 - fVar35) * (fVar34 - fVar35));
      local_1d8 = ZEXT416((uint)fVar33);
      fVar33 = fVar33 / __real_3eeb851f;
      uVar14 = 0;
      local_148 = 0;
      if (__real_00000000 <= fVar33) {
        local_148 = (longlong)(fVar33 - __real_5f000000) & (longlong)fVar33 >> 0x3f |
                    (longlong)fVar33;
      }
      if (__real_5f7fffff < fVar33) {
        local_148 = 0xffffffffffffffff;
      }
      local_148 = local_148 + (local_148 == 0);
      ::utils::xz_rotation_mat3_bad(&local_270,fVar28 * fVar30,fVar19 * fVar30);
      local_f8 = 1;
      if (1 < p7) {
        if (local_11c != 2) {
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (extraout_RAX == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
            alloc::alloc::handle_alloc_error();
          }
          fVar31 = fVar31 + __real_3e428f5c;
          fVar30 = (float)local_138._0_4_ * fVar31 * __real_3f000000;
          fVar28 = (float)local_118 * fVar31 * (float)__xmm_00000000000000003f0000003f000000;
          fVar19 = local_118._4_4_ * fVar31 * __xmm_00000000000000003f0000003f000000._4_4_;
          *(ulonglong *)extraout_RAX = CONCAT44(fVar26 + fVar19,fVar29 + fVar28);
          extraout_RAX[2] = fVar30 + fVar27;
          *(ulonglong *)(extraout_RAX + 3) = CONCAT44(fVar26 - fVar19,fVar29 - fVar28);
          extraout_RAX[5] = fVar27 - fVar30;
          local_1b8 = 2;
          local_1a8 = 2;
          pfStack_1b0 = extraout_RAX;
          ::utils::curve::Curve::curve_new(&local_248,&local_1b8,'\0');
          lVar11 = local_140;
          if (!SBORROW8(0,local_248)) {
            local_188 = local_218;
            local_198 = local_228;
            uStack_190 = uStack_220;
            local_1a8 = local_238;
            lStack_1a0 = lStack_230;
            local_1b8 = local_248;
            pfStack_1b0 = pfStack_240;
            ::utils::curve::Curve::curve_resample(&local_2a8,&local_1b8,__real_3fa89375,'\x01');
            if (local_1b8 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            if (lStack_1a0 != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            uVar12 = local_298;
            uVar14 = local_2a8;
            lVar11 = local_140;
            if (!SBORROW8(0,local_2a8)) {
              pfVar6 = (float *)CONCAT44(uStack_29c,local_2a0);
              if (local_290 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              local_118 = uVar14;
              local_138._0_8_ = pfVar6;
              if (uVar12 == 0) {
                pfVar9 = (float *)&DAT_00000004;
              }
              else {
                std::alloc::__default_lib_allocator::__rust_alloc_zeroed();
                pfVar9 = extraout_RAX_00;
                if (extraout_RAX_00 == (float *)0x0) {
                    /* WARNING: Subroutine does not return */
                  alloc::raw_vec::handle_error();
                }
              }
              local_1c8 = uVar12;
              local_100 = pfVar9;
              construct_wooden_railing(p4,p6,p8,0.0,pfVar6,uVar12,pfVar9,uVar12);
              if (local_1c8 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              lVar11 = local_140;
              if (local_118 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                lVar11 = local_140;
              }
            }
          }
          goto LAB_1407fcde7;
        }
        local_f8 = 2;
      }
      lVar11 = local_140;
      if (local_f8 != 0) {
        local_148 = local_148 + 1;
        uVar7 = local_1d8._0_4_;
        local_318 = (float)local_1d8._0_4_ * __real_3e99999a;
        if (__real_3e4ccccd <= (float)local_1d8._0_4_ * __real_3e99999a) {
          local_318 = __real_3e4ccccd;
        }
        local_318 = local_318 / (float)local_1d8._0_4_;
        uStack_314 = local_1d8._4_4_;
        uStack_310 = local_1d8._8_4_;
        uStack_30c = local_1d8._12_4_;
        local_118 = local_270;
        uStack_110 = 0;
        local_138._0_4_ = local_268;
        fStack_1c0 = _UNK_142a8c668 * 0.0;
        fStack_1bc = _UNK_142a8c66c * 0.0;
        local_1c8 = CONCAT44((float)((ulonglong)local_264 >> 0x20) * _UNK_142a8c664,
                             (float)local_264 * ___xmm_00000000000000003eeb851f3eeb851f);
        local_100 = (float *)CONCAT44(local_100._4_4_,local_25c * __real_3eeb851f);
        fVar28 = (float)local_258 * ___xmm_00000000000000003e828f5c3e828f5c;
        fVar29 = (float)((ulonglong)local_258 >> 0x20) * _UNK_142a8c674;
        fVar26 = local_250 * ___real_3e828f5c;
        do {
          ::utils::random_splits(&local_248,local_148,local_318,p4);
          local_1d8._0_8_ = uVar14;
          fVar27 = (float)(int)uVar14 + __real_3f800000;
          fVar19 = fVar27 * ___real_3ea66666;
          local_150 = pfStack_240;
          if (local_238 != 0) {
            pfVar6 = pfStack_240 + local_238;
            fVar27 = fVar27 * __real_00000000;
            bVar10 = true;
            pfVar9 = pfStack_240;
            pfVar16 = pfVar17;
            do {
              pfVar1 = pfVar9 + 1;
              pfVar8 = pfVar1;
              pfVar17 = pfVar9;
              if (bVar10) {
                pfVar17 = pfVar16;
                if (pfVar1 == pfVar6) break;
                pfVar8 = pfVar9 + 2;
                pfVar16 = pfVar9;
                pfVar17 = pfVar1;
              }
              pfVar9 = pfVar8;
              fVar30 = *pfVar16;
              fVar33 = *pfVar17;
              fVar22 = (float)__xmm_00000000000000003f8000003f800000 - fVar30;
              fVar23 = __xmm_00000000000000003f8000003f800000._4_4_ - fVar33;
              fVar31 = fVar30 * fVar32 + fVar37 * fVar22 + fVar19;
              fVar18 = fVar33 * fVar32 + fVar37 * fVar23 + fVar19;
              fVar21 = fVar22 * fVar34 + fVar30 * fVar35 + fVar27;
              fVar22 = fVar22 * fVar36 + fVar30 * fVar20 + fVar27;
              fVar30 = fVar23 * fVar34 + fVar33 * fVar35 + fVar27;
              fVar33 = fVar23 * fVar36 + fVar33 * fVar20 + fVar27;
              fVar23 = fVar21 - fVar30;
              fVar24 = fVar22 - fVar33;
              fVar25 = fVar31 - fVar18;
              local_1f8 = fVar30 * __real_3f000000 + fVar21 * __real_3f000000;
              local_200 = CONCAT44(fVar18 * __xmm_00000000000000003f0000003f000000._4_4_ +
                                   fVar31 * (float)__xmm_00000000000000003f0000003f000000 +
                                   _UNK_142a8c694,
                                   fVar22 * __real_3f000000 + fVar33 * __real_3f000000);
              fVar30 = SQRT(fVar24 * fVar24 + fVar25 * fVar25 + fVar23 * fVar23);
              local_208 = (float)local_138._0_4_ * fVar30;
              local_210 = CONCAT44(fVar30 * local_118._4_4_,fVar30 * (float)local_118);
              local_2a8 = local_1c8;
              local_2a0 = local_100._0_4_;
              local_1b8 = CONCAT44(fVar29,fVar28);
              pfStack_1b0 = (float *)CONCAT44(pfStack_1b0._4_4_,fVar26);
              country_core::resources::render::Affine3Packed::from_cols
                        (&local_348,(float *)&local_210,(float *)&local_2a8,(float *)&local_1b8,
                         (float *)&local_200);
              local_168 = country_core::geometry::instanced_wall::BrickSourceId::from_wall_id(p5);
              lVar11 = *(longlong *)p4;
              uVar14 = lVar11 + 0xa0761d6478bd642f;
              *(ulonglong *)p4 = uVar14;
              auVar2._8_8_ = 0;
              auVar2._0_8_ = uVar14 ^ 0xe7037ed1a0b428db;
              auVar4._8_8_ = 0;
              auVar4._0_8_ = uVar14;
              uVar12 = SUB168(auVar2 * auVar4,8) ^ SUB168(auVar2 * auVar4,0);
              if ((uint)((int)uVar12 * 1000) < 0x128) {
                uVar15 = lVar11 + 0x40ec3ac8f17ac85e;
                do {
                  uVar14 = uVar14 + 0xa0761d6478bd642f;
                  auVar3._8_8_ = 0;
                  auVar3._0_8_ = uVar15 ^ 0xe7037ed1a0b428db;
                  auVar5._8_8_ = 0;
                  auVar5._0_8_ = uVar15;
                  uVar12 = SUB168(auVar3 * auVar5,8) ^ SUB168(auVar3 * auVar5,0);
                  uVar15 = uVar15 + 0xa0761d6478bd642f;
                } while ((uint)((int)uVar12 * 1000) < 0x128);
                *(ulonglong *)p4 = uVar14;
              }
              local_188 = 0;
              uStack_180 = 0;
              local_198 = local_328;
              uStack_190 = uStack_320;
              local_1a8 = local_338;
              lStack_1a0 = lStack_330;
              local_1b8 = local_348;
              pfStack_1b0 = pfStack_340;
              local_178 = __xmm_0000000000000000c2c80000c2c80000;
              local_170 = 0xc2c80000;
              local_16c = (undefined4)((uVar12 & 0xffffffff) * 1000 >> 0x20);
              local_164 = CONCAT44(*pfVar17 * (float)uVar7,*pfVar16 * (float)uVar7);
              local_15c = 0x40;
              country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push_brick
                        (p8,&local_1b8);
              bVar10 = false;
              pfVar16 = pfVar17;
            } while (pfVar9 != pfVar6);
          }
          if (local_248 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          uVar13 = local_1d8._0_4_ + 1;
          uVar14 = (ulonglong)uVar13;
          lVar11 = local_140;
        } while (uVar13 != local_f8);
      }
    }
LAB_1407fcde7:
    lVar11 = lVar11 + 1;
    if (lVar11 == 2) {
      return;
    }
  } while( true );
}

