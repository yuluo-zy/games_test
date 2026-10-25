
// WARNING: Removing unreachable block (ram,0x000140a9568b)
// WARNING: Removing unreachable block (ram,0x000140a956c6)
// WARNING: Removing unreachable block (ram,0x000140a956de)
// WARNING: Removing unreachable block (ram,0x000140a956fc)
// WARNING: Removing unreachable block (ram,0x000140a95710)
// WARNING: Removing unreachable block (ram,0x000140a9571f)
// WARNING: Removing unreachable block (ram,0x000140a9573b)
// WARNING: Removing unreachable block (ram,0x000140a95776)
// WARNING: Removing unreachable block (ram,0x000140a9578e)
// WARNING: Removing unreachable block (ram,0x000140a957e0)
// WARNING: Removing unreachable block (ram,0x000140a957eb)
// WARNING: Removing unreachable block (ram,0x000140a957a8)
// WARNING: Removing unreachable block (ram,0x000140a957b5)
// WARNING: Removing unreachable block (ram,0x000140a957c0)
// WARNING: Removing unreachable block (ram,0x000140a957cf)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::systems::wall::wall_state::wall_roof_hashmap::wall_roof_hashmap(void)

{
  byte *pbVar1;
  longlong *plVar2;
  uint *puVar3;
  undefined1 auVar4 [16];
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  longlong lVar8;
  int *piVar9;
  undefined4 *puVar10;
  undefined8 uVar11;
  undefined8 uVar12;
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
  byte *extraout_RAX_08;
  longlong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  uint uVar23;
  longlong *in_RCX;
  ulonglong *puVar24;
  longlong *plVar25;
  undefined8 *in_RDX;
  ulonglong uVar26;
  undefined8 *puVar27;
  longlong lVar28;
  undefined8 *puVar29;
  uint *puVar30;
  undefined1 (*pauVar31) [16];
  undefined8 *in_R8;
  ulonglong uVar32;
  longlong *in_R9;
  longlong lVar33;
  longlong lVar34;
  ulonglong uVar35;
  ushort uVar36;
  longlong lVar37;
  undefined1 (*pauVar38) [16];
  undefined8 *puVar39;
  byte bVar40;
  char cVar41;
  char cVar43;
  char cVar44;
  char cVar45;
  undefined1 auVar42 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
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
  undefined8 *local_168;
  code *pcStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  longlong local_148;
  longlong lStack_140;
  undefined8 local_138;
  longlong *local_130;
  longlong local_128;
  longlong local_120;
  undefined8 *local_118;
  undefined4 local_10c;
  longlong local_108;
  longlong local_100;
  undefined **local_f8;
  longlong lStack_f0;
  undefined8 **local_e8;
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
  puVar39 = in_RDX;
  puffin::are_scopes_on();
  local_118 = in_RDX;
  if (extraout_AL != '\0') {
    puVar39 = (undefined8 *)&DAT_142ada041;
    uVar26 = 0x4d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar26 != 0) {
        if (uVar26 < 0x4d) {
          if ((char)(&DAT_142ada041)[uVar26] < -0x40) {
LAB_140a94d74:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar26 != 0x4d) goto LAB_140a94d74;
      }
      puVar39 = (undefined8 *)&DAT_142ada041;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar26 != 0xfffffffffffffffe) {
          if (uVar26 + 2 < 0x4d) {
            if ((char)(&UNK_142ada043)[uVar26] < -0x40) {
LAB_140a94dd2:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar26 + 2 != 0x4d) goto LAB_140a94dd2;
        }
        puVar39 = (undefined8 *)(&UNK_142ada043 + uVar26);
      }
    }
    puVar20 = &DAT_142ada494;
    do {
      bVar40 = puVar20[-1];
      uVar23 = (uint)(char)bVar40;
      if ((char)bVar40 < '\0') {
        bVar5 = puVar20[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar20[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar20 + -4;
            puVar20 = puVar20 + -4;
            uVar23 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar20 = puVar20 + -3;
            uVar23 = bVar6 & 0xf;
          }
          uVar23 = bVar5 & 0x3f | uVar23 << 6;
        }
        else {
          puVar20 = puVar20 + -2;
          uVar23 = bVar5 & 0x1f;
        }
        uVar23 = (uint)(bVar40 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_140a94e99;
LAB_140a94eae:
        if (puVar20 == &UNK_142ada44f) break;
        if (puVar20 + -0x142ada44f < (undefined *)0x44) {
          if (-0x41 < (char)puVar20[1]) break;
        }
        else if (puVar20 + -0x142ada44f == (undefined *)0x44) break;
                    // WARNING: Subroutine does not return
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
                       (), plVar25 = extraout_RAX_12, extraout_RAX_12 == (longlong *)0x0))
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
    *(undefined8 **)(extraout_RAX_02 + 0x10) = puVar39;
    lVar28 = extraout_RAX_03;
  }
  else {
    lVar28 = *(longlong *)(extraout_RAX_02 + 8);
    puVar39 = *(undefined8 **)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = lVar28 + 1;
  puVar27 = puVar39;
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
    lVar21 = extraout_RAX_05;
  }
  else {
    lVar21 = *(longlong *)(extraout_RAX_04 + 8);
    puVar27 = *(undefined8 **)(extraout_RAX_04 + 0x10);
  }
  *(longlong *)(extraout_RAX_04 + 8) = lVar21 + 1;
  local_108 = *in_R9;
  lVar8 = in_R9[1];
  puVar30 = *(uint **)(local_108 + 0xf8);
  puVar3 = puVar30 + *(longlong *)(local_108 + 0x100);
  local_b8 = (longlong *)CONCAT71(local_b8._1_7_,*(undefined1 *)(local_108 + 0x108));
  piVar9 = (int *)*in_RCX;
  local_10c = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  local_100 = *in_stack_00000038;
  puVar10 = (undefined4 *)in_stack_00000038[2];
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
  uVar35 = 0;
  uVar26 = 0;
  local_128 = 8;
  local_a0 = 0;
  local_a8 = (undefined4 *)((ulonglong)local_a8._4_4_ << 0x20);
