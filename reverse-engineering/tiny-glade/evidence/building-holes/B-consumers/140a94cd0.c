
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::wall::wall_state::wall_roof_hashmap::wall_roof_hashmap(void)

{
  byte *pbVar1;
  uint *puVar2;
  undefined1 auVar3 [16];
  byte bVar4;
  byte bVar5;
  undefined4 uVar6;
  int *piVar7;
  undefined4 *puVar8;
  undefined8 uVar9;
  longlong lVar10;
  longlong lVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  int iVar17;
  longlong *plVar18;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar19;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar20;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  byte *extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong lVar21;
  undefined8 *puVar22;
  ulonglong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  ulonglong extraout_RAX_09;
  byte *extraout_RAX_10;
  longlong extraout_RAX_11;
  ulonglong extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  uint uVar23;
  longlong *in_RCX;
  ulonglong *puVar24;
  longlong *plVar25;
  undefined8 *in_RDX;
  ulonglong uVar26;
  undefined8 *puVar27;
  longlong lVar28;
  longlong *plVar29;
  uint *puVar30;
  undefined1 (*pauVar31) [16];
  undefined8 *in_R8;
  ulonglong uVar32;
  ulonglong uVar33;
  longlong *in_R9;
  longlong lVar34;
  longlong lVar35;
  ulonglong uVar36;
  ushort uVar37;
  longlong lVar38;
  longlong lVar39;
  undefined1 (*pauVar40) [16];
  undefined8 *puVar41;
  byte bVar42;
  char cVar43;
  char cVar45;
  char cVar46;
  char cVar47;
  undefined1 auVar44 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  undefined1 unaff_XMM8_Bc;
  undefined1 unaff_XMM8_Bd;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  undefined1 unaff_XMM8_Bg;
  undefined1 unaff_XMM8_Bh;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  undefined1 unaff_XMM8_Bk;
  undefined1 unaff_XMM8_Bl;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  undefined1 unaff_XMM8_Bo;
  undefined1 unaff_XMM8_Bp;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *local_168;
  code *pcStack_160;
  longlong local_158;
  longlong lStack_150;
  longlong local_148;
  longlong lStack_140;
  longlong local_138;
  longlong *local_130;
  longlong local_128;
  longlong local_120;
  undefined8 *local_118;
  undefined4 local_10c;
  longlong local_108;
  longlong local_100;
  undefined **local_f8;
  ulonglong uStack_f0;
  longlong **local_e8;
  longlong lStack_e0;
  longlong local_d8;
  undefined8 *puStack_d0;
  ulonglong local_c0;
  longlong *local_b8;
  longlong *local_b0;
  undefined8 local_a8;
  longlong local_a0;
  char local_93;
  undefined1 local_92;
  undefined1 local_91;
  undefined8 local_90;
  
  local_90 = 0xfffffffffffffffe;
  puVar41 = in_RDX;
  puffin::are_scopes_on();
  local_118 = in_RDX;
  if (extraout_AL != '\0') {
    puVar41 = (undefined8 *)&DAT_142ada041;
    uVar26 = 0x4d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar26 != 0) {
        if (uVar26 < 0x4d) {
          if ((char)(&DAT_142ada041)[uVar26] < -0x40) {
LAB_140a94d74:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar26 != 0x4d) goto LAB_140a94d74;
      }
      puVar41 = (undefined8 *)&DAT_142ada041;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar26 != 0xfffffffffffffffe) {
          if (uVar26 + 2 < 0x4d) {
            if ((char)(&UNK_142ada043)[uVar26] < -0x40) {
LAB_140a94dd2:
                    /* WARNING: Subroutine does not return */
              core::str::slice_error_fail();
            }
          }
          else if (uVar26 + 2 != 0x4d) goto LAB_140a94dd2;
        }
        puVar41 = (undefined8 *)(&UNK_142ada043 + uVar26);
      }
    }
    puVar20 = &DAT_142ada494;
    do {
      bVar42 = puVar20[-1];
      uVar23 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar4 = puVar20[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar20[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar20 + -4;
            puVar20 = puVar20 + -4;
            uVar23 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar20 = puVar20 + -3;
            uVar23 = bVar5 & 0xf;
          }
          uVar23 = bVar4 & 0x3f | uVar23 << 6;
        }
        else {
          puVar20 = puVar20 + -2;
          uVar23 = bVar4 & 0x1f;
        }
        uVar23 = (uint)(bVar42 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_140a94e99;
LAB_140a94eae:
        if (puVar20 == &UNK_142ada44f) break;
        if (puVar20 + -0x142ada44f < (undefined *)0x44) {
          if (-0x41 < (char)puVar20[1]) break;
        }
        else if (puVar20 + -0x142ada44f == (undefined *)0x44) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar20 = puVar20 + -1;
      if (uVar23 == 0x5c) goto LAB_140a94eae;
LAB_140a94e99:
      if (uVar23 == 0x2f) goto LAB_140a94eae;
    } while (puVar20 != &DAT_142ada450);
    local_b8 = in_RCX;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar25 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar25 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0))
    goto LAB_140a95dec;
    if (*plVar25 != 0) goto LAB_140a95e39;
    *plVar25 = -1;
    local_b0 = plVar25;
    puffin::ThreadProfiler::begin_scope();
    *local_b0 = *local_b0 + 1;
    in_RCX = local_b8;
  }
  puVar22 = local_118;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  local_93 = extraout_AL;
  if ((*extraout_RAX_02 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_02[0] = 1;
    extraout_RAX_02[1] = 0;
    extraout_RAX_02[2] = 0;
    extraout_RAX_02[3] = 0;
    extraout_RAX_02[4] = 0;
    extraout_RAX_02[5] = 0;
    extraout_RAX_02[6] = 0;
    extraout_RAX_02[7] = 0;
    *(longlong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
    *(undefined8 **)(extraout_RAX_02 + 0x10) = puVar41;
    lVar28 = extraout_RAX_03;
  }
  else {
    lVar28 = *(longlong *)(extraout_RAX_02 + 8);
    puVar41 = *(undefined8 **)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = lVar28 + 1;
  lVar11 = _UNK_142ad9ef8;
  lVar10 = _DAT_142ad9ef0;
  uVar26 = _UNK_142ad9ee8;
  puVar20 = PTR_DAT_142ad9ee0;
  puVar27 = puVar41;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_04 & 1) == 0) {
    local_92 = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_04[0] = 1;
    extraout_RAX_04[1] = 0;
    extraout_RAX_04[2] = 0;
    extraout_RAX_04[3] = 0;
    extraout_RAX_04[4] = 0;
    extraout_RAX_04[5] = 0;
    extraout_RAX_04[6] = 0;
    extraout_RAX_04[7] = 0;
    *(longlong *)(extraout_RAX_04 + 8) = extraout_RAX_05;
    *(undefined8 **)(extraout_RAX_04 + 0x10) = puVar27;
    lVar35 = extraout_RAX_05;
  }
  else {
    lVar35 = *(longlong *)(extraout_RAX_04 + 8);
    puVar27 = *(undefined8 **)(extraout_RAX_04 + 0x10);
  }
  *(longlong *)(extraout_RAX_04 + 8) = lVar35 + 1;
  local_108 = *in_R9;
  lVar39 = in_R9[1];
  puVar30 = *(uint **)(local_108 + 0xf8);
  puVar2 = puVar30 + *(longlong *)(local_108 + 0x100);
  local_b8 = (longlong *)CONCAT71(local_b8._1_7_,*(undefined1 *)(local_108 + 0x108));
  piVar7 = (int *)*in_RCX;
  local_10c = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  local_100 = *in_stack_00000038;
  puVar8 = (undefined4 *)in_stack_00000038[2];
  iVar14 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                            CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                  CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  local_b0 = (longlong *)*puVar22;
  local_130 = local_b0 + 4;
  local_120 = 8;
  uVar36 = 0;
  uVar33 = 0;
  local_128 = 8;
  local_a0 = 0;
  local_a8 = (undefined4 *)((ulonglong)local_a8._4_4_ << 0x20);
LAB_140a95116:
  do {
    if ((char)local_b8 == '\0') {
      if (uVar36 == uVar33) {
        do {
          if (puVar30 == puVar2) goto LAB_140a9557b;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar33 = *(ulonglong *)(*(longlong *)(lVar39 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0);
        } while (uVar33 == 0);
        lVar21 = *(longlong *)(lVar39 + 0x100) + (ulonglong)uVar23 * 0xa0;
        if (*(ulonglong *)(local_108 + 0x110) <
            *(ulonglong *)
             (*(longlong *)(lVar39 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar21 + 0x94) * 0x48)) {
          lVar34 = *(longlong *)(lVar39 + 0x1a8) + (ulonglong)*(uint *)(lVar21 + 0x94) * 0x48;
          uVar36 = *(ulonglong *)
                    (*(longlong *)(lVar34 + 0x38) + *(ulonglong *)(local_108 + 0x110) * 8);
          if (uVar36 == 0) goto LAB_140a952d3;
          local_a0 = *(longlong *)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar36 * 0x30);
          uVar36 = *(ulonglong *)(local_108 + 0x118);
          if (uVar36 < *(ulonglong *)(lVar21 + 0x88)) goto LAB_140a952f5;
LAB_140a9530d:
          local_a8 = (undefined4 *)((ulonglong)local_a8._4_4_ << 0x20);
        }
        else {
LAB_140a952d3:
          local_a0 = 0;
          uVar36 = *(ulonglong *)(local_108 + 0x118);
          if (*(ulonglong *)(lVar21 + 0x88) <= uVar36) goto LAB_140a9530d;
LAB_140a952f5:
          if (*(longlong *)(*(longlong *)(lVar21 + 0x80) + uVar36 * 8) == 0) goto LAB_140a9530d;
          local_a8 = (undefined4 *)CONCAT44(local_a8._4_4_,(int)CONCAT71((int7)(uVar36 >> 8),1));
        }
        local_120 = *(longlong *)(lVar21 + 0x50);
        uVar36 = 0;
      }
      puVar22 = (undefined8 *)(local_120 + uVar36 * 0x10);
      uVar32 = (ulonglong)*(uint *)(local_120 + 8 + uVar36 * 0x10);
    }
    else {
      if (uVar36 == uVar33) {
        do {
          if (puVar30 == puVar2) goto LAB_140a9557b;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar33 = *(ulonglong *)(*(longlong *)(lVar39 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48);
        } while (uVar33 == 0);
        lVar21 = *(longlong *)(lVar39 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        uVar36 = *(ulonglong *)(lVar21 + 0x40);
        if ((*(ulonglong *)(local_108 + 0x110) < uVar36) &&
           (uVar32 = *(ulonglong *)
                      (*(longlong *)(lVar21 + 0x38) + *(ulonglong *)(local_108 + 0x110) * 8),
           uVar32 != 0)) {
          local_a0 = *(longlong *)(*(longlong *)(lVar21 + 0x18) + 0x10 + ~uVar32 * 0x30);
          uVar32 = *(ulonglong *)(local_108 + 0x118);
          if (uVar36 <= uVar32) goto LAB_140a951b6;
LAB_140a952a3:
          local_a8 = (undefined4 *)
                     CONCAT44(local_a8._4_4_,
                              (int)CONCAT71((int7)((ulonglong)*(longlong *)(lVar21 + 0x38) >> 8),
                                            *(longlong *)(*(longlong *)(lVar21 + 0x38) + uVar32 * 8)
                                            != 0));
        }
        else {
          local_a0 = 0;
          uVar32 = *(ulonglong *)(local_108 + 0x118);
          if (uVar32 < uVar36) goto LAB_140a952a3;
LAB_140a951b6:
          local_a8 = (undefined4 *)((ulonglong)local_a8._4_4_ << 0x20);
        }
        local_128 = *(longlong *)(lVar21 + 8);
        uVar36 = 0;
      }
      puVar22 = (undefined8 *)(local_128 + uVar36 * 8);
      uVar32 = uVar36 & 0xffffffff;
    }
    plVar29 = (longlong *)(uVar32 * 0x58 + local_a0);
    local_c0 = *puVar22;
    uVar36 = uVar36 + 1;
    plVar25 = (longlong *)*plVar29;
    local_f8 = (undefined **)plVar25;
    if (local_b0[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar21 = *local_b0;
      bVar42 = (byte)(extraout_RAX_06 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar34 = 0;
      uVar32 = extraout_RAX_06;
      while( true ) {
        uVar32 = local_b0[1] & uVar32;
        auVar3 = *(undefined1 (*) [16])(lVar21 + uVar32);
        cVar43 = auVar44[0];
        auVar52[0] = -(auVar3[0] == cVar43);
        cVar45 = auVar44[1];
        auVar52[1] = -(auVar3[1] == cVar45);
        cVar46 = auVar44[2];
        auVar52[2] = -(auVar3[2] == cVar46);
        cVar47 = auVar44[3];
        auVar52[3] = -(auVar3[3] == cVar47);
        auVar52[4] = -(auVar3[4] == cVar43);
        auVar52[5] = -(auVar3[5] == cVar45);
        auVar52[6] = -(auVar3[6] == cVar46);
        auVar52[7] = -(auVar3[7] == cVar47);
        auVar52[8] = -(auVar3[8] == cVar43);
        auVar52[9] = -(auVar3[9] == cVar45);
        auVar52[10] = -(auVar3[10] == cVar46);
        auVar52[0xb] = -(auVar3[0xb] == cVar47);
        auVar52[0xc] = -(auVar3[0xc] == cVar43);
        auVar52[0xd] = -(auVar3[0xd] == cVar45);
        auVar52[0xe] = -(auVar3[0xe] == cVar46);
        auVar52[0xf] = -(auVar3[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar52 >> 7,0) & 1) | (ushort)(SUB161(auVar52 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar52 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar52 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar52 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar52 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar52 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar52 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar52 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar52 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar52 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar52 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar52 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar52 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar52 >> 0x77,0) & 1) << 0xe | (ushort)(auVar52[0xf] >> 7) << 0xf
        ;
        uVar23 = (uint)uVar37;
        while (uVar37 != 0) {
          uVar19 = 0;
          for (uVar12 = uVar23; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
            uVar19 = uVar19 + 1;
          }
          lVar38 = (uVar19 + uVar32 & local_b0[1]) * -0x2a8;
          if (*(longlong **)(lVar21 + -0x2a8 + lVar38) == plVar25) {
            if ((*(ulonglong *)(lVar21 + -0x290 + lVar38) < 2) ||
               (*(float *)(lVar21 + -0x270 + lVar38) <= 0.0)) {
              local_91 = 1;
                    /* WARNING: Subroutine does not return */
              core::panicking::panic();
            }
            if (1 < *(uint *)(lVar21 + lVar38 + -0x48)) goto LAB_140a95490;
            if (((ulonglong)local_a8 & 1) == 0) {
              local_91 = 1;
              hashbrown::map::HashMap::
              insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              if (extraout_RAX_07 >> 0x20 != 0) {
                local_f8 = &PTR_s_Only_one_roof_should_be_associat_142ada5a8;
                uStack_f0 = 1;
                local_e8 = (longlong **)0x8;
                lStack_e0 = 0;
                local_d8 = 0;
                local_91 = 1;
                    /* WARNING: Subroutine does not return */
                core::panicking::panic_fmt();
              }
            }
            else {
              local_91 = 1;
              hashbrown::map::HashMap::
              insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
            }
            goto LAB_140a95116;
          }
          uVar37 = (ushort)(uVar23 - 1) & (ushort)uVar23;
          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar37);
        }
        auVar48[0] = -(auVar3[0] == (char)iVar14);
        auVar48[1] = -(auVar3[1] == (char)((uint)iVar14 >> 8));
        auVar48[2] = -(auVar3[2] == (char)((uint)iVar14 >> 0x10));
        auVar48[3] = -(auVar3[3] == (char)((uint)iVar14 >> 0x18));
        auVar48[4] = -(auVar3[4] == (char)iVar15);
        auVar48[5] = -(auVar3[5] == (char)((uint)iVar15 >> 8));
        auVar48[6] = -(auVar3[6] == (char)((uint)iVar15 >> 0x10));
        auVar48[7] = -(auVar3[7] == (char)((uint)iVar15 >> 0x18));
        auVar48[8] = -(auVar3[8] == (char)iVar16);
        auVar48[9] = -(auVar3[9] == (char)((uint)iVar16 >> 8));
        auVar48[10] = -(auVar3[10] == (char)((uint)iVar16 >> 0x10));
        auVar48[0xb] = -(auVar3[0xb] == (char)((uint)iVar16 >> 0x18));
        auVar48[0xc] = -(auVar3[0xc] == (char)iVar17);
        auVar48[0xd] = -(auVar3[0xd] == (char)((uint)iVar17 >> 8));
        auVar48[0xe] = -(auVar3[0xe] == (char)((uint)iVar17 >> 0x10));
        auVar48[0xf] = -(auVar3[0xf] == (char)((uint)iVar17 >> 0x18));
        if ((((((((((((((((SUB161(auVar48 >> 7,0) & 1) != 0 || (SUB161(auVar48 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar48 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar48 >> 0x1f,0) & 1) != 0) || (SUB161(auVar48 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar48 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar48 >> 0x37,0) & 1) != 0) || (SUB161(auVar48 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar48 >> 0x47,0) & 1) != 0) || (SUB161(auVar48 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar48 >> 0x57,0) & 1) != 0) || (SUB161(auVar48 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar48 >> 0x67,0) & 1) != 0) || (SUB161(auVar48 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar48 >> 0x77,0) & 1) != 0) || auVar48[0xf] < '\0') break;
        uVar32 = uVar32 + lVar34 + 0x10;
        lVar34 = lVar34 + 0x10;
      }
    }
LAB_140a95490:
    if (((ulonglong)local_a8 & 1) == 0) {
      local_168 = plVar29;
      if (*piVar7 == 0xe) {
        pcStack_160 = resources::walls::impl$7::fmt;
        local_f8 = &PTR_s_Tried_deleting_roof_during_Repla_142ada538;
        uStack_f0 = 2;
        local_d8 = 0;
        local_e8 = &local_168;
        lStack_e0 = 1;
        local_91 = 1;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      if (*piVar7 == 0xd) {
        pcStack_160 = resources::walls::impl$7::fmt;
        local_f8 = &PTR_s_Tried_deleting_roof_during_Repla_142ada4f0;
        uStack_f0 = 2;
        local_d8 = 0;
        local_e8 = &local_168;
        lStack_e0 = 1;
        local_91 = 1;
                    /* WARNING: Subroutine does not return */
        core::panicking::panic_fmt();
      }
      lVar21 = *plVar29;
      *puVar8 = local_10c;
      lVar34 = *(longlong *)(local_100 + 0x30);
      uVar9 = *(undefined8 *)(local_100 + 0x40);
      if (lVar34 == *(longlong *)(local_100 + 0x20)) {
        local_91 = 1;
        local_c0 = uVar36;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
        uVar36 = local_c0;
      }
      lVar38 = *(longlong *)(local_100 + 0x28);
      *(undefined8 *)(lVar38 + lVar34 * 0x10) = uVar9;
      *(longlong *)(lVar38 + 8 + lVar34 * 0x10) = lVar21;
      *(longlong *)(local_100 + 0x30) = lVar34 + 1;
      *(longlong *)(local_100 + 0x40) = *(longlong *)(local_100 + 0x40) + 1;
    }
  } while( true );
LAB_140a9557b:
  pauVar40 = (undefined1 (*) [16])*local_b0;
  local_b8 = (longlong *)local_b0[3];
  auVar44 = *pauVar40;
  uVar23 = ~(uint)(ushort)((ushort)(SUB161(auVar44 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar44[0xf] >> 7) << 0xf);
  pauVar31 = pauVar40 + 1;
  local_a0 = CONCAT44(local_a0._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x1c));
  lVar39 = *in_stack_00000030;
  local_a8 = (undefined4 *)in_stack_00000030[2];
  local_c0 = lVar39 + 0x20;
LAB_140a95601:
  while (plVar25 = local_b0, local_b8 != (longlong *)0x0) {
    if ((short)uVar23 == 0) {
      do {
        auVar44 = *pauVar31;
        uVar37 = (ushort)(SUB161(auVar44 >> 7,0) & 1) | (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar44 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar44 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar44 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar44 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar44 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar44 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar44 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar44 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar44 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar44 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar44 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar44 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar44[0xf] >> 7) << 0xf;
        pauVar40 = pauVar40 + -0x2a8;
        pauVar31 = pauVar31 + 1;
      } while (uVar37 == 0xffff);
      uVar19 = ~(uint)uVar37;
      uVar23 = uVar19 & -(uint)uVar37 - 2;
    }
    else {
      uVar19 = uVar23;
      uVar23 = uVar23 - 1 & uVar23;
    }
    local_b8 = (longlong *)((longlong)local_b8 + -1);
    uVar12 = 0;
    for (; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
      uVar12 = uVar12 + 1;
    }
    lVar21 = *(longlong *)((longlong)pauVar40 + (ulonglong)uVar12 * -0x2a8 + -0x2a8);
    local_138 = lVar21;
    resources::walls::public_walls::PublicWalls::has_roof();
    if (lVar11 != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar42 = (byte)(extraout_RAX_08 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar34 = 0;
      uVar33 = extraout_RAX_08;
      while( true ) {
        uVar33 = uVar26 & uVar33;
        auVar3 = *(undefined1 (*) [16])(puVar20 + uVar33);
        cVar43 = auVar44[0];
        auVar53[0] = -(auVar3[0] == cVar43);
        cVar45 = auVar44[1];
        auVar53[1] = -(auVar3[1] == cVar45);
        cVar46 = auVar44[2];
        auVar53[2] = -(auVar3[2] == cVar46);
        cVar47 = auVar44[3];
        auVar53[3] = -(auVar3[3] == cVar47);
        auVar53[4] = -(auVar3[4] == cVar43);
        auVar53[5] = -(auVar3[5] == cVar45);
        auVar53[6] = -(auVar3[6] == cVar46);
        auVar53[7] = -(auVar3[7] == cVar47);
        auVar53[8] = -(auVar3[8] == cVar43);
        auVar53[9] = -(auVar3[9] == cVar45);
        auVar53[10] = -(auVar3[10] == cVar46);
        auVar53[0xb] = -(auVar3[0xb] == cVar47);
        auVar53[0xc] = -(auVar3[0xc] == cVar43);
        auVar53[0xd] = -(auVar3[0xd] == cVar45);
        auVar53[0xe] = -(auVar3[0xe] == cVar46);
        auVar53[0xf] = -(auVar3[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar53 >> 7,0) & 1) | (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe | (ushort)(auVar53[0xf] >> 7) << 0xf
        ;
        uVar19 = (uint)uVar37;
        while (uVar37 != 0) {
          uVar12 = 0;
          for (uVar13 = uVar19; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar12 = uVar12 + 1;
          }
          if (lVar21 == *(longlong *)(puVar20 + (uVar12 + uVar33 & uVar26) * -0x10 + -0x10))
          goto LAB_140a957e0;
          uVar37 = (ushort)(uVar19 - 1) & (ushort)uVar19;
          uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar37);
        }
        auVar49[0] = -(auVar3[0] == -1);
        auVar49[1] = -(auVar3[1] == -1);
        auVar49[2] = -(auVar3[2] == -1);
        auVar49[3] = -(auVar3[3] == -1);
        auVar49[4] = -(auVar3[4] == -1);
        auVar49[5] = -(auVar3[5] == -1);
        auVar49[6] = -(auVar3[6] == -1);
        auVar49[7] = -(auVar3[7] == -1);
        auVar49[8] = -(auVar3[8] == -1);
        auVar49[9] = -(auVar3[9] == -1);
        auVar49[10] = -(auVar3[10] == -1);
        auVar49[0xb] = -(auVar3[0xb] == -1);
        auVar49[0xc] = -(auVar3[0xc] == -1);
        auVar49[0xd] = -(auVar3[0xd] == -1);
        auVar49[0xe] = -(auVar3[0xe] == -1);
        auVar49[0xf] = -(auVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar49 >> 7,0) & 1) != 0 || (SUB161(auVar49 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar49 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar49 >> 0x1f,0) & 1) != 0) || (SUB161(auVar49 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar49 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar49 >> 0x37,0) & 1) != 0) || (SUB161(auVar49 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar49 >> 0x47,0) & 1) != 0) || (SUB161(auVar49 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar49 >> 0x57,0) & 1) != 0) || (SUB161(auVar49 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar49 >> 0x67,0) & 1) != 0) || (SUB161(auVar49 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar49 >> 0x77,0) & 1) != 0) || auVar49[0xf] < '\0') break;
        uVar33 = uVar33 + lVar34 + 0x10;
        lVar34 = lVar34 + 0x10;
      }
    }
    if (lVar11 != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar42 = (byte)(extraout_RAX_09 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar34 = 0;
      uVar33 = extraout_RAX_09;
      while( true ) {
        uVar33 = uVar26 & uVar33;
        auVar3 = *(undefined1 (*) [16])(puVar20 + uVar33);
        cVar43 = auVar44[0];
        auVar54[0] = -(auVar3[0] == cVar43);
        cVar45 = auVar44[1];
        auVar54[1] = -(auVar3[1] == cVar45);
        cVar46 = auVar44[2];
        auVar54[2] = -(auVar3[2] == cVar46);
        cVar47 = auVar44[3];
        auVar54[3] = -(auVar3[3] == cVar47);
        auVar54[4] = -(auVar3[4] == cVar43);
        auVar54[5] = -(auVar3[5] == cVar45);
        auVar54[6] = -(auVar3[6] == cVar46);
        auVar54[7] = -(auVar3[7] == cVar47);
        auVar54[8] = -(auVar3[8] == cVar43);
        auVar54[9] = -(auVar3[9] == cVar45);
        auVar54[10] = -(auVar3[10] == cVar46);
        auVar54[0xb] = -(auVar3[0xb] == cVar47);
        auVar54[0xc] = -(auVar3[0xc] == cVar43);
        auVar54[0xd] = -(auVar3[0xd] == cVar45);
        auVar54[0xe] = -(auVar3[0xe] == cVar46);
        auVar54[0xf] = -(auVar3[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar54 >> 7,0) & 1) | (ushort)(SUB161(auVar54 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar54 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar54 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar54 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar54 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar54 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar54 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar54 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar54 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar54 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar54 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar54 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar54 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar54 >> 0x77,0) & 1) << 0xe | (ushort)(auVar54[0xf] >> 7) << 0xf
        ;
        uVar19 = (uint)uVar37;
        while (uVar37 != 0) {
          uVar12 = 0;
          for (uVar13 = uVar19; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar12 = uVar12 + 1;
          }
          if (lVar21 == *(longlong *)(puVar20 + (uVar12 + uVar33 & uVar26) * -0x10 + -0x10))
          goto LAB_140a957e0;
          uVar37 = (ushort)(uVar19 - 1) & (ushort)uVar19;
          uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar37);
        }
        auVar50[0] = -(auVar3[0] == -1);
        auVar50[1] = -(auVar3[1] == -1);
        auVar50[2] = -(auVar3[2] == -1);
        auVar50[3] = -(auVar3[3] == -1);
        auVar50[4] = -(auVar3[4] == -1);
        auVar50[5] = -(auVar3[5] == -1);
        auVar50[6] = -(auVar3[6] == -1);
        auVar50[7] = -(auVar3[7] == -1);
        auVar50[8] = -(auVar3[8] == -1);
        auVar50[9] = -(auVar3[9] == -1);
        auVar50[10] = -(auVar3[10] == -1);
        auVar50[0xb] = -(auVar3[0xb] == -1);
        auVar50[0xc] = -(auVar3[0xc] == -1);
        auVar50[0xd] = -(auVar3[0xd] == -1);
        auVar50[0xe] = -(auVar3[0xe] == -1);
        auVar50[0xf] = -(auVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar50 >> 7,0) & 1) != 0 || (SUB161(auVar50 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar50 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar50 >> 0x1f,0) & 1) != 0) || (SUB161(auVar50 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar50 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar50 >> 0x37,0) & 1) != 0) || (SUB161(auVar50 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar50 >> 0x47,0) & 1) != 0) || (SUB161(auVar50 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar50 >> 0x57,0) & 1) != 0) || (SUB161(auVar50 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar50 >> 0x67,0) & 1) != 0) || (SUB161(auVar50 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar50 >> 0x77,0) & 1) != 0) || auVar50[0xf] < '\0') break;
        uVar33 = uVar33 + lVar34 + 0x10;
        lVar34 = lVar34 + 0x10;
      }
    }
    if (extraout_AL_00 != '\0') goto LAB_140a957fb;
  }
  local_e8 = (longlong **)lVar10;
  lStack_e0 = lVar11;
  local_f8 = (undefined **)puVar20;
  uStack_f0 = uVar26;
  uVar6 = *(undefined4 *)((longlong)local_118 + 0x1c);
  puVar8 = (undefined4 *)local_118[2];
  *puVar8 = uVar6;
  plVar29 = local_b0 + 6;
  local_d8 = lVar28;
  puStack_d0 = puVar41;
  if ((local_b0[7] != 0) && (local_b0[7] * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar18 = local_b0;
  plVar25[10] = local_d8;
  plVar25[0xb] = (longlong)puStack_d0;
  plVar25[8] = (longlong)local_e8;
  plVar25[9] = lStack_e0;
  *plVar29 = (longlong)local_f8;
  plVar25[7] = uStack_f0;
  local_e8 = (longlong **)lVar10;
  lStack_e0 = lVar11;
  local_f8 = (undefined **)puVar20;
  uStack_f0 = uVar26;
  *puVar8 = uVar6;
  plVar25 = local_b0 + 0xc;
  lVar28 = 0;
  local_d8 = lVar35;
  puStack_d0 = puVar27;
  if ((local_b0[0xd] != 0) && (lVar28 = local_b0[0xd] * 0x11 + 0x21, lVar28 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  auVar44._8_8_ = lStack_e0;
  auVar44._0_8_ = local_e8;
  auVar3._8_8_ = puStack_d0;
  auVar3._0_8_ = local_d8;
  *(undefined1 (*) [16])(plVar18 + 0x10) = auVar3;
  *(undefined1 (*) [16])(plVar18 + 0xe) = auVar44;
  *plVar25 = (longlong)local_f8;
  plVar18[0xd] = uStack_f0;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_10 & 1) == 0) {
    local_91 = 0;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_10[0] = 1;
    extraout_RAX_10[1] = 0;
    extraout_RAX_10[2] = 0;
    extraout_RAX_10[3] = 0;
    extraout_RAX_10[4] = 0;
    extraout_RAX_10[5] = 0;
    extraout_RAX_10[6] = 0;
    extraout_RAX_10[7] = 0;
    *(longlong *)(extraout_RAX_10 + 8) = extraout_RAX_11;
    *(longlong *)(extraout_RAX_10 + 0x10) = lVar28;
    local_148 = extraout_RAX_11;
  }
  else {
    local_148 = *(longlong *)(extraout_RAX_10 + 8);
    lVar28 = *(longlong *)(extraout_RAX_10 + 0x10);
  }
  *(longlong *)(extraout_RAX_10 + 8) = local_148 + 1;
  local_158 = lVar10;
  lStack_150 = lVar11;
  local_168 = (longlong *)puVar20;
  pcStack_160 = (code *)uVar26;
  local_100 = *in_stack_00000028;
  lVar10 = in_stack_00000028[1];
  puVar30 = *(uint **)(local_100 + 0xf8);
  puVar2 = puVar30 + *(longlong *)(local_100 + 0x100);
  local_b8 = (longlong *)CONCAT71(local_b8._1_7_,*(undefined1 *)(local_100 + 0x108));
  local_a8 = (undefined4 *)0x8;
  local_a0 = 0;
  local_c0 = 8;
  uVar26 = 0;
  uVar33 = 0;
  lStack_140 = lVar28;
LAB_140a95a09:
  do {
    if ((char)local_b8 == '\0') {
      if (uVar33 == uVar26) {
        do {
          if (puVar30 == puVar2) goto LAB_140a95cb6;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar26 = *(ulonglong *)(*(longlong *)(lVar10 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0);
        } while (uVar26 == 0);
        lVar28 = *(longlong *)(lVar10 + 0x100) + (ulonglong)uVar23 * 0xa0;
        uVar33 = (ulonglong)*(uint *)(lVar28 + 0x94);
        if (*(ulonglong *)(local_100 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x40 + uVar33 * 0x48)) {
          lVar11 = *(longlong *)(lVar10 + 0x1a8) + uVar33 * 0x48;
          uVar33 = *(ulonglong *)
                    (*(longlong *)(lVar11 + 0x38) + *(ulonglong *)(local_100 + 0x110) * 8);
          if (uVar33 == 0) goto LAB_140a95b58;
          local_a0 = *(longlong *)(*(longlong *)(lVar11 + 0x18) + 0x10 + ~uVar33 * 0x30);
        }
        else {
LAB_140a95b58:
          local_a0 = 0;
        }
        local_c0 = *(longlong *)(lVar28 + 0x50);
        uVar33 = 0;
      }
      puVar24 = (ulonglong *)(local_c0 + uVar33 * 0x10);
      uVar36 = (ulonglong)*(uint *)(local_c0 + 8 + uVar33 * 0x10);
    }
    else {
      if (uVar33 == uVar26) {
        do {
          if (puVar30 == puVar2) goto LAB_140a95cb6;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar26 = *(ulonglong *)(*(longlong *)(lVar10 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48);
        } while (uVar26 == 0);
        lVar28 = *(longlong *)(lVar10 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        if ((*(ulonglong *)(local_100 + 0x110) < *(ulonglong *)(lVar28 + 0x40)) &&
           (uVar33 = *(ulonglong *)
                      (*(longlong *)(lVar28 + 0x38) + *(ulonglong *)(local_100 + 0x110) * 8),
           uVar33 != 0)) {
          local_a0 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar33 * 0x30);
        }
        else {
          local_a0 = 0;
        }
        local_a8 = *(undefined4 **)(lVar28 + 8);
        uVar33 = 0;
      }
      puVar24 = (ulonglong *)((longlong)local_a8 + uVar33 * 8);
      uVar36 = uVar33 & 0xffffffff;
    }
    if (*puVar24 >> 0x20 == 0) goto LAB_140a95cb6;
    uVar33 = uVar33 + 1;
    lVar28 = *(longlong *)(uVar36 * 0x30 + local_a0);
    if (local_b0[3] != 0) {
      local_f8 = (undefined **)lVar28;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar11 = *local_b0;
      bVar42 = (byte)(extraout_RAX_12 >> 0x39);
      auVar44 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar44 = pshuflw(auVar44,auVar44,0);
      lVar35 = 0;
      uVar36 = extraout_RAX_12;
      while( true ) {
        uVar36 = local_b0[1] & uVar36;
        auVar3 = *(undefined1 (*) [16])(lVar11 + uVar36);
        cVar43 = auVar44[0];
        auVar55[0] = -(auVar3[0] == cVar43);
        cVar45 = auVar44[1];
        auVar55[1] = -(auVar3[1] == cVar45);
        cVar46 = auVar44[2];
        auVar55[2] = -(auVar3[2] == cVar46);
        cVar47 = auVar44[3];
        auVar55[3] = -(auVar3[3] == cVar47);
        auVar55[4] = -(auVar3[4] == cVar43);
        auVar55[5] = -(auVar3[5] == cVar45);
        auVar55[6] = -(auVar3[6] == cVar46);
        auVar55[7] = -(auVar3[7] == cVar47);
        auVar55[8] = -(auVar3[8] == cVar43);
        auVar55[9] = -(auVar3[9] == cVar45);
        auVar55[10] = -(auVar3[10] == cVar46);
        auVar55[0xb] = -(auVar3[0xb] == cVar47);
        auVar55[0xc] = -(auVar3[0xc] == cVar43);
        auVar55[0xd] = -(auVar3[0xd] == cVar45);
        auVar55[0xe] = -(auVar3[0xe] == cVar46);
        auVar55[0xf] = -(auVar3[0xf] == cVar47);
        uVar37 = (ushort)(SUB161(auVar55 >> 7,0) & 1) | (ushort)(SUB161(auVar55 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar55 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar55 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar55 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar55 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar55 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar55 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar55 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar55 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar55 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar55 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar55 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar55 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar55 >> 0x77,0) & 1) << 0xe | (ushort)(auVar55[0xf] >> 7) << 0xf
        ;
        uVar23 = (uint)uVar37;
        while (uVar37 != 0) {
          uVar19 = 0;
          for (uVar12 = uVar23; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
            uVar19 = uVar19 + 1;
          }
          lVar39 = (uVar19 + uVar36 & local_b0[1]) * -0x2a8;
          if (*(longlong *)(lVar11 + -0x2a8 + lVar39) == lVar28) {
            if ((*(ulonglong *)(lVar11 + -0x290 + lVar39) < 2) ||
               (*(float *)(lVar11 + -0x270 + lVar39) <= 0.0)) {
                    /* WARNING: Subroutine does not return */
              core::panicking::panic();
            }
            hashbrown::map::HashMap::
            insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            goto LAB_140a95a09;
          }
          uVar37 = (ushort)(uVar23 - 1) & (ushort)uVar23;
          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar37);
        }
        auVar51[0] = -(auVar3[0] == -1);
        auVar51[1] = -(auVar3[1] == -1);
        auVar51[2] = -(auVar3[2] == -1);
        auVar51[3] = -(auVar3[3] == -1);
        auVar51[4] = -(auVar3[4] == -1);
        auVar51[5] = -(auVar3[5] == -1);
        auVar51[6] = -(auVar3[6] == -1);
        auVar51[7] = -(auVar3[7] == -1);
        auVar51[8] = -(auVar3[8] == -1);
        auVar51[9] = -(auVar3[9] == -1);
        auVar51[10] = -(auVar3[10] == -1);
        auVar51[0xb] = -(auVar3[0xb] == -1);
        auVar51[0xc] = -(auVar3[0xc] == -1);
        auVar51[0xd] = -(auVar3[0xd] == -1);
        auVar51[0xe] = -(auVar3[0xe] == -1);
        auVar51[0xf] = -(auVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x1f,0) & 1) != 0) || (SUB161(auVar51 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x37,0) & 1) != 0) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
        uVar36 = uVar36 + lVar35 + 0x10;
        lVar35 = lVar35 + 0x10;
      }
    }
  } while( true );
