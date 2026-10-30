void __cdecl system_decorator::cull_oob_decorators::cull_oob_decorators(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  ulonglong *puVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  int iVar11;
  char extraout_AL;
  char extraout_AL_00;
  byte bVar12;
  char extraout_AL_01;
  char extraout_AL_02;
  undefined1 extraout_AL_03;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong lVar14;
  ulonglong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  byte *extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong extraout_RAX_11;
  undefined1 (*extraout_RAX_12) [16];
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong *extraout_RAX_16;
  uint uVar15;
  undefined1 (*in_RCX) [16];
  longlong lVar16;
  longlong *plVar17;
  undefined8 *in_RDX;
  ulonglong uVar18;
  undefined8 uVar19;
  ulonglong uVar20;
  undefined1 (**ppauVar21) [16];
  undefined1 (*pauVar22) [16];
  ushort uVar23;
  undefined1 (*pauVar24) [16];
  undefined1 (*pauVar25) [16];
  undefined8 *in_R8;
  char *pcVar26;
  undefined1 (*in_R9) [16];
  ulonglong uVar27;
  longlong lVar28;
  ulonglong uVar29;
  longlong lVar30;
  undefined4 uVar31;
  longlong lVar32;
  longlong lVar33;
  undefined1 *puVar34;
  longlong lVar35;
  bool bVar36;
  char cVar37;
  char cVar39;
  char cVar40;
  char cVar41;
  undefined1 auVar38 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
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
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  undefined8 *in_stack_00000068;
  longlong *in_stack_00000070;
  undefined8 *in_stack_00000078;
  char local_2e8;
  undefined1 (*local_2a8) [16];
  undefined1 (**ppauStack_2a0) [16];
  ulonglong local_298;
  longlong lStack_290;
  longlong local_288;
  undefined8 uStack_280;
  longlong local_278;
  undefined8 uStack_270;
  undefined8 local_1f0;
  longlong *local_1e8;
  longlong local_1e0;
  undefined1 (*local_1d8) [16];
  undefined1 (**ppauStack_1d0) [16];
  ulonglong local_1c8;
  longlong lStack_1c0;
  longlong local_1b8;
  ulonglong local_1b0;
  undefined1 (*local_1a8) [16];
  undefined1 (**ppauStack_1a0) [16];
  ulonglong local_198;
  longlong lStack_190;
  longlong local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  ulonglong local_168;
  longlong lStack_160;
  longlong local_158;
  undefined8 local_148;
  undefined8 local_140;
  undefined4 *local_138;
  undefined8 *local_130;
  undefined1 (**local_128) [16];
  longlong local_120;
  longlong local_118;
  undefined8 local_110;
  undefined1 (*local_108) [16];
  undefined8 uStack_100;
  ulonglong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong local_e0;
  undefined4 local_d4;
  longlong *local_d0;
  undefined8 *local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined1 (*local_b0) [16];
  undefined4 *local_a8;
  undefined1 (*local_a0) [16];
  ulonglong local_98;
  longlong *local_90;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  local_130 = in_R8;
  local_b0 = in_RCX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_158 = 0;
  }
  else {
    uVar18 = 0x3a;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar18 != 0) {
        if (uVar18 < 0x3a) {
          if ((char)(&DAT_142a8e683)[uVar18] < -0x40) {
LAB_14080d5da:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar18 != 0x3a) goto LAB_14080d5da;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar18 != 0xfffffffffffffffe)) {
        if (uVar18 + 2 < 0x3a) {
          if ((char)(&UNK_142a8e685)[uVar18] < -0x40) {
LAB_14080d638:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar18 + 2 != 0x3a) goto LAB_14080d638;
      }
    }
    pcVar26 = "assertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)";
    do {
      bVar12 = pcVar26[-1];
      uVar13 = (uint)(char)bVar12;
      if ((char)bVar12 < '\0') {
        bVar2 = pcVar26[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = pcVar26[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = (byte *)(pcVar26 + -4);
            pcVar26 = pcVar26 + -4;
            uVar13 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar26 = pcVar26 + -3;
            uVar13 = bVar3 & 0xf;
          }
          uVar13 = bVar2 & 0x3f | uVar13 << 6;
        }
        else {
          pcVar26 = pcVar26 + -2;
          uVar13 = bVar2 & 0x1f;
        }
        uVar13 = (uint)(bVar12 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_14080d6f4;
LAB_14080d705:
        if (pcVar26 == "") break;
        if (pcVar26 + -0x142a8e8af < (char *)0x33) {
          if (-0x41 < pcVar26[1]) break;
        }
        else if (pcVar26 + -0x142a8e8af == (char *)0x33) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar26 = pcVar26 + -1;
      if (uVar13 == 0x5c) goto LAB_14080d705;
LAB_14080d6f4:
      if (uVar13 == 0x2f) goto LAB_14080d705;
    } while (pcVar26 !=
             "crates/systems/decorator/src/cull_oob_decorators.rsassertion failed: decorator_archivist.is_decorator_cull_allowed(locator.decorator)"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar17 = extraout_RAX_01 + 1;
    }
    else {
      if (*extraout_RAX_01 != 0) goto LAB_14080e7c3;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar17 = extraout_RAX_16;
      if (extraout_RAX_16 == (longlong *)0x0) goto LAB_14080e7c3;
    }
    if (*plVar17 != 0) goto LAB_14080e7e9;
    *plVar17 = -1;
    local_90 = plVar17;
    puffin::ThreadProfiler::begin_scope();
    *local_90 = *local_90 + 1;
    local_158 = 1;
  }
  uVar19 = *(undefined8 *)*in_R9;
  local_90 = (longlong *)uVar19;
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  local_f8 = local_298;
  lStack_f0 = lStack_290;
  local_108 = local_2a8;
  uStack_100 = ppauStack_2a0;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
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
    *(undefined8 *)(extraout_RAX_02 + 0x10) = uVar19;
    local_288 = extraout_RAX_03;
  }
  else {
    local_288 = *(longlong *)(extraout_RAX_02 + 8);
    uVar19 = *(undefined8 *)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = local_288 + 1;
  local_298 = 0;
  lStack_290 = 0;
  local_2a8 = (undefined1 (*) [16])&DAT_142a8e510;
  ppauStack_2a0 = (undefined1 (**) [16])0x0;
  local_1c8 = local_f8;
  lStack_1c0 = lStack_f0;
  local_1d8 = local_108;
  ppauStack_1d0 = uStack_100;
  local_1a8 = local_b0;
  local_178 = &local_1a8;
  uStack_280 = uVar19;
  ppauStack_1a0 = &local_2a8;
  local_c8 = in_RDX;
  local_a0 = in_R9;
  _ZN9hashbrown3raw21RawIterRange_LT_T_GT_9fold_impl17h111590a7f0aff7d0E();
  local_1a8 = local_2a8;
  ppauStack_1a0 = ppauStack_2a0;
  local_198 = local_298;
  lStack_190 = lStack_290;
  local_188 = local_288;
  uStack_180 = uStack_280;
  puVar4 = (ulonglong *)*in_stack_00000038;
  lVar30 = in_stack_00000038[1];
  uVar18 = *puVar4;
  uVar20 = 0;
  uVar29 = uVar18 - *(ulonglong *)(lVar30 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar30 + 0x18)) {
    uVar29 = uVar20;
  }
  uVar27 = uVar18 - *(ulonglong *)(lVar30 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar30 + 0x38)) {
    uVar27 = uVar20;
  }
  lVar16 = uVar29 * 0x30 + *(longlong *)(lVar30 + 8);
  uVar18 = *(ulonglong *)(lVar30 + 0x10) - uVar29;
  if (*(ulonglong *)(lVar30 + 0x10) < uVar29) {
    lVar16 = 8;
    uVar18 = uVar20;
  }
  lVar14 = uVar27 * 0x30 + *(longlong *)(lVar30 + 0x28);
  uVar29 = *(ulonglong *)(lVar30 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar30 + 0x30) < uVar27) {
    lVar14 = 8;
    uVar29 = uVar20;
  }
  *puVar4 = (*(longlong *)(lVar30 + 0x40) - uVar29) - uVar18;
  lVar30 = uVar18 * 0x30 + lVar16;
  lVar35 = uVar29 * 0x30 + lVar14;
  iVar8 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                 CONCAT13(unaff_XMM8_Bd,
                          CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  iVar9 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                 CONCAT13(unaff_XMM8_Bh,
                          CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  iVar10 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
LAB_14080da19:
  do {
    if ((lVar16 == 0) || (lVar16 == lVar30)) {
      if ((lVar14 == 0) || (lVar14 == lVar35)) break;
      lVar16 = 0;
      lVar28 = lVar14 + 0x30;
      lVar32 = lVar14;
    }
    else {
      lVar28 = lVar14;
      lVar32 = lVar16;
      lVar16 = lVar16 + 0x30;
    }
    lVar14 = lVar28;
    *puVar4 = *puVar4 + 1;
    plVar17 = *(longlong **)*local_b0;
    if (plVar17[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar12 = (byte)(extraout_RAX_04 >> 0x39);
      auVar38 = ZEXT216(CONCAT11(bVar12,bVar12));
      auVar38 = pshuflw(auVar38,auVar38,0);
      lVar28 = 0;
      uVar18 = extraout_RAX_04;
      while( true ) {
        uVar18 = plVar17[1] & uVar18;
        pcVar26 = (char *)(*plVar17 + uVar18);
        cVar37 = auVar38[0];
        auVar46[0] = -(*pcVar26 == cVar37);
        cVar39 = auVar38[1];
        auVar46[1] = -(pcVar26[1] == cVar39);
        cVar40 = auVar38[2];
        auVar46[2] = -(pcVar26[2] == cVar40);
        cVar41 = auVar38[3];
        auVar46[3] = -(pcVar26[3] == cVar41);
        auVar46[4] = -(pcVar26[4] == cVar37);
        auVar46[5] = -(pcVar26[5] == cVar39);
        auVar46[6] = -(pcVar26[6] == cVar40);
        auVar46[7] = -(pcVar26[7] == cVar41);
        auVar46[8] = -(pcVar26[8] == cVar37);
        auVar46[9] = -(pcVar26[9] == cVar39);
        auVar46[10] = -(pcVar26[10] == cVar40);
        auVar46[0xb] = -(pcVar26[0xb] == cVar41);
        auVar46[0xc] = -(pcVar26[0xc] == cVar37);
        auVar46[0xd] = -(pcVar26[0xd] == cVar39);
        auVar46[0xe] = -(pcVar26[0xe] == cVar40);
        auVar46[0xf] = -(pcVar26[0xf] == cVar41);
        uVar23 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe | (ushort)(auVar46[0xf] >> 7) << 0xf
        ;
        uVar13 = (uint)uVar23;
        if (uVar23 != 0) {
          do {
            uVar15 = 0;
            for (uVar6 = uVar13; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
              uVar15 = uVar15 + 1;
            }
            if (*(longlong *)(lVar32 + 0x18) ==
                *(longlong *)(*plVar17 + -0x2a8 + (uVar15 + uVar18 & plVar17[1]) * -0x2a8)) {
              local_81 = 1;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              goto LAB_14080da19;
            }
            uVar23 = (ushort)(uVar13 - 1) & (ushort)uVar13;
            uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar23);
          } while (uVar23 != 0);
        }
        auVar42[0] = -(*pcVar26 == (char)iVar8);
        auVar42[1] = -(pcVar26[1] == (char)((uint)iVar8 >> 8));
        auVar42[2] = -(pcVar26[2] == (char)((uint)iVar8 >> 0x10));
        auVar42[3] = -(pcVar26[3] == (char)((uint)iVar8 >> 0x18));
        auVar42[4] = -(pcVar26[4] == (char)iVar9);
        auVar42[5] = -(pcVar26[5] == (char)((uint)iVar9 >> 8));
        auVar42[6] = -(pcVar26[6] == (char)((uint)iVar9 >> 0x10));
        auVar42[7] = -(pcVar26[7] == (char)((uint)iVar9 >> 0x18));
        auVar42[8] = -(pcVar26[8] == (char)iVar10);
        auVar42[9] = -(pcVar26[9] == (char)((uint)iVar10 >> 8));
        auVar42[10] = -(pcVar26[10] == (char)((uint)iVar10 >> 0x10));
        auVar42[0xb] = -(pcVar26[0xb] == (char)((uint)iVar10 >> 0x18));
        auVar42[0xc] = -(pcVar26[0xc] == (char)iVar11);
        auVar42[0xd] = -(pcVar26[0xd] == (char)((uint)iVar11 >> 8));
        auVar42[0xe] = -(pcVar26[0xe] == (char)((uint)iVar11 >> 0x10));
        auVar42[0xf] = -(pcVar26[0xf] == (char)((uint)iVar11 >> 0x18));
        if ((((((((((((((((SUB161(auVar42 >> 7,0) & 1) != 0 || (SUB161(auVar42 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar42 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar42 >> 0x1f,0) & 1) != 0) || (SUB161(auVar42 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar42 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar42 >> 0x37,0) & 1) != 0) || (SUB161(auVar42 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar42 >> 0x47,0) & 1) != 0) || (SUB161(auVar42 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar42 >> 0x57,0) & 1) != 0) || (SUB161(auVar42 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar42 >> 0x67,0) & 1) != 0) || (SUB161(auVar42 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar42 >> 0x77,0) & 1) != 0) || auVar42[0xf] < '\0') break;
        uVar18 = uVar18 + lVar28 + 0x10;
        lVar28 = lVar28 + 0x10;
      }
    }
  } while( true );
  puVar4 = (ulonglong *)*in_stack_00000040;
  lVar30 = in_stack_00000040[1];
  uVar18 = *puVar4;
  uVar20 = 0;
  uVar29 = uVar18 - *(ulonglong *)(lVar30 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar30 + 0x18)) {
    uVar29 = uVar20;
  }
  uVar27 = uVar18 - *(ulonglong *)(lVar30 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar30 + 0x38)) {
    uVar27 = uVar20;
  }
  lVar16 = uVar29 * 0x10 + *(longlong *)(lVar30 + 8);
  uVar18 = *(ulonglong *)(lVar30 + 0x10) - uVar29;
  if (*(ulonglong *)(lVar30 + 0x10) < uVar29) {
    lVar16 = 8;
    uVar18 = uVar20;
  }
  lVar14 = uVar27 * 0x10 + *(longlong *)(lVar30 + 0x28);
  uVar29 = *(ulonglong *)(lVar30 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar30 + 0x30) < uVar27) {
    lVar14 = 8;
    uVar29 = uVar20;
  }
  *puVar4 = (*(longlong *)(lVar30 + 0x40) - uVar29) - uVar18;
  lVar30 = uVar18 * 0x10 + lVar16;
  lVar35 = uVar29 * 0x10 + lVar14;
LAB_14080dbd9:
  do {
    if ((lVar16 == 0) || (lVar16 == lVar30)) {
      if ((lVar14 == 0) || (lVar14 == lVar35)) break;
      lVar28 = lVar14 + 0x10;
      lVar32 = 0;
      lVar33 = lVar14;
    }
    else {
      lVar28 = lVar14;
      lVar32 = lVar16 + 0x10;
      lVar33 = lVar16;
    }
    lVar16 = lVar32;
    lVar14 = lVar28;
    *puVar4 = *puVar4 + 1;
    plVar17 = *(longlong **)*local_b0;
    if (plVar17[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar12 = (byte)(extraout_RAX_05 >> 0x39);
      auVar38 = ZEXT216(CONCAT11(bVar12,bVar12));
      auVar38 = pshuflw(auVar38,auVar38,0);
      lVar28 = 0;
      uVar18 = extraout_RAX_05;
      while( true ) {
        uVar18 = plVar17[1] & uVar18;
        pcVar26 = (char *)(*plVar17 + uVar18);
        cVar37 = auVar38[0];
        auVar47[0] = -(*pcVar26 == cVar37);
        cVar39 = auVar38[1];
        auVar47[1] = -(pcVar26[1] == cVar39);
        cVar40 = auVar38[2];
        auVar47[2] = -(pcVar26[2] == cVar40);
        cVar41 = auVar38[3];
        auVar47[3] = -(pcVar26[3] == cVar41);
        auVar47[4] = -(pcVar26[4] == cVar37);
        auVar47[5] = -(pcVar26[5] == cVar39);
        auVar47[6] = -(pcVar26[6] == cVar40);
        auVar47[7] = -(pcVar26[7] == cVar41);
        auVar47[8] = -(pcVar26[8] == cVar37);
        auVar47[9] = -(pcVar26[9] == cVar39);
        auVar47[10] = -(pcVar26[10] == cVar40);
        auVar47[0xb] = -(pcVar26[0xb] == cVar41);
        auVar47[0xc] = -(pcVar26[0xc] == cVar37);
        auVar47[0xd] = -(pcVar26[0xd] == cVar39);
        auVar47[0xe] = -(pcVar26[0xe] == cVar40);
        auVar47[0xf] = -(pcVar26[0xf] == cVar41);
        uVar23 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf
        ;
        uVar13 = (uint)uVar23;
        if (uVar23 != 0) {
          do {
            uVar15 = 0;
            for (uVar6 = uVar13; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
              uVar15 = uVar15 + 1;
            }
            if (*(longlong *)(lVar33 + 8) ==
                *(longlong *)(*plVar17 + -0x2a8 + (uVar15 + uVar18 & plVar17[1]) * -0x2a8)) {
              local_81 = 1;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              goto LAB_14080dbd9;
            }
            uVar23 = (ushort)(uVar13 - 1) & (ushort)uVar13;
            uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar23);
          } while (uVar23 != 0);
        }
        auVar43[0] = -(*pcVar26 == -1);
        auVar43[1] = -(pcVar26[1] == -1);
        auVar43[2] = -(pcVar26[2] == -1);
        auVar43[3] = -(pcVar26[3] == -1);
        auVar43[4] = -(pcVar26[4] == -1);
        auVar43[5] = -(pcVar26[5] == -1);
        auVar43[6] = -(pcVar26[6] == -1);
        auVar43[7] = -(pcVar26[7] == -1);
        auVar43[8] = -(pcVar26[8] == -1);
        auVar43[9] = -(pcVar26[9] == -1);
        auVar43[10] = -(pcVar26[10] == -1);
        auVar43[0xb] = -(pcVar26[0xb] == -1);
        auVar43[0xc] = -(pcVar26[0xc] == -1);
        auVar43[0xd] = -(pcVar26[0xd] == -1);
        auVar43[0xe] = -(pcVar26[0xe] == -1);
        auVar43[0xf] = -(pcVar26[0xf] == -1);
        if ((((((((((((((((SUB161(auVar43 >> 7,0) & 1) != 0 || (SUB161(auVar43 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar43 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar43 >> 0x1f,0) & 1) != 0) || (SUB161(auVar43 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar43 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar43 >> 0x37,0) & 1) != 0) || (SUB161(auVar43 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar43 >> 0x47,0) & 1) != 0) || (SUB161(auVar43 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar43 >> 0x57,0) & 1) != 0) || (SUB161(auVar43 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar43 >> 0x67,0) & 1) != 0) || (SUB161(auVar43 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar43 >> 0x77,0) & 1) != 0) || auVar43[0xf] < '\0') break;
        uVar18 = uVar18 + lVar28 + 0x10;
        lVar28 = lVar28 + 0x10;
      }
    }
  } while( true );
  puVar4 = (ulonglong *)*in_stack_00000048;
  lVar30 = in_stack_00000048[1];
  uVar18 = *puVar4;
  uVar20 = 0;
  uVar29 = uVar18 - *(ulonglong *)(lVar30 + 0x18);
  if (uVar18 < *(ulonglong *)(lVar30 + 0x18)) {
    uVar29 = uVar20;
  }
  uVar27 = uVar18 - *(ulonglong *)(lVar30 + 0x38);
  if (uVar18 < *(ulonglong *)(lVar30 + 0x38)) {
    uVar27 = uVar20;
  }
  lVar16 = uVar29 * 0x40 + *(longlong *)(lVar30 + 8);
  uVar18 = *(ulonglong *)(lVar30 + 0x10) - uVar29;
  if (*(ulonglong *)(lVar30 + 0x10) < uVar29) {
    lVar16 = 8;
    uVar18 = uVar20;
  }
  lVar14 = uVar27 * 0x40 + *(longlong *)(lVar30 + 0x28);
  uVar29 = *(ulonglong *)(lVar30 + 0x30) - uVar27;
  if (*(ulonglong *)(lVar30 + 0x30) < uVar27) {
    lVar14 = 8;
    uVar29 = uVar20;
  }
  uVar20 = (*(longlong *)(lVar30 + 0x40) - uVar29) - uVar18;
  *puVar4 = uVar20;
  lVar30 = uVar18 * 0x40 + lVar16;
  local_a8 = (undefined4 *)(uVar29 * 0x40 + lVar14);
LAB_14080dda6:
  do {
    if ((lVar16 == 0) || (lVar16 == lVar30)) {
      if ((lVar14 == 0) || ((undefined4 *)lVar14 == local_a8)) break;
      lVar16 = 0;
      lVar35 = lVar14 + 0x40;
      lVar28 = lVar14;
    }
    else {
      lVar35 = lVar14;
      lVar28 = lVar16;
      lVar16 = lVar16 + 0x40;
    }
    lVar14 = lVar35;
    *puVar4 = *puVar4 + 1;
    plVar17 = *(longlong **)*local_b0;
    if (plVar17[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar12 = (byte)(extraout_RAX_06 >> 0x39);
      auVar38 = ZEXT216(CONCAT11(bVar12,bVar12));
      auVar38 = pshuflw(auVar38,auVar38,0);
      lVar35 = 0;
      uVar18 = extraout_RAX_06;
      while( true ) {
        uVar18 = plVar17[1] & uVar18;
        pcVar26 = (char *)(*plVar17 + uVar18);
        cVar37 = auVar38[0];
        auVar48[0] = -(*pcVar26 == cVar37);
        cVar39 = auVar38[1];
        auVar48[1] = -(pcVar26[1] == cVar39);
        cVar40 = auVar38[2];
        auVar48[2] = -(pcVar26[2] == cVar40);
        cVar41 = auVar38[3];
        auVar48[3] = -(pcVar26[3] == cVar41);
        auVar48[4] = -(pcVar26[4] == cVar37);
        auVar48[5] = -(pcVar26[5] == cVar39);
        auVar48[6] = -(pcVar26[6] == cVar40);
        auVar48[7] = -(pcVar26[7] == cVar41);
        auVar48[8] = -(pcVar26[8] == cVar37);
        auVar48[9] = -(pcVar26[9] == cVar39);
        auVar48[10] = -(pcVar26[10] == cVar40);
        auVar48[0xb] = -(pcVar26[0xb] == cVar41);
        auVar48[0xc] = -(pcVar26[0xc] == cVar37);
        auVar48[0xd] = -(pcVar26[0xd] == cVar39);
        auVar48[0xe] = -(pcVar26[0xe] == cVar40);
        auVar48[0xf] = -(pcVar26[0xf] == cVar41);
        uVar23 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1
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
        uVar13 = (uint)uVar23;
        if (uVar23 != 0) {
          uVar20 = *(ulonglong *)(lVar28 + 0x20);
          do {
            uVar15 = 0;
            for (uVar6 = uVar13; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
              uVar15 = uVar15 + 1;
            }
            if (uVar20 == *(ulonglong *)
                           (*plVar17 + -0x2a8 + (uVar15 + uVar18 & plVar17[1]) * -0x2a8)) {
              if (*(int *)(lVar28 + 8) != 0) {
                local_81 = 1;
                hashbrown::map::HashMap::
                insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                          ();
              }
              goto LAB_14080dda6;
            }
            uVar23 = (ushort)(uVar13 - 1) & (ushort)uVar13;
            uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar23);
          } while (uVar23 != 0);
        }
        auVar44[0] = -(*pcVar26 == -1);
        auVar44[1] = -(pcVar26[1] == -1);
        auVar44[2] = -(pcVar26[2] == -1);
        auVar44[3] = -(pcVar26[3] == -1);
        auVar44[4] = -(pcVar26[4] == -1);
        auVar44[5] = -(pcVar26[5] == -1);
        auVar44[6] = -(pcVar26[6] == -1);
        auVar44[7] = -(pcVar26[7] == -1);
        auVar44[8] = -(pcVar26[8] == -1);
        auVar44[9] = -(pcVar26[9] == -1);
        auVar44[10] = -(pcVar26[10] == -1);
        auVar44[0xb] = -(pcVar26[0xb] == -1);
        auVar44[0xc] = -(pcVar26[0xc] == -1);
        auVar44[0xd] = -(pcVar26[0xd] == -1);
        auVar44[0xe] = -(pcVar26[0xe] == -1);
        auVar44[0xf] = -(pcVar26[0xf] == -1);
        uVar23 = (ushort)(SUB161(auVar44 >> 7,0) & 1) | (ushort)(SUB161(auVar44 >> 0xf,0) & 1) << 1
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
                 (ushort)(SUB161(auVar44 >> 0x77,0) & 1) << 0xe | (ushort)(auVar44[0xf] >> 7) << 0xf
        ;
        uVar20 = (ulonglong)uVar23;
        if (uVar23 != 0) break;
        uVar18 = uVar18 + lVar35 + 0x10;
        lVar35 = lVar35 + 0x10;
      }
    }
  } while( true );
  local_81 = 1;
  country_core::resources::walls::decorator_subtype::impl$8::from();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_07 & 1) == 0) {
    local_81 = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_07[0] = 1;
    extraout_RAX_07[1] = 0;
    extraout_RAX_07[2] = 0;
    extraout_RAX_07[3] = 0;
    extraout_RAX_07[4] = 0;
    extraout_RAX_07[5] = 0;
    extraout_RAX_07[6] = 0;
    extraout_RAX_07[7] = 0;
    *(longlong *)(extraout_RAX_07 + 8) = extraout_RAX_08;
    *(ulonglong *)(extraout_RAX_07 + 0x10) = uVar20;
    local_1b8 = extraout_RAX_08;
  }
  else {
    local_1b8 = *(longlong *)(extraout_RAX_07 + 8);
    uVar20 = *(ulonglong *)(extraout_RAX_07 + 0x10);
  }
  *(longlong *)(extraout_RAX_07 + 8) = local_1b8 + 1;
  local_1c8 = 0;
  lStack_1c0 = 0;
  local_1d8 = (undefined1 (*) [16])&DAT_142a8e510;
  ppauStack_1d0 = (undefined1 (**) [16])0x0;
  local_128 = ppauStack_1a0;
  if (ppauStack_1a0 == (undefined1 (**) [16])0x0) {
    local_110 = 0;
  }
  else {
    uVar18 = (longlong)ppauStack_1a0 * 8 + 0x17U & 0xfffffffffffffff0;
    local_120 = (longlong)ppauStack_1a0 + uVar18 + 0x11;
    local_118 = (longlong)local_1a8 - uVar18;
    local_110 = 0x10;
  }
  local_148 = *local_130;
  local_d0 = (longlong *)*in_stack_00000030;
  local_140 = *local_c8;
  local_d4 = *(undefined4 *)(local_a0[1] + 0xc);
  local_138 = *(undefined4 **)local_a0[1];
  local_e8 = lStack_190;
  local_1b0 = uVar20;
  if (lStack_190 != 0) {
    auVar38 = *local_1a8;
    local_b8 = *in_stack_00000068;
    local_e0 = *in_stack_00000070;
    local_1f0 = *in_stack_00000078;
    local_a0 = local_1a8;
    pauVar24 = local_1a8 + 1;
    local_1e8 = local_d0 + 4;
    uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar13 == 0) {
        do {
          auVar38 = *pauVar24;
          uVar23 = (ushort)(SUB161(auVar38 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
          local_a0 = local_a0 + -8;
          pauVar24 = pauVar24 + 1;
        } while (uVar23 == 0xffff);
        uVar13 = ~(uint)uVar23;
        uVar15 = uVar13 & -(uint)uVar23 - 2;
      }
      else {
        uVar15 = uVar13 - 1 & uVar13;
      }
      local_c8 = (undefined8 *)CONCAT44(local_c8._4_4_,uVar15);
      iVar8 = 0;
      for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
        iVar8 = iVar8 + 1;
      }
      local_c0 = *(undefined8 *)((longlong)local_a0 + (-8 - (ulonglong)(uint)(iVar8 << 3)));
      country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators();
      local_a8 = (undefined4 *)extraout_RAX_09;
      if (extraout_RAX_09 != 0) {
        pauVar25 = *(undefined1 (**) [16])(extraout_RAX_09 + 0x30);
        lVar30 = *(longlong *)(extraout_RAX_09 + 0x48);
        auVar38 = *pauVar25;
        pauVar22 = pauVar25 + 1;
        local_98 = local_98 & 0xffffffff00000000;
        uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
LAB_14080e0c0:
        bVar36 = lVar30 != 0;
        lVar30 = lVar30 + -1;
        if (bVar36) {
          if ((short)uVar13 == 0) {
            do {
              auVar38 = *pauVar22;
              uVar23 = (ushort)(SUB161(auVar38 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
              pauVar25 = pauVar25 + -0x50;
              pauVar22 = pauVar22 + 1;
            } while (uVar23 == 0xffff);
            uVar13 = ~(uint)uVar23;
            uVar15 = uVar13 & -(uint)uVar23 - 2;
          }
          else {
            uVar15 = uVar13 - 1 & uVar13;
          }
          uVar6 = 0;
          for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar6 = uVar6 + 1;
          }
          uVar18 = (ulonglong)uVar6;
          uVar6 = *(uint *)pauVar25[uVar18 * -5 + -5];
          country_core::resources::walls::decorator_archivist::DecoratorArchivist::
          is_decorator_cull_allowed();
          uVar13 = uVar15;
          if ((extraout_AL_00 != '\0') &&
             (bVar12 = pauVar25[uVar18 * -5 + -1][0xe] - 0x38, bVar12 != 6 && bVar12 < 8)) {
            country_core::resources::rt::impl$5::from();
            puVar34 = pauVar25[uVar18 * -5 + -5] + 8;
            ppauVar21 = (undefined1 (**) [16])(ulonglong)uVar6;
            is_decorator_location_valid();
            if (local_2e8 == '\v') {
              local_2a8 = (undefined1 (*) [16])CONCAT44(local_2a8._4_4_,uVar6);
              if (local_d0[3] != 0) {
                core::hash::BuildHasher::
                hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                          ();
                lVar16 = *local_d0;
                bVar12 = (byte)(extraout_RAX_10 >> 0x39);
                auVar38 = ZEXT216(CONCAT11(bVar12,bVar12));
                auVar38 = pshuflw(auVar38,auVar38,0);
                lVar14 = 0;
                uVar18 = extraout_RAX_10;
                while( true ) {
                  uVar18 = local_d0[1] & uVar18;
                  pcVar26 = (char *)(lVar16 + uVar18);
                  cVar37 = auVar38[0];
                  auVar49[0] = -(*pcVar26 == cVar37);
                  cVar39 = auVar38[1];
                  auVar49[1] = -(pcVar26[1] == cVar39);
                  cVar40 = auVar38[2];
                  auVar49[2] = -(pcVar26[2] == cVar40);
                  cVar41 = auVar38[3];
                  auVar49[3] = -(pcVar26[3] == cVar41);
                  auVar49[4] = -(pcVar26[4] == cVar37);
                  auVar49[5] = -(pcVar26[5] == cVar39);
                  auVar49[6] = -(pcVar26[6] == cVar40);
                  auVar49[7] = -(pcVar26[7] == cVar41);
                  auVar49[8] = -(pcVar26[8] == cVar37);
                  auVar49[9] = -(pcVar26[9] == cVar39);
                  auVar49[10] = -(pcVar26[10] == cVar40);
                  auVar49[0xb] = -(pcVar26[0xb] == cVar41);
                  auVar49[0xc] = -(pcVar26[0xc] == cVar37);
                  auVar49[0xd] = -(pcVar26[0xd] == cVar39);
                  auVar49[0xe] = -(pcVar26[0xe] == cVar40);
                  auVar49[0xf] = -(pcVar26[0xf] == cVar41);
                  uVar23 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                  uVar15 = (uint)uVar23;
                  while (uVar23 != 0) {
                    uVar5 = 0;
                    for (uVar7 = uVar15; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                      uVar5 = uVar5 + 1;
                    }
                    uVar29 = uVar5 + uVar18 & local_d0[1];
                    puVar34 = (undefined1 *)(uVar29 * 0x10);
                    local_1e0 = lVar14;
                    if (*(uint *)(lVar16 + -0x10 + uVar29 * -0x10) == uVar6) {
                      lVar16 = *(longlong *)(lVar16 + -8 + uVar29 * -0x10);
                      if (*(int *)(lVar16 + 0x60) != 0) goto LAB_14080e0c0;
                      uStack_100 = (undefined1 (**) [16])
                                   CONCAT44(uStack_100._4_4_,*(undefined4 *)(lVar16 + 0x6c));
                      local_108 = *(undefined1 (**) [16])(lVar16 + 100);
                      local_f8 = *(ulonglong *)(lVar16 + 0x80);
                      lStack_f0 = *(longlong *)(lVar16 + 0x88);
                      country_core::systems::collision::world::impl$21::to_offset_collider_builder()
                      ;
                      uVar19 = uStack_270;
                      lVar16 = local_278;
                      local_168 = local_298;
                      lStack_160 = lStack_290;
                      local_178._0_4_ = SUB84(local_2a8,0);
                      local_178._4_4_ = (undefined4)((ulonglong)local_2a8 >> 0x20);
                      uStack_170 = SUB84(ppauStack_2a0,0);
                      uStack_16c = (undefined4)((ulonglong)ppauStack_2a0 >> 0x20);
                      if (1 < (uint)local_288) {
                        std::alloc::__default_lib_allocator::__rust_dealloc();
                      }
                      local_2a8 = (undefined1 (*) [16])
                                  CONCAT44(local_178._4_4_,(undefined4)local_178);
                      ppauStack_2a0 = (undefined1 (**) [16])CONCAT44(uStack_16c,uStack_170);
                      local_298 = local_168;
                      lStack_290 = lStack_160;
                      local_288 = lVar16;
                      uStack_280 = uVar19;
                      ppauVar21 = &local_2a8;
                      country_core::systems::collision::world::RaycastWorld::
                      decorator_overlaps_with_others<tuple_<country_core::systems::collision::world::OffsetColliderTransform,alloc::sync::Arc<dyn_<parry3d::shape::shape::Shape>,alloc::alloc::Global>_>_>
                                ();
                      if (extraout_RAX_11 != 0) goto LAB_14080e378;
                      goto LAB_14080e0c0;
                    }
                    uVar23 = (ushort)(uVar15 - 1) & (ushort)uVar15;
                    uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar23);
                  }
                  auVar45[0] = -(*pcVar26 == -1);
                  auVar45[1] = -(pcVar26[1] == -1);
                  auVar45[2] = -(pcVar26[2] == -1);
                  auVar45[3] = -(pcVar26[3] == -1);
                  auVar45[4] = -(pcVar26[4] == -1);
                  auVar45[5] = -(pcVar26[5] == -1);
                  auVar45[6] = -(pcVar26[6] == -1);
                  auVar45[7] = -(pcVar26[7] == -1);
                  auVar45[8] = -(pcVar26[8] == -1);
                  auVar45[9] = -(pcVar26[9] == -1);
                  auVar45[10] = -(pcVar26[10] == -1);
                  auVar45[0xb] = -(pcVar26[0xb] == -1);
                  auVar45[0xc] = -(pcVar26[0xc] == -1);
                  auVar45[0xd] = -(pcVar26[0xd] == -1);
                  auVar45[0xe] = -(pcVar26[0xe] == -1);
                  auVar45[0xf] = -(pcVar26[0xf] == -1);
                  if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 ||
                                   (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                                  (SUB161(auVar45 >> 0x17,0) & 1) != 0) ||
                                 (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                                (SUB161(auVar45 >> 0x27,0) & 1) != 0) ||
                               (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
                              (SUB161(auVar45 >> 0x37,0) & 1) != 0) ||
                             (SUB161(auVar45 >> 0x3f,0) & 1) != 0) ||
                            (SUB161(auVar45 >> 0x47,0) & 1) != 0) ||
                           (SUB161(auVar45 >> 0x4f,0) & 1) != 0) ||
                          (SUB161(auVar45 >> 0x57,0) & 1) != 0) ||
                         (SUB161(auVar45 >> 0x5f,0) & 1) != 0) ||
                        (SUB161(auVar45 >> 0x67,0) & 1) != 0) ||
                       (SUB161(auVar45 >> 0x6f,0) & 1) != 0) || (SUB161(auVar45 >> 0x77,0) & 1) != 0
                      ) || auVar45[0xf] < '\0') break;
                  uVar18 = uVar18 + lVar14 + 0x10;
                  lVar14 = lVar14 + 0x10;
                }
              }
            }
            else {
LAB_14080e378:
              uVar31 = (undefined4)CONCAT71((int7)((ulonglong)puVar34 >> 8),1);
              if ((local_98 & 1) == 0) {
                country_core::resources::history::History::is_recording();
                if (extraout_AL_01 == '\0') {
                  uVar31 = 0;
                }
                else {
                  *local_138 = local_d4;
                  ppauVar21 = (undefined1 (**) [16])0x0;
                  country_core::resources::walls::decorator_archivist::DecoratorArchivist::
                  archive_storage_decorators<ref_<str_>_>();
                }
              }
              country_core::resources::rt::impl$5::from();
              local_298 = CONCAT44(local_298._4_4_,uVar6);
              local_2a8 = extraout_RAX_12;
              ppauStack_2a0 = ppauVar21;
              country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms
              ::position();
              uStack_170 = (undefined4)uStack_100;
              local_178._0_4_ = SUB84(local_108,0);
              local_178._4_4_ = (undefined4)((ulonglong)local_108 >> 0x20);
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::decorator::DecoratorLocator,system_decorator::cull_oob_decorators::cull_oob_decorators::DecoratorRemoveInfo,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              local_98 = CONCAT44(local_98._4_4_,uVar31);
            }
          }
          goto LAB_14080e0c0;
        }
      }
      local_e8 = local_e8 + -1;
      uVar13 = (uint)local_c8;
    } while (local_e8 != 0);
  }
  if ((local_128 != (undefined1 (**) [16])0x0) && (local_120 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  auVar38 = *local_1d8;
  if (ppauStack_1d0 == (undefined1 (**) [16])0x0) {
    local_b8 = 0;
  }
  else {
    uVar18 = ((longlong)ppauStack_1d0 + 1) * 0x28 + 0xfU & 0xfffffffffffffff0;
    local_98 = (longlong)ppauStack_1d0 + uVar18 + 0x11;
    local_e0 = (longlong)local_1d8 - uVar18;
    local_b8 = 0x10;
  }
  pauVar24 = local_1d8 + 1;
  local_b0 = (undefined1 (*) [16])
             CONCAT44(local_b0._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x1c));
  local_a8 = (undefined4 *)in_stack_00000030[2];
  local_a0 = (undefined1 (*) [16])
             CONCAT44(local_a0._4_4_,*(undefined4 *)((longlong)local_130 + 0x1c));
  local_c8 = (undefined8 *)local_130[2];
  local_c0 = *in_stack_00000028;
  uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar38 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar38[0xf] >> 7) << 0xf);
  local_e8 = (longlong)ppauStack_1d0;
  pauVar22 = local_1d8;
  lVar30 = lStack_1c0;
  while (lVar30 != 0) {
    lVar30 = lVar30 + -1;
    if ((short)uVar13 == 0) {
      do {
        auVar38 = *pauVar24;
        uVar23 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar38[0xf] >> 7) << 0xf;
        pauVar22 = pauVar22 + -0x28;
        pauVar24 = pauVar24 + 1;
      } while (uVar23 == 0xffff);
      uVar13 = ~(uint)uVar23;
      uVar15 = uVar13 & -(uint)uVar23 - 2;
    }
    else {
      uVar15 = uVar13 - 1 & uVar13;
    }
    uVar6 = 0;
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar6 = uVar6 + 1;
    }
    uVar29 = (ulonglong)uVar6;
    puVar4 = (ulonglong *)((longlong)pauVar22 + uVar29 * -0x28 + -0x28);
    ppauVar21 = (undefined1 (**) [16])*puVar4;
    uVar18 = puVar4[1];
    uVar31 = *(undefined4 *)((longlong)pauVar22 + uVar29 * -0x28 + -0x18);
    plVar17 = (longlong *)((longlong)pauVar22 + uVar29 * -0x28 + -0x14);
    local_178 = (undefined1 (**) [16])*plVar17;
    lVar16 = plVar17[1];
    uStack_170 = (undefined4)lVar16;
    uStack_16c = (undefined4)((ulonglong)lVar16 >> 0x20);
    if (*(longlong *)((longlong)pauVar22 + uVar29 * -0x28 + -0x28) == 3) break;
    country_core::resources::walls::decorator_archivist::DecoratorArchivist::
    is_decorator_cull_allowed();
    if (extraout_AL_02 == '\0') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    *local_a8 = local_b0._0_4_;
    local_108 = (undefined1 (*) [16])CONCAT44(local_108._4_4_,uVar31);
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    plVar17 = extraout_RAX_13;
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>_>
              ();
    if (plVar17 != (longlong *)0x0) {
      LOCK();
      *plVar17 = *plVar17 + -1;
      UNLOCK();
      if (*plVar17 == 0) {
        alloc::sync::Arc::
        drop_slow<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>();
      }
    }
    lStack_290 = CONCAT44(lStack_290._4_4_,uVar31);
    local_2a8 = (undefined1 (*) [16])&DAT_00000004;
    ppauStack_2a0 = ppauVar21;
    local_298 = uVar18;
    country_core::resources::history::History::is_recording();
    local_108 = (undefined1 (*) [16])CONCAT44(uStack_170,local_178._4_4_);
    uStack_100 = (undefined1 (**) [16])
                 (CONCAT35(uStack_100._5_3_,CONCAT14(extraout_AL_03,uStack_16c)) ^ 0x100000000);
    *(undefined4 *)local_c8 = local_a0._0_4_;
    *local_138 = local_d4;
    apply_decorator_op::apply_decorator_op();
    uVar13 = uVar15;
  }
  if ((local_e8 != 0) && (local_98 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_158 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_14 == 1) {
      plVar17 = extraout_RAX_14 + 1;
    }
    else {
      if (*extraout_RAX_14 != 0) {
LAB_14080e7c3:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar17 = extraout_RAX_15;
      if (extraout_RAX_15 == (longlong *)0x0) goto LAB_14080e7c3;
    }
    if (*plVar17 != 0) {
LAB_14080e7e9:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar17 = -1;
    local_90 = plVar17;
    puffin::ThreadProfiler::end_scope();
    *local_90 = *local_90 + 1;
  }
  return;
}