LAB_140a95116:
  do {
    if ((char)local_b8 == '\0') {
      if (uVar35 == uVar26) {
        do {
          if (puVar30 == puVar3) goto LAB_140a9557b;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar26 = *(ulonglong *)(*(longlong *)(lVar8 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0);
        } while (uVar26 == 0);
        lVar34 = *(longlong *)(lVar8 + 0x100) + (ulonglong)uVar23 * 0xa0;
        if (*(ulonglong *)(local_108 + 0x110) <
            *(ulonglong *)
             (*(longlong *)(lVar8 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar34 + 0x94) * 0x48)) {
          lVar33 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)*(uint *)(lVar34 + 0x94) * 0x48;
          uVar35 = *(ulonglong *)
                    (*(longlong *)(lVar33 + 0x38) + *(ulonglong *)(local_108 + 0x110) * 8);
          if (uVar35 == 0) goto LAB_140a952d3;
          local_a0 = *(longlong *)(*(longlong *)(lVar33 + 0x18) + 0x10 + ~uVar35 * 0x30);
          uVar35 = *(ulonglong *)(local_108 + 0x118);
          if (uVar35 < *(ulonglong *)(lVar34 + 0x88)) goto LAB_140a952f5;
LAB_140a9530d:
          local_a8 = (undefined4 *)((ulonglong)local_a8._4_4_ << 0x20);
        }
        else {
LAB_140a952d3:
          local_a0 = 0;
          uVar35 = *(ulonglong *)(local_108 + 0x118);
          if (*(ulonglong *)(lVar34 + 0x88) <= uVar35) goto LAB_140a9530d;
LAB_140a952f5:
          if (*(longlong *)(*(longlong *)(lVar34 + 0x80) + uVar35 * 8) == 0) goto LAB_140a9530d;
          local_a8 = (undefined4 *)CONCAT44(local_a8._4_4_,(int)CONCAT71((int7)(uVar35 >> 8),1));
        }
        local_120 = *(longlong *)(lVar34 + 0x50);
        uVar35 = 0;
      }
      puVar22 = (undefined8 *)(local_120 + uVar35 * 0x10);
      uVar32 = (ulonglong)*(uint *)(local_120 + 8 + uVar35 * 0x10);
    }
    else {
      if (uVar35 == uVar26) {
        do {
          if (puVar30 == puVar3) goto LAB_140a9557b;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar26 = *(ulonglong *)(*(longlong *)(lVar8 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48);
        } while (uVar26 == 0);
        lVar34 = *(longlong *)(lVar8 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        uVar35 = *(ulonglong *)(lVar34 + 0x40);
        if ((*(ulonglong *)(local_108 + 0x110) < uVar35) &&
           (uVar32 = *(ulonglong *)
                      (*(longlong *)(lVar34 + 0x38) + *(ulonglong *)(local_108 + 0x110) * 8),
           uVar32 != 0)) {
          local_a0 = *(longlong *)(*(longlong *)(lVar34 + 0x18) + 0x10 + ~uVar32 * 0x30);
          uVar32 = *(ulonglong *)(local_108 + 0x118);
          if (uVar32 < uVar35) goto LAB_140a952a3;
LAB_140a951b6:
          local_a8 = (undefined4 *)((ulonglong)local_a8._4_4_ << 0x20);
        }
        else {
          local_a0 = 0;
          uVar32 = *(ulonglong *)(local_108 + 0x118);
          if (uVar35 <= uVar32) goto LAB_140a951b6;
LAB_140a952a3:
          local_a8 = (undefined4 *)
                     CONCAT44(local_a8._4_4_,
                              (int)CONCAT71((int7)((ulonglong)*(longlong *)(lVar34 + 0x38) >> 8),
                                            *(longlong *)(*(longlong *)(lVar34 + 0x38) + uVar32 * 8)
                                            != 0));
        }
        local_128 = *(longlong *)(lVar34 + 8);
        uVar35 = 0;
      }
      puVar22 = (undefined8 *)(local_128 + uVar35 * 8);
      uVar32 = uVar35 & 0xffffffff;
    }
    puVar29 = (undefined8 *)(uVar32 * 0x58 + local_a0);
    local_c0 = *puVar22;
    uVar35 = uVar35 + 1;
    puVar22 = (undefined8 *)*puVar29;
    local_f8 = (undefined **)puVar22;
    if (local_b0[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar34 = *local_b0;
      bVar40 = (byte)(extraout_RAX_06 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar40,bVar40));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar33 = 0;
      uVar32 = extraout_RAX_06;
      while( true ) {
        uVar32 = local_b0[1] & uVar32;
        auVar4 = *(undefined1 (*) [16])(lVar34 + uVar32);
        cVar41 = auVar42[0];
        auVar48[0] = -(auVar4[0] == cVar41);
        cVar43 = auVar42[1];
        auVar48[1] = -(auVar4[1] == cVar43);
        cVar44 = auVar42[2];
        auVar48[2] = -(auVar4[2] == cVar44);
        cVar45 = auVar42[3];
        auVar48[3] = -(auVar4[3] == cVar45);
        auVar48[4] = -(auVar4[4] == cVar41);
        auVar48[5] = -(auVar4[5] == cVar43);
        auVar48[6] = -(auVar4[6] == cVar44);
        auVar48[7] = -(auVar4[7] == cVar45);
        auVar48[8] = -(auVar4[8] == cVar41);
        auVar48[9] = -(auVar4[9] == cVar43);
        auVar48[10] = -(auVar4[10] == cVar44);
        auVar48[0xb] = -(auVar4[0xb] == cVar45);
        auVar48[0xc] = -(auVar4[0xc] == cVar41);
        auVar48[0xd] = -(auVar4[0xd] == cVar43);
        auVar48[0xe] = -(auVar4[0xe] == cVar44);
        auVar48[0xf] = -(auVar4[0xf] == cVar45);
        uVar36 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe | (ushort)(auVar48[0xf] >> 7) << 0xf
        ;
        uVar23 = (uint)uVar36;
        while (uVar36 != 0) {
          uVar19 = 0;
          for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar19 = uVar19 + 1;
          }
          lVar37 = (uVar19 + uVar32 & local_b0[1]) * -0x2a8;
          if (*(undefined8 **)(lVar34 + -0x2a8 + lVar37) == puVar22) {
            if ((*(ulonglong *)(lVar34 + -0x290 + lVar37) < 2) ||
               (*(float *)(lVar34 + -0x270 + lVar37) <= 0.0)) {
              local_91 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            if (1 < *(uint *)(lVar34 + lVar37 + -0x48)) goto LAB_140a95490;
            if (((ulonglong)local_a8 & 1) == 0) {
              local_91 = 1;
              hashbrown::map::HashMap::
              insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              if (extraout_RAX_07 >> 0x20 != 0) {
                local_f8 = &PTR_s_Only_one_roof_should_be_associat_142ada5a8;
                lStack_f0 = 1;
                local_e8 = (undefined8 **)0x8;
                lStack_e0 = 0;
                local_d8 = 0;
                local_91 = 1;
                    // WARNING: Subroutine does not return
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
          uVar36 = (ushort)(uVar23 - 1) & (ushort)uVar23;
          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar36);
        }
        auVar46[0] = -(auVar4[0] == (char)iVar14);
        auVar46[1] = -(auVar4[1] == (char)((uint)iVar14 >> 8));
        auVar46[2] = -(auVar4[2] == (char)((uint)iVar14 >> 0x10));
        auVar46[3] = -(auVar4[3] == (char)((uint)iVar14 >> 0x18));
        auVar46[4] = -(auVar4[4] == (char)iVar15);
        auVar46[5] = -(auVar4[5] == (char)((uint)iVar15 >> 8));
        auVar46[6] = -(auVar4[6] == (char)((uint)iVar15 >> 0x10));
        auVar46[7] = -(auVar4[7] == (char)((uint)iVar15 >> 0x18));
        auVar46[8] = -(auVar4[8] == (char)iVar16);
        auVar46[9] = -(auVar4[9] == (char)((uint)iVar16 >> 8));
        auVar46[10] = -(auVar4[10] == (char)((uint)iVar16 >> 0x10));
        auVar46[0xb] = -(auVar4[0xb] == (char)((uint)iVar16 >> 0x18));
        auVar46[0xc] = -(auVar4[0xc] == (char)iVar17);
        auVar46[0xd] = -(auVar4[0xd] == (char)((uint)iVar17 >> 8));
        auVar46[0xe] = -(auVar4[0xe] == (char)((uint)iVar17 >> 0x10));
        auVar46[0xf] = -(auVar4[0xf] == (char)((uint)iVar17 >> 0x18));
        if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x1f,0) & 1) != 0) || (SUB161(auVar46 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x37,0) & 1) != 0) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
        uVar32 = uVar32 + lVar33 + 0x10;
        lVar33 = lVar33 + 0x10;
      }
    }
LAB_140a95490:
    if (((ulonglong)local_a8 & 1) == 0) {
      local_168 = puVar29;
      if (*piVar9 == 0xe) {
        pcStack_160 = resources::walls::impl$7::fmt;
        local_f8 = &PTR_s_Tried_deleting_roof_during_Repla_142ada538;
        lStack_f0 = 2;
        local_d8 = 0;
        local_e8 = &local_168;
        lStack_e0 = 1;
        local_91 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      if (*piVar9 == 0xd) {
        pcStack_160 = resources::walls::impl$7::fmt;
        local_f8 = &PTR_s_Tried_deleting_roof_during_Repla_142ada4f0;
        lStack_f0 = 2;
        local_d8 = 0;
        local_e8 = &local_168;
        lStack_e0 = 1;
        local_91 = 1;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      uVar11 = *puVar29;
      *puVar10 = local_10c;
      lVar34 = *(longlong *)(local_100 + 0x30);
      uVar12 = *(undefined8 *)(local_100 + 0x40);
      if (lVar34 == *(longlong *)(local_100 + 0x20)) {
        local_91 = 1;
        local_c0 = uVar35;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
        uVar35 = local_c0;
      }
      lVar33 = *(longlong *)(local_100 + 0x28);
      *(undefined8 *)(lVar33 + lVar34 * 0x10) = uVar12;
      *(undefined8 *)(lVar33 + 8 + lVar34 * 0x10) = uVar11;
      *(longlong *)(local_100 + 0x30) = lVar34 + 1;
      *(longlong *)(local_100 + 0x40) = *(longlong *)(local_100 + 0x40) + 1;
    }
  } while( true );
LAB_140a9557b:
  pauVar38 = (undefined1 (*) [16])*local_b0;
  local_b8 = (longlong *)local_b0[3];
  auVar42 = *pauVar38;
  uVar23 = ~(uint)(ushort)((ushort)(SUB161(auVar42 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar42[0xf] >> 7) << 0xf);
  pauVar31 = pauVar38 + 1;
  local_a0 = CONCAT44(local_a0._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x1c));
  lVar8 = *in_stack_00000030;
  local_a8 = (undefined4 *)in_stack_00000030[2];
  local_c0 = lVar8 + 0x20;
  while (plVar25 = local_b0, local_b8 != (longlong *)0x0) {
    if ((short)uVar23 == 0) {
      do {
        auVar42 = *pauVar31;
        uVar36 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
        pauVar38 = pauVar38 + -0x2a8;
        pauVar31 = pauVar31 + 1;
      } while (uVar36 == 0xffff);
      uVar19 = ~(uint)uVar36;
      uVar23 = uVar19 & -(uint)uVar36 - 2;
    }
    else {
      uVar19 = uVar23;
      uVar23 = uVar23 - 1 & uVar23;
    }
    local_b8 = (longlong *)((longlong)local_b8 + -1);
    uVar13 = 0;
    for (; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
      uVar13 = uVar13 + 1;
    }
    uVar11 = *(undefined8 *)((longlong)pauVar38 + (ulonglong)uVar13 * -0x2a8 + -0x2a8);
    local_138 = uVar11;
    resources::walls::public_walls::PublicWalls::has_roof();
                    // WARNING: Read-only address (ram,0x000142ad9ed0) is written
    if (extraout_AL_00 != '\0') {
                    // WARNING: Read-only address (ram,0x000142ad9ed0) is written
      *local_a8 = (undefined4)local_a0;
      lVar34 = *(longlong *)(lVar8 + 0x30);
      uVar12 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar34 == *(longlong *)(lVar8 + 0x20)) {
        local_91 = 1;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar33 = *(longlong *)(lVar8 + 0x28);
      *(undefined8 *)(lVar33 + lVar34 * 0x10) = uVar12;
      *(undefined8 *)(lVar33 + 8 + lVar34 * 0x10) = uVar11;
      *(longlong *)(lVar8 + 0x30) = lVar34 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
    }
  }
  local_e8 = (undefined8 **)0x0;
  lStack_e0 = 0;
  local_f8 = (undefined **)&DAT_142ad9ed0;
  lStack_f0 = 0;
  uVar7 = *(undefined4 *)((longlong)local_118 + 0x1c);
  puVar10 = (undefined4 *)local_118[2];
  *puVar10 = uVar7;
  plVar2 = local_b0 + 6;
  local_d8 = lVar28;
  puStack_d0 = puVar39;
  if ((local_b0[7] != 0) && (local_b0[7] * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar18 = local_b0;
  plVar25[10] = local_d8;
  plVar25[0xb] = (longlong)puStack_d0;
  plVar25[8] = (longlong)local_e8;
  plVar25[9] = lStack_e0;
  *plVar2 = (longlong)local_f8;
  plVar25[7] = lStack_f0;
  local_e8 = (undefined8 **)0x0;
  lStack_e0 = 0;
  local_f8 = (undefined **)&DAT_142ad9ed0;
  lStack_f0 = 0;
  *puVar10 = uVar7;
  plVar25 = local_b0 + 0xc;
  lVar28 = 0;
  local_d8 = lVar21;
  puStack_d0 = puVar27;
  if ((local_b0[0xd] != 0) && (lVar28 = local_b0[0xd] * 0x11 + 0x21, lVar28 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  auVar42._8_8_ = lStack_e0;
  auVar42._0_8_ = local_e8;
  auVar4._8_8_ = puStack_d0;
  auVar4._0_8_ = local_d8;
  *(undefined1 (*) [16])(plVar18 + 0x10) = auVar4;
  *(undefined1 (*) [16])(plVar18 + 0xe) = auVar42;
  *plVar25 = (longlong)local_f8;
  plVar18[0xd] = lStack_f0;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_08 & 1) == 0) {
    local_91 = 0;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_08[0] = 1;
    extraout_RAX_08[1] = 0;
    extraout_RAX_08[2] = 0;
    extraout_RAX_08[3] = 0;
    extraout_RAX_08[4] = 0;
    extraout_RAX_08[5] = 0;
    extraout_RAX_08[6] = 0;
    extraout_RAX_08[7] = 0;
    *(longlong *)(extraout_RAX_08 + 8) = extraout_RAX_09;
    *(longlong *)(extraout_RAX_08 + 0x10) = lVar28;
    local_148 = extraout_RAX_09;
  }
  else {
    local_148 = *(longlong *)(extraout_RAX_08 + 8);
    lVar28 = *(longlong *)(extraout_RAX_08 + 0x10);
  }
  *(longlong *)(extraout_RAX_08 + 8) = local_148 + 1;
  local_158 = 0;
  uStack_150 = 0;
  local_168 = (undefined8 *)&DAT_142ad9ed0;
  pcStack_160 = (code *)0x0;
  local_100 = *in_stack_00000028;
  lVar21 = in_stack_00000028[1];
  puVar30 = *(uint **)(local_100 + 0xf8);
  puVar3 = puVar30 + *(longlong *)(local_100 + 0x100);
  local_b8 = (longlong *)CONCAT71(local_b8._1_7_,*(undefined1 *)(local_100 + 0x108));
  local_a8 = (undefined4 *)0x8;
  local_a0 = 0;
  local_c0 = 8;
  uVar26 = 0;
  uVar35 = 0;
  lStack_140 = lVar28;
LAB_140a95a09:
  do {
    if ((char)local_b8 == '\0') {
      if (uVar35 == uVar26) {
        do {
          if (puVar30 == puVar3) goto LAB_140a95cb6;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar26 = *(ulonglong *)(*(longlong *)(lVar21 + 0x100) + 0x58 + (ulonglong)uVar23 * 0xa0);
        } while (uVar26 == 0);
        lVar28 = *(longlong *)(lVar21 + 0x100) + (ulonglong)uVar23 * 0xa0;
        uVar35 = (ulonglong)*(uint *)(lVar28 + 0x94);
        if (*(ulonglong *)(local_100 + 0x110) <
            *(ulonglong *)(*(longlong *)(lVar21 + 0x1a8) + 0x40 + uVar35 * 0x48)) {
          lVar8 = *(longlong *)(lVar21 + 0x1a8) + uVar35 * 0x48;
          uVar35 = *(ulonglong *)
                    (*(longlong *)(lVar8 + 0x38) + *(ulonglong *)(local_100 + 0x110) * 8);
          if (uVar35 == 0) goto LAB_140a95b58;
          local_a0 = *(longlong *)(*(longlong *)(lVar8 + 0x18) + 0x10 + ~uVar35 * 0x30);
        }
        else {
LAB_140a95b58:
          local_a0 = 0;
        }
        local_c0 = *(longlong *)(lVar28 + 0x50);
        uVar35 = 0;
      }
      puVar24 = (ulonglong *)(local_c0 + uVar35 * 0x10);
      uVar32 = (ulonglong)*(uint *)(local_c0 + 8 + uVar35 * 0x10);
    }
    else {
      if (uVar35 == uVar26) {
        do {
          if (puVar30 == puVar3) goto LAB_140a95cb6;
          uVar23 = *puVar30;
          puVar30 = puVar30 + 1;
          uVar26 = *(ulonglong *)(*(longlong *)(lVar21 + 0x1a8) + 0x10 + (ulonglong)uVar23 * 0x48);
        } while (uVar26 == 0);
        lVar28 = *(longlong *)(lVar21 + 0x1a8) + (ulonglong)uVar23 * 0x48;
        if ((*(ulonglong *)(local_100 + 0x110) < *(ulonglong *)(lVar28 + 0x40)) &&
           (uVar35 = *(ulonglong *)
                      (*(longlong *)(lVar28 + 0x38) + *(ulonglong *)(local_100 + 0x110) * 8),
           uVar35 != 0)) {
          local_a0 = *(longlong *)(*(longlong *)(lVar28 + 0x18) + 0x10 + ~uVar35 * 0x30);
        }
        else {
          local_a0 = 0;
        }
        local_a8 = *(undefined4 **)(lVar28 + 8);
        uVar35 = 0;
      }
      puVar24 = (ulonglong *)((longlong)local_a8 + uVar35 * 8);
      uVar32 = uVar35 & 0xffffffff;
    }
    if (*puVar24 >> 0x20 == 0) break;
    uVar35 = uVar35 + 1;
    lVar28 = *(longlong *)(uVar32 * 0x30 + local_a0);
    if (local_b0[3] != 0) {
      local_f8 = (undefined **)lVar28;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar8 = *local_b0;
      bVar40 = (byte)(extraout_RAX_10 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar40,bVar40));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar34 = 0;
      uVar32 = extraout_RAX_10;
      while( true ) {
        uVar32 = local_b0[1] & uVar32;
        auVar4 = *(undefined1 (*) [16])(lVar8 + uVar32);
        cVar41 = auVar42[0];
        auVar49[0] = -(auVar4[0] == cVar41);
        cVar43 = auVar42[1];
        auVar49[1] = -(auVar4[1] == cVar43);
        cVar44 = auVar42[2];
        auVar49[2] = -(auVar4[2] == cVar44);
        cVar45 = auVar42[3];
        auVar49[3] = -(auVar4[3] == cVar45);
        auVar49[4] = -(auVar4[4] == cVar41);
        auVar49[5] = -(auVar4[5] == cVar43);
        auVar49[6] = -(auVar4[6] == cVar44);
        auVar49[7] = -(auVar4[7] == cVar45);
        auVar49[8] = -(auVar4[8] == cVar41);
        auVar49[9] = -(auVar4[9] == cVar43);
        auVar49[10] = -(auVar4[10] == cVar44);
        auVar49[0xb] = -(auVar4[0xb] == cVar45);
        auVar49[0xc] = -(auVar4[0xc] == cVar41);
        auVar49[0xd] = -(auVar4[0xd] == cVar43);
        auVar49[0xe] = -(auVar4[0xe] == cVar44);
        auVar49[0xf] = -(auVar4[0xf] == cVar45);
        uVar36 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe | (ushort)(auVar49[0xf] >> 7) << 0xf
        ;
        uVar23 = (uint)uVar36;
        while (uVar36 != 0) {
          uVar19 = 0;
          for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar19 = uVar19 + 1;
          }
          lVar33 = (uVar19 + uVar32 & local_b0[1]) * -0x2a8;
          if (*(longlong *)(lVar8 + -0x2a8 + lVar33) == lVar28) {
            if ((*(ulonglong *)(lVar8 + -0x290 + lVar33) < 2) ||
               (*(float *)(lVar8 + -0x270 + lVar33) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            hashbrown::map::HashMap::
            insert<u64,ref_<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            goto LAB_140a95a09;
          }
          uVar36 = (ushort)(uVar23 - 1) & (ushort)uVar23;
          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar36);
        }
        auVar47[0] = -(auVar4[0] == -1);
        auVar47[1] = -(auVar4[1] == -1);
        auVar47[2] = -(auVar4[2] == -1);
        auVar47[3] = -(auVar4[3] == -1);
        auVar47[4] = -(auVar4[4] == -1);
        auVar47[5] = -(auVar4[5] == -1);
        auVar47[6] = -(auVar4[6] == -1);
        auVar47[7] = -(auVar4[7] == -1);
        auVar47[8] = -(auVar4[8] == -1);
        auVar47[9] = -(auVar4[9] == -1);
        auVar47[10] = -(auVar4[10] == -1);
        auVar47[0xb] = -(auVar4[0xb] == -1);
        auVar47[0xc] = -(auVar4[0xc] == -1);
        auVar47[0xd] = -(auVar4[0xd] == -1);
        auVar47[0xe] = -(auVar4[0xe] == -1);
        auVar47[0xf] = -(auVar4[0xf] == -1);
        if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 || (SUB161(auVar47 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar47 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar47 >> 0x1f,0) & 1) != 0) || (SUB161(auVar47 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar47 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar47 >> 0x37,0) & 1) != 0) || (SUB161(auVar47 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar47 >> 0x47,0) & 1) != 0) || (SUB161(auVar47 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar47 >> 0x57,0) & 1) != 0) || (SUB161(auVar47 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar47 >> 0x67,0) & 1) != 0) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') break;
        uVar32 = uVar32 + lVar34 + 0x10;
        lVar34 = lVar34 + 0x10;
      }
    }
  } while( true );
LAB_140a95cb6:
  local_d8 = local_148;
  puStack_d0 = (undefined8 *)lStack_140;
  local_e8 = (undefined8 **)local_158;
  lStack_e0 = uStack_150;
  local_f8 = (undefined **)local_168;
  lStack_f0 = (longlong)pcStack_160;
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  puVar39 = (undefined8 *)*in_R8;
  if ((puVar39[1] != 0) && (puVar39[1] * 0x11 != -0x21)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  puVar39[4] = local_d8;
  puVar39[5] = puStack_d0;
  puVar39[2] = local_e8;
  puVar39[3] = lStack_e0;
  *puVar39 = local_f8;
  puVar39[1] = lStack_f0;
  if (local_93 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_11 == 1) {
      plVar25 = extraout_RAX_11 + 1;
    }
    else if ((*extraout_RAX_11 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar25 = extraout_RAX_13, extraout_RAX_13 == (longlong *)0x0)) {
LAB_140a95dec:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar25 != 0) {
LAB_140a95e39:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar25 = -1;
    local_b8 = plVar25;
    puffin::ThreadProfiler::end_scope();
    *local_b8 = *local_b8 + 1;
  }
  return;
}

