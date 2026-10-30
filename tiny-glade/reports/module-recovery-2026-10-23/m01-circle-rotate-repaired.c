
void __cdecl country_core::systems::wall::shapes::circle::ui_rotate_circle::ui_rotate_circle(void)

{
  char *pcVar1;
  float *pfVar2;
  undefined8 uVar3;
  uint *puVar4;
  undefined8 *puVar5;
  longlong *plVar6;
  longlong lVar7;
  undefined4 *puVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  uint uVar12;
  uint uVar13;
  bool bVar14;
  undefined2 uVar15;
  longlong lVar16;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  float *pfVar17;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 *extraout_RAX_04;
  longlong lVar18;
  undefined8 *extraout_RAX_05;
  void *extraout_RAX_06;
  undefined8 *in_RCX;
  longlong in_RDX;
  ulonglong uVar19;
  longlong **pplVar20;
  code *pcVar21;
  longlong *in_R8;
  longlong lVar22;
  uint uVar23;
  longlong in_R9;
  ushort uVar24;
  longlong lVar25;
  ulonglong uVar26;
  void *_Dst;
  longlong lVar27;
  byte bVar28;
  char cVar29;
  char cVar34;
  float fVar30;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  undefined4 extraout_XMM0_Da_03;
  undefined4 extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float extraout_XMM0_Da_06;
  float extraout_XMM0_Da_07;
  float extraout_XMM0_Da_08;
  char cVar32;
  char cVar33;
  float fVar35;
  undefined1 auVar31 [16];
  float fVar36;
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined4 uVar43;
  float fVar44;
  float fVar45;
  undefined4 uVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  longlong in_stack_00000028;
  undefined8 *in_stack_00000030;
  longlong in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000050;
  longlong *in_stack_00000058;
  longlong *in_stack_00000060;
  longlong *in_stack_00000068;
  float *in_stack_00000070;
  byte *in_stack_00000078;
  longlong *in_stack_00000080;
  longlong *local_338;
  longlong lStack_330;
  longlong local_328;
  longlong **pplStack_320;
  void *pvStack_318;
  size_t sStack_310;
  float local_300;
  undefined4 uStack_2fc;
  undefined8 uStack_2f8;
  undefined8 local_2f0;
  longlong *local_2e8;
  code *pcStack_2e0;
  undefined1 *local_2d8;
  code *pcStack_2d0;
  float *pfStack_2c8;
  code *pcStack_2c0;
  undefined8 local_2b0;
  undefined8 local_2a8;
  undefined8 local_2a0;
  undefined4 *local_298;
  longlong local_290;
  char local_288 [2];
  undefined2 uStack_286;
  float fStack_284;
  undefined2 uStack_280;
  undefined2 uStack_27e;
  undefined4 uStack_27c;
  undefined1 local_278;
  undefined1 uStack_277;
  undefined2 uStack_276;
  float fStack_274;
  undefined8 local_270;
  undefined4 uStack_268;
  undefined4 uStack_264;
  ulonglong uStack_260;
  ulonglong local_258;
  undefined8 uStack_250;
  undefined4 uStack_248;
  undefined2 uStack_244;
  undefined4 local_240;
  undefined4 local_228;
  undefined1 local_210;
  undefined1 local_200;
  undefined8 local_1ff;
  undefined7 uStack_1f7;
  undefined1 uStack_1f0;
  undefined7 uStack_1ef;
  undefined2 local_1e0;
  undefined8 local_190;
  longlong **local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  undefined1 auStack_164 [8];
  undefined4 uStack_15c;
  float fStack_158;
  undefined4 local_154;
  undefined8 *local_150;
  ulonglong local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined4 uStack_130;
  undefined4 uStack_12c;
  undefined7 uStack_128;
  undefined1 local_121;
  undefined7 uStack_120;
  undefined1 uStack_119;
  undefined *puStack_118;
  void *local_108;
  longlong local_100;
  undefined1 local_f2;
  undefined1 uStack_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  uVar23 = *(uint *)*in_RCX;
  if (uVar23 < 6) {
    local_2a8 = *(undefined8 *)(in_R9 + 0x40);
    local_298 = *(undefined4 **)(in_R9 + 0x50);
    local_154 = *(undefined4 *)(in_R9 + 0x5c);
    local_2a0 = *(undefined8 *)(in_R9 + 0x60);
    local_2b0 = *(undefined8 *)(in_stack_00000028 + 0x20);
    puVar4 = (uint *)*in_stack_00000030;
    puVar5 = (undefined8 *)in_stack_00000030[4];
    local_150 = (undefined8 *)in_stack_00000030[8];
    if ((uVar23 == 1) || (uVar23 == 2)) {
      lVar18 = *(longlong *)((uint *)*in_RCX + 2);
      plVar6 = (longlong *)*in_R8;
      local_288[0] = (char)lVar18;
      local_288[1] = (char)((ulonglong)lVar18 >> 8);
      uStack_286 = (undefined2)((ulonglong)lVar18 >> 0x10);
      fStack_284 = (float)((ulonglong)lVar18 >> 0x20);
      local_100 = lVar18;
      if (plVar6[3] != 0) {
        local_290 = in_RDX;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar7 = local_100;
        lVar27 = *plVar6;
        bVar28 = (byte)(extraout_RAX >> 0x39);
        auVar31 = ZEXT216(CONCAT11(bVar28,bVar28));
        auVar31 = pshuflw(auVar31,auVar31,0);
        lVar22 = 0;
        uVar19 = extraout_RAX;
        while( true ) {
          uVar19 = plVar6[1] & uVar19;
          pcVar1 = (char *)(lVar27 + uVar19);
          cVar29 = auVar31[0];
          auVar40[0] = -(*pcVar1 == cVar29);
          cVar32 = auVar31[1];
          auVar40[1] = -(pcVar1[1] == cVar32);
          cVar33 = auVar31[2];
          auVar40[2] = -(pcVar1[2] == cVar33);
          cVar34 = auVar31[3];
          auVar40[3] = -(pcVar1[3] == cVar34);
          auVar40[4] = -(pcVar1[4] == cVar29);
          auVar40[5] = -(pcVar1[5] == cVar32);
          auVar40[6] = -(pcVar1[6] == cVar33);
          auVar40[7] = -(pcVar1[7] == cVar34);
          auVar40[8] = -(pcVar1[8] == cVar29);
          auVar40[9] = -(pcVar1[9] == cVar32);
          auVar40[10] = -(pcVar1[10] == cVar33);
          auVar40[0xb] = -(pcVar1[0xb] == cVar34);
          auVar40[0xc] = -(pcVar1[0xc] == cVar29);
          auVar40[0xd] = -(pcVar1[0xd] == cVar32);
          auVar40[0xe] = -(pcVar1[0xe] == cVar33);
          auVar40[0xf] = -(pcVar1[0xf] == cVar34);
          uVar24 = (ushort)(SUB161(auVar40 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar40[0xf] >> 7) << 0xf;
          uVar23 = (uint)uVar24;
          while (uVar24 != 0) {
            uVar12 = 0;
            for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
              uVar12 = uVar12 + 1;
            }
            lVar25 = (uVar12 + uVar19 & plVar6[1]) * -0x2a8;
            if (*(longlong *)(lVar27 + -0x2a8 + lVar25) == lVar18) {
              if ((*(ulonglong *)(lVar27 + -0x290 + lVar25) < 2) ||
                 (*(float *)(lVar27 + -0x270 + lVar25) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
                core::panicking::panic();
              }
              lVar27 = lVar27 + lVar25;
              if (*(int *)(lVar27 + -0x48) != 0) {
                return;
              }
              local_148 = *(ulonglong *)(lVar27 + -0x44);
              uStack_140 = *(undefined8 *)(lVar27 + -0x3c);
              local_288[0] = (char)local_100;
              local_288[1] = (char)((ulonglong)local_100 >> 8);
              uStack_286 = (undefined2)((ulonglong)local_100 >> 0x10);
              fStack_284 = (float)((ulonglong)local_100 >> 0x20);
              if (plVar6[3] == 0) goto LAB_1409ad197;
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
              fVar36 = __real_3f800000;
              lVar18 = *plVar6;
              bVar28 = (byte)(extraout_RAX_00 >> 0x39);
              auVar31 = ZEXT216(CONCAT11(bVar28,bVar28));
              auVar31 = pshuflw(auVar31,auVar31,0);
              lVar22 = 0;
              uVar19 = extraout_RAX_00;
              goto LAB_1409acbc6;
            }
            uVar24 = (ushort)(uVar23 - 1) & (ushort)uVar23;
            uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar24);
          }
          auVar37[0] = -(*pcVar1 == -1);
          auVar37[1] = -(pcVar1[1] == -1);
          auVar37[2] = -(pcVar1[2] == -1);
          auVar37[3] = -(pcVar1[3] == -1);
          auVar37[4] = -(pcVar1[4] == -1);
          auVar37[5] = -(pcVar1[5] == -1);
          auVar37[6] = -(pcVar1[6] == -1);
          auVar37[7] = -(pcVar1[7] == -1);
          auVar37[8] = -(pcVar1[8] == -1);
          auVar37[9] = -(pcVar1[9] == -1);
          auVar37[10] = -(pcVar1[10] == -1);
          auVar37[0xb] = -(pcVar1[0xb] == -1);
          auVar37[0xc] = -(pcVar1[0xc] == -1);
          auVar37[0xd] = -(pcVar1[0xd] == -1);
          auVar37[0xe] = -(pcVar1[0xe] == -1);
          auVar37[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar37 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar37 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar37 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar37 >> 0x2f,0) & 1) != 0) || (SUB161(auVar37 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar37 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar37 >> 0x47,0) & 1) != 0) || (SUB161(auVar37 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar37 >> 0x57,0) & 1) != 0) || (SUB161(auVar37 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar37 >> 0x67,0) & 1) != 0) || (SUB161(auVar37 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar37 >> 0x77,0) & 1) != 0) || auVar37[0xf] < '\0') break;
          uVar19 = uVar19 + lVar22 + 0x10;
          lVar22 = lVar22 + 0x10;
        }
      }
                    /* WARNING: Subroutine does not return */
      core::option::unwrap_failed();
    }
  }
  in_stack_00000078[0] = 0;
  in_stack_00000078[1] = 0;
  in_stack_00000078[2] = 0;
  in_stack_00000078[3] = 0;
  return;
