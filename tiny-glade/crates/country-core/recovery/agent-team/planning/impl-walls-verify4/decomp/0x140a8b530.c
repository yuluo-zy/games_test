
void __cdecl
country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  char *pcVar3;
  size_t sVar4;
  float fVar5;
  float fVar6;
  byte bVar7;
  byte bVar8;
  ulonglong *puVar9;
  ulonglong uVar10;
  undefined8 uVar11;
  void *pvVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  float fVar17;
  char extraout_AL;
  uint uVar18;
  int iVar19;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar20;
  longlong *extraout_RAX_01;
  longlong lVar21;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  ulonglong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  undefined1 (*pauVar22) [16];
  undefined1 *extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  undefined8 uVar23;
  longlong *extraout_RAX_12;
  undefined1 *extraout_RAX_13;
  undefined1 *puVar24;
  undefined1 (*extraout_RAX_14) [16];
  undefined8 *extraout_RAX_15;
  undefined8 *extraout_RAX_16;
  undefined8 *extraout_RAX_17;
  undefined8 *extraout_RAX_18;
  undefined8 *extraout_RAX_19;
  undefined8 *extraout_RAX_20;
  undefined8 *extraout_RAX_21;
  undefined8 *extraout_RAX_22;
  undefined8 *extraout_RAX_23;
  undefined1 *extraout_RAX_24;
  undefined1 *extraout_RAX_25;
  undefined1 *_Dst;
  undefined1 (*_Size) [16];
  longlong *extraout_RAX_26;
  longlong *_Dst_00;
  ulonglong extraout_RAX_27;
  longlong *extraout_RAX_28;
  longlong *extraout_RAX_29;
  longlong *extraout_RAX_30;
  uint uVar25;
  undefined8 *in_RCX;
  uint uVar26;
  undefined8 *in_RDX;
  ulonglong uVar27;
  ulonglong uVar28;
  ulonglong uVar29;
  ulonglong uVar30;
  undefined1 (*pauVar31) [16];
  float *pfVar32;
  longlong lVar33;
  undefined1 (*pauVar34) [16];
  undefined1 (*pauVar35) [16];
  longlong *plVar36;
  longlong lVar37;
  longlong *in_R8;
  longlong lVar38;
  undefined8 *in_R9;
  longlong lVar39;
  undefined4 uVar40;
  ulonglong uVar41;
  ushort uVar43;
  ulonglong uVar44;
  undefined1 (*pauVar45) [16];
  bool bVar46;
  byte bVar47;
  char cVar48;
  char cVar52;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  char cVar50;
  char cVar51;
  undefined1 auVar49 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  undefined1 auVar58 [12];
  undefined1 auVar59 [16];
  undefined1 auVar60 [16];
  undefined1 auVar61 [16];
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  float fVar64;
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
  float fVar65;
  float fVar66;
  float fVar67;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong in_stack_00000050;
  undefined8 *in_stack_00000058;
  undefined8 *in_stack_00000060;
  longlong *in_stack_00000068;
  longlong *in_stack_00000070;
  longlong local_e88 [62];
  byte local_c98;
  undefined4 local_c97;
  undefined8 local_c90;
  ulonglong local_c88;
  undefined8 local_c80;
  undefined8 uStack_c78;
  undefined1 (*local_c70) [16];
  longlong *plStack_c68;
  undefined1 (*local_c60) [16];
  longlong lStack_c58;
  longlong local_c50;
  longlong lStack_c48;
  undefined8 local_c40;
  undefined4 local_c38;
  undefined4 local_c34;
  undefined1 (*local_be8) [16];
  longlong *plStack_be0;
  undefined1 (*local_bd8) [16];
  undefined1 (**ppauStack_bd0) [16];
  undefined8 local_bc8;
  undefined8 uStack_bc0;
  undefined8 local_bb8;
  ulonglong local_948;
  undefined1 *local_940;
  ulonglong local_938;
  undefined1 *local_930;
  undefined1 *local_928;
  undefined1 *local_920;
  undefined4 local_918;
  undefined1 (*local_910) [16];
  longlong *plStack_908;
  undefined1 (*local_900) [16];
  undefined1 (**ppauStack_8f8) [16];
  undefined8 local_8f0;
  undefined8 uStack_8e8;
  undefined8 local_8e0;
  undefined1 (*local_8d8) [16];
  undefined1 (*local_8d0) [16];
  undefined1 (*local_8c8) [16];
  undefined8 local_8c0;
  undefined8 local_8b8;
  undefined8 local_8b0;
  undefined8 uStack_8a8;
  undefined8 local_8a0;
  undefined8 local_898;
  undefined4 local_890;
  undefined8 local_888;
  undefined8 local_880;
  undefined8 local_878;
  undefined8 local_870;
  undefined8 local_868;
  undefined8 local_860;
  undefined8 uStack_858;
  undefined8 local_850;
  undefined8 local_848;
  undefined4 local_840;
  undefined4 local_838;
  undefined8 *local_830;
  undefined8 *local_828;
  longlong local_820;
  undefined8 *local_818;
  longlong local_810;
  undefined8 *local_808;
  longlong local_800;
  undefined8 *local_7f8;
  longlong local_7f0;
  undefined8 *local_7e8;
  longlong local_7e0;
  undefined8 *local_7d8;
  longlong local_7d0;
  undefined8 *local_7c8;
  undefined1 local_7c0;
  undefined4 uStack_7bf;
  undefined8 local_7b8;
  ulonglong local_7b0;
  undefined8 local_7a8;
  undefined8 uStack_7a0;
  undefined1 (*local_798) [16];
  longlong *plStack_790;
  undefined1 (*local_788) [16];
  longlong lStack_780;
  longlong local_778;
  longlong lStack_770;
  undefined8 local_768;
  undefined4 local_760;
  undefined4 local_75c;
  undefined1 local_758;
  undefined8 local_750;
  undefined8 local_748;
  undefined8 local_740;
  undefined8 uStack_738;
  undefined8 local_730;
  undefined8 uStack_728;
  undefined8 local_720;
  undefined8 uStack_718;
  undefined8 local_710;
  undefined8 uStack_708;
  undefined8 local_700;
  undefined4 local_6f8;
  int local_6f0;
  undefined *local_6ec;
  undefined8 uStack_6e4;
  undefined8 local_6dc;
  float local_6d4;
  float local_6d0;
  float local_6cc;
  undefined4 local_6c8;
  undefined4 local_6c4;
  undefined4 local_6c0;
  undefined4 local_6bc;
  undefined4 local_6b8;
  undefined1 local_6b4;
  undefined1 local_6b3;
  char local_6b2;
  undefined1 local_6b1;
  undefined1 local_6b0;
  undefined2 local_6af;
  undefined8 local_6a8;
  undefined8 uStack_6a0;
  undefined8 local_698;
  undefined8 uStack_690;
  undefined1 (*local_688) [16];
  longlong *plStack_680;
  undefined1 (*local_678) [16];
  longlong lStack_670;
  longlong local_668;
  longlong lStack_660;
  undefined8 local_658;
  undefined1 (*local_648) [16];
  longlong *plStack_640;
  undefined1 (*local_638) [16];
  longlong lStack_630;
  longlong local_628;
  longlong lStack_620;
  undefined8 local_618;
  undefined1 (*local_608) [16];
  longlong *plStack_600;
  undefined1 (*local_5f8) [16];
  ulonglong uStack_5f0;
  undefined1 *local_5e8;
  ulonglong uStack_5e0;
  undefined8 local_5d8;
  longlong local_5d0;
  undefined4 local_5c8;
  uint uStack_5c4;
  longlong local_5b8;
  longlong local_598;
  longlong local_580;
  longlong local_568;
  longlong local_548;
  longlong local_530;
  longlong local_518;
  longlong local_478;
  longlong local_360;
  longlong local_350;
  longlong local_340;
  longlong local_330;
  longlong local_320;
  longlong local_310;
  undefined8 *local_300;
  undefined1 (*local_2f0) [16];
  longlong local_2e8;
  longlong local_2e0;
  ulonglong local_2d8;
  undefined8 local_2d0;
  undefined1 (*local_2c8) [16];
  longlong local_2c0;
  undefined8 *local_2b8;
  longlong local_2b0;
  undefined8 *local_2a8;
  longlong local_2a0;
  undefined8 *local_298;
  longlong local_290;
  undefined8 *local_288;
  undefined8 *local_280;
  longlong local_278;
  undefined8 *local_270;
  longlong local_268;
  undefined8 *local_260;
  undefined8 local_258;
  undefined8 uStack_250;
  longlong local_248;
  longlong local_238;
  undefined8 local_230;
  undefined1 *local_228;
  undefined8 local_220;
  undefined8 local_218;
  ulonglong local_210;
  undefined *local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined4 local_1e8;
  undefined4 local_1e4;
  uint local_1e0;
  undefined4 local_1dc;
  undefined8 local_1d8;
  undefined8 *local_1d0;
  undefined1 *local_1c8;
  undefined4 local_1c0;
  undefined4 local_1b8;
  longlong local_1b0;
  ulonglong local_1a8;
  undefined1 *local_1a0;
  undefined4 uStack_198;
  undefined1 (*local_190) [16];
  longlong *local_188;
  undefined8 local_180;
  ulonglong local_178;
  ulonglong local_170;
  undefined8 local_168;
  undefined1 (*local_160) [16];
  longlong *local_158;
  undefined1 (*local_150) [16];
  undefined8 *local_148;
  longlong *local_140;
  longlong *local_138;
  undefined1 (*local_130) [16];
  undefined1 (*local_128) [16];
  undefined4 local_120;
  byte local_119;
  longlong *local_118;
  undefined1 (*local_110) [16];
  undefined1 (*local_108) [16];
  longlong *local_100;
  undefined1 local_f3;
  undefined1 local_f2;
  undefined8 local_f0;
  undefined7 uVar42;
  
  local_f0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_138 = in_R8;
  if (extraout_AL == '\0') {
    local_248 = 0;
  }
  else {
    uVar27 = 0x5d;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar27 != 0) {
        if (uVar27 < 0x5d) {
          if ((char)(&DAT_142ad8bd1)[uVar27] < -0x40) {
LAB_140a8b601:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar27 != 0x5d) goto LAB_140a8b601;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar27 != 0xfffffffffffffffe)) {
        if (uVar27 + 2 < 0x5d) {
          if ((char)(&UNK_142ad8bd3)[uVar27] < -0x40) {
LAB_140a8b65f:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar27 + 2 != 0x5d) goto LAB_140a8b65f;
      }
    }
    puVar20 = &DAT_142ad92b4;
    do {
      bVar47 = puVar20[-1];
      uVar25 = (uint)(char)bVar47;
      if ((char)bVar47 < '\0') {
        bVar7 = puVar20[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar20[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = puVar20 + -4;
            puVar20 = puVar20 + -4;
            uVar25 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar20 = puVar20 + -3;
            uVar25 = bVar8 & 0xf;
          }
          uVar25 = bVar7 & 0x3f | uVar25 << 6;
        }
        else {
          puVar20 = puVar20 + -2;
          uVar25 = bVar7 & 0x1f;
        }
        uVar25 = (uint)(bVar47 & 0x3f) | uVar25 << 6;
        if (uVar25 != 0x5c) goto LAB_140a8b729;
LAB_140a8b73e:
        if (puVar20 == &UNK_142ad9267) break;
        if (puVar20 + -0x142ad9267 < (undefined *)0x4c) {
          if (-0x41 < (char)puVar20[1]) break;
        }
        else if (puVar20 + -0x142ad9267 == (undefined *)0x4c) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar20 = puVar20 + -1;
      if (uVar25 == 0x5c) goto LAB_140a8b73e;
LAB_140a8b729:
      if (uVar25 == 0x2f) goto LAB_140a8b73e;
    } while (puVar20 != &DAT_142ad9268);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar36 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar36 = extraout_RAX_28, extraout_RAX_28 == (longlong *)0x0)) {
LAB_140a8ddaf:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar36 != 0) {
LAB_140a8df97:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar36 = -1;
    local_100 = plVar36;
    puffin::ThreadProfiler::begin_scope();
    *local_100 = *local_100 + 1;
    local_248 = 1;
  }
  *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
  local_150 = (undefined1 (*) [16])*in_RDX;
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_100 = (longlong *)*in_RCX;
  puVar9 = (ulonglong *)*in_stack_00000048;
  lVar37 = in_stack_00000048[1];
  uVar27 = *puVar9;
  uVar29 = 0;
  uVar30 = uVar27 - *(ulonglong *)(lVar37 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar37 + 0x18)) {
    uVar30 = uVar29;
  }
  uVar28 = uVar27 - *(ulonglong *)(lVar37 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar37 + 0x38)) {
    uVar28 = uVar29;
  }
  uVar27 = *(ulonglong *)(lVar37 + 0x30);
  uVar41 = uVar27 - uVar28;
  if (uVar28 > uVar27) {
    uVar41 = uVar29;
  }
  uVar10 = *(ulonglong *)(lVar37 + 0x10);
  uVar44 = uVar10 - uVar30;
  if (uVar30 > uVar10) {
    uVar44 = uVar29;
  }
  uVar29 = *(ulonglong *)(lVar37 + 0x40);
  *puVar9 = uVar29 - (uVar44 + uVar41);
  local_148 = in_R9;
  if (uVar30 <= uVar10 && uVar10 - uVar30 != 0 || uVar28 <= uVar27 && uVar27 - uVar28 != 0) {
    *puVar9 = uVar29;
    pauVar35 = *(undefined1 (**) [16])*local_150;
    local_188 = *(longlong **)((longlong)local_150[1] + 8);
    auVar49 = *pauVar35;
    uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                            (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
    pauVar34 = pauVar35 + 1;
    local_168 = CONCAT44(local_168._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x1c));
    lVar37 = *in_stack_00000030;
    local_160 = (undefined1 (*) [16])in_stack_00000030[2];
    local_118 = (longlong *)(lVar37 + 0x20);
    while (local_188 != (longlong *)0x0) {
      if ((short)uVar25 == 0) {
        do {
          auVar49 = *pauVar34;
          uVar43 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                   (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
          pauVar35 = pauVar35 + -0x2a8;
          pauVar34 = pauVar34 + 1;
        } while (uVar43 == 0xffff);
        uVar18 = ~(uint)uVar43;
        uVar25 = uVar18 & -(uint)uVar43 - 2;
      }
      else {
        uVar18 = uVar25;
        uVar25 = uVar25 - 1 & uVar25;
      }
      local_188 = (longlong *)((longlong)local_188 + -1);
      uVar26 = 0;
      for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
        uVar26 = uVar26 + 1;
      }
      if (1 < *(byte *)((longlong)pauVar35 + (ulonglong)uVar26 * -0x2a8 + -7)) {
        uVar23 = *(undefined8 *)((longlong)pauVar35 + (ulonglong)uVar26 * -0x2a8 + -0x2a8);
        *(undefined4 *)local_160 = (undefined4)local_168;
        lVar38 = *(longlong *)(lVar37 + 0x30);
        uVar11 = *(undefined8 *)(lVar37 + 0x40);
        if (lVar38 == *(longlong *)(lVar37 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar33 = *(longlong *)(lVar37 + 0x28);
        *(undefined8 *)(lVar33 + lVar38 * 0x10) = uVar11;
        *(undefined8 *)(lVar33 + 8 + lVar38 * 0x10) = uVar23;
        *(longlong *)(lVar37 + 0x30) = lVar38 + 1;
        *(longlong *)(lVar37 + 0x40) = *(longlong *)(lVar37 + 0x40) + 1;
      }
    }
  }
  else {
    local_168 = CONCAT44(local_168._4_4_,*(undefined4 *)((longlong)in_stack_00000030 + 0x1c));
    lVar37 = *in_stack_00000030;
    local_160 = (undefined1 (*) [16])in_stack_00000030[2];
  }
  puVar9 = (ulonglong *)*in_stack_00000038;
  lVar38 = in_stack_00000038[1];
  uVar27 = *puVar9;
  uVar29 = 0;
  uVar30 = uVar27 - *(ulonglong *)(lVar38 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar38 + 0x18)) {
    uVar30 = uVar29;
  }
  uVar28 = uVar27 - *(ulonglong *)(lVar38 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar38 + 0x38)) {
    uVar28 = uVar29;
  }
  lVar33 = uVar30 * 0x10 + *(longlong *)(lVar38 + 8);
  uVar27 = *(ulonglong *)(lVar38 + 0x10) - uVar30;
  if (*(ulonglong *)(lVar38 + 0x10) < uVar30) {
    lVar33 = 8;
    uVar27 = uVar29;
  }
  lVar21 = uVar28 * 0x10 + *(longlong *)(lVar38 + 0x28);
  uVar30 = *(ulonglong *)(lVar38 + 0x30) - uVar28;
  if (*(ulonglong *)(lVar38 + 0x30) < uVar28) {
    lVar21 = 8;
    uVar30 = uVar29;
  }
  *puVar9 = (*(longlong *)(lVar38 + 0x40) - uVar30) - uVar27;
  local_118 = (longlong *)(uVar27 * 0x10 + lVar33);
  local_158 = (longlong *)(uVar30 * 0x10 + lVar21);
  local_188 = local_100 + 4;
  iVar19 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar16 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  local_128 = (undefined1 (*) [16])(lVar37 + 0x20);
