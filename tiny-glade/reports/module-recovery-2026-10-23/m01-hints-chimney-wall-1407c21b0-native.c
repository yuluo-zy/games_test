
void __cdecl system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys(void)

{
  byte *pbVar1;
  char *pcVar2;
  undefined8 *puVar3;
  float *pfVar4;
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  undefined4 *puVar8;
  undefined8 uVar9;
  code *pcVar10;
  uint uVar11;
  undefined1 auVar12 [12];
  char extraout_AL;
  char extraout_AL_00;
  undefined1 extraout_AL_01;
  byte extraout_AL_02;
  uint uVar13;
  uint extraout_EAX;
  undefined4 extraout_EAX_00;
  undefined4 extraout_EAX_01;
  uint uVar14;
  uint extraout_EAX_02;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong *extraout_RAX_05;
  float *extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  uint *extraout_RAX_11;
  longlong extraout_RAX_12;
  longlong *_Dst;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong *extraout_RAX_16;
  longlong *plVar15;
  ulonglong in_RDX;
  undefined1 (*pauVar16) [16];
  longlong lVar17;
  ulonglong uVar18;
  undefined1 (*pauVar19) [16];
  undefined1 in_R8 [8];
  undefined *puVar20;
  longlong lVar21;
  longlong lVar22;
  int iVar23;
  ushort uVar24;
  ulonglong uVar25;
  ulonglong uVar26;
  longlong lVar27;
  longlong lVar28;
  undefined1 (*pauVar29) [16];
  bool bVar30;
  byte bVar31;
  char cVar32;
  char cVar36;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  char cVar34;
  char cVar35;
  undefined1 auVar33 [16];
  float fVar37;
  undefined1 extraout_XMM0 [16];
  undefined1 auVar38 [16];
  float fVar39;
  undefined1 auVar40 [16];
  float fVar41;
  float fVar42;
  float fVar43;
  float fVar44;
  float fVar45;
  undefined1 unaff_XMM6_Ba;
  char cVar46;
  undefined1 uVar47;
  undefined1 unaff_XMM6_Bb;
  char cVar48;
  undefined1 uVar49;
  undefined1 unaff_XMM6_Bc;
  char cVar50;
  undefined1 uVar51;
  undefined1 unaff_XMM6_Bd;
  char cVar52;
  undefined1 uVar53;
  undefined1 unaff_XMM6_Be;
  char cVar54;
  undefined1 uVar55;
  undefined1 unaff_XMM6_Bf;
  char cVar56;
  undefined1 uVar57;
  undefined1 uVar58;
  undefined1 unaff_XMM6_Bg;
  char cVar59;
  undefined1 uVar60;
  undefined1 unaff_XMM6_Bh;
  char cVar61;
  undefined1 uVar62;
  undefined1 unaff_XMM6_Bi;
  char cVar63;
  undefined1 uVar64;
  undefined1 unaff_XMM6_Bj;
  char cVar65;
  undefined1 uVar66;
  undefined1 unaff_XMM6_Bk;
  char cVar67;
  undefined1 uVar68;
  undefined1 unaff_XMM6_Bl;
  char cVar69;
  undefined1 uVar70;
  undefined1 unaff_XMM6_Bm;
  char cVar71;
  undefined1 uVar72;
  undefined1 unaff_XMM6_Bn;
  char cVar73;
  undefined1 uVar74;
  undefined1 unaff_XMM6_Bo;
  char cVar75;
  undefined1 uVar76;
  undefined1 unaff_XMM6_Bp;
  char cVar77;
  undefined1 uVar78;
  float fVar79;
  float fVar80;
  float fVar81;
  float fVar82;
  float fVar83;
  float fVar84;
  float fVar85;
  float fVar86;
  float fVar87;
  float fVar88;
  float fVar89;
  float fVar90;
  float fVar91;
  longlong in_stack_00000028;
  float local_458;
  float fStack_454;
  float fStack_450;
  float fStack_44c;
  undefined8 local_448;
  float fStack_440;
  undefined8 uStack_430;
  undefined8 local_3d0;
  undefined4 local_3c8;
  undefined2 local_3c4;
  undefined1 uStack_3c2;
  undefined5 uStack_3c1;
  undefined4 local_3bc;
  float local_3b8;
  float fStack_3b4;
  float fStack_3b0;
  float fStack_3ac;
  undefined8 local_3a8;
  undefined8 uStack_3a0;
  undefined8 local_398;
  undefined8 uStack_390;
  float local_388;
  float fStack_384;
  float fStack_380;
  undefined4 uStack_37c;
  undefined8 local_378;
  undefined8 uStack_370;
  undefined8 local_368;
  undefined8 uStack_360;
  undefined8 local_358;
  undefined8 uStack_350;
  longlong local_348;
  longlong lStack_340;
  longlong local_330;
  longlong local_328;
  undefined8 local_320;
  uint7 local_318;
  undefined1 uStack_311;
  undefined7 uStack_310;
  undefined7 local_308;
  undefined1 uStack_301;
  undefined4 local_300;
  undefined3 uStack_2fc;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined4 local_2e8;
  longlong local_2d8;
  undefined4 local_2c4;
  undefined4 local_2c0;
  float local_2bc;
  undefined8 local_2b8;
  undefined4 local_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  ulonglong local_298;
  float *local_290;
  byte local_288;
  undefined1 uStack_287;
  uint5 uStack_286;
  undefined1 uStack_281;
  undefined7 uStack_280;
  undefined1 uStack_279;
  ulonglong local_278;
  longlong local_268;
  ulonglong local_260;
  longlong local_258;
  longlong local_250;
  float local_248;
  float fStack_244;
  float fStack_240;
  float fStack_23c;
  undefined8 local_238;
  float fStack_230;
  float fStack_22c;
  float local_228;
  float fStack_224;
  undefined4 uStack_220;
  undefined4 uStack_21c;
  byte local_218;
  undefined2 uStack_217;
  undefined4 uStack_215;
  uint uStack_211;
  uint uStack_20d;
  undefined1 uStack_209;
  code *local_208;
  undefined1 auStack_200 [12];
  float fStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  ushort uStack_1e0;
  undefined2 uStack_1de;
  undefined4 uStack_1dc;
  longlong local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  longlong local_198;
  longlong lStack_190;
  float *local_188;
  undefined2 local_177;
  undefined1 local_175;
  uint local_174;
  int local_170;
  longlong *local_168;
  uint7 local_160;
  undefined1 uStack_159;
  undefined7 uStack_158;
  undefined1 uStack_151;
  longlong local_150;
  longlong lStack_148;
  longlong local_140;
  longlong local_138;
  uint local_130;
  float local_12c;
  undefined1 (*local_128) [16];
  longlong *local_120;
  undefined1 local_118 [8];
  float fStack_110;
  float fStack_10c;
  undefined8 local_108;
  float fStack_100;
  float fStack_fc;
  uint local_f8;
  undefined1 local_f3;
  byte local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_2d8 = 0;
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') {
      return;
    }
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x48) {
          if ((char)(&DAT_142a86458)[in_RDX] < -0x40) {
LAB_1407c2277:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x48) goto LAB_1407c2277;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x48) {
          if ("stem_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::f"[in_RDX] <
              -0x40) {
LAB_1407c22e4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x48) goto LAB_1407c22e4;
      }
    }
    local_118 = in_R8;
    puVar20 = &DAT_142a865ad;
    do {
      bVar31 = puVar20[-1];
      uVar13 = (uint)(char)bVar31;
      if ((char)bVar31 < '\0') {
        bVar5 = puVar20[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar20[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar20 + -4;
            puVar20 = puVar20 + -4;
            uVar13 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar20 = puVar20 + -3;
            uVar13 = bVar6 & 0xf;
          }
          uVar13 = bVar5 & 0x3f | uVar13 << 6;
        }
        else {
          puVar20 = puVar20 + -2;
          uVar13 = bVar5 & 0x1f;
        }
        uVar13 = (uint)(bVar31 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1407c23b4;
LAB_1407c23c5:
        if (puVar20 == &UNK_142a8656f) break;
        if (puVar20 + -0x142a8656f < (undefined *)0x3d) {
          if (-0x41 < (char)puVar20[1]) break;
        }
        else if (puVar20 + -0x142a8656f == (undefined *)0x3d) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar20 = puVar20 + -1;
      if (uVar13 == 0x5c) goto LAB_1407c23c5;
LAB_1407c23b4:
      if (uVar13 == 0x2f) goto LAB_1407c23c5;
    } while (puVar20 != &DAT_142a86570);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar15 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar15 = extraout_RAX_13, extraout_RAX_13 == (longlong *)0x0))
    goto LAB_1407c44ef;
    if (*plVar15 != 0) goto LAB_1407c44fb;
    *plVar15 = -1;
    local_168 = plVar15;
    puffin::ThreadProfiler::begin_scope();
    *local_168 = *local_168 + 1;
    local_2d8 = 1;
    in_R8 = local_118;
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE != '\0') {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_02 == 1) {
        plVar15 = extraout_RAX_02 + 1;
      }
      else if ((*extraout_RAX_02 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar15 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0))
      goto LAB_1407c44ef;
      if (*plVar15 != 0) goto LAB_1407c44fb;
      *plVar15 = -1;
      local_118 = (undefined1  [8])plVar15;
      puffin::ThreadProfiler::end_scope();
      goto LAB_1407c24e1;
    }
  }
  country_core::resources::walls::decorator_subtype::impl$8::from();
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter();
  local_268 = CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
  local_128 = (undefined1 (*) [16])CONCAT44(uStack_1ec,uStack_1f0);
  local_320 = *(undefined8 *)in_R8;
  local_2c4 = *(undefined4 *)(in_stack_00000028 + 0x1c);
  puVar8 = *(undefined4 **)(in_stack_00000028 + 0x10);
  iVar23 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar46 = (char)iVar23;
  cVar48 = (char)((uint)iVar23 >> 8);
  cVar50 = (char)((uint)iVar23 >> 0x10);
  cVar52 = (char)((uint)iVar23 >> 0x18);
  iVar23 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar54 = (char)iVar23;
  cVar56 = (char)((uint)iVar23 >> 8);
  cVar59 = (char)((uint)iVar23 >> 0x10);
  cVar61 = (char)((uint)iVar23 >> 0x18);
  iVar23 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar63 = (char)iVar23;
  cVar65 = (char)((uint)iVar23 >> 8);
  cVar67 = (char)((uint)iVar23 >> 0x10);
  cVar69 = (char)((uint)iVar23 >> 0x18);
  iVar23 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar71 = (char)iVar23;
  cVar73 = (char)((uint)iVar23 >> 8);
  cVar75 = (char)((uint)iVar23 >> 0x10);
  cVar77 = (char)((uint)iVar23 >> 0x18);
  uVar13 = (uint)uStack_1e0;
  lVar17 = auStack_200._0_8_;
  lVar22 = (longlong)stack0xfffffffffffffe08;
  local_f2 = local_218;
  local_258 = local_1d8;
  uVar25 = local_260;
  local_260 = (ulonglong)local_208;
