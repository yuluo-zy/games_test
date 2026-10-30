
void __cdecl
system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::
generate_dormer_roofs(void)

{
  byte *pbVar1;
  undefined8 *puVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  float fVar5;
  float fVar6;
  byte bVar7;
  undefined4 uVar8;
  undefined4 *puVar9;
  undefined1 auVar10 [16];
  int iVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  int iVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  undefined1 extraout_AL_02;
  undefined1 extraout_AL_03;
  ushort uVar19;
  uint uVar20;
  uint uVar21;
  int iVar22;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  ulonglong *extraout_RAX_03;
  undefined1 (*extraout_RAX_04) [16];
  byte *extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong lVar23;
  byte *extraout_RAX_07;
  ulonglong extraout_RAX_08;
  longlong lVar24;
  undefined1 *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  undefined8 *extraout_RAX_12;
  ulonglong extraout_RAX_13;
  ulonglong extraout_RAX_14;
  undefined8 *extraout_RAX_15;
  longlong extraout_RAX_16;
  undefined8 *puVar25;
  undefined8 *extraout_RAX_17;
  undefined1 (*extraout_RAX_18) [16];
  ulonglong extraout_RAX_19;
  undefined1 (*pauVar26) [16];
  undefined8 *extraout_RAX_20;
  longlong extraout_RAX_21;
  longlong extraout_RAX_22;
  ulonglong extraout_RAX_23;
  longlong *extraout_RAX_24;
  ulonglong *extraout_RAX_25;
  longlong *extraout_RAX_26;
  uint uVar27;
  ulonglong *puVar28;
  longlong *plVar29;
  undefined1 uVar30;
  char *in_RDX;
  ulonglong uVar31;
  undefined8 *puVar32;
  undefined1 (*pauVar33) [16];
  undefined1 (*pauVar34) [16];
  byte bVar35;
  undefined1 (*pauVar36) [16];
  undefined8 *in_R8;
  char *pcVar37;
  ulonglong uVar38;
  undefined1 *puVar39;
  undefined8 *in_R9;
  ulonglong uVar40;
  longlong lVar41;
  undefined8 *puVar42;
  undefined1 (*pauVar43) [16];
  ulonglong *puVar44;
  bool bVar45;
  char cVar46;
  byte bVar47;
  char cVar48;
  char cVar51;
  char cVar53;
  char cVar54;
  char cVar55;
  undefined4 extraout_XMM0_Da;
  char cVar50;
  char cVar52;
  float fVar56;
  undefined1 auVar49 [16];
  float fVar57;
  float fVar62;
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  undefined1 auVar60 [16];
  undefined1 auVar61 [16];
  float fVar63;
  undefined1 auVar64 [16];
  undefined1 auVar65 [16];
  undefined1 auVar66 [16];
  undefined1 auVar67 [16];
  float fVar68;
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
  float fVar69;
  float fVar70;
  float fVar71;
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  undefined1 unaff_XMM8_Bc;
  undefined1 unaff_XMM8_Bd;
  undefined1 unaff_XMM8_Be;
  char cVar72;
  undefined1 unaff_XMM8_Bf;
  char cVar73;
  undefined1 unaff_XMM8_Bg;
  char cVar74;
  undefined1 unaff_XMM8_Bh;
  char cVar75;
  undefined1 unaff_XMM8_Bi;
  char cVar76;
  undefined1 unaff_XMM8_Bj;
  char cVar77;
  undefined1 unaff_XMM8_Bk;
  char cVar78;
  undefined1 unaff_XMM8_Bl;
  char cVar79;
  undefined1 unaff_XMM8_Bm;
  char cVar80;
  undefined1 unaff_XMM8_Bn;
  char cVar81;
  undefined1 unaff_XMM8_Bo;
  char cVar82;
  undefined1 unaff_XMM8_Bp;
  char cVar83;
  float fVar84;
  float fVar85;
  float fVar86;
  float fVar87;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong *in_stack_00000050;
  float local_418;
  float fStack_414;
  float fStack_410;
  float fStack_40c;
  float local_408;
  float fStack_404;
  float fStack_400;
  float fStack_3fc;
  float fStack_3f8;
  float fStack_3f4;
  undefined8 local_328;
  undefined1 (*pauStack_320) [16];
  undefined1 (*local_318) [16];
  undefined8 uStack_310;
  undefined8 local_308;
  char *pcStack_300;
  undefined8 local_2f8;
  ulonglong local_2e8;
  char *local_2d8;
  ulonglong local_2d0;
  longlong *local_2c8;
  ulonglong *local_2c0;
  ulonglong local_2b0;
  undefined1 (*local_2a8) [16];
  undefined1 (*pauStack_2a0) [16];
  undefined8 local_298;
  ushort uStack_290;
  undefined2 uStack_28e;
  undefined4 uStack_28c;
  undefined4 uStack_288;
  undefined4 uStack_284;
  undefined4 uStack_280;
  undefined4 uStack_27c;
  undefined4 uStack_278;
  undefined4 uStack_274;
  undefined1 (*pauStack_270) [16];
  undefined1 (*pauStack_268) [16];
  undefined8 local_260;
  undefined1 (*pauStack_258) [16];
  longlong local_250;
  char *pcStack_248;
  undefined8 local_240;
  undefined4 local_22c;
  longlong local_228;
  undefined1 (*local_220) [16];
  undefined1 (*local_218) [16];
  undefined8 local_210;
  undefined1 (*local_208) [16];
  undefined1 (*pauStack_200) [16];
  undefined8 local_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  char *pcStack_1e0;
  undefined4 local_1d8;
  undefined4 uStack_1d4;
  longlong local_1c8;
  ulonglong local_1c0;
  undefined1 (*local_1b8) [16];
  undefined8 local_1b0;
  longlong *local_1a8;
  undefined1 (*local_1a0) [16];
  undefined1 (*local_198) [16];
  undefined1 (*pauStack_190) [16];
  undefined1 (*local_188) [16];
  undefined1 (*pauStack_180) [16];
  longlong local_178;
  char *pcStack_170;
  undefined8 local_168;
  longlong local_160;
  longlong local_158;
  undefined8 local_150;
  undefined8 local_148;
  undefined8 *local_140;
  undefined1 (*local_138) [16];
  undefined1 (*local_130) [16];
  undefined1 (*local_128) [16];
  ulonglong local_120;
  ulonglong *local_118;
  ulonglong local_110;
  ulonglong *local_108;
  undefined1 (*local_100) [16];
  char local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_140 = in_R8;
  puffin::are_scopes_on();
  local_f3 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    pcVar37 = 
    "system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::f"
    ;
    if (extraout_RAX == 1) {
      if (in_RDX != (char *)0x0) {
        if (in_RDX < (char *)0x66) {
          if ((char)(&DAT_142a873e0)[(longlong)in_RDX] < -0x40) {
LAB_1407cb276:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != (char *)0x66) goto LAB_1407cb276;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (in_RDX != (char *)0xfffffffffffffffe) {
          if (in_RDX + 2 < (char *)0x66) {
            if ("stem_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::f"
                [(longlong)in_RDX] < -0x40) {
LAB_1407cb2c3:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (in_RDX + 2 != (char *)0x66) goto LAB_1407cb2c3;
        }
        pcVar37 = "stem_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::f"
                  + (longlong)in_RDX;
      }
    }
    in_RDX = pcVar37;
    pcVar37 = "cache of dormers";
    do {
      bVar35 = pcVar37[-1];
      uVar20 = (uint)(char)bVar35;
      if ((char)bVar35 < '\0') {
        bVar47 = pcVar37[-2];
        if ((char)bVar47 < -0x40) {
          bVar7 = pcVar37[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)(pcVar37 + -4);
            pcVar37 = pcVar37 + -4;
            uVar20 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar37 = pcVar37 + -3;
            uVar20 = bVar7 & 0xf;
          }
          uVar20 = bVar47 & 0x3f | uVar20 << 6;
        }
        else {
          pcVar37 = pcVar37 + -2;
          uVar20 = bVar47 & 0x1f;
        }
        uVar20 = (uint)(bVar35 & 0x3f) | uVar20 << 6;
        if (uVar20 != 0x5c) goto LAB_1407cb384;
LAB_1407cb396:
        if (pcVar37 == "") break;
        if (pcVar37 + -0x142a87517 < (char *)0x5b) {
          if (-0x41 < pcVar37[1]) break;
        }
        else if (pcVar37 + -0x142a87517 == (char *)0x5b) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar37 = pcVar37 + -1;
      if (uVar20 == 0x5c) goto LAB_1407cb396;
LAB_1407cb384:
      if (uVar20 == 0x2f) goto LAB_1407cb396;
    } while (pcVar37 !=
             "crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rscache of dormers"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      puVar28 = (ulonglong *)(extraout_RAX_01 + 1);
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), puVar28 = extraout_RAX_25, extraout_RAX_25 == (ulonglong *)0x0))
    goto LAB_1407cd519;
    if (*puVar28 != 0) goto LAB_1407cd5d9;
    *puVar28 = 0xffffffffffffffff;
    local_108 = puVar28;
    puffin::ThreadProfiler::begin_scope();
    *local_108 = *local_108 + 1;
  }
  country_core::resources::walls::decorator_subtype::impl$8::from();
  local_120 = 0;
  local_118 = (ulonglong *)&DAT_00000008;
  local_110 = 0;
  local_f1 = 1;
  local_2d8 = in_RDX;
  puffin::are_scopes_on();
  local_128 = (undefined1 (*) [16])CONCAT71(local_128._1_7_,extraout_AL_00);
  if (extraout_AL_00 == '\0') {
    local_328 = (undefined1 (*) [16])0x0;
  }
  else {
    pcVar37 = "cache of dormers";
    do {
      bVar35 = pcVar37[-1];
      uVar20 = (uint)(char)bVar35;
      if ((char)bVar35 < '\0') {
        bVar47 = pcVar37[-2];
        if ((char)bVar47 < -0x40) {
          bVar7 = pcVar37[-3];
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)(pcVar37 + -4);
            pcVar37 = pcVar37 + -4;
            uVar20 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar37 = pcVar37 + -3;
            uVar20 = bVar7 & 0xf;
          }
          uVar20 = bVar47 & 0x3f | uVar20 << 6;
        }
        else {
          pcVar37 = pcVar37 + -2;
          uVar20 = bVar47 & 0x1f;
        }
        uVar20 = (uint)(bVar35 & 0x3f) | uVar20 << 6;
        if (uVar20 != 0x5c) goto LAB_1407cb524;
LAB_1407cb53d:
        if (pcVar37 == "") break;
        if (pcVar37 + -0x142a87517 < (char *)0x5b) {
          if (-0x41 < pcVar37[1]) break;
        }
        else if (pcVar37 + -0x142a87517 == (char *)0x5b) break;
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar37 = pcVar37 + -1;
      if (uVar20 == 0x5c) goto LAB_1407cb53d;
LAB_1407cb524:
      if (uVar20 == 0x2f) goto LAB_1407cb53d;
    } while (pcVar37 !=
             "crates/systems/decorator/src/decorator_visual/cottage_dormers/assemble_dormer_roof_tiles.rscache of dormers"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 0) {
      local_f1 = 1;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      puVar28 = extraout_RAX_03;
      if (extraout_RAX_03 == (ulonglong *)0x0) goto LAB_1407cd65b;
    }
    else {
      if ((int)*extraout_RAX_02 != 1) {
LAB_1407cd65b:
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      puVar28 = (ulonglong *)(extraout_RAX_02 + 1);
    }
    if (*puVar28 != 0) {
      local_f1 = 1;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *puVar28 = 0xffffffffffffffff;
    in_RDX = "cache of dormers";
    local_108 = puVar28;
    puffin::ThreadProfiler::begin_scope();
    *local_108 = *local_108 + 1;
    local_328 = (undefined1 (*) [16])0x1;
    pauStack_320 = extraout_RAX_04;
  }
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_05 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_05[0] = 1;
    extraout_RAX_05[1] = 0;
    extraout_RAX_05[2] = 0;
    extraout_RAX_05[3] = 0;
    extraout_RAX_05[4] = 0;
    extraout_RAX_05[5] = 0;
    extraout_RAX_05[6] = 0;
    extraout_RAX_05[7] = 0;
    *(longlong *)(extraout_RAX_05 + 8) = extraout_RAX_06;
    *(char **)(extraout_RAX_05 + 0x10) = in_RDX;
    local_178 = extraout_RAX_06;
  }
  else {
    local_178 = *(longlong *)(extraout_RAX_05 + 8);
    in_RDX = *(char **)(extraout_RAX_05 + 0x10);
  }
  *(longlong *)(extraout_RAX_05 + 8) = local_178 + 1;
  local_188 = (undefined1 (*) [16])0x0;
  pauStack_180 = (undefined1 (*) [16])0x0;
  local_198 = (undefined1 (*) [16])&DAT_142a87290;
  pauStack_190 = (undefined1 (*) [16])0x0;
  pcStack_170 = in_RDX;
  country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
  local_108 = (ulonglong *)*in_stack_00000028;
  local_158 = *in_stack_00000030;
  local_160 = in_stack_00000030[1];
  local_1b0 = *in_R9;
  pauVar33 = local_2a8;
  lVar24 = CONCAT44(uStack_284,uStack_288);
  pauVar34 = pauStack_2a0;
  uVar20 = (uint)uStack_290;
  while (lVar24 != 0) {
    local_148 = lVar24 + -1;
    if ((short)uVar20 == 0) {
      do {
        auVar49 = *pauVar34;
        uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
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
        pauVar33 = pauVar33 + -0x98;
        pauVar34 = pauVar34 + 1;
      } while (uVar19 == 0xffff);
      uVar20 = ~(uint)uVar19;
      uVar27 = uVar20 & -(uint)uVar19 - 2;
    }
    else {
      if (pauVar33 == (undefined1 (*) [16])0x0) break;
      uVar27 = uVar20 - 1 & uVar20;
    }
    local_100 = (undefined1 (*) [16])CONCAT44(local_100._4_4_,uVar27);
    uVar27 = 0;
    for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
      uVar27 = uVar27 + 1;
    }
    lVar23 = (ulonglong)uVar27 * -0x98;
    local_130 = *(undefined1 (**) [16])((longlong)pauVar33 + lVar23 + -0x98);
    pauVar26 = *(undefined1 (**) [16])((longlong)pauVar33 + lVar23 + -0x60);
    auVar49 = *pauVar26;
    uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
    pauVar36 = pauVar26 + 1;
    lVar23 = *(longlong *)((longlong)pauVar33 + lVar23 + -0x48);
    local_138 = pauVar33;
    while (lVar23 != 0) {
      if ((short)uVar20 == 0) {
        do {
          auVar49 = *pauVar36;
          uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
          pauVar26 = pauVar26 + -0x50;
          pauVar36 = pauVar36 + 1;
        } while (uVar19 == 0xffff);
        uVar27 = ~(uint)uVar19;
        uVar20 = uVar27 & -(uint)uVar19 - 2;
      }
      else {
        uVar27 = uVar20;
        uVar20 = uVar20 - 1 & uVar20;
      }
      uVar21 = 0;
      for (; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
        uVar21 = uVar21 + 1;
      }
      country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::try_get_subtype();
      lVar23 = lVar23 + -1;
      if (extraout_RAX_07 != (byte *)0x0) {
        bVar35 = *extraout_RAX_07;
        if ((((bVar35 < 0x18) && ((0x800186U >> (bVar35 & 0x1f) & 1) != 0)) ||
            ((bVar35 == 0 && (pauVar26[(ulonglong)uVar21 * -5 + -1][0xc] == '\x03')))) &&
           (((country_core::resources::walls::public_walls::PublicWalls::roof_entity(),
             extraout_RAX_08 >> 0x20 != 0 &&
             (uVar31 = extraout_RAX_08 & 0xffffffff, uVar31 < *(ulonglong *)(local_160 + 0x10))) &&
            (*(int *)(*(longlong *)(local_160 + 8) + uVar31 * 0x14) ==
             (int)(extraout_RAX_08 >> 0x20))))) {
          lVar24 = *(longlong *)(local_160 + 8) + uVar31 * 0x14;
          uVar31 = (ulonglong)*(uint *)(lVar24 + 4);
          if (((uVar31 != 0xffffffff) && (uVar31 < *(ulonglong *)(local_158 + 0x38))) &&
             ((*(ulonglong *)
                (*(longlong *)(local_158 + 0x28) + (ulonglong)(*(uint *)(lVar24 + 4) >> 6) * 8) >>
               (uVar31 & 0x3f) & 1) != 0)) {
            if (*(ulonglong *)(local_158 + 0x110) <
                *(ulonglong *)
                 (*(longlong *)(local_160 + 0x1a8) + 0x40 +
                 (ulonglong)*(uint *)(lVar24 + 0xc) * 0x48)) {
              lVar41 = *(longlong *)(local_160 + 0x1a8) + (ulonglong)*(uint *)(lVar24 + 0xc) * 0x48;
              uVar31 = *(ulonglong *)
                        (*(longlong *)(lVar41 + 0x38) + *(ulonglong *)(local_158 + 0x110) * 8);
              if (uVar31 == 0) goto LAB_1407cb96f;
              local_1b8 = *(undefined1 (**) [16])
                           (*(longlong *)(lVar41 + 0x18) + 0x10 + ~uVar31 * 0x30);
            }
            else {
LAB_1407cb96f:
              local_1b8 = (undefined1 (*) [16])0x0;
            }
            uVar27 = *(uint *)(lVar24 + 0x10);
            country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
            transform();
            lVar24 = (ulonglong)uVar27 * 0x58;
            local_298._0_4_ = uStack_1f0;
            pauStack_2a0 = local_1f8;
            puVar4 = (undefined8 *)((longlong)local_1b8 + lVar24 + 0x24);
            uVar17 = *puVar4;
            uVar18 = puVar4[1];
            auVar49 = *(undefined1 (*) [16])((longlong)local_1b8 + lVar24 + 0x30);
            uStack_27c = auVar49._4_4_;
            uStack_278 = auVar49._8_4_;
            uStack_274 = auVar49._12_4_;
            uStack_28c = (undefined4)uVar17;
            uStack_288 = (undefined4)((ulonglong)uVar17 >> 0x20);
            uStack_284 = (undefined4)uVar18;
            uStack_280 = (undefined4)((ulonglong)uVar18 >> 0x20);
            local_298._4_4_ = *(undefined4 *)((longlong)local_1b8 + lVar24 + 0x4c);
            uVar8 = *(undefined4 *)((longlong)local_1b8 + lVar24 + 0x50);
            local_2a8 = local_130;
            uStack_290 = (ushort)uVar8;
            uStack_28e = (undefined2)((uint)uVar8 >> 0x10);
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::resources::walls::WallId,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::DormerCache>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
          }
        }
      }
    }
    pauVar33 = local_138;
    lVar24 = local_148;
    uVar20 = (uint)local_100;
  }
  auVar49 = *local_198;
  uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
  pauVar33 = local_198 + 1;
  local_130 = (undefined1 (*) [16])(in_stack_00000050 + 4);
  iVar22 = -(uint)(CONCAT13(unaff_XMM6_Bd,
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
  pauVar34 = local_198;
  pauVar36 = pauStack_180;
  while (pauVar36 != (undefined1 (*) [16])0x0) {
    pauVar36 = (undefined1 (*) [16])(pauVar36[-1] + 0xf);
    if ((short)uVar20 == 0) {
      do {
        auVar49 = *pauVar33;
        uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
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
        pauVar34 = pauVar34 + -0x40;
        pauVar33 = pauVar33 + 1;
      } while (uVar19 == 0xffff);
      uVar20 = ~(uint)uVar19;
      uVar27 = uVar20 & -(uint)uVar19 - 2;
    }
    else {
      uVar27 = uVar20 - 1 & uVar20;
    }
    iVar11 = 0;
    for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
      iVar11 = iVar11 + 1;
    }
    uVar21 = iVar11 << 6;
    uVar20 = uVar27;
    if (in_stack_00000050[3] != 0) {
      uVar31 = (ulonglong)local_148 >> 0x20;
      local_148 = CONCAT44((int)uVar31,uVar27);
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      uVar31 = local_110;
      lVar24 = *in_stack_00000050;
      bVar35 = (byte)((ulonglong)extraout_RAX_09 >> 0x39);
      auVar49 = ZEXT216(CONCAT11(bVar35,bVar35));
      auVar49 = pshuflw(auVar49,auVar49,0);
      pauVar26 = (undefined1 (*) [16])0x0;
      puVar39 = extraout_RAX_09;
      while( true ) {
        uVar38 = in_stack_00000050[1] & (ulonglong)puVar39;
        auVar65 = *(undefined1 (*) [16])(lVar24 + uVar38);
        cVar46 = auVar49[0];
        auVar64[0] = -(auVar65[0] == cVar46);
        cVar50 = auVar49[1];
        auVar64[1] = -(auVar65[1] == cVar50);
        cVar52 = auVar49[2];
        auVar64[2] = -(auVar65[2] == cVar52);
        cVar54 = auVar49[3];
        auVar64[3] = -(auVar65[3] == cVar54);
        auVar64[4] = -(auVar65[4] == cVar46);
        auVar64[5] = -(auVar65[5] == cVar50);
        auVar64[6] = -(auVar65[6] == cVar52);
        auVar64[7] = -(auVar65[7] == cVar54);
        auVar64[8] = -(auVar65[8] == cVar46);
        auVar64[9] = -(auVar65[9] == cVar50);
        auVar64[10] = -(auVar65[10] == cVar52);
        auVar64[0xb] = -(auVar65[0xb] == cVar54);
        auVar64[0xc] = -(auVar65[0xc] == cVar46);
        auVar64[0xd] = -(auVar65[0xd] == cVar50);
        auVar64[0xe] = -(auVar65[0xe] == cVar52);
        auVar64[0xf] = -(auVar65[0xf] == cVar54);
        uVar19 = (ushort)(SUB161(auVar64 >> 7,0) & 1) | (ushort)(SUB161(auVar64 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar64 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar64 >> 0x77,0) & 1) << 0xe | (ushort)(auVar64[0xf] >> 7) << 0xf
        ;
        uVar27 = (uint)uVar19;
        uVar20 = (uint)local_148;
        if (uVar19 != 0) {
          do {
            uVar12 = 0;
            for (uVar13 = uVar27; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
              uVar12 = uVar12 + 1;
            }
            uVar40 = uVar12 + uVar38 & in_stack_00000050[1];
            local_138 = pauVar36;
            if (*(int *)((longlong)pauVar34 + (-0x40 - (ulonglong)uVar21)) ==
                *(int *)(lVar24 + -0x40 + uVar40 * -0x40)) {
              lVar41 = uVar40 * -0x40;
              lVar23 = lVar24 + lVar41;
              local_100 = *(undefined1 (**) [16])((longlong)pauVar34 + (-0x38 - (ulonglong)uVar21));
              if (local_100 != *(undefined1 (**) [16])(lVar24 + -0x38 + lVar41)) {
                if (local_110 == local_120) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                            ();
                }
                local_118[uVar31] = (ulonglong)local_100;
                local_110 = uVar31 + 1;
                uVar38 = *(ulonglong *)(lVar23 + -0x38);
                if (local_110 == local_120) {
                  alloc::raw_vec::RawVec::
                  grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                            ();
                }
                local_118[uVar31 + 1] = uVar38;
                local_110 = uVar31 + 2;
                goto LAB_1407cbdb7;
              }
              fVar57 = *(float *)((longlong)pauVar34 + (-0x30 - (ulonglong)uVar21));
              if ((fVar57 == *(float *)(lVar23 + -0x30)) &&
                 (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x30)))) {
                fVar57 = *(float *)((longlong)pauVar34 + (-0x2c - (ulonglong)uVar21));
                if ((fVar57 == *(float *)(lVar23 + -0x2c)) &&
                   (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x2c)))) {
                  fVar57 = *(float *)((longlong)pauVar34 + (-0x28 - (ulonglong)uVar21));
                  if ((fVar57 == *(float *)(lVar23 + -0x28)) &&
                     (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x28)))) {
                    fVar57 = *(float *)((longlong)pauVar34 + (-0x14 - (ulonglong)uVar21));
                    if ((fVar57 == *(float *)(lVar23 + -0x14)) &&
                       (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x14)))) {
                      fVar57 = *(float *)((longlong)pauVar34 + (-0x10 - (ulonglong)uVar21));
                      if ((fVar57 == *(float *)(lVar23 + -0x10)) &&
                         (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x10)))) {
                        fVar57 = *(float *)((longlong)pauVar34 + (-0x1c - (ulonglong)uVar21));
                        if ((fVar57 == *(float *)(lVar23 + -0x1c)) &&
                           (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x1c)))) {
                          fVar57 = *(float *)((longlong)pauVar34 + (-0x18 - (ulonglong)uVar21));
                          if ((fVar57 == *(float *)(lVar23 + -0x18)) &&
                             (!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x18)))) {
                            fVar57 = *(float *)((longlong)pauVar34 + (-0xc - (ulonglong)uVar21));
                            if ((fVar57 == *(float *)(lVar23 + -0xc)) &&
                               ((!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0xc)) &&
                                (*(char *)((longlong)pauVar34 + (-4 - (ulonglong)uVar21)) ==
                                 *(char *)(lVar23 + -4))))) {
                              fVar57 = *(float *)((longlong)pauVar34 + (-8 - (ulonglong)uVar21));
                              if ((fVar57 == *(float *)(lVar23 + -8)) &&
                                 ((!NAN(fVar57) && !NAN(*(float *)(lVar23 + -8)) &&
                                  (*(char *)((longlong)pauVar34 + (-3 - (ulonglong)uVar21)) ==
                                   *(char *)(lVar23 + -3))))) {
                                fVar57 = *(float *)((longlong)pauVar34 + (-0x24 - (ulonglong)uVar21)
                                                   );
                                if ((fVar57 == *(float *)(lVar23 + -0x24)) &&
                                   ((!NAN(fVar57) && !NAN(*(float *)(lVar23 + -0x24)) &&
                                    (*(int *)((longlong)pauVar34 + (-0x20 - (ulonglong)uVar21)) ==
                                     *(int *)(lVar23 + -0x20))))) goto LAB_1407cbdb7;
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
              if (local_110 == local_120) {
                alloc::raw_vec::RawVec::
                grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                          ();
              }
              local_118[uVar31] = (ulonglong)local_100;
              local_110 = uVar31 + 1;
              goto LAB_1407cbdb7;
            }
            uVar19 = (ushort)(uVar27 - 1) & (ushort)uVar27;
            uVar27 = CONCAT22((short)(uVar27 - 1 >> 0x10),uVar19);
            local_100 = pauVar26;
          } while (uVar19 != 0);
        }
        auVar58[0] = -(auVar65[0] == (char)iVar22);
        auVar58[1] = -(auVar65[1] == (char)((uint)iVar22 >> 8));
        auVar58[2] = -(auVar65[2] == (char)((uint)iVar22 >> 0x10));
        auVar58[3] = -(auVar65[3] == (char)((uint)iVar22 >> 0x18));
        auVar58[4] = -(auVar65[4] == (char)iVar14);
        auVar58[5] = -(auVar65[5] == (char)((uint)iVar14 >> 8));
        auVar58[6] = -(auVar65[6] == (char)((uint)iVar14 >> 0x10));
        auVar58[7] = -(auVar65[7] == (char)((uint)iVar14 >> 0x18));
        auVar58[8] = -(auVar65[8] == (char)iVar15);
        auVar58[9] = -(auVar65[9] == (char)((uint)iVar15 >> 8));
        auVar58[10] = -(auVar65[10] == (char)((uint)iVar15 >> 0x10));
        auVar58[0xb] = -(auVar65[0xb] == (char)((uint)iVar15 >> 0x18));
        auVar58[0xc] = -(auVar65[0xc] == (char)iVar16);
        auVar58[0xd] = -(auVar65[0xd] == (char)((uint)iVar16 >> 8));
        auVar58[0xe] = -(auVar65[0xe] == (char)((uint)iVar16 >> 0x10));
        auVar58[0xf] = -(auVar65[0xf] == (char)((uint)iVar16 >> 0x18));
        if ((((((((((((((((SUB161(auVar58 >> 7,0) & 1) != 0 || (SUB161(auVar58 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar58 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar58 >> 0x1f,0) & 1) != 0) || (SUB161(auVar58 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar58 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar58 >> 0x37,0) & 1) != 0) || (SUB161(auVar58 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar58 >> 0x47,0) & 1) != 0) || (SUB161(auVar58 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar58 >> 0x57,0) & 1) != 0) || (SUB161(auVar58 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar58 >> 0x67,0) & 1) != 0) || (SUB161(auVar58 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar58 >> 0x77,0) & 1) != 0) || auVar58[0xf] < '\0') break;
        puVar39 = pauVar26[1] + uVar38;
        pauVar26 = pauVar26 + 1;
      }
    }
    uVar38 = local_110;
    uVar31 = *(ulonglong *)((longlong)pauVar34 + (-0x38 - (ulonglong)uVar21));
    if (local_110 == local_120) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    local_118[uVar38] = uVar31;
    local_110 = uVar38 + 1;
LAB_1407cbdb7:
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::resources::walls::WallId,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::DormerCache>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::resources::walls::WallId,system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::DormerCache>_>_>
              ();
  }
  pauVar34 = (undefined1 (*) [16])*in_stack_00000050;
  lVar24 = in_stack_00000050[3];
  auVar49 = *pauVar34;
  uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
  pauVar33 = pauVar34 + 1;
  while (uVar31 = local_110, bVar45 = lVar24 != 0, lVar24 = lVar24 + -1, bVar45) {
    if ((short)uVar20 == 0) {
      do {
        auVar49 = *pauVar33;
        uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) | (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1
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
        pauVar34 = pauVar34 + -0x40;
        pauVar33 = pauVar33 + 1;
      } while (uVar19 == 0xffff);
      uVar27 = ~(uint)uVar19;
      uVar20 = uVar27 & -(uint)uVar19 - 2;
    }
    else {
      uVar27 = uVar20;
      uVar20 = uVar20 - 1 & uVar20;
    }
    iVar22 = 0;
    for (; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
      iVar22 = iVar22 + 1;
    }
    uVar38 = *(ulonglong *)((longlong)pauVar34 + (-0x38 - (ulonglong)(uint)(iVar22 << 6)));
    if (local_110 == local_120) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    local_118[uVar31] = uVar38;
    local_110 = uVar31 + 1;
  }
  uStack_288 = (undefined4)local_178;
  uStack_284 = (undefined4)((ulonglong)local_178 >> 0x20);
  uStack_280 = SUB84(pcStack_170,0);
  uStack_27c = (undefined4)((ulonglong)pcStack_170 >> 0x20);
  local_298._0_4_ = SUB84(local_188,0);
  local_298._4_4_ = (undefined4)((ulonglong)local_188 >> 0x20);
  uStack_290 = (ushort)pauStack_180;
  uStack_28e = (undefined2)((ulonglong)pauStack_180 >> 0x10);
  uStack_28c = (undefined4)((ulonglong)pauStack_180 >> 0x20);
  local_2a8 = local_198;
  pauStack_2a0 = pauStack_190;
  if ((in_stack_00000050[1] != 0) && (in_stack_00000050[1] * 0x41 != -0x51)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  in_stack_00000050[4] = CONCAT44(uStack_284,uStack_288);
  in_stack_00000050[5] = CONCAT44(uStack_27c,uStack_280);
  in_stack_00000050[2] = CONCAT44(local_298._4_4_,(undefined4)local_298);
  in_stack_00000050[3] = CONCAT44(uStack_28c,CONCAT22(uStack_28e,uStack_290));
  *in_stack_00000050 = (longlong)local_2a8;
  in_stack_00000050[1] = (longlong)pauStack_2a0;
  if ((char)local_128 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_10 == 0) {
      local_f1 = 1;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar29 = extraout_RAX_11;
      if (extraout_RAX_11 == (longlong *)0x0) goto LAB_1407cd673;
    }
    else {
      if ((int)*extraout_RAX_10 != 1) {
LAB_1407cd673:
        local_f1 = 1;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar29 = extraout_RAX_10 + 1;
    }
    if (*plVar29 != 0) {
      local_f1 = 1;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar29 = -1;
    local_130 = (undefined1 (*) [16])plVar29;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_130 = *(longlong *)local_130 + 1;
  }
  uVar31 = local_110;
  if (1 < local_110) {
    if (local_110 < 0x15) {
      puVar28 = local_118 + 1;
      lVar24 = 8;
      do {
        uVar38 = puVar28[-1];
        uVar31 = *puVar28;
        lVar23 = lVar24;
        if (uVar31 < uVar38) {
          do {
            *(ulonglong *)((longlong)local_118 + lVar23) = uVar38;
            puVar44 = local_118;
            if (lVar23 == 8) goto LAB_1407cd561;
            uVar38 = *(ulonglong *)((longlong)local_118 + lVar23 + -0x10);
            lVar23 = lVar23 + -8;
          } while (uVar31 < uVar38);
          puVar44 = (ulonglong *)(lVar23 + (longlong)local_118);
LAB_1407cd561:
          *puVar44 = uVar31;
        }
        puVar28 = puVar28 + 1;
        lVar24 = lVar24 + 8;
      } while (puVar28 != local_118 + local_110);
    }
    else {
      local_f1 = 1;
      core::slice::sort::stable::
      driftsort_main<country_core::resources::walls::WallId,bool_(*)(ref_<country_core::resources::walls::WallId>,ref_<country_core::resources::walls::WallId>),alloc::vec::Vec<country_core::resources::walls::WallId,alloc::alloc::Global>_>
                ();
    }
    uVar31 = local_110;
    if (1 < local_110) {
      uVar38 = 1;
      do {
        uVar40 = uVar38;
        if (local_118[uVar38] == local_118[uVar38 - 1]) goto LAB_1407cd70a;
        uVar38 = uVar38 + 1;
      } while (local_110 != uVar38);
    }
  }
  goto LAB_1407cbfc3;
LAB_1407cc95b:
  uVar38 = uVar31 & uVar38;
  pcVar37 = (char *)(local_1c0 + uVar38);
  cVar48 = auVar49[0];
  auVar67[0] = -(*pcVar37 == cVar48);
  cVar51 = auVar49[1];
  auVar67[1] = -(pcVar37[1] == cVar51);
  cVar53 = auVar49[2];
  auVar67[2] = -(pcVar37[2] == cVar53);
  cVar55 = auVar49[3];
  auVar67[3] = -(pcVar37[3] == cVar55);
  auVar67[4] = -(pcVar37[4] == cVar48);
  auVar67[5] = -(pcVar37[5] == cVar51);
  auVar67[6] = -(pcVar37[6] == cVar53);
  auVar67[7] = -(pcVar37[7] == cVar55);
  auVar67[8] = -(pcVar37[8] == cVar48);
  auVar67[9] = -(pcVar37[9] == cVar51);
  auVar67[10] = -(pcVar37[10] == cVar53);
  auVar67[0xb] = -(pcVar37[0xb] == cVar55);
  auVar67[0xc] = -(pcVar37[0xc] == cVar48);
  auVar67[0xd] = -(pcVar37[0xd] == cVar51);
  auVar67[0xe] = -(pcVar37[0xe] == cVar53);
  auVar67[0xf] = -(pcVar37[0xf] == cVar55);
  uVar19 = (ushort)(SUB161(auVar67 >> 7,0) & 1) | (ushort)(SUB161(auVar67 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar67 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar67 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar67 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar67 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar67 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar67 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar67 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar67 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar67 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar67 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar67 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar67 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar67 >> 0x77,0) & 1) << 0xe | (ushort)(auVar67[0xf] >> 7) << 0xf;
  uVar27 = (uint)uVar19;
  while( true ) {
    if (uVar19 == 0) goto LAB_1407cc9af;
    uVar21 = 0;
    for (uVar12 = uVar27; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      uVar21 = uVar21 + 1;
    }
    if (local_2b0 == *(ulonglong *)((longlong)local_1a0 + (uVar21 + uVar38 & uVar31) * -0x2a8))
    break;
    uVar19 = (ushort)(uVar27 - 1) & (ushort)uVar27;
    uVar27 = CONCAT22((short)(uVar27 - 1 >> 0x10),uVar19);
  }
  goto LAB_1407cca3a;
LAB_1407cc9af:
  auVar61[0] = -(*pcVar37 == cVar46);
  auVar61[1] = -(pcVar37[1] == cVar50);
  auVar61[2] = -(pcVar37[2] == cVar52);
  auVar61[3] = -(pcVar37[3] == cVar54);
  auVar61[4] = -(pcVar37[4] == cVar72);
  auVar61[5] = -(pcVar37[5] == cVar73);
  auVar61[6] = -(pcVar37[6] == cVar74);
  auVar61[7] = -(pcVar37[7] == cVar75);
  auVar61[8] = -(pcVar37[8] == cVar76);
  auVar61[9] = -(pcVar37[9] == cVar77);
  auVar61[10] = -(pcVar37[10] == cVar78);
  auVar61[0xb] = -(pcVar37[0xb] == cVar79);
  auVar61[0xc] = -(pcVar37[0xc] == cVar80);
  auVar61[0xd] = -(pcVar37[0xd] == cVar81);
  auVar61[0xe] = -(pcVar37[0xe] == cVar82);
  auVar61[0xf] = -(pcVar37[0xf] == cVar83);
  if ((((((((((((((((SUB161(auVar61 >> 7,0) & 1) != 0 || (SUB161(auVar61 >> 0xf,0) & 1) != 0) ||
                  (SUB161(auVar61 >> 0x17,0) & 1) != 0) || (SUB161(auVar61 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar61 >> 0x27,0) & 1) != 0) || (SUB161(auVar61 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar61 >> 0x37,0) & 1) != 0) || (SUB161(auVar61 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar61 >> 0x47,0) & 1) != 0) || (SUB161(auVar61 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar61 >> 0x57,0) & 1) != 0) || (SUB161(auVar61 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar61 >> 0x67,0) & 1) != 0) || (SUB161(auVar61 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar61 >> 0x77,0) & 1) != 0) || auVar61[0xf] < '\0') {
LAB_1407cd4c3:
    local_f2 = 1;
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  uVar38 = uVar38 + lVar23 + 0x10;
  lVar23 = lVar23 + 0x10;
  goto LAB_1407cc95b;
LAB_1407cca3a:
  local_f2 = 1;
  gothic_wall_window::get_gothic_hat_bones();
  local_1f8 = local_188;
  local_208 = local_198;
  pauStack_200 = pauStack_190;
  local_f2 = 0;
  gothic_wall_window::calc_gothic_half_dormer_profile_ws();
LAB_1407ccff2:
  util_assemble_gothic_dormer::
  assemble_gothic_dormer_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_6>
            ();
  if (local_2a8 != (undefined1 (*) [16])0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (CONCAT44(uStack_28c,CONCAT22(uStack_28e,uStack_290)) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (pauStack_270 != (undefined1 (*) [16])0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (pauStack_258 != (undefined1 (*) [16])0x0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((byte)(bVar35 - 1) < 2) {
LAB_1407cd0c0:
    country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
    transform();
    if (2 < (byte)(local_100[-1][0xc] - 1)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (bVar35 == 1) {
      country_core::resources::walls::public_walls::PublicWalls::get();
      if (extraout_RAX_21 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      country_core::resources::walls::decorator::DecoratorRank::new();
      uVar30 = extraout_AL_02;
      country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
      local_328._0_2_ = CONCAT11(uVar30,extraout_AL_03);
      std::collections::hash::map::HashMap::
      get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                ();
      if (extraout_RAX_22 == 0) {
        local_198 = (undefined1 (*) [16])&local_328;
        pauStack_190 = (undefined1 (*) [16])
                       country_core::resources::walls::decorator_subtype::impl$25::fmt;
        local_2a8 = (undefined1 (*) [16])
                    &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
        pauStack_2a0 = (undefined1 (*) [16])0x1;
        uStack_288 = 0;
        uStack_284 = 0;
        local_298 = &local_198;
        uStack_290 = 1;
        uStack_28e = 0;
        uStack_28c = 0;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      local_1f8 = (undefined1 (*) [16])CONCAT44(extraout_XMM0_Da,(undefined4)local_1f8);
      cVar48 = local_100[-1][0xc];
      if (((cVar48 != '\x01') && (cVar48 != '\x02')) && (cVar48 != '\x03')) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
    }
    assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_7>
              ();
  }
  goto LAB_1407cd250;
LAB_1407cd70a:
  while (uVar40 = uVar40 + 1, uVar31 = uVar38, uVar40 < local_110) {
    if (local_118[uVar40] != local_118[uVar38 - 1]) {
      local_118[uVar38] = local_118[uVar40];
      uVar38 = uVar38 + 1;
    }
  }
LAB_1407cbfc3:
  local_110 = uVar31;
  local_f1 = 1;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_12 != (undefined8 *)0x0) {
    if ((extraout_RAX_12[3] != 0x35) ||
       (iVar22 = memcmp(&anon_8b5f636f5ecef8ae31f545d005a6bf08_5_llvm_10917514084104779652,
                        (void *)extraout_RAX_12[2],0x35), iVar22 != 0)) {
      local_2a8 = (undefined1 (*) [16])0x0;
      local_f1 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_f1 = 1;
    local_1c8 = (**(code **)(extraout_RAX_12[1] + 0x18))(*extraout_RAX_12);
    if (local_1c8 != 0) {
      local_2d0 = local_120;
      local_130 = (undefined1 (*) [16])(local_118 + local_110);
      local_22c = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
      local_1a8 = (longlong *)*in_stack_00000048;
      puVar9 = (undefined4 *)in_stack_00000048[2];
      local_138 = (undefined1 (*) [16])*local_140;
      local_210 = *in_stack_00000040;
      local_2c8 = local_1a8 + 4;
      local_1b8 = local_138 + 2;
      iVar22 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                                CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                      CONCAT13(unaff_XMM8_Bd,
                               CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
      cVar46 = (char)iVar22;
      cVar50 = (char)((uint)iVar22 >> 8);
      cVar52 = (char)((uint)iVar22 >> 0x10);
      cVar54 = (char)((uint)iVar22 >> 0x18);
      iVar22 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                                CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                      CONCAT13(unaff_XMM8_Bh,
                               CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
      cVar72 = (char)iVar22;
      cVar73 = (char)((uint)iVar22 >> 8);
      cVar74 = (char)((uint)iVar22 >> 0x10);
      cVar75 = (char)((uint)iVar22 >> 0x18);
      iVar22 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                                CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                      CONCAT13(unaff_XMM8_Bl,
                               CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
      cVar76 = (char)iVar22;
      cVar77 = (char)((uint)iVar22 >> 8);
      cVar78 = (char)((uint)iVar22 >> 0x10);
      cVar79 = (char)((uint)iVar22 >> 0x18);
      iVar22 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                                CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                      CONCAT13(unaff_XMM8_Bp,
                               CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
      cVar80 = (char)iVar22;
      cVar81 = (char)((uint)iVar22 >> 8);
      cVar82 = (char)((uint)iVar22 >> 0x10);
      cVar83 = (char)((uint)iVar22 >> 0x18);
      local_2c0 = local_118;
      puVar28 = local_118;
      while ((undefined1 (*) [16])puVar28 != local_130) {
        local_2e8 = *puVar28;
        country_core::resources::walls::public_walls::PublicWalls::roof_entity();
        puVar28 = puVar28 + 1;
        if (((extraout_RAX_13 >> 0x20 != 0) &&
            (uVar31 = extraout_RAX_13 & 0xffffffff, uVar31 < *(ulonglong *)(local_160 + 0x10))) &&
           (*(int *)(*(longlong *)(local_160 + 8) + uVar31 * 0x14) == (int)(extraout_RAX_13 >> 0x20)
           )) {
          lVar24 = *(longlong *)(local_160 + 8) + uVar31 * 0x14;
          uVar31 = (ulonglong)*(uint *)(lVar24 + 4);
          if (((uVar31 != 0xffffffff) && (uVar31 < *(ulonglong *)(local_158 + 0x38))) &&
             ((*(ulonglong *)
                (*(longlong *)(local_158 + 0x28) + (ulonglong)(*(uint *)(lVar24 + 4) >> 6) * 8) >>
               (uVar31 & 0x3f) & 1) != 0)) {
            if (*(ulonglong *)(local_158 + 0x110) <
                *(ulonglong *)
                 (*(longlong *)(local_160 + 0x1a8) + 0x40 +
                 (ulonglong)*(uint *)(lVar24 + 0xc) * 0x48)) {
              lVar23 = *(longlong *)(local_160 + 0x1a8) + (ulonglong)*(uint *)(lVar24 + 0xc) * 0x48;
              uVar31 = *(ulonglong *)
                        (*(longlong *)(lVar23 + 0x38) + *(ulonglong *)(local_158 + 0x110) * 8);
              if (uVar31 == 0) goto LAB_1407cc1f3;
              lVar23 = *(longlong *)(*(longlong *)(lVar23 + 0x18) + 0x10 + ~uVar31 * 0x30);
            }
            else {
LAB_1407cc1f3:
              lVar23 = 0;
            }
            lVar24 = (ulonglong)*(uint *)(lVar24 + 0x10) * 0x58;
            local_140 = (undefined8 *)(lVar23 + lVar24);
            *puVar9 = local_22c;
            pauVar33 = *(undefined1 (**) [16])(lVar23 + lVar24);
            local_2a8 = pauVar33;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
            lVar24 = *local_1a8;
            bVar35 = (byte)(extraout_RAX_14 >> 0x38);
            bVar47 = bVar35 >> 1;
            auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
            auVar49 = pshuflw(auVar49,auVar49,0);
            lVar23 = 0;
            uVar31 = extraout_RAX_14;
            while( true ) {
              uVar31 = local_1a8[1] & uVar31;
              pcVar37 = (char *)(lVar24 + uVar31);
              cVar48 = auVar49[0];
              auVar65[0] = -(*pcVar37 == cVar48);
              cVar51 = auVar49[1];
              auVar65[1] = -(pcVar37[1] == cVar51);
              cVar53 = auVar49[2];
              auVar65[2] = -(pcVar37[2] == cVar53);
              cVar55 = auVar49[3];
              auVar65[3] = -(pcVar37[3] == cVar55);
              auVar65[4] = -(pcVar37[4] == cVar48);
              auVar65[5] = -(pcVar37[5] == cVar51);
              auVar65[6] = -(pcVar37[6] == cVar53);
              auVar65[7] = -(pcVar37[7] == cVar55);
              auVar65[8] = -(pcVar37[8] == cVar48);
              auVar65[9] = -(pcVar37[9] == cVar51);
              auVar65[10] = -(pcVar37[10] == cVar53);
              auVar65[0xb] = -(pcVar37[0xb] == cVar55);
              auVar65[0xc] = -(pcVar37[0xc] == cVar48);
              auVar65[0xd] = -(pcVar37[0xd] == cVar51);
              auVar65[0xe] = -(pcVar37[0xe] == cVar53);
              auVar65[0xf] = -(pcVar37[0xf] == cVar55);
              uVar19 = (ushort)(SUB161(auVar65 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar65 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar65 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar65 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar65 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar65 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar65 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar65 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar65 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar65 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar65 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar65 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar65 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar65 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar65 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar65[0xf] >> 7) << 0xf;
              uVar20 = (uint)uVar19;
              while (uVar19 != 0) {
                uVar27 = 0;
                for (uVar21 = uVar20; (uVar21 & 1) == 0; uVar21 = uVar21 >> 1 | 0x80000000) {
                  uVar27 = uVar27 + 1;
                }
                uVar38 = uVar27 + uVar31 & local_1a8[1];
                if (*(undefined1 (**) [16])(lVar24 + -0x18 + uVar38 * -0x18) == pauVar33) {
                  lVar24 = *(longlong *)(lVar24 + uVar38 * -0x18 + -8);
                  goto joined_r0x0001407cc2da;
                }
                uVar19 = (ushort)(uVar20 - 1) & (ushort)uVar20;
                uVar20 = CONCAT22((short)(uVar20 - 1 >> 0x10),uVar19);
              }
              auVar59[0] = -(*pcVar37 == cVar46);
              auVar59[1] = -(pcVar37[1] == cVar50);
              auVar59[2] = -(pcVar37[2] == cVar52);
              auVar59[3] = -(pcVar37[3] == cVar54);
              auVar59[4] = -(pcVar37[4] == cVar72);
              auVar59[5] = -(pcVar37[5] == cVar73);
              auVar59[6] = -(pcVar37[6] == cVar74);
              auVar59[7] = -(pcVar37[7] == cVar75);
              auVar59[8] = -(pcVar37[8] == cVar76);
              auVar59[9] = -(pcVar37[9] == cVar77);
              auVar59[10] = -(pcVar37[10] == cVar78);
              auVar59[0xb] = -(pcVar37[0xb] == cVar79);
              auVar59[0xc] = -(pcVar37[0xc] == cVar80);
              auVar59[0xd] = -(pcVar37[0xd] == cVar81);
              auVar59[0xe] = -(pcVar37[0xe] == cVar82);
              auVar59[0xf] = -(pcVar37[0xf] == cVar83);
              if ((((((((((((((((SUB161(auVar59 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar59 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar59 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar59 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar59 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar59 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar59 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar59 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar59 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar59 >> 0x4f,0) & 1) != 0) || (SUB161(auVar59 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar59 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar59 >> 0x67,0) & 1) != 0) || (SUB161(auVar59 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar59 >> 0x77,0) & 1) != 0) || auVar59[0xf] < '\0') break;
              uVar31 = uVar31 + lVar23 + 0x10;
              lVar23 = lVar23 + 0x10;
            }
            if (local_1a8[2] == 0) {
              hashbrown::raw::RawTable::
              reserve_rehash<tuple_<bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<bevy_ecs::entity::Entity,country_core::render::nani::InstanceHandle,std::hash::random::RandomState>_>
                        ();
            }
            std::alloc::__default_lib_allocator::__rust_alloc();
            if (extraout_RAX_15 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
              alloc::alloc::handle_alloc_error();
            }
            *extraout_RAX_15 = 0;
            extraout_RAX_15[1] = 4;
            extraout_RAX_15[2] = 0;
            *(undefined1 *)(extraout_RAX_15 + 3) = 0;
            lVar24 = *(longlong *)(local_1c8 + 0x60);
LAB_1407cc352:
            lVar23 = *(longlong *)(lVar24 + 8);
            do {
              if (lVar23 == -1) goto LAB_1407cc352;
              if (lVar23 < 0) {
                    // WARNING: Subroutine does not return
                alloc::sync::impl$21::downgrade::panic_cold_display<ref_<str_>_>();
              }
              LOCK();
              lVar41 = *(longlong *)(lVar24 + 8);
              bVar45 = lVar23 == lVar41;
              if (bVar45) {
                *(longlong *)(lVar24 + 8) = lVar23 + 1;
                lVar41 = lVar23;
              }
              UNLOCK();
              lVar23 = lVar41;
            } while (!bVar45);
            pauVar34 = (undefined1 (*) [16])*local_1a8;
            uVar31 = local_1a8[1];
            uVar38 = extraout_RAX_14 & uVar31;
            auVar49 = *(undefined1 (*) [16])(*pauVar34 + uVar38);
            uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
            uVar20 = (uint)uVar19;
            if (uVar19 == 0) {
              lVar23 = 0x10;
              do {
                uVar38 = uVar38 + lVar23 & uVar31;
                auVar49 = *(undefined1 (*) [16])(*pauVar34 + uVar38);
                lVar23 = lVar23 + 0x10;
                uVar20 = (uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
              } while (uVar20 == 0);
            }
            uVar27 = 0;
            for (; (uVar20 & 1) == 0; uVar20 = uVar20 >> 1 | 0x80000000) {
              uVar27 = uVar27 + 1;
            }
            uVar38 = uVar27 + uVar38 & uVar31;
            bVar47 = (*pauVar34)[uVar38];
            if (-1 < (char)bVar47) {
              auVar49 = *pauVar34;
              uVar20 = 0;
              for (uVar27 = (uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                  (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
                uVar20 = uVar20 + 1;
              }
              uVar38 = (ulonglong)uVar20;
              bVar47 = (*pauVar34)[uVar38];
            }
            bVar35 = bVar35 >> 1;
            (*pauVar34)[uVar38] = bVar35;
            pauVar34[1][uVar38 - 0x10 & uVar31] = bVar35;
            local_1a8[2] = local_1a8[2] - (ulonglong)(bVar47 & 1);
            *(undefined1 (**) [16])((longlong)pauVar34 + uVar38 * -0x18 + -0x18) = pauVar33;
            *(longlong *)((longlong)pauVar34 + uVar38 * -0x18 + -0x10) = lVar24;
            *(undefined8 **)((longlong)pauVar34 + uVar38 * -0x18 + -8) = extraout_RAX_15;
            local_1a8[3] = local_1a8[3] + 1;
            lVar24 = *(longlong *)((longlong)pauVar34 + uVar38 * -0x18 + -8);
joined_r0x0001407cc2da:
            if (lVar24 == 0) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            LOCK();
            bVar45 = *(char *)(lVar24 + 0x18) == '\0';
            if (bVar45) {
              *(char *)(lVar24 + 0x18) = '\x01';
            }
            UNLOCK();
            if (!bVar45) {
              parking_lot::raw_mutex::RawMutex::lock_slow();
            }
            parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
            if ((*(longlong *)(lVar24 + 0x10) * 4 + *(longlong *)(lVar24 + 8) == 4 ||
                 *(longlong *)(lVar24 + 0x10) == 0) && (*(longlong *)(local_1c8 + 0x28) == 0)) {
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
            country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
            clear<country_core::resources::roof_tile_ssbo::RoofTileSsbo>();
            local_2b0 = local_2e8;
            country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators();
            puVar4 = local_140;
            if (extraout_RAX_16 != 0) {
              pauVar34 = *(undefined1 (**) [16])(extraout_RAX_16 + 0x30);
              local_148 = *(longlong *)(extraout_RAX_16 + 0x48);
              auVar49 = *pauVar34;
              uVar20 = ~(uint)(ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
              pauVar33 = pauVar34 + 1;
              puVar2 = local_140 + 1;
              puVar3 = (undefined8 *)((longlong)local_140 + 0x24);
              puVar25 = local_140 + 8;
LAB_1407cd250:
              if (local_148 != 0) {
                if ((short)uVar20 == 0) {
                  do {
                    auVar49 = *pauVar33;
                    uVar19 = (ushort)(SUB161(auVar49 >> 7,0) & 1) |
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
                    pauVar34 = pauVar34 + -0x50;
                    pauVar33 = pauVar33 + 1;
                  } while (uVar19 == 0xffff);
                  uVar27 = ~(uint)uVar19;
                  uVar20 = uVar27 & -(uint)uVar19 - 2;
                }
                else {
                  uVar27 = uVar20;
                  uVar20 = uVar20 - 1 & uVar20;
                }
                uVar21 = 0;
                for (; (uVar27 & 1) == 0; uVar27 = uVar27 >> 1 | 0x80000000) {
                  uVar21 = uVar21 + 1;
                }
                uVar31 = (ulonglong)uVar21;
                iVar22 = *(int *)pauVar34[uVar31 * -5 + -5];
                country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
                local_148 = local_148 + -1;
                local_2a8 = (undefined1 (*) [16])CONCAT44(local_2a8._4_4_,iVar22);
                if (*(longlong *)(local_138[1] + 8) != 0) {
                  local_100 = pauVar34 + uVar31 * -5;
                  local_128 = (undefined1 (*) [16])(pauVar34[uVar31 * -5 + -5] + 8);
                  core::hash::BuildHasher::
                  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                            ();
                  bVar35 = (byte)(extraout_RAX_23 >> 0x39);
                  auVar49 = ZEXT216(CONCAT11(bVar35,bVar35));
                  auVar49 = pshuflw(auVar49,auVar49,0);
                  lVar23 = 0;
                  uVar31 = extraout_RAX_23;
                  do {
                    uVar31 = *(ulonglong *)(*local_138 + 8) & uVar31;
                    pcVar37 = (char *)(*(longlong *)*local_138 + uVar31);
                    cVar48 = auVar49[0];
                    auVar66[0] = -(*pcVar37 == cVar48);
                    cVar51 = auVar49[1];
                    auVar66[1] = -(pcVar37[1] == cVar51);
                    cVar53 = auVar49[2];
                    auVar66[2] = -(pcVar37[2] == cVar53);
                    cVar55 = auVar49[3];
                    auVar66[3] = -(pcVar37[3] == cVar55);
                    auVar66[4] = -(pcVar37[4] == cVar48);
                    auVar66[5] = -(pcVar37[5] == cVar51);
                    auVar66[6] = -(pcVar37[6] == cVar53);
                    auVar66[7] = -(pcVar37[7] == cVar55);
                    auVar66[8] = -(pcVar37[8] == cVar48);
                    auVar66[9] = -(pcVar37[9] == cVar51);
                    auVar66[10] = -(pcVar37[10] == cVar53);
                    auVar66[0xb] = -(pcVar37[0xb] == cVar55);
                    auVar66[0xc] = -(pcVar37[0xc] == cVar48);
                    auVar66[0xd] = -(pcVar37[0xd] == cVar51);
                    auVar66[0xe] = -(pcVar37[0xe] == cVar53);
                    auVar66[0xf] = -(pcVar37[0xf] == cVar55);
                    uVar19 = (ushort)(SUB161(auVar66 >> 7,0) & 1) |
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
                    uVar27 = (uint)uVar19;
                    while (uVar19 != 0) {
                      uVar21 = 0;
                      for (uVar12 = uVar27; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
                        uVar21 = uVar21 + 1;
                      }
                      if (*(int *)(*(longlong *)*local_138 + -0x10 +
                                  (uVar21 + uVar31 & *(ulonglong *)(*local_138 + 8)) * -0x10) ==
                          iVar22) {
                        bVar35 = (byte)extraout_EAX;
                        if ((bVar35 == 0) && (local_100[-1][0xc] == '\x03')) {
                          country_core::resources::walls::cached_decorator_transforms::
                          CachedDecoratorTransforms::transform();
                          if (2 < (byte)(local_100[-1][0xc] - 1)) {
                    // WARNING: Subroutine does not return
                            core::panicking::panic();
                          }
                          local_1f8 = (undefined1 (*) [16])
                                      CONCAT44(local_1f8._4_4_ + 0.8,(undefined4)local_1f8);
                          local_2a8 = (undefined1 (*) [16])*local_140;
                          pauStack_270 = (undefined1 (*) [16])puVar4[7];
                          uStack_274 = (undefined4)((ulonglong)puVar4[6] >> 0x20);
                          uStack_284 = (undefined4)*puVar3;
                          uStack_280 = (undefined4)((ulonglong)*puVar3 >> 0x20);
                          pauStack_2a0 = (undefined1 (*) [16])*puVar2;
                          uStack_290 = (ushort)((ulonglong)*(undefined8 *)((longlong)puVar4 + 0x14)
                                               >> 0x20);
                          uStack_28e = (undefined2)
                                       ((ulonglong)*(undefined8 *)((longlong)puVar4 + 0x14) >> 0x30)
                          ;
                          uStack_28c = (undefined4)*(undefined8 *)((longlong)puVar4 + 0x1c);
                          uStack_288 = (undefined4)
                                       ((ulonglong)*(undefined8 *)((longlong)puVar4 + 0x1c) >> 0x20)
                          ;
                          local_298._0_4_ = (undefined4)puVar4[2];
                          local_298._4_4_ = (undefined4)((ulonglong)puVar4[2] >> 0x20);
                          pauStack_268 = (undefined1 (*) [16])*puVar25;
                          local_260 = (undefined1 (*) [16])
                                      CONCAT44(*(undefined4 *)((longlong)local_140 + 0x4c),
                                               *(undefined4 *)(puVar4 + 9));
                          pauStack_258 = (undefined1 (*) [16])
                                         CONCAT35(pauStack_258._5_3_,*(undefined5 *)(local_140 + 10)
                                                 );
                          uStack_27c = 0x3f000000;
                          uStack_278 = 0x3e4ccccd;
                          country_core::systems::wall::valid_enclosure::ShapeParameters::expand();
                          local_260 = (undefined1 (*) [16])
                                      CONCAT44(local_1f8._4_4_ + 0.2,(undefined4)local_260);
                          assemble_dormer_roof_tiles<system_decorator::decorator_visual::cottage_dormers::assemble_dormer_roof_tiles::generate_dormer_roofs::closure_env_7>
                                    ();
                          goto LAB_1407cd250;
                        }
                        country_core::resources::roofs::roof_shape::Roof::is_flat();
                        if ((extraout_AL_01 != '\0') || (0x17 < bVar35)) goto LAB_1407cd250;
                        if ((0x800180U >> (extraout_EAX & 0x1f) & 1) == 0) {
                          if ((6U >> (extraout_EAX & 0x1f) & 1) != 0) goto LAB_1407cd0c0;
                          goto LAB_1407cd250;
                        }
                        country_core::resources::walls::cached_decorator_transforms::
                        CachedDecoratorTransforms::transform();
                        if ((bVar35 != 7) && ((extraout_EAX & 0xff) != 0x17)) {
                          if ((extraout_EAX & 0xff) != 8) {
                    // WARNING: Subroutine does not return
                            core::panicking::panic();
                          }
                          gothic_dormer::gothic_dormer_profile_curve();
                          pauVar36 = local_220;
                          auVar49._8_8_ = 0;
                          auVar49._0_8_ = local_218;
                          uVar31 = SUB168(auVar49 * ZEXT816(0xc),0);
                          local_150 = local_220;
                          if ((SUB168(auVar49 * ZEXT816(0xc),8) != 0) ||
                             (0x7ffffffffffffffc < uVar31)) {
LAB_1407cd888:
                    // WARNING: Subroutine does not return
                            alloc::raw_vec::handle_error();
                          }
                          if (uVar31 == 0) {
                            puVar32 = (undefined8 *)&DAT_00000004;
                            pauVar26 = (undefined1 (*) [16])0x0;
                            pauVar43 = local_218;
                          }
                          else {
                            local_1a0 = local_218;
                            local_1c0 = uVar31;
                            std::alloc::__default_lib_allocator::__rust_alloc();
                            puVar32 = extraout_RAX_17;
                            pauVar26 = local_1a0;
                            pauVar43 = local_1a0;
                            if (extraout_RAX_17 == (undefined8 *)0x0) goto LAB_1407cd888;
                          }
                          fVar68 = fStack_410 + fStack_410;
                          fVar62 = fStack_414 + fStack_414;
                          fVar63 = local_418 * (local_418 + local_418);
                          fVar57 = (local_418 + local_418) * fStack_40c;
                          fVar69 = fStack_3fc * (1.0 - (fStack_414 * fVar62 + fStack_410 * fVar68));
                          fVar70 = fStack_3fc * (fVar62 * local_418 + fStack_40c * fVar68);
                          fVar71 = fStack_3fc * (fVar68 * local_418 - fVar62 * fStack_40c);
                          fVar85 = fStack_3f8 * (fVar62 * local_418 - fStack_40c * fVar68);
                          fVar86 = fStack_3f8 * (1.0 - (fStack_410 * fVar68 + fVar63));
                          fVar87 = fStack_3f8 * (fVar57 + fVar68 * fStack_414);
                          fVar84 = fStack_3f4 * (fVar62 * fStack_40c + fVar68 * local_418);
                          fVar57 = fStack_3f4 * (fVar68 * fStack_414 - fVar57);
                          fVar62 = fStack_3f4 * (1.0 - (fVar63 + fStack_414 * fVar62));
                          if (pauVar43 != (undefined1 (*) [16])0x0) {
                            fVar63 = fVar84 * 0.0;
                            fVar68 = fVar57 * 0.0;
                            fVar56 = fVar62 * 0.0;
                            if (pauVar43 == (undefined1 (*) [16])0x1) {
                              uVar31 = 0;
                            }
                            else {
                              uVar31 = 0;
                              puVar42 = puVar32;
                              do {
                                fVar5 = *(float *)(*pauVar36 + uVar31 * 8);
                                fVar6 = *(float *)(*pauVar36 + uVar31 * 8 + 4);
                                *puVar42 = CONCAT44(fVar6 * fVar86 + fVar5 * fVar70 + fVar68 +
                                                    fStack_404,
                                                    fVar6 * fVar85 + fVar5 * fVar69 + fVar63 +
                                                    local_408);
                                *(float *)(puVar42 + 1) =
                                     fVar6 * fVar87 + fVar5 * fVar71 + fVar56 + fStack_400;
                                fVar5 = *(float *)(*pauVar36 + uVar31 * 8 + 8);
                                fVar6 = *(float *)(*pauVar36 + uVar31 * 8 + 0xc);
                                *(ulonglong *)((longlong)puVar42 + 0xc) =
                                     CONCAT44(fVar6 * fVar86 + fVar5 * fVar70 + fVar68 + fStack_404,
                                              fVar6 * fVar85 + fVar5 * fVar69 + fVar63 + local_408);
                                *(float *)((longlong)puVar42 + 0x14) =
                                     fVar6 * fVar87 + fVar5 * fVar71 + fVar56 + fStack_400;
                                uVar31 = uVar31 + 2;
                                puVar42 = puVar42 + 3;
                              } while (((ulonglong)pauVar43 & 0xfffffffffffffffe) != uVar31);
                            }
                            if (((ulonglong)pauVar43 & 1) != 0) {
                              fVar5 = *(float *)(*pauVar36 + uVar31 * 8);
                              fVar6 = *(float *)(*pauVar36 + uVar31 * 8 + 4);
                              *(ulonglong *)((longlong)puVar32 + uVar31 * 0xc) =
                                   CONCAT44(fVar6 * fVar86 + fVar5 * fVar70 + fVar68 + fStack_404,
                                            fVar6 * fVar85 + fVar5 * fVar69 + fVar63 + local_408);
                              *(float *)((longlong)puVar32 + uVar31 * 0xc + 8) =
                                   fVar6 * fVar87 + fVar5 * fVar71 + fVar56 + fStack_400;
                            }
                          }
                          local_198 = pauVar26;
                          pauStack_190 = (undefined1 (*) [16])puVar32;
                          local_188 = pauVar43;
                          ::utils::curve::Curve::
                          try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
                                    ();
                          cVar46 = -1;
                          cVar50 = -1;
                          cVar52 = -1;
                          cVar54 = -1;
                          cVar72 = -1;
                          cVar73 = -1;
                          cVar74 = -1;
                          cVar75 = -1;
                          cVar76 = -1;
                          cVar77 = -1;
                          cVar78 = -1;
                          cVar79 = -1;
                          cVar80 = -1;
                          cVar81 = -1;
                          cVar82 = -1;
                          cVar83 = -1;
                          if (SBORROW8(0,(longlong)local_208)) {
                            local_198 = pauStack_200;
                            pauStack_190 = local_1f8;
                    // WARNING: Subroutine does not return
                            core::result::unwrap_failed();
                          }
                          local_2f8 = CONCAT44(uStack_1d4,local_1d8);
                          uStack_310 = CONCAT44(uStack_1ec,uStack_1f0);
                          local_308 = CONCAT44(uStack_1e4,uStack_1e8);
                          pcStack_300 = pcStack_1e0;
                          local_318 = local_1f8;
                          local_328 = local_208;
                          pauStack_320 = pauStack_200;
                          if (local_228 != 0) {
                            std::alloc::__default_lib_allocator::__rust_dealloc();
                          }
                          gothic_dormer::gothic_dormer_profile_curve();
                          pauVar36 = local_220;
                          auVar10._8_8_ = 0;
                          auVar10._0_8_ = local_218;
                          pauVar26 = SUB168(auVar10 * ZEXT816(0xc),0);
                          local_128 = local_220;
                          if ((SUB168(auVar10 * ZEXT816(0xc),8) != 0) ||
                             ((undefined1 (*) [16])0x7ffffffffffffffc < pauVar26)) {
LAB_1407cd89b:
                    // WARNING: Subroutine does not return
                            alloc::raw_vec::handle_error();
                          }
                          if (pauVar26 == (undefined1 (*) [16])0x0) {
                            puVar32 = (undefined8 *)&DAT_00000004;
                            pauVar26 = (undefined1 (*) [16])0x0;
                            pauVar43 = local_218;
                          }
                          else {
                            local_150 = local_218;
                            local_1a0 = pauVar26;
                            std::alloc::__default_lib_allocator::__rust_alloc();
                            puVar32 = extraout_RAX_20;
                            pauVar26 = local_150;
                            pauVar43 = local_150;
                            if (extraout_RAX_20 == (undefined8 *)0x0) goto LAB_1407cd89b;
                          }
                          local_188 = pauVar43;
                          local_198 = pauVar26;
                          pauStack_190 = (undefined1 (*) [16])puVar32;
                          if (local_188 != (undefined1 (*) [16])0x0) {
                            pauVar26 = (undefined1 (*) [16])0x0;
                            puVar32 = (undefined8 *)pauStack_190;
                            do {
                              fVar63 = *(float *)(*pauVar36 + (longlong)pauVar26 * 8);
                              fVar68 = *(float *)(*pauVar36 + (longlong)pauVar26 * 8 + 4);
                              *puVar32 = CONCAT44((fVar68 * fVar86 - fVar63 * fVar70) + fVar57 * 0.0
                                                  + fStack_404,
                                                  (fVar68 * fVar85 - fVar63 * fVar69) + fVar84 * 0.0
                                                  + local_408);
                              *(float *)(puVar32 + 1) =
                                   (fVar68 * fVar87 - fVar63 * fVar71) + fVar62 * 0.0 + fStack_400;
                              pauVar26 = (undefined1 (*) [16])(*pauVar26 + 1);
                              puVar32 = (undefined8 *)((longlong)puVar32 + 0xc);
                            } while (local_188 != pauVar26);
                          }
                          ::utils::curve::Curve::
                          try_new_from_points<glam::f32::vec3::Vec3,utils::curve::CurveCoordSemantic3D>
                                    ();
                          if (SBORROW8(0,(longlong)local_208)) {
                            local_198 = pauStack_200;
                            pauStack_190 = local_1f8;
                    // WARNING: Subroutine does not return
                            core::result::unwrap_failed();
                          }
                          local_168 = CONCAT44(uStack_1d4,local_1d8);
                          pauStack_180 = (undefined1 (*) [16])CONCAT44(uStack_1ec,uStack_1f0);
                          local_178 = CONCAT44(uStack_1e4,uStack_1e8);
                          pcStack_170 = pcStack_1e0;
                          local_188 = local_1f8;
                          local_198 = local_208;
                          pauStack_190 = pauStack_200;
                          if (local_228 != 0) {
                            std::alloc::__default_lib_allocator::__rust_dealloc();
                          }
                          local_2a8 = local_328;
                          pauStack_2a0 = pauStack_320;
                          local_298._0_4_ = SUB84(local_318,0);
                          local_298._4_4_ = (undefined4)((ulonglong)local_318 >> 0x20);
                          uStack_290 = (ushort)uStack_310;
                          uStack_28e = (undefined2)((ulonglong)uStack_310 >> 0x10);
                          uStack_28c = (undefined4)((ulonglong)uStack_310 >> 0x20);
                          uStack_288 = (undefined4)local_308;
                          uStack_284 = (undefined4)((ulonglong)local_308 >> 0x20);
                          uStack_280 = SUB84(pcStack_300,0);
                          uStack_27c = (undefined4)((ulonglong)pcStack_300 >> 0x20);
                          uStack_278 = (undefined4)local_2f8;
                          uStack_274 = (undefined4)((ulonglong)local_2f8 >> 0x20);
                          local_240 = local_168;
                          local_250 = local_178;
                          pcStack_248 = pcStack_170;
                          local_260 = local_188;
                          pauStack_258 = pauStack_180;
                          pauStack_270 = local_198;
                          pauStack_268 = pauStack_190;
                          goto LAB_1407ccff2;
                        }
                        if (bVar35 == 0x17) {
                          std::alloc::__default_lib_allocator::__rust_alloc();
                          if (extraout_RAX_18 == (undefined1 (*) [16])0x0) {
                    // WARNING: Subroutine does not return
                            alloc::alloc::handle_alloc_error();
                          }
                          *(undefined8 *)*extraout_RAX_18 = 0x3fa28f5c80000000;
                          *(undefined8 *)(*extraout_RAX_18 + 8) = 0x3f19999a;
                          local_198 = (undefined1 (*) [16])0x2;
                          local_188 = (undefined1 (*) [16])0x2;
                          pauStack_190 = extraout_RAX_18;
                        }
                        else {
                          country_core::resources::gothic_window_profile::
                          gothic_half_dormer_profile_curve();
                        }
                        local_150 = (undefined1 (*) [16])
                                    (CONCAT44(local_150._4_4_,extraout_EAX) & 0xffffffff000000ff);
                        local_f2 = 1;
                        country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall();
                        puVar44 = local_108;
                        if (local_108[3] == 0) goto LAB_1407cd4c3;
                        core::hash::BuildHasher::
                        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>
                                  ();
                        local_1c0 = *puVar44;
                        uVar31 = puVar44[1];
                        bVar47 = (byte)(extraout_RAX_19 >> 0x39);
                        auVar49 = ZEXT216(CONCAT11(bVar47,bVar47));
                        auVar49 = pshuflw(auVar49,auVar49,0);
                        local_1a0 = (undefined1 (*) [16])(local_1c0 - 0x2a8);
                        lVar23 = 0;
                        uVar38 = extraout_RAX_19;
                        goto LAB_1407cc95b;
                      }
                      uVar19 = (ushort)(uVar27 - 1) & (ushort)uVar27;
                      uVar27 = CONCAT22((short)(uVar27 - 1 >> 0x10),uVar19);
                    }
                    auVar60[0] = -(*pcVar37 == cVar46);
                    auVar60[1] = -(pcVar37[1] == cVar50);
                    auVar60[2] = -(pcVar37[2] == cVar52);
                    auVar60[3] = -(pcVar37[3] == cVar54);
                    auVar60[4] = -(pcVar37[4] == cVar72);
                    auVar60[5] = -(pcVar37[5] == cVar73);
                    auVar60[6] = -(pcVar37[6] == cVar74);
                    auVar60[7] = -(pcVar37[7] == cVar75);
                    auVar60[8] = -(pcVar37[8] == cVar76);
                    auVar60[9] = -(pcVar37[9] == cVar77);
                    auVar60[10] = -(pcVar37[10] == cVar78);
                    auVar60[0xb] = -(pcVar37[0xb] == cVar79);
                    auVar60[0xc] = -(pcVar37[0xc] == cVar80);
                    auVar60[0xd] = -(pcVar37[0xd] == cVar81);
                    auVar60[0xe] = -(pcVar37[0xe] == cVar82);
                    auVar60[0xf] = -(pcVar37[0xf] == cVar83);
                    if ((((((((((((((((SUB161(auVar60 >> 7,0) & 1) != 0 ||
                                     (SUB161(auVar60 >> 0xf,0) & 1) != 0) ||
                                    (SUB161(auVar60 >> 0x17,0) & 1) != 0) ||
                                   (SUB161(auVar60 >> 0x1f,0) & 1) != 0) ||
                                  (SUB161(auVar60 >> 0x27,0) & 1) != 0) ||
                                 (SUB161(auVar60 >> 0x2f,0) & 1) != 0) ||
                                (SUB161(auVar60 >> 0x37,0) & 1) != 0) ||
                               (SUB161(auVar60 >> 0x3f,0) & 1) != 0) ||
                              (SUB161(auVar60 >> 0x47,0) & 1) != 0) ||
                             (SUB161(auVar60 >> 0x4f,0) & 1) != 0) ||
                            (SUB161(auVar60 >> 0x57,0) & 1) != 0) ||
                           (SUB161(auVar60 >> 0x5f,0) & 1) != 0) ||
                          (SUB161(auVar60 >> 0x67,0) & 1) != 0) ||
                         (SUB161(auVar60 >> 0x6f,0) & 1) != 0) ||
                        (SUB161(auVar60 >> 0x77,0) & 1) != 0) || auVar60[0xf] < '\0') break;
                    uVar31 = uVar31 + lVar23 + 0x10;
                    lVar23 = lVar23 + 0x10;
                  } while( true );
                }
                goto LAB_1407cd250;
              }
            }
            parking_lot_core::parking_lot::deadlock_impl::release_resource();
            LOCK();
            bVar45 = *(char *)(lVar24 + 0x18) == '\x01';
            if (bVar45) {
              *(char *)(lVar24 + 0x18) = '\0';
            }
            UNLOCK();
            if (!bVar45) {
              parking_lot::raw_mutex::RawMutex::unlock_slow();
            }
          }
        }
      }
      if (local_2d0 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_f3 != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_24 == 1) {
          plVar29 = extraout_RAX_24 + 1;
        }
        else if ((*extraout_RAX_24 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), plVar29 = extraout_RAX_26, extraout_RAX_26 == (longlong *)0x0)) {
LAB_1407cd519:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        if (*plVar29 != 0) {
LAB_1407cd5d9:
                    // WARNING: Subroutine does not return
          core::cell::panic_already_borrowed();
        }
        *plVar29 = -1;
        local_108 = (ulonglong *)plVar29;
        puffin::ThreadProfiler::end_scope();
        *local_108 = *local_108 + 1;
      }
      return;
    }
  }
  local_f1 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