LAB_140a8b985:
  do {
    if ((lVar33 == 0) || ((longlong *)lVar33 == local_118)) {
      if ((lVar21 == 0) || ((longlong *)lVar21 == local_158)) break;
      lVar33 = 0;
      lVar38 = lVar21;
      lVar21 = lVar21 + 0x10;
    }
    else {
      lVar38 = lVar33;
      lVar33 = lVar33 + 0x10;
    }
    *puVar9 = *puVar9 + 1;
    pauVar34 = *(undefined1 (**) [16])(lVar38 + 8);
    if (local_100[3] != 0) {
      local_608 = pauVar34;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar47 = (byte)(extraout_RAX_02 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
      auVar49 = pshuflw(auVar49,auVar49,0);
      lVar39 = 0;
      uVar27 = extraout_RAX_02;
      while( true ) {
        uVar27 = local_100[1] & uVar27;
        pcVar3 = (char *)(*local_100 + uVar27);
        cVar48 = auVar49[0];
        auVar59[0] = -(*pcVar3 == cVar48);
        cVar50 = auVar49[1];
        auVar59[1] = -(pcVar3[1] == cVar50);
        cVar51 = auVar49[2];
        auVar59[2] = -(pcVar3[2] == cVar51);
        cVar52 = auVar49[3];
        auVar59[3] = -(pcVar3[3] == cVar52);
        auVar59[4] = -(pcVar3[4] == cVar48);
        auVar59[5] = -(pcVar3[5] == cVar50);
        auVar59[6] = -(pcVar3[6] == cVar51);
        auVar59[7] = -(pcVar3[7] == cVar52);
        auVar59[8] = -(pcVar3[8] == cVar48);
        auVar59[9] = -(pcVar3[9] == cVar50);
        auVar59[10] = -(pcVar3[10] == cVar51);
        auVar59[0xb] = -(pcVar3[0xb] == cVar52);
        auVar59[0xc] = -(pcVar3[0xc] == cVar48);
        auVar59[0xd] = -(pcVar3[0xd] == cVar50);
        auVar59[0xe] = -(pcVar3[0xe] == cVar51);
        auVar59[0xf] = -(pcVar3[0xf] == cVar52);
        uVar43 = (ushort)(SUB161(auVar59 >> 7,0) & 1) | (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe | (ushort)(auVar59[0xf] >> 7) << 0xf
        ;
        uVar25 = (uint)uVar43;
        while (uVar43 != 0) {
          uVar18 = 0;
          for (uVar26 = uVar25; (uVar26 & 1) == 0; uVar26 = uVar26 >> 1 | 0x80000000) {
            uVar18 = uVar18 + 1;
          }
          if (*(undefined1 (**) [16])(*local_100 + -0x78 + (uVar18 + uVar27 & local_100[1]) * -0x78)
              == pauVar34) {
            uVar23 = *(undefined8 *)(lVar38 + 8);
            *(undefined4 *)local_160 = (undefined4)local_168;
            lVar38 = *(longlong *)(lVar37 + 0x30);
            uVar11 = *(undefined8 *)(lVar37 + 0x40);
            if (lVar38 == *(longlong *)(lVar37 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
            lVar39 = *(longlong *)(lVar37 + 0x28);
            *(undefined8 *)(lVar39 + lVar38 * 0x10) = uVar11;
            *(undefined8 *)(lVar39 + 8 + lVar38 * 0x10) = uVar23;
            *(longlong *)(lVar37 + 0x30) = lVar38 + 1;
            *(longlong *)(lVar37 + 0x40) = *(longlong *)(lVar37 + 0x40) + 1;
            goto LAB_140a8b985;
          }
          uVar43 = (ushort)(uVar25 - 1) & (ushort)uVar25;
          uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar43);
        }
        auVar53[0] = -(*pcVar3 == (char)iVar19);
        auVar53[1] = -(pcVar3[1] == (char)((uint)iVar19 >> 8));
        auVar53[2] = -(pcVar3[2] == (char)((uint)iVar19 >> 0x10));
        auVar53[3] = -(pcVar3[3] == (char)((uint)iVar19 >> 0x18));
        auVar53[4] = -(pcVar3[4] == (char)iVar14);
        auVar53[5] = -(pcVar3[5] == (char)((uint)iVar14 >> 8));
        auVar53[6] = -(pcVar3[6] == (char)((uint)iVar14 >> 0x10));
        auVar53[7] = -(pcVar3[7] == (char)((uint)iVar14 >> 0x18));
        auVar53[8] = -(pcVar3[8] == (char)iVar15);
        auVar53[9] = -(pcVar3[9] == (char)((uint)iVar15 >> 8));
        auVar53[10] = -(pcVar3[10] == (char)((uint)iVar15 >> 0x10));
        auVar53[0xb] = -(pcVar3[0xb] == (char)((uint)iVar15 >> 0x18));
        auVar53[0xc] = -(pcVar3[0xc] == (char)iVar16);
        auVar53[0xd] = -(pcVar3[0xd] == (char)((uint)iVar16 >> 8));
        auVar53[0xe] = -(pcVar3[0xe] == (char)((uint)iVar16 >> 0x10));
        auVar53[0xf] = -(pcVar3[0xf] == (char)((uint)iVar16 >> 0x18));
        if ((((((((((((((((SUB161(auVar53 >> 7,0) & 1) != 0 || (SUB161(auVar53 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar53 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar53 >> 0x1f,0) & 1) != 0) || (SUB161(auVar53 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar53 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar53 >> 0x37,0) & 1) != 0) || (SUB161(auVar53 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar53 >> 0x47,0) & 1) != 0) || (SUB161(auVar53 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar53 >> 0x57,0) & 1) != 0) || (SUB161(auVar53 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar53 >> 0x67,0) & 1) != 0) || (SUB161(auVar53 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar53 >> 0x77,0) & 1) != 0) || auVar53[0xf] < '\0') break;
        uVar27 = uVar27 + lVar39 + 0x10;
        lVar39 = lVar39 + 0x10;
      }
    }
  } while( true );
  puVar9 = (ulonglong *)*in_stack_00000040;
  lVar38 = in_stack_00000040[1];
  uVar27 = *puVar9;
  uVar29 = 0;
  uVar30 = uVar27 - *(ulonglong *)(lVar38 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar38 + 0x18)) {
    uVar30 = uVar29;
  }
  uVar28 = uVar27 - *(ulonglong *)(lVar38 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar38 + 0x38)) {
    uVar28 = uVar29;
  }
  lVar33 = uVar30 * 0x10 + *(longlong *)(lVar38 + 8);
  uVar27 = *(ulonglong *)(lVar38 + 0x10) - uVar30;
  if (*(ulonglong *)(lVar38 + 0x10) < uVar30) {
    lVar33 = 8;
    uVar27 = uVar29;
  }
  lVar21 = uVar28 * 0x10 + *(longlong *)(lVar38 + 0x28);
  uVar30 = *(ulonglong *)(lVar38 + 0x30) - uVar28;
  if (*(ulonglong *)(lVar38 + 0x30) < uVar28) {
    lVar21 = 8;
    uVar30 = uVar29;
  }
  *puVar9 = (*(longlong *)(lVar38 + 0x40) - uVar30) - uVar27;
  local_118 = (longlong *)(uVar27 * 0x10 + lVar33);
  local_158 = (longlong *)(uVar30 * 0x10 + lVar21);
LAB_140a8bb85:
  do {
    if ((lVar33 == 0) || ((longlong *)lVar33 == local_118)) {
      if ((lVar21 == 0) || ((longlong *)lVar21 == local_158)) break;
      lVar33 = 0;
      lVar38 = lVar21;
      lVar21 = lVar21 + 0x10;
    }
    else {
      lVar38 = lVar33;
      lVar33 = lVar33 + 0x10;
    }
    *puVar9 = *puVar9 + 1;
    pauVar34 = *(undefined1 (**) [16])(lVar38 + 8);
    if (local_100[3] != 0) {
      local_608 = pauVar34;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar47 = (byte)(extraout_RAX_03 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
      auVar49 = pshuflw(auVar49,auVar49,0);
      lVar39 = 0;
      uVar27 = extraout_RAX_03;
      while( true ) {
        uVar27 = local_100[1] & uVar27;
        pcVar3 = (char *)(*local_100 + uVar27);
        cVar48 = auVar49[0];
        auVar60[0] = -(*pcVar3 == cVar48);
        cVar50 = auVar49[1];
        auVar60[1] = -(pcVar3[1] == cVar50);
        cVar51 = auVar49[2];
        auVar60[2] = -(pcVar3[2] == cVar51);
        cVar52 = auVar49[3];
        auVar60[3] = -(pcVar3[3] == cVar52);
        auVar60[4] = -(pcVar3[4] == cVar48);
        auVar60[5] = -(pcVar3[5] == cVar50);
        auVar60[6] = -(pcVar3[6] == cVar51);
        auVar60[7] = -(pcVar3[7] == cVar52);
        auVar60[8] = -(pcVar3[8] == cVar48);
        auVar60[9] = -(pcVar3[9] == cVar50);
        auVar60[10] = -(pcVar3[10] == cVar51);
        auVar60[0xb] = -(pcVar3[0xb] == cVar52);
        auVar60[0xc] = -(pcVar3[0xc] == cVar48);
        auVar60[0xd] = -(pcVar3[0xd] == cVar50);
        auVar60[0xe] = -(pcVar3[0xe] == cVar51);
        auVar60[0xf] = -(pcVar3[0xf] == cVar52);
        uVar43 = (ushort)(SUB161(auVar60 >> 7,0) & 1) | (ushort)(SUB161(auVar60 >> 0xf,0) & 1) << 1
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
        uVar25 = (uint)uVar43;
        while (uVar43 != 0) {
          uVar18 = 0;
          for (uVar26 = uVar25; (uVar26 & 1) == 0; uVar26 = uVar26 >> 1 | 0x80000000) {
            uVar18 = uVar18 + 1;
          }
          if (*(undefined1 (**) [16])(*local_100 + -0x78 + (uVar18 + uVar27 & local_100[1]) * -0x78)
              == pauVar34) {
            uVar23 = *(undefined8 *)(lVar38 + 8);
            *(undefined4 *)local_160 = (undefined4)local_168;
            lVar38 = *(longlong *)(lVar37 + 0x30);
            uVar11 = *(undefined8 *)(lVar37 + 0x40);
            if (lVar38 == *(longlong *)(lVar37 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
            lVar39 = *(longlong *)(lVar37 + 0x28);
            *(undefined8 *)(lVar39 + lVar38 * 0x10) = uVar11;
            *(undefined8 *)(lVar39 + 8 + lVar38 * 0x10) = uVar23;
            *(longlong *)(lVar37 + 0x30) = lVar38 + 1;
            *(longlong *)(lVar37 + 0x40) = *(longlong *)(lVar37 + 0x40) + 1;
            goto LAB_140a8bb85;
          }
          uVar43 = (ushort)(uVar25 - 1) & (ushort)uVar25;
          uVar25 = CONCAT22((short)(uVar25 - 1 >> 0x10),uVar43);
        }
        auVar54[0] = -(*pcVar3 == -1);
        auVar54[1] = -(pcVar3[1] == -1);
        auVar54[2] = -(pcVar3[2] == -1);
        auVar54[3] = -(pcVar3[3] == -1);
        auVar54[4] = -(pcVar3[4] == -1);
        auVar54[5] = -(pcVar3[5] == -1);
        auVar54[6] = -(pcVar3[6] == -1);
        auVar54[7] = -(pcVar3[7] == -1);
        auVar54[8] = -(pcVar3[8] == -1);
        auVar54[9] = -(pcVar3[9] == -1);
        auVar54[10] = -(pcVar3[10] == -1);
        auVar54[0xb] = -(pcVar3[0xb] == -1);
        auVar54[0xc] = -(pcVar3[0xc] == -1);
        auVar54[0xd] = -(pcVar3[0xd] == -1);
        auVar54[0xe] = -(pcVar3[0xe] == -1);
        auVar54[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar54 >> 7,0) & 1) != 0 || (SUB161(auVar54 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar54 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar54 >> 0x1f,0) & 1) != 0) || (SUB161(auVar54 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar54 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar54 >> 0x37,0) & 1) != 0) || (SUB161(auVar54 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar54 >> 0x47,0) & 1) != 0) || (SUB161(auVar54 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar54 >> 0x57,0) & 1) != 0) || (SUB161(auVar54 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar54 >> 0x67,0) & 1) != 0) || (SUB161(auVar54 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar54 >> 0x77,0) & 1) != 0) || auVar54[0xf] < '\0') break;
        uVar27 = uVar27 + lVar39 + 0x10;
        lVar39 = lVar39 + 0x10;
      }
    }
  } while( true );
  pauVar34 = (undefined1 (*) [16])*local_138;
  local_168 = *local_148;
  **(undefined4 **)(in_stack_00000050 + 0x10) = *(undefined4 *)(in_stack_00000050 + 0x1c);
  plStack_600 = local_100;
  local_5f8 = local_150;
  local_608 = pauVar34;
  country_core::utils::dirty_tiles::impl$3::
  iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env_0>
            ();
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  plVar36 = local_100;
  pauVar35 = local_150;
  if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_5e8 = (undefined1 *)*extraout_RAX_04;
  uStack_5e0 = extraout_RAX_04[1];
  *extraout_RAX_04 = *extraout_RAX_04 + 1;
  local_608 = (undefined1 (*) [16])
              &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
  plStack_600 = (longlong *)0x0;
  local_5f8 = (undefined1 (*) [16])0x0;
  uStack_5f0 = 0;
  plStack_be0 = local_100;
  local_bd8 = local_150;
  local_f3 = 1;
  local_be8 = pauVar34;
  ppauStack_bd0 = &local_608;
  country_core::utils::dirty_tiles::impl$3::
  iter_dirty_tiles<country_core::systems::wall::wall_state::publish_public_wall_state::gather_lazy_update_sources::closure_env_1>
            ();
  auVar49 = *local_608;
  local_118 = plStack_600;
  if (plStack_600 == (longlong *)0x0) {
    local_138 = (longlong *)0x0;
  }
  else {
    uVar27 = (longlong)plStack_600 * 8 + 0x17U & 0xfffffffffffffff0;
    local_128 = (undefined1 (*) [16])((longlong)plStack_600 + uVar27 + 0x11);
    local_148 = (undefined8 *)((longlong)local_608 - uVar27);
    local_138 = (longlong *)0x10;
  }
  pauVar34 = local_608 + 1;
  uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
  local_158 = plVar36 + 0xc;
  local_160 = pauVar35 + 2;
  pauVar35 = local_608;
  uVar27 = uStack_5f0;
LAB_140a8c150:
  while (plVar36 = local_100, uVar27 != 0) {
    if ((short)uVar25 == 0) {
      do {
        auVar49 = *pauVar34;
        uVar43 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
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
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
        pauVar35 = pauVar35 + -8;
        pauVar34 = pauVar34 + 1;
      } while (uVar43 == 0xffff);
      uVar18 = ~(uint)uVar43;
      uVar25 = uVar18 & -(uint)uVar43 - 2;
    }
    else {
      uVar18 = uVar25;
      uVar25 = uVar25 - 1 & uVar25;
    }
    uVar27 = uVar27 + -1;
    iVar19 = 0;
    for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
      iVar19 = iVar19 + 1;
    }
    pauVar22 = *(undefined1 (**) [16])((longlong)pauVar35 + (-8 - (ulonglong)(uint)(iVar19 << 3)));
    local_be8 = pauVar22;
    if (*(longlong *)((longlong)local_150[1] + 8) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar37 = *(longlong *)*local_150;
      bVar47 = (byte)(extraout_RAX_06 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
      auVar49 = pshuflw(auVar49,auVar49,0);
      lVar38 = 0;
      uVar30 = extraout_RAX_06;
      while( true ) {
        uVar30 = *(ulonglong *)((longlong)*local_150 + 8) & uVar30;
        pcVar3 = (char *)(lVar37 + uVar30);
        cVar48 = auVar49[0];
        auVar61[0] = -(*pcVar3 == cVar48);
        cVar50 = auVar49[1];
        auVar61[1] = -(pcVar3[1] == cVar50);
        cVar51 = auVar49[2];
        auVar61[2] = -(pcVar3[2] == cVar51);
        cVar52 = auVar49[3];
        auVar61[3] = -(pcVar3[3] == cVar52);
        auVar61[4] = -(pcVar3[4] == cVar48);
        auVar61[5] = -(pcVar3[5] == cVar50);
        auVar61[6] = -(pcVar3[6] == cVar51);
        auVar61[7] = -(pcVar3[7] == cVar52);
        auVar61[8] = -(pcVar3[8] == cVar48);
        auVar61[9] = -(pcVar3[9] == cVar50);
        auVar61[10] = -(pcVar3[10] == cVar51);
        auVar61[0xb] = -(pcVar3[0xb] == cVar52);
        auVar61[0xc] = -(pcVar3[0xc] == cVar48);
        auVar61[0xd] = -(pcVar3[0xd] == cVar50);
        auVar61[0xe] = -(pcVar3[0xe] == cVar51);
        auVar61[0xf] = -(pcVar3[0xf] == cVar52);
        uVar43 = (ushort)(SUB161(auVar61 >> 7,0) & 1) | (ushort)(SUB161(auVar61 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar61 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar61 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar61 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar61 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar61 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar61 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar61 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar61 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar61 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar61 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar61 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar61 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar61 >> 0x77,0) & 1) << 0xe | (ushort)(auVar61[0xf] >> 7) << 0xf
        ;
        uVar18 = (uint)uVar43;
        while (uVar43 != 0) {
          uVar26 = 0;
          for (uVar13 = uVar18; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar26 = uVar26 + 1;
          }
          lVar33 = (uVar26 + uVar30 & *(ulonglong *)((longlong)*local_150 + 8)) * -0x2a8;
          if (*(undefined1 (**) [16])(lVar37 + -0x2a8 + lVar33) == pauVar22) {
            if ((*(ulonglong *)(lVar37 + -0x290 + lVar33) < 2) ||
               (*(float *)(lVar37 + -0x270 + lVar33) <= 0.0)) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
            local_be8 = pauVar22;
            if (local_100[3] == 0) goto LAB_140a8c217;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar38 = *local_100;
            bVar47 = (byte)(extraout_RAX_05 >> 0x39);
            auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
            auVar49 = pshuflw(auVar49,auVar49,0);
            lVar21 = 0;
            uVar30 = extraout_RAX_05;
            goto LAB_140a8c052;
          }
          uVar43 = (ushort)(uVar18 - 1) & (ushort)uVar18;
          uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar43);
        }
        auVar55[0] = -(*pcVar3 == -1);
        auVar55[1] = -(pcVar3[1] == -1);
        auVar55[2] = -(pcVar3[2] == -1);
        auVar55[3] = -(pcVar3[3] == -1);
        auVar55[4] = -(pcVar3[4] == -1);
        auVar55[5] = -(pcVar3[5] == -1);
        auVar55[6] = -(pcVar3[6] == -1);
        auVar55[7] = -(pcVar3[7] == -1);
        auVar55[8] = -(pcVar3[8] == -1);
        auVar55[9] = -(pcVar3[9] == -1);
        auVar55[10] = -(pcVar3[10] == -1);
        auVar55[0xb] = -(pcVar3[0xb] == -1);
        auVar55[0xc] = -(pcVar3[0xc] == -1);
        auVar55[0xd] = -(pcVar3[0xd] == -1);
        auVar55[0xe] = -(pcVar3[0xe] == -1);
        auVar55[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar55 >> 7,0) & 1) != 0 || (SUB161(auVar55 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar55 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar55 >> 0x1f,0) & 1) != 0) || (SUB161(auVar55 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar55 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar55 >> 0x37,0) & 1) != 0) || (SUB161(auVar55 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar55 >> 0x47,0) & 1) != 0) || (SUB161(auVar55 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar55 >> 0x57,0) & 1) != 0) || (SUB161(auVar55 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar55 >> 0x67,0) & 1) != 0) || (SUB161(auVar55 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar55 >> 0x77,0) & 1) != 0) || auVar55[0xf] < '\0') break;
        uVar30 = uVar30 + lVar38 + 0x10;
        lVar38 = lVar38 + 0x10;
      }
    }
  }
  if ((local_118 != (longlong *)0x0) && (local_128 != (undefined1 (*) [16])0x0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_608 = (undefined1 (*) [16])(plVar36 + 0x12);
  local_2c8 = local_608;
  hashbrown::map::HashMap::
  retain<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,hashbrown::set::impl_1::retain::closure_env_0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::wall::wall_state::publish_public_wall_state::publish_public_wall_state::closure_env_0>_>
            ();
  pauVar35 = (undefined1 (*) [16])local_100[0x12];
  lVar37 = local_100[0x15];
  auVar49 = *pauVar35;
  uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
  pauVar34 = pauVar35 + 1;
  local_128 = (undefined1 (*) [16])
              CONCAT44(local_128._4_4_,*(undefined4 *)((longlong)in_stack_00000028 + 0x1c));
  local_118 = (longlong *)*in_stack_00000028;
  local_138 = (longlong *)in_stack_00000028[2];
  local_148 = (undefined8 *)((longlong)local_118 + 0x20);
  while (lVar37 != 0) {
    if ((short)uVar25 == 0) {
      do {
        auVar49 = *pauVar34;
        pauVar35 = pauVar35 + -8;
        pauVar34 = pauVar34 + 1;
        uVar25 = (uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                               (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
      } while (uVar25 == 0xffff);
      uVar25 = ~uVar25;
    }
    iVar19 = 0;
    for (uVar18 = uVar25; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
      iVar19 = iVar19 + 1;
    }
    local_1d8 = *(undefined8 *)((longlong)pauVar35 + (-8 - (ulonglong)(uint)(iVar19 << 3)));
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>_>
              ();
    uVar25 = uVar25 - 1 & uVar25;
    lVar37 = lVar37 + -1;
    if ((!SBORROW8(0,(longlong)plStack_600)) &&
       (memcpy(&local_be8,&plStack_600,0x2a0), !SBORROW8(0,(longlong)local_be8))) {
      core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>()
      ;
      uVar11 = local_1d8;
      *(undefined4 *)local_138 = local_128._0_4_;
      lVar38 = *(longlong *)((longlong)local_118 + 0x30);
      uVar23 = *(undefined8 *)((longlong)local_118 + 0x40);
      if (lVar38 == *(longlong *)((longlong)local_118 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar33 = *(longlong *)((longlong)local_118 + 0x28);
      *(undefined8 *)(lVar33 + lVar38 * 0x10) = uVar23;
      *(undefined8 *)(lVar33 + 8 + lVar38 * 0x10) = uVar11;
      *(longlong *)((longlong)local_118 + 0x30) = lVar38 + 1;
      *(longlong *)((longlong)local_118 + 0x40) = *(longlong *)((longlong)local_118 + 0x40) + 1;
    }
  }
  pauVar22 = (undefined1 (*) [16])local_100[6];
  local_118 = (longlong *)local_100[9];
  auVar49 = *pauVar22;
  local_148 = (undefined8 *)
              (ulonglong)
              ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                             (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
  pauVar34 = pauVar22 + 1;
  pauVar45 = (undefined1 (*) [16])local_100[0xc];
  auVar49 = *pauVar45;
  uVar25 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar49[0xf] >> 7) << 0xf);
  local_230 = *in_stack_00000058;
  local_2e8 = *in_stack_00000070;
  local_2e0 = *in_stack_00000068;
  local_2d0 = *in_stack_00000060;
  local_1e8 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  local_238 = *in_stack_00000030;
  pauVar31 = (undefined1 (*) [16])in_stack_00000030[2];
  pauVar35 = pauVar45 + 1;
  local_138 = (longlong *)local_100[0xf];
  uVar27 = local_238 + 0x20;
  local_2f0 = pauVar31;
  local_2d8 = uVar27;
LAB_140a8c513:
  plVar36 = local_100;
  fVar65 = 24.0;
  if (pauVar22 != (undefined1 (*) [16])0x0) {
    if (local_118 != (longlong *)0x0) {
      uVar18 = (uint)local_148;
      if ((short)local_148 == 0) {
        do {
          auVar49 = *pauVar34;
          uVar43 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                   (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
          pauVar22 = pauVar22 + -8;
          pauVar34 = pauVar34 + 1;
        } while (uVar43 == 0xffff);
        uVar26 = -(uint)uVar43 - 2;
        uVar27 = (ulonglong)uVar26;
        uVar18 = ~(uint)uVar43;
        uVar26 = uVar18 & uVar26;
      }
      else {
        uVar26 = uVar18 - 1 & uVar18;
      }
      local_148 = (undefined8 *)(ulonglong)uVar26;
      local_118 = (longlong *)((longlong)local_118 + -1);
      local_110 = (undefined1 (*) [16])CONCAT71((int7)(uVar27 >> 8),1);
      uVar26 = uVar25;
      local_128 = pauVar22;
      goto LAB_140a8c621;
    }
    local_118 = (longlong *)0x0;
  }
  if ((pauVar45 != (undefined1 (*) [16])0x0) && (local_138 != (longlong *)0x0)) {
    if ((short)uVar25 == 0) {
      do {
        auVar49 = *pauVar35;
        uVar43 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
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
                 (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar49[0xf] >> 7) << 0xf;
        pauVar45 = pauVar45 + -8;
        pauVar35 = pauVar35 + 1;
      } while (uVar43 == 0xffff);
      uVar25 = ~(uint)uVar43;
      uVar26 = uVar25 & -(uint)uVar43 - 2;
      pauVar22 = pauVar45;
    }
    else {
      uVar26 = uVar25 - 1 & uVar25;
      pauVar22 = pauVar45;
    }
    local_138 = (longlong *)((longlong)local_138 + -1);
    local_128 = (undefined1 (*) [16])0x0;
    local_110 = (undefined1 (*) [16])0x0;
    pauVar45 = pauVar22;
    uVar18 = uVar25;
LAB_140a8c621:
    uVar25 = uVar26;
    iVar19 = 0;
    for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
      iVar19 = iVar19 + 1;
    }
    pauVar22 = *(undefined1 (**) [16])((longlong)pauVar22 + (-8 - (ulonglong)(uint)(iVar19 << 3)));
    local_608 = pauVar22;
    local_190 = pauVar22;
    if (local_100[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar37 = *local_100;
      bVar47 = (byte)((ulonglong)extraout_RAX_07 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
      auVar49 = pshuflw(auVar49,auVar49,0);
      uVar27 = lVar37 - 0x78;
      lVar38 = 0;
      puVar24 = extraout_RAX_07;
      do {
        pauVar31 = (undefined1 (*) [16])(local_100[1] & (ulonglong)puVar24);
        pcVar3 = (char *)((longlong)*pauVar31 + lVar37);
        cVar48 = auVar49[0];
        auVar63[0] = -(*pcVar3 == cVar48);
        cVar50 = auVar49[1];
        auVar63[1] = -(pcVar3[1] == cVar50);
        cVar51 = auVar49[2];
        auVar63[2] = -(pcVar3[2] == cVar51);
        cVar52 = auVar49[3];
        auVar63[3] = -(pcVar3[3] == cVar52);
        auVar63[4] = -(pcVar3[4] == cVar48);
        auVar63[5] = -(pcVar3[5] == cVar50);
        auVar63[6] = -(pcVar3[6] == cVar51);
        auVar63[7] = -(pcVar3[7] == cVar52);
        auVar63[8] = -(pcVar3[8] == cVar48);
        auVar63[9] = -(pcVar3[9] == cVar50);
        auVar63[10] = -(pcVar3[10] == cVar51);
        auVar63[0xb] = -(pcVar3[0xb] == cVar52);
        auVar63[0xc] = -(pcVar3[0xc] == cVar48);
        auVar63[0xd] = -(pcVar3[0xd] == cVar50);
        auVar63[0xe] = -(pcVar3[0xe] == cVar51);
        auVar63[0xf] = -(pcVar3[0xf] == cVar52);
        uVar43 = (ushort)(SUB161(auVar63 >> 7,0) & 1) | (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe | (ushort)(auVar63[0xf] >> 7) << 0xf
        ;
        uVar18 = (uint)uVar43;
        while (uVar43 != 0) {
          uVar26 = 0;
          for (uVar13 = uVar18; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
            uVar26 = uVar26 + 1;
          }
          uVar30 = (ulonglong)((longlong)*pauVar31 + (ulonglong)uVar26) & local_100[1];
          lVar21 = -uVar30;
          lVar33 = uVar30 * -0x78;
          if (*(undefined1 (**) [16])(uVar27 + lVar33) == pauVar22) {
            pauVar22 = local_128;
            if ((*(ulonglong *)(lVar37 + -0x60 + lVar33) < 2) ||
               (*(float *)(lVar37 + -0x40 + lVar33) <= 0.0)) goto LAB_140a8c513;
            local_108 = local_190;
            *(undefined4 *)*local_2f0 = local_1e8;
            lVar38 = *(longlong *)(local_238 + 0x30);
            local_130 = *(undefined1 (**) [16])(local_238 + 0x40);
            if (lVar38 == *(longlong *)(local_238 + 0x20)) {
              alloc::raw_vec::RawVec::
              grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                        ();
            }
            lVar39 = *(longlong *)(local_238 + 0x28);
            *(undefined1 (**) [16])(lVar39 + lVar38 * 0x10) = local_130;
            *(undefined1 (**) [16])(lVar39 + 8 + lVar38 * 0x10) = local_108;
            *(longlong *)(local_238 + 0x30) = lVar38 + 1;
            *(longlong *)(local_238 + 0x40) = *(longlong *)(local_238 + 0x40) + 1;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            hashbrown::raw::RawTable::
            remove_entry<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState>_>
                      ();
            if (SBORROW8(0,(longlong)plStack_600)) {
              local_e88[0] = -0x8000000000000000;
              uVar42 = (undefined7)((ulonglong)lVar21 >> 8);
LAB_140a8c82b:
              local_180 = 0;
              uVar40 = (undefined4)CONCAT71(uVar42,1);
              local_120 = 1;
            }
            else {
              memcpy(local_e88,&plStack_600,0x2a0);
              if (SBORROW8(0,local_e88[0])) {
                uVar42 = (undefined7)((ulonglong)lVar21 >> 8);
                goto LAB_140a8c82b;
              }
              memcpy(&local_608,local_e88,0x2a0);
              if ((char)local_110 != '\0') {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              local_119 = local_c98;
              local_1b8 = local_c97;
              local_218 = local_c90;
              local_210 = local_c88;
              local_698 = local_c80;
              uStack_690 = uStack_c78;
              local_618 = local_c40;
              local_628 = local_c50;
              lStack_620 = lStack_c48;
              local_638 = local_c60;
              lStack_630 = lStack_c58;
              local_648 = local_c70;
              plStack_640 = plStack_c68;
              local_1dc = local_c38;
              local_1e4 = local_c34;
              if (local_608 != (undefined1 (*) [16])0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (uStack_5f0 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_5d0 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_5b8 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_598 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_580 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_568 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              if (local_548 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              core::ptr::
              drop_in_place<country_core::render::sparse_instance_buffer::SparseInstanceBufferAllocation<country_core::resources::sparkle::SparkleSsbo>_>
                        ();
              uVar23 = extraout_RAX_08;
              if (local_530 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                uVar23 = extraout_RAX_09;
              }
              if (local_518 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                uVar23 = extraout_RAX_10;
              }
              if (local_478 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                uVar23 = extraout_RAX_11;
              }
              local_180 = CONCAT71((int7)((ulonglong)uVar23 >> 8),1);
              uVar40 = 0;
              local_120 = 0;
            }
            local_108 = (undefined1 (*) [16])(lVar37 + lVar33);
            lVar33 = lVar37 + lVar33 + -0x70;
            local_1c0 = local_1b8;
            local_6a8 = local_698;
            uStack_6a0 = uStack_690;
            local_658 = local_618;
            local_668 = local_628;
            lStack_660 = lStack_620;
            local_678 = local_638;
            lStack_670 = lStack_630;
            local_688 = local_648;
            plStack_680 = plStack_640;
            lVar37 = *(longlong *)(local_2e8 + 8);
            lVar38 = *(longlong *)(local_2e8 + 0x10) * 0x98;
            goto LAB_140a8cb61;
          }
          uVar43 = (ushort)(uVar18 - 1) & (ushort)uVar18;
          uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar43);
        }
        auVar57[0] = -(*pcVar3 == -1);
        auVar57[1] = -(pcVar3[1] == -1);
        auVar57[2] = -(pcVar3[2] == -1);
        auVar57[3] = -(pcVar3[3] == -1);
        auVar57[4] = -(pcVar3[4] == -1);
        auVar57[5] = -(pcVar3[5] == -1);
        auVar57[6] = -(pcVar3[6] == -1);
        auVar57[7] = -(pcVar3[7] == -1);
        auVar57[8] = -(pcVar3[8] == -1);
        auVar57[9] = -(pcVar3[9] == -1);
        auVar57[10] = -(pcVar3[10] == -1);
        auVar57[0xb] = -(pcVar3[0xb] == -1);
        auVar57[0xc] = -(pcVar3[0xc] == -1);
        auVar57[0xd] = -(pcVar3[0xd] == -1);
        auVar57[0xe] = -(pcVar3[0xe] == -1);
        auVar57[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar57 >> 7,0) & 1) != 0 || (SUB161(auVar57 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar57 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar57 >> 0x1f,0) & 1) != 0) || (SUB161(auVar57 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar57 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar57 >> 0x37,0) & 1) != 0) || (SUB161(auVar57 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar57 >> 0x47,0) & 1) != 0) || (SUB161(auVar57 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar57 >> 0x57,0) & 1) != 0) || (SUB161(auVar57 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar57 >> 0x67,0) & 1) != 0) || (SUB161(auVar57 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar57 >> 0x77,0) & 1) != 0) || auVar57[0xf] < '\0') break;
        puVar24 = (undefined1 *)((longlong)pauVar31[1] + lVar38);
        lVar38 = lVar38 + 0x10;
      } while( true );
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (local_100[9] != 0) {
    uVar27 = local_100[7];
    if (uVar27 != 0) {
      pauVar31 = (undefined1 (*) [16])CONCAT71((int7)((ulonglong)pauVar31 >> 8),0xff);
      memset((void *)local_100[6],(int)pauVar31,uVar27 + 0x11);
    }
    plVar36[9] = 0;
    uVar30 = (uVar27 + 1 & 0xfffffffffffffff8) - (uVar27 + 1 >> 3);
    if (uVar27 < 8) {
      uVar30 = uVar27;
    }
    plVar36[8] = uVar30;
  }
  if (plVar36[0xf] != 0) {
    uVar27 = plVar36[0xd];
    if (uVar27 != 0) {
      pauVar31 = (undefined1 (*) [16])CONCAT71((int7)((ulonglong)pauVar31 >> 8),0xff);
      memset((void *)*local_158,(int)pauVar31,uVar27 + 0x11);
    }
    plVar36[0xf] = 0;
    uVar30 = (uVar27 + 1 & 0xfffffffffffffff8) - (uVar27 + 1 >> 3);
    if (uVar27 < 8) {
      uVar30 = uVar27;
    }
    plVar36[0xe] = uVar30;
  }
  if (plVar36[0x15] != 0) {
    uVar27 = plVar36[0x13];
    if (uVar27 != 0) {
      memset(*(void **)*local_2c8,(int)CONCAT71((int7)((ulonglong)pauVar31 >> 8),0xff),uVar27 + 0x11
            );
    }
    plVar36[0x15] = 0;
    uVar30 = (uVar27 + 1 & 0xfffffffffffffff8) - (uVar27 + 1 >> 3);
    if (uVar27 < 8) {
      uVar30 = uVar27;
    }
    plVar36[0x14] = uVar30;
  }
  if (local_248 == 0) {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_29 == 1) {
    plVar36 = extraout_RAX_29 + 1;
  }
  else if ((*extraout_RAX_29 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar36 = extraout_RAX_30, extraout_RAX_30 == (longlong *)0x0))
  goto LAB_140a8ddaf;
  if (*plVar36 == 0) {
    *plVar36 = -1;
    local_100 = plVar36;
    puffin::ThreadProfiler::end_scope();
    *local_100 = *local_100 + 1;
    return;
  }
  goto LAB_140a8df97;
LAB_140a8c052:
  uVar30 = local_100[1] & uVar30;
  pcVar3 = (char *)(lVar38 + uVar30);
  cVar48 = auVar49[0];
  auVar62[0] = -(*pcVar3 == cVar48);
  cVar50 = auVar49[1];
  auVar62[1] = -(pcVar3[1] == cVar50);
  cVar51 = auVar49[2];
  auVar62[2] = -(pcVar3[2] == cVar51);
  cVar52 = auVar49[3];
  auVar62[3] = -(pcVar3[3] == cVar52);
  auVar62[4] = -(pcVar3[4] == cVar48);
  auVar62[5] = -(pcVar3[5] == cVar50);
  auVar62[6] = -(pcVar3[6] == cVar51);
  auVar62[7] = -(pcVar3[7] == cVar52);
  auVar62[8] = -(pcVar3[8] == cVar48);
  auVar62[9] = -(pcVar3[9] == cVar50);
  auVar62[10] = -(pcVar3[10] == cVar51);
  auVar62[0xb] = -(pcVar3[0xb] == cVar52);
  auVar62[0xc] = -(pcVar3[0xc] == cVar48);
  auVar62[0xd] = -(pcVar3[0xd] == cVar50);
  auVar62[0xe] = -(pcVar3[0xe] == cVar51);
  auVar62[0xf] = -(pcVar3[0xf] == cVar52);
  uVar43 = (ushort)(SUB161(auVar62 >> 7,0) & 1) | (ushort)(SUB161(auVar62 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar62 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar62 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar62 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar62 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar62 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar62 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar62 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar62 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar62 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar62 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar62 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar62 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar62 >> 0x77,0) & 1) << 0xe | (ushort)(auVar62[0xf] >> 7) << 0xf;
  uVar18 = (uint)uVar43;
  while (uVar43 != 0) {
    uVar26 = 0;
    for (uVar13 = uVar18; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar26 = uVar26 + 1;
    }
    lVar39 = (uVar26 + uVar30 & local_100[1]) * -0x78;
    if (*(undefined1 (**) [16])(lVar38 + -0x78 + lVar39) == pauVar22) {
      fVar65 = *(float *)(lVar38 + -0x18 + lVar39);
      lVar38 = *(longlong *)(lVar37 + lVar33 + -0x260);
      lVar37 = lVar38 + *(longlong *)(lVar37 + lVar33 + -600) * 0xc;
      goto LAB_140a8c0e5;
    }
    uVar43 = (ushort)(uVar18 - 1) & (ushort)uVar18;
    uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar43);
  }
  auVar56[0] = -(*pcVar3 == -1);
  auVar56[1] = -(pcVar3[1] == -1);
  auVar56[2] = -(pcVar3[2] == -1);
  auVar56[3] = -(pcVar3[3] == -1);
  auVar56[4] = -(pcVar3[4] == -1);
  auVar56[5] = -(pcVar3[5] == -1);
  auVar56[6] = -(pcVar3[6] == -1);
  auVar56[7] = -(pcVar3[7] == -1);
  auVar56[8] = -(pcVar3[8] == -1);
  auVar56[9] = -(pcVar3[9] == -1);
  auVar56[10] = -(pcVar3[10] == -1);
  auVar56[0xb] = -(pcVar3[0xb] == -1);
  auVar56[0xc] = -(pcVar3[0xc] == -1);
  auVar56[0xd] = -(pcVar3[0xd] == -1);
  auVar56[0xe] = -(pcVar3[0xe] == -1);
  auVar56[0xf] = -(pcVar3[0xf] == -1);
  if ((((((((((((((((SUB161(auVar56 >> 7,0) & 1) != 0 || (SUB161(auVar56 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar56 >> 0x17,0) & 1) != 0) || (SUB161(auVar56 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar56 >> 0x27,0) & 1) != 0) || (SUB161(auVar56 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar56 >> 0x37,0) & 1) != 0) || (SUB161(auVar56 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar56 >> 0x47,0) & 1) != 0) || (SUB161(auVar56 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar56 >> 0x57,0) & 1) != 0) || (SUB161(auVar56 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar56 >> 0x67,0) & 1) != 0) || (SUB161(auVar56 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar56 >> 0x77,0) & 1) != 0) || auVar56[0xf] < '\0') {
LAB_140a8c217:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar30 = uVar30 + lVar21 + 0x10;
  lVar21 = lVar21 + 0x10;
  goto LAB_140a8c052;
  while (extraout_XMM0_Da <= fVar5) {
LAB_140a8c0e5:
    if (lVar38 == lVar37) goto LAB_140a8c150;
    resources::terrain_height::TerrainHeightsData::
    sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
    fVar5 = *(float *)(lVar38 + 4);
    lVar38 = lVar38 + 0xc;
    if (fVar65 < fVar5) {
      if ((extraout_XMM0_Da != fVar5) || (NAN(extraout_XMM0_Da) || NAN(fVar5))) {
        hashbrown::map::HashMap::
        insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                  ();
        goto LAB_140a8c150;
      }
      goto LAB_140a8c0e5;
    }
  }
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  goto LAB_140a8c150;
  while( true ) {
    if (lVar33 < 0) {
                    // WARNING: Subroutine does not return
      alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
    }
    LOCK();
    lVar21 = *(longlong *)(lVar38 + 8);
    bVar46 = lVar33 == lVar21;
    if (bVar46) {
      *(longlong *)(lVar38 + 8) = lVar33 + 1;
      lVar21 = lVar33;
    }
    UNLOCK();
    lVar33 = lVar21;
    if (bVar46) break;
    while (lVar33 == -1) {
LAB_140a8cfa5:
      lVar33 = *(longlong *)(lVar38 + 8);
    }
  }
  local_310 = lVar38;
  local_2b0 = lVar38;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_18 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX_18 = 0;
  extraout_RAX_18[1] = 4;
  extraout_RAX_18[2] = 0;
  *(undefined1 *)(extraout_RAX_18 + 3) = 0;
  lVar38 = *(longlong *)(lVar37 + 0x60);
  do {
    lVar33 = *(longlong *)(lVar38 + 8);
    while (lVar33 != -1) {
      if (lVar33 < 0) {
                    // WARNING: Subroutine does not return
        alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
      }
      LOCK();
      lVar21 = *(longlong *)(lVar38 + 8);
      bVar46 = lVar33 == lVar21;
      if (bVar46) {
        *(longlong *)(lVar38 + 8) = lVar33 + 1;
        lVar21 = lVar33;
      }
      UNLOCK();
      lVar33 = lVar21;
      if (bVar46) {
        local_320 = lVar38;
        local_2a8 = extraout_RAX_18;
        local_2a0 = lVar38;
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_19 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_19 = 0;
        extraout_RAX_19[1] = 4;
        extraout_RAX_19[2] = 0;
        *(undefined1 *)(extraout_RAX_19 + 3) = 0;
        lVar37 = *(longlong *)(lVar37 + 0x60);
        goto LAB_140a8d0ac;
      }
    }
  } while( true );
  while( true ) {
    if (lVar33 < 0) {
                    // WARNING: Subroutine does not return
      alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
    }
    LOCK();
    lVar21 = *(longlong *)(lVar38 + 8);
    bVar46 = lVar33 == lVar21;
    if (bVar46) {
      *(longlong *)(lVar38 + 8) = lVar33 + 1;
      lVar21 = lVar33;
    }
    UNLOCK();
    lVar33 = lVar21;
    if (bVar46) break;
    while (lVar33 == -1) {
LAB_140a8d24c:
      lVar33 = *(longlong *)(lVar38 + 8);
    }
  }
  local_350 = lVar38;
  local_270 = extraout_RAX_22;
  local_268 = lVar38;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_23 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  *extraout_RAX_23 = 0;
  extraout_RAX_23[1] = 4;
  extraout_RAX_23[2] = 0;
  *(undefined1 *)(extraout_RAX_23 + 3) = 0;
  lVar37 = *(longlong *)(lVar37 + 0x60);
LAB_140a8d2cc:
  lVar38 = *(longlong *)(lVar37 + 8);
  do {
    if (lVar38 == -1) goto LAB_140a8d2cc;
    if (lVar38 < 0) {
                    // WARNING: Subroutine does not return
      alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
    }
    LOCK();
    lVar33 = *(longlong *)(lVar37 + 8);
    bVar46 = lVar38 == lVar33;
    if (bVar46) {
      *(longlong *)(lVar37 + 8) = lVar38 + 1;
      lVar33 = lVar38;
    }
    UNLOCK();
    lVar38 = lVar33;
  } while (!bVar46);
  if (pauVar22 != (undefined1 (*) [16])0x0) {
    if (pauVar22 == (undefined1 (*) [16])0x1) {
      uVar27 = 0;
    }
    else {
      uVar27 = 0;
      do {
        fVar66 = *(float *)((longlong)*pauVar31 + uVar27 * 4);
        fVar67 = *(float *)((longlong)*pauVar31 + uVar27 * 4 + 4);
        fVar64 = fVar66;
        if (fVar65 <= fVar66) {
          fVar64 = fVar65;
        }
        fVar66 = (float)(~-(uint)NAN(fVar65) & (uint)fVar64 | -(uint)NAN(fVar65) & (uint)fVar66);
        fVar65 = fVar67;
        if (fVar66 <= fVar67) {
          fVar65 = fVar66;
        }
        fVar65 = (float)(-(uint)NAN(fVar66) & (uint)fVar67 | ~-(uint)NAN(fVar66) & (uint)fVar65);
        uVar27 = uVar27 + 2;
      } while (((ulonglong)pauVar22 & 0xfffffffffffffffe) != uVar27);
    }
    if (((ulonglong)pauVar22 & 1) != 0) {
      fVar66 = *(float *)((longlong)*pauVar31 + uVar27 * 4);
      fVar67 = fVar66;
      if (fVar65 <= fVar66) {
        fVar67 = fVar65;
      }
      fVar65 = (float)(~-(uint)NAN(fVar65) & (uint)fVar67 | -(uint)NAN(fVar65) & (uint)fVar66);
    }
  }
  if (pauVar22 == (undefined1 (*) [16])0x0) {
    fVar67 = 0.0;
    fVar66 = 100.0;
  }
  else {
    if (pauVar22 == (undefined1 (*) [16])0x1) {
      fVar66 = 100.0;
      uVar27 = 0;
    }
    else {
      pfVar32 = (float *)(local_140 + 2);
      uVar27 = 0;
      fVar66 = 100.0;
      do {
        fVar67 = pfVar32[-3];
        fVar64 = *pfVar32;
        fVar6 = fVar67;
        if (fVar66 <= fVar67) {
          fVar6 = fVar66;
        }
        fVar67 = (float)(~-(uint)NAN(fVar66) & (uint)fVar6 | -(uint)NAN(fVar66) & (uint)fVar67);
        fVar66 = fVar64;
        if (fVar67 <= fVar64) {
          fVar66 = fVar67;
        }
        fVar66 = (float)(-(uint)NAN(fVar67) & (uint)fVar64 | ~-(uint)NAN(fVar67) & (uint)fVar66);
        uVar27 = uVar27 + 2;
        pfVar32 = pfVar32 + 6;
      } while (((ulonglong)pauVar22 & 0xfffffffffffffffe) != uVar27);
    }
    if (((ulonglong)pauVar22 & 1) != 0) {
      fVar67 = *(float *)((longlong)local_140 + uVar27 * 0xc + 4);
      fVar64 = fVar67;
      if (fVar66 <= fVar67) {
        fVar64 = fVar66;
      }
      fVar66 = (float)(~-(uint)NAN(fVar66) & (uint)fVar64 | -(uint)NAN(fVar66) & (uint)fVar67);
    }
    if (pauVar22 == (undefined1 (*) [16])0x1) {
      fVar67 = 0.0;
      uVar27 = 0;
    }
    else {
      pfVar32 = (float *)(local_140 + 2);
      uVar27 = 0;
      fVar67 = 0.0;
      do {
        fVar64 = pfVar32[-3];
        fVar6 = *pfVar32;
        fVar17 = fVar64;
        if (fVar64 <= fVar67) {
          fVar17 = fVar67;
        }
        fVar64 = (float)(~-(uint)NAN(fVar67) & (uint)fVar17 | -(uint)NAN(fVar67) & (uint)fVar64);
        fVar67 = fVar6;
        if (fVar6 <= fVar64) {
          fVar67 = fVar64;
        }
        fVar67 = (float)(-(uint)NAN(fVar64) & (uint)fVar6 | ~-(uint)NAN(fVar64) & (uint)fVar67);
        uVar27 = uVar27 + 2;
        pfVar32 = pfVar32 + 6;
      } while (((ulonglong)pauVar22 & 0xfffffffffffffffe) != uVar27);
    }
    if (((ulonglong)pauVar22 & 1) != 0) {
      fVar64 = *(float *)((longlong)local_140 + uVar27 * 0xc + 4);
      fVar6 = fVar64;
      if (fVar64 <= fVar67) {
        fVar6 = fVar67;
      }
      fVar67 = (float)(~-(uint)NAN(fVar67) & (uint)fVar6 | -(uint)NAN(fVar67) & (uint)fVar64);
    }
  }
  local_140 = *(longlong **)((longlong)local_108 - 0x60);
  sVar4 = (longlong)local_140 * 8;
  local_360 = lVar37;
  if (((ulonglong)local_140 >> 0x3d != 0) || (0x7ffffffffffffffc < sVar4)) {
LAB_140a8e1cb:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar12 = *(void **)((longlong)local_108 - 0x68);
  local_260 = extraout_RAX_23;
  if (sVar4 == 0) {
    local_228 = &DAT_00000004;
    local_1a8 = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_24 == (undefined1 *)0x0) goto LAB_140a8e1cb;
    local_1a8 = (ulonglong)local_140;
    local_228 = extraout_RAX_24;
  }
  memcpy(local_228,pvVar12,sVar4);
  puVar24 = *(undefined1 **)((longlong)local_108 - 0x48);
  sVar4 = (longlong)puVar24 * 4;
  if (((ulonglong)puVar24 >> 0x3e != 0) || (0x7ffffffffffffffc < sVar4)) {
LAB_140a8e1de:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar12 = *(void **)((longlong)local_108 - 0x50);
  if (sVar4 == 0) {
    _Dst = &DAT_00000004;
    local_1a0 = (undefined1 *)0x0;
  }
  else {
    local_1c8 = puVar24;
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_25 == (undefined1 *)0x0) goto LAB_140a8e1de;
    local_1a0 = local_1c8;
    _Dst = extraout_RAX_25;
    puVar24 = local_1c8;
  }
  local_1c8 = _Dst;
  memcpy(_Dst,pvVar12,sVar4);
  pauVar31 = local_150;
  pauVar22 = local_bd8;
  plVar36 = plStack_be0;
  local_6bc = *(undefined4 *)((longlong)local_108 - 0x40);
  uVar18 = (int)local_178 - 2;
  if (1 < uVar18) {
    uVar18 = 2;
  }
  local_6f0 = 2;
  if (uVar18 != 0) {
    if (uVar18 == 2) {
      local_1f8 = *(undefined8 *)((longlong)local_108 - 0x24);
      local_208 = *(undefined **)((longlong)local_108 - 0x34);
      uStack_200 = *(undefined8 *)((longlong)local_108 - 0x2c);
      local_6f0 = (int)local_178;
    }
    else {
      local_6f0 = 3;
    }
  }
  if ((char)local_180 == '\0') {
    local_5e8 = (undefined1 *)0x0;
    uStack_5e0 = 0;
    local_5f8 = (undefined1 (*) [16])0x0;
    uStack_5f0 = 0;
    local_608 = (undefined1 (*) [16])0x0;
    plStack_600 = (longlong *)0x0;
    local_5d8 = 0;
    local_258 = 0;
    uStack_250 = 0;
    local_170 = 0;
    local_178 = 4;
    local_220 = 0;
    local_180 = (ulonglong)local_180._4_4_ << 0x20;
  }
  else {
    uStack_198 = local_1c0;
    local_258 = local_6a8;
    uStack_250 = uStack_6a0;
    local_608 = local_688;
    plStack_600 = plStack_680;
    local_5f8 = local_678;
    uStack_5f0 = lStack_670;
    local_5e8 = (undefined1 *)local_668;
    uStack_5e0 = lStack_660;
    local_5d8 = local_658;
    local_170 = (ulonglong)local_119;
    local_220 = local_218;
    local_178 = local_210;
    local_180 = CONCAT44(local_180._4_4_,local_1dc);
  }
  fVar64 = 0.3;
  if ((char)local_1b0 == '\0') {
    fVar64 = 0.5;
  }
  auVar58 = ZEXT812(0x40200000);
  if ((char)local_1b0 == '\0') {
    auVar58 = ZEXT812(0x41600000);
  }
  local_6b1 = local_2c0 != 0;
  local_710 = 0;
  uStack_708 = 0;
  local_720 = 0;
  uStack_718 = 0;
  local_730 = 0;
  uStack_728 = 0;
  local_740 = 0;
  uStack_738 = 0;
  local_700 = 0;
  local_8d8 = local_130;
  local_8d0 = local_110;
  local_8c8 = local_130;
  local_6c8 = *(undefined4 *)((longlong)local_108 - 0x18);
  local_6c0 = *(undefined4 *)((longlong)local_108 - 0x14);
  local_6b8 = *(undefined4 *)((longlong)local_108 - 0x10);
  local_6b4 = *(undefined1 *)((longlong)local_108 - 0xc);
  local_6b3 = *(undefined1 *)((longlong)local_108 - 10);
  local_6b0 = *(undefined1 *)((longlong)local_108 - 9);
  local_6af = *(undefined2 *)((longlong)local_108 - 8);
  local_948 = local_1a8;
  local_940 = local_228;
  local_938 = (ulonglong)local_140;
  local_930 = local_1a0;
  local_928 = local_1c8;
  local_8e0 = local_bb8;
  if (fVar64 <= fVar5) {
    fVar64 = fVar5;
  }
  if (fVar64 <= auVar58._0_4_) {
    auVar58._4_8_ = auVar58._4_8_;
    auVar58._0_4_ = fVar64;
  }
  local_8f0 = local_bc8;
  uStack_8e8 = uStack_bc0;
  local_900 = local_bd8;
  ppauStack_8f8 = ppauStack_bd0;
  local_910 = local_be8;
  plStack_908 = plStack_be0;
  local_6c4 = auVar58._0_4_;
  local_8c0 = 0;
  local_8b8 = 4;
  local_8b0 = 0;
  uStack_8a8 = 0;
  local_8a0 = 4;
  local_898 = 0;
  local_890 = 0;
  local_888 = 0;
  local_880 = 4;
  local_878 = 0;
  local_6dc = local_1f8;
  local_6ec = local_208;
  uStack_6e4 = uStack_200;
  local_830 = local_1d0;
  local_828 = local_2b8;
  local_820 = local_278;
  local_818 = local_280;
  local_810 = local_268;
  local_808 = local_270;
  local_800 = local_2b0;
  local_7f8 = local_288;
  local_7f0 = local_2a0;
  local_7e8 = local_2a8;
  local_7e0 = local_290;
  local_7d8 = local_298;
  local_7c8 = local_260;
  local_870 = 0;
  local_868 = 4;
  local_860 = 0;
  uStack_858 = 0;
  local_850 = 4;
  local_848 = 0;
  local_7c0 = (undefined1)local_170;
  uStack_7bf = uStack_198;
  local_7b8 = local_220;
  local_7b0 = local_178;
  local_7a8 = local_258;
  uStack_7a0 = uStack_250;
  local_768 = local_5d8;
  local_778 = (longlong)local_5e8;
  lStack_770 = uStack_5e0;
  local_788 = local_5f8;
  lStack_780 = uStack_5f0;
  local_798 = local_608;
  plStack_790 = plStack_600;
  local_760 = (undefined4)local_180;
  local_75c = local_1e4;
  local_758 = 0;
  local_750 = 0;
  local_748 = 4;
  local_6f8 = 0;
  local_840 = 0;
  local_838 = 0;
  local_6b2 = (char)local_1b0;
  local_920 = puVar24;
  local_918 = uVar40;
  local_7d0 = lVar37;
  local_6d4 = fVar66;
  local_6d0 = fVar67;
  local_6cc = fVar65;
  if (*(char *)(local_2e0 + 0xf4) == '\x01') {
    auVar49._8_8_ = 0;
    auVar49._0_8_ = local_bd8;
    _Size = SUB168(auVar49 * ZEXT816(0xc),0);
    if ((SUB168(auVar49 * ZEXT816(0xc),8) != 0) || (0x7ffffffffffffffc < _Size)) {
LAB_140a8e1f1:
      local_f2 = 1;
                    // WARNING: Subroutine does not return
      alloc::raw_vec::handle_error();
    }
    if (_Size == (undefined1 (*) [16])0x0) {
      _Dst_00 = (longlong *)&DAT_00000004;
      local_110 = (undefined1 (*) [16])0x0;
    }
    else {
      local_108 = _Size;
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_26 == (longlong *)0x0) goto LAB_140a8e1f1;
      local_110 = pauVar22;
      _Dst_00 = extraout_RAX_26;
      _Size = local_108;
    }
    memcpy(_Dst_00,plVar36,(size_t)_Size);
    uStack_5c4 = (local_1e0 & 0xffff0000) + 1;
    local_608 = local_110;
    local_5f8 = pauVar22;
    uStack_5f0 = -0x8000000000000000;
    local_5d8 = CONCAT44(local_5d8._4_4_,1);
    local_5c8 = 0x40400000;
    local_f2 = 1;
    plStack_600 = _Dst_00;
    local_1e0 = uStack_5c4;
    country_core::utils::debug_viz::DebugLine::show();
  }
  memcpy(&local_608,&local_948,0x2a0);
  local_f2 = 0;
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::WallId,country_core::resources::walls::public_wall_state::PublicWallState,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (!SBORROW8(0,(longlong)local_be8)) {
    local_f2 = 0;
    core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>();
  }
  uVar27 = 0;
  pauVar22 = local_128;
  if ((!SBORROW8(0,local_e88[0])) && ((char)local_120 != '\0')) {
    core::ptr::drop_in_place<country_core::resources::walls::public_wall_state::PublicWallState>();
    uVar27 = extraout_RAX_27;
    pauVar22 = local_128;
  }
  goto LAB_140a8c513;
  while( true ) {
    if (lVar38 < 0) {
                    // WARNING: Subroutine does not return
      alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
    }
    LOCK();
    lVar33 = *(longlong *)(lVar37 + 8);
    bVar46 = lVar38 == lVar33;
    if (bVar46) {
      *(longlong *)(lVar37 + 8) = lVar38 + 1;
      lVar33 = lVar38;
    }
    UNLOCK();
    lVar38 = lVar33;
    if (bVar46) break;
    while (lVar38 == -1) {
LAB_140a8d0ac:
      lVar38 = *(longlong *)(lVar37 + 8);
    }
  }
  local_330 = lVar37;
  local_298 = extraout_RAX_19;
  local_290 = lVar37;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_20 != (undefined8 *)0x0) {
    local_208 = &anon_107183261458dea85466985370debbb6_60_llvm_3641509820080252961;
    uStack_200 = 0x3d;
    if ((extraout_RAX_20[3] != 0x3d) ||
       (local_1d0 = extraout_RAX_20 + 2,
       iVar19 = memcmp(&anon_107183261458dea85466985370debbb6_60_llvm_3641509820080252961,
                       (void *)extraout_RAX_20[2],0x3d), iVar19 != 0)) {
      local_608 = (undefined1 (*) [16])0x0;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar37 = (**(code **)(extraout_RAX_20[1] + 0x18))(*extraout_RAX_20);
    if (lVar37 != 0) {
      local_288 = extraout_RAX_17;
      local_1d0 = puVar2;
      std::alloc::__default_lib_allocator::__rust_alloc();
      pauVar31 = local_110;
      pauVar22 = local_130;
      if (extraout_RAX_21 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_21 = 0;
      extraout_RAX_21[1] = 4;
      extraout_RAX_21[2] = 0;
      *(undefined1 *)(extraout_RAX_21 + 3) = 0;
      lVar38 = *(longlong *)(lVar37 + 0x60);
      do {
        lVar33 = *(longlong *)(lVar38 + 8);
        while (lVar33 != -1) {
          if (lVar33 < 0) {
                    // WARNING: Subroutine does not return
            alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
          }
          LOCK();
          lVar21 = *(longlong *)(lVar38 + 8);
          bVar46 = lVar33 == lVar21;
          if (bVar46) {
            *(longlong *)(lVar38 + 8) = lVar33 + 1;
            lVar21 = lVar33;
          }
          UNLOCK();
          lVar33 = lVar21;
          if (bVar46) {
            local_340 = lVar38;
            local_280 = extraout_RAX_21;
            local_278 = lVar38;
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_22 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_22 = 0;
            extraout_RAX_22[1] = 4;
            extraout_RAX_22[2] = 0;
            *(undefined1 *)(extraout_RAX_22 + 3) = 0;
            lVar38 = *(longlong *)(lVar37 + 0x60);
            goto LAB_140a8d24c;
          }
        }
      } while( true );
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
  while( true ) {
    puVar2 = (undefined8 *)(lVar37 + 0x70);
    lVar37 = lVar37 + 0x98;
    lVar38 = local_2c0 + -0x98;
    if ((undefined1 (*) [16])*puVar2 == local_190) break;
LAB_140a8cb61:
    local_2c0 = lVar38;
    if (local_2c0 == 0) break;
  }
  pauVar22 = *(undefined1 (**) [16])((longlong)local_108 - 0x60);
  uVar27 = (longlong)pauVar22 * 8;
  local_1a0 = (undefined1 *)CONCAT44(local_1a0._4_4_,uVar40);
  if (((ulonglong)pauVar22 >> 0x3d != 0) || (0x7ffffffffffffffc < uVar27)) {
LAB_140a8e1a5:
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  pvVar12 = *(void **)((longlong)local_108 - 0x68);
  local_130 = (undefined1 (*) [16])uVar27;
  local_1b0 = lVar33;
  if (uVar27 == 0) {
    plVar36 = (longlong *)&DAT_00000004;
    local_110 = (undefined1 (*) [16])0x0;
    pauVar31 = local_110;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    plVar36 = extraout_RAX_12;
    pauVar31 = pauVar22;
    if (extraout_RAX_12 == (longlong *)0x0) goto LAB_140a8e1a5;
  }
  local_110 = pauVar31;
  sVar4 = (size_t)local_130;
  local_130 = (undefined1 (*) [16])plVar36;
  memcpy(plVar36,pvVar12,sVar4);
  uVar30 = *(ulonglong *)((longlong)local_108 - 0x48);
  uVar27 = uVar30 * 4;
  if ((uVar30 >> 0x3e == 0) && (uVar27 < 0x7ffffffffffffffd)) {
    pvVar12 = *(void **)((longlong)local_108 - 0x50);
    if (uVar27 == 0) {
      puVar24 = &DAT_00000004;
      local_140 = (longlong *)0x0;
      plVar36 = local_140;
    }
    else {
      local_178 = uVar27;
      std::alloc::__default_lib_allocator::__rust_alloc();
      puVar24 = extraout_RAX_13;
      uVar27 = local_178;
      plVar36 = (longlong *)uVar30;
      if (extraout_RAX_13 == (undefined1 *)0x0) goto LAB_140a8e1b8;
    }
    local_140 = plVar36;
    memcpy(puVar24,pvVar12,uVar27);
    uVar40 = *(undefined4 *)((longlong)local_108 - 0x40);
    local_608 = local_110;
    plStack_600 = (longlong *)local_130;
    uStack_5f0 = (ulonglong)local_140;
    local_5d8 = CONCAT44(local_5d8._4_4_,uVar40);
    local_178 = (ulonglong)*(uint *)((longlong)local_108 - 0x38);
    local_5f8 = pauVar22;
    local_5e8 = puVar24;
    uStack_5e0 = uVar30;
    ::utils::curve::Curve::
    crashy_resample_remap_per_segment_2<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D,glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D,country_core::systems::wall::wall_state::publish_public_wall_state::process::closure_env_0>
              ();
    if (local_608 != (undefined1 (*) [16])0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (uStack_5f0 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    pauVar22 = local_bd8;
    local_140 = plStack_be0;
    local_170 = (longlong)local_bd8 * 4;
    local_130 = local_bd8;
    if (local_bd8 == (undefined1 (*) [16])0x0) {
      local_110 = (undefined1 (*) [16])&DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_14 == (undefined1 (*) [16])0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      plVar36 = local_140 + 1;
      pauVar31 = (undefined1 (*) [16])0x0;
      local_110 = extraout_RAX_14;
      do {
        fVar5 = *(float *)((longlong)plVar36 + -4);
        resources::terrain_height::TerrainHeightsData::
        sample_at_world_xz<country_core::resources::terrain_height::TerrainHeightSsbo>();
        *(float *)((longlong)*local_110 + (longlong)pauVar31 * 4) = fVar5 - extraout_XMM0_Da_00;
        pauVar31 = (undefined1 (*) [16])((longlong)*pauVar31 + 1);
        plVar36 = (longlong *)((longlong)plVar36 + 0xc);
      } while (pauVar22 != pauVar31);
    }
    fVar5 = *(float *)((longlong)local_108 - 0x1c);
    local_1b0 = CONCAT71(local_1b0._1_7_,*(undefined1 *)((longlong)local_108 - 0xb));
    frozen::FrozenMap::
    get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
              ();
    if (extraout_RAX_15 != (undefined8 *)0x0) {
      local_208 = &anon_107183261458dea85466985370debbb6_35_llvm_3641509820080252961;
      uStack_200 = 0x3a;
      if ((extraout_RAX_15[3] != 0x3a) ||
         (iVar19 = memcmp(&anon_107183261458dea85466985370debbb6_35_llvm_3641509820080252961,
                          (void *)extraout_RAX_15[2],0x3a), iVar19 != 0)) {
        local_608 = (undefined1 (*) [16])0x0;
                    // WARNING: Subroutine does not return
        core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
      }
      lVar37 = (**(code **)(extraout_RAX_15[1] + 0x18))(*extraout_RAX_15);
      if (lVar37 != 0) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX_16 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX_16 = 0;
        extraout_RAX_16[1] = 4;
        extraout_RAX_16[2] = 0;
        *(undefined1 *)(extraout_RAX_16 + 3) = 0;
        puVar2 = *(undefined8 **)(lVar37 + 0x60);
        do {
          lVar38 = puVar2[1];
          while (lVar38 != -1) {
            if (lVar38 < 0) {
                    // WARNING: Subroutine does not return
              alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
            }
            LOCK();
            lVar33 = puVar2[1];
            bVar46 = lVar38 == lVar33;
            if (bVar46) {
              puVar2[1] = lVar38 + 1;
              lVar33 = lVar38;
            }
            UNLOCK();
            lVar38 = lVar33;
            if (bVar46) {
              local_300 = puVar2;
              local_2b8 = extraout_RAX_16;
              std::alloc::__default_lib_allocator::__rust_alloc();
              if (extraout_RAX_17 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
                alloc::alloc::handle_alloc_error();
              }
              *extraout_RAX_17 = 0;
              extraout_RAX_17[1] = 4;
              extraout_RAX_17[2] = 0;
              *(undefined1 *)(extraout_RAX_17 + 3) = 0;
              lVar38 = *(longlong *)(lVar37 + 0x60);
              goto LAB_140a8cfa5;
            }
          }
        } while( true );
      }
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_140a8e1b8:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}

