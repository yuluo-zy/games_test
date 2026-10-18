// draw @ 0x1400ddc80
// sig: void __cdecl draw(void)


void __cdecl tiny_glade::egui_backend::EguiBackend::draw(void)

{
  byte *pbVar1;
  float *pfVar2;
  char *pcVar3;
  undefined8 *puVar4;
  byte bVar5;
  size_t sVar6;
  undefined1 (*pauVar7) [16];
  uint uVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  undefined1 auVar13 [14];
  undefined6 uVar14;
  char extraout_AL;
  uint uVar15;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *plVar16;
  longlong *extraout_RAX_04;
  longlong *plVar17;
  ulonglong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong extraout_RAX_08;
  ulonglong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong extraout_RAX_11;
  ulonglong extraout_RAX_12;
  undefined8 *extraout_RAX_13;
  undefined4 uVar18;
  undefined8 *in_RCX;
  size_t *psVar19;
  longlong *plVar20;
  undefined4 uVar21;
  uint uVar22;
  longlong in_RDX;
  ulonglong uVar23;
  longlong *plVar24;
  undefined8 *puVar25;
  longlong in_R8;
  longlong lVar26;
  longlong lVar27;
  longlong *in_R9;
  ushort uVar28;
  ulonglong uVar29;
  longlong lVar30;
  longlong in_R11;
  longlong *plVar31;
  undefined *puVar32;
  longlong *plVar33;
  byte bVar34;
  bool bVar35;
  byte bVar36;
  char cVar37;
  char cVar40;
  char cVar41;
  float fVar38;
  char cVar42;
  float extraout_XMM0_Da;
  float fVar43;
  float fVar44;
  float fVar45;
  undefined1 auVar39 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  float fVar50;
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  longlong *local_b60;
  longlong *plStack_b58;
  longlong local_b50;
  undefined8 uStack_b48;
  undefined8 local_b40;
  longlong *plStack_b38;
  undefined8 local_b30;
  undefined8 uStack_b28;
  undefined8 local_b20;
  undefined8 uStack_b18;
  longlong *local_778;
  longlong *plStack_770;
  longlong *local_768;
  longlong *plStack_760;
  longlong *local_758;
  undefined6 uStack_750;
  undefined2 uStack_74a;
  undefined6 uStack_748;
  char cStack_742;
  undefined1 uStack_741;
  undefined6 uStack_740;
  undefined2 uStack_73a;
  undefined6 local_738;
  char cStack_732;
  undefined1 uStack_731;
  undefined8 uStack_730;
  longlong local_388;
  longlong local_380;
  longlong *local_370;
  longlong *local_368;
  longlong *plStack_360;
  longlong *local_358;
  longlong *plStack_350;
  longlong *local_348;
  undefined6 uStack_340;
  undefined2 uStack_33a;
  undefined8 uStack_338;
  size_t sStack_330;
  longlong local_328;
  undefined8 uStack_320;
  byte local_2c0;
  byte local_2bd;
  longlong *local_268;
  longlong *plStack_260;
  longlong *local_258;
  longlong *plStack_250;
  undefined8 local_248;
  longlong *local_238;
  longlong *local_230;
  longlong *local_228;
  longlong *local_220;
  longlong *local_218;
  longlong *plStack_210;
  longlong *local_208;
  longlong *plStack_200;
  longlong *local_1f8;
  undefined8 *local_1e8;
  undefined8 *local_1e0;
  longlong *local_1d8;
  undefined8 *puStack_1d0;
  longlong *local_1c8;
  longlong *plStack_1c0;
  longlong *local_1b8;
  void *pvStack_1b0;
  longlong local_1a8;
  size_t sStack_1a0;
  longlong local_198;
  longlong *local_188;
  longlong *local_180;
  undefined8 *puStack_178;
  longlong *local_170;
  longlong *plStack_168;
  longlong *local_160;
  void *pvStack_158;
  longlong local_150;
  longlong local_148;
  longlong *local_140;
  longlong *local_138;
  undefined8 local_130;
  longlong *local_128;
  longlong *plStack_120;
  undefined8 local_118;
  longlong *plStack_110;
  int local_108;
  undefined4 uStack_104;
  undefined4 uStack_100;
  undefined4 uStack_fc;
  void *local_f8;
  longlong lStack_f0;
  longlong *local_e8;
  longlong *local_e0;
  undefined8 *local_d8;
  longlong *local_d0;
  longlong *local_c8;
  longlong *local_c0;
  undefined1 local_b5;
  undefined1 local_b4;
  undefined1 local_b3;
  undefined1 local_b2;
  undefined1 local_b1;
  undefined8 local_b0;
  
  local_b0 = 0xfffffffffffffffe;
  local_130 = *(undefined8 *)(in_RDX + 400);
  local_1e8 = in_RCX + 9;
  plVar24 = (longlong *)in_RCX[10];
  local_c0 = (longlong *)in_RCX[0xb];
  in_RCX[0xb] = 0;
  local_d8 = in_RCX;
  while (local_c0 != (longlong *)0x0) {
    plVar24 = plVar24 + 5;
    local_d0 = plVar24;
    local_c0 = (longlong *)((longlong)local_c0 + -1);
    rhapsody::raw::managed::handle::impl$6::drop();
  }
  puStack_178 = local_d8 + 3;
  local_188 = (longlong *)local_d8[4];
  local_170 = (longlong *)local_d8[5];
  local_d8[5] = 0;
  local_180 = local_188 + (longlong)local_170 * 0xb;
  plStack_168 = (longlong *)0x0;
  local_1e0 = local_d8 + 0xf;
  local_138 = local_d8 + 0x13;
  iVar9 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                 CONCAT13(unaff_XMM6_Bd,
                          CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar10 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  plVar24 = local_138;
  local_380 = in_R8;
  local_370 = in_R9;
  while (local_188 != local_180) {
    plVar16 = local_188 + 0xb;
    plVar31 = (longlong *)*local_188;
    local_148 = local_188[1];
    local_198 = local_188[10];
    local_1a8 = local_188[8];
    sStack_1a0 = local_188[9];
    local_1b8 = (longlong *)local_188[6];
    pvStack_1b0 = (void *)local_188[7];
    local_1c8 = (longlong *)local_188[4];
    plStack_1c0 = (longlong *)local_188[5];
    local_1d8 = (longlong *)local_188[2];
    puStack_1d0 = (undefined8 *)local_188[3];
    local_188 = plVar16;
    if (plVar31 == (longlong *)0x2) break;
    uStack_338._0_6_ = (undefined6)local_1a8;
    uStack_338._6_2_ = (undefined2)((ulonglong)local_1a8 >> 0x30);
    uStack_340 = SUB86(pvStack_1b0,0);
    uStack_33a = (undefined2)((ulonglong)pvStack_1b0 >> 0x30);
    local_368 = local_1d8;
    plStack_360 = puStack_1d0;
    local_358 = local_1c8;
    plStack_350 = plStack_1c0;
    local_348 = local_1b8;
    sStack_330 = sStack_1a0;
    local_328 = local_198;
    puffin::are_scopes_on();
    if (extraout_AL == '\0') {
      local_140 = (longlong *)0x0;
    }
    else {
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX == 1) {
        if (plVar24 != (longlong *)0x0) {
          if (&DAT_00000035 < plVar24) {
            if (plVar24 != (longlong *)0x36) goto LAB_1400df699;
          }
          else if (*(char *)((longlong)plVar24 + 0x14293adfb) < -0x40) {
LAB_1400df699:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        core::str::impl$0::rfind<ref_<str_>_>();
        if ((extraout_RAX_00 == 1) && (plVar24 != (longlong *)0xfffffffffffffffe)) {
          if ((longlong)plVar24 + 2U < 0x36) {
            if (*(char *)((longlong)plVar24 + 0x14293adfd) < -0x40) {
LAB_1400df6be:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if ((longlong)plVar24 + 2U != 0x36) goto LAB_1400df6be;
        }
      }
      puVar32 = &DAT_14293b135;
      do {
        bVar34 = puVar32[-1];
        uVar15 = (uint)(char)bVar34;
        if ((char)bVar34 < '\0') {
          bVar36 = puVar32[-2];
          if ((char)bVar36 < -0x40) {
            bVar5 = puVar32[-3];
            if ((char)bVar5 < -0x40) {
              pbVar1 = puVar32 + -4;
              puVar32 = puVar32 + -4;
              uVar15 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              puVar32 = puVar32 + -3;
              uVar15 = bVar5 & 0xf;
            }
            uVar15 = bVar36 & 0x3f | uVar15 << 6;
          }
          else {
            puVar32 = puVar32 + -2;
            uVar15 = bVar36 & 0x1f;
          }
          uVar15 = (uint)(bVar34 & 0x3f) | uVar15 << 6;
          if (uVar15 != 0x5c) goto LAB_1400ddff4;
LAB_1400de010:
          if (puVar32 == &UNK_14293b10f) break;
          if (puVar32 + -0x14293b10f < (undefined *)0x25) {
            if (-0x41 < (char)puVar32[1]) break;
          }
          else if (puVar32 + -0x14293b10f == (undefined *)0x25) break;
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
        puVar32 = puVar32 + -1;
        if (uVar15 == 0x5c) goto LAB_1400de010;
LAB_1400ddff4:
        if (uVar15 == 0x2f) goto LAB_1400de010;
      } while (puVar32 != &DAT_14293b110);
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_01 == 0) {
        std::sys::thread_local::native::lazy::Storage::
        initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                  ();
        plVar24 = extraout_RAX_02;
        if (extraout_RAX_02 == (longlong *)0x0) goto LAB_1400df577;
      }
      else {
        if ((int)*extraout_RAX_01 != 1) {
LAB_1400df577:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        plVar24 = extraout_RAX_01 + 1;
      }
      if (*plVar24 != 0) {
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar24 = -1;
      local_c8 = plVar24;
      puffin::ThreadProfiler::begin_scope();
      *local_c8 = *local_c8 + 1;
      plStack_210 = extraout_RAX_03;
      local_140 = (longlong *)0x1;
    }
    plVar24 = local_348;
    local_218 = local_140;
    bVar35 = SBORROW8(0,(longlong)plStack_350);
    plVar16 = local_348 + 5;
    if (!bVar35) {
      plVar16 = &uStack_338;
    }
    lVar27 = *plVar16;
    psVar19 = (size_t *)(local_348 + 6);
    if (!bVar35) {
      psVar19 = &sStack_330;
    }
    sVar6 = *psVar19;
    if (bVar35) {
      plVar20 = (longlong *)local_348[3];
      in_R11 = 0;
      plVar16 = local_c0;
      plVar33 = (longlong *)local_348[4];
    }
    else {
      plVar16 = (longlong *)CONCAT26(uStack_33a,uStack_340);
      in_R11 = CONCAT71((int7)((ulonglong)in_R11 >> 8),1);
      if (plVar16 == (longlong *)0x0) {
        local_d0 = (longlong *)0x1;
        plVar16 = (longlong *)0x0;
        plVar20 = (longlong *)0x1;
        plVar33 = (longlong *)0x0;
      }
      else {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_04 == (longlong *)0x0) {
          local_e0 = local_d0;
          local_e8 = local_c0;
          local_b3 = 0;
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        uVar23 = (ulonglong)in_R11 >> 8;
        if (0xf < (ulonglong)((longlong)extraout_RAX_04 - (longlong)plVar24) &&
            (longlong *)0x3 < plVar16) {
          plVar17 = (longlong *)((ulonglong)plVar16 & 0xfffffffffffffffc);
          plVar20 = (longlong *)0x0;
          do {
            pfVar2 = (float *)((longlong)plVar24 + (longlong)plVar20 * 4);
            fVar38 = *pfVar2 * 255.0 + 0.5;
            fVar43 = pfVar2[1] * 255.0 + 0.5;
            fVar44 = pfVar2[2] * 255.0 + 0.5;
            fVar45 = pfVar2[3] * 255.0 + 0.5;
            fVar50 = 0.0;
            if (0.0 <= fVar45) {
              fVar50 = fVar45;
            }
            fVar45 = 255.0;
            if (fVar50 <= 255.0) {
              fVar45 = fVar50;
            }
            fVar50 = 0.0;
            if (0.0 <= fVar38) {
              fVar50 = fVar38;
            }
            fVar38 = 0.0;
            if (0.0 <= fVar44) {
              fVar38 = fVar44;
            }
            fVar44 = 255.0;
            if (fVar38 <= 255.0) {
              fVar44 = fVar38;
            }
            fVar38 = 255.0;
            if (fVar50 <= 255.0) {
              fVar38 = fVar50;
            }
            fVar50 = 0.0;
            if (0.0 <= fVar43) {
              fVar50 = fVar43;
            }
            fVar43 = 255.0;
            if (fVar50 <= 255.0) {
              fVar43 = fVar50;
            }
            uVar28 = (ushort)(int)fVar45 & 0xff;
            auVar13._12_2_ = uVar28;
            auVar13._0_12_ = ZEXT212(uVar28) << 0x40;
            puVar4 = (undefined8 *)((longlong)extraout_RAX_04 + (longlong)plVar20 * 4);
            *puVar4 = CONCAT44(((int)fVar43 & 0xffU) * 0x1010101,((int)fVar38 & 0xffU) * 0x1010101);
            *(uint *)(puVar4 + 1) = ((int)fVar44 & 0xffU) * 0x1010101;
            *(int *)((longlong)puVar4 + 0xc) =
                 (int)(((ulonglong)auVar13._8_6_ & 0xffffffff) * 0x1010101);
            plVar20 = (longlong *)((longlong)plVar20 + 4);
            if (plVar17 == plVar20) goto LAB_1400de2b1;
          } while( true );
        }
        plVar17 = (longlong *)0x0;
        do {
          fVar38 = *(float *)((longlong)plVar24 + (longlong)plVar17 * 4) * 255.0 + 0.5;
          fVar50 = 0.0;
          if (0.0 <= fVar38) {
            fVar50 = fVar38;
          }
          fVar38 = 255.0;
          if (fVar50 <= 255.0) {
            fVar38 = fVar50;
          }
          *(uint *)((longlong)extraout_RAX_04 + (longlong)plVar17 * 4) =
               ((int)fVar38 & 0xffU) * 0x1010101;
          plVar17 = (longlong *)((longlong)plVar17 + 1);
LAB_1400de2b1:
          in_R11 = CONCAT71((int7)uVar23,1);
          local_d0 = extraout_RAX_04;
          plVar20 = extraout_RAX_04;
          plVar33 = plVar16;
        } while (plVar16 != plVar17);
      }
    }
    plStack_770 = (longlong *)local_148;
    local_c8 = (longlong *)CONCAT44(local_c8._4_4_,(int)in_R11);
    local_b3 = (undefined1)in_R11;
    local_778 = plVar31;
    local_e8 = plVar16;
    local_e0 = local_d0;
    local_c0 = plVar16;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<enum2_<epaint::TextureId>_>_>();
    lVar30 = local_d8[0xf];
    bVar34 = (byte)(extraout_RAX_05 >> 0x38);
    bVar36 = bVar34 >> 1;
    auVar39 = ZEXT216(CONCAT11(bVar36,bVar36));
    auVar39 = pshuflw(auVar39,auVar39,0);
    lVar26 = 0;
    uVar23 = extraout_RAX_05;
    while( true ) {
      uVar23 = local_d8[0x10] & uVar23;
      pcVar3 = (char *)(lVar30 + uVar23);
      cVar37 = auVar39[0];
      auVar48[0] = -(*pcVar3 == cVar37);
      cVar40 = auVar39[1];
      auVar48[1] = -(pcVar3[1] == cVar40);
      cVar41 = auVar39[2];
      auVar48[2] = -(pcVar3[2] == cVar41);
      cVar42 = auVar39[3];
      auVar48[3] = -(pcVar3[3] == cVar42);
      auVar48[4] = -(pcVar3[4] == cVar37);
      auVar48[5] = -(pcVar3[5] == cVar40);
      auVar48[6] = -(pcVar3[6] == cVar41);
      auVar48[7] = -(pcVar3[7] == cVar42);
      auVar48[8] = -(pcVar3[8] == cVar37);
      auVar48[9] = -(pcVar3[9] == cVar40);
      auVar48[10] = -(pcVar3[10] == cVar41);
      auVar48[0xb] = -(pcVar3[0xb] == cVar42);
      auVar48[0xc] = -(pcVar3[0xc] == cVar37);
      auVar48[0xd] = -(pcVar3[0xd] == cVar40);
      auVar48[0xe] = -(pcVar3[0xe] == cVar41);
      auVar48[0xf] = -(pcVar3[0xf] == cVar42);
      uVar28 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar48 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar48 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar48 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar48 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar48 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar48 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar48 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar48 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar48 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar48 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar48 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe | (ushort)(auVar48[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar28;
      local_b3 = SUB81(local_c8,0);
      while (uVar28 != 0) {
        uVar22 = 0;
        for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar22 = uVar22 + 1;
        }
        uVar29 = uVar22 + uVar23 & local_d8[0x10];
        if ((*(longlong **)(lVar30 + -0x48 + uVar29 * -0x48) == plVar31) &&
           (in_R11 = local_148, *(longlong *)(lVar30 + -0x40 + uVar29 * -0x48) == local_148)) {
          uVar18 = 0;
          uVar21 = 0;
          if (((ulonglong)local_368 & 1) != 0) {
            uVar18 = plStack_360._0_4_;
            uVar21 = local_358._0_4_;
          }
          plStack_120 = (longlong *)((longlong)plVar33 << 2);
          local_118 = (undefined8 *)CONCAT44(uVar21,uVar18);
          plStack_110 = (longlong *)(lVar27 << 0x20);
          uStack_104 = 1;
          uStack_100 = 0;
          uStack_fc = 0;
          local_e0 = local_d0;
          local_e8 = local_c0;
          local_128 = plVar20;
          local_108 = (int)sVar6;
          rhapsody::raw::device::GraphicsDevice::update_texture();
          if ((int)local_b60 != 0xb) {
            local_738 = (undefined6)local_b20;
            cStack_732 = (char)((ulonglong)local_b20 >> 0x30);
            uStack_731 = (undefined1)((ulonglong)local_b20 >> 0x38);
            uStack_730 = uStack_b18;
            uStack_748 = (undefined6)local_b30;
            cStack_742 = (char)((ulonglong)local_b30 >> 0x30);
            uStack_741 = (undefined1)((ulonglong)local_b30 >> 0x38);
            uStack_740 = (undefined6)uStack_b28;
            uStack_73a = (undefined2)((ulonglong)uStack_b28 >> 0x30);
            local_758 = (longlong *)local_b40;
            uStack_750 = SUB86(plStack_b38,0);
            uStack_74a = (undefined2)((ulonglong)plStack_b38 >> 0x30);
            local_768 = (longlong *)local_b50;
            plStack_760 = (longlong *)uStack_b48;
            local_778 = local_b60;
            plStack_770 = plStack_b58;
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          goto LAB_1400de6d0;
        }
        uVar28 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar28);
      }
      auVar46[0] = -(*pcVar3 == (char)iVar9);
      auVar46[1] = -(pcVar3[1] == (char)((uint)iVar9 >> 8));
      auVar46[2] = -(pcVar3[2] == (char)((uint)iVar9 >> 0x10));
      auVar46[3] = -(pcVar3[3] == (char)((uint)iVar9 >> 0x18));
      auVar46[4] = -(pcVar3[4] == (char)iVar10);
      auVar46[5] = -(pcVar3[5] == (char)((uint)iVar10 >> 8));
      auVar46[6] = -(pcVar3[6] == (char)((uint)iVar10 >> 0x10));
      auVar46[7] = -(pcVar3[7] == (char)((uint)iVar10 >> 0x18));
      auVar46[8] = -(pcVar3[8] == (char)iVar11);
      auVar46[9] = -(pcVar3[9] == (char)((uint)iVar11 >> 8));
      auVar46[10] = -(pcVar3[10] == (char)((uint)iVar11 >> 0x10));
      auVar46[0xb] = -(pcVar3[0xb] == (char)((uint)iVar11 >> 0x18));
      auVar46[0xc] = -(pcVar3[0xc] == (char)iVar12);
      auVar46[0xd] = -(pcVar3[0xd] == (char)((uint)iVar12 >> 8));
      auVar46[0xe] = -(pcVar3[0xe] == (char)((uint)iVar12 >> 0x10));
      auVar46[0xf] = -(pcVar3[0xf] == (char)((uint)iVar12 >> 0x18));
      if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar46 >> 0x17,0) & 1) != 0) || (SUB161(auVar46 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar46 >> 0x27,0) & 1) != 0) || (SUB161(auVar46 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar46 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar46 >> 0x3f,0) & 1) != 0) || (SUB161(auVar46 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar46 >> 0x4f,0) & 1) != 0) || (SUB161(auVar46 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar46 >> 0x5f,0) & 1) != 0) || (SUB161(auVar46 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar46 >> 0x6f,0) & 1) != 0) || (SUB161(auVar46 >> 0x77,0) & 1) != 0) ||
          auVar46[0xf] < '\0') break;
      uVar23 = uVar23 + lVar26 + 0x10;
      lVar26 = lVar26 + 0x10;
    }
    if (local_d8[0x11] == 0) {
      local_e0 = local_d0;
      local_e8 = local_c0;
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<enum2_<epaint::TextureId>,rhapsody::raw::texture::Texture>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<enum2_<epaint::TextureId>,rhapsody::raw::texture::Texture,std::hash::random::RandomState>_>
                ();
      local_148 = (longlong)plStack_770;
      plVar31 = local_778;
    }
    local_b60 = (longlong *)(ulonglong)(uint)((int)sVar6 * (int)lVar27);
    local_b3 = SUB81(local_c8,0);
    local_128 = plVar33;
    if (plVar33 != local_b60) {
      local_778 = (longlong *)0x0;
      local_e0 = local_d0;
      local_e8 = local_c0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<usize,usize>();
    }
    local_e0 = local_d0;
    local_e8 = local_c0;
    rhapsody::raw::texture::TextureDesc::new_2d();
    local_e0 = local_d0;
    local_e8 = local_c0;
    local_b3 = SUB81(local_c8,0);
    rhapsody::raw::texture::TextureDesc::sampled();
    local_e0 = local_d0;
    local_e8 = local_c0;
    local_b3 = SUB81(local_c8,0);
    rhapsody::raw::device::GraphicsDevice::create_texture<ref_<slice2_<u8>_>_>();
    uStack_748 = (undefined6)uStack_b28;
    uStack_750 = (undefined6)local_b30;
    uVar14 = uStack_750;
    if ((int)local_b60 != 0xb) {
      local_738 = (undefined6)local_b20;
      cStack_732 = (char)((ulonglong)local_b20 >> 0x30);
      uStack_731 = (undefined1)((ulonglong)local_b20 >> 0x38);
      uStack_730 = uStack_b18;
      cStack_742 = (char)((ulonglong)local_b30 >> 0x30);
      uStack_741 = (undefined1)((ulonglong)local_b30 >> 0x38);
      uStack_73a = (undefined2)((ulonglong)uStack_b28 >> 0x30);
      local_758 = (longlong *)local_b40;
      uStack_750 = SUB86(plStack_b38,0);
      uStack_74a = (undefined2)((ulonglong)plStack_b38 >> 0x30);
      local_768 = (longlong *)local_b50;
      plStack_760 = (longlong *)uStack_b48;
      local_778 = local_b60;
      plStack_770 = plStack_b58;
      uStack_740 = uStack_748;
      uStack_748 = uVar14;
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    cStack_742 = (char)((ulonglong)uStack_b28 >> 0x30);
    uStack_741 = (undefined1)((ulonglong)uStack_b28 >> 0x38);
    local_758 = plStack_b38;
    uStack_74a = (undefined2)((ulonglong)local_b30 >> 0x30);
    local_768 = (longlong *)uStack_b48;
    plStack_760 = (longlong *)local_b40;
    local_778 = plStack_b58;
    plStack_770 = (longlong *)local_b50;
    pauVar7 = (undefined1 (*) [16])local_d8[0xf];
    uVar23 = local_d8[0x10];
    uVar29 = extraout_RAX_05 & uVar23;
    auVar39 = *(undefined1 (*) [16])(*pauVar7 + uVar29);
    uVar28 = (ushort)(SUB161(auVar39 >> 7,0) & 1) | (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar39[0xf] >> 7) << 0xf;
    uVar15 = (uint)uVar28;
    if (uVar28 == 0) {
      lVar27 = 0x10;
      do {
        uVar29 = uVar29 + lVar27 & uVar23;
        auVar39 = *(undefined1 (*) [16])(*pauVar7 + uVar29);
        lVar27 = lVar27 + 0x10;
        uVar15 = (uint)(ushort)((ushort)(SUB161(auVar39 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar39[0xf] >> 7) << 0xf);
      } while (uVar15 == 0);
    }
    uVar22 = 0;
    for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    uVar29 = uVar22 + uVar29 & uVar23;
    bVar36 = (*pauVar7)[uVar29];
    if (-1 < (char)bVar36) {
      auVar39 = *pauVar7;
      uVar15 = 0;
      for (uVar22 = (uint)(ushort)((ushort)(SUB161(auVar39 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar39 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar39 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar39 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar39 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar39 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar39 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar39 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar39 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar39 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar39 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar39 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar39 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar39 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar39 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar39[0xf] >> 7) << 0xf); (uVar22 & 1) == 0;
          uVar22 = uVar22 >> 1 | 0x80000000) {
        uVar15 = uVar15 + 1;
      }
      uVar29 = (ulonglong)uVar15;
      bVar36 = (*pauVar7)[uVar29];
    }
    bVar34 = bVar34 >> 1;
    (*pauVar7)[uVar29] = bVar34;
    pauVar7[1][uVar29 - 0x10 & uVar23] = bVar34;
    local_d8[0x11] = local_d8[0x11] - (ulonglong)(bVar36 & 1);
    *(longlong **)((longlong)pauVar7 + uVar29 * -0x48 + -0x48) = plVar31;
    *(longlong *)((longlong)pauVar7 + uVar29 * -0x48 + -0x40) = local_148;
    plVar24 = (longlong *)((longlong)pauVar7 + uVar29 * -0x48 + -0x38);
    *plVar24 = (longlong)plStack_b58;
    plVar24[1] = local_b50;
    puVar4 = (undefined8 *)((longlong)pauVar7 + uVar29 * -0x48 + -0x28);
    *puVar4 = uStack_b48;
    puVar4[1] = local_b40;
    puVar4 = (undefined8 *)((longlong)pauVar7 + uVar29 * -0x48 + -0x18);
    *puVar4 = plStack_b38;
    puVar4[1] = local_b30;
    *(undefined8 *)((longlong)pauVar7 + uVar29 * -0x48 + -8) = uStack_b28;
    local_d8[0x12] = local_d8[0x12] + 1;
LAB_1400de6d0:
    plVar24 = local_140;
    if (((byte)local_c8 & local_c0 != (longlong *)0x0) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
      plVar24 = local_218;
    }
    if (plVar24 != (longlong *)0x0) {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_06 == 0) {
        std::sys::thread_local::native::lazy::Storage::
        initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                  ();
        plVar24 = extraout_RAX_07;
        if (extraout_RAX_07 == (longlong *)0x0) goto LAB_1400df566;
      }
      else {
        if ((int)*extraout_RAX_06 != 1) {
LAB_1400df566:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        plVar24 = extraout_RAX_06 + 1;
      }
      if (*plVar24 != 0) {
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar24 = -1;
      local_c8 = plVar24;
      puffin::ThreadProfiler::end_scope();
      *local_c8 = *local_c8 + 1;
    }
    plVar24 = plStack_350;
    if (SBORROW8(0,(longlong)plStack_350)) {
      LOCK();
      *local_348 = *local_348 + -1;
      UNLOCK();
      if (*local_348 == 0) {
        alloc::sync::Arc::drop_slow<epaint::image::ColorImage,alloc::alloc::Global>();
      }
    }
    else if (plStack_350 != (longlong *)0x0) {
      plVar24 = (longlong *)((longlong)plStack_350 << 2);
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  core::ptr::
  drop_in_place<alloc::vec::drain::Drain<tuple_<enum2_<epaint::TextureId>,epaint::image::ImageDelta>,alloc::alloc::Global>_>
            ();
  uVar23 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX_08 == 0) || (*(ulonglong *)(local_380 + 0x218) <= uVar23)) ||
      (uVar23 = *(ulonglong *)(*(longlong *)(local_380 + 0x210) + uVar23 * 8), uVar23 == 0)) ||
     (*(longlong *)(*(longlong *)(local_380 + 0x1e0) + 0x20 + ~uVar23 * 0x60) == 0)) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  puVar32 = &DAT_14293b150;
  country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  local_c0 = (longlong *)puVar32;
  if ((extraout_RAX_09 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_368 = (longlong *)((ulonglong)local_368 & 0xffffffff00000000);
  plStack_360 = local_370;
  country_core::render::pass_builder::framebuffer_binding::ColorAttachment::desc();
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_768 = local_358;
  plStack_760 = plStack_350;
  local_778 = local_368;
  plStack_770 = plStack_360;
  local_1d8 = (longlong *)CONCAT44(local_1d8._4_4_,2);
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  local_b4 = 1;
  uVar23 = 0x905c303950d415d1;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_11 == 0) || (*(ulonglong *)(local_380 + 0x218) <= uVar23)) ||
     ((uVar23 = *(ulonglong *)(*(longlong *)(local_380 + 0x210) + uVar23 * 8), uVar23 == 0 ||
      (*(longlong *)(*(longlong *)(local_380 + 0x1e0) + 0x20 + ~uVar23 * 0x60) == 0)))) {
    local_b4 = 1;
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  local_b4 = 1;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  local_b4 = 1;
  egui::context::Context::
  write<f32,egui::context::impl_13::input::closure_env_0<f32,egui::context::impl_15::pixels_per_point::closure_env_0>_>
            ();
  local_b4 = 1;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  local_768 = (longlong *)local_d8[2];
  local_778 = (longlong *)*local_d8;
  plStack_770 = (longlong *)local_d8[1];
  *local_d8 = 0;
  local_d8[1] = 8;
  local_d8[2] = 0;
  local_b4 = 1;
  egui::context::Context::tessellate();
  local_220 = plStack_360 + (longlong)local_358 * 10;
  local_238 = plStack_360;
  local_230 = plStack_360;
  local_228 = local_368;
LAB_1400deb3e:
  if (local_230 != local_220) {
    plVar24 = local_230 + 10;
    plVar16 = (longlong *)*local_230;
    local_198 = local_230[9];
    local_1a8 = local_230[7];
    sStack_1a0 = local_230[8];
    local_1b8 = (longlong *)local_230[5];
    pvStack_1b0 = (void *)local_230[6];
    local_1c8 = (longlong *)local_230[3];
    plStack_1c0 = (longlong *)local_230[4];
    local_1d8 = (longlong *)local_230[1];
    puStack_1d0 = (undefined8 *)local_230[2];
    local_230 = plVar24;
    if (plVar16 == (longlong *)0x3) goto LAB_1400df24a;
    local_188 = plVar16;
    local_180 = local_1d8;
    puStack_178 = puStack_1d0;
    local_170 = local_1c8;
    plStack_168 = plStack_1c0;
    local_160 = local_1b8;
    pvStack_158 = pvStack_1b0;
    local_150 = local_1a8;
    if (plVar16 == (longlong *)0x2) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    local_108 = (int)plStack_1c0;
    uStack_104 = (undefined4)((ulonglong)plStack_1c0 >> 0x20);
    uStack_100 = SUB84(local_1b8,0);
    uStack_fc = (undefined4)((ulonglong)local_1b8 >> 0x20);
    local_128 = plVar16;
    plStack_120 = local_1d8;
    local_118 = puStack_1d0;
    plStack_110 = local_1c8;
    local_f8 = pvStack_1b0;
    lStack_f0 = local_1a8;
    if ((local_1a8 != 0) && (plStack_1c0 != (longlong *)0x0)) {
      if (local_d8[0x12] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<enum2_<epaint::TextureId>_>_>();
        lVar27 = local_d8[0xf];
        bVar34 = (byte)(extraout_RAX_12 >> 0x39);
        auVar39 = ZEXT216(CONCAT11(bVar34,bVar34));
        auVar39 = pshuflw(auVar39,auVar39,0);
        lVar30 = 0;
        uVar23 = extraout_RAX_12;
        while( true ) {
          uVar23 = local_d8[0x10] & uVar23;
          pcVar3 = (char *)(lVar27 + uVar23);
          cVar37 = auVar39[0];
          auVar49[0] = -(*pcVar3 == cVar37);
          cVar40 = auVar39[1];
          auVar49[1] = -(pcVar3[1] == cVar40);
          cVar41 = auVar39[2];
          auVar49[2] = -(pcVar3[2] == cVar41);
          cVar42 = auVar39[3];
          auVar49[3] = -(pcVar3[3] == cVar42);
          auVar49[4] = -(pcVar3[4] == cVar37);
          auVar49[5] = -(pcVar3[5] == cVar40);
          auVar49[6] = -(pcVar3[6] == cVar41);
          auVar49[7] = -(pcVar3[7] == cVar42);
          auVar49[8] = -(pcVar3[8] == cVar37);
          auVar49[9] = -(pcVar3[9] == cVar40);
          auVar49[10] = -(pcVar3[10] == cVar41);
          auVar49[0xb] = -(pcVar3[0xb] == cVar42);
          auVar49[0xc] = -(pcVar3[0xc] == cVar37);
          auVar49[0xd] = -(pcVar3[0xd] == cVar40);
          auVar49[0xe] = -(pcVar3[0xe] == cVar41);
          auVar49[0xf] = -(pcVar3[0xf] == cVar42);
          uVar28 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar49[0xf] >> 7) << 0xf;
          uVar15 = (uint)uVar28;
          while (uVar28 != 0) {
            uVar22 = 0;
            for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar22 = uVar22 + 1;
            }
            uVar29 = uVar22 + uVar23 & local_d8[0x10];
            if ((local_128 == *(longlong **)(lVar27 + -0x48 + uVar29 * -0x48)) &&
               (plStack_120 == *(longlong **)(lVar27 + -0x40 + uVar29 * -0x48))) {
              rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu();
              rhapsody::raw::device::GraphicsDevice::create_mapped_buffer();
              if ((int)local_368 != 0xb) {
                local_738 = (undefined6)local_328;
                cStack_732 = (char)((ulonglong)local_328 >> 0x30);
                uStack_731 = (undefined1)((ulonglong)local_328 >> 0x38);
                uStack_730 = uStack_320;
                uStack_748 = (undefined6)uStack_338;
                cStack_742 = (char)uStack_338._6_2_;
                uStack_741 = (undefined1)((ushort)uStack_338._6_2_ >> 8);
                uStack_740 = (undefined6)sStack_330;
                uStack_73a = (undefined2)(sStack_330 >> 0x30);
                local_758 = local_348;
                uStack_750 = uStack_340;
                uStack_74a = uStack_33a;
                local_768 = local_358;
                plStack_760 = plStack_350;
                local_778 = local_368;
                plStack_770 = plStack_360;
                    // WARNING: Subroutine does not return
                core::result::unwrap_failed();
              }
              local_248 = CONCAT26(uStack_33a,uStack_340);
              local_258 = plStack_350;
              plStack_250 = local_348;
              local_268 = plStack_360;
              plStack_260 = local_358;
              if (sStack_330 != CONCAT44(uStack_104,local_108) << 2) {
                local_b5 = 1;
                    // WARNING: Subroutine does not return
                core::slice::impl$0::copy_from_slice::len_mismatch_fail();
              }
              memcpy((void *)CONCAT26(uStack_338._6_2_,(undefined6)uStack_338),plStack_110,
                     sStack_330);
              local_b5 = 1;
              rhapsody::raw::buffer::BufferDesc::new_cpu_to_gpu();
              local_b5 = 1;
              rhapsody::raw::device::GraphicsDevice::create_mapped_buffer();
              if ((int)local_368 != 0xb) {
                local_738 = (undefined6)local_328;
                cStack_732 = (char)((ulonglong)local_328 >> 0x30);
                uStack_731 = (undefined1)((ulonglong)local_328 >> 0x38);
                uStack_730 = uStack_320;
                uStack_748 = (undefined6)uStack_338;
                cStack_742 = (char)uStack_338._6_2_;
                uStack_741 = (undefined1)((ushort)uStack_338._6_2_ >> 8);
                uStack_740 = (undefined6)sStack_330;
                uStack_73a = (undefined2)(sStack_330 >> 0x30);
                local_758 = local_348;
                uStack_750 = uStack_340;
                uStack_74a = uStack_33a;
                local_768 = local_358;
                plStack_760 = plStack_350;
                local_778 = local_368;
                plStack_770 = plStack_360;
                    // WARNING: Subroutine does not return
                core::result::unwrap_failed();
              }
              local_1f8 = (longlong *)CONCAT26(uStack_33a,uStack_340);
              local_208 = plStack_350;
              plStack_200 = local_348;
              local_218 = plStack_360;
              plStack_210 = local_358;
              if (sStack_330 != lStack_f0 * 0x14) {
                local_b1 = 1;
                local_b2 = 1;
                    // WARNING: Subroutine does not return
                core::slice::impl$0::copy_from_slice::len_mismatch_fail();
              }
              memcpy((void *)CONCAT26(uStack_338._6_2_,(undefined6)uStack_338),local_f8,sStack_330);
              local_b1 = 1;
              local_b2 = 1;
              toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
              local_b1 = 1;
              local_b2 = 1;
              egui::context::Context::
              write<f32,egui::context::impl_13::input::closure_env_0<f32,egui::context::impl_15::pixels_per_point::closure_env_0>_>
                        ();
              local_b1 = 1;
              local_b2 = 1;
              country_core::render::pass_builder::draw_stream::DrawPacket::new();
              local_b1 = 1;
              local_b2 = 1;
              country_core::render::pass_builder::draw_builder::DrawBuilder::new();
              std::alloc::__default_lib_allocator::__rust_alloc();
              if (extraout_RAX_13 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::raw_vec::handle_error();
              }
              *extraout_RAX_13 =
                   CONCAT44((float)(extraout_RAX_10 >> 0x20) / extraout_XMM0_Da,
                            (float)(extraout_RAX_10 & 0xffffffff) / extraout_XMM0_Da);
              if (CONCAT26(uStack_74a,uStack_750) != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              uStack_750 = 8;
              uStack_74a = 0;
              uStack_748 = SUB86(extraout_RAX_13,0);
              cStack_742 = (char)((ulonglong)extraout_RAX_13 >> 0x30);
              uStack_741 = (undefined1)((ulonglong)extraout_RAX_13 >> 0x38);
              uStack_740 = 8;
              uStack_73a = 0;
              memcpy(&local_368,&local_778,0xf8);
              local_778 = (longlong *)CONCAT44(local_778._4_4_,2);
              local_388 = lVar27 + uVar29 * -0x48 + -0x38;
              country_core::render::pass_builder::draw_builder::impl$3::
              bind_texture_to_draw_builder<ref_<rhapsody::raw::texture::Texture>_>();
              memcpy(&local_778,&local_368,0xf8);
              local_b1 = 1;
              local_b2 = 1;
              country_core::render::pass_builder::draw_builder::DrawBuilder::
              bind_external_storage_buffer();
              local_2c0 = local_2c0 & 0x7f;
              local_2bd = local_2bd & 0x9f | 0x40;
              memcpy(&local_778,&local_368,0xf8);
              local_b1 = 1;
              local_b2 = 1;
              country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao_indexed();
              local_758 = (longlong *)local_248;
              local_768 = local_258;
              plStack_760 = plStack_250;
              local_778 = local_268;
              plStack_770 = plStack_260;
              lVar27 = local_d8[0xb];
              if (lVar27 == local_d8[9]) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                          ();
              }
              lVar30 = local_d8[10];
              *(longlong **)(lVar30 + 0x20 + lVar27 * 0x28) = local_758;
              puVar4 = (undefined8 *)(lVar30 + 0x10 + lVar27 * 0x28);
              *puVar4 = local_768;
              puVar4[1] = plStack_760;
              puVar4 = (undefined8 *)(lVar30 + lVar27 * 0x28);
              *puVar4 = local_778;
              puVar4[1] = plStack_770;
              lVar30 = lVar27 + 1;
              local_d8[0xb] = lVar30;
              local_758 = local_1f8;
              local_768 = local_208;
              plStack_760 = plStack_200;
              local_778 = local_218;
              plStack_770 = plStack_210;
              if (lVar30 == local_d8[9]) {
                alloc::raw_vec::RawVec::
                grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                          ();
              }
              lVar26 = local_d8[10];
              *(longlong **)(lVar26 + 0x20 + lVar30 * 0x28) = local_758;
              puVar4 = (undefined8 *)(lVar26 + 0x10 + lVar30 * 0x28);
              *puVar4 = local_768;
              puVar4[1] = plStack_760;
              puVar4 = (undefined8 *)(lVar26 + lVar30 * 0x28);
              *puVar4 = local_778;
              puVar4[1] = plStack_770;
              local_d8[0xb] = lVar27 + 2;
              if (local_118 != (undefined8 *)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (CONCAT44(uStack_fc,uStack_100) != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if ((int)local_188 == 2) {
                LOCK();
                *local_180 = *local_180 + -1;
                UNLOCK();
                if (*local_180 == 0) {
                  alloc::sync::Arc::
                  drop_slow<dyn_<core::convert::AsRef<slice2_<u8>_>,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
                            ();
                }
              }
              goto LAB_1400deb3e;
            }
            uVar28 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar28);
          }
          auVar47[0] = -(*pcVar3 == -1);
          auVar47[1] = -(pcVar3[1] == -1);
          auVar47[2] = -(pcVar3[2] == -1);
          auVar47[3] = -(pcVar3[3] == -1);
          auVar47[4] = -(pcVar3[4] == -1);
          auVar47[5] = -(pcVar3[5] == -1);
          auVar47[6] = -(pcVar3[6] == -1);
          auVar47[7] = -(pcVar3[7] == -1);
          auVar47[8] = -(pcVar3[8] == -1);
          auVar47[9] = -(pcVar3[9] == -1);
          auVar47[10] = -(pcVar3[10] == -1);
          auVar47[0xb] = -(pcVar3[0xb] == -1);
          auVar47[0xc] = -(pcVar3[0xc] == -1);
          auVar47[0xd] = -(pcVar3[0xd] == -1);
          auVar47[0xe] = -(pcVar3[0xe] == -1);
          auVar47[0xf] = -(pcVar3[0xf] == -1);
          if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 || (SUB161(auVar47 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar47 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar47 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar47 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar47 >> 0x2f,0) & 1) != 0) || (SUB161(auVar47 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar47 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar47 >> 0x47,0) & 1) != 0) || (SUB161(auVar47 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar47 >> 0x57,0) & 1) != 0) || (SUB161(auVar47 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar47 >> 0x67,0) & 1) != 0) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') break;
          uVar23 = uVar23 + lVar30 + 0x10;
          lVar30 = lVar30 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    if (puStack_1d0 != (undefined8 *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (CONCAT44(uStack_fc,uStack_100) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if ((int)local_188 == 2) {
      LOCK();
      *local_180 = *local_180 + -1;
      UNLOCK();
      if (*local_180 == 0) {
        alloc::sync::Arc::
        drop_slow<dyn_<core::convert::AsRef<slice2_<u8>_>,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
                  ();
      }
    }
    goto LAB_1400deb3e;
  }
LAB_1400df24a:
  local_b4 = 1;
  core::ptr::
  drop_in_place<alloc::vec::into_iter::IntoIter<epaint::ClippedPrimitive,alloc::alloc::Global>_>();
  memcpy(&local_778,&local_b60,1000);
  local_b4 = 0;
  country_core::render::pass_builder::draw_stream::RasterPass::render();
  local_1c8 = local_d8 + 6;
  plStack_1c0 = (longlong *)local_d8[8];
  local_d8[8] = 0;
  puVar25 = (undefined8 *)local_d8[7] + (longlong)plStack_1c0 * 2;
  local_1b8 = (longlong *)0x0;
  puVar4 = (undefined8 *)local_d8[7];
  puStack_1d0 = puVar25;
  while( true ) {
    if (puVar4 == puVar25) {
      return;
    }
    local_128 = (longlong *)*puVar4;
    plStack_120 = (longlong *)puVar4[1];
    local_1d8 = puVar4 + 2;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<enum2_<epaint::TextureId>_>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<enum2_<epaint::TextureId>,rhapsody::raw::texture::Texture>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<enum2_<epaint::TextureId>,enum2_<epaint::TextureId>,rhapsody::raw::texture::Texture>_>
              ();
    plVar24 = local_768;
    if ((int)local_778 == 2) break;
    uStack_338._0_6_ = local_738;
    plStack_350 = (longlong *)CONCAT26(uStack_74a,uStack_750);
    local_348 = (longlong *)CONCAT17(uStack_741,CONCAT16(cStack_742,uStack_748));
    uStack_340 = uStack_740;
    uStack_33a = uStack_73a;
    local_358 = local_758;
    local_368 = local_768;
    plStack_360 = plStack_760;
    if (cStack_732 == '\a') break;
    uStack_748 = local_738;
    uStack_750 = uStack_740;
    uStack_74a = uStack_73a;
    local_768 = local_758;
    local_778 = plVar24;
    plStack_770 = plStack_760;
    cStack_742 = cStack_732;
    uStack_741 = uStack_731;
    plStack_760 = plStack_350;
    local_758 = local_348;
    bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
    rhapsody::raw::managed::handle::impl$6::drop();
    puVar4 = puVar4 + 2;
  }
                    // WARNING: Subroutine does not return
  core::option::expect_failed();
}

