// 0x140b2fc00 ui_adjust_wall_height size=2647 sig=void __cdecl ui_adjust_wall_height(void)
// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000142aeba8f) overlaps instruction at (ram,0x000142aeba8e)
// 

void __cdecl
country_core::systems::wall::adjust_wall_height::ui_adjust_wall_height::ui_adjust_wall_height(void)

{
  byte *pbVar1;
  longlong *plVar2;
  char *pcVar3;
  float fVar4;
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  undefined4 uVar8;
  undefined4 uVar9;
  longlong lVar10;
  longlong lVar11;
  longlong *plVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  undefined4 *puVar16;
  longlong lVar17;
  undefined4 *puVar18;
  longlong lVar19;
  undefined4 *puVar20;
  longlong lVar21;
  uint uVar22;
  uint uVar23;
  int iVar24;
  int iVar25;
  int iVar26;
  int iVar27;
  char extraout_AL;
  byte bVar28;
  uint uVar29;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined **ppuVar30;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  undefined *puVar31;
  longlong *plVar32;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  uint uVar33;
  undefined8 *in_RCX;
  longlong *plVar34;
  longlong *plVar35;
  undefined8 *in_RDX;
  ulonglong uVar36;
  longlong lVar37;
  longlong lVar38;
  longlong *in_R9;
  longlong lVar39;
  ushort uVar40;
  ulonglong uVar41;
  ulonglong uVar42;
  undefined1 (*pauVar43) [16];
  undefined1 (*pauVar44) [16];
  ulonglong uVar45;
  ulonglong uVar46;
  bool bVar47;
  bool bVar48;
  bool bVar49;
  char cVar50;
  char cVar52;
  char cVar53;
  char cVar54;
  undefined1 auVar51 [16];
  undefined1 auVar55 [16];
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  undefined1 auVar58 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  char cVar59;
  undefined1 unaff_XMM6_Bc;
  char cVar60;
  undefined1 unaff_XMM6_Bd;
  char cVar61;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  char cVar62;
  undefined1 unaff_XMM6_Bg;
  char cVar63;
  undefined1 unaff_XMM6_Bh;
  char cVar64;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar65;
  undefined1 unaff_XMM6_Bk;
  char cVar66;
  undefined1 unaff_XMM6_Bl;
  char cVar67;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar68;
  undefined1 unaff_XMM6_Bo;
  char cVar69;
  undefined1 unaff_XMM6_Bp;
  char cVar70;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  undefined8 *in_stack_00000050;
  longlong *local_a8;
  uint local_98;
  
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    uVar36 = 0x5d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar36 != 0) {
        if (uVar36 < 0x5d) {
          if ((char)(&DAT_142aeb2d4)[uVar36] < -0x40) {
LAB_140b2fca0:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar36 != 0x5d) goto LAB_140b2fca0;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar36 != 0xfffffffffffffffe)) {
        if (uVar36 + 2 < 0x5d) {
          if ((char)(&UNK_142aeb2d6)[uVar36] < -0x40) {
LAB_140b2fcfe:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar36 + 2 != 0x5d) goto LAB_140b2fcfe;
      }
    }
    ppuVar30 = &PTR_DAT_142aeb928;
    do {
      bVar28 = *(byte *)((longlong)ppuVar30 + -1);
      uVar33 = (uint)(char)bVar28;
      if ((char)bVar28 < '\0') {
        bVar5 = *(byte *)((longlong)ppuVar30 + -2);
        if ((char)bVar5 < -0x40) {
          bVar6 = *(byte *)((longlong)ppuVar30 + -3);
          if ((char)bVar6 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar30 + -4);
            ppuVar30 = (undefined **)((longlong)ppuVar30 + -4);
            uVar33 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar30 = (undefined **)((longlong)ppuVar30 + -3);
            uVar33 = bVar6 & 0xf;
          }
          uVar33 = bVar5 & 0x3f | uVar33 << 6;
        }
        else {
          ppuVar30 = (undefined **)((longlong)ppuVar30 + -2);
          uVar33 = bVar5 & 0x1f;
        }
        uVar33 = (uint)(bVar28 & 0x3f) | uVar33 << 6;
        if (uVar33 != 0x5c) goto LAB_140b2fdc9;
LAB_140b2fddf:
        if (ppuVar30 == (undefined **)&UNK_142aeb8d7) break;
        if ((undefined *)((longlong)ppuVar30 + -0x142aeb8d7) < (undefined *)0x50) {
          if (-0x41 < *(char *)((longlong)ppuVar30 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar30 + -0x142aeb8d7) == (undefined *)0x50) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar30 = (undefined **)((longlong)ppuVar30 + -1);
      if (uVar33 == 0x5c) goto LAB_140b2fddf;
LAB_140b2fdc9:
      if (uVar33 == 0x2f) goto LAB_140b2fddf;
    } while (ppuVar30 != (undefined **)&DAT_142aeb8d8);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar34 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar34 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_140b307c4;
    if (*plVar34 != 0) goto LAB_140b3078a;
    *plVar34 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar34 = *plVar34 + 1;
  }
  uVar33 = *(uint *)*in_RCX;
  if (uVar33 < 6) {
    lVar10 = in_stack_00000050[1];
    uVar36 = *(ulonglong *)*in_stack_00000050;
    uVar41 = 0;
    uVar45 = uVar36 - *(ulonglong *)(lVar10 + 0x18);
    if (uVar36 < *(ulonglong *)(lVar10 + 0x18)) {
      uVar45 = uVar41;
    }
    uVar42 = uVar36 - *(ulonglong *)(lVar10 + 0x38);
    if (uVar36 < *(ulonglong *)(lVar10 + 0x38)) {
      uVar42 = uVar41;
    }
    bVar47 = uVar42 <= *(ulonglong *)(lVar10 + 0x30);
    uVar42 = *(ulonglong *)(lVar10 + 0x30) - uVar42;
    uVar36 = uVar42;
    if (!bVar47) {
      uVar36 = uVar41;
    }
    bVar48 = uVar45 <= *(ulonglong *)(lVar10 + 0x10);
    uVar45 = *(ulonglong *)(lVar10 + 0x10) - uVar45;
    uVar46 = uVar45;
    if (!bVar48) {
      uVar46 = uVar41;
    }
    lVar11 = *(longlong *)((uint *)*in_RCX + 2);
    uVar36 = uVar46 + uVar36;
    if (bVar48 && uVar45 != 0 || bVar47 && uVar42 != 0) {
      uVar36 = uVar41;
    }
    *(ulonglong *)*in_stack_00000050 = *(longlong *)(lVar10 + 0x40) - uVar36;
    plVar12 = (longlong *)*in_RDX;
    pauVar43 = (undefined1 (*) [16])*plVar12;
    auVar51 = *pauVar43;
    local_98 = ~(uint)(ushort)((ushort)(SUB161(auVar51 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar51 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar51 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar51 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar51 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar51 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar51 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar51 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar51 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar51 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar51 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar51 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar51[0xf] >> 7) << 0xf);
    lVar10 = *in_stack_00000040;
    lVar13 = in_stack_00000040[1];
    lVar14 = *in_R9;
    uVar7 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
    lVar15 = *in_stack_00000028;
    puVar16 = (undefined4 *)in_stack_00000028[2];
    uVar8 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    lVar17 = *in_stack_00000030;
    puVar18 = (undefined4 *)in_stack_00000030[2];
    uVar9 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    lVar19 = *in_stack_00000038;
    puVar20 = (undefined4 *)in_stack_00000038[2];
    iVar24 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                              CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                    CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    cVar59 = (char)((uint)iVar24 >> 8);
    cVar60 = (char)((uint)iVar24 >> 0x10);
    cVar61 = (char)((uint)iVar24 >> 0x18);
    iVar25 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                              CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                    CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    cVar62 = (char)((uint)iVar25 >> 8);
    cVar63 = (char)((uint)iVar25 >> 0x10);
    cVar64 = (char)((uint)iVar25 >> 0x18);
    iVar26 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                              CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                    CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    cVar65 = (char)((uint)iVar26 >> 8);
    cVar66 = (char)((uint)iVar26 >> 0x10);
    cVar67 = (char)((uint)iVar26 >> 0x18);
    iVar27 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                              CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                    CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    cVar68 = (char)((uint)iVar27 >> 8);
    cVar69 = (char)((uint)iVar27 >> 0x10);
    cVar70 = (char)((uint)iVar27 >> 0x18);
    local_a8 = (longlong *)plVar12[3];
    pauVar44 = pauVar43 + 1;
    plVar34 = (longlong *)(lVar17 + 0x40);
    plVar2 = (longlong *)(lVar17 + 0x30);
LAB_140b3006b:
    if (local_a8 != (longlong *)0x0) {
      if ((short)local_98 == 0) {
        do {
          auVar51 = *pauVar44;
          uVar40 = (ushort)(SUB161(auVar51 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar51 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar51 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar51 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar51 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar51 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar51 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar51 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar51 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar51 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar51 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar51 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar51 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar51 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar51 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar51[0xf] >> 7) << 0xf;
          pauVar43 = pauVar43 + -0x2a8;
          pauVar44 = pauVar44 + 1;
        } while (uVar40 == 0xffff);
        uVar29 = ~(uint)uVar40;
        local_98 = uVar29 & -(uint)uVar40 - 2;
      }
      else {
        uVar29 = local_98;
        local_98 = local_98 - 1 & local_98;
      }
      local_a8 = (longlong *)((longlong)local_a8 + -1);
      uVar22 = 0;
      for (; (uVar29 & 1) == 0; uVar29 = uVar29 >> 1 | 0x80000000) {
        uVar22 = uVar22 + 1;
      }
      lVar21 = *(longlong *)((longlong)pauVar43 + (ulonglong)uVar22 * -0x2a8 + -0x2a8);
      if (plVar12[9] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar38 = plVar12[6];
        bVar28 = (byte)(extraout_RAX_02 >> 0x39);
        auVar51 = ZEXT216(CONCAT11(bVar28,bVar28));
        auVar51 = pshuflw(auVar51,auVar51,0);
        lVar39 = 0;
        uVar36 = extraout_RAX_02;
        while( true ) {
          uVar36 = plVar12[7] & uVar36;
          pcVar3 = (char *)(lVar38 + uVar36);
          cVar50 = auVar51[0];
          auVar57[0] = -(*pcVar3 == cVar50);
          cVar52 = auVar51[1];
          auVar57[1] = -(pcVar3[1] == cVar52);
          cVar53 = auVar51[2];
          auVar57[2] = -(pcVar3[2] == cVar53);
          cVar54 = auVar51[3];
          auVar57[3] = -(pcVar3[3] == cVar54);
          auVar57[4] = -(pcVar3[4] == cVar50);
          auVar57[5] = -(pcVar3[5] == cVar52);
          auVar57[6] = -(pcVar3[6] == cVar53);
          auVar57[7] = -(pcVar3[7] == cVar54);
          auVar57[8] = -(pcVar3[8] == cVar50);
          auVar57[9] = -(pcVar3[9] == cVar52);
          auVar57[10] = -(pcVar3[10] == cVar53);
          auVar57[0xb] = -(pcVar3[0xb] == cVar54);
          auVar57[0xc] = -(pcVar3[0xc] == cVar50);
          auVar57[0xd] = -(pcVar3[0xd] == cVar52);
          auVar57[0xe] = -(pcVar3[0xe] == cVar53);
          auVar57[0xf] = -(pcVar3[0xf] == cVar54);
          uVar40 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar57[0xf] >> 7) << 0xf;
          uVar29 = (uint)uVar40;
          while (uVar40 != 0) {
            uVar22 = 0;
            for (uVar23 = uVar29; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
              uVar22 = uVar22 + 1;
            }
            uVar41 = uVar22 + uVar36 & plVar12[7];
            if (*(longlong *)(lVar38 + -0x10 + uVar41 * -0x10) == lVar21) {
              uVar36 = *(ulonglong *)(lVar38 + -8 + uVar41 * -0x10);
              if (uVar36 >> 0x20 == 0) goto LAB_140b30290;
              uVar41 = uVar36 & 0xffffffff;
              if ((uVar41 < *(ulonglong *)(lVar13 + 0x10)) &&
                 (*(int *)(*(longlong *)(lVar13 + 8) + uVar41 * 0x14) == (int)(uVar36 >> 0x20))) {
                lVar38 = *(longlong *)(lVar13 + 8) + uVar41 * 0x14;
                uVar36 = (ulonglong)*(uint *)(lVar38 + 4);
                if ((uVar36 != 0xffffffff) &&
                   ((uVar36 < *(ulonglong *)(lVar10 + 0x38) &&
                    ((*(ulonglong *)
                       (*(longlong *)(lVar10 + 0x28) + (ulonglong)(*(uint *)(lVar38 + 4) >> 6) * 8)
                      >> (uVar36 & 0x3f) & 1) != 0)))) {
                  fVar4 = *(float *)(*(longlong *)
                                      (*(longlong *)
                                        (*(longlong *)(lVar13 + 0x1a8) + 0x18 +
                                        (ulonglong)*(uint *)(lVar38 + 0xc) * 0x48) + 0x10 +
                                      ~*(ulonglong *)
                                        (*(longlong *)
                                          (*(longlong *)(lVar13 + 0x1a8) + 0x38 +
                                          (ulonglong)*(uint *)(lVar38 + 0xc) * 0x48) +
                                        *(longlong *)(lVar10 + 0x110) * 8) * 0x30) +
                                    (ulonglong)*(uint *)(lVar38 + 0x10) * 8);
                  if ((fVar4 != 1.0) || (NAN(fVar4))) goto LAB_140b3006b;
                  goto LAB_140b30290;
                }
              }
                    // WARNING: Subroutine does not return
              core::result::unwrap_failed();
            }
            uVar40 = (ushort)(uVar29 - 1) & (ushort)uVar29;
            uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar40);
          }
          auVar55[0] = -(*pcVar3 == (char)iVar24);
          auVar55[1] = -(pcVar3[1] == cVar59);
          auVar55[2] = -(pcVar3[2] == cVar60);
          auVar55[3] = -(pcVar3[3] == cVar61);
          auVar55[4] = -(pcVar3[4] == (char)iVar25);
          auVar55[5] = -(pcVar3[5] == cVar62);
          auVar55[6] = -(pcVar3[6] == cVar63);
          auVar55[7] = -(pcVar3[7] == cVar64);
          auVar55[8] = -(pcVar3[8] == (char)iVar26);
          auVar55[9] = -(pcVar3[9] == cVar65);
          auVar55[10] = -(pcVar3[10] == cVar66);
          auVar55[0xb] = -(pcVar3[0xb] == cVar67);
          auVar55[0xc] = -(pcVar3[0xc] == (char)iVar27);
          auVar55[0xd] = -(pcVar3[0xd] == cVar68);
          auVar55[0xe] = -(pcVar3[0xe] == cVar69);
          auVar55[0xf] = -(pcVar3[0xf] == cVar70);
          if ((((((((((((((((SUB161(auVar55 >> 7,0) & 1) != 0 || (SUB161(auVar55 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar55 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar55 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar55 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar55 >> 0x2f,0) & 1) != 0) || (SUB161(auVar55 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar55 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar55 >> 0x47,0) & 1) != 0) || (SUB161(auVar55 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar55 >> 0x57,0) & 1) != 0) || (SUB161(auVar55 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar55 >> 0x67,0) & 1) != 0) || (SUB161(auVar55 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar55 >> 0x77,0) & 1) != 0) || auVar55[0xf] < '\0') break;
          uVar36 = uVar36 + lVar39 + 0x10;
          lVar39 = lVar39 + 0x10;
        }
      }
LAB_140b30290:
      if (plVar12[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar38 = *plVar12;
        bVar28 = (byte)(extraout_RAX_03 >> 0x39);
        auVar51 = ZEXT216(CONCAT11(bVar28,bVar28));
        auVar51 = pshuflw(auVar51,auVar51,0);
        lVar39 = 0;
        uVar36 = extraout_RAX_03;
        do {
          uVar36 = plVar12[1] & uVar36;
          pcVar3 = (char *)(lVar38 + uVar36);
          cVar50 = auVar51[0];
          auVar58[0] = -(*pcVar3 == cVar50);
          cVar52 = auVar51[1];
          auVar58[1] = -(pcVar3[1] == cVar52);
          cVar53 = auVar51[2];
          auVar58[2] = -(pcVar3[2] == cVar53);
          cVar54 = auVar51[3];
          auVar58[3] = -(pcVar3[3] == cVar54);
          auVar58[4] = -(pcVar3[4] == cVar50);
          auVar58[5] = -(pcVar3[5] == cVar52);
          auVar58[6] = -(pcVar3[6] == cVar53);
          auVar58[7] = -(pcVar3[7] == cVar54);
          auVar58[8] = -(pcVar3[8] == cVar50);
          auVar58[9] = -(pcVar3[9] == cVar52);
          auVar58[10] = -(pcVar3[10] == cVar53);
          auVar58[0xb] = -(pcVar3[0xb] == cVar54);
          auVar58[0xc] = -(pcVar3[0xc] == cVar50);
          auVar58[0xd] = -(pcVar3[0xd] == cVar52);
          auVar58[0xe] = -(pcVar3[0xe] == cVar53);
          auVar58[0xf] = -(pcVar3[0xf] == cVar54);
          uVar40 = (ushort)(SUB161(auVar58 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar58 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar58 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar58 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar58 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar58 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar58 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar58 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar58 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar58 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar58 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar58 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar58 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar58 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar58 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar58[0xf] >> 7) << 0xf;
          uVar29 = (uint)uVar40;
          if (uVar40 != 0) {
LAB_140b302f6:
            uVar22 = 0;
            for (uVar23 = uVar29; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
              uVar22 = uVar22 + 1;
            }
            lVar37 = (uVar22 + uVar36 & plVar12[1]) * -0x2a8;
            if (*(longlong *)(lVar38 + -0x2a8 + lVar37) != lVar21) goto code_r0x000140b30311;
            if ((*(ulonglong *)(lVar38 + -0x290 + lVar37) < 2) ||
               (*(float *)(lVar38 + -0x270 + lVar37) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            if (*(char *)(lVar38 + lVar37 + -9) != '\0') goto LAB_140b3006b;
            if (uVar33 - 1 < 2) {
              if ((lVar11 != lVar21) || (bVar49 = true, *(uint *)(lVar38 + lVar37 + -0x48) < 2))
              goto LAB_140b3006b;
            }
            else {
              if (uVar33 != 0) {
                if (extraout_AL == '\0') {
                  return;
                }
                puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
                if (*extraout_RAX_08 == 1) {
                  local_a8 = extraout_RAX_08 + 1;
                }
                else if ((*extraout_RAX_08 != 0) ||
                        (std::sys::thread_local::native::lazy::Storage::
                         initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                                   (), local_a8 = extraout_RAX_09,
                        extraout_RAX_09 == (longlong *)0x0)) goto LAB_140b307c4;
                if (*local_a8 != 0) goto LAB_140b3078a;
                *local_a8 = -1;
                puffin::ThreadProfiler::end_scope();
                goto LAB_140b306a0;
              }
              bVar49 = false;
            }
            resources::walls::public_walls::PublicWalls::roof_component();
            lVar38 = lVar38 + lVar37 + -0x2a0;
            lVar39 = lVar14;
            adjust_wall_ui_mode::UiWallHeight::widget();
            if ((bVar49 || (!bVar48 || uVar45 == 0) && (!bVar47 || uVar42 == 0)) ||
               (((1 < *(byte *)(extraout_RAX_04 + 0x18) &&
                 (*(longlong *)(extraout_RAX_04 + 0x38) == lVar39)) &&
                (*(char *)(extraout_RAX_04 + 0x19) != '\x02')))) {
              adjust_wall_ui_mode::UiWallHeight::signifier();
              cVar50 = '\x02';
              if ((1 < *(byte *)(extraout_RAX_04 + 0x18)) &&
                 (lVar38 = *(longlong *)(extraout_RAX_04 + 0x38), lVar38 == lVar39)) {
                cVar50 = *(char *)(extraout_RAX_04 + 0x19);
              }
              bVar28 = cVar50 - 2;
              if (2 < bVar28) {
                bVar28 = 3;
              }
              bVar49 = 0xfffffffebd51456f <
                       (ulonglong)(longlong)*(int *)(&DAT_142aeba90 + (ulonglong)bVar28 * 4);
              puVar31 = &DAT_142aeba90 + *(int *)(&DAT_142aeba90 + (ulonglong)bVar28 * 4);
              switch(bVar28) {
              case 0:
                goto LAB_140b30577;
              case 1:
                *puVar18 = uVar8;
                lVar38 = *plVar34;
                lVar39 = *plVar2;
                if (lVar39 == *(longlong *)(lVar17 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar37 = *(longlong *)(lVar17 + 0x28);
                *(longlong *)(lVar37 + lVar39 * 0x10) = lVar38;
                *(longlong *)(lVar37 + 8 + lVar39 * 0x10) = lVar21;
                *(longlong *)(lVar17 + 0x30) = lVar39 + 1;
                *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
                *puVar20 = uVar9;
                lVar21 = *(longlong *)(lVar19 + 0x40);
                lVar39 = *(longlong *)(lVar19 + 0x30);
                if (lVar39 == *(longlong *)(lVar19 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                            ();
                }
                *(longlong *)(*(longlong *)(lVar19 + 0x28) + lVar39 * 8) = lVar21;
                plVar32 = (longlong *)(lVar19 + 0x30);
                plVar35 = (longlong *)(lVar19 + 0x40);
                break;
              case 2:
                *puVar18 = uVar8;
                lVar38 = *plVar34;
                lVar39 = *plVar2;
                if (lVar39 == *(longlong *)(lVar17 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar37 = *(longlong *)(lVar17 + 0x28);
                *(longlong *)(lVar37 + lVar39 * 0x10) = lVar38;
                *(longlong *)(lVar37 + 8 + lVar39 * 0x10) = lVar21;
                plVar32 = plVar2;
                plVar35 = plVar34;
                break;
              case 3:
                *puVar16 = uVar7;
                lVar38 = *(longlong *)(lVar15 + 0x40);
                lVar39 = *(longlong *)(lVar15 + 0x30);
                if (lVar39 == *(longlong *)(lVar15 + 0x20)) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                            ();
                }
                lVar37 = *(longlong *)(lVar15 + 0x28);
                *(longlong *)(lVar37 + lVar39 * 0x10) = lVar38;
                *(longlong *)(lVar37 + 8 + lVar39 * 0x10) = lVar21;
                plVar32 = (longlong *)(lVar15 + 0x30);
                plVar35 = (longlong *)(lVar15 + 0x40);
                break;
              case 4:
              case 5:
              case 9:
              case 0x20:
              case 0x37:
              case 0x57:
              case 0x5c:
              case 0x68:
              case 0x6b:
              case 0x6c:
              case 0x70:
              case 0x8b:
              case 0x8f:
              case 0x94:
              case 0xa8:
              case 0xbf:
              case 0xe0:
              case 0xe4:
              case 0xed:
                    // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
              case 0xc:
              case 0xd:
              case 0x10:
              case 0x2d:
              case 0x40:
              case 0x43:
              case 0x48:
              case 0x6f:
              case 0x7c:
              case 0x8c:
              case 0x9f:
              case 0xac:
              case 0xb0:
              case 0xd3:
              case 0xdb:
              case 0xe5:
              case 0xf5:
              case 0xfd:
                puVar31 = (undefined *)CONCAT71((int7)((ulonglong)puVar31 >> 8),(byte)puVar31 - 2);
              default:
                out(0x4a,(int)puVar31);
              case 8:
              case 0x11:
              case 0x14:
              case 0x15:
              case 0x28:
              case 0x44:
              case 0x47:
              case 0x73:
              case 0x74:
              case 0x88:
              case 0x9c:
              case 0xa3:
              case 0xaf:
              case 0xb8:
              case 200:
              case 0xd7:
              case 0xe8:
                bVar49 = 1 < (byte)puVar31;
              case 0x31:
              case 0x53:
              case 0x83:
              case 0x84:
              case 0xa4:
              case 0xc0:
              case 199:
code_r0x000142aeba94:
                *(int *)(lVar38 + 4) = *(int *)(lVar38 + 4) + 0x42aeba90 + (uint)bVar49;
                    // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
              case 0x19:
              case 0x29:
              case 0x2c:
              case 0x4c:
              case 99:
              case 100:
              case 0x7f:
              case 0x90:
              case 0x93:
              case 0x97:
              case 0xb7:
              case 0xc3:
              case 0xc4:
              case 0xf0:
              case 0xf9:
                halt_baddata();
              case 0x1c:
              case 0x21:
              case 0x25:
              case 0x67:
              case 0x78:
              case 0x7b:
              case 0x87:
              case 0x9b:
              case 0xa0:
              case 0xb3:
              case 0xb4:
              case 0xd8:
              case 0xec:
              case 0xf4:
              case 0xf8:
                DAT_142aeba90 = 0xe8;
                halt_baddata();
              case 0x1d:
              case 0x24:
              case 0x30:
              case 0x34:
              case 0x38:
              case 0x3b:
              case 0x3c:
              case 0x4b:
              case 0x60:
              case 0x98:
              case 0xa7:
              case 0xab:
              case 0xcb:
              case 0xd4:
              case 0xe1:
              case 0xe9:
              case 0xf1:
                bVar49 = 1 < (byte)puVar31;
                goto code_r0x000142aeba94;
              case 0x4f:
              case 0x58:
              case 0x5b:
              case 0x5f:
              case 0x80:
              case 0xbb:
              case 0xbc:
              case 0xcf:
              case 0xdc:
              case 0xfc:
                halt_baddata();
              case 0xcc:
                    // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
              case 0xdf:
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
                    // WARNING: Bad instruction - Truncating control flow here
                halt_baddata();
              }
              *plVar32 = lVar39 + 1;
              *plVar35 = *plVar35 + 1;
            }
            else {
LAB_140b30577:
            }
            goto LAB_140b3006b;
          }
LAB_140b30320:
          auVar56[0] = -(*pcVar3 == (char)iVar24);
          auVar56[1] = -(pcVar3[1] == cVar59);
          auVar56[2] = -(pcVar3[2] == cVar60);
          auVar56[3] = -(pcVar3[3] == cVar61);
          auVar56[4] = -(pcVar3[4] == (char)iVar25);
          auVar56[5] = -(pcVar3[5] == cVar62);
          auVar56[6] = -(pcVar3[6] == cVar63);
          auVar56[7] = -(pcVar3[7] == cVar64);
          auVar56[8] = -(pcVar3[8] == (char)iVar26);
          auVar56[9] = -(pcVar3[9] == cVar65);
          auVar56[10] = -(pcVar3[10] == cVar66);
          auVar56[0xb] = -(pcVar3[0xb] == cVar67);
          auVar56[0xc] = -(pcVar3[0xc] == (char)iVar27);
          auVar56[0xd] = -(pcVar3[0xd] == cVar68);
          auVar56[0xe] = -(pcVar3[0xe] == cVar69);
          auVar56[0xf] = -(pcVar3[0xf] == cVar70);
          if ((((((((((((((((SUB161(auVar56 >> 7,0) & 1) != 0 || (SUB161(auVar56 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar56 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar56 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar56 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar56 >> 0x2f,0) & 1) != 0) || (SUB161(auVar56 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar56 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar56 >> 0x47,0) & 1) != 0) || (SUB161(auVar56 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar56 >> 0x57,0) & 1) != 0) || (SUB161(auVar56 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar56 >> 0x67,0) & 1) != 0) || (SUB161(auVar56 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar56 >> 0x77,0) & 1) != 0) || auVar56[0xf] < '\0') break;
          uVar36 = uVar36 + lVar39 + 0x10;
          lVar39 = lVar39 + 0x10;
        } while( true );
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_05 == 1) {
      local_a8 = extraout_RAX_05 + 1;
    }
    else if ((*extraout_RAX_05 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), local_a8 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0)) {
LAB_140b307c4:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*local_a8 != 0) {
LAB_140b3078a:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *local_a8 = -1;
    puffin::ThreadProfiler::end_scope();
LAB_140b306a0:
    *local_a8 = *local_a8 + 1;
  }
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
  return;
code_r0x000140b30311:
  uVar40 = (ushort)(uVar29 - 1) & (ushort)uVar29;
  uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar40);
  if (uVar40 == 0) goto LAB_140b30320;
  goto LAB_140b302f6;
}