joined_r0x0001407c261b:
  local_140 = lVar17;
  local_138 = lVar22;
  if (local_260 != 2) {
    lVar21 = lVar17;
    if (lVar22 != 0) {
      if (local_258 == 0) {
        local_258 = 0;
      }
      else {
        if ((short)uVar13 == 0) {
          do {
            auVar33 = *local_128;
            uVar24 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
            lVar22 = lVar22 + -0x980;
            local_128 = local_128 + 1;
          } while (uVar24 == 0xffff);
          uVar13 = ~(uint)uVar24;
          local_f8 = uVar13 & -(uint)uVar24 - 2;
        }
        else {
          local_f8 = uVar13 - 1 & uVar13;
        }
        uVar14 = 0;
        for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
          uVar14 = uVar14 + 1;
        }
        local_138 = lVar22;
        country_core::resources::walls::decorator_storage::impl$16::from();
        local_258 = local_258 + -1;
        lVar21 = local_140;
        uVar13 = local_f8;
        if (extraout_RAX_03 != 3) {
          lVar21 = (ulonglong)uVar14 * -0x98 + local_138 + -0x90;
          local_328 = lVar22;
          goto LAB_1407c280f;
        }
      }
    }
    uVar25 = local_260;
    local_140 = lVar21;
    if ((local_260 & 1) == 0) {
      local_138 = 0;
      lVar17 = lVar21;
    }
    else {
      local_138 = 0;
      lVar17 = 0;
      if (lVar21 != 0) {
        local_138 = 0;
        local_140 = 0;
        local_f8 = uVar13;
        goto LAB_1407c280f;
      }
    }
  }
  local_260 = uVar25;
  lVar21 = local_268;
  if ((local_f2 & local_268 != 0) != 0) {
    local_260 = 2;
    local_268 = 0;
    local_140 = lVar17;
    local_f8 = uVar13;
LAB_1407c280f:
    country_core::resources::walls::decorator_storage::Decorators::iter_modified();
    lVar28 = CONCAT17((undefined1)uStack_211,CONCAT43(uStack_215,CONCAT21(uStack_217,local_218)));
    pauVar29 = (undefined1 (*) [16])CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
    local_168 = stack0xfffffffffffffe08;
    local_330 = CONCAT44(uStack_1ec,uStack_1f0);
    uVar14 = (uint)(ushort)auStack_200._0_2_;
LAB_1407c2866:
    uVar13 = local_f8;
    lVar17 = local_140;
    lVar22 = local_138;
    uVar25 = local_260;
    if (local_168 != (longlong *)0x0) {
      if ((short)uVar14 == 0) {
        do {
          auVar33 = *pauVar29;
          uVar24 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
          lVar28 = lVar28 + -0x40;
          pauVar29 = pauVar29 + 1;
        } while (uVar24 == 0xffff);
        uVar14 = ~(uint)uVar24;
        local_130 = uVar14 & -(uint)uVar24 - 2;
      }
      else {
        local_130 = uVar14 - 1 & uVar14;
        if (lVar28 == 0) goto joined_r0x0001407c261b;
      }
      iVar23 = 0;
      for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
        iVar23 = iVar23 + 1;
      }
      uVar7 = *(undefined4 *)((lVar28 - (ulonglong)(uint)(iVar23 << 2)) + -4);
      local_218 = (byte)uVar7;
      uStack_217 = (undefined2)((uint)uVar7 >> 8);
      uStack_215._0_1_ = (undefined1)((uint)uVar7 >> 0x18);
      if (*(longlong *)(local_330 + 0x48) != 0) {
        local_118._0_4_ = uVar7;
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        local_168 = (longlong *)((longlong)local_168 + -1);
        lVar17 = *(longlong *)(local_330 + 0x30);
        bVar31 = (byte)(extraout_RAX_04 >> 0x39);
        auVar33 = ZEXT216(CONCAT11(bVar31,bVar31));
        auVar33 = pshuflw(auVar33,auVar33,0);
        lVar22 = 0;
        uVar25 = extraout_RAX_04;
        while( true ) {
          uVar25 = *(ulonglong *)(local_330 + 0x38) & uVar25;
          pcVar2 = (char *)(lVar17 + uVar25);
          cVar32 = auVar33[0];
          auVar40[0] = -(*pcVar2 == cVar32);
          cVar34 = auVar33[1];
          auVar40[1] = -(pcVar2[1] == cVar34);
          cVar35 = auVar33[2];
          auVar40[2] = -(pcVar2[2] == cVar35);
          cVar36 = auVar33[3];
          auVar40[3] = -(pcVar2[3] == cVar36);
          auVar40[4] = -(pcVar2[4] == cVar32);
          auVar40[5] = -(pcVar2[5] == cVar34);
          auVar40[6] = -(pcVar2[6] == cVar35);
          auVar40[7] = -(pcVar2[7] == cVar36);
          auVar40[8] = -(pcVar2[8] == cVar32);
          auVar40[9] = -(pcVar2[9] == cVar34);
          auVar40[10] = -(pcVar2[10] == cVar35);
          auVar40[0xb] = -(pcVar2[0xb] == cVar36);
          auVar40[0xc] = -(pcVar2[0xc] == cVar32);
          auVar40[0xd] = -(pcVar2[0xd] == cVar34);
          auVar40[0xe] = -(pcVar2[0xe] == cVar35);
          auVar40[0xf] = -(pcVar2[0xf] == cVar36);
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
          uVar13 = (uint)uVar24;
          while (uVar24 != 0) {
            uVar14 = 0;
            for (uVar11 = uVar13; (uVar11 & 1) == 0; uVar11 = uVar11 >> 1 | 0x80000000) {
              uVar14 = uVar14 + 1;
            }
            lVar27 = (uVar14 + uVar25 & *(ulonglong *)(local_330 + 0x38)) * -0x50;
            if (local_118._0_4_ == *(int *)(lVar17 + -0x50 + lVar27)) {
              country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
              uVar14 = local_130;
              if (1 < (byte)(extraout_AL_00 - 0x13U)) goto LAB_1407c2866;
              lVar17 = lVar17 + lVar27;
              uVar57 = *(undefined1 *)(lVar17 + -1);
              country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms
              ::transform();
              *puVar8 = local_2c4;
              blueprint::DecoratorBlueprints::insert();
              local_2a8 = 0;
              uStack_2a0 = 4;
              local_298 = 0;
              local_250 = CONCAT44(local_250._4_4_,*(undefined4 *)(lVar17 + -0x20));
              local_f1 = 1;
              local_120 = extraout_RAX_05;
              local_f3 = uVar57;
              country_core::resources::walls::decorator::DecoratorExtraInfo::as_chimney();
              lVar17 = local_238;
              fVar79 = fStack_23c;
              fVar42 = fStack_240;
              fVar39 = fStack_244;
              fVar90 = local_248;
              if (*local_120 != 0 || local_120[1] != 0) {
                local_218 = 0x28;
                uStack_217 = 0xa917;
                uStack_215 = 0x142;
                uStack_211 = 0x100;
                uStack_20d = 0;
                uStack_209 = 0;
                local_208 = (code *)0x8;
                _auStack_200 = (undefined1  [16])0x0;
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              local_120[1] = 0;
              *local_120 = 1;
              local_120[2] = CONCAT44(fStack_244,local_248);
              local_120[3] = CONCAT44(fStack_23c,fStack_240);
              local_120[4] = local_238;
              local_120[5] = CONCAT44(fStack_22c,fStack_230);
              local_120[6] = CONCAT44(fStack_224,local_228);
              local_120[7] = CONCAT44(uStack_21c,uStack_220);
              local_290 = extraout_RAX_06;
              if (extraout_AL_00 != '\x13') {
                local_2f8._0_4_ = fStack_22c;
                local_2f8._4_4_ = local_228;
                uStack_2f0 = CONCAT44(uStack_220,fStack_224);
                fVar91 = fStack_23c * fStack_23c;
                fVar43 = fStack_240 * fStack_240 + fStack_244 * fStack_244 + local_248 * local_248;
                fVar80 = fStack_240 * 0.28 + fStack_244 * 0.0 + local_248 * 0.0;
                fVar80 = fVar80 + fVar80;
                fVar83 = fVar80 * local_248;
                fVar37 = fVar80 * fStack_244;
                fVar41 = fVar80 * fStack_23c;
                local_108 = CONCAT44(local_248,fStack_240);
                fStack_100 = fStack_244;
                fStack_fc = fStack_23c;
                fVar44 = fStack_240 * 0.0;
                fVar45 = fStack_244 * 0.28;
                fVar81 = fStack_23c * 0.0;
                fVar87 = local_248 * -0.28;
                fVar88 = fStack_240 * -0.0;
                fVar89 = fStack_23c * 0.0;
                fVar84 = local_248 + local_248;
                fVar85 = fStack_244 + fStack_244;
                fVar86 = fStack_23c + fStack_23c;
                fVar80 = (fStack_244 * -0.0 + local_248 * 0.0) * fVar86 +
                         fVar80 * fStack_240 + (fVar91 - fVar43) * 0.28;
                fVar82 = fVar80 + fStack_230;
                local_2e8 = uStack_21c;
                local_218 = SUB41(fStack_22c,0);
                uStack_217 = (undefined2)((uint)fStack_22c >> 8);
                uStack_215 = (undefined4)(CONCAT44(local_228,fStack_22c) >> 0x18);
                uStack_211._0_1_ = (undefined1)((uint)local_228 >> 0x18);
                uStack_211._1_3_ = SUB43(fStack_224,0);
                uStack_20d = (uint)(CONCAT44(uStack_220,fStack_224) >> 0x18);
                uStack_209 = (undefined1)((uint)uStack_220 >> 0x18);
                local_208 = (code *)CONCAT44(local_208._4_4_,uStack_21c);
                fStack_3ac = (float)((uint)fStack_3ac & 0xff000000);
                local_3b8 = (float)((uint)local_3b8 & 0xffffff);
                fStack_3b4 = 0.0;
                fStack_3b0 = 0.0;
                uStack_158 = 0;
                local_160 = (uint7)(uint)local_3b8;
                uStack_159 = 0;
                lVar22 = local_120[0x18];
                if (lVar22 == local_120[0x16]) {
                  local_f1 = 1;
                  alloc::raw_vec::RawVec::
                  grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                            ();
                }
                fVar83 = (float)lVar17 +
                         (fVar88 + fVar45) * fVar86 + fVar83 + (fVar91 - fVar43) * 0.0;
                fVar37 = (float)((ulonglong)lVar17 >> 0x20) +
                         (fVar87 + fVar44) * fVar86 + fVar37 + (fVar91 - fVar43) * 0.0;
                uVar57 = (undefined1)((uint)fVar37 >> 8);
                uVar47 = (undefined1)((uint)fVar37 >> 0x10);
                uVar49 = (undefined1)((uint)fVar37 >> 0x18);
                fVar80 = fVar80 + 0.0;
                fVar41 = (fVar89 + fVar81) * fVar86 + fVar41 + (fVar91 - fVar43) * 0.0 + 0.0;
                lVar17 = local_120[0x17];
                lVar27 = lVar22 * 0x160;
                *(undefined8 *)(lVar17 + lVar27) = 0;
                *(undefined8 *)(lVar17 + 0x10 + lVar27) = 0;
                *(undefined8 *)(lVar17 + 0x20 + lVar27) = 0;
                *(undefined4 *)(lVar17 + 0x30 + lVar27) = 0;
                *(undefined4 *)(lVar17 + 0x40 + lVar27) = 0;
                *(undefined4 *)(lVar17 + 0x80 + lVar27) = 0;
                *(undefined4 *)(lVar17 + 0x90 + lVar27) = 0;
                *(char **)(lVar17 + 0xa0 + lVar27) = "decorators/vent_pipe";
                *(undefined8 *)(lVar17 + 0xa8 + lVar27) = 0x14;
                *(undefined **)(lVar17 + 0xb0 + lVar27) =
                     &anon_b17bdb6b936b48c513e8fa54c354b9a3_0_llvm_3328826292648170336;
                *(undefined8 *)(lVar17 + 0xb8 + lVar27) = 0x17;
                pfVar4 = (float *)(lVar17 + 0xc0 + lVar27);
                *pfVar4 = fVar90;
                pfVar4[1] = fVar39;
                pfVar4[2] = fVar42;
                pfVar4[3] = fVar79;
                *(ulonglong *)(lVar17 + 0xd0 + lVar27) =
                     CONCAT17(uVar49,CONCAT16(uVar47,CONCAT15(uVar57,CONCAT14(SUB41(fVar37,0),fVar83
                                                                             ))));
                *(float *)(lVar17 + 0xd8 + lVar27) = fVar82;
                puVar3 = (undefined8 *)(lVar17 + 0xdc + lVar27);
                *puVar3 = CONCAT17((undefined1)uStack_211,
                                   CONCAT43(uStack_215,CONCAT21(uStack_217,local_218)));
                puVar3[1] = CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
                *(undefined4 *)(lVar17 + 0xec + lVar27) = local_208._0_4_;
                *(undefined4 *)(lVar17 + 0xf0 + lVar27) = 0;
                *(undefined4 *)(lVar17 + 0xf8 + lVar27) = 0;
                *(undefined1 *)(lVar17 + 0x114 + lVar27) = 2;
                *(undefined4 *)(lVar17 + 0x118 + lVar27) = 0;
                *(undefined1 *)(lVar17 + 0x13c + lVar27) = 2;
                *(ulonglong *)(lVar17 + 0x144 + lVar27) = CONCAT71(uStack_158,uStack_159);
                *(ulonglong *)(lVar17 + 0x13d + lVar27) = CONCAT17(uStack_159,local_160);
                *(undefined1 *)(lVar17 + 0x14c + lVar27) = 2;
                *(undefined1 *)(lVar17 + 0x14e + lVar27) = local_f3;
                *(undefined2 *)(lVar17 + 0x14f + lVar27) = 0x900;
                local_120[0x18] = lVar22 + 1;
                local_f1 = 1;
                country_core::asset_libraries::mesh_library::AssetMeshLibrary::get_by_name();
                uVar9 = CONCAT17(uVar49,CONCAT16(uVar47,CONCAT15(uVar57,CONCAT14(SUB41(fVar37,0),
                                                                                 fVar83))));
                auVar12[8] = SUB41(fVar80,0);
                auVar12._0_8_ = uVar9;
                auVar12[9] = (char)((uint)fVar80 >> 8);
                auVar12[10] = (char)((uint)fVar80 >> 0x10);
                auVar12[0xb] = (char)((uint)fVar80 >> 0x18);
                auVar33[0xc] = SUB41(fVar41,0);
                auVar33._0_12_ = auVar12;
                auVar33[0xd] = (char)((uint)fVar41 >> 8);
                auVar33[0xe] = (char)((uint)fVar41 >> 0x10);
                auVar33[0xf] = (char)((uint)fVar41 >> 0x18);
                _local_118 = auVar33;
                if (extraout_RAX_09 == 0) {
                  local_f1 = 1;
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                fVar44 = fVar42 + fVar42;
                fVar41 = fVar90 * fVar84;
                fVar84 = fVar84 * fVar79;
                local_3b8 = (float)local_2f8 * (1.0 - (fVar39 * fVar85 + fVar42 * fVar44));
                fStack_3b4 = (float)local_2f8 * (fVar90 * fVar85 + fVar44 * fVar79);
                fStack_3b0 = (float)local_2f8 * (fVar90 * fVar44 - fVar85 * fVar79);
                fStack_3ac = (float)local_2f8 * 0.0;
                local_118._4_4_ = (undefined4)((ulonglong)uVar9 >> 0x20);
                uVar7 = local_118._4_4_;
                fStack_110 = auVar12._8_4_;
                fVar80 = fStack_110;
                fStack_10c = auVar33._12_4_;
                fVar37 = fStack_10c;
                local_3a8 = CONCAT44(local_2f8._4_4_ * (1.0 - (fVar42 * fVar44 + fVar41)),
                                     local_2f8._4_4_ * (fVar90 * fVar85 - fVar44 * fVar79));
                uStack_3a0 = CONCAT44(local_2f8._4_4_ * 0.0,
                                      local_2f8._4_4_ * (fVar39 * fVar44 + fVar84));
                local_398 = CONCAT44((float)uStack_2f0 * (fVar39 * fVar44 - fVar84),
                                     (float)uStack_2f0 * (fVar85 * fVar79 + fVar90 * fVar44));
                uStack_390 = CONCAT44((float)uStack_2f0 * 0.0,
                                      (float)uStack_2f0 * (1.0 - (fVar41 + fVar39 * fVar85)));
                fStack_384 = (float)local_118._4_4_;
                uStack_37c = 0x3f800000;
                local_f1 = 1;
                local_388 = fVar83;
                fStack_380 = fVar82;
                country_core::render::mesh::Mesh::as_triangles();
                local_250 = CONCAT17(uStack_151,uStack_158);
                lStack_190 = local_250 + local_150 * 0x24;
                local_218 = 0;
                uStack_217 = 0;
                uStack_215 = 0;
                uStack_211 = (uint)uStack_211._1_3_ << 8;
                local_1d8 = 0;
                local_188 = &local_3b8;
                local_198 = local_250;
                alloc::vec::spec_extend::impl$1::
                spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array_<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env_3>,alloc::alloc::Global>
                          ();
                plVar15 = local_120;
                if (CONCAT17(uStack_159,local_160) != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                local_2f8 = 0x3f0000003f000000;
                uStack_2f0 = CONCAT44(uStack_2f0._4_4_,0x3f000000);
                uStack_20d = 0;
                uStack_215 = 0;
                uStack_211 = 0;
                uVar57 = *(undefined1 *)(local_290 + 1);
                local_318 = (uint7)CONCAT21(uStack_217,local_218);
                uStack_310 = 0;
                uStack_311 = 0;
                lVar17 = plVar15[0x18];
                if (lVar17 == plVar15[0x16]) {
                  local_f1 = 1;
                  alloc::raw_vec::RawVec::
                  grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
                            ();
                }
                fVar44 = fVar42 * 0.58 + fVar39 * 1.42 + fVar90 * -0.07;
                fVar44 = fVar44 + fVar44;
                fVar45 = fVar86 * (local_108._4_4_ * 1.42 + fVar39 * 0.07) +
                         fVar44 * fVar42 + (fVar91 - fVar43) * 0.58;
                fVar83 = fVar83 + fVar86 * (fStack_100 * 0.58 + fVar42 * -1.42) +
                                  fVar44 * fVar90 + (fVar91 - fVar43) * -0.07;
                uVar47 = SUB41(fVar83,0);
                uVar49 = (undefined1)((uint)fVar83 >> 8);
                uVar51 = (undefined1)((uint)fVar83 >> 0x10);
                uVar53 = (undefined1)((uint)fVar83 >> 0x18);
                fVar41 = (float)uVar7 +
                         fVar86 * ((float)local_108 * -0.07 + fVar90 * -0.58) +
                         fVar44 * fVar39 + (fVar91 - fVar43) * 1.42;
                uVar55 = SUB41(fVar41,0);
                uVar58 = (undefined1)((uint)fVar41 >> 8);
                uVar60 = (undefined1)((uint)fVar41 >> 0x10);
                uVar62 = (undefined1)((uint)fVar41 >> 0x18);
                fVar80 = fVar80 + fVar45;
                uVar64 = SUB41(fVar80,0);
                uVar66 = (undefined1)((uint)fVar80 >> 8);
                uVar68 = (undefined1)((uint)fVar80 >> 0x10);
                uVar70 = (undefined1)((uint)fVar80 >> 0x18);
                fVar37 = fVar37 + fVar86 * (fStack_fc * 0.0 + fVar79 * 0.0) +
                                  fVar44 * fVar79 + (fVar91 - fVar43) * 0.0;
                uVar72 = SUB41(fVar37,0);
                uVar74 = (undefined1)((uint)fVar37 >> 8);
                uVar76 = (undefined1)((uint)fVar37 >> 0x10);
                uVar78 = (undefined1)((uint)fVar37 >> 0x18);
                lVar22 = local_120[0x17];
                lVar27 = lVar17 * 0x160;
                *(undefined8 *)(lVar22 + lVar27) = 0;
                *(undefined8 *)(lVar22 + 0x10 + lVar27) = 0;
                *(undefined8 *)(lVar22 + 0x20 + lVar27) = 0;
                *(undefined4 *)(lVar22 + 0x30 + lVar27) = 0;
                *(undefined8 *)(lVar22 + 0x34 + lVar27) = local_2b8;
                *(undefined4 *)(lVar22 + 0x3c + lVar27) = local_2b0;
                *(undefined4 *)(lVar22 + 0x40 + lVar27) = 0;
                puVar3 = (undefined8 *)(lVar22 + 0x44 + lVar27);
                *puVar3 = CONCAT17((undefined1)uStack_211,
                                   CONCAT43(uStack_215,CONCAT21(uStack_217,local_218)));
                puVar3[1] = CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
                puVar3 = (undefined8 *)(lVar22 + 0x54 + lVar27);
                *puVar3 = local_208;
                puVar3[1] = auStack_200._0_8_;
                puVar3 = (undefined8 *)(lVar22 + 100 + lVar27);
                *puVar3 = stack0xfffffffffffffe08;
                puVar3[1] = CONCAT44(uStack_1ec,uStack_1f0);
                puVar3 = (undefined8 *)(lVar22 + 0x70 + lVar27);
                *puVar3 = CONCAT44(uStack_1e8,uStack_1ec);
                puVar3[1] = CONCAT26(uStack_1de,CONCAT24(uStack_1e0,uStack_1e4));
                *(undefined4 *)(lVar22 + 0x80 + lVar27) = 0;
                *(undefined8 *)(lVar22 + 0x84 + lVar27) = local_3d0;
                *(undefined4 *)(lVar22 + 0x8c + lVar27) = local_3c8;
                *(undefined4 *)(lVar22 + 0x90 + lVar27) = 0;
                *(ulonglong *)(lVar22 + 0x94 + lVar27) =
                     CONCAT53(uStack_3c1,CONCAT12(uStack_3c2,local_3c4));
                *(undefined4 *)(lVar22 + 0x9c + lVar27) = local_3bc;
                *(undefined8 *)(lVar22 + 0xa0 + lVar27) = 1;
                *(undefined8 *)(lVar22 + 0xa8 + lVar27) = 0;
                *(undefined **)(lVar22 + 0xb0 + lVar27) =
                     &anon_b17bdb6b936b48c513e8fa54c354b9a3_2_llvm_3328826292648170336;
                *(undefined8 *)(lVar22 + 0xb8 + lVar27) = 10;
                pfVar4 = (float *)(lVar22 + 0xc0 + lVar27);
                *pfVar4 = fVar90;
                pfVar4[1] = fVar39;
                pfVar4[2] = fVar42;
                pfVar4[3] = fVar79;
                *(ulonglong *)(lVar22 + 0xd0 + lVar27) =
                     CONCAT17(uVar62,CONCAT16(uVar60,CONCAT15(uVar58,CONCAT14(uVar55,fVar83))));
                *(float *)(lVar22 + 0xd8 + lVar27) = fVar45 + fVar82;
                *(undefined4 *)(lVar22 + 0xec + lVar27) = local_2e8;
                puVar3 = (undefined8 *)(lVar22 + 0xdc + lVar27);
                *puVar3 = local_2f8;
                puVar3[1] = uStack_2f0;
                *(undefined4 *)(lVar22 + 0xf0 + lVar27) = 0;
                *(undefined4 *)(lVar22 + 0xf8 + lVar27) = 0;
                *(ulonglong *)(lVar22 + 0x10c + lVar27) = local_278;
                puVar3 = (undefined8 *)(lVar22 + 0xfc + lVar27);
                *puVar3 = CONCAT17(uStack_281,CONCAT52(uStack_286,CONCAT11(uStack_287,local_288)));
                puVar3[1] = CONCAT17(uStack_279,uStack_280);
                *(undefined1 *)(lVar22 + 0x114 + lVar27) = 2;
                *(undefined1 *)(lVar22 + 0x117 + lVar27) = local_175;
                *(undefined2 *)(lVar22 + 0x115 + lVar27) = local_177;
                *(undefined4 *)(lVar22 + 0x118 + lVar27) = 0;
                plVar15 = (longlong *)(lVar22 + 300 + lVar27);
                *plVar15 = local_150;
                plVar15[1] = lStack_148;
                puVar3 = (undefined8 *)(lVar22 + 0x11c + lVar27);
                *puVar3 = CONCAT17(uStack_159,local_160);
                puVar3[1] = CONCAT17(uStack_151,uStack_158);
                *(undefined1 *)(lVar22 + 0x13c + lVar27) = 2;
                *(ulonglong *)(lVar22 + 0x144 + lVar27) = CONCAT71(uStack_310,uStack_311);
                *(ulonglong *)(lVar22 + 0x13d + lVar27) = CONCAT17(uStack_311,local_318);
                *(undefined1 *)(lVar22 + 0x14c + lVar27) = 1;
                *(undefined1 *)(lVar22 + 0x14d + lVar27) = uVar57;
                *(undefined1 *)(lVar22 + 0x14e + lVar27) = local_f3;
                *(undefined2 *)(lVar22 + 0x14f + lVar27) = 0x900;
                *(ulonglong *)(lVar22 + 0x151 + lVar27) = CONCAT17(uStack_301,local_308);
                *(ulonglong *)(lVar22 + 0x158 + lVar27) =
                     CONCAT35(uStack_2fc,CONCAT41(local_300,uStack_301));
                local_120[0x18] = lVar17 + 1;
                goto LAB_1407c4017;
              }
              local_f1 = 1;
              country_core::resources::walls::decorator_storage::DecoratorStorageAddr::as_wall();
              local_f1 = 1;
              country_core::resources::walls::public_walls::PublicWalls::get();
              if (extraout_RAX_07 == 0) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              local_f1 = 1;
              local_108 = extraout_RAX_07;
              country_core::resources::walls::public_wall_state::PublicWallState::max_y();
              local_f1 = 1;
              country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
              lVar17 = *(longlong *)(local_108 + 0x48);
              if (lVar17 == 1) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              if (lVar17 == 0) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              fVar90 = *(float *)(extraout_RAX_08 + 4);
              uVar9 = *(undefined8 *)(local_108 + 0x40);
              local_218 = (byte)uVar9;
              uStack_217 = (undefined2)((ulonglong)uVar9 >> 8);
              uStack_215 = (undefined4)((ulonglong)uVar9 >> 0x18);
              uStack_211._0_1_ = (undefined1)((ulonglong)uVar9 >> 0x38);
              uStack_211._1_3_ = (uint3)lVar17;
              uStack_20d = (uint)((ulonglong)lVar17 >> 0x18);
              uStack_209 = (undefined1)((ulonglong)lVar17 >> 0x38);
              local_208 = core::ops::function::FnOnce::
                          call_once<utils::curve::curve_slice::impl_5::remap_2d::closure_env_0<slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>,tuple_<usize,ref_<glam::f32::vec3::Vec3>_>_>
              ;
              _auStack_200 = ZEXT816(0);
              local_f1 = 1;
              ::utils::curve::curve_slice::CurveSlice::
              extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
                        ();
              uVar25 = (ulonglong)(uint)auStack_200._8_4_;
              if (*(ulonglong *)(local_108 + 0x48) <= uVar25) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              if (*(ulonglong *)(local_108 + 0x48) <= uVar25 + 1) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              fVar80 = 1.0 - fStack_1f4;
              fVar39 = *(float *)(*(longlong *)(local_108 + 0x40) + 4 + uVar25 * 0xc);
              fVar42 = fStack_1f4 *
                       *(float *)(*(longlong *)(local_108 + 0x40) + 4 + (uVar25 + 1) * 0xc);
              fVar79 = extraout_XMM0_Da - fVar90;
              fVar83 = 0.0;
              if (fVar79 <= 0.0) {
                fVar79 = 0.0;
              }
              fVar91 = *local_290 + fVar79 + 3.0;
              fStack_22c = 0.85;
              fStack_224 = 0.85;
              fVar37 = fStack_23c * fStack_23c;
              fVar41 = fStack_240 * fStack_240 + fStack_244 * fStack_244 + local_248 * local_248;
              fVar44 = fStack_240 * 0.33333334 + fStack_244 * 0.0 + local_248 * 0.0;
              fVar44 = fVar44 + fVar44;
              fVar43 = fStack_23c + fStack_23c;
              fVar45 = fStack_240 * 0.15 + fStack_244 * 0.0 + local_248 * 0.0;
              fVar45 = fVar45 + fVar45;
              fVar81 = fVar91 * 0.5;
              local_238 = CONCAT44(local_238._4_4_ + fVar81 +
                                   (local_248 * -0.33333334 + fStack_240 * 0.0) * fVar43 +
                                   fVar44 * fStack_244 + (fVar37 - fVar41) * 0.0 +
                                   (local_248 * -0.15 + fStack_240 * 0.0) * fVar43 +
                                   fVar45 * fStack_244 + (fVar37 - fVar41) * 0.0,
                                   (float)local_238 +
                                   (fStack_240 * -0.0 + fStack_244 * 0.33333334) * fVar43 +
                                   fVar44 * local_248 + (fVar37 - fVar41) * 0.0 +
                                   (fStack_240 * -0.0 + fStack_244 * 0.15) * fVar43 +
                                   fVar45 * local_248 + (fVar37 - fVar41) * 0.0);
              fStack_230 = (fStack_244 * -0.0 + local_248 * 0.0) * fVar43 +
                           fVar45 * fStack_240 + (fVar37 - fVar41) * 0.15 +
                           (fStack_244 * -0.0 + local_248 * 0.0) * fVar43 +
                           fVar44 * fStack_240 + (fVar37 - fVar41) * 0.33333334 + fStack_230;
              local_228 = fVar91;
              if (0.0 < *(float *)(local_108 + 0x280)) {
                lVar17 = *(longlong *)(local_108 + 0x78);
                uVar25 = *(ulonglong *)(local_108 + 0x80);
                local_f1 = 1;
                ::utils::curve::Curve::
                get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>
                          ();
                uVar18 = (ulonglong)extraout_EAX;
                if (uVar25 <= uVar18) {
                  local_f1 = 1;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (uVar25 <= uVar18 + 1) {
                  local_f1 = 1;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                fVar37 = *(float *)(lVar17 + uVar18 * 4);
                if (0.5 < (*(float *)(lVar17 + 4 + uVar18 * 4) - fVar37) * extraout_XMM0_Da_00 +
                          fVar37) goto LAB_1407c396b;
              }
              fVar83 = fVar90 - (fVar80 * fVar39 + fVar42);
              if (5.0 <= fVar83) {
                fVar83 = 0.0;
                goto LAB_1407c396b;
              }
              if (fVar83 <= 1.5) {
                fVar83 = 1.5;
              }
              uStack_390 = CONCAT44(uStack_21c,uStack_220);
              local_3b8 = local_248;
              fStack_3b4 = fStack_244;
              fStack_3b0 = fStack_240;
              fStack_3ac = fStack_23c;
              uStack_3a0 = CONCAT44(0x3fe00000,fStack_230);
              local_398 = CONCAT44(0x3f99999a,fVar83);
              local_3a8._4_4_ = (float)((ulonglong)local_238 >> 0x20);
              local_3a8._0_4_ = (undefined4)local_238;
              local_3a8 = CONCAT44((local_3a8._4_4_ - fVar81) - fVar83 * 0.5,(undefined4)local_3a8);
              local_f1 = 1;
              country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
              local_218 = (byte)extraout_EAX_00;
              uStack_217 = (undefined2)((uint)extraout_EAX_00 >> 8);
              uStack_215._1_3_ = (undefined3)extraout_XMM0_Da_01;
              uStack_215 = CONCAT31(uStack_215._1_3_,(char)((uint)extraout_EAX_00 >> 0x18));
              uStack_211._0_1_ = (undefined1)((uint)extraout_XMM0_Da_01 >> 0x18);
              local_f1 = 1;
              ::utils::wall_space::impl$1::project_unbounded();
              local_12c = uStack_3a0._4_4_;
              fVar39 = (float)local_398;
              local_f1 = 1;
              country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
              lVar17 = *(longlong *)(local_108 + 0x98);
              if (lVar17 == 1) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::option::unwrap_failed();
              }
              if (lVar17 == 0) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              uVar9 = *(undefined8 *)(local_108 + 0x90);
              local_218 = (byte)uVar9;
              uStack_217 = (undefined2)((ulonglong)uVar9 >> 8);
              uStack_215 = (undefined4)((ulonglong)uVar9 >> 0x18);
              uStack_211._0_1_ = (undefined1)((ulonglong)uVar9 >> 0x38);
              uStack_211._1_3_ = (uint3)lVar17;
              uStack_20d = (uint)((ulonglong)lVar17 >> 0x18);
              uStack_209 = (undefined1)((ulonglong)lVar17 >> 0x38);
              local_208 = core::ops::function::FnOnce::
                          call_once<utils::curve::curve_slice::impl_5::remap_2d::closure_env_0<slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>,tuple_<usize,ref_<glam::f32::vec3::Vec3>_>_>
              ;
              _auStack_200 = ZEXT816(0);
              local_f1 = 1;
              local_2c0 = extraout_EAX_01;
              ::utils::curve::curve_slice::CurveSlice::
              extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
                        ();
              uVar25 = (ulonglong)(uint)auStack_200._8_4_;
              if (*(ulonglong *)(local_108 + 0xe8) <= uVar25) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              if (*(ulonglong *)(local_108 + 0xe8) <= uVar25 + 1) {
                local_f1 = 1;
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              fVar90 = fVar90 - fVar83 * 0.5;
              local_2bc = *(float *)(*(longlong *)(local_108 + 0xe0) + uVar25 * 4);
              local_2bc = (*(float *)(*(longlong *)(local_108 + 0xe0) + 4 + uVar25 * 4) - local_2bc)
                          * fStack_1f4 + local_2bc;
              pauVar19 = *(undefined1 (**) [16])(lVar21 + 0x30);
              lVar17 = *(longlong *)(lVar21 + 0x48);
              auVar33 = *pauVar19;
              pauVar16 = pauVar19 + 1;
              local_12c = local_12c * 0.5;
              iVar23 = local_118._0_4_;
              uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                                       (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                                       (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                                       (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar33[0xf] >> 7) << 0xf);
              goto LAB_1407c37c2;
            }
            uVar24 = (ushort)(uVar13 - 1) & (ushort)uVar13;
            uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar24);
          }
          auVar38[0] = -(*pcVar2 == cVar46);
          auVar38[1] = -(pcVar2[1] == cVar48);
          auVar38[2] = -(pcVar2[2] == cVar50);
          auVar38[3] = -(pcVar2[3] == cVar52);
          auVar38[4] = -(pcVar2[4] == cVar54);
          auVar38[5] = -(pcVar2[5] == cVar56);
          auVar38[6] = -(pcVar2[6] == cVar59);
          auVar38[7] = -(pcVar2[7] == cVar61);
          auVar38[8] = -(pcVar2[8] == cVar63);
          auVar38[9] = -(pcVar2[9] == cVar65);
          auVar38[10] = -(pcVar2[10] == cVar67);
          auVar38[0xb] = -(pcVar2[0xb] == cVar69);
          auVar38[0xc] = -(pcVar2[0xc] == cVar71);
          auVar38[0xd] = -(pcVar2[0xd] == cVar73);
          auVar38[0xe] = -(pcVar2[0xe] == cVar75);
          auVar38[0xf] = -(pcVar2[0xf] == cVar77);
          if ((((((((((((((((SUB161(auVar38 >> 7,0) & 1) != 0 || (SUB161(auVar38 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar38 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar38 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar38 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar38 >> 0x2f,0) & 1) != 0) || (SUB161(auVar38 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar38 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar38 >> 0x47,0) & 1) != 0) || (SUB161(auVar38 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar38 >> 0x57,0) & 1) != 0) || (SUB161(auVar38 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar38 >> 0x67,0) & 1) != 0) || (SUB161(auVar38 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar38 >> 0x77,0) & 1) != 0) || auVar38[0xf] < '\0') break;
          uVar25 = uVar25 + lVar22 + 0x10;
          lVar22 = lVar22 + 0x10;
        }
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    goto joined_r0x0001407c261b;
  }
  if (local_2d8 == 0) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_15 == 1) {
    plVar15 = extraout_RAX_15 + 1;
  }
  else if ((*extraout_RAX_15 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar15 = extraout_RAX_16, extraout_RAX_16 == (longlong *)0x0)) {
LAB_1407c44ef:
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  if (*plVar15 == 0) {
    *plVar15 = -1;
    local_118 = (undefined1  [8])plVar15;
    puffin::ThreadProfiler::end_scope();
LAB_1407c24e1:
    *(longlong *)local_118 = *(longlong *)local_118 + 1;
    return;
  }
LAB_1407c44fb:
                    // WARNING: Subroutine does not return
  core::cell::panic_already_borrowed();
  while ((char)uVar26 == '\0') {
LAB_1407c37c2:
    bVar30 = lVar17 == 0;
    lVar17 = lVar17 + -1;
    if (bVar30) {
      if (fVar90 + fVar39 * 0.5 + 0.3 < local_2bc) {
        local_f1 = 1;
        ::utils::curve::Curve::
        get_approx_coord_from_pos<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemanticThresholded3D>
                  ();
        uVar25 = (ulonglong)extraout_EAX_02;
        if (*(ulonglong *)(local_108 + 0xd0) <= uVar25) {
          local_f1 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        if (*(ulonglong *)(local_108 + 0xd0) <= uVar25 + 1) {
          local_f1 = 1;
                    // WARNING: Subroutine does not return
          core::panicking::panic_bounds_check();
        }
        fVar90 = *(float *)(*(longlong *)(local_108 + 200) + uVar25 * 4);
        fVar90 = (*(float *)(*(longlong *)(local_108 + 200) + 4 + uVar25 * 4) - fVar90) *
                 extraout_XMM0_Da_03 + fVar90;
        if ((fVar90 == 0.0) && (!NAN(fVar90))) {
          local_f1 = 1;
          country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
          box_trigger_volume();
          local_198 = CONCAT17(uStack_151,uStack_158);
          lStack_190 = local_198 + local_150 * 0x24;
          local_218 = 0;
          uStack_217 = 0;
          uStack_215 = 0;
          uStack_211 = uStack_211 & 0xffffff00;
          local_1d8 = 0;
          local_108 = local_198;
          alloc::vec::spec_extend::impl$1::
          spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array_<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env_1>,alloc::alloc::Global>
                    ();
          if (CONCAT17(uStack_159,local_160) != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          bVar31 = 1;
          goto LAB_1407c396d;
        }
      }
      break;
    }
    if ((short)uVar13 == 0) {
      do {
        auVar33 = *pauVar16;
        uVar24 = (ushort)(SUB161(auVar33 >> 7,0) & 1) | (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar33[0xf] >> 7) << 0xf;
        pauVar19 = pauVar19 + -0x50;
        pauVar16 = pauVar16 + 1;
      } while (uVar24 == 0xffff);
      uVar14 = ~(uint)uVar24;
      uVar13 = uVar14 & -(uint)uVar24 - 2;
    }
    else {
      uVar14 = uVar13;
      uVar13 = uVar13 - 1 & uVar13;
    }
    uVar11 = 0;
    for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
      uVar11 = uVar11 + 1;
    }
    uVar18 = (ulonglong)uVar11;
    uVar26 = 0;
    uVar25 = (ulonglong)(byte)pauVar19[uVar18 * -5 + -3][10] - 1;
    if (2 < (byte)(pauVar19[uVar18 * -5 + -3][10] - 2)) {
      uVar25 = uVar26;
    }
    if (uVar25 == 0) {
      fVar42 = *(float *)(pauVar19[uVar18 * -5 + -5] + 0xc);
      local_f1 = 1;
      local_174 = uVar13;
      local_170 = *(int *)pauVar19[uVar18 * -5 + -5];
      country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype();
      local_f1 = 1;
      uVar57 = extraout_AL_01;
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      local_f1 = 1;
      local_288 = extraout_AL_02;
      uStack_287 = uVar57;
      std::collections::hash::map::HashMap::
      get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                ();
      if (extraout_RAX_10 == 0) {
        local_160 = (uint7)&local_288;
        uStack_159 = (undefined1)((ulonglong)&local_288 >> 0x38);
        uStack_158 = 0x14125cef0;
        uStack_151 = 0;
        local_218 = 0x80;
        uStack_217 = 0xc21c;
        uStack_215 = 0x142;
        uStack_211 = 0x100;
        uStack_20d = 0;
        uStack_209 = 0;
        local_208 = (code *)&local_160;
        _auStack_200 = ZEXT816(1);
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      fVar80 = *(float *)(extraout_RAX_10 + 0x30);
      local_218 = (byte)local_2c0;
      uStack_217 = (undefined2)((uint)local_2c0 >> 8);
      uStack_215._1_3_ = (undefined3)extraout_XMM0_Da_02;
      uStack_215 = CONCAT31(uStack_215._1_3_,(char)((uint)local_2c0 >> 0x18));
      uStack_211._0_1_ = (undefined1)((uint)extraout_XMM0_Da_02 >> 0x18);
      if ((*(float *)(extraout_RAX_10 + 0x3c) - *(float *)(extraout_RAX_10 + 0x38)) * 0.5 +
          fVar39 * 0.5 <= ABS(fVar42 - fVar90)) {
        uVar26 = 0;
      }
      else {
        local_f1 = 1;
        ::utils::wall_space::impl$1::distance();
        uVar26 = (ulonglong)(extraout_XMM0._0_4_ < fVar80 * 0.5 + local_12c);
      }
      iVar23 = local_118._0_4_;
      uVar13 = local_174;
      if (local_170 == local_118._0_4_) goto LAB_1407c37c2;
    }
    else {
      if (uVar25 != 1) {
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      if (*(int *)pauVar19[uVar18 * -5 + -5] == iVar23) goto LAB_1407c37c2;
    }
  }
LAB_1407c396b:
  bVar31 = 0;
LAB_1407c396d:
  local_f1 = 1;
  country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume
            ();
  local_198 = CONCAT44(fStack_3ac,fStack_3b0);
  lStack_190 = local_198 + local_3a8 * 0x24;
  local_218 = 0;
  uStack_217 = 0;
  uStack_215 = 0;
  uStack_211 = uStack_211 & 0xffffff00;
  local_1d8 = 0;
  local_108 = local_198;
  alloc::vec::spec_extend::impl$1::
  spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array_<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env_1>,alloc::alloc::Global>
            ();
  if (CONCAT44(fStack_3b4,local_3b8) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (bVar31 == 0) {
    uStack_390 = CONCAT44(uStack_21c,uStack_220);
    local_3b8 = local_248;
    fStack_3b4 = fStack_244;
    fStack_3b0 = fStack_240;
    fStack_3ac = fStack_23c;
    uStack_3a0 = CONCAT44(fStack_22c + -0.2,fStack_230);
    local_398 = CONCAT44(fStack_224 + -0.2,0x3ecccccd);
    local_3a8._4_4_ = (float)((ulonglong)local_238 >> 0x20);
    local_3a8._0_4_ = (undefined4)local_238;
    local_3a8 = CONCAT44((local_3a8._4_4_ - fVar81) + -0.2,(undefined4)local_3a8);
    local_f1 = 1;
    country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
    box_trigger_volume();
    local_198 = CONCAT17(uStack_151,uStack_158);
    lStack_190 = local_198 + local_150 * 0x24;
    local_218 = 0;
    uStack_217 = 0;
    uStack_215 = 0;
    uStack_211 = uStack_211 & 0xffffff00;
    local_1d8 = 0;
    local_108 = local_198;
    alloc::vec::spec_extend::impl$1::
    spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array_<glam::f32::vec3::Vec3,3>,system_decorator::decorator_visual::chimney_wall::generate_wall_chimneys::closure_env_1>,alloc::alloc::Global>
              ();
    if (CONCAT17(uStack_159,local_160) != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  local_f1 = 1;
  country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::transform
            ();
  fVar42 = fStack_44c * fStack_44c;
  fVar80 = fStack_450 * fStack_450 + fStack_454 * fStack_454 + local_458 * local_458;
  fVar39 = fStack_450 * 0.33333334 + fStack_454 * 0.0 + local_458 * 0.0;
  fVar39 = fVar39 + fVar39;
  fVar90 = fStack_44c + fStack_44c;
  local_448 = CONCAT44(fVar91 + local_448._4_4_ +
                       fVar90 * (fStack_450 * 0.0 - local_458 * 0.33333334) +
                       fVar39 * fStack_454 + (fVar42 - fVar80) * 0.0,
                       (float)local_448 +
                       fVar90 * (fStack_454 * 0.33333334 - fStack_450 * 0.0) +
                       fVar39 * local_458 + (fVar42 - fVar80) * 0.0);
  fStack_440 = fVar90 * (local_458 * 0.0 - fStack_454 * 0.0) +
               fVar39 * fStack_450 + (fVar42 - fVar80) * 0.33333334 + fStack_440;
  uStack_20d = 0;
  uStack_215 = 0;
  uStack_211 = 0;
  uVar57 = *(undefined1 *)(local_290 + 1);
  uStack_280 = 0;
  local_288 = local_218;
  uStack_287 = (undefined1)uStack_217;
  uStack_286 = (uint5)(byte)((ushort)uStack_217 >> 8);
  uStack_281 = 0;
  lVar17 = local_120[0x18];
  if (lVar17 == local_120[0x16]) {
    local_f1 = 1;
    alloc::raw_vec::RawVec::
    grow_one<system_decorator::utils::visual_entity_builder::WindowVisualEntityBuilder,alloc::alloc::Global>
              ();
  }
  fVar41 = fStack_230;
  fVar80 = fStack_23c;
  fVar42 = fStack_240;
  fVar39 = fStack_244;
  fVar90 = local_248;
  lVar22 = local_120[0x17];
  lVar27 = lVar17 * 0x160;
  *(undefined8 *)(lVar22 + lVar27) = 0;
  *(undefined8 *)(lVar22 + 0x10 + lVar27) = 0;
  *(undefined8 *)(lVar22 + 0x20 + lVar27) = 0;
  *(undefined4 *)(lVar22 + 0x30 + lVar27) = 0;
  *(ulonglong *)(lVar22 + 0x34 + lVar27) = CONCAT17(uStack_301,local_308);
  *(undefined4 *)(lVar22 + 0x3c + lVar27) = local_300;
  *(undefined4 *)(lVar22 + 0x40 + lVar27) = 0;
  puVar3 = (undefined8 *)(lVar22 + 0x44 + lVar27);
  *puVar3 = CONCAT17((undefined1)uStack_211,CONCAT43(uStack_215,CONCAT21(uStack_217,local_218)));
  puVar3[1] = CONCAT17(uStack_209,CONCAT43(uStack_20d,uStack_211._1_3_));
  puVar3 = (undefined8 *)(lVar22 + 0x54 + lVar27);
  *puVar3 = local_208;
  puVar3[1] = auStack_200._0_8_;
  puVar3 = (undefined8 *)(lVar22 + 100 + lVar27);
  *puVar3 = stack0xfffffffffffffe08;
  puVar3[1] = CONCAT44(uStack_1ec,uStack_1f0);
  puVar3 = (undefined8 *)(lVar22 + 0x70 + lVar27);
  *puVar3 = CONCAT44(uStack_1e8,uStack_1ec);
  puVar3[1] = CONCAT26(uStack_1de,CONCAT24(uStack_1e0,uStack_1e4));
  *(undefined4 *)(lVar22 + 0x80 + lVar27) = 0;
  *(undefined8 *)(lVar22 + 0x84 + lVar27) = local_2b8;
  *(undefined4 *)(lVar22 + 0x8c + lVar27) = local_2b0;
  *(undefined4 *)(lVar22 + 0x90 + lVar27) = 0;
  *(undefined8 *)(lVar22 + 0x94 + lVar27) = local_3d0;
  *(undefined4 *)(lVar22 + 0x9c + lVar27) = local_3c8;
  *(undefined8 *)(lVar22 + 0xa0 + lVar27) = 1;
  *(undefined8 *)(lVar22 + 0xa8 + lVar27) = 0;
  *(undefined **)(lVar22 + 0xb0 + lVar27) =
       &anon_b17bdb6b936b48c513e8fa54c354b9a3_2_llvm_3328826292648170336;
  *(undefined8 *)(lVar22 + 0xb8 + lVar27) = 10;
  puVar3 = (undefined8 *)(lVar22 + 0xe0 + lVar27);
  *puVar3 = 0x3f8000003f800000;
  puVar3[1] = uStack_430;
  puVar3 = (undefined8 *)(lVar22 + 0xd0 + lVar27);
  *puVar3 = local_448;
  puVar3[1] = CONCAT44(0x3f800000,fStack_440);
  puVar3 = (undefined8 *)(lVar22 + 0xc0 + lVar27);
  *puVar3 = CONCAT44(fStack_454,local_458);
  puVar3[1] = CONCAT44(fStack_44c,fStack_450);
  *(undefined4 *)(lVar22 + 0xf0 + lVar27) = 0;
  *(undefined4 *)(lVar22 + 0xf8 + lVar27) = 0;
  *(longlong *)(lVar22 + 0x10c + lVar27) = local_150;
  puVar3 = (undefined8 *)(lVar22 + 0xfc + lVar27);
  *puVar3 = CONCAT17(uStack_159,local_160);
  puVar3[1] = CONCAT17(uStack_151,uStack_158);
  *(undefined1 *)(lVar22 + 0x114 + lVar27) = 2;
  *(undefined1 *)(lVar22 + 0x117 + lVar27) = uStack_3c2;
  *(undefined2 *)(lVar22 + 0x115 + lVar27) = local_3c4;
  *(undefined4 *)(lVar22 + 0x118 + lVar27) = 0;
  plVar15 = (longlong *)(lVar22 + 300 + lVar27);
  *plVar15 = local_3a8;
  plVar15[1] = uStack_3a0;
  puVar3 = (undefined8 *)(lVar22 + 0x11c + lVar27);
  *puVar3 = CONCAT44(fStack_3b4,local_3b8);
  puVar3[1] = CONCAT44(fStack_3ac,fStack_3b0);
  *(undefined1 *)(lVar22 + 0x13c + lVar27) = 2;
  *(ulonglong *)(lVar22 + 0x144 + lVar27) = CONCAT71(uStack_280,uStack_281);
  *(ulonglong *)(lVar22 + 0x13d + lVar27) =
       CONCAT17(uStack_281,CONCAT52(uStack_286,CONCAT11(uStack_287,local_288)));
  *(undefined1 *)(lVar22 + 0x14c + lVar27) = 0;
  *(undefined1 *)(lVar22 + 0x14d + lVar27) = uVar57;
  *(undefined1 *)(lVar22 + 0x14e + lVar27) = local_f3;
  *(undefined2 *)(lVar22 + 0x14f + lVar27) = 0x900;
  *(ulonglong *)(lVar22 + 0x158 + lVar27) = CONCAT71(uStack_310,uStack_311);
  *(ulonglong *)(lVar22 + 0x151 + lVar27) = CONCAT17(uStack_311,local_318);
  local_120[0x18] = lVar17 + 1;
  uVar13 = (uint)(float)local_238;
  uVar47 = (undefined1)local_238;
  uVar49 = (undefined1)((ulonglong)local_238 >> 8);
  uVar51 = (undefined1)((ulonglong)local_238 >> 0x10);
  uVar53 = (undefined1)((ulonglong)local_238 >> 0x18);
  uVar55 = 0;
  uVar58 = 0;
  uVar60 = 0;
  uVar62 = 0;
  uVar64 = 0;
  uVar66 = 0;
  uVar68 = 0;
  uVar70 = 0;
  uVar72 = 0;
  uVar74 = 0;
  uVar76 = 0;
  uVar78 = 0;
  fVar37 = local_238._4_4_;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_11 == (uint *)0x0) {
    local_f1 = 1;
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  fVar43 = fVar37 - fVar81;
  fVar44 = fVar42 * fVar42 + fVar39 * fVar39 + fVar90 * fVar90;
  fVar45 = fVar42 * 0.0 + fVar39 * 0.0 + fVar90 * 1.0;
  fVar45 = fVar45 + fVar45;
  *extraout_RAX_11 = (uint)(bVar31 ^ 1);
  extraout_RAX_11[1] = (uint)fVar83;
  extraout_RAX_11[2] = uVar13;
  extraout_RAX_11[3] = (uint)fVar41;
  extraout_RAX_11[4] = (uint)fVar43;
  extraout_RAX_11[5] = (uint)(fVar81 + fVar37);
  *(ulonglong *)(extraout_RAX_11 + 6) =
       CONCAT44(fVar45 * fVar42 + (fVar80 * fVar80 - fVar44) * 0.0 +
                (fVar90 * 0.0 - fVar39 * 1.0) * (fVar80 + fVar80),
                fVar45 * fVar90 + (fVar80 * fVar80 - fVar44) * 1.0 +
                (fVar39 * 0.0 - fVar42 * 0.0) * (fVar80 + fVar80));
  extraout_RAX_11[8] = (uint)local_250;
  extraout_RAX_11[9] = (uint)(fVar79 + 3.0 + fVar43);
  if (local_120[0x23] != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_120[0x23] = (longlong)extraout_RAX_11;
LAB_1407c4017:
  uVar25 = local_298;
  if (2 < local_298) {
    uVar18 = local_298 / 3;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_12 == 0) {
      local_f1 = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (uVar25 - 3 < 3) {
      uVar25 = 0;
    }
    else {
      lVar17 = 0;
      uVar25 = 0;
      do {
        iVar23 = (int)lVar17;
        *(int *)(extraout_RAX_12 + lVar17 * 4) = iVar23;
        *(int *)(extraout_RAX_12 + 4 + lVar17 * 4) = iVar23 + 1;
        *(int *)(extraout_RAX_12 + 8 + lVar17 * 4) = iVar23 + 2;
        uVar25 = uVar25 + 2;
        *(int *)(extraout_RAX_12 + 0xc + lVar17 * 4) = iVar23 + 3;
        *(int *)(extraout_RAX_12 + 0x10 + lVar17 * 4) = iVar23 + 4;
        *(int *)(extraout_RAX_12 + 0x14 + lVar17 * 4) = iVar23 + 5;
        lVar17 = lVar17 + 6;
      } while ((uVar18 & 0x7fffffffffffffe) != uVar25);
    }
    if ((uVar18 & 1) != 0) {
      iVar23 = (int)uVar25;
      *(int *)(extraout_RAX_12 + uVar25 * 0xc) = iVar23 * 3;
      *(int *)(extraout_RAX_12 + 4 + uVar25 * 0xc) = iVar23 * 3 + 1;
      *(int *)(extraout_RAX_12 + 8 + uVar25 * 0xc) = iVar23 * 3 + 2;
    }
  }
  local_278 = local_298;
  local_288 = (byte)local_2a8;
  uStack_287 = (undefined1)((ulonglong)local_2a8 >> 8);
  uStack_286 = (uint5)((ulonglong)local_2a8 >> 0x10);
  uStack_281 = (undefined1)((ulonglong)local_2a8 >> 0x38);
  uStack_280 = (undefined7)uStack_2a0;
  uStack_279 = (undefined1)((ulonglong)uStack_2a0 >> 0x38);
  local_160 = 0;
  uStack_159 = 0x80;
  local_218 = 0;
  uStack_217 = 0;
  uStack_215 = 0;
  uStack_211 = CONCAT31(uStack_211._1_3_,0x80);
  local_f1 = 0;
  country_core::systems::collision::world::RaycastMeshData::new_indexed();
  local_218 = 1;
  uStack_217 = 0;
  uStack_215 = 0;
  uStack_211 = 0x100;
  uStack_20d = 0;
  uStack_209 = 0;
  local_198 = local_348;
  lStack_190 = lStack_340;
  local_1a8 = local_358;
  uStack_1a0 = uStack_350;
  local_1b8 = local_368;
  uStack_1b0 = uStack_360;
  local_1c8 = local_378;
  uStack_1c0 = uStack_370;
  local_208 = (code *)CONCAT44(fStack_3b4,local_3b8);
  auStack_200._4_4_ = fStack_3ac;
  auStack_200._0_4_ = fStack_3b0;
  local_1d8 = CONCAT44(fStack_384,local_388);
  uStack_1d0 = CONCAT44(uStack_37c,fStack_380);
  uStack_1e8 = (undefined4)local_398;
  uStack_1e4 = (undefined4)((ulonglong)local_398 >> 0x20);
  uStack_1e0 = (ushort)uStack_390;
  uStack_1de = (undefined2)((ulonglong)uStack_390 >> 0x10);
  uStack_1dc = (undefined4)((ulonglong)uStack_390 >> 0x20);
  uStack_1f0 = (undefined4)uStack_3a0;
  uStack_1ec = (undefined4)((ulonglong)uStack_3a0 >> 0x20);
  stack0xfffffffffffffe08 = (longlong *)local_3a8;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_218,0x90);
  LOCK();
  lVar17 = *_Dst;
  *_Dst = *_Dst + 1;
  UNLOCK();
  if (*_Dst == 0 || SCARRY8(lVar17,1) != *_Dst < 0) {
                    // WARNING: Does not return
    pcVar10 = (code *)invalidInstructionException();
    (*pcVar10)();
  }
  uStack_20d = uStack_20d & 0xffffff00;
  local_218 = 0;
  uStack_217 = 0;
  uStack_215 = 0;
  uStack_211 = 0;
  local_f1 = 0;
  blueprint::DecoratorBlueprint::set_colliders();
  iVar23 = -(uint)(CONCAT13(uVar53,CONCAT12(uVar51,CONCAT11(uVar49,uVar47))) ==
                  CONCAT13(uVar53,CONCAT12(uVar51,CONCAT11(uVar49,uVar47))));
  cVar46 = (char)iVar23;
  cVar48 = (char)((uint)iVar23 >> 8);
  cVar50 = (char)((uint)iVar23 >> 0x10);
  cVar52 = (char)((uint)iVar23 >> 0x18);
  iVar23 = -(uint)(CONCAT13(uVar62,CONCAT12(uVar60,CONCAT11(uVar58,uVar55))) ==
                  CONCAT13(uVar62,CONCAT12(uVar60,CONCAT11(uVar58,uVar55))));
  cVar54 = (char)iVar23;
  cVar56 = (char)((uint)iVar23 >> 8);
  cVar59 = (char)((uint)iVar23 >> 0x10);
  cVar61 = (char)((uint)iVar23 >> 0x18);
  iVar23 = -(uint)(CONCAT13(uVar70,CONCAT12(uVar68,CONCAT11(uVar66,uVar64))) ==
                  CONCAT13(uVar70,CONCAT12(uVar68,CONCAT11(uVar66,uVar64))));
  cVar63 = (char)iVar23;
  cVar65 = (char)((uint)iVar23 >> 8);
  cVar67 = (char)((uint)iVar23 >> 0x10);
  cVar69 = (char)((uint)iVar23 >> 0x18);
  iVar23 = -(uint)(CONCAT13(uVar78,CONCAT12(uVar76,CONCAT11(uVar74,uVar72))) ==
                  CONCAT13(uVar78,CONCAT12(uVar76,CONCAT11(uVar74,uVar72))));
  cVar71 = (char)iVar23;
  cVar73 = (char)((uint)iVar23 >> 8);
  cVar75 = (char)((uint)iVar23 >> 0x10);
  cVar77 = (char)((uint)iVar23 >> 0x18);
  uVar14 = local_130;
  goto LAB_1407c2866;
}

