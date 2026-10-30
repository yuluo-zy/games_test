
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
system_decorator::decorator_visual::stairs_railings_assemble::construct_wooden_railing
          (void *p1,uint p2,void *p3,float p4,float *p5,ulonglong p6,float *p7,ulonglong p8)

{
  float fVar1;
  longlong lVar2;
  undefined1 auVar3 [16];
  undefined1 auVar4 [16];
  undefined1 auVar5 [16];
  undefined1 auVar6 [16];
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  bool bVar15;
  float *pfVar16;
  ulonglong uVar17;
  float *pfVar18;
  ulonglong uVar19;
  ulonglong uVar20;
  float *pfVar21;
  ulonglong uVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  uint uVar29;
  uint uVar30;
  float fVar31;
  float fVar32;
  float fVar33;
  float fVar34;
  float unaff_XMM13_Da;
  float unaff_XMM13_Db;
  float fVar35;
  float fVar36;
  float fVar37;
  float local_270;
  float fStack_26c;
  float local_268;
  undefined8 local_260;
  float local_258;
  float *local_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined4 local_218;
  undefined4 local_214;
  uint local_210;
  undefined4 local_20c;
  float *local_208;
  undefined8 local_200;
  float local_1f8;
  float local_1f4;
  float local_1f0;
  float local_1ec;
  float local_1e8;
  float local_1e4;
  float local_1e0;
  float local_1dc;
  undefined1 local_1d8 [8];
  float local_1d0;
  undefined1 local_1c8 [16];
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined1 local_198 [16];
  undefined1 local_188 [16];
  undefined8 local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  
  if (p6 != 0) {
    local_208 = p5 + p6 * 3;
    local_250 = (float *)((longlong)p3 + 0x58);
    pfVar21 = p5;
    uVar17 = 0;
    local_1f4 = p4;
    do {
      fVar35 = pfVar21[1] + __real_3d8f5c29;
      fVar37 = pfVar21[2];
      fVar36 = *pfVar21;
      fVar23 = fastrand::Rng::rng_f32(p1);
      fVar24 = fastrand::Rng::rng_f32(p1);
      fVar25 = fastrand::Rng::rng_f32(p1);
      fVar34 = __real_3f800000;
      if (uVar17 == p8) {
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_bounds_check();
      }
      uVar19 = uVar17 + 1;
      fVar26 = p7[uVar17];
      fVar23 = fVar26 * (fVar23 + ___xmm_0000000000000000bf000000bf000000) *
               (float)__xmm_00000000000000003e99999a3e99999a +
               (float)__xmm_00000000000000003f80000000000000;
      fVar24 = fVar26 * (fVar24 + _UNK_142a171f4) * __xmm_00000000000000003e99999a3e99999a._4_4_ +
               __xmm_00000000000000003f80000000000000._4_4_;
      if (uVar19 < p6) {
        unaff_XMM13_Da = p5[uVar19 * 3];
        unaff_XMM13_Db = p5[uVar19 * 3 + 2];
      }
      fVar36 = fVar36 + __xmm_00000000000000000000000000000000;
      fVar37 = fVar37 + DAT_1429259a4;
      fVar25 = (fVar25 + __real_bf000000) * fVar26 * __real_3e99999a + __real_00000000;
      uVar20 = 0;
      if (uVar17 != 0) {
        uVar20 = uVar17 - 1;
      }
      fVar26 = 0.0;
      fVar28 = 0.0;
      if (uVar20 < p6) {
        fVar26 = p5[uVar20 * 3];
        fVar28 = p5[uVar20 * 3 + 2];
      }
      uVar29 = (int)((uint)(uVar19 < p6) << 0x1f) >> 0x1f;
      uVar30 = (int)((uint)(uVar19 < p6) << 0x1f) >> 0x1f;
      uVar17 = CONCAT44(~uVar30 & (uint)fVar28,~uVar29 & (uint)fVar26) |
               CONCAT44((uint)unaff_XMM13_Db & uVar30,(uint)unaff_XMM13_Da & uVar29);
      fVar26 = fVar36 - (float)uVar17;
      fVar28 = fVar37 - (float)(uVar17 >> 0x20);
      fVar31 = __real_3f800000 / SQRT(fVar28 * fVar28 + fVar26 * fVar26);
      local_248 = CONCAT44(fVar31 * fVar28,fVar31 * fVar26);
      ::utils::swizzlers::impl$0::x0y(&local_1f0,(float *)&local_248);
      fVar28 = local_1f0;
      fVar26 = __real_3e20c49c;
      fVar35 = fVar35 + ___real_be3851ec;
      fVar31 = local_1e8 * __real_3e20c49c;
      fVar34 = fVar34 / SQRT(local_1f0 * local_1f0 + local_1e8 * local_1e8);
      local_248 = CONCAT44(fVar34 * local_1f0,
                           (float)((uint)fVar34 ^ ___xmm_80000000800000008000000080000000) *
                           local_1e8);
      ::utils::swizzlers::impl$0::x0y(local_1d8,(float *)&local_248);
      local_270 = fVar28 * ___xmm_00000000000000003e20c49c3e20c49c;
      fStack_26c = local_1ec * _UNK_142a8c6c4;
      local_258 = fVar25 * ___real_3f849ba6;
      local_260 = CONCAT44(fVar24 * _UNK_142a8c6d4,fVar23 * ___xmm_00000000000000003f849ba63f849ba6)
      ;
      local_248 = CONCAT44(local_1d8._4_4_ * _UNK_142a8c6c4,
                           local_1d8._0_4_ * ___xmm_00000000000000003e20c49c3e20c49c);
      uStack_240 = CONCAT44(uStack_240._4_4_,local_1d0 * fVar26);
      local_268 = fVar31;
      local_1e4 = fVar36;
      local_1e0 = fVar35;
      local_1dc = fVar37;
      country_core::resources::render::Affine3Packed::from_cols
                (&local_178,&local_270,(float *)&local_260,(float *)&local_248,&local_1e4);
      lVar2 = *(longlong *)p1;
      uVar17 = lVar2 + 0xa0761d6478bd642f;
      *(ulonglong *)p1 = uVar17;
      auVar3._8_8_ = 0;
      auVar3._0_8_ = uVar17 ^ 0xe7037ed1a0b428db;
      auVar9._8_8_ = 0;
      auVar9._0_8_ = uVar17;
      uVar20 = SUB168(auVar3 * auVar9,8) ^ SUB168(auVar3 * auVar9,0);
      if ((uint)((int)uVar20 * 1000) < 0x128) {
        uVar22 = lVar2 + 0x40ec3ac8f17ac85e;
        do {
          uVar17 = uVar17 + 0xa0761d6478bd642f;
          auVar4._8_8_ = 0;
          auVar4._0_8_ = uVar22 ^ 0xe7037ed1a0b428db;
          auVar10._8_8_ = 0;
          auVar10._0_8_ = uVar22;
          uVar20 = SUB168(auVar4 * auVar10,8) ^ SUB168(auVar4 * auVar10,0);
          uVar22 = uVar22 + 0xa0761d6478bd642f;
        } while ((uint)((int)uVar20 * 1000) < 0x128);
        *(ulonglong *)p1 = uVar17;
      }
      pfVar21 = pfVar21 + 3;
      local_228 = local_158;
      uStack_220 = uStack_150;
      local_238 = local_168;
      uStack_230 = uStack_160;
      local_248 = local_178;
      uStack_240 = uStack_170;
      local_218 = 0;
      local_214 = (undefined4)((uVar20 & 0xffffffff) * 1000 >> 0x20);
      local_20c = 0x18;
      pfVar16 = local_250;
      local_210 = p2;
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push_wood
                (local_250,&local_248);
      uVar17 = uVar19;
    } while (pfVar21 != local_208);
    if (p6 != 0) {
      bVar15 = true;
      uVar17 = 0;
      fVar37 = (float)__xmm_00000000000000003e4ccccd3e4ccccd;
      fVar36 = __xmm_00000000000000003e4ccccd3e4ccccd._4_4_;
      do {
        fVar34 = __real_be19999a;
        pfVar21 = p5 + 3;
        pfVar18 = pfVar16;
        pfVar16 = p5;
        if (bVar15) {
          if (pfVar21 == local_208) {
            return;
          }
          pfVar18 = p5;
          pfVar16 = pfVar21;
          pfVar21 = p5 + 6;
        }
        fVar23 = *pfVar18;
        fVar24 = pfVar18[2];
        fVar32 = pfVar18[1] + __real_be19999a;
        fVar25 = *pfVar16;
        fVar35 = pfVar16[1];
        fVar26 = pfVar16[2];
        fVar28 = fastrand::Rng::rng_f32(p1);
        fVar31 = fastrand::Rng::rng_f32(p1);
        fVar27 = fastrand::Rng::rng_f32(p1);
        if (uVar17 == p8) {
                    /* WARNING: Subroutine does not return */
          core::panicking::panic_bounds_check();
        }
        fVar1 = p7[uVar17];
        fVar25 = fVar25 + (fVar28 + __real_bf000000) * fVar1 * __real_3e4ccccd;
        fVar28 = fVar25 - fVar23;
        local_1b8 = CONCAT44(fVar1,fVar1);
        uStack_1b0 = 0;
        fVar33 = (fVar31 + ___xmm_0000000000000000bf000000bf000000) * fVar1 * fVar37 +
                 fVar35 + fVar34;
        fVar26 = (fVar27 + _UNK_142a171f4) * fVar1 * fVar36 + fVar26;
        fVar36 = fVar33 - fVar32;
        fVar37 = fVar26 - fVar24;
        fVar34 = __real_3f800000 / SQRT(fVar37 * fVar37 + fVar28 * fVar28 + fVar36 * fVar36);
        fVar28 = fVar28 * fVar34;
        fVar36 = fVar34 * fVar36;
        fVar34 = fVar34 * fVar37;
        fVar35 = fVar28 - fVar36 * __xmm_00000000000000000000000000000000;
        fVar31 = fVar36 * __xmm_00000000000000000000000000000000 - fVar34;
        fVar27 = fVar34 * DAT_1429259a4 - fVar28 * 0.0;
        fVar37 = __real_3f800000 / SQRT(fVar35 * fVar35 + fVar27 * fVar27 + fVar31 * fVar31);
        fVar31 = fVar37 * fVar31;
        fVar27 = fVar37 * fVar27;
        fVar37 = fVar37 * fVar35;
        local_268 = SQRT((fVar24 - fVar26) * (fVar24 - fVar26) +
                         (fVar32 - fVar33) * (fVar32 - fVar33) +
                         (fVar23 - fVar25) * (fVar23 - fVar25));
        local_270 = local_268 * fVar28;
        local_198 = ZEXT416((uint)local_268);
        local_1a8 = CONCAT44(local_268,local_268);
        uStack_1a0 = 0;
        fStack_26c = local_268 * fVar36;
        local_268 = local_268 * fVar34;
        fVar35 = fVar36 * fVar31 - fVar27 * fVar28;
        local_200 = CONCAT44(fVar27 * _UNK_142a8c6c4,
                             fVar31 * ___xmm_00000000000000003e20c49c3e20c49c);
        local_1f8 = fVar37 * __real_3e20c49c;
        fVar36 = fVar27 * fVar34 - fVar37 * fVar36;
        fVar37 = fVar37 * fVar28 - fVar31 * fVar34;
        fVar34 = __real_3f800000 / SQRT(fVar37 * fVar37 + fVar36 * fVar36 + fVar35 * fVar35);
        local_258 = fVar34 * fVar35 * __real_3e20c49c;
        local_260 = CONCAT44(fVar34 * fVar37 * _UNK_142a8c6c4,
                             fVar34 * fVar36 * ___xmm_00000000000000003e20c49c3e20c49c);
        fVar34 = fVar23 + ___xmm_00000000000000003f10a3d700000000;
        local_188 = ZEXT416((uint)fVar24);
        local_248 = CONCAT44(fVar32 + _UNK_142a8c704,fVar34);
        local_1c8 = ZEXT416((uint)(fVar24 + __real_00000000));
        uStack_240 = CONCAT44(uStack_240._4_4_,fVar24 + __real_00000000);
        country_core::resources::render::Affine3Packed::from_cols
                  (&local_148,(float *)&local_200,&local_270,(float *)&local_260,(float *)&local_248
                  );
        if (local_1f4 < __real_3fcccccd) {
          lVar2 = *(longlong *)p1;
          uVar19 = lVar2 + 0xa0761d6478bd642f;
          *(ulonglong *)p1 = uVar19;
          auVar5._8_8_ = 0;
          auVar5._0_8_ = uVar19 ^ 0xe7037ed1a0b428db;
          auVar11._8_8_ = 0;
          auVar11._0_8_ = uVar19;
          uVar20 = SUB168(auVar5 * auVar11,8) ^ SUB168(auVar5 * auVar11,0);
          if ((uint)((int)uVar20 * 1000) < 0x128) {
            uVar22 = lVar2 + 0x40ec3ac8f17ac85e;
            do {
              uVar19 = uVar19 + 0xa0761d6478bd642f;
              auVar6._8_8_ = 0;
              auVar6._0_8_ = uVar22 ^ 0xe7037ed1a0b428db;
              auVar12._8_8_ = 0;
              auVar12._0_8_ = uVar22;
              uVar20 = SUB168(auVar6 * auVar12,8) ^ SUB168(auVar6 * auVar12,0);
              uVar22 = uVar22 + 0xa0761d6478bd642f;
            } while ((uint)((int)uVar20 * 1000) < 0x128);
            *(ulonglong *)p1 = uVar19;
          }
          local_228 = local_128;
          uStack_220 = uStack_120;
          local_238 = local_138;
          uStack_230 = uStack_130;
          local_248 = local_148;
          uStack_240 = uStack_140;
          local_218 = 0;
          local_214 = (undefined4)((uVar20 & 0xffffffff) * 1000 >> 0x20);
          local_20c = 0x18;
          local_210 = p2;
          country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push_wood
                    (local_250,&local_248);
        }
        fVar31 = fVar32 + __real_be19999a;
        fVar33 = fVar33 + __real_be19999a;
        fVar24 = fastrand::Rng::rng_f32(p1);
        fVar35 = fastrand::Rng::rng_f32(p1);
        fVar28 = fastrand::Rng::rng_f32(p1);
        fVar36 = __xmm_00000000000000003e4ccccd3e4ccccd._4_4_;
        fVar37 = (float)__xmm_00000000000000003e4ccccd3e4ccccd;
        fVar23 = (fVar25 + fVar1 * (fVar24 + __real_bf000000) * __real_3e4ccccd) - fVar23;
        fVar31 = (fVar33 + (float)local_1b8 * (fVar35 + ___xmm_0000000000000000bf000000bf000000) *
                           (float)__xmm_00000000000000003e4ccccd3e4ccccd) - fVar31;
        fVar24 = (fVar26 + local_1b8._4_4_ * (fVar28 + _UNK_142a171f4) *
                           __xmm_00000000000000003e4ccccd3e4ccccd._4_4_) - (float)local_188._0_4_;
        fVar35 = __real_3f800000 / SQRT(fVar24 * fVar24 + fVar23 * fVar23 + fVar31 * fVar31);
        fVar23 = fVar23 * fVar35;
        fVar31 = fVar35 * fVar31;
        fVar35 = fVar35 * fVar24;
        fVar26 = fVar23 - fVar31 * __xmm_00000000000000000000000000000000;
        fVar28 = fVar31 * __xmm_00000000000000000000000000000000 - fVar35;
        fVar25 = fVar35 * DAT_1429259a4 - fVar23 * __real_00000000;
        fVar24 = __real_3f800000 / SQRT(fVar26 * fVar26 + fVar25 * fVar25 + fVar28 * fVar28);
        fVar28 = fVar24 * fVar28;
        fVar25 = fVar24 * fVar25;
        fVar24 = fVar24 * fVar26;
        local_270 = (float)local_198._0_4_ * fVar23;
        fStack_26c = (float)local_1a8 * fVar31;
        local_268 = local_1a8._4_4_ * fVar35;
        fVar26 = fVar31 * fVar28 - fVar25 * fVar23;
        local_200 = CONCAT44(fVar25 * _UNK_142a8c6c4,
                             fVar28 * ___xmm_00000000000000003e20c49c3e20c49c);
        local_1f8 = fVar24 * __real_3e20c49c;
        fVar25 = fVar25 * fVar35 - fVar24 * fVar31;
        fVar23 = fVar24 * fVar23 - fVar28 * fVar35;
        fVar24 = __real_3f800000 / SQRT(fVar23 * fVar23 + fVar25 * fVar25 + fVar26 * fVar26);
        local_260 = CONCAT44(fVar24 * fVar23 * _UNK_142a8c6c4,
                             fVar24 * fVar25 * ___xmm_00000000000000003e20c49c3e20c49c);
        local_258 = fVar24 * fVar26 * __real_3e20c49c;
        local_248 = CONCAT44(fVar32 + ___real_3f74fdf4,fVar34);
        uStack_240 = CONCAT44(uStack_240._4_4_,local_1c8._0_4_);
        country_core::resources::render::Affine3Packed::from_cols
                  (&local_118,(float *)&local_200,&local_270,(float *)&local_260,(float *)&local_248
                  );
        lVar2 = *(longlong *)p1;
        uVar19 = lVar2 + 0xa0761d6478bd642f;
        *(ulonglong *)p1 = uVar19;
        auVar7._8_8_ = 0;
        auVar7._0_8_ = uVar19 ^ 0xe7037ed1a0b428db;
        auVar13._8_8_ = 0;
        auVar13._0_8_ = uVar19;
        uVar20 = SUB168(auVar7 * auVar13,8) ^ SUB168(auVar7 * auVar13,0);
        if ((uint)((int)uVar20 * 1000) < 0x128) {
          uVar22 = lVar2 + 0x40ec3ac8f17ac85e;
          do {
            uVar19 = uVar19 + 0xa0761d6478bd642f;
            auVar8._8_8_ = 0;
            auVar8._0_8_ = uVar22 ^ 0xe7037ed1a0b428db;
            auVar14._8_8_ = 0;
            auVar14._0_8_ = uVar22;
            uVar20 = SUB168(auVar8 * auVar14,8) ^ SUB168(auVar8 * auVar14,0);
            uVar22 = uVar22 + 0xa0761d6478bd642f;
          } while ((uint)((int)uVar20 * 1000) < 0x128);
          *(ulonglong *)p1 = uVar19;
        }
        uVar17 = uVar17 + 1;
        local_228 = local_f8;
        uStack_220 = uStack_f0;
        local_238 = local_108;
        uStack_230 = uStack_100;
        local_248 = local_118;
        uStack_240 = uStack_110;
        local_218 = 0;
        local_214 = (undefined4)((uVar20 & 0xffffffff) * 1000 >> 0x20);
        local_20c = 0x18;
        local_210 = p2;
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::push_wood
                  (local_250,&local_248);
        bVar15 = false;
        p5 = pfVar21;
      } while (pfVar21 != local_208);
    }
  }
  return;
}

