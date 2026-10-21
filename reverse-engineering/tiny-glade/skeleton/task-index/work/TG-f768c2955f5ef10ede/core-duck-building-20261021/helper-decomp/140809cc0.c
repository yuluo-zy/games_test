
// WARNING: Type propagation algorithm not settling
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::decorator_visual::stairs::walk_shape(void)

{
  ulonglong *puVar1;
  undefined4 *puVar2;
  float *******pppppppfVar3;
  ulonglong *puVar4;
  float *******pppppppfVar5;
  float *******pppppppfVar6;
  float *pfVar7;
  undefined4 uVar8;
  undefined8 uVar9;
  undefined1 auVar10 [12];
  undefined4 uVar11;
  undefined4 uVar12;
  undefined4 uVar13;
  undefined4 uVar14;
  undefined4 uVar15;
  undefined4 uVar16;
  undefined4 uVar17;
  undefined4 uVar18;
  undefined4 uVar19;
  undefined4 uVar20;
  undefined4 uVar21;
  undefined4 uVar22;
  undefined4 uVar23;
  undefined4 uVar24;
  undefined1 auVar25 [16];
  ulonglong uVar26;
  ulonglong uVar27;
  undefined1 auVar28 [8];
  char cVar29;
  float *_Dst;
  float *******pppppppfVar30;
  bool bVar31;
  undefined8 *in_RCX;
  longlong lVar32;
  float *******pppppppfVar33;
  float *******pppppppfVar34;
  longlong in_RDX;
  longlong lVar35;
  float *******pppppppfVar36;
  float *******pppppppfVar37;
  undefined8 *******pppppppuVar38;
  char *in_R8;
  ulonglong uVar39;
  undefined4 *puVar40;
  char *in_R9;
  ulonglong uVar41;
  float *******pppppppfVar42;
  ulonglong uVar43;
  undefined8 *******pppppppuVar44;
  undefined8 *******pppppppuVar45;
  ulonglong uVar46;
  undefined8 *******pppppppuVar47;
  uint uVar48;
  uint uVar49;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float fVar50;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  uint extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  undefined8 uVar51;
  float fVar52;
  undefined1 auVar53 [12];
  undefined1 auVar54 [16];
  float fVar55;
  float fVar56;
  float fVar57;
  float fVar58;
  float fVar59;
  float fVar60;
  float fVar61;
  float fVar65;
  undefined1 auVar62 [12];
  float fVar66;
  float fVar67;
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  undefined1 auVar68 [12];
  uint uVar69;
  uint uVar70;
  undefined1 auVar71 [12];
  float in_stack_00000028;
  float in_stack_00000030;
  float in_stack_00000038;
  float in_stack_00000040;
  float in_stack_00000048;
  float in_stack_00000050;
  float in_stack_00000058;
  float in_stack_00000068;
  undefined4 local_324 [3];
  float local_318;
  float fStack_314;
  undefined4 uStack_310;
  undefined4 uStack_30c;
  undefined8 local_308;
  float *local_300;
  longlong local_2f8;
  undefined8 *******local_2f0;
  float *******pppppppfStack_2e8;
  float *******local_2e0;
  float *******pppppppfStack_2d8;
  float *******local_2d0;
  float *******pppppppfStack_2c8;
  float *******local_2c0;
  float *******local_2b8;
  float *******local_2b0;
  undefined8 *******local_2a8;
  float *local_2a0;
  undefined1 *local_298;
  undefined8 *******local_290;
  undefined8 *******local_288;
  undefined4 *local_280;
  undefined8 *******local_278;
  undefined8 *******local_270;
  undefined8 *******local_268;
  undefined8 *******local_260;
  float *******local_258;
  float *******pppppppfStack_250;
  float *******pppppppfStack_248;
  float *******local_240;
  float *******pppppppfStack_238;
  float *******pppppppfStack_230;
  undefined8 *******local_228;
  longlong lStack_220;
  undefined1 local_214 [8];
  undefined1 local_20c [4];
  undefined8 *******local_208;
  undefined4 local_1fc;
  undefined1 local_1f8 [4];
  undefined1 local_1f4 [12];
  undefined1 local_1e8 [8];
  longlong lStack_1e0;
  undefined1 local_1d8 [16];
  undefined8 *******local_1c8 [2];
  float *******local_1b8;
  float *******pppppppfStack_1b0;
  float *******local_1a8;
  float *******pppppppfStack_1a0;
  float *******local_198;
  undefined8 local_190;
  float *******pppppppfStack_188;
  undefined8 *******local_180;
  float *******local_178;
  float *******pppppppfStack_170;
  float *******local_168;
  float *******pppppppfStack_160;
  float *******local_158;
  float *******pppppppfStack_150;
  float *******local_148;
  longlong local_140;
  undefined8 *******local_130;
  longlong lStack_128;
  undefined8 *******local_120;
  undefined8 *******local_118;
  float *******pppppppfStack_110;
  undefined8 *******local_108;
  float *local_100;
  undefined1 local_f4;
  undefined1 local_f3;
  undefined1 local_f2;
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
  ::utils::wall_space::impl$1::subtract();
  uVar48 = 0;
  if (*(uint *)(in_RDX + 600) < 2) {
    uVar48 = *(uint *)(in_RDX + 600);
  }
  if ((uVar48 & 1) == 0) {
    if (*in_R8 != '\x01') {
      in_RCX[4] = 0;
      in_RCX[5] = 4;
      in_RCX[6] = 0;
      in_RCX[7] = 0x8000000000000000;
      in_RCX[10] = 0x8000000000000000;
LAB_14080a1f6:
      in_RCX[0xd] = 0x8000000000000000;
      *in_RCX = 0;
      return;
    }
    local_118 = (undefined8 *******)0x0;
    pppppppfStack_110 = (float *******)&DAT_00000004;
    local_108 = (undefined8 *******)0x0;
    local_130 = (undefined8 *******)0x0;
    lStack_128 = 4;
    local_120 = (undefined8 *******)0x0;
    local_f1 = in_R8[2];
    if (local_f1 == '\0') {
      auVar68 = ZEXT812(0x3f800000);
    }
    else {
      auVar68 = ZEXT812(0xbf800000);
    }
    if (0.001 <= ABS(extraout_XMM0_Da)) {
      fVar50 = ceilf(ABS(extraout_XMM0_Da));
      uVar39 = 0;
      if (0.0 <= fVar50) {
        uVar39 = (longlong)(fVar50 - 9.223372e+18) & (longlong)fVar50 >> 0x3f | (longlong)fVar50;
      }
      uVar41 = 0xffffffffffffffff;
      if (fVar50 <= 1.8446743e+19) {
        uVar41 = uVar39;
      }
      pppppppuVar44 = (undefined8 *******)(uVar41 + (uVar41 == 0));
      pppppppuVar47 = (undefined8 *******)((longlong)pppppppuVar44 + 1);
      if (pppppppuVar47 != (undefined8 *******)0x0) {
        if (((ulonglong)pppppppuVar47 >> 0x3d != 0) ||
           (0x7ffffffffffffffc < (ulonglong)((longlong)pppppppuVar47 * 8))) {
LAB_14080ba32:
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        local_2b0 = (float *******)0x0;
        alloc::raw_vec::finish_grow<alloc::alloc::Global>();
        if ((int)local_178 == 1) goto LAB_14080ba32;
        pppppppfStack_110 = pppppppfStack_170;
        local_118 = pppppppuVar47;
      }
      fVar50 = (float)(~-(uint)NAN(extraout_XMM0_Da) &
                       ((uint)extraout_XMM0_Da & 0x80000000 |
                       (uint)__xmm_3f8000003f8000003f8000003f800000) |
                      -(uint)NAN(extraout_XMM0_Da) & 0x7fc00000);
      if (in_R8[1] != '\0') {
        ::utils::wall_space::impl$1::project_clamp();
      }
      if (*in_R9 != '\x01') {
        in_RCX[4] = 0;
        in_RCX[5] = 4;
        in_RCX[6] = 0;
        bVar31 = true;
        goto LAB_14080ab5a;
      }
      if (in_R9[1] != '\0') {
        ::utils::wall_space::impl$1::project_clamp();
      }
      fVar55 = 1.0 / SQRT(in_stack_00000038 * in_stack_00000038 +
                          in_stack_00000040 * in_stack_00000040);
      fVar52 = 1.0 / SQRT(in_stack_00000028 * in_stack_00000028 +
                          in_stack_00000030 * in_stack_00000030);
      auVar53._0_4_ = -in_stack_00000040 * fVar55;
      auVar53._4_4_ = in_stack_00000038 * fVar55;
      auVar53._8_4_ = 0x80000000;
      uVar48 = (uint)((int)fVar55 - 1U < 0x7fffff ||
                     (int)ABS(fVar55) - 0x800000U < 0x7f000000 && -1 < (int)fVar55);
      auVar71._0_4_ = (int)(uVar48 << 0x1f) >> 0x1f;
      auVar71._4_4_ = (int)(uVar48 << 0x1f) >> 0x1f;
      auVar71._8_4_ = 0;
      uVar48 = (uint)((int)fVar52 - 1U < 0x7fffff ||
                     (int)ABS(fVar52) - 0x800000U < 0x7f000000 && -1 < (int)fVar52);
      auVar62._0_4_ = (int)(uVar48 << 0x1f) >> 0x1f;
      auVar62._4_4_ = (int)(uVar48 << 0x1f) >> 0x1f;
      auVar62._8_4_ = 0;
      auVar10._4_4_ = in_stack_00000028 * fVar52;
      auVar10._0_4_ = -in_stack_00000030 * fVar52;
      auVar10._8_4_ = 0x80000000;
      local_318 = auVar68._0_4_;
      uStack_310 = auVar68._8_4_;
      fStack_314 = local_318;
      uStack_30c = uStack_310;
      for (pppppppuVar45 = (undefined8 *******)0x0; pppppppuVar38 = local_108,
          pppppppuVar45 < pppppppuVar47;
          pppppppuVar45 = (undefined8 *******)((longlong)pppppppuVar45 + 1)) {
        ::utils::wall_space::impl$1::subtract();
        ::utils::wall_space::impl$1::project_clamp();
        ::utils::lerp();
        pppppppuVar38 = local_120;
        if (local_120 == local_130) {
          alloc::raw_vec::RawVec::grow_one<f32,alloc::alloc::Global>();
        }
        *(float *)(lStack_128 + (longlong)pppppppuVar38 * 4) = extraout_XMM0_Da_02;
        local_120 = (undefined8 *******)((longlong)pppppppuVar38 + 1);
        fVar57 = extraout_XMM0_Da_01;
        ::utils::wall_space::impl$1::project_clamp();
        local_2b8 = (float *******)(ulonglong)extraout_XMM0_Da_03;
        ::utils::wall_space::WallSpaceCoord::get_pos_xz();
        fVar52 = SUB124(auVar62 & auVar10,0) * fVar50;
        fVar55 = SUB124(auVar62 & auVar10,4) * fVar50;
        if ((pppppppuVar45 != (undefined8 *******)0x0) &&
           (fVar52 = SUB124(auVar71 & auVar53,0) * fVar50,
           fVar55 = SUB124(auVar71 & auVar53,4) * fVar50, pppppppuVar45 != pppppppuVar44)) {
          fVar52 = fVar57;
          ::utils::wall_space::WallSpaceCoord::get_tangent();
          fVar52 = -fVar52;
          fVar55 = extraout_XMM0_Da_05;
        }
        pppppppuVar38 = local_108;
        if (local_108 == local_118) {
          alloc::raw_vec::RawVec::grow_one<tuple_<u8,u32>,alloc::alloc::Global>();
        }
        pppppppfStack_110[(longlong)pppppppuVar38] =
             (float ******)
             CONCAT44(fVar57 + extraout_XMM0_Da_02 * 0.5 * fVar55 * fStack_314,
                      extraout_XMM0_Da_04 + extraout_XMM0_Da_02 * 0.5 * fVar52 * local_318);
        local_108 = (undefined8 *******)((longlong)pppppppuVar38 + 1);
      }
      if ((undefined8 *******)0x1 < local_108) {
        lVar32 = (longlong)local_108 * 8;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (_Dst == (float *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        memset(_Dst,0,lVar32 - 8);
        (_Dst + (longlong)pppppppuVar38 * 2 + -2)[0] = 0.0;
        (_Dst + (longlong)pppppppuVar38 * 2 + -2)[1] = 0.0;
        local_2f8 = lVar32;
        if (local_108 < (undefined8 *******)0x2) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar50 = SUB84(pppppppfStack_110[1],0) - SUB84(*pppppppfStack_110,0);
        fVar52 = (float)((ulonglong)pppppppfStack_110[1] >> 0x20) -
                 (float)((ulonglong)*pppppppfStack_110 >> 0x20);
        fVar55 = 1.0 / SQRT(fVar52 * fVar52 + fVar50 * fVar50);
        uVar48 = (uint)((int)fVar55 - 1U < 0x7fffff ||
                       (int)ABS(fVar55) - 0x800000U < 0x7f000000 && -1 < (int)fVar55);
        *(ulonglong *)_Dst =
             CONCAT44((int)(uVar48 << 0x1f) >> 0x1f,(int)(uVar48 << 0x1f) >> 0x1f) &
             CONCAT44(fVar55 * fVar52,fVar55 * fVar50);
        if (pppppppuVar38 <= (undefined8 *******)((longlong)local_108 - 1U)) {
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar50 = SUB84(pppppppfStack_110[(longlong)local_108 + -1],0) -
                 SUB84(pppppppfStack_110[(longlong)local_108 + -2],0);
        fVar52 = (float)((ulonglong)pppppppfStack_110[(longlong)local_108 + -1] >> 0x20) -
                 (float)((ulonglong)pppppppfStack_110[(longlong)local_108 + -2] >> 0x20);
        fVar55 = 1.0 / SQRT(fVar52 * fVar52 + fVar50 * fVar50);
        uVar48 = (uint)((int)fVar55 - 1U < 0x7fffff ||
                       (int)ABS(fVar55) - 0x800000U < 0x7f000000 && -1 < (int)fVar55);
        *(ulonglong *)(_Dst + (longlong)local_108 * 2 + -2) =
             CONCAT44((int)(uVar48 << 0x1f) >> 0x1f,(int)(uVar48 << 0x1f) >> 0x1f) &
             CONCAT44(fVar55 * fVar52,fVar55 * fVar50);
        local_1f4._4_8_ = pppppppuVar38;
        local_288 = (undefined8 *******)0x2;
        local_100 = _Dst;
        if (local_108 != (undefined8 *******)0x2) {
          fVar50 = SUB84(pppppppfStack_110[2],0) - SUB84(*pppppppfStack_110,0);
          fVar52 = (float)((ulonglong)pppppppfStack_110[2] >> 0x20) -
                   (float)((ulonglong)*pppppppfStack_110 >> 0x20);
          fVar55 = 1.0 / SQRT(fVar52 * fVar52 + fVar50 * fVar50);
          uVar48 = (uint)((int)fVar55 - 1U < 0x7fffff ||
                         (int)ABS(fVar55) - 0x800000U < 0x7f000000 && -1 < (int)fVar55);
          *(ulonglong *)(_Dst + 2) =
               CONCAT44((int)(uVar48 << 0x1f) >> 0x1f,(int)(uVar48 << 0x1f) >> 0x1f) &
               CONCAT44(fVar55 * fVar52,fVar55 * fVar50);
          auVar54 = ___xmm_7fffffff7fffffff7fffffff7fffffff;
          auVar25 = ___xmm_3f8000003f8000003f8000003f800000;
          local_288 = (undefined8 *******)0x3;
          if (local_108 != (undefined8 *******)0x3) {
            pppppppfVar33 = pppppppfStack_110 + 1;
            pppppppfVar34 = pppppppfStack_110 + 2;
            pppppppfVar30 = pppppppfStack_110 + 3;
            pppppppfVar37 = (float *******)(_Dst + 4);
            uVar43 = (longlong)pppppppuVar38 + 0x1ffffffffffffffeU & 0x1fffffffffffffff;
            uVar39 = (longlong)local_108 * 8 - 0x20U >> 3;
            uVar41 = uVar39;
            if (uVar43 < uVar39) {
              uVar41 = uVar43;
            }
            pppppppfVar36 = pppppppfVar30;
            pppppppfVar42 = pppppppfVar37;
            if (7 < uVar41) {
              uVar46 = uVar39;
              if (uVar43 < uVar39) {
                uVar46 = uVar43;
              }
              if ((pppppppfVar37 <= pppppppfVar37 + uVar46) && (_Dst + 5 <= _Dst + uVar46 * 2 + 5))
              {
                if (uVar43 < uVar39) {
                  uVar39 = uVar43;
                }
                if ((pppppppfStack_110 + uVar39 + 4 <= pppppppfVar37) ||
                   (_Dst + uVar39 * 2 + 6 <= pppppppfVar33)) {
                  uVar41 = uVar41 + 1;
                  uVar39 = 4;
                  if ((uVar41 & 3) != 0) {
                    uVar39 = (ulonglong)((uint)uVar41 & 3);
                  }
                  lVar32 = uVar41 - uVar39;
                  pppppppfVar36 = pppppppfVar30 + lVar32;
                  pppppppfVar33 = pppppppfVar33 + lVar32;
                  pppppppfVar34 = pppppppfVar34 + lVar32;
                  pppppppfVar42 = pppppppfVar37 + lVar32;
                  lVar35 = 0;
                  do {
                    pppppppfVar5 = pppppppfStack_110 + lVar35 + 1;
                    pppppppfVar3 = pppppppfVar30 + lVar35;
                    pppppppfVar6 = pppppppfStack_110 + lVar35 + 5;
                    fVar61 = *(float *)pppppppfVar6 - *(float *)pppppppfVar3;
                    fVar65 = *(float *)((longlong)pppppppfVar6 + 4) -
                             *(float *)((longlong)pppppppfVar3 + 4);
                    fVar66 = *(float *)(pppppppfVar6 + 1) - *(float *)(pppppppfVar3 + 1);
                    fVar67 = *(float *)((longlong)pppppppfVar6 + 0xc) -
                             *(float *)((longlong)pppppppfVar3 + 0xc);
                    fVar50 = *(float *)pppppppfVar3 - *(float *)pppppppfVar5;
                    fVar55 = *(float *)((longlong)pppppppfVar3 + 4) -
                             *(float *)((longlong)pppppppfVar5 + 4);
                    fVar59 = *(float *)(pppppppfVar3 + 1) - *(float *)(pppppppfVar5 + 1);
                    fVar60 = *(float *)((longlong)pppppppfVar3 + 0xc) -
                             *(float *)((longlong)pppppppfVar5 + 0xc);
                    auVar63._0_4_ = fVar61 * fVar61;
                    auVar63._4_4_ = fVar65 * fVar65;
                    auVar63._8_4_ = fVar66 * fVar66;
                    auVar63._12_4_ = fVar67 * fVar67;
                    auVar64._0_4_ = fVar50 * fVar50 + fVar55 * fVar55;
                    auVar64._4_4_ = fVar59 * fVar59 + fVar60 * fVar60;
                    auVar64._8_4_ = auVar63._0_4_ + auVar63._4_4_;
                    auVar64._12_4_ = auVar63._8_4_ + auVar63._12_4_;
                    auVar64 = sqrtps(auVar63,auVar64);
                    auVar64 = divps(auVar25,auVar64);
                    fVar52 = auVar64._0_4_;
                    fVar57 = auVar64._4_4_;
                    fVar56 = auVar64._8_4_;
                    fVar58 = auVar64._12_4_;
                    auVar64 = auVar64 & auVar54;
                    uVar48 = -(uint)((int)((int)fVar52 - 1U ^ 0x80000000) < -0x7f800001) |
                             -(uint)(-0x1000001 < (int)(auVar64._0_4_ - 0x800000U ^ 0x80000000) ||
                                    (int)fVar52 < 0) ^ 0xffffffff;
                    uVar49 = -(uint)((int)((int)fVar57 - 1U ^ 0x80000000) < -0x7f800001) |
                             -(uint)(-0x1000001 < (int)(auVar64._4_4_ - 0x800000U ^ 0x80000000) ||
                                    (int)fVar57 < 0) ^ 0xffffffff;
                    uVar69 = -(uint)((int)((int)fVar56 - 1U ^ 0x80000000) < -0x7f800001) |
                             -(uint)(-0x1000001 < (int)(auVar64._8_4_ - 0x800000U ^ 0x80000000) ||
                                    (int)fVar56 < 0) ^ 0xffffffff;
                    uVar70 = -(uint)((int)((int)fVar58 - 1U ^ 0x80000000) < -0x7f800001) |
                             -(uint)(-0x1000001 < (int)(auVar64._12_4_ - 0x800000U ^ 0x80000000) ||
                                    (int)fVar58 < 0) ^ 0xffffffff;
                    pfVar7 = _Dst + lVar35 * 2 + 8;
                    *pfVar7 = (float)((uint)(fVar61 * fVar56) & uVar69);
                    pfVar7[1] = (float)((uint)(fVar56 * fVar65) & uVar69);
                    pfVar7[2] = (float)((uint)(fVar66 * fVar58) & uVar70);
                    pfVar7[3] = (float)((uint)(fVar58 * fVar67) & uVar70);
                    pppppppfVar3 = pppppppfVar37 + lVar35;
                    *pppppppfVar3 =
                         (float ******)
                         CONCAT44((uint)(fVar52 * fVar55) & uVar48,(uint)(fVar50 * fVar52) & uVar48)
                    ;
                    *(uint *)(pppppppfVar3 + 1) = (uint)(fVar59 * fVar57) & uVar49;
                    *(uint *)((longlong)pppppppfVar3 + 0xc) = (uint)(fVar57 * fVar60) & uVar49;
                    lVar35 = lVar35 + 4;
                  } while (lVar32 != lVar35);
                }
              }
            }
            do {
              pppppppfVar30 = pppppppfVar34;
              local_288 = local_108;
              if (pppppppfVar42 == (float *******)(_Dst + (longlong)pppppppuVar38 * 2)) break;
              pppppppfVar37 = pppppppfVar36 + 1;
              fVar50 = SUB84(*pppppppfVar36,0) - SUB84(*pppppppfVar33,0);
              fVar52 = (float)((ulonglong)*pppppppfVar36 >> 0x20) -
                       (float)((ulonglong)*pppppppfVar33 >> 0x20);
              fVar55 = 1.0 / SQRT(fVar52 * fVar52 + fVar50 * fVar50);
              uVar48 = (uint)((int)fVar55 - 1U < 0x7fffff ||
                             (int)ABS(fVar55) - 0x800000U < 0x7f000000 && -1 < (int)fVar55);
              *pppppppfVar42 =
                   (float ******)
                   (CONCAT44((int)(uVar48 << 0x1f) >> 0x1f,(int)(uVar48 << 0x1f) >> 0x1f) &
                   CONCAT44(fVar55 * fVar52,fVar55 * fVar50));
              pppppppfVar42 = pppppppfVar42 + 1;
              pppppppfVar34 = pppppppfVar36;
              pppppppfVar36 = pppppppfVar37;
              pppppppfVar33 = pppppppfVar30;
            } while (pppppppfVar37 != pppppppfStack_110 + (longlong)local_108);
          }
        }
LAB_14080b0e8:
        cVar29 = local_f1;
        uVar9 = local_1f4._4_8_;
        local_2b0 = pppppppfStack_110 + (longlong)local_288;
        local_290 = local_288;
        if (pppppppuVar38 < local_288) {
          local_290 = pppppppuVar38;
        }
        local_278 = (undefined8 *******)(lStack_128 + (longlong)local_120 * 4);
        local_268 = local_120;
        if (local_290 <= local_120) {
          local_268 = local_290;
        }
        local_2a0 = local_100 + (longlong)pppppppuVar38 * 2;
        local_2a8 = (undefined8 *******)local_100;
        local_298 = (undefined1 *)0x0;
        local_280 = (undefined4 *)lStack_128;
        local_270 = (undefined8 *******)0x0;
        local_258 = (float *******)0x0;
        local_228 = (undefined8 *******)0x0;
        lStack_220 = 0;
        local_2b8 = pppppppfStack_110;
        local_260 = local_290;
        local_208 = pppppppuVar38;
        pppppppfVar33 = pppppppfStack_150;
        pppppppfVar34 = local_148;
        do {
          local_148 = pppppppfVar34;
          pppppppfStack_150 = pppppppfVar33;
          if (lStack_220 == 0) {
            if (local_270 < local_268) {
              lVar32 = (longlong)local_298 + (longlong)local_270;
              local_1d8._8_8_ = (longlong)local_270 * 4 + (longlong)local_280;
              if (local_258 == (float *******)0x0) {
                local_1d8._0_8_ = (float *******)((longlong)local_2a8 + lVar32 * 2 * 4);
                lStack_1e0 = (longlong)(local_2b8 + lVar32);
                local_1e8 = (undefined1  [8])0x1;
                local_1c8[0] = &local_2b8;
                local_270 = (undefined8 *******)((longlong)local_270 + 1);
                itertools::tuple_impl::impl$31::
                collect_from_iter_no_buf<tuple_<ref_<glam::f32::vec2::Vec2>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<tuple_<ref_<glam::f32::vec2::Vec2>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>,ref_mut_<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec2::Vec2>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs::walk_shape::closure_env_4>_>_>_>
                          ();
                pppppppfVar33 = local_158;
                local_240 = local_178;
                if (local_178 != (float *******)0x0) {
                  local_178 = pppppppfStack_170;
                  pppppppfStack_170 = local_168;
                  local_158 = pppppppfStack_150;
                  local_168 = pppppppfStack_160;
                  pppppppfStack_160 = pppppppfVar33;
                  goto LAB_14080b30d;
                }
              }
              else {
                local_178 = pppppppfStack_238;
                pppppppfStack_170 = pppppppfStack_230;
                local_270 = (undefined8 *******)((longlong)local_270 + 1);
                local_168 = local_2b8 + lVar32;
                pppppppfStack_160 = (float *******)((longlong)local_2a8 + lVar32 * 2 * 4);
                local_158 = (float *******)local_1d8._8_8_;
                if (local_240 != (float *******)0x0) {
LAB_14080b30d:
                  local_2c0 = local_158;
                  local_2d0 = local_168;
                  pppppppfStack_2c8 = pppppppfStack_160;
                  local_2e0 = local_178;
                  pppppppfStack_2d8 = pppppppfStack_170;
                  local_2f0 = local_228;
                  pppppppfStack_2e8 = local_240;
                  local_258 = local_240;
                  pppppppfStack_250 = local_178;
                  pppppppfStack_248 = pppppppfStack_170;
                  local_240 = local_168;
                  pppppppfStack_238 = pppppppfStack_160;
                  pppppppfStack_230 = local_158;
                  local_228 = (undefined8 *******)((longlong)local_228 + 1);
                  local_1b8 = local_178;
                  pppppppfStack_1b0 = pppppppfStack_170;
                  local_1a8 = local_168;
                  pppppppfStack_1a0 = pppppppfStack_160;
                  local_198 = local_158;
                  goto LAB_14080b34e;
                }
              }
            }
LAB_14080b8f0:
            in_RCX[6] = local_108;
            in_RCX[4] = local_118;
            in_RCX[5] = pppppppfStack_110;
            in_RCX[7] = uVar9;
            in_RCX[8] = local_100;
            in_RCX[9] = local_208;
            in_RCX[10] = local_130;
            in_RCX[0xb] = lStack_128;
            in_RCX[0xc] = local_120;
            goto LAB_14080a1f6;
          }
          lStack_220 = 0;
          core::iter::adapters::enumerate::impl$1::
          nth<itertools::tuple_impl::TupleWindows<core::iter::adapters::map::Map<core::iter::adapters::zip::Zip<core::iter::adapters::zip::Zip<core::slice::iter::Iter<glam::f32::vec2::Vec2>,core::slice::iter::Iter<glam::f32::vec2::Vec2>_>,core::slice::iter::Iter<f32>_>,system_decorator::decorator_visual::stairs::walk_shape::closure_env_4>,tuple_<tuple_<ref_<glam::f32::vec2::Vec2>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>,tuple_<ref_<glam::f32::vec2::Vec2>,ref_<glam::f32::vec2::Vec2>,ref_<f32>_>_>_>_>
                    ();
          if (pppppppfStack_2e8 == (float *******)0x0) goto LAB_14080b8f0;
LAB_14080b34e:
          pppppppuVar44 = local_2f0;
          fVar50 = *(float *)pppppppfStack_2d8 * 0.5 * 1.25;
          fVar55 = *(float *)local_2c0 * 0.5 * 1.25;
          fVar52 = fVar50 * -*(float *)((longlong)local_2e0 + 4) * local_318;
          fVar50 = fVar50 * *(float *)local_2e0 * fStack_314;
          fVar57 = SUB84(*pppppppfStack_2e8,0);
          fVar59 = (float)((ulonglong)*pppppppfStack_2e8 >> 0x20);
          fVar60 = fVar57 - fVar52;
          fVar58 = fVar59 - fVar50;
          uVar51 = CONCAT44(fVar50 + fVar59,fVar52 + fVar57);
          fVar50 = fVar55 * -*(float *)((longlong)pppppppfStack_2c8 + 4) * local_318;
          fVar55 = fVar55 * *(float *)pppppppfStack_2c8 * fStack_314;
          fVar52 = SUB84(*local_2d0,0);
          fVar57 = (float)((ulonglong)*local_2d0 >> 0x20);
          fVar59 = fVar52 - fVar50;
          fVar56 = fVar57 - fVar55;
          auVar54._0_8_ = CONCAT44(fVar55 + fVar57,fVar50 + fVar52);
          auVar54._8_4_ = fVar59;
          auVar54._12_4_ = fVar56;
          if (cVar29 == '\0') {
            register0x00001208 = fVar60;
            local_1e8 = (undefined1  [8])uVar51;
            register0x0000120c = fVar58;
          }
          else {
            register0x00001208 = fVar59;
            local_1e8 = (undefined1  [8])auVar54._0_8_;
            register0x0000120c = fVar56;
            auVar54._8_4_ = fVar60;
            auVar54._0_8_ = uVar51;
            auVar54._12_4_ = fVar58;
          }
          local_1d8._0_8_ = auVar54._8_8_;
          local_1d8._8_4_ = auVar54._0_4_;
          local_1d8._12_4_ = auVar54._4_4_;
          pppppppfVar33 = pppppppfStack_150;
          pppppppfVar34 = local_148;
          if ((local_2f0 != (undefined8 *******)0x0) &&
             (pppppppuVar47 = (undefined8 *******)((longlong)local_2f0 + 1),
             pppppppuVar47 < local_108)) {
            local_178 = (float *******)local_1e8;
            pppppppfStack_170 = (float *******)local_1c8;
            local_158 = (float *******)0x0;
            local_140 = 4;
            bVar31 = true;
            local_168 = local_178;
            pppppppfStack_160 = pppppppfStack_170;
            do {
              local_140 = local_140 + -1;
              pppppppfVar30 = local_168;
              if (local_168 == pppppppfStack_160) {
                pppppppfStack_160 = pppppppfStack_170;
                pppppppfVar30 = local_178;
                pppppppfVar33 = pppppppfStack_150;
                pppppppfVar34 = local_148;
                if (local_178 == pppppppfStack_170) break;
              }
              local_168 = pppppppfVar30 + 1;
              if (bVar31) {
                local_190 = 1;
                local_180 = &local_178;
                pppppppfStack_188 = pppppppfVar30;
                itertools::tuple_impl::impl$30::
                collect_from_iter_no_buf<ref_<glam::f32::vec2::Vec2>,core::iter::adapters::chain::Chain<core::iter::sources::once::Once<ref_<glam::f32::vec2::Vec2>_>,ref_mut_<core::iter::adapters::cycle::Cycle<core::slice::iter::Iter<glam::f32::vec2::Vec2>_>_>_>_>
                          ();
                local_158 = local_1b8;
                pppppppfVar30 = local_1a8;
                pppppppfStack_150 = local_1b8;
                local_148 = pppppppfStack_1b0;
                pppppppfVar33 = pppppppfStack_1b0;
                pppppppfVar34 = local_1a8;
                if (local_1b8 == (float *******)0x0) break;
              }
              pppppppuVar45 = local_108;
              fVar50 = SUB84(*local_148,0);
              fVar52 = (float)((ulonglong)*local_148 >> 0x20);
              local_158 = pppppppfStack_150;
              if (1e-08 < ((float)((ulonglong)*pppppppfVar30 >> 0x20) - fVar52) *
                          (SUB84(*pppppppfStack_150,0) - fVar50) -
                          (SUB84(*pppppppfVar30,0) - fVar50) *
                          ((float)((ulonglong)*pppppppfStack_150 >> 0x20) - fVar52)) {
                if (local_108 <= pppppppuVar44) {
                  pppppppfStack_150 = local_148;
                  local_148 = pppppppfVar30;
                    // WARNING: Subroutine does not return
                  alloc::vec::impl$1::remove::assert_failed();
                }
                uVar39 = ~(ulonglong)pppppppuVar44;
                pppppppfStack_150 = local_148;
                local_148 = pppppppfVar30;
                memmove(pppppppfStack_110 + (longlong)pppppppuVar44,
                        pppppppfStack_110 + (longlong)pppppppuVar44 + 1,
                        ((longlong)local_108 + uVar39) * 8);
                pppppppuVar38 = local_208;
                local_108 = (undefined8 *******)((longlong)pppppppuVar45 + -1);
                if (local_208 <= pppppppuVar44) {
                    // WARNING: Subroutine does not return
                  alloc::vec::impl$1::remove::assert_failed();
                }
                local_300 = local_100 + (longlong)pppppppuVar44 * 2;
                memmove(local_300,local_100 + (longlong)pppppppuVar44 * 2 + 2,
                        ((longlong)local_208 + uVar39) * 8);
                pppppppuVar45 = local_120;
                if (local_120 <= pppppppuVar44) {
                    // WARNING: Subroutine does not return
                  alloc::vec::impl$1::remove::assert_failed();
                }
                pppppppuVar38 = (undefined8 *******)((longlong)pppppppuVar38 - 1);
                memmove((void *)(lStack_128 + (longlong)pppppppuVar44 * 4),
                        (void *)(lStack_128 + (longlong)pppppppuVar44 * 4 + 4),
                        (uVar39 + (longlong)local_120) * 4);
                local_120 = (undefined8 *******)((longlong)pppppppuVar45 + -1);
                if (pppppppuVar44 == (undefined8 *******)0x1) {
                  if (local_108 < (undefined8 *******)0x2) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  if (pppppppuVar38 == (undefined8 *******)0x0) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  fVar50 = SUB84(pppppppfStack_110[1],0) - SUB84(*pppppppfStack_110,0);
                  fVar55 = (float)((ulonglong)pppppppfStack_110[1] >> 0x20) -
                           (float)((ulonglong)*pppppppfStack_110 >> 0x20);
                  fVar57 = 1.0 / SQRT(fVar55 * fVar55 + fVar50 * fVar50);
                  fVar52 = 0.0;
                  if ((int)fVar57 - 1U < 0x7fffff ||
                      (int)ABS(fVar57) - 0x800000U < 0x7f000000 && -1 < (int)fVar57) {
                    fVar50 = fVar50 * fVar57;
                    fVar52 = fVar55 * fVar57;
                  }
                  else {
                    fVar50 = 0.0;
                  }
                  *local_100 = fVar50;
                  local_100[1] = fVar52;
                  if ((undefined8 *******)0x2 < local_108) goto LAB_14080b7ee;
LAB_14080b833:
                  if (local_108 <= pppppppuVar44) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  if (local_108 <= (undefined8 *******)((longlong)pppppppuVar44 - 1U)) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  pppppppuVar47 = pppppppuVar44;
                  if (pppppppuVar38 <= pppppppuVar44) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                }
                else {
                  if (local_108 <= pppppppuVar44) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  if (local_108 <= (undefined8 *******)((longlong)pppppppuVar44 - 2U)) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  if (pppppppuVar38 <= (undefined8 *******)((longlong)pppppppuVar44 - 1U)) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  fVar50 = SUB84(pppppppfStack_110[(longlong)pppppppuVar44],0) -
                           SUB84(pppppppfStack_110[(longlong)pppppppuVar44 + -2],0);
                  fVar55 = (float)((ulonglong)pppppppfStack_110[(longlong)pppppppuVar44] >> 0x20) -
                           (float)((ulonglong)pppppppfStack_110[(longlong)pppppppuVar44 + -2] >>
                                  0x20);
                  fVar57 = 1.0 / SQRT(fVar55 * fVar55 + fVar50 * fVar50);
                  fVar52 = 0.0;
                  if ((int)fVar57 - 1U < 0x7fffff ||
                      (int)ABS(fVar57) - 0x800000U < 0x7f000000 && -1 < (int)fVar57) {
                    fVar50 = fVar50 * fVar57;
                    fVar52 = fVar55 * fVar57;
                  }
                  else {
                    fVar50 = 0.0;
                  }
                  local_100[(longlong)pppppppuVar44 * 2 + -2] = fVar50;
                  local_100[(longlong)((longlong)pppppppuVar44 - 1U) * 2 + 1] = fVar52;
                  if (local_108 <= pppppppuVar47) goto LAB_14080b833;
LAB_14080b7ee:
                  if (local_108 <= (undefined8 *******)((longlong)pppppppuVar44 - 1U)) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  if (pppppppuVar38 <= pppppppuVar44) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                }
                fVar50 = SUB84(pppppppfStack_110[(longlong)pppppppuVar47],0) -
                         SUB84(pppppppfStack_110[(longlong)pppppppuVar44 + -1],0);
                fVar52 = (float)((ulonglong)pppppppfStack_110[(longlong)pppppppuVar47] >> 0x20) -
                         (float)((ulonglong)pppppppfStack_110[(longlong)pppppppuVar44 + -1] >> 0x20)
                ;
                fVar55 = 1.0 / SQRT(fVar52 * fVar52 + fVar50 * fVar50);
                uVar48 = (uint)((int)fVar55 - 1U < 0x7fffff ||
                               (int)ABS(fVar55) - 0x800000U < 0x7f000000 && -1 < (int)fVar55);
                *(ulonglong *)local_300 =
                     CONCAT44((int)(uVar48 << 0x1f) >> 0x1f,(int)(uVar48 << 0x1f) >> 0x1f) &
                     CONCAT44(fVar55 * fVar52,fVar55 * fVar50);
                local_288 = local_108;
                goto LAB_14080b0e8;
              }
              bVar31 = false;
              pppppppfStack_150 = local_148;
              pppppppfVar33 = local_148;
              local_148 = pppppppfVar30;
              pppppppfVar34 = pppppppfVar30;
            } while (local_140 != 0);
          }
        } while( true );
      }
    }
    in_RCX[6] = local_108;
    in_RCX[4] = local_118;
    in_RCX[5] = pppppppfStack_110;
    bVar31 = false;
LAB_14080ab5a:
    in_RCX[7] = 0x8000000000000000;
    in_RCX[10] = 0x8000000000000000;
    in_RCX[0xd] = 0x8000000000000000;
    *in_RCX = 0;
    if (local_130 != (undefined8 *******)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((bVar31) && (local_118 != (undefined8 *******)0x0)) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
                    // WARNING: Read-only address (ram,0x000142925930) is written
                    // WARNING: Read-only address (ram,0x00014292e980) is written
    return;
  }
  local_108 = *(undefined8 ********)(in_RDX + 0x26c);
  local_118 = *(undefined8 ********)(in_RDX + 0x25c);
  pppppppfStack_110 = *(float ********)(in_RDX + 0x264);
  ::utils::wall_space::WallSpace::space_length();
  fVar52 = 1.0 / extraout_XMM0_Da_00;
  fVar55 = 0.0;
  fVar50 = 0.0;
  if ((*in_R8 == '\x01') && (in_R8[1] == '\x01')) {
    fVar50 = in_stack_00000048 * 0.5 * fVar52;
  }
  if ((*in_R9 == '\x01') && (in_R9[1] == '\x01')) {
    fVar55 = in_stack_00000050 * 0.5 * fVar52;
  }
  fVar57 = fVar52 * in_stack_00000058;
  fVar59 = fVar52 * in_stack_00000068;
  fVar56 = fVar55;
  if (extraout_XMM0_Da < 0.0) {
    fVar59 = fVar57;
    fVar56 = fVar50;
    fVar50 = fVar55;
    fVar57 = fVar52 * in_stack_00000068;
  }
  local_298 = local_20c;
  local_290 = (undefined8 *******)local_214;
  fVar52 = (fVar59 - fVar56) - (fVar50 + fVar57);
  uVar48 = -(uint)NAN(fVar59 - fVar57);
  uVar49 = -(uint)NAN(fVar52);
  uVar48 = -(uint)((float)(uVar49 & 0x7fc00000 | ~uVar49 & ((uint)fVar52 & 0x80000000 | 0x3f800000))
                   * (float)(uVar48 & 0x7fc00000 |
                            ~uVar48 & ((uint)(fVar59 - fVar57) & 0x80000000 | 0x3f800000)) != 1.0);
  fVar50 = (float)(~uVar48 & (uint)(fVar50 + fVar57) | (uint)fVar57 & uVar48);
  local_f2 = 1;
  local_130 = (undefined8 *******)0x0;
  lStack_128 = 4;
  local_120 = (undefined8 *******)0x0;
  local_190 = 0;
  pppppppfStack_188 = (float *******)&DAT_00000004;
  local_180 = (undefined8 *******)0x0;
  local_1b8 = (float *******)0x0;
  pppppppfStack_1b0 = (float *******)0x4;
  local_1a8 = (float *******)0x0;
  local_1e8 = (undefined1  [8])0x0;
  auVar28 = local_1e8;
  local_1e8 = (undefined1  [8])0x0;
  lStack_1e0 = 4;
  auVar25._8_8_ = 0;
  auVar25._0_8_ = local_1d8._8_8_;
  local_1d8 = auVar25 << 0x40;
  local_2e0 = (float *******)0x0;
  local_324[0] = 0;
  local_1fc = 0;
  local_2b8 = (float *******)local_1f4;
  local_2b0 = (float *******)local_1f8;
  local_2a8 = &local_130;
  local_2a0 = (float *)&local_f2;
  local_288 = (undefined8 *******)local_324;
  local_280 = &local_1fc;
  local_270 = (undefined8 *******)&local_190;
  local_268 = &local_1b8;
  local_260 = (undefined8 *******)local_1e8;
  local_f3 = 1;
  local_f4 = 0;
  local_308 = 0;
  local_278 = &local_2f0;
  if (NAN(fVar50)) {
    local_1e8 = auVar28;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fVar52 = (float)(~uVar48 & (uint)(fVar59 - fVar56) | (uint)fVar59 & uVar48);
  if (NAN(fVar52)) {
    local_1e8 = auVar28;
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  fmodf(fVar50,1.0);
  fmodf(fVar52,1.0);
  local_178 = (float *******)&local_f3;
  pppppppfStack_170 = (float *******)&local_f4;
  local_168 = (float *******)&local_2b8;
  pppppppfStack_160 = (float *******)&local_308;
  ::utils::geometry::rectangle::Rectangle2d::
  circular_slice_segment_points<system_decorator::decorator_visual::stairs::walk_shape::closure_env_1>
            ();
  if (0.0 <= extraout_XMM0_Da) goto LAB_14080a77a;
  if (1 < local_120) {
    lVar32 = lStack_128 + (longlong)local_120 * 8;
    if ((ulonglong)local_120 >> 1 == 1) {
      uVar39 = 0;
    }
    else {
      lVar35 = (longlong)local_120 - 1;
      uVar39 = 0;
      do {
        uVar9 = *(undefined8 *)(lStack_128 + uVar39 * 8);
        *(undefined8 *)(lStack_128 + uVar39 * 8) = *(undefined8 *)(lStack_128 + lVar35 * 8);
        *(undefined8 *)(lStack_128 + lVar35 * 8) = uVar9;
        uVar9 = *(undefined8 *)(lStack_128 + 8 + uVar39 * 8);
        *(undefined8 *)(lStack_128 + 8 + uVar39 * 8) =
             *(undefined8 *)(lVar32 + (uVar39 ^ 0x1ffffffffffffffe) * 8);
        *(undefined8 *)(lVar32 + (uVar39 ^ 0x1ffffffffffffffe) * 8) = uVar9;
        uVar39 = uVar39 + 2;
        lVar35 = lVar35 + -2;
      } while (((ulonglong)local_120 >> 1 & 0xfffffffffffffffe) != uVar39);
    }
    if (((ulonglong)local_120 & 2) != 0) {
      uVar9 = *(undefined8 *)(lStack_128 + uVar39 * 8);
      *(undefined8 *)(lStack_128 + uVar39 * 8) = *(undefined8 *)(lVar32 + ~uVar39 * 8);
      *(undefined8 *)(lVar32 + ~uVar39 * 8) = uVar9;
    }
  }
  if (1 < local_180) {
    if ((ulonglong)local_180 >> 1 == 1) {
      uVar39 = 0;
    }
    else {
      lVar32 = (longlong)local_180 - 1;
      uVar39 = 0;
      do {
        uVar41 = (ulonglong)pppppppfStack_188[uVar39];
        pppppppfStack_188[uVar39] = pppppppfStack_188[lVar32];
        pppppppfStack_188[lVar32] = (float ******)uVar41;
        uVar41 = (ulonglong)pppppppfStack_188[uVar39 + 1];
        pppppppfStack_188[uVar39 + 1] =
             pppppppfStack_188[(longlong)local_180 + (uVar39 ^ 0x1ffffffffffffffe)];
        pppppppfStack_188[(longlong)local_180 + (uVar39 ^ 0x1ffffffffffffffe)] =
             (float ******)uVar41;
        uVar39 = uVar39 + 2;
        lVar32 = lVar32 + -2;
      } while (((ulonglong)local_180 >> 1 & 0xfffffffffffffffe) != uVar39);
    }
    if (((ulonglong)local_180 & 2) != 0) {
      uVar41 = (ulonglong)pppppppfStack_188[uVar39];
      pppppppfStack_188[uVar39] = pppppppfStack_188[(longlong)local_180 + ~uVar39];
      pppppppfStack_188[(longlong)local_180 + ~uVar39] = (float ******)uVar41;
    }
  }
  if (local_180 != (undefined8 *******)0x0) {
    uVar39 = (longlong)local_180 - 1U & 0x1fffffffffffffff;
    pppppppfVar33 = pppppppfStack_188;
    if (2 < uVar39) {
      uVar39 = uVar39 + 1;
      uVar41 = uVar39 & 0xfffffffffffffffc;
      pppppppfVar33 = pppppppfStack_188 + uVar41;
      uVar43 = 0;
      do {
        puVar1 = (ulonglong *)(pppppppfStack_188 + uVar43);
        uVar48 = *(uint *)((longlong)puVar1 + 4);
        uVar46 = puVar1[1];
        uVar49 = *(uint *)((longlong)puVar1 + 0xc);
        puVar4 = (ulonglong *)(pppppppfStack_188 + uVar43 + 2);
        uVar26 = *puVar4;
        uVar69 = *(uint *)((longlong)puVar4 + 4);
        uVar27 = puVar4[1];
        uVar70 = *(uint *)((longlong)puVar4 + 0xc);
        puVar4 = (ulonglong *)(pppppppfStack_188 + uVar43);
        *(uint *)puVar4 = (uint)*puVar1 ^ 0x80000000;
        *(uint *)((longlong)puVar4 + 4) = uVar48 ^ 0x80000000;
        *(uint *)(puVar4 + 1) = (uint)uVar46 ^ 0x80000000;
        *(uint *)((longlong)puVar4 + 0xc) = uVar49 ^ 0x80000000;
        puVar1 = (ulonglong *)(pppppppfStack_188 + uVar43 + 2);
        *(uint *)puVar1 = (uint)uVar26 ^ 0x80000000;
        *(uint *)((longlong)puVar1 + 4) = uVar69 ^ 0x80000000;
        *(uint *)(puVar1 + 1) = (uint)uVar27 ^ 0x80000000;
        *(uint *)((longlong)puVar1 + 0xc) = uVar70 ^ 0x80000000;
        uVar43 = uVar43 + 4;
      } while (uVar41 != uVar43);
      if (uVar39 == uVar41) goto LAB_14080a516;
    }
    do {
      *pppppppfVar33 = (float ******)((ulonglong)*pppppppfVar33 ^ 0x8000000080000000);
      pppppppfVar33 = pppppppfVar33 + 1;
    } while (pppppppfVar33 != pppppppfStack_188 + (longlong)local_180);
  }
LAB_14080a516:
  if (1 < local_1a8) {
    uVar39 = (ulonglong)local_1a8 >> 1;
    if (local_1a8 < 0x10) {
      uVar41 = 0;
    }
    else {
      uVar41 = uVar39 & 0xfffffffffffffff8;
      puVar40 = (undefined4 *)((longlong)pppppppfStack_1b0 + (longlong)local_1a8 * 4 + -0x10);
      uVar43 = 0;
      do {
        puVar2 = (undefined4 *)((longlong)pppppppfStack_1b0 + uVar43 * 4);
        uVar8 = *puVar2;
        uVar11 = puVar2[1];
        uVar12 = puVar2[2];
        uVar13 = puVar2[3];
        puVar2 = (undefined4 *)((longlong)pppppppfStack_1b0 + 0x10 + uVar43 * 4);
        uVar14 = *puVar2;
        uVar15 = puVar2[1];
        uVar16 = puVar2[2];
        uVar17 = puVar2[3];
        uVar18 = puVar40[-4];
        uVar19 = puVar40[-3];
        uVar20 = puVar40[-2];
        uVar21 = puVar40[-1];
        uVar22 = *puVar40;
        uVar23 = puVar40[1];
        uVar24 = puVar40[2];
        puVar2 = (undefined4 *)((longlong)pppppppfStack_1b0 + uVar43 * 4);
        *puVar2 = puVar40[3];
        puVar2[1] = uVar24;
        puVar2[2] = uVar23;
        puVar2[3] = uVar22;
        puVar2 = (undefined4 *)((longlong)pppppppfStack_1b0 + 0x10 + uVar43 * 4);
        *puVar2 = uVar21;
        puVar2[1] = uVar20;
        puVar2[2] = uVar19;
        puVar2[3] = uVar18;
        *puVar40 = uVar13;
        puVar40[1] = uVar12;
        puVar40[2] = uVar11;
        puVar40[3] = uVar8;
        puVar40[-4] = uVar17;
        puVar40[-3] = uVar16;
        puVar40[-2] = uVar15;
        puVar40[-1] = uVar14;
        uVar43 = uVar43 + 8;
        puVar40 = puVar40 + -8;
      } while (uVar41 != uVar43);
      if (uVar39 == uVar41) goto LAB_14080a5dd;
    }
    puVar40 = (undefined4 *)((longlong)pppppppfStack_1b0 + uVar41 * 4);
    lVar32 = -uVar41;
    lVar35 = (longlong)pppppppfStack_1b0 + (longlong)local_1a8 * 4 + -4;
    do {
      uVar8 = *puVar40;
      *puVar40 = *(undefined4 *)(lVar35 + lVar32 * 4);
      *(undefined4 *)(lVar35 + lVar32 * 4) = uVar8;
      lVar32 = lVar32 + -1;
      puVar40 = puVar40 + 1;
    } while (-lVar32 != uVar39);
  }
LAB_14080a5dd:
  if (1 < (ulonglong)local_1d8._0_8_) {
    uVar39 = (ulonglong)local_1d8._0_8_ >> 1;
    if ((ulonglong)local_1d8._0_8_ < 0x10) {
      uVar41 = 0;
    }
    else {
      uVar41 = uVar39 & 0xfffffffffffffff8;
      puVar40 = (undefined4 *)(lStack_1e0 + local_1d8._0_8_ * 4 + -0x10);
      uVar43 = 0;
      do {
        puVar2 = (undefined4 *)(lStack_1e0 + uVar43 * 4);
        uVar8 = *puVar2;
        uVar11 = puVar2[1];
        uVar12 = puVar2[2];
        uVar13 = puVar2[3];
        puVar2 = (undefined4 *)(lStack_1e0 + 0x10 + uVar43 * 4);
        uVar14 = *puVar2;
        uVar15 = puVar2[1];
        uVar16 = puVar2[2];
        uVar17 = puVar2[3];
        uVar18 = puVar40[-4];
        uVar19 = puVar40[-3];
        uVar20 = puVar40[-2];
        uVar21 = puVar40[-1];
        uVar22 = *puVar40;
        uVar23 = puVar40[1];
        uVar24 = puVar40[2];
        puVar2 = (undefined4 *)(lStack_1e0 + uVar43 * 4);
        *puVar2 = puVar40[3];
        puVar2[1] = uVar24;
        puVar2[2] = uVar23;
        puVar2[3] = uVar22;
        puVar2 = (undefined4 *)(lStack_1e0 + 0x10 + uVar43 * 4);
        *puVar2 = uVar21;
        puVar2[1] = uVar20;
        puVar2[2] = uVar19;
        puVar2[3] = uVar18;
        *puVar40 = uVar13;
        puVar40[1] = uVar12;
        puVar40[2] = uVar11;
        puVar40[3] = uVar8;
        puVar40[-4] = uVar17;
        puVar40[-3] = uVar16;
        puVar40[-2] = uVar15;
        puVar40[-1] = uVar14;
        uVar43 = uVar43 + 8;
        puVar40 = puVar40 + -8;
      } while (uVar41 != uVar43);
      if (uVar39 == uVar41) goto LAB_14080a6ad;
    }
    puVar40 = (undefined4 *)(lStack_1e0 + uVar41 * 4);
    lVar32 = -uVar41;
    lVar35 = lStack_1e0 + local_1d8._0_8_ * 4 + -4;
    do {
      uVar8 = *puVar40;
      *puVar40 = *(undefined4 *)(lVar35 + lVar32 * 4);
      *(undefined4 *)(lVar35 + lVar32 * 4) = uVar8;
      lVar32 = lVar32 + -1;
      puVar40 = puVar40 + 1;
    } while (-lVar32 != uVar39);
  }
LAB_14080a6ad:
  pppppppfVar33 = local_2e0;
  pppppppuVar44 = &local_2f0;
  if ((float *******)0x1 < local_2e0) {
    pppppppfVar33 = pppppppfStack_2e8;
    pppppppuVar44 = local_2f0;
  }
  if ((float *******)0x1 < pppppppfVar33) {
    uVar39 = (ulonglong)pppppppfVar33 >> 1;
    if (&DAT_0000000f < pppppppfVar33) {
      uVar41 = uVar39 & 0xfffffffffffffff8;
      puVar40 = (undefined4 *)((longlong)pppppppuVar44 + (longlong)pppppppfVar33 * 4 + -0x10);
      uVar43 = 0;
      do {
        puVar2 = (undefined4 *)((longlong)pppppppuVar44 + uVar43 * 4);
        uVar8 = *puVar2;
        uVar11 = puVar2[1];
        uVar12 = puVar2[2];
        uVar13 = puVar2[3];
        puVar2 = (undefined4 *)((longlong)pppppppuVar44 + uVar43 * 4 + 0x10);
        uVar14 = *puVar2;
        uVar15 = puVar2[1];
        uVar16 = puVar2[2];
        uVar17 = puVar2[3];
        uVar18 = puVar40[-4];
        uVar19 = puVar40[-3];
        uVar20 = puVar40[-2];
        uVar21 = puVar40[-1];
        uVar22 = *puVar40;
        uVar23 = puVar40[1];
        uVar24 = puVar40[2];
        puVar2 = (undefined4 *)((longlong)pppppppuVar44 + uVar43 * 4);
        *puVar2 = puVar40[3];
        puVar2[1] = uVar24;
        puVar2[2] = uVar23;
        puVar2[3] = uVar22;
        puVar2 = (undefined4 *)((longlong)pppppppuVar44 + uVar43 * 4 + 0x10);
        *puVar2 = uVar21;
        puVar2[1] = uVar20;
        puVar2[2] = uVar19;
        puVar2[3] = uVar18;
        *puVar40 = uVar13;
        puVar40[1] = uVar12;
        puVar40[2] = uVar11;
        puVar40[3] = uVar8;
        puVar40[-4] = uVar17;
        puVar40[-3] = uVar16;
        puVar40[-2] = uVar15;
        puVar40[-1] = uVar14;
        uVar43 = uVar43 + 8;
        puVar40 = puVar40 + -8;
      } while (uVar41 != uVar43);
      if (uVar39 == uVar41) goto LAB_14080a77a;
    }
    else {
      uVar41 = 0;
    }
    puVar40 = (undefined4 *)((longlong)pppppppuVar44 + uVar41 * 4);
    lVar35 = -uVar41;
    lVar32 = (longlong)pppppppfVar33 * 4 + -4;
    do {
      uVar8 = *puVar40;
      *puVar40 = *(undefined4 *)((longlong)pppppppuVar44 + lVar35 * 4 + lVar32);
      *(undefined4 *)((longlong)pppppppuVar44 + lVar35 * 4 + lVar32) = uVar8;
      lVar35 = lVar35 + -1;
      puVar40 = puVar40 + 1;
    } while (-lVar35 != uVar39);
  }
LAB_14080a77a:
  in_RCX[6] = local_120;
  in_RCX[4] = local_130;
  in_RCX[5] = lStack_128;
  in_RCX[9] = local_180;
  in_RCX[7] = local_190;
  in_RCX[8] = pppppppfStack_188;
  in_RCX[0xc] = local_1a8;
  in_RCX[10] = local_1b8;
  in_RCX[0xb] = pppppppfStack_1b0;
  in_RCX[0xd] = local_1e8;
  in_RCX[0xe] = lStack_1e0;
  in_RCX[0xf] = local_1d8._0_8_;
  in_RCX[1] = local_2f0;
  in_RCX[2] = pppppppfStack_2e8;
  in_RCX[3] = local_2e0;
  *in_RCX = 1;
  return;
}

