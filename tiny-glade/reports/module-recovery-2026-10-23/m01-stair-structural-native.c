
void __cdecl
system_decorator::decorator_visual::stairs_assemble::create_structural_colliders_and_initial_graph
          (void)

{
  ulonglong ***pppuVar1;
  ulonglong **ppuVar2;
  byte *pbVar3;
  float *pfVar4;
  int *piVar5;
  undefined8 uVar6;
  byte bVar7;
  byte bVar8;
  int iVar9;
  longlong lVar10;
  longlong lVar11;
  uint uVar12;
  uint uVar13;
  float fVar14;
  float fVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  ulonglong **ppuVar20;
  ulonglong **ppuVar21;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined1 uVar22;
  char extraout_AL_02;
  uint uVar23;
  int extraout_EAX;
  int extraout_EAX_00;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong lVar24;
  char *pcVar25;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  void *_Dst;
  void *_Dst_00;
  void *_Dst_01;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  ulonglong **extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  int *piVar26;
  int *piVar27;
  ulonglong extraout_RAX_11;
  ulonglong extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong *extraout_RAX_16;
  longlong *extraout_RAX_17;
  undefined8 *in_RCX;
  longlong *plVar28;
  longlong *in_RDX;
  ulonglong uVar29;
  int iVar30;
  longlong lVar31;
  ulonglong **ppuVar32;
  longlong *in_R8;
  undefined **ppuVar33;
  char *pcVar34;
  ulonglong **ppuVar35;
  longlong *in_R9;
  ushort uVar36;
  ulonglong uVar37;
  ulonglong ***pppuVar38;
  longlong lVar39;
  float fVar40;
  ulonglong ***pppuVar41;
  int iVar42;
  float fVar43;
  ulonglong **ppuVar44;
  bool bVar45;
  byte bVar46;
  char cVar47;
  char cVar48;
  char cVar55;
  float fVar49;
  char cVar51;
  char cVar52;
  char cVar53;
  char cVar54;
  char cVar56;
  float fVar57;
  undefined1 auVar50 [16];
  undefined1 auVar58 [16];
  float fVar59;
  undefined1 auVar60 [16];
  undefined1 auVar61 [16];
  undefined1 auVar62 [16];
  int iVar68;
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  undefined1 auVar65 [16];
  undefined1 auVar66 [16];
  int iVar69;
  undefined1 auVar67 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  char cVar70;
  undefined1 unaff_XMM6_Bh;
  char cVar71;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  char cVar72;
  undefined1 unaff_XMM6_Bk;
  char cVar73;
  undefined1 unaff_XMM6_Bl;
  char cVar74;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  char cVar75;
  undefined1 unaff_XMM6_Bo;
  char cVar76;
  undefined1 unaff_XMM6_Bp;
  char cVar77;
  undefined1 unaff_XMM11_Ba;
  undefined1 unaff_XMM11_Bb;
  undefined1 unaff_XMM11_Bc;
  undefined1 unaff_XMM11_Bd;
  undefined1 unaff_XMM11_Be;
  undefined1 unaff_XMM11_Bf;
  undefined1 unaff_XMM11_Bg;
  undefined1 unaff_XMM11_Bh;
  undefined1 unaff_XMM11_Bi;
  undefined1 unaff_XMM11_Bj;
  undefined1 unaff_XMM11_Bk;
  undefined1 unaff_XMM11_Bl;
  undefined1 unaff_XMM11_Bm;
  undefined1 unaff_XMM11_Bn;
  undefined1 unaff_XMM11_Bo;
  undefined1 unaff_XMM11_Bp;
  float fVar78;
  float fVar79;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  ulonglong ***local_378;
  code *pcStack_370;
  ulonglong ***local_368;
  code *pcStack_360;
  undefined1 local_358 [16];
  undefined8 local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  undefined4 *local_2f0;
  undefined4 *local_2e8;
  undefined8 local_2e0;
  longlong local_2d8;
  longlong local_2d0;
  longlong local_2c8;
  longlong local_2c0;
  longlong local_2b8;
  longlong *local_2b0;
  longlong local_2a8;
  longlong local_2a0;
  ulonglong **local_290;
  ulonglong ***local_288;
  ulonglong ***pppuStack_280;
  undefined1 local_278 [16];
  undefined8 local_268;
  undefined8 uStack_260;
  longlong local_250;
  float local_248;
  ulonglong ***local_240;
  float local_238;
  float fStack_234;
  float fStack_230;
  undefined4 uStack_22c;
  ulonglong ****local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined1 local_208 [16];
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a0;
  ulonglong *local_198;
  undefined4 local_190;
  undefined4 local_180;
  undefined4 local_17c;
  int local_178;
  int iStack_174;
  int local_170;
  int iStack_16c;
  undefined4 *local_168;
  ulonglong ***local_160;
  int *local_158;
  int *local_150;
  ulonglong ***local_148;
  ulonglong ***local_140;
  char *local_138;
  ulonglong *local_130;
  ulonglong ***local_128;
  ulonglong ***pppuStack_120;
  undefined1 local_118 [8];
  code *pcStack_110;
  longlong *local_100;
  undefined4 local_f4;
  longlong *local_f0;
  undefined1 local_e5;
  undefined1 local_e4;
  char local_e3;
  undefined1 local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_e3 = extraout_AL;
  if (extraout_AL != '\0') {
    uVar29 = 0x62;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar29 != 0) {
        if (uVar29 < 0x62) {
          if ((char)(&DAT_142a87b8b)[uVar29] < -0x40) {
LAB_1407d218e:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar29 != 0x62) goto LAB_1407d218e;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar29 != 0xfffffffffffffffe)) {
        if (uVar29 + 2 < 0x62) {
          if ((char)(&UNK_142a87b8d)[uVar29] < -0x40) {
LAB_1407d21e5:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar29 + 2 != 0x62) goto LAB_1407d21e5;
      }
    }
    ppuVar33 = &PTR_DAT_142a87f40;
    do {
      bVar46 = *(byte *)((longlong)ppuVar33 + -1);
      uVar23 = (uint)(char)bVar46;
      if ((char)bVar46 < '\0') {
        bVar7 = *(byte *)((longlong)ppuVar33 + -2);
        if ((char)bVar7 < -0x40) {
          bVar8 = *(byte *)((longlong)ppuVar33 + -3);
          if ((char)bVar8 < -0x40) {
            pbVar3 = (byte *)((longlong)ppuVar33 + -4);
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
            uVar23 = bVar8 & 0x3f | (*pbVar3 & 7) << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
            uVar23 = bVar8 & 0xf;
          }
          uVar23 = bVar7 & 0x3f | uVar23 << 6;
        }
        else {
          ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
          uVar23 = bVar7 & 0x1f;
        }
        uVar23 = (uint)(bVar46 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_1407d22b4;
LAB_1407d22c6:
        if (ppuVar33 == (undefined **)&UNK_142a87eff) break;
        if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar33 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
      if (uVar23 == 0x5c) goto LAB_1407d22c6;
LAB_1407d22b4:
      if (uVar23 == 0x2f) goto LAB_1407d22c6;
    } while (ppuVar33 != (undefined **)&DAT_142a87f00);
    local_f0 = in_R9;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar28 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar28 = extraout_RAX_16, extraout_RAX_16 == (longlong *)0x0))
    goto LAB_1407d38c0;
    if (*plVar28 != 0) goto LAB_1407d3985;
    *plVar28 = -1;
    local_100 = plVar28;
    puffin::ThreadProfiler::begin_scope();
    *local_100 = *local_100 + 1;
    in_R9 = local_f0;
  }
  lVar10 = *in_R9;
  local_2d0 = lVar10 + 0x198;
  local_2b0 = (longlong *)*in_stack_00000028;
  local_180 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_2e0 = *in_RCX;
  local_2f0 = (undefined4 *)in_RCX[2];
  local_17c = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_2e8 = (undefined4 *)in_RDX[2];
  local_2d8 = *in_RDX + 0x90;
  local_100 = (longlong *)CONCAT44(local_100._4_4_,*(undefined4 *)((longlong)in_R8 + 0x1c));
  lVar11 = *in_R8;
  local_168 = (undefined4 *)in_R8[2];
  local_1a0 = *in_stack_00000030;
  iVar16 = -(uint)(CONCAT13(unaff_XMM11_Bd,
                            CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))) ==
                  CONCAT13(unaff_XMM11_Bd,
                           CONCAT12(unaff_XMM11_Bc,CONCAT11(unaff_XMM11_Bb,unaff_XMM11_Ba))));
  iVar17 = -(uint)(CONCAT13(unaff_XMM11_Bh,
                            CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))) ==
                  CONCAT13(unaff_XMM11_Bh,
                           CONCAT12(unaff_XMM11_Bg,CONCAT11(unaff_XMM11_Bf,unaff_XMM11_Be))));
  iVar18 = -(uint)(CONCAT13(unaff_XMM11_Bl,
                            CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))) ==
                  CONCAT13(unaff_XMM11_Bl,
                           CONCAT12(unaff_XMM11_Bk,CONCAT11(unaff_XMM11_Bj,unaff_XMM11_Bi))));
  iVar19 = -(uint)(CONCAT13(unaff_XMM11_Bp,
                            CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))) ==
                  CONCAT13(unaff_XMM11_Bp,
                           CONCAT12(unaff_XMM11_Bo,CONCAT11(unaff_XMM11_Bn,unaff_XMM11_Bm))));
  lVar24 = *(longlong *)(lVar10 + 0x10) * 0x150 + *(longlong *)(lVar10 + 8);
  lVar10 = *(longlong *)(lVar10 + 8);
  while (lVar31 = lVar10, lVar31 != lVar24) {
    while ((*(char *)(lVar31 + 0x148) != '\x01' || (SBORROW8(0,*(longlong *)(lVar31 + 0x98))))) {
      lVar31 = lVar31 + 0x150;
      if (lVar31 == lVar24) goto LAB_1407d314d;
    }
    local_128 = *(ulonglong ****)(lVar31 + 0x60);
    local_378 = (ulonglong ***)(*(ulonglong *)(lVar31 + 0x118) >> 3);
    local_2a8 = lVar24;
    if (local_128 != local_378) {
      local_238 = 0.0;
      fStack_234 = 0.0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<usize,usize>();
    }
    local_f0 = (longlong *)(lVar31 + 0x128);
    lVar10 = *(longlong *)(lVar31 + 0xa0);
    local_240 = *(ulonglong ****)(lVar31 + 0xa8);
    if (local_2b0[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                ();
      bVar46 = (byte)(extraout_RAX_02 >> 0x39);
      auVar50 = ZEXT216(CONCAT11(bVar46,bVar46));
      auVar50 = pshuflw(auVar50,auVar50,0);
      lVar24 = 0;
      uVar29 = extraout_RAX_02;
      while( true ) {
        uVar29 = local_2b0[1] & uVar29;
        auVar64 = *(undefined1 (*) [16])(*local_2b0 + uVar29);
        cVar47 = auVar50[0];
        auVar60[0] = -(auVar64[0] == cVar47);
        cVar51 = auVar50[1];
        auVar60[1] = -(auVar64[1] == cVar51);
        cVar53 = auVar50[2];
        auVar60[2] = -(auVar64[2] == cVar53);
        cVar55 = auVar50[3];
        auVar60[3] = -(auVar64[3] == cVar55);
        auVar60[4] = -(auVar64[4] == cVar47);
        auVar60[5] = -(auVar64[5] == cVar51);
        auVar60[6] = -(auVar64[6] == cVar53);
        auVar60[7] = -(auVar64[7] == cVar55);
        auVar60[8] = -(auVar64[8] == cVar47);
        auVar60[9] = -(auVar64[9] == cVar51);
        auVar60[10] = -(auVar64[10] == cVar53);
        auVar60[0xb] = -(auVar64[0xb] == cVar55);
        auVar60[0xc] = -(auVar64[0xc] == cVar47);
        auVar60[0xd] = -(auVar64[0xd] == cVar51);
        auVar60[0xe] = -(auVar64[0xe] == cVar53);
        auVar60[0xf] = -(auVar64[0xf] == cVar55);
        uVar36 = (ushort)(SUB161(auVar60 >> 7,0) & 1) | (ushort)(SUB161(auVar60 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar60 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar60 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar60 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar60 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar60 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar60 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar60 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar60 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar60 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar60 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar60 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar60 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar60 >> 0x77,0) & 1) << 0xe | (ushort)(auVar60[0xf] >> 7) << 0xf
        ;
        uVar23 = (uint)uVar36;
        if (uVar36 != 0) {
          do {
            uVar12 = 0;
            for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
              uVar12 = uVar12 + 1;
            }
            uVar37 = uVar12 + uVar29 & local_2b0[1];
            lVar39 = *local_2b0 + uVar37 * -8;
            uVar6 = *(undefined8 *)(lVar39 + -8);
            auVar63._0_4_ = -(uint)((int)uVar6 == (int)*local_f0);
            iVar68 = -(uint)((int)((ulonglong)uVar6 >> 0x20) == (int)((ulonglong)*local_f0 >> 0x20))
            ;
            auVar63._4_4_ = auVar63._0_4_;
            auVar63._8_4_ = iVar68;
            auVar63._12_4_ = iVar68;
            iVar68 = movmskpd((int)uVar37 * 8,auVar63);
            local_f4 = (undefined4)CONCAT71((int7)((ulonglong)lVar39 >> 8),1);
            if (iVar68 == 3) goto LAB_1407d25c0;
            uVar36 = (ushort)(uVar23 - 1) & (ushort)uVar23;
            uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar36);
          } while (uVar36 != 0);
        }
        auVar58[0] = -(auVar64[0] == (char)iVar16);
        auVar58[1] = -(auVar64[1] == (char)((uint)iVar16 >> 8));
        auVar58[2] = -(auVar64[2] == (char)((uint)iVar16 >> 0x10));
        auVar58[3] = -(auVar64[3] == (char)((uint)iVar16 >> 0x18));
        auVar58[4] = -(auVar64[4] == (char)iVar17);
        auVar58[5] = -(auVar64[5] == (char)((uint)iVar17 >> 8));
        auVar58[6] = -(auVar64[6] == (char)((uint)iVar17 >> 0x10));
        auVar58[7] = -(auVar64[7] == (char)((uint)iVar17 >> 0x18));
        auVar58[8] = -(auVar64[8] == (char)iVar18);
        auVar58[9] = -(auVar64[9] == (char)((uint)iVar18 >> 8));
        auVar58[10] = -(auVar64[10] == (char)((uint)iVar18 >> 0x10));
        auVar58[0xb] = -(auVar64[0xb] == (char)((uint)iVar18 >> 0x18));
        auVar58[0xc] = -(auVar64[0xc] == (char)iVar19);
        auVar58[0xd] = -(auVar64[0xd] == (char)((uint)iVar19 >> 8));
        auVar58[0xe] = -(auVar64[0xe] == (char)((uint)iVar19 >> 0x10));
        auVar58[0xf] = -(auVar64[0xf] == (char)((uint)iVar19 >> 0x18));
        if ((((((((((((((((SUB161(auVar58 >> 7,0) & 1) != 0 || (SUB161(auVar58 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar58 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar58 >> 0x1f,0) & 1) != 0) || (SUB161(auVar58 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar58 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar58 >> 0x37,0) & 1) != 0) || (SUB161(auVar58 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar58 >> 0x47,0) & 1) != 0) || (SUB161(auVar58 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar58 >> 0x57,0) & 1) != 0) || (SUB161(auVar58 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar58 >> 0x67,0) & 1) != 0) || (SUB161(auVar58 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar58 >> 0x77,0) & 1) != 0) || auVar58[0xf] < '\0') break;
        uVar29 = uVar29 + lVar24 + 0x10;
        lVar24 = lVar24 + 0x10;
      }
    }
    local_f4 = 0;
LAB_1407d25c0:
    uVar29 = *(ulonglong *)(lVar31 + 0x140);
    if (uVar29 < 2) {
      pcVar34 = (char *)(lVar31 + 0x130);
    }
    else {
      pcVar34 = *(char **)(lVar31 + 0x130);
      uVar29 = *(ulonglong *)(lVar31 + 0x138);
    }
    local_2c8 = lVar31 + 0xd0;
    local_2a0 = lVar31 + 0x150;
    local_2c0 = lVar31 + 0xf0;
    pcVar25 = pcVar34;
    pppuVar41 = (ulonglong ***)0x0;
LAB_1407d2619:
    pppuVar38 = pppuVar41;
    if (pcVar25 != pcVar34 + uVar29 * 4) {
      while( true ) {
        uVar36 = *(ushort *)(pcVar25 + 2);
        ppuVar44 = (ulonglong **)(ulonglong)uVar36;
        pppuVar41 = (ulonglong ***)((longlong)pppuVar38 + (longlong)ppuVar44);
        if (pppuVar41 == (ulonglong ***)0xffffffffffffffff) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_overflow_fail();
        }
        pppuVar1 = (ulonglong ***)((longlong)pppuVar41 + 1);
        ppuVar32 = (ulonglong **)((longlong)pppuVar1 - (longlong)pppuVar38);
        if (pppuVar1 < pppuVar38) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_index_order_fail();
        }
        if (local_240 <= pppuVar41) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        if (*(ulonglong ****)(lVar31 + 0x78) <= pppuVar41) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        if (*(ulonglong ****)(lVar31 + 0x60) <= pppuVar41) {
                    // WARNING: Subroutine does not return
          core::slice::index::slice_end_index_len_fail();
        }
        local_158 = (int *)((longlong)pppuVar38 * 8 + *(longlong *)(lVar31 + 0x70));
        local_150 = (int *)((longlong)pppuVar38 * 4 + *(longlong *)(lVar31 + 0x58));
        auVar50[0xf] = 0;
        auVar50._0_15_ = stack0xfffffffffffffee9;
        _local_118 = auVar50 << 8;
        local_288 = (ulonglong ***)(*(ulonglong *)(lVar31 + 0x118) >> 3);
        local_128 = pppuVar38;
        pppuStack_120 = pppuVar41;
        if (((pppuVar41 < pppuVar38) || (local_288 <= pppuVar38)) || (local_288 <= pppuVar41)) {
          local_378 = (ulonglong ***)&local_128;
          pcStack_370 = core::ops::range::impl$9::fmt<usize>;
          local_368 = (ulonglong ***)&local_288;
          pcStack_360 = core::fmt::num::imp::impl$9::fmt;
          local_238 = 84.23456;
          fStack_234 = 1.4013e-45;
          fStack_230 = 2.8026e-45;
          uStack_22c = 0;
          local_218 = 0;
          local_228 = &local_378;
          uStack_220 = 2;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        local_138 = pcVar25 + 4;
        pfVar4 = (float *)(lVar10 + (longlong)pppuVar38 * 0xc);
        uVar37 = (ulonglong)((uint)*(ulonglong *)(lVar31 + 0x118) & 7) +
                 (ulonglong)((uint)*(ulonglong *)(lVar31 + 0x110) & 7) * 8 + (longlong)pppuVar38;
        local_140 = (ulonglong ***)
                    ((*(ulonglong *)(lVar31 + 0x110) & 0xfffffffffffffff8) + (uVar37 >> 6) * 8);
        uVar23 = (uint)(uVar37 >> 3) & 7;
        uVar37 = (ulonglong)((uint)uVar37 & 7);
        local_160 = pppuVar41;
        local_148 = pppuVar38;
        if (*pcVar25 != '\x03') break;
        if (ppuVar32 != (ulonglong **)0x2) {
          local_238 = 0.0;
          fStack_234 = 0.0;
                    // WARNING: Subroutine does not return
          core::panicking::assert_failed<usize,usize>();
        }
        fVar40 = *pfVar4;
        fVar43 = pfVar4[1];
        fVar14 = pfVar4[2];
        fVar15 = pfVar4[3];
        uVar6 = *(undefined8 *)(pfVar4 + 4);
        fVar78 = (float)uVar6;
        if (fVar43 <= fVar78) {
          country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall();
          fVar79 = 0.0;
          if (extraout_RAX_04 != 0) {
            cVar47 = *(char *)(lVar31 + 0x108);
            goto joined_r0x0001407d27d3;
          }
        }
        else {
          country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall();
          fVar79 = 0.0;
          if (extraout_RAX_03 != 0) {
            cVar47 = *(char *)(lVar31 + 0xe8);
joined_r0x0001407d27d3:
            fVar79 = 0.0;
            if (cVar47 == '\x01') {
              country_core::resources::stairs::stairs_assembly_params::StairNodeAnchor::try_as_wall
                        ();
              fVar79 = 0.28;
              if (extraout_RAX_05 != 0) {
                fVar79 = 0.0;
              }
            }
          }
        }
        fVar49 = fVar40 - fVar15;
        fVar57 = fVar14 - (float)((ulonglong)uVar6 >> 0x20);
        fVar59 = 1.0 / SQRT(fVar57 * fVar57 + fVar49 * fVar49);
        local_378 = (ulonglong ***)CONCAT44(fVar59 * fVar57,fVar59 * fVar49);
        ::utils::swizzlers::impl$0::x0y();
        if ((fVar78 <= fVar43) || (*(byte *)(lVar31 + 0xe8) < 2)) {
LAB_1407d286b:
          fVar49 = 0.0;
          if ((fVar78 < fVar43) && (fVar49 = 0.0, 1 < *(byte *)(lVar31 + 0x108))) {
            calculate_adjusted_ladder_positions::closure_0();
            fVar49 = 0.0;
            if (extraout_AL_01 == '\0') {
              fVar49 = 0.3;
            }
          }
        }
        else {
          calculate_adjusted_ladder_positions::closure_0();
          fVar49 = 0.3;
          if (extraout_AL_00 != '\0') goto LAB_1407d286b;
        }
        if (fVar43 <= fVar78) {
          fVar40 = fVar40 + local_238 * fVar49;
          unaff_XMM6_Ba = SUB41(fVar40,0);
          unaff_XMM6_Bb = (undefined1)((uint)fVar40 >> 8);
          unaff_XMM6_Bc = (undefined1)((uint)fVar40 >> 0x10);
          unaff_XMM6_Bd = (undefined1)((uint)fVar40 >> 0x18);
          fVar43 = fVar43 + fStack_234 * fVar49;
          unaff_XMM6_Be = SUB41(fVar43,0);
          unaff_XMM6_Bf = (undefined1)((uint)fVar43 >> 8);
          unaff_XMM6_Bg = (undefined1)((uint)fVar43 >> 0x10);
          unaff_XMM6_Bh = (undefined1)((uint)fVar43 >> 0x18);
          fVar14 = fVar14 + fStack_230 * fVar49;
          unaff_XMM6_Bi = SUB41(fVar14,0);
          unaff_XMM6_Bj = (undefined1)((uint)fVar14 >> 8);
          unaff_XMM6_Bk = (undefined1)((uint)fVar14 >> 0x10);
          unaff_XMM6_Bl = (undefined1)((uint)fVar14 >> 0x18);
          fVar15 = fVar15 + local_238 * fVar79;
          unaff_XMM6_Bm = SUB41(fVar15,0);
          unaff_XMM6_Bn = (undefined1)((uint)fVar15 >> 8);
          unaff_XMM6_Bo = (undefined1)((uint)fVar15 >> 0x10);
          unaff_XMM6_Bp = (undefined1)((uint)fVar15 >> 0x18);
        }
        else {
          fVar40 = fVar40 - local_238 * fVar79;
          unaff_XMM6_Ba = SUB41(fVar40,0);
          unaff_XMM6_Bb = (undefined1)((uint)fVar40 >> 8);
          unaff_XMM6_Bc = (undefined1)((uint)fVar40 >> 0x10);
          unaff_XMM6_Bd = (undefined1)((uint)fVar40 >> 0x18);
          fVar43 = fVar43 - fStack_234 * fVar79;
          unaff_XMM6_Be = SUB41(fVar43,0);
          unaff_XMM6_Bf = (undefined1)((uint)fVar43 >> 8);
          unaff_XMM6_Bg = (undefined1)((uint)fVar43 >> 0x10);
          unaff_XMM6_Bh = (undefined1)((uint)fVar43 >> 0x18);
          fVar14 = fVar14 - fStack_230 * fVar79;
          unaff_XMM6_Bi = SUB41(fVar14,0);
          unaff_XMM6_Bj = (undefined1)((uint)fVar14 >> 8);
          unaff_XMM6_Bk = (undefined1)((uint)fVar14 >> 0x10);
          unaff_XMM6_Bl = (undefined1)((uint)fVar14 >> 0x18);
          fVar15 = fVar15 - local_238 * fVar49;
          unaff_XMM6_Bm = SUB41(fVar15,0);
          unaff_XMM6_Bn = (undefined1)((uint)fVar15 >> 8);
          unaff_XMM6_Bo = (undefined1)((uint)fVar15 >> 0x10);
          unaff_XMM6_Bp = (undefined1)((uint)fVar15 >> 0x18);
        }
        local_268 = *(undefined8 *)local_158;
        uStack_260 = *(undefined8 *)(local_158 + 2);
        local_2f8 = *(undefined8 *)local_150;
        gen_stairs_structural_collider();
        pppuVar41 = local_148;
        bVar45 = true;
        if (((char)local_f4 != '\0') &&
           (lVar24 = uVar37 + (ulonglong)uVar23 * 8, ((ulonglong)*local_140 >> lVar24 & 1) == 0)) {
          local_198 = (ulonglong *)0x1;
          if (uVar36 == 0) {
            local_288 = (ulonglong ***)0x1;
            local_128 = (ulonglong ***)&local_198;
            pppuStack_120 = (ulonglong ***)core::fmt::num::imp::impl$9::fmt;
            pcStack_110 = core::fmt::num::imp::impl$9::fmt;
            local_118 = (undefined1  [8])&local_288;
            local_238 = 84.24542;
            fStack_234 = 1.4013e-45;
            fStack_230 = 2.8026e-45;
            uStack_22c = 0;
            local_218 = 0;
            local_228 = &local_128;
            uStack_220 = 2;
            local_e2 = 1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          uVar37 = lVar24 + 1;
          bVar45 = ((ulonglong)local_140[uVar37 >> 6] >> (uVar37 & 0x3f) & 1) != 0;
        }
        local_1b8 = local_308;
        uStack_1b0 = uStack_300;
        local_1c8 = local_318;
        uStack_1c0 = uStack_310;
        local_1d8 = local_328;
        uStack_1d0 = uStack_320;
        local_1e8 = local_338;
        uStack_1e0 = uStack_330;
        local_1f8 = local_348;
        uStack_1f0 = uStack_340;
        local_208 = local_358;
        local_218 = local_368;
        uStack_210 = pcStack_360;
        local_228 = (ulonglong ****)local_378;
        uStack_220 = pcStack_370;
        local_238 = 1.4013e-45;
        fStack_234 = 0.0;
        fStack_230 = 1.4013e-45;
        uStack_22c = 0;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        memcpy(_Dst,&local_238,0x90);
        fStack_230 = (float)*local_f0;
        uStack_22c = (undefined4)((ulonglong)*local_f0 >> 0x20);
        fVar40 = SUB84(pppuVar41,0);
        local_238 = (float)CONCAT31(local_238._1_3_,'\x10' - bVar45);
        local_e2 = 0;
        fStack_234 = fVar40;
        country_core::systems::collision::world::RaycastMesh::new();
        *local_2f0 = local_180;
        *local_2e8 = local_17c;
        local_250 = *local_f0;
        _local_118 = local_278;
        local_128 = local_288;
        pppuStack_120 = pppuStack_280;
        local_198 = (ulonglong *)0x3e99999a00000000;
        local_190 = 0;
        local_e4 = 1;
        local_248 = fVar40;
        ::utils::IsoTransform::from_translation();
        local_e4 = 0;
        country_core::systems::collision::world::RaycastWorld::
        upsert_collider<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform>
                  ();
        *local_168 = local_100._0_4_;
        local_238 = (float)*local_f0;
        fStack_234 = (float)((ulonglong)*local_f0 >> 0x20);
        local_e2 = 0;
        fStack_230 = fVar40;
        country_core::resources::stairs::structural_graph::StairsStructuralGraph::upsert_span();
        pcVar25 = local_138;
        pppuVar38 = local_160;
        if (local_138 == pcVar34 + uVar29 * 4) goto LAB_1407d3080;
      }
      local_290 = (ulonglong **)((longlong)ppuVar44 + 1);
      local_2b8 = uVar37 + (ulonglong)uVar23 * 8;
      ppuVar35 = (ulonglong **)0x0;
      bVar45 = false;
      pcVar25 = local_138;
      if (pppuVar1 != pppuVar38) {
        do {
          ppuVar2 = (ulonglong **)((longlong)ppuVar35 + 1);
          ppuVar21 = ppuVar35;
          ppuVar20 = (ulonglong **)local_130;
          if (!bVar45) {
            pcVar25 = local_138;
            pppuVar41 = local_160;
            if (ppuVar32 <= ppuVar2) break;
            ppuVar21 = ppuVar2;
            ppuVar2 = (ulonglong **)((longlong)ppuVar35 + 2);
            ppuVar20 = ppuVar35;
          }
          ppuVar35 = ppuVar2;
          local_130 = (ulonglong *)ppuVar21;
          if (ppuVar32 <= ppuVar20) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          local_248 = pfVar4[(longlong)ppuVar20 * 3 + 2];
          local_250 = *(longlong *)(pfVar4 + (longlong)ppuVar20 * 3);
          uStack_260 = CONCAT44(uStack_260._4_4_,pfVar4[(longlong)local_130 * 3 + 2]);
          local_268 = *(undefined8 *)(pfVar4 + (longlong)local_130 * 3);
          gen_stairs_structural_collider();
          if ((char)local_f4 == '\0') {
LAB_1407d2d4e:
            local_238 = 1.4013e-45;
            fStack_234 = 0.0;
            fStack_230 = 1.4013e-45;
            uStack_22c = 0;
            local_1b8 = local_308;
            uStack_1b0 = uStack_300;
            local_1c8 = local_318;
            uStack_1c0 = uStack_310;
            local_1d8 = local_328;
            uStack_1d0 = uStack_320;
            local_1e8 = local_338;
            uStack_1e0 = uStack_330;
            local_1f8 = local_348;
            uStack_1f0 = uStack_340;
            local_208 = local_358;
            local_218 = local_368;
            uStack_210 = pcStack_360;
            local_228 = (ulonglong ****)local_378;
            uStack_220 = pcStack_370;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (_Dst_00 == (void *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            memcpy(_Dst_00,&local_238,0x90);
            uVar37 = CONCAT17(unaff_XMM6_Bh,
                              CONCAT16(unaff_XMM6_Bg,
                                       CONCAT15(unaff_XMM6_Bf,
                                                CONCAT14(unaff_XMM6_Be,
                                                         CONCAT13(unaff_XMM6_Bd,
                                                                  CONCAT12(unaff_XMM6_Bc,
                                                                           CONCAT11(unaff_XMM6_Bb,
                                                                                    unaff_XMM6_Ba)))
                                                        )))) ^
                     CONCAT17(unaff_XMM6_Bh,
                              CONCAT16(unaff_XMM6_Bg,
                                       CONCAT15(unaff_XMM6_Bf,
                                                CONCAT14(unaff_XMM6_Be,
                                                         CONCAT13(unaff_XMM6_Bd,
                                                                  CONCAT12(unaff_XMM6_Bc,
                                                                           CONCAT11(unaff_XMM6_Bb,
                                                                                    unaff_XMM6_Ba)))
                                                        ))));
            unaff_XMM6_Ba = (undefined1)uVar37;
            unaff_XMM6_Bb = (undefined1)(uVar37 >> 8);
            unaff_XMM6_Bc = (undefined1)(uVar37 >> 0x10);
            unaff_XMM6_Bd = (undefined1)(uVar37 >> 0x18);
            unaff_XMM6_Be = (undefined1)(uVar37 >> 0x20);
            unaff_XMM6_Bf = (undefined1)(uVar37 >> 0x28);
            unaff_XMM6_Bg = (undefined1)(uVar37 >> 0x30);
            unaff_XMM6_Bh = (undefined1)(uVar37 >> 0x38);
            uVar37 = CONCAT17(unaff_XMM6_Bp,
                              CONCAT16(unaff_XMM6_Bo,
                                       CONCAT15(unaff_XMM6_Bn,
                                                CONCAT14(unaff_XMM6_Bm,
                                                         CONCAT13(unaff_XMM6_Bl,
                                                                  CONCAT12(unaff_XMM6_Bk,
                                                                           CONCAT11(unaff_XMM6_Bj,
                                                                                    unaff_XMM6_Bi)))
                                                        )))) ^
                     CONCAT17(unaff_XMM6_Bp,
                              CONCAT16(unaff_XMM6_Bo,
                                       CONCAT15(unaff_XMM6_Bn,
                                                CONCAT14(unaff_XMM6_Bm,
                                                         CONCAT13(unaff_XMM6_Bl,
                                                                  CONCAT12(unaff_XMM6_Bk,
                                                                           CONCAT11(unaff_XMM6_Bj,
                                                                                    unaff_XMM6_Bi)))
                                                        ))));
            unaff_XMM6_Bi = (undefined1)uVar37;
            unaff_XMM6_Bj = (undefined1)(uVar37 >> 8);
            unaff_XMM6_Bk = (undefined1)(uVar37 >> 0x10);
            unaff_XMM6_Bl = (undefined1)(uVar37 >> 0x18);
            unaff_XMM6_Bm = (undefined1)(uVar37 >> 0x20);
            unaff_XMM6_Bn = (undefined1)(uVar37 >> 0x28);
            unaff_XMM6_Bo = (undefined1)(uVar37 >> 0x30);
            unaff_XMM6_Bp = (undefined1)(uVar37 >> 0x38);
            uVar22 = 0xf;
          }
          else {
            local_198 = (ulonglong *)ppuVar20;
            if (ppuVar44 < ppuVar20) {
              local_288 = (ulonglong ***)local_290;
              local_128 = (ulonglong ***)&local_198;
              pppuStack_120 = (ulonglong ***)core::fmt::num::imp::impl$9::fmt;
              pcStack_110 = core::fmt::num::imp::impl$9::fmt;
              local_118 = (undefined1  [8])&local_288;
              local_238 = 84.24542;
              fStack_234 = 1.4013e-45;
              fStack_230 = 2.8026e-45;
              uStack_22c = 0;
              local_218 = 0;
              local_228 = &local_128;
              uStack_220 = 2;
              local_e1 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            if (((ulonglong)local_140[(ulonglong)(local_2b8 + (longlong)ppuVar20) >> 6] >>
                 (local_2b8 + (longlong)ppuVar20 & 0x3fU) & 1) != 0) goto LAB_1407d2d4e;
            local_198 = local_130;
            if (ppuVar44 < local_130) {
              local_288 = (ulonglong ***)local_290;
              local_128 = (ulonglong ***)&local_198;
              pppuStack_120 = (ulonglong ***)core::fmt::num::imp::impl$9::fmt;
              pcStack_110 = core::fmt::num::imp::impl$9::fmt;
              local_118 = (undefined1  [8])&local_288;
              local_238 = 84.24542;
              fStack_234 = 1.4013e-45;
              fStack_230 = 2.8026e-45;
              uStack_22c = 0;
              local_218 = 0;
              local_228 = &local_128;
              uStack_220 = 2;
              local_e1 = 1;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            uVar37 = (longlong)local_130 + local_2b8;
            ppuVar2 = local_140[uVar37 >> 6];
            local_238 = 1.4013e-45;
            fStack_234 = 0.0;
            fStack_230 = 1.4013e-45;
            uStack_22c = 0;
            local_1b8 = local_308;
            uStack_1b0 = uStack_300;
            local_1c8 = local_318;
            uStack_1c0 = uStack_310;
            local_1d8 = local_328;
            uStack_1d0 = uStack_320;
            local_1e8 = local_338;
            uStack_1e0 = uStack_330;
            local_1f8 = local_348;
            uStack_1f0 = uStack_340;
            local_208 = local_358;
            local_218 = local_368;
            uStack_210 = pcStack_360;
            local_228 = (ulonglong ****)local_378;
            uStack_220 = pcStack_370;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (_Dst_01 == (void *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            memcpy(_Dst_01,&local_238,0x90);
            if (((ulonglong)ppuVar2 >> (uVar37 & 0x3f) & 1) == 0) {
              uVar22 = 0x10;
              unaff_XMM6_Ba = 0;
              unaff_XMM6_Bb = 0;
              unaff_XMM6_Bc = 0x80;
              unaff_XMM6_Bd = 0x3f;
              unaff_XMM6_Be = 0;
              unaff_XMM6_Bf = 0;
              unaff_XMM6_Bg = 0;
              unaff_XMM6_Bh = 0;
              unaff_XMM6_Bi = 0;
              unaff_XMM6_Bj = 0;
              unaff_XMM6_Bk = 0;
              unaff_XMM6_Bl = 0;
              unaff_XMM6_Bm = 0;
              unaff_XMM6_Bn = 0;
              unaff_XMM6_Bo = 0;
              unaff_XMM6_Bp = 0;
            }
            else {
              uVar37 = CONCAT17(unaff_XMM6_Bh,
                                CONCAT16(unaff_XMM6_Bg,
                                         CONCAT15(unaff_XMM6_Bf,
                                                  CONCAT14(unaff_XMM6_Be,
                                                           CONCAT13(unaff_XMM6_Bd,
                                                                    CONCAT12(unaff_XMM6_Bc,
                                                                             CONCAT11(unaff_XMM6_Bb,
                                                                                      unaff_XMM6_Ba)
                                                                            )))))) ^
                       CONCAT17(unaff_XMM6_Bh,
                                CONCAT16(unaff_XMM6_Bg,
                                         CONCAT15(unaff_XMM6_Bf,
                                                  CONCAT14(unaff_XMM6_Be,
                                                           CONCAT13(unaff_XMM6_Bd,
                                                                    CONCAT12(unaff_XMM6_Bc,
                                                                             CONCAT11(unaff_XMM6_Bb,
                                                                                      unaff_XMM6_Ba)
                                                                            ))))));
              unaff_XMM6_Ba = (undefined1)uVar37;
              unaff_XMM6_Bb = (undefined1)(uVar37 >> 8);
              unaff_XMM6_Bc = (undefined1)(uVar37 >> 0x10);
              unaff_XMM6_Bd = (undefined1)(uVar37 >> 0x18);
              unaff_XMM6_Be = (undefined1)(uVar37 >> 0x20);
              unaff_XMM6_Bf = (undefined1)(uVar37 >> 0x28);
              unaff_XMM6_Bg = (undefined1)(uVar37 >> 0x30);
              unaff_XMM6_Bh = (undefined1)(uVar37 >> 0x38);
              uVar37 = CONCAT17(unaff_XMM6_Bp,
                                CONCAT16(unaff_XMM6_Bo,
                                         CONCAT15(unaff_XMM6_Bn,
                                                  CONCAT14(unaff_XMM6_Bm,
                                                           CONCAT13(unaff_XMM6_Bl,
                                                                    CONCAT12(unaff_XMM6_Bk,
                                                                             CONCAT11(unaff_XMM6_Bj,
                                                                                      unaff_XMM6_Bi)
                                                                            )))))) ^
                       CONCAT17(unaff_XMM6_Bp,
                                CONCAT16(unaff_XMM6_Bo,
                                         CONCAT15(unaff_XMM6_Bn,
                                                  CONCAT14(unaff_XMM6_Bm,
                                                           CONCAT13(unaff_XMM6_Bl,
                                                                    CONCAT12(unaff_XMM6_Bk,
                                                                             CONCAT11(unaff_XMM6_Bj,
                                                                                      unaff_XMM6_Bi)
                                                                            ))))));
              unaff_XMM6_Bi = (undefined1)uVar37;
              unaff_XMM6_Bj = (undefined1)(uVar37 >> 8);
              unaff_XMM6_Bk = (undefined1)(uVar37 >> 0x10);
              unaff_XMM6_Bl = (undefined1)(uVar37 >> 0x18);
              unaff_XMM6_Bm = (undefined1)(uVar37 >> 0x20);
              unaff_XMM6_Bn = (undefined1)(uVar37 >> 0x28);
              unaff_XMM6_Bo = (undefined1)(uVar37 >> 0x30);
              unaff_XMM6_Bp = (undefined1)(uVar37 >> 0x38);
              uVar22 = 0xf;
            }
          }
          fVar40 = (float)((int)ppuVar20 + (int)local_148);
          fStack_230 = (float)*local_f0;
          uStack_22c = (undefined4)((ulonglong)*local_f0 >> 0x20);
          local_238 = (float)CONCAT31(local_238._1_3_,uVar22);
          local_e1 = 0;
          fStack_234 = fVar40;
          country_core::systems::collision::world::RaycastMesh::new();
          *local_2f0 = local_180;
          *local_2e8 = local_17c;
          _local_118 = local_278;
          local_128 = local_288;
          pppuStack_120 = pppuStack_280;
          local_198 = (ulonglong *)0x3e99999a00000000;
          local_190 = 0;
          local_e5 = 1;
          ::utils::IsoTransform::from_translation();
          local_e5 = 0;
          country_core::systems::collision::world::RaycastWorld::
          upsert_collider<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,utils::IsoTransform>
                    ();
          *local_168 = local_100._0_4_;
          local_238 = (float)*local_f0;
          fStack_234 = (float)((ulonglong)*local_f0 >> 0x20);
          local_e1 = 0;
          fStack_230 = fVar40;
          country_core::resources::stairs::structural_graph::StairsStructuralGraph::upsert_span();
          bVar45 = true;
          pcVar25 = local_138;
          pppuVar41 = local_160;
        } while (ppuVar35 < ppuVar32);
      }
      goto LAB_1407d2619;
    }
LAB_1407d3080:
    fVar40 = 0.0;
    if (local_240 != (ulonglong ***)0x0) {
      fVar40 = (float)((int)local_240 + -1);
    }
    lVar24 = local_2a8;
    lVar10 = local_2a0;
    if (fVar40 != 0.0) {
      *local_168 = local_100._0_4_;
      hashbrown::map::HashMap::
      insert<country_core::resources::stairs::stairs_state::StairGraphEdge,u32,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      fVar43 = 0.0;
      while (lVar24 = local_2a8, lVar10 = local_2a0, (uint)fVar43 < (uint)fVar40) {
        *local_168 = local_100._0_4_;
        local_238 = *(float *)(lVar31 + 0x128);
        fStack_234 = *(float *)(lVar31 + 300);
        local_378 = *(ulonglong ****)(lVar31 + 0x128);
        pcStack_370 = (code *)CONCAT44(pcStack_370._4_4_,fVar43);
        fVar43 = (float)((int)fVar43 + 1);
        fStack_230 = fVar43;
        country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_span()
        ;
      }
    }
  }
LAB_1407d314d:
  puffin::are_scopes_on();
  if (extraout_AL_02 != '\0') {
    ppuVar33 = &PTR_DAT_142a87f40;
    do {
      bVar46 = *(byte *)((longlong)ppuVar33 + -1);
      uVar23 = (uint)(char)bVar46;
      if ((char)bVar46 < '\0') {
        bVar7 = *(byte *)((longlong)ppuVar33 + -2);
        if ((char)bVar7 < -0x40) {
          bVar8 = *(byte *)((longlong)ppuVar33 + -3);
          if ((char)bVar8 < -0x40) {
            pbVar3 = (byte *)((longlong)ppuVar33 + -4);
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
            uVar23 = bVar8 & 0x3f | (*pbVar3 & 7) << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
            uVar23 = bVar8 & 0xf;
          }
          uVar23 = bVar7 & 0x3f | uVar23 << 6;
        }
        else {
          ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
          uVar23 = bVar7 & 0x1f;
        }
        uVar23 = (uint)(bVar46 & 0x3f) | uVar23 << 6;
        if (uVar23 != 0x5c) goto LAB_1407d31e4;
LAB_1407d31f5:
        if (ppuVar33 == (undefined **)&UNK_142a87eff) break;
        if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar33 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
      if (uVar23 == 0x5c) goto LAB_1407d31f5;
LAB_1407d31e4:
      if (uVar23 == 0x2f) goto LAB_1407d31f5;
    } while (ppuVar33 != (undefined **)&DAT_142a87f00);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar28 = extraout_RAX_07;
      if (extraout_RAX_07 == (longlong *)0x0) goto LAB_1407d3c84;
    }
    else {
      if ((int)*extraout_RAX_06 != 1) {
LAB_1407d3c84:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar28 = extraout_RAX_06 + 1;
    }
    if (*plVar28 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar28 = -1;
    local_f0 = plVar28;
    puffin::ThreadProfiler::begin_scope();
    *local_f0 = *local_f0 + 1;
    local_130 = (ulonglong *)extraout_RAX_08;
  }
  local_138 = (char *)CONCAT71(local_138._1_7_,extraout_AL_02);
  local_238 = 0.0;
  fStack_234 = 0.0;
  fStack_230 = 5.60519e-45;
  uStack_22c = 0;
  local_228 = (ulonglong ****)0x0;
  country_core::resources::stairs::stairs_state::StairsState::graph();
  local_140 = *(ulonglong ****)(extraout_RAX_09 + 8) + *(longlong *)(extraout_RAX_09 + 0x10) * 5;
  local_f0 = (longlong *)(lVar11 + 0x1a0);
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  cVar47 = (char)((uint)iVar16 >> 8);
  cVar51 = (char)((uint)iVar16 >> 0x10);
  cVar53 = (char)((uint)iVar16 >> 0x18);
  iVar17 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  cVar55 = (char)((uint)iVar17 >> 8);
  cVar70 = (char)((uint)iVar17 >> 0x10);
  cVar71 = (char)((uint)iVar17 >> 0x18);
  iVar18 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  cVar72 = (char)((uint)iVar18 >> 8);
  cVar73 = (char)((uint)iVar18 >> 0x10);
  cVar74 = (char)((uint)iVar18 >> 0x18);
  iVar19 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  cVar75 = (char)((uint)iVar19 >> 8);
  cVar76 = (char)((uint)iVar19 >> 0x10);
  cVar77 = (char)((uint)iVar19 >> 0x18);
  pppuVar41 = *(ulonglong ****)(extraout_RAX_09 + 8);
  piVar26 = (int *)&DAT_00000004;
  while (pppuVar41 != local_140) {
    iVar68 = *(int *)(pppuVar41 + 4);
    local_228 = (ulonglong ****)0x0;
    local_148 = pppuVar41;
    country_core::resources::stairs::stairs_state::StairsState::graph();
    local_378 = (ulonglong ***)CONCAT44(local_378._4_4_,iVar68);
    indexmap::map::IndexMap::
    get<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
              ();
    if ((extraout_RAX_10 == 0) || (*(longlong *)(extraout_RAX_10 + 0x10) == 0)) {
      pppuVar41 = (ulonglong ***)0x0;
    }
    else {
      piVar27 = *(int **)(extraout_RAX_10 + 8);
      piVar5 = piVar27 + *(longlong *)(extraout_RAX_10 + 0x10) * 2;
      pppuVar41 = (ulonglong ***)0x0;
      do {
        iVar9 = *piVar27;
        piVar27 = piVar27 + 2;
        if (pppuVar41 == (ulonglong ***)CONCAT44(fStack_234,local_238)) {
          alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
          piVar26 = (int *)CONCAT44(uStack_22c,fStack_230);
        }
        piVar26[(longlong)pppuVar41] = iVar9;
        pppuVar41 = (ulonglong ***)((longlong)pppuVar41 + 1);
        local_228 = (ulonglong ****)pppuVar41;
      } while (piVar27 != piVar5);
      piVar26 = (int *)CONCAT44(uStack_22c,fStack_230);
    }
    local_148 = local_148 + 5;
    piVar5 = piVar26 + (longlong)pppuVar41;
    local_160 = (ulonglong ***)0x0;
    piVar27 = piVar26;
    local_150 = piVar26;
LAB_1407d345e:
    pppuVar41 = local_148;
    piVar26 = local_150;
    if (piVar27 != piVar5) {
      pppuVar41 = (ulonglong ***)((longlong)local_160 + 1);
      iVar9 = *piVar27;
      local_158 = piVar27 + 1;
      local_160 = pppuVar41;
LAB_1407d3520:
      piVar27 = local_158;
      if (pppuVar41 == (ulonglong ***)0x0) {
        if (piVar26 == piVar5) goto LAB_1407d345e;
      }
      else {
        uVar29 = (longlong)piVar5 - (longlong)piVar26;
        piVar26 = piVar26 + (longlong)pppuVar41;
        if ((ulonglong ***)(uVar29 >> 2) <= pppuVar41) goto LAB_1407d345e;
      }
      iVar69 = *piVar26;
      iVar30 = iVar9;
      country_core::resources::stairs::stairs_state::StairSegment::new();
      local_170 = extraout_EAX;
      iStack_16c = iVar30;
      country_core::resources::stairs::stairs_state::StairSegment::new();
      piVar26 = piVar26 + 1;
      iVar42 = 0;
      iVar30 = 0;
      local_178 = extraout_EAX_00;
      iStack_174 = iVar69;
      if (local_170 == iVar68) {
LAB_1407d3579:
        if (local_178 == iVar68) {
LAB_1407d36e3:
          *local_168 = local_100._0_4_;
          local_128 = (ulonglong ***)CONCAT44(iStack_16c,local_170);
          pppuStack_120 = (ulonglong ***)CONCAT44(pppuStack_120._4_4_,iVar30);
          local_378 = (ulonglong ***)CONCAT44(iStack_174,local_178);
          pcStack_370 = (code *)CONCAT44(pcStack_370._4_4_,iVar42);
          country_core::resources::stairs::structural_graph::StairsStructuralGraph::add_edge_by_span
                    ();
          pppuVar41 = (ulonglong ***)0x0;
        }
        else {
          pppuVar41 = (ulonglong ***)0x0;
          if (*(longlong *)(lVar11 + 0x198) != 0) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                      ();
            lVar10 = *(longlong *)(lVar11 + 0x180);
            bVar46 = (byte)(extraout_RAX_11 >> 0x39);
            auVar50 = ZEXT216(CONCAT11(bVar46,bVar46));
            auVar50 = pshuflw(auVar50,auVar50,0);
            lVar24 = 0;
            uVar29 = extraout_RAX_11;
            while( true ) {
              uVar29 = *(ulonglong *)(lVar11 + 0x188) & uVar29;
              pcVar34 = (char *)(lVar10 + uVar29);
              cVar48 = auVar50[0];
              auVar66[0] = -(*pcVar34 == cVar48);
              cVar52 = auVar50[1];
              auVar66[1] = -(pcVar34[1] == cVar52);
              cVar54 = auVar50[2];
              auVar66[2] = -(pcVar34[2] == cVar54);
              cVar56 = auVar50[3];
              auVar66[3] = -(pcVar34[3] == cVar56);
              auVar66[4] = -(pcVar34[4] == cVar48);
              auVar66[5] = -(pcVar34[5] == cVar52);
              auVar66[6] = -(pcVar34[6] == cVar54);
              auVar66[7] = -(pcVar34[7] == cVar56);
              auVar66[8] = -(pcVar34[8] == cVar48);
              auVar66[9] = -(pcVar34[9] == cVar52);
              auVar66[10] = -(pcVar34[10] == cVar54);
              auVar66[0xb] = -(pcVar34[0xb] == cVar56);
              auVar66[0xc] = -(pcVar34[0xc] == cVar48);
              auVar66[0xd] = -(pcVar34[0xd] == cVar52);
              auVar66[0xe] = -(pcVar34[0xe] == cVar54);
              auVar66[0xf] = -(pcVar34[0xf] == cVar56);
              uVar36 = (ushort)(SUB161(auVar66 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar66 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar66 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar66 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar66 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar66 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar66 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar66 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar66 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar66 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar66 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar66 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar66 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar66 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar66 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar66[0xf] >> 7) << 0xf;
              uVar23 = (uint)uVar36;
              while (uVar36 != 0) {
                uVar12 = 0;
                for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
                  uVar12 = uVar12 + 1;
                }
                uVar37 = uVar12 + uVar29 & *(ulonglong *)(lVar11 + 0x188);
                uVar6 = *(undefined8 *)(lVar10 + -0xc + uVar37 * -0xc);
                auVar67._0_4_ = -(uint)((int)uVar6 == local_178);
                iVar69 = -(uint)((int)((ulonglong)uVar6 >> 0x20) == iStack_174);
                auVar67._4_4_ = auVar67._0_4_;
                auVar67._8_4_ = iVar69;
                auVar67._12_4_ = iVar69;
                iVar42 = movmskpd(iVar42,auVar67);
                if (iVar42 == 3) {
                  iVar69 = *(int *)(lVar10 + -4 + uVar37 * -0xc);
                  iVar42 = iVar69 + -1;
                  if (iVar69 == 0) {
                    // WARNING: Subroutine does not return
                    core::option::unwrap_failed();
                  }
                  goto LAB_1407d36e3;
                }
                uVar36 = (ushort)(uVar23 - 1) & (ushort)uVar23;
                uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar36);
              }
              auVar62[0] = -(*pcVar34 == (char)iVar16);
              auVar62[1] = -(pcVar34[1] == cVar47);
              auVar62[2] = -(pcVar34[2] == cVar51);
              auVar62[3] = -(pcVar34[3] == cVar53);
              auVar62[4] = -(pcVar34[4] == (char)iVar17);
              auVar62[5] = -(pcVar34[5] == cVar55);
              auVar62[6] = -(pcVar34[6] == cVar70);
              auVar62[7] = -(pcVar34[7] == cVar71);
              auVar62[8] = -(pcVar34[8] == (char)iVar18);
              auVar62[9] = -(pcVar34[9] == cVar72);
              auVar62[10] = -(pcVar34[10] == cVar73);
              auVar62[0xb] = -(pcVar34[0xb] == cVar74);
              auVar62[0xc] = -(pcVar34[0xc] == (char)iVar19);
              auVar62[0xd] = -(pcVar34[0xd] == cVar75);
              auVar62[0xe] = -(pcVar34[0xe] == cVar76);
              auVar62[0xf] = -(pcVar34[0xf] == cVar77);
              if ((((((((((((((((SUB161(auVar62 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar62 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar62 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar62 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar62 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar62 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar62 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar62 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar62 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar62 >> 0x4f,0) & 1) != 0) || (SUB161(auVar62 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar62 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar62 >> 0x67,0) & 1) != 0) || (SUB161(auVar62 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar62 >> 0x77,0) & 1) != 0) || auVar62[0xf] < '\0') break;
              uVar29 = uVar29 + lVar24 + 0x10;
              lVar24 = lVar24 + 0x10;
            }
            pppuVar41 = (ulonglong ***)0x0;
          }
        }
      }
      else {
        pppuVar41 = (ulonglong ***)0x0;
        if (*(longlong *)(lVar11 + 0x198) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                    ();
          lVar10 = *(longlong *)(lVar11 + 0x180);
          bVar46 = (byte)(extraout_RAX_12 >> 0x39);
          auVar50 = ZEXT216(CONCAT11(bVar46,bVar46));
          auVar50 = pshuflw(auVar50,auVar50,0);
          lVar24 = 0;
          uVar29 = extraout_RAX_12;
          while( true ) {
            uVar29 = *(ulonglong *)(lVar11 + 0x188) & uVar29;
            pcVar34 = (char *)(lVar10 + uVar29);
            cVar48 = auVar50[0];
            auVar64[0] = -(*pcVar34 == cVar48);
            cVar52 = auVar50[1];
            auVar64[1] = -(pcVar34[1] == cVar52);
            cVar54 = auVar50[2];
            auVar64[2] = -(pcVar34[2] == cVar54);
            cVar56 = auVar50[3];
            auVar64[3] = -(pcVar34[3] == cVar56);
            auVar64[4] = -(pcVar34[4] == cVar48);
            auVar64[5] = -(pcVar34[5] == cVar52);
            auVar64[6] = -(pcVar34[6] == cVar54);
            auVar64[7] = -(pcVar34[7] == cVar56);
            auVar64[8] = -(pcVar34[8] == cVar48);
            auVar64[9] = -(pcVar34[9] == cVar52);
            auVar64[10] = -(pcVar34[10] == cVar54);
            auVar64[0xb] = -(pcVar34[0xb] == cVar56);
            auVar64[0xc] = -(pcVar34[0xc] == cVar48);
            auVar64[0xd] = -(pcVar34[0xd] == cVar52);
            auVar64[0xe] = -(pcVar34[0xe] == cVar54);
            auVar64[0xf] = -(pcVar34[0xf] == cVar56);
            uVar36 = (ushort)(SUB161(auVar64 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar64 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar64 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar64 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar64 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar64 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar64 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar64 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar64 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar64 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar64 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar64 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar64 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar64 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar64 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar64[0xf] >> 7) << 0xf;
            uVar23 = (uint)uVar36;
            while (uVar36 != 0) {
              uVar12 = 0;
              for (uVar13 = uVar23; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
                uVar12 = uVar12 + 1;
              }
              uVar37 = uVar12 + uVar29 & *(ulonglong *)(lVar11 + 0x188);
              uVar6 = *(undefined8 *)(lVar10 + -0xc + uVar37 * -0xc);
              auVar65._0_4_ = -(uint)((int)uVar6 == local_170);
              iVar69 = -(uint)((int)((ulonglong)uVar6 >> 0x20) == iStack_16c);
              auVar65._4_4_ = auVar65._0_4_;
              auVar65._8_4_ = iVar69;
              auVar65._12_4_ = iVar69;
              iVar30 = movmskpd(iVar30,auVar65);
              if (iVar30 == 3) {
                iVar30 = *(int *)(lVar10 + -4 + uVar37 * -0xc);
                if (iVar30 == 0) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                iVar30 = iVar30 + -1;
                goto LAB_1407d3579;
              }
              uVar36 = (ushort)(uVar23 - 1) & (ushort)uVar23;
              uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar36);
            }
            auVar61[0] = -(*pcVar34 == (char)iVar16);
            auVar61[1] = -(pcVar34[1] == cVar47);
            auVar61[2] = -(pcVar34[2] == cVar51);
            auVar61[3] = -(pcVar34[3] == cVar53);
            auVar61[4] = -(pcVar34[4] == (char)iVar17);
            auVar61[5] = -(pcVar34[5] == cVar55);
            auVar61[6] = -(pcVar34[6] == cVar70);
            auVar61[7] = -(pcVar34[7] == cVar71);
            auVar61[8] = -(pcVar34[8] == (char)iVar18);
            auVar61[9] = -(pcVar34[9] == cVar72);
            auVar61[10] = -(pcVar34[10] == cVar73);
            auVar61[0xb] = -(pcVar34[0xb] == cVar74);
            auVar61[0xc] = -(pcVar34[0xc] == (char)iVar19);
            auVar61[0xd] = -(pcVar34[0xd] == cVar75);
            auVar61[0xe] = -(pcVar34[0xe] == cVar76);
            auVar61[0xf] = -(pcVar34[0xf] == cVar77);
            if ((((((((((((((((SUB161(auVar61 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar61 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar61 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar61 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar61 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar61 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar61 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar61 >> 0x3f,0) & 1) != 0) || (SUB161(auVar61 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar61 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar61 >> 0x57,0) & 1) != 0) || (SUB161(auVar61 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar61 >> 0x67,0) & 1) != 0) || (SUB161(auVar61 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar61 >> 0x77,0) & 1) != 0) || auVar61[0xf] < '\0') break;
            uVar29 = uVar29 + lVar24 + 0x10;
            lVar24 = lVar24 + 0x10;
          }
          pppuVar41 = (ulonglong ***)0x0;
        }
      }
      goto LAB_1407d3520;
    }
  }
  if (CONCAT44(fStack_234,local_238) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((char)local_138 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_13 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar28 = extraout_RAX_14;
      if (extraout_RAX_14 == (longlong *)0x0) goto LAB_1407d3c92;
    }
    else {
      if ((int)*extraout_RAX_13 != 1) {
LAB_1407d3c92:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar28 = extraout_RAX_13 + 1;
    }
    if (*plVar28 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar28 = -1;
    local_f0 = plVar28;
    puffin::ThreadProfiler::end_scope();
    *local_f0 = *local_f0 + 1;
  }
  if (local_e3 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_15 == 1) {
      plVar28 = extraout_RAX_15 + 1;
    }
    else if ((*extraout_RAX_15 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar28 = extraout_RAX_17, extraout_RAX_17 == (longlong *)0x0)) {
LAB_1407d38c0:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar28 != 0) {
LAB_1407d3985:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar28 = -1;
    local_f0 = plVar28;
    puffin::ThreadProfiler::end_scope();
    *local_f0 = *local_f0 + 1;
  }
  return;
}