LAB_1409acbc6:
  uVar19 = plVar6[1] & uVar19;
  pcVar1 = (char *)(lVar18 + uVar19);
  cVar29 = auVar31[0];
  auVar41[0] = -(*pcVar1 == cVar29);
  cVar32 = auVar31[1];
  auVar41[1] = -(pcVar1[1] == cVar32);
  cVar33 = auVar31[2];
  auVar41[2] = -(pcVar1[2] == cVar33);
  cVar34 = auVar31[3];
  auVar41[3] = -(pcVar1[3] == cVar34);
  auVar41[4] = -(pcVar1[4] == cVar29);
  auVar41[5] = -(pcVar1[5] == cVar32);
  auVar41[6] = -(pcVar1[6] == cVar33);
  auVar41[7] = -(pcVar1[7] == cVar34);
  auVar41[8] = -(pcVar1[8] == cVar29);
  auVar41[9] = -(pcVar1[9] == cVar32);
  auVar41[10] = -(pcVar1[10] == cVar33);
  auVar41[0xb] = -(pcVar1[0xb] == cVar34);
  auVar41[0xc] = -(pcVar1[0xc] == cVar29);
  auVar41[0xd] = -(pcVar1[0xd] == cVar32);
  auVar41[0xe] = -(pcVar1[0xe] == cVar33);
  auVar41[0xf] = -(pcVar1[0xf] == cVar34);
  uVar24 = (ushort)(SUB161(auVar41 >> 7,0) & 1) | (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe | (ushort)(auVar41[0xf] >> 7) << 0xf;
  uVar23 = (uint)uVar24;
  if (uVar24 != 0) {
    while( true ) {
      uVar12 = 0;
      for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
        uVar12 = uVar12 + 1;
      }
      lVar25 = (uVar12 + uVar19 & plVar6[1]) * -0x2a8;
      if (*(longlong *)(lVar18 + -0x2a8 + lVar25) == lVar7) break;
      uVar24 = (ushort)(uVar23 - 1) & (ushort)uVar23;
      uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar24);
      if (uVar24 == 0) goto LAB_1409acc06;
    }
    if ((*(ulonglong *)(lVar18 + -0x290 + lVar25) < 2) ||
       (*(float *)(lVar18 + -0x270 + lVar25) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic();
    }
    lVar7 = *(longlong *)(lVar18 + lVar25 + -600);
    if (lVar7 == 0) {
                    /* WARNING: Subroutine does not return */
      core::panicking::panic_bounds_check();
    }
    pfVar17 = *(float **)(lVar18 + lVar25 + -0x260);
    pfVar2 = pfVar17 + lVar7 * 3;
    fVar44 = pfVar17[1];
    local_108 = (void *)CONCAT44(local_108._4_4_,(float)*puVar4);
    local_188 = (longlong **)*puVar5;
    local_178 = (undefined8 *)CONCAT44(pfVar17[2],*pfVar17);
    fVar30 = __real_4b189680;
    do {
      fVar35 = *pfVar17;
      fVar50 = pfVar17[1];
      fVar45 = pfVar17[2];
      fVar47 = fVar45 * *(float *)(puVar5 + 0x14) +
               fVar50 * *(float *)(puVar5 + 0x12) + fVar35 * *(float *)(puVar5 + 0x10) +
               *(float *)(puVar5 + 0x16);
      fVar48 = fVar45 * *(float *)((longlong)puVar5 + 0xa4) +
               fVar50 * *(float *)((longlong)puVar5 + 0x94) +
               fVar35 * *(float *)((longlong)puVar5 + 0x84) + *(float *)((longlong)puVar5 + 0xb4);
      fVar49 = fVar45 * *(float *)(puVar5 + 0x15) +
               fVar50 * *(float *)(puVar5 + 0x13) + fVar35 * *(float *)(puVar5 + 0x11) +
               *(float *)(puVar5 + 0x17);
      uStack_180._4_4_ = (float)((ulonglong)puVar5[1] >> 0x20);
      fVar47 = (((fVar49 * *(float *)(puVar5 + 4) +
                  fVar48 * *(float *)(puVar5 + 2) + fVar47 * local_188._0_4_ +
                 *(float *)(puVar5 + 6)) /
                (fVar49 * *(float *)((longlong)puVar5 + 0x2c) +
                 fVar48 * *(float *)((longlong)puVar5 + 0x1c) + fVar47 * uStack_180._4_4_ +
                *(float *)((longlong)puVar5 + 0x3c))) * __real_3f000000 + __real_3f000000) *
               (float)*puVar4;
      if (fVar47 < fVar30) {
        local_178 = (undefined8 *)CONCAT44(fVar45,fVar35);
      }
      uStack_170 = 0;
      fVar44 = (float)(~-(uint)(fVar47 < fVar30) & (uint)fVar44 |
                      (uint)fVar50 & -(uint)(fVar47 < fVar30));
      if (fVar30 <= fVar47) {
        fVar47 = fVar30;
      }
      fVar30 = fVar47;
      pfVar17 = pfVar17 + 3;
    } while (pfVar17 != pfVar2);
    fVar47 = (float)local_148;
    fVar49 = (float)(local_148 >> 0x20);
    fVar50 = (float)local_178;
    fVar45 = local_178._4_4_;
    fVar30 = (float)local_178 - fVar47;
    fVar35 = local_178._4_4_ - fVar49;
    fVar48 = __real_3f800000 / SQRT(fVar35 * fVar35 + fVar30 * fVar30);
    local_190 = CONCAT44(fVar48 * fVar35,fVar48 * fVar30);
    uStack_180 = puVar5[1];
    ::utils::swizzlers::impl$1::xvy();
    fVar45 = fVar45 + (float)CONCAT22(uStack_27e,uStack_280);
    fVar30 = (float)CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0]));
    fVar44 = fVar44 + fStack_284;
    ::utils::xz_rotation_bad();
    fVar35 = fStack_284;
    uVar15 = uStack_286;
    cVar32 = local_288[1];
    cVar29 = local_288[0];
    ::utils::swizzlers::impl$1::xvy();
    fVar30 = (float)CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0])) *
             (float)__xmm_00000000000000003e99999a3e99999a + fVar50 + fVar30;
    fStack_274 = fStack_284 * __xmm_00000000000000003e99999a3e99999a._4_4_ + fVar44;
    local_270._0_4_ = (float)CONCAT22(uStack_27e,uStack_280) * __real_3e99999a + fVar45;
    local_278 = SUB41(fVar30,0);
    uStack_277 = (undefined1)((uint)fVar30 >> 8);
    uStack_276 = (undefined2)((uint)fVar30 >> 0x10);
    local_270._4_4_ = 0x3f800000;
    uStack_268 = 0x3f800000;
    uStack_264 = 0x3f800000;
    local_288[0] = cVar29;
    local_288[1] = cVar32;
    uStack_286 = uVar15;
    fStack_284 = fVar35;
    rectangle::ui_edit_rectangle_orientation::box_trigger_volume();
    local_138 = &local_100;
    uStack_130 = 0x40b34ec0;
    uStack_12c = 1.4013e-45;
    local_288[0] = '\x10';
    local_288[1] = 0xdd;
    uStack_286 = 0x42ab;
    fStack_284 = 1.4013e-45;
    uStack_280 = 1;
    uStack_27e = 0;
    uStack_27c = 0.0;
    local_278 = SUB81(&local_138,0);
    uStack_277 = (undefined1)((ulonglong)&local_138 >> 8);
    uStack_276 = (undefined2)((ulonglong)&local_138 >> 0x10);
    fStack_274 = (float)((ulonglong)&local_138 >> 0x20);
    local_270._0_4_ = 1.4013e-45;
    local_270._4_4_ = 0;
    uStack_268 = 0;
    uStack_264 = 0;
    local_f2 = 1;
    alloc::fmt::format::format_inner();
    local_2e8 = local_338;
    pcStack_2e0 = (code *)lStack_330;
    local_2d8 = (undefined1 *)local_328;
    local_138._7_1_ = SUB41(local_300,0);
    uStack_130 = (undefined4)(CONCAT44(uStack_2fc,local_300) >> 8);
    uStack_12c._3_1_ = (undefined1)uStack_2f8;
    uStack_12c = (float)CONCAT13(uStack_12c._3_1_,(int3)((uint)uStack_2fc >> 8));
    uStack_128 = (undefined7)((ulonglong)uStack_2f8 >> 8);
    local_121 = (undefined1)local_2f0;
    uStack_120 = (undefined7)((ulonglong)local_2f0 >> 8);
    local_1ff = CONCAT44(local_138._4_4_,(undefined4)local_138);
    uStack_1f7 = (undefined7)CONCAT44(uStack_12c,uStack_130);
    uStack_1f0 = uStack_12c._3_1_;
    uStack_1ef = uStack_128;
    local_288[0] = '\0';
    local_288[1] = 0;
    uStack_286 = 0;
    fStack_284 = 0.0;
    uStack_280 = 8;
    uStack_27e = 0;
    uStack_27c = 0.0;
    local_278 = 0;
    uStack_277 = 0;
    uStack_276 = 0;
    fStack_274 = 0.0;
    local_270._0_4_ = 1.4013e-45;
    uStack_268 = 1;
    uStack_264 = 0;
    local_258 = CONCAT44(local_258._4_4_,2);
    local_240 = 3;
    local_228 = 3;
    local_210 = 0;
    local_200 = 2;
    local_1e0 = 0x100;
    local_f2 = 0;
    pplVar20 = &local_2e8;
    systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
    lVar18 = local_100;
    uVar46 = local_154;
    puVar8 = local_298;
    cVar29 = '\x02';
    if ((1 < *(byte *)(extraout_RAX_01 + 0x18)) &&
       (*(longlong ***)(extraout_RAX_01 + 0x38) == pplVar20)) {
      cVar29 = *(char *)(extraout_RAX_01 + 0x19);
    }
    bVar28 = 3;
    if ((byte)(cVar29 - 2U) < 3) {
      bVar28 = cVar29 - 2U;
    }
    local_188 = pplVar20;
    local_108 = (void *)extraout_RAX_01;
                    /* WARNING: Switch is manually overridden */
    switch(bVar28) {
    case 1:
      *local_298 = local_154;
      resources::walls::decorator_archivist::DecoratorArchivist::
      archive_current_decorators_in_storage<ref_<str_>_>();
      *puVar8 = uVar46;
      resources::walls::decorator_archivist::DecoratorArchivist::decorator_cull_allow_wall();
      fVar45 = fVar45 - fVar49;
      uVar9 = *local_150;
      uVar3 = *(undefined8 *)((longlong)local_150 + 0xc);
      fVar30 = (float)((ulonglong)uVar9 >> 0x20);
      fVar35 = (float)((ulonglong)uVar3 >> 0x20);
      fVar44 = (fVar44 - fVar30) / fVar35;
      fVar50 = fVar44 * *(float *)((longlong)local_150 + 0x14) + *(float *)(local_150 + 1);
      fVar36 = fVar50 - fVar49;
      ::utils::cartesian_to_polar();
      ::utils::circular_f32::CircularF32::as_f32();
      ::utils::cartesian_to_polar();
      *in_stack_00000070 = fVar45 - fVar36;
      in_stack_00000070[1] = fVar36;
      in_stack_00000070[2] = extraout_XMM0_Da;
      fVar36 = __real_3f800000;
      *(ulonglong *)(in_stack_00000070 + 3) =
           CONCAT44(fVar44 * fVar35 + fVar30,fVar44 * (float)uVar3 + (float)uVar9);
      in_stack_00000070[5] = fVar50;
      *(undefined1 *)(in_stack_00000070 + 6) = 0;
      break;
    case 2:
      uVar9 = *local_150;
      uVar3 = *(undefined8 *)((longlong)local_150 + 0xc);
      fVar35 = (float)((ulonglong)uVar9 >> 0x20);
      fVar50 = (float)((ulonglong)uVar3 >> 0x20);
      fVar45 = (in_stack_00000070[4] - fVar35) / fVar50;
      fVar44 = *(float *)((longlong)local_150 + 0x14);
      fVar30 = *(float *)(local_150 + 1);
      ::utils::cartesian_to_polar();
      ::utils::circular_f32::CircularF32::new();
      if ((extraout_XMM0_Da_00 != uStack_140._4_4_) ||
         (NAN(extraout_XMM0_Da_00) || NAN(uStack_140._4_4_))) {
        fStack_158 = extraout_XMM0_Da_00;
        ::utils::swizzlers::impl$1::xvy();
        lVar18 = local_100;
        ::utils::circular_f32::CircularF32::distance();
        fVar48 = 0.0;
        ::utils::inverse_lerp();
        if (0.0 <= extraout_XMM0_Da_01) {
          fVar48 = extraout_XMM0_Da_01;
        }
        fStack_274 = fVar36;
        if (fVar48 <= fVar36) {
          fStack_274 = fVar48;
        }
        local_270._0_4_ = (float)CONCAT31(local_270._1_3_,2);
        local_288[0] = (char)lVar18;
        local_288[1] = (char)((ulonglong)lVar18 >> 8);
        uStack_286 = (undefined2)((ulonglong)lVar18 >> 0x10);
        fStack_284 = (float)((ulonglong)lVar18 >> 0x20);
        *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
        lVar18 = *in_stack_00000060;
        lVar7 = *(longlong *)(lVar18 + 0x30);
        uVar10 = *(undefined8 *)(lVar18 + 0x40);
        if (lVar7 == *(longlong *)(lVar18 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                    ();
        }
        lVar22 = *(longlong *)(lVar18 + 0x28);
        *(undefined8 *)(lVar22 + lVar7 * 0x28) = uVar10;
        puVar5 = (undefined8 *)(lVar22 + 0x18 + lVar7 * 0x28);
        *puVar5 = CONCAT44(fStack_274,CONCAT22(uStack_276,CONCAT11(uStack_277,local_278)));
        puVar5[1] = CONCAT44(local_270._4_4_,(float)local_270);
        puVar5 = (undefined8 *)(lVar22 + 8 + lVar7 * 0x28);
        *puVar5 = CONCAT44(fStack_284,CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0])));
        puVar5[1] = CONCAT44(uStack_27c,CONCAT22(uStack_27e,uStack_280));
        *(longlong *)(lVar18 + 0x30) = lVar7 + 1;
        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
        uStack_130 = (float)uStack_140;
        local_138._0_4_ = (undefined4)local_148;
        local_138._4_4_ = (undefined4)(local_148 >> 0x20);
        uStack_12c = fStack_158;
        uVar46 = *(undefined4 *)(local_290 + 0x1c);
        puVar8 = *(undefined4 **)(local_290 + 0x10);
        *puVar8 = uVar46;
        ::utils::geometry::circle::Circle2d::as_curve2();
        resources::walls::inner_walls::InnerWalls::
        _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_curve::closure_env_0>
                  ();
        *puVar8 = uVar46;
        fStack_284 = (float)(undefined4)local_138;
        uStack_280 = (undefined2)local_138._4_4_;
        uStack_27e = (undefined2)((uint)local_138._4_4_ >> 0x10);
        uStack_27c = (float)uStack_130;
        local_278 = SUB41(uStack_12c,0);
        uStack_277 = (undefined1)((uint)uStack_12c >> 8);
        uStack_276 = (undefined2)((uint)uStack_12c >> 0x10);
        local_288[0] = '\0';
        local_288[1] = 0;
        uStack_286 = 0;
        resources::walls::inner_walls::InnerWalls::
        _try_update_state<country_core::resources::walls::inner_walls::impl_1::update_wall_state_enclosure::closure_env_0>
                  ();
        lVar18 = local_100;
        local_288[0] = (char)local_100;
        local_288[1] = (char)((ulonglong)local_100 >> 8);
        uStack_286 = (undefined2)((ulonglong)local_100 >> 0x10);
        fStack_284 = (float)((ulonglong)local_100 >> 0x20);
        if (plVar6[9] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
          lVar7 = plVar6[6];
          bVar28 = (byte)(extraout_RAX_02 >> 0x39);
          auVar31 = ZEXT216(CONCAT11(bVar28,bVar28));
          auVar31 = pshuflw(auVar31,auVar31,0);
          lVar22 = 0;
          uVar19 = extraout_RAX_02;
          while( true ) {
            uVar19 = plVar6[7] & uVar19;
            pcVar1 = (char *)(lVar7 + uVar19);
            cVar29 = auVar31[0];
            auVar42[0] = -(*pcVar1 == cVar29);
            cVar32 = auVar31[1];
            auVar42[1] = -(pcVar1[1] == cVar32);
            cVar33 = auVar31[2];
            auVar42[2] = -(pcVar1[2] == cVar33);
            cVar34 = auVar31[3];
            auVar42[3] = -(pcVar1[3] == cVar34);
            auVar42[4] = -(pcVar1[4] == cVar29);
            auVar42[5] = -(pcVar1[5] == cVar32);
            auVar42[6] = -(pcVar1[6] == cVar33);
            auVar42[7] = -(pcVar1[7] == cVar34);
            auVar42[8] = -(pcVar1[8] == cVar29);
            auVar42[9] = -(pcVar1[9] == cVar32);
            auVar42[10] = -(pcVar1[10] == cVar33);
            auVar42[0xb] = -(pcVar1[0xb] == cVar34);
            auVar42[0xc] = -(pcVar1[0xc] == cVar29);
            auVar42[0xd] = -(pcVar1[0xd] == cVar32);
            auVar42[0xe] = -(pcVar1[0xe] == cVar33);
            auVar42[0xf] = -(pcVar1[0xf] == cVar34);
            uVar24 = (ushort)(SUB161(auVar42 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar42[0xf] >> 7) << 0xf;
            uVar23 = (uint)uVar24;
            while (uVar24 != 0) {
              uVar12 = 0;
              for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
                uVar12 = uVar12 + 1;
              }
              uVar26 = uVar12 + uVar19 & plVar6[7];
              if (*(longlong *)(lVar7 + -0x10 + uVar26 * -0x10) == lVar18) {
                uVar19 = *(ulonglong *)(lVar7 + -8 + uVar26 * -0x10);
                if (uVar19 >> 0x20 != 0) {
                  uStack_280 = (undefined2)local_100;
                  uStack_27e = (undefined2)((ulonglong)local_100 >> 0x10);
                  uStack_27c = (float)((ulonglong)local_100 >> 0x20);
                  local_278 = 2;
                  local_288[0] = (char)uVar19;
                  local_288[1] = (char)(uVar19 >> 8);
                  uStack_286 = (undefined2)(uVar19 >> 0x10);
                  fStack_284 = (float)(uVar19 >> 0x20);
                  bevy_ecs::event::writer::EventWriter::
                  write<country_core::systems::audio::wall_audio_orchestrator::RequestWallMaterialAudio>
                            ();
                }
                goto code_r0x0001409ad71e;
              }
              uVar24 = (ushort)(uVar23 - 1) & (ushort)uVar23;
              uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar24);
            }
            auVar39[0] = -(*pcVar1 == -1);
            auVar39[1] = -(pcVar1[1] == -1);
            auVar39[2] = -(pcVar1[2] == -1);
            auVar39[3] = -(pcVar1[3] == -1);
            auVar39[4] = -(pcVar1[4] == -1);
            auVar39[5] = -(pcVar1[5] == -1);
            auVar39[6] = -(pcVar1[6] == -1);
            auVar39[7] = -(pcVar1[7] == -1);
            auVar39[8] = -(pcVar1[8] == -1);
            auVar39[9] = -(pcVar1[9] == -1);
            auVar39[10] = -(pcVar1[10] == -1);
            auVar39[0xb] = -(pcVar1[0xb] == -1);
            auVar39[0xc] = -(pcVar1[0xc] == -1);
            auVar39[0xd] = -(pcVar1[0xd] == -1);
            auVar39[0xe] = -(pcVar1[0xe] == -1);
            auVar39[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar39 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar39 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar39 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar39 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar39 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar39 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar39 >> 0x3f,0) & 1) != 0) || (SUB161(auVar39 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar39 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar39 >> 0x57,0) & 1) != 0) || (SUB161(auVar39 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar39 >> 0x67,0) & 1) != 0) || (SUB161(auVar39 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar39 >> 0x77,0) & 1) != 0) || auVar39[0xf] < '\0') break;
            uVar19 = uVar19 + lVar22 + 0x10;
            lVar22 = lVar22 + 0x10;
          }
        }
      }