LAB_140a957e0:
  if (extraout_AL_00 == '\0') {
LAB_140a957fb:
    *local_a8 = (undefined4)local_a0;
    lVar34 = *(longlong *)(lVar39 + 0x30);
    uVar9 = *(undefined8 *)(lVar39 + 0x40);
    if (lVar34 == *(longlong *)(lVar39 + 0x20)) {
      local_91 = 1;
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar38 = *(longlong *)(lVar39 + 0x28);
    *(undefined8 *)(lVar38 + lVar34 * 0x10) = uVar9;
    *(longlong *)(lVar38 + 8 + lVar34 * 0x10) = lVar21;
    *(longlong *)(lVar39 + 0x30) = lVar34 + 1;
    *(longlong *)(lVar39 + 0x40) = *(longlong *)(lVar39 + 0x40) + 1;
  }
  goto LAB_140a95601;
LAB_140a95cb6:
  local_d8 = local_148;
  puStack_d0 = (undefined8 *)lStack_140;
  local_e8 = (longlong **)local_158;
  lStack_e0 = lStack_150;
  local_f8 = (undefined **)local_168;
  uStack_f0 = (ulonglong)pcStack_160;
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  plVar25 = (longlong *)*in_R8;
  if ((plVar25[1] != 0) && (plVar25[1] * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar25[4] = local_d8;
  plVar25[5] = (longlong)puStack_d0;
  plVar25[2] = (longlong)local_e8;
  plVar25[3] = lStack_e0;
  *plVar25 = (longlong)local_f8;
  plVar25[1] = uStack_f0;
  if (local_93 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_13 == 1) {
      plVar25 = extraout_RAX_13 + 1;
    }
    else if ((*extraout_RAX_13 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar25 = extraout_RAX_15, extraout_RAX_15 == (longlong *)0x0)) {
LAB_140a95dec:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar25 != 0) {
LAB_140a95e39:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar25 = -1;
    local_b8 = plVar25;
    puffin::ThreadProfiler::end_scope();
    *local_b8 = *local_b8 + 1;
  }
  return;
}