code_r0x0001409ad71e:
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_04 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      local_178 = extraout_RAX_04;
      ::utils::swizzlers::impl$1::xvy();
      *local_178 = CONCAT44(fVar45 * fVar50 + fVar35,fVar45 * (float)uVar3 + (float)uVar9);
      *(float *)(local_178 + 1) = fVar45 * fVar44 + fVar30;
      *(uint *)((longlong)local_178 + 0x14) = CONCAT22(uStack_27e,uStack_280);
      *(ulonglong *)((longlong)local_178 + 0xc) =
           CONCAT44(fStack_284,CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0])));
      local_258 = local_258 & 0xffffffff00000000;
      uStack_248 = 0x3f800000;
      local_288[0] = '\x02';
      local_288[1] = 0;
      uStack_286 = 0;
      fStack_284 = 0.0;
      uStack_280 = SUB82(local_178,0);
      uStack_27e = (undefined2)((ulonglong)local_178 >> 0x10);
      uStack_27c = (float)((ulonglong)local_178 >> 0x20);
      local_278 = 2;
      uStack_277 = 0;
      uStack_276 = 0;
      fStack_274 = 0.0;
      local_270._0_4_ = 0.0;
      local_270._4_4_ = 0x80000000;
      uStack_244 = 1;
      country_core::utils::debug_viz::DebugLine::show();
      lVar22 = local_100;
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar18 = *in_stack_00000040;
      lVar7 = *(longlong *)(lVar18 + 0x30);
      uVar9 = *(undefined8 *)(lVar18 + 0x40);
      if (lVar7 == *(longlong *)(lVar18 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar16 = local_100;
      lVar25 = *(longlong *)(lVar18 + 0x28);
      *(undefined8 *)(lVar25 + lVar7 * 0x10) = uVar9;
      *(longlong *)(lVar25 + 8 + lVar7 * 0x10) = lVar22;
      *(longlong *)(lVar18 + 0x30) = lVar7 + 1;
      *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      if ((*(byte *)(lVar27 + -0x198) & 1) == 0) {
        local_288[0] = -0x60;
        local_288[1] = 0xae;
        uStack_286 = 0x42ae;
        fStack_284 = 1.4013e-45;
        uStack_280 = 1;
        uStack_27e = 0;
        uStack_27c = 0.0;
        local_278 = 8;
        uStack_277 = 0;
        uStack_276 = 0;
        fStack_274 = 0.0;
        local_270._0_4_ = 0.0;
        local_270._4_4_ = 0;
        uStack_268 = 0;
        uStack_264 = 0;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      uVar46 = *(undefined4 *)(lVar27 + -0x20);
      uVar43 = *(undefined4 *)(lVar27 + -0x194);
      uStack_286 = (undefined2)local_148;
      fStack_284 = (float)(local_148 >> 0x10);
      uStack_280 = (undefined2)(local_148 >> 0x30);
      uStack_27e = (undefined2)uStack_140;
      uStack_27c = (float)((ulonglong)uStack_140 >> 0x10);
      local_278 = (undefined1)((ulonglong)uStack_140 >> 0x30);
      uStack_277 = (undefined1)((ulonglong)uStack_140 >> 0x38);
      *(undefined4 *)in_stack_00000050[2] = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
      lVar18 = *in_stack_00000050;
      lVar27 = *(longlong *)(lVar18 + 0x30);
      uVar9 = *(undefined8 *)(lVar18 + 0x40);
      if (lVar27 == *(longlong *)(lVar18 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
      }
      lVar7 = *(longlong *)(lVar18 + 0x28);
      lVar22 = lVar27 * 0x40;
      *(undefined2 *)(lVar7 + lVar22) = 0x100;
      puVar5 = (undefined8 *)(lVar7 + 2 + lVar22);
      *puVar5 = CONCAT44(fStack_284,CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0])));
      puVar5[1] = CONCAT44(uStack_27c,CONCAT22(uStack_27e,uStack_280));
      *(ushort *)(lVar7 + 0x12 + lVar22) = CONCAT11(uStack_277,local_278);
      *(undefined4 *)(lVar7 + 0x14 + lVar22) = uVar46;
      *(undefined4 *)(lVar7 + 0x18 + lVar22) = uVar43;
      *(undefined8 *)(lVar7 + 0x20 + lVar22) = 1;
      *(longlong *)(lVar7 + 0x28 + lVar22) = lVar16;
      *(undefined8 *)(lVar7 + 0x38 + lVar22) = uVar9;
      *(longlong *)(lVar18 + 0x30) = lVar27 + 1;
      *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      ::utils::cartesian_to_polar();
      ::utils::circular_f32::CircularF32::new();
      uVar46 = *(undefined4 *)(in_stack_00000078 + 4);
      uVar43 = extraout_XMM0_Da_04;
      if (*(int *)in_stack_00000078 == 0) goto LAB_1409ad946;
      goto LAB_1409ad935;
    case 3:
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar27 = *in_stack_00000040;
      lVar7 = *(longlong *)(lVar27 + 0x30);
      uVar9 = *(undefined8 *)(lVar27 + 0x40);
      if (lVar7 == *(longlong *)(lVar27 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar25 = local_100;
      lVar22 = *(longlong *)(lVar27 + 0x28);
      *(undefined8 *)(lVar22 + lVar7 * 0x10) = uVar9;
      *(longlong *)(lVar22 + 8 + lVar7 * 0x10) = lVar18;
      *(longlong *)(lVar27 + 0x30) = lVar7 + 1;
      *(longlong *)(lVar27 + 0x40) = *(longlong *)(lVar27 + 0x40) + 1;
      local_2e8 = (longlong *)0x0;
      pcStack_2e0 = (code *)0x8;
      local_2d8 = (undefined1 *)0x0;
      local_138._0_4_ = (undefined4)local_148;
      local_138._4_4_ = (undefined4)(local_148 >> 0x20);
      uStack_130 = (undefined4)uStack_140;
      uStack_12c = (float)((ulonglong)uStack_140 >> 0x20);
      uStack_f1 = 1;
      ::utils::circular_f32::CircularF32::set_value();
      fStack_274 = (float)(undefined4)local_138;
      local_270._0_4_ = (float)local_138._4_4_;
      local_270._4_4_ = uStack_130;
      uStack_268 = uStack_12c;
      local_258 = local_148;
      uStack_250 = uStack_140;
      uStack_280 = (undefined2)lVar25;
      uStack_27e = (undefined2)((ulonglong)lVar25 >> 0x10);
      uStack_27c = (float)((ulonglong)lVar25 >> 0x20);
      local_278 = 0;
      uStack_277 = 0;
      uStack_276 = 0;
      uStack_260 = (ulonglong)(uint)uStack_260;
      local_288[0] = '\x04';
      local_288[1] = 0;
      uStack_286 = 0;
      fStack_284 = -0.0;
      uStack_f1 = 1;
      resources::history::wall_compound_edit::CompoundEditBuilder::next();
      **(undefined4 **)(in_stack_00000038 + 0x10) = *(undefined4 *)(in_stack_00000038 + 0x1c);
      local_138._0_4_ = 0;
      local_138._4_4_ = 0;
      uStack_130 = 8;
      uStack_12c = 0.0;
      uStack_128 = 0;
      local_121 = 0;
      local_270 = (longlong)pcStack_2e0 + (longlong)local_2d8 * 0x18;
      local_288[0] = (char)pcStack_2e0;
      local_288[1] = (char)((ulonglong)pcStack_2e0 >> 8);
      uStack_286 = (undefined2)((ulonglong)pcStack_2e0 >> 0x10);
      fStack_284 = (float)((ulonglong)pcStack_2e0 >> 0x20);
      uStack_280 = SUB82(pcStack_2e0,0);
      local_278 = SUB81(local_2e8,0);
      uStack_277 = (undefined1)((ulonglong)local_2e8 >> 8);
      uStack_276 = (undefined2)((ulonglong)local_2e8 >> 0x10);
      fStack_274 = (float)((ulonglong)local_2e8 >> 0x20);
      pcVar21 = pcStack_2e0;
      uStack_27e = uStack_286;
      uStack_27c = fStack_284;
      alloc::boxed::iter::impl$13::
      from_iter<country_core::resources::history::wall_compound_edit::ConcurrentEdits,core::iter::adapters::map::Map<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>,alloc::alloc::Global>,alloc::alloc::Global>,country_core::resources::history::wall_compound_edit::impl_0::new::closure_env_0>_>
                ();
      uStack_280 = (undefined2)extraout_RAX_03;
      uStack_27e = (undefined2)((ulonglong)extraout_RAX_03 >> 0x10);
      uStack_27c = (float)((ulonglong)extraout_RAX_03 >> 0x20);
      local_278 = SUB81(pcVar21,0);
      uStack_277 = (undefined1)((ulonglong)pcVar21 >> 8);
      uStack_276 = (undefined2)((ulonglong)pcVar21 >> 0x10);
      fStack_274 = (float)((ulonglong)pcVar21 >> 0x20);
      local_288[0] = '\a';
      local_288[1] = 0;
      uStack_286 = 0;
      fStack_284 = 0.0;
      uStack_f1 = 0;
      resources::history::History::add();
      *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
      lVar18 = *in_stack_00000058;
      lVar27 = *(longlong *)(lVar18 + 0x30);
      uVar9 = *(undefined8 *)(lVar18 + 0x40);
      if (lVar27 == *(longlong *)(lVar18 + 0x20)) {
        uStack_f1 = 0;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar18 + 0x28) + lVar27 * 8) = uVar9;
      *(longlong *)(lVar18 + 0x30) = lVar27 + 1;
      *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      *(undefined1 *)(in_stack_00000070 + 6) = 1;
    }
    ::utils::cartesian_to_polar();
    ::utils::circular_f32::CircularF32::new();
    uVar46 = *(undefined4 *)(in_stack_00000078 + 4);
    uVar43 = extraout_XMM0_Da_02;
    if ((*in_stack_00000078 & 1) == 0) {
LAB_1409ad946:
      in_stack_00000078[0] = 1;
      in_stack_00000078[1] = 0;
      in_stack_00000078[2] = 0;
      in_stack_00000078[3] = 0;
      *(undefined4 *)(in_stack_00000078 + 4) = uVar43;
      bVar14 = false;
    }
    else {
      expf(*(float *)(*in_stack_00000080 + 0x28) * __real_c1200000);
      local_288[0] = (char)uVar46;
      local_288[1] = (char)((uint)uVar46 >> 8);
      uStack_286 = (undefined2)((uint)uVar46 >> 0x10);
      ::utils::circular_f32::CircularF32::lerp();
      uVar43 = extraout_XMM0_Da_03;
LAB_1409ad935:
      in_stack_00000078[0] = 1;
      in_stack_00000078[1] = 0;
      in_stack_00000078[2] = 0;
      in_stack_00000078[3] = 0;
      *(undefined4 *)(in_stack_00000078 + 4) = uVar43;
      bVar14 = true;
    }
    local_288[0] = (char)uVar43;
    local_288[1] = (char)((uint)uVar43 >> 8);
    uStack_286 = (undefined2)((uint)uVar43 >> 0x10);
    ::utils::circular_f32::CircularF32::as_f32();
    fVar44 = extraout_XMM0_Da_05;
    ::utils::polar_to_cartesian();
    fVar47 = ((float)uStack_140 + fVar36) * extraout_XMM0_Da_06 + fVar47;
    fStack_284 = ((float)uStack_140 + fVar36) * fVar44 + fVar49;
    local_288[0] = SUB41(fVar47,0);
    local_288[1] = (char)((uint)fVar47 >> 8);
    uStack_286 = (undefined2)((uint)fVar47 >> 0x10);
    ::utils::swizzlers::impl$1::xvy();
    if ((bVar14) && (*(char *)(in_stack_00000070 + 6) == '\x01')) {
      local_138._0_4_ = uVar46;
      ::utils::circular_f32::CircularF32::distance();
      if (extraout_XMM0_Da_07 <= __real_3c23d70a) {
        *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
        lVar18 = *in_stack_00000068;
        lVar27 = *(longlong *)(lVar18 + 0x30);
        uVar9 = *(undefined8 *)(lVar18 + 0x40);
        if (lVar27 == *(longlong *)(lVar18 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar7 = *(longlong *)(lVar18 + 0x28);
        *(undefined4 *)(lVar7 + lVar27 * 0x20) = 0x23;
        *(undefined8 *)(lVar7 + 0x18 + lVar27 * 0x20) = uVar9;
        *(longlong *)(lVar18 + 0x30) = lVar27 + 1;
        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
        *(undefined1 *)(in_stack_00000070 + 6) = 2;
      }
      else {
        fVar44 = 0.0;
        ::utils::inverse_lerp();
        if (0.0 <= extraout_XMM0_Da_08) {
          fVar44 = extraout_XMM0_Da_08;
        }
        fVar30 = __real_3f800000;
        if (fVar44 <= __real_3f800000) {
          fVar30 = fVar44;
        }
        local_288[0] = auStack_164[0];
        local_288[1] = auStack_164[1];
        uStack_286 = auStack_164._2_2_;
        fStack_284 = auStack_164._4_4_;
        uStack_280 = (undefined2)uStack_15c;
        uStack_27e = (undefined2)((uint)uStack_15c >> 0x10);
        *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
        lVar18 = *in_stack_00000068;
        lVar27 = *(longlong *)(lVar18 + 0x30);
        uVar9 = *(undefined8 *)(lVar18 + 0x40);
        if (lVar27 == *(longlong *)(lVar18 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar7 = *(longlong *)(lVar18 + 0x28);
        lVar22 = lVar27 * 0x20;
        *(undefined4 *)(lVar7 + lVar22) = 0x22;
        *(float *)(lVar7 + 4 + lVar22) = fVar30;
        *(ulonglong *)(lVar7 + 8 + lVar22) =
             CONCAT44(fStack_284,CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0])));
        *(uint *)(lVar7 + 0x10 + lVar22) = CONCAT22(uStack_27e,uStack_280);
        *(undefined8 *)(lVar7 + 0x18 + lVar22) = uVar9;
        *(longlong *)(lVar18 + 0x30) = lVar27 + 1;
        *(longlong *)(lVar18 + 0x40) = *(longlong *)(lVar18 + 0x40) + 1;
      }
    }
    ::utils::xz_rotation_bad();
    local_300 = __real_3f4ccccd;
    if (((*(char *)((longlong)local_108 + 0x18) != '\0') &&
        (*(longlong ***)((longlong)local_108 + 0x38) == local_188)) &&
       (*(int *)((longlong)local_108 + 0x28) != 2)) {
      local_300 = fVar36;
    }
    uVar9 = CONCAT44(fStack_284,CONCAT22(uStack_286,CONCAT11(local_288[1],local_288[0])));
    uVar3 = CONCAT44(uStack_27c,CONCAT22(uStack_27e,uStack_280));
    local_288[0] = '\0';
    uStack_280 = 0xdd60;
    uStack_27e = 0x42ab;
    uStack_27c = 1.4013e-45;
    local_278 = 0xc;
    uStack_277 = 0;
    uStack_276 = 0;
    fStack_274 = 0.0;
    local_270._0_4_ = (float)CONCAT31(local_270._1_3_,1);
    uStack_268 = 0x42abdd6c;
    uStack_264 = 1;
    uStack_260 = 0x14;
    uStack_250 = 2;
    lVar18 = 0;
    while( true ) {
      if (lVar18 == 2) {
        return;
      }
      lVar27 = lVar18 + 1;
      cVar29 = local_288[lVar18 * 0x18];
      if (cVar29 == '\x02') {
        return;
      }
      uVar10 = *(undefined8 *)(&uStack_280 + lVar18 * 0xc);
      uVar11 = *(undefined8 *)(&local_278 + lVar18 * 0x18);
      local_2e8 = &local_100;
      pcStack_2e0 = resources::walls::impl$7::fmt;
      local_2d8 = auStack_164;
      pcStack_2d0 = glam::f32::vec3::impl$93::fmt;
      pfStack_2c8 = &local_300;
      pcStack_2c0 = core::fmt::float::impl$5::fmt;
      local_138._0_4_ = 0x42abdd90;
      local_138._4_4_ = 1;
      uStack_130 = 3;
      uStack_12c = 0.0;
      uStack_128 = SUB87(&local_2e8,0);
      local_121 = (undefined1)((ulonglong)&local_2e8 >> 0x38);
      uStack_120 = 3;
      uStack_119 = 0;
      puStack_118 = (undefined *)0x0;
      local_258 = lVar27;
      alloc::fmt::format::format_inner();
      fVar36 = local_300;
      local_188 = pplStack_320;
      local_108 = pvStack_318;
      local_138._0_4_ = auStack_164._0_4_;
      local_138._4_4_ = auStack_164._4_4_;
      uStack_130 = uStack_15c;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_05 == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_05 = 1;
      extraout_RAX_05[1] = 1;
      extraout_RAX_05[2] = uVar10;
      extraout_RAX_05[3] = uVar11;
      extraout_RAX_05[4] = uVar9;
      extraout_RAX_05[5] = uVar3;
      *(float *)(extraout_RAX_05 + 6) = fVar36;
      *(ulonglong *)((longlong)extraout_RAX_05 + 0x34) =
           CONCAT44(local_138._4_4_,(undefined4)local_138);
      *(undefined4 *)((longlong)extraout_RAX_05 + 0x3c) = uStack_130;
      *(char *)(extraout_RAX_05 + 8) = cVar29;
      local_138._0_4_ = SUB84(extraout_RAX_05,0);
      local_138._4_4_ = (undefined4)((ulonglong)extraout_RAX_05 >> 0x20);
      uStack_130 = 0x42abddc0;
      uStack_12c = 1.4013e-45;
      if ((longlong)sStack_310 < 0) break;
      local_178 = extraout_RAX_05;
      if (sStack_310 == 0) {
        _Dst = (void *)0x1;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        _Dst = extraout_RAX_06;
        if (extraout_RAX_06 == (void *)0x0) break;
      }
      memcpy(_Dst,local_108,sStack_310);
      local_138._0_4_ = (undefined4)sStack_310;
      local_138._4_4_ = (undefined4)(sStack_310 >> 0x20);
      uStack_130 = SUB84(_Dst,0);
      uStack_12c = (float)((ulonglong)_Dst >> 0x20);
      uStack_128 = (undefined7)sStack_310;
      local_121 = (undefined1)(sStack_310 >> 0x38);
      uStack_120 = SUB87(local_178,0);
      uStack_119 = (undefined1)((ulonglong)local_178 >> 0x38);
      puStack_118 = &DAT_142abddc0;
      append_only_vec::AppendOnlyVec::
      push<country_core::systems::ui::signifiers::signifier_create::UiAppendSignifier>();
      lVar18 = lVar27;
      if (local_188 != (longlong **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
                    /* WARNING: Subroutine does not return */
    alloc::raw_vec::handle_error();
  }
LAB_1409acc06:
  auVar38[0] = -(*pcVar1 == -1);
  auVar38[1] = -(pcVar1[1] == -1);
  auVar38[2] = -(pcVar1[2] == -1);
  auVar38[3] = -(pcVar1[3] == -1);
  auVar38[4] = -(pcVar1[4] == -1);
  auVar38[5] = -(pcVar1[5] == -1);
  auVar38[6] = -(pcVar1[6] == -1);
  auVar38[7] = -(pcVar1[7] == -1);
  auVar38[8] = -(pcVar1[8] == -1);
  auVar38[9] = -(pcVar1[9] == -1);
  auVar38[10] = -(pcVar1[10] == -1);
  auVar38[0xb] = -(pcVar1[0xb] == -1);
  auVar38[0xc] = -(pcVar1[0xc] == -1);
  auVar38[0xd] = -(pcVar1[0xd] == -1);
  auVar38[0xe] = -(pcVar1[0xe] == -1);
  auVar38[0xf] = -(pcVar1[0xf] == -1);
  if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar38 >> 0x17,0) & 1) != 0) || (SUB161(auVar38 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar38 >> 0x27,0) & 1) != 0) || (SUB161(auVar38 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x37,0) & 1) != 0) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') {
LAB_1409ad197:
                    /* WARNING: Subroutine does not return */
    core::option::unwrap_failed();
  }
  uVar19 = uVar19 + lVar22 + 0x10;
  lVar22 = lVar22 + 0x10;
  goto LAB_1409acbc6;
}

