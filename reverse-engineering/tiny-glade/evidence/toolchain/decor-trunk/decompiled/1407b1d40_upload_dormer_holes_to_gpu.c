// ADDR 0x1407b1d40
// SIG void __cdecl upload_dormer_holes_to_gpu(void)
// ORIG system::upload_dormer_holes_to_gpu


void __cdecl system_decorator::dormer_holes::upload_dormer_holes_to_gpu(void)

{
  byte *pbVar1;
  undefined4 uVar2;
  undefined4 uVar3;
  undefined4 uVar4;
  undefined1 auVar5 [16];
  byte bVar6;
  byte bVar7;
  byte bVar8;
  longlong lVar9;
  longlong lVar10;
  ushort uVar11;
  undefined *puVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  uint uVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  code *extraout_RAX_03;
  undefined8 extraout_RAX_04;
  code *extraout_RAX_05;
  void *_Dst;
  longlong *extraout_RAX_06;
  code *extraout_RAX_07;
  longlong *extraout_RAX_08;
  code *extraout_RAX_09;
  undefined4 *puVar14;
  code *pcVar15;
  longlong lVar16;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  code *extraout_RAX_12;
  longlong *extraout_RAX_13;
  code *extraout_RAX_14;
  undefined8 *extraout_RAX_15;
  undefined8 *puVar17;
  undefined8 *extraout_RAX_16;
  longlong *plVar18;
  longlong *extraout_RAX_17;
  code *extraout_RAX_18;
  longlong *extraout_RAX_19;
  code *extraout_RAX_20;
  longlong *extraout_RAX_21;
  code *pcVar19;
  ulonglong in_RDX;
  undefined1 uVar20;
  char cVar21;
  int iVar22;
  undefined4 uVar23;
  undefined4 *puVar24;
  undefined8 uVar25;
  undefined1 (*pauVar26) [16];
  undefined1 (*pauVar27) [16];
  undefined8 *in_R8;
  char *pcVar28;
  int iVar29;
  int iVar30;
  void *pvVar31;
  bool bVar32;
  float fVar33;
  float fVar34;
  code *local_328;
  code *pcStack_320;
  code **local_318;
  code *pcStack_310;
  longlong local_308;
  undefined8 uStack_300;
  char *local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  longlong local_2a0;
  code *local_278;
  code *pcStack_270;
  code **local_268;
  code *pcStack_260;
  longlong local_258;
  undefined8 uStack_250;
  char *local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined4 local_22c;
  undefined4 local_228;
  int local_224;
  undefined4 local_220;
  undefined4 local_218;
  undefined8 local_200;
  undefined8 local_1f8;
  code *local_1f0 [2];
  uint local_1dc;
  longlong local_1d8;
  undefined8 local_1d0;
  undefined4 local_1c8;
  undefined4 uStack_1c4;
  code *local_1c0;
  code *local_1b8;
  code *pcStack_1b0;
  code *local_1a8;
  code **ppcStack_1a0;
  code *local_198;
  longlong lStack_190;
  undefined8 local_188;
  char *local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined **local_168;
  undefined8 local_160;
  code **local_158;
  undefined8 local_150;
  undefined8 local_148;
  void *local_138;
  code *local_130;
  char local_122;
  char local_121;
  void *local_120;
  code *local_118;
  code *local_110;
  code *local_108;
  code *local_100;
  code *local_f8;
  code *local_f0;
  undefined1 local_e4;
  undefined1 local_e3;
  undefined1 local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_121 = extraout_AL;
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x3a) {
          if ((char)(&DAT_142a8398b)[in_RDX] < -0x40) {
LAB_1407b1e06:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x3a) goto LAB_1407b1e06;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x3a) {
          if ((char)(&DAT_142a8398d)[in_RDX] < -0x40) {
LAB_1407b1e53:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x3a) goto LAB_1407b1e53;
      }
    }
    pcVar28 = "alloc vectors";
    do {
      bVar6 = pcVar28[-1];
      uVar13 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = pcVar28[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = pcVar28[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = (byte *)(pcVar28 + -4);
            pcVar28 = pcVar28 + -4;
            uVar13 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar28 = pcVar28 + -3;
            uVar13 = bVar8 & 0xf;
          }
          uVar13 = bVar7 & 0x3f | uVar13 << 6;
        }
        else {
          pcVar28 = pcVar28 + -2;
          uVar13 = bVar7 & 0x1f;
        }
        uVar13 = (uint)(bVar6 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1407b1f14;
LAB_1407b1f26:
        if (pcVar28 == "") break;
        if (pcVar28 + -0x142a83acf < (char *)0x2c) {
          if (-0x41 < pcVar28[1]) break;
        }
        else if (pcVar28 + -0x142a83acf == (char *)0x2c) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar28 = pcVar28 + -1;
      if (uVar13 == 0x5c) goto LAB_1407b1f26;
LAB_1407b1f14:
      if (uVar13 == 0x2f) goto LAB_1407b1f26;
    } while (pcVar28 != "crates/systems/decorator/src/dormer_holes.rsalloc vectors");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      pcVar15 = (code *)(extraout_RAX_01 + 1);
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), pcVar15 = extraout_RAX_20, extraout_RAX_20 == (code *)0x0))
    goto LAB_1407b3b52;
    if (*(longlong *)pcVar15 != 0) goto LAB_1407b3ba0;
    *(longlong *)pcVar15 = -1;
    local_f8 = pcVar15;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
  }
  local_e1 = 0;
  local_e2 = 0;
  puffin::are_scopes_on();
  if (extraout_AL_00 == '\0') {
    local_1d8 = 0;
  }
  else {
    pcVar28 = "alloc vectors";
    do {
      bVar6 = pcVar28[-1];
      uVar13 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = pcVar28[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = pcVar28[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = (byte *)(pcVar28 + -4);
            pcVar28 = pcVar28 + -4;
            uVar13 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar28 = pcVar28 + -3;
            uVar13 = bVar8 & 0xf;
          }
          uVar13 = bVar7 & 0x3f | uVar13 << 6;
        }
        else {
          pcVar28 = pcVar28 + -2;
          uVar13 = bVar7 & 0x1f;
        }
        uVar13 = (uint)(bVar6 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1407b2084;
LAB_1407b209c:
        if (pcVar28 == "") break;
        if (pcVar28 + -0x142a83acf < (char *)0x2c) {
          if (-0x41 < pcVar28[1]) break;
        }
        else if (pcVar28 + -0x142a83acf == (char *)0x2c) break;
        local_e1 = 0;
        local_e2 = 0;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar28 = pcVar28 + -1;
      if (uVar13 == 0x5c) goto LAB_1407b209c;
LAB_1407b2084:
      if (uVar13 == 0x2f) goto LAB_1407b209c;
    } while (pcVar28 != "crates/systems/decorator/src/dormer_holes.rsalloc vectors");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_02 == 0) {
      local_e1 = 0;
      local_e2 = 0;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      pcVar15 = extraout_RAX_03;
      if (extraout_RAX_03 == (code *)0x0) goto LAB_1407b3e96;
    }
    else {
      if ((int)*extraout_RAX_02 != 1) {
LAB_1407b3e96:
        local_e1 = 0;
        local_e2 = 0;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      pcVar15 = (code *)(extraout_RAX_02 + 1);
    }
    if (*(longlong *)pcVar15 != 0) {
      local_e1 = 0;
      local_e2 = 0;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *(longlong *)pcVar15 = -1;
    local_f8 = pcVar15;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
    local_1d8 = 1;
    local_1d0 = extraout_RAX_04;
  }
  local_e4 = 0;
  country_core::resources::walls::dormers::DormerHoleSsbo::dummy();
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_05 == (code *)0x0) {
    local_e4 = 0;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  *(code **)(extraout_RAX_05 + 0x20) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x28) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x10) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x18) = ppcStack_1a0;
  *(code **)extraout_RAX_05 = local_1b8;
  *(code **)(extraout_RAX_05 + 8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x50) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x58) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x40) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x48) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x30) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x38) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x80) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x88) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x70) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x78) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x60) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x68) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x90) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x98) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0xa0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0xa8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0xb0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0xb8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0xd0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0xd8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0xe0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0xe8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0xc0) = local_1b8;
  *(code **)(extraout_RAX_05 + 200) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0xf0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0xf8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x100) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x108) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x110) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x118) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x120) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x128) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x130) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x138) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x140) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x148) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x150) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x158) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x170) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x178) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x160) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x168) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x1a0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x1a8) = lStack_190;
  *(code **)(extraout_RAX_05 + 400) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x198) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x180) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x188) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x1b0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x1b8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x1c0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x1c8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x1d0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x1d8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x1e0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x1e8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x1f0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x1f8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x200) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x208) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x210) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x218) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x220) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x228) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x230) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x238) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x240) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x248) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x250) = local_1a8;
  *(code ***)(extraout_RAX_05 + 600) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x260) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x268) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x270) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x278) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x290) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x298) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x280) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x288) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x2c0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x2c8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x2b0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x2b8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x2a0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x2a8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x2d0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x2d8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x2e0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x2e8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x2f0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x2f8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x300) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x308) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x310) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x318) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 800) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x328) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x330) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x338) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x340) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x348) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x350) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x358) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x360) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x368) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x370) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x378) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x380) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x388) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x390) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x398) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x3b0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x3b8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x3a0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x3a8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x3e0) = local_198;
  *(longlong *)(extraout_RAX_05 + 1000) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x3d0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x3d8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x3c0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x3c8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x3f0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x3f8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x400) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x408) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x410) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x418) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x420) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x428) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x430) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x438) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x440) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x448) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x450) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x458) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x460) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x468) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x470) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x478) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x480) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x488) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x490) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x498) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x4a0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x4a8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x4b0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x4b8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x4d0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x4d8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x4c0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x4c8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x500) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x508) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x4f0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x4f8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x4e0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x4e8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x510) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x518) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x520) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x528) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x530) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x538) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x540) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x548) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x550) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x558) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x560) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x568) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x570) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x578) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x580) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x588) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x590) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x598) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x5a0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x5a8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x5b0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x5b8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x5c0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x5c8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x5d0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x5d8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x5f0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x5f8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x5e0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x5e8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x620) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x628) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x610) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x618) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x600) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x608) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x630) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x638) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x640) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x648) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x650) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x658) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x660) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x668) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x670) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x678) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x680) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x688) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x690) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x698) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x6a0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x6a8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x6b0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x6b8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x6c0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x6c8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x6d0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x6d8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x6e0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x6e8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x6f0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x6f8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x710) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x718) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x700) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x708) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x740) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x748) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x730) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x738) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x720) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x728) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x750) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x758) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x760) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x768) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x770) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x778) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x780) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x788) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x790) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x798) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x7a0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x7a8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x7b0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x7b8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x7c0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x7c8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 2000) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x7d8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x7e0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x7e8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x7f0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x7f8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x800) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x808) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x810) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x818) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x830) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x838) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x820) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x828) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x860) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x868) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x850) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x858) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x840) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x848) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x870) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x878) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x880) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x888) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x890) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x898) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x8a0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x8a8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x8b0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x8b8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x8c0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x8c8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x8d0) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x8d8) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x8e0) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x8e8) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x8f0) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x8f8) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x900) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x908) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x910) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x918) = ppcStack_1a0;
  *(code **)(extraout_RAX_05 + 0x920) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x928) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x930) = local_1b8;
  *(code **)(extraout_RAX_05 + 0x938) = pcStack_1b0;
  *(code **)(extraout_RAX_05 + 0x950) = local_198;
  *(longlong *)(extraout_RAX_05 + 0x958) = lStack_190;
  *(code **)(extraout_RAX_05 + 0x940) = local_1a8;
  *(code ***)(extraout_RAX_05 + 0x948) = ppcStack_1a0;
  local_278 = (code *)&DAT_00000032;
  local_268 = (code **)&DAT_00000032;
  local_e4 = 0;
  pcStack_270 = extraout_RAX_05;
  alloc::vec::
  from_elem<alloc::vec::Vec<country_core::resources::walls::dormers::DormerHoleSsbo,alloc::alloc::Global>_>
            ();
  pcVar15 = pcStack_1b0;
  local_100 = local_1b8;
  local_f0 = local_1a8;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
    local_e4 = 1;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  memset(_Dst,0,900);
  local_130 = pcVar15;
  local_138 = _Dst;
  if (local_1d8 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 0) {
      local_108 = local_f0;
      local_110 = pcVar15;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      pcVar19 = extraout_RAX_07;
      if (extraout_RAX_07 == (code *)0x0) goto LAB_1407b3e05;
    }
    else {
      if ((int)*extraout_RAX_06 != 1) {
LAB_1407b3e05:
        local_108 = local_f0;
        local_110 = pcVar15;
        local_118 = local_100;
        local_e1 = 1;
        local_e2 = 1;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      pcVar19 = (code *)(extraout_RAX_06 + 1);
    }
    if (*(longlong *)pcVar19 != 0) {
      local_108 = local_f0;
      local_110 = pcVar15;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *(longlong *)pcVar19 = -1;
    local_f8 = pcVar19;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
  }
  pcVar15 = local_130;
  pvVar31 = local_138;
  local_108 = local_f0;
  local_118 = local_100;
  local_e1 = 1;
  local_e2 = 1;
  local_110 = local_130;
  puffin::are_scopes_on();
  if (extraout_AL_01 != '\0') {
    pcVar28 = "alloc vectors";
    do {
      bVar6 = pcVar28[-1];
      uVar13 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = pcVar28[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = pcVar28[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = (byte *)(pcVar28 + -4);
            pcVar28 = pcVar28 + -4;
            uVar13 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar28 = pcVar28 + -3;
            uVar13 = bVar8 & 0xf;
          }
          uVar13 = bVar7 & 0x3f | uVar13 << 6;
        }
        else {
          pcVar28 = pcVar28 + -2;
          uVar13 = bVar7 & 0x1f;
        }
        uVar13 = (uint)(bVar6 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1407b2c14;
LAB_1407b2c28:
        if (pcVar28 == "") break;
        if (pcVar28 + -0x142a83acf < (char *)0x2c) {
          if (-0x41 < pcVar28[1]) break;
        }
        else if (pcVar28 + -0x142a83acf == (char *)0x2c) break;
        local_108 = local_f0;
        local_118 = local_100;
        local_e1 = 1;
        local_e2 = 1;
        local_110 = pcVar15;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar28 = pcVar28 + -1;
      if (uVar13 == 0x5c) goto LAB_1407b2c28;
LAB_1407b2c14:
      if (uVar13 == 0x2f) goto LAB_1407b2c28;
    } while (pcVar28 != "crates/systems/decorator/src/dormer_holes.rsalloc vectors");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_08 == 0) {
      local_108 = local_f0;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      local_110 = pcVar15;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      pcVar19 = extraout_RAX_09;
      if (extraout_RAX_09 == (code *)0x0) goto LAB_1407b3eb5;
    }
    else {
      if ((int)*extraout_RAX_08 != 1) {
LAB_1407b3eb5:
        local_108 = local_f0;
        local_118 = local_100;
        local_e1 = 1;
        local_e2 = 1;
        local_110 = pcVar15;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      pcVar19 = (code *)(extraout_RAX_08 + 1);
    }
    if (*(longlong *)pcVar19 != 0) {
      local_108 = local_f0;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      local_110 = pcVar15;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *(longlong *)pcVar19 = -1;
    local_f8 = pcVar19;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
    pcVar15 = local_130;
    pvVar31 = local_138;
  }
  pauVar27 = *(undefined1 (**) [16])*in_R8;
  local_2a0 = ((undefined8 *)*in_R8)[3];
  auVar5 = *pauVar27;
  pauVar26 = pauVar27 + 1;
  uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar5 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar5[0xf] >> 7) << 0xf);
  local_122 = extraout_AL_01;
  while (bVar32 = local_2a0 != 0, local_2a0 = local_2a0 + -1, bVar32) {
    if ((short)uVar13 == 0) {
      do {
        auVar5 = *pauVar26;
        uVar11 = (ushort)(SUB161(auVar5 >> 7,0) & 1) | (ushort)(SUB161(auVar5 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar5 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar5 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar5 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar5 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar5 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar5 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar5 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar5 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar5 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar5 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar5 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar5 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar5 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar5[0xf] >> 7) << 0xf;
        pauVar27 = pauVar27 + -0x10;
        pauVar26 = pauVar26 + 1;
      } while (uVar11 == 0xffff);
      uVar13 = ~(uint)uVar11;
      local_1dc = uVar13 & -(uint)uVar11 - 2;
    }
    else {
      local_1dc = uVar13 - 1 & uVar13;
    }
    iVar22 = 0;
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      iVar22 = iVar22 + 1;
    }
    lVar9 = *(longlong *)((longlong)pauVar27 + (-8 - (ulonglong)(uint)(iVar22 << 4)));
    puVar24 = *(undefined4 **)(lVar9 + 0xf8);
    puVar14 = puVar24 + *(longlong *)(lVar9 + 0x100) * 0xc;
    while (pcVar15 = local_130, uVar13 = local_1dc, puVar24 != puVar14) {
      uVar23 = *puVar24;
      local_220 = puVar24[1];
      uVar2 = puVar24[2];
      local_218 = puVar24[3];
      uVar3 = puVar24[4];
      local_22c = puVar24[5];
      uVar4 = puVar24[6];
      local_228 = puVar24[7];
      lVar9 = *(longlong *)(puVar24 + 8);
      uVar25 = *(undefined8 *)(puVar24 + 10);
      local_1d0 = CONCAT44(uVar2,uVar23);
      local_1d8 = lVar9;
      local_1c8 = uVar3;
      uStack_1c4 = uVar4;
      ::utils::geometry::rectangle::Rectangle2d::aabb();
      fVar34 = local_200._4_4_ + 65.0;
      fVar33 = floorf(((float)local_200 + 65.0) / 8.666667);
      iVar22 = (int)fVar33;
      if (2.1474835e+09 < fVar33) {
        iVar22 = 0x7fffffff;
      }
      if (NAN(fVar33)) {
        iVar22 = 0;
      }
      fVar33 = floorf(fVar34 / 8.666667);
      iVar30 = (int)fVar33;
      if (2.1474835e+09 < fVar33) {
        iVar30 = 0x7fffffff;
      }
      if (NAN(fVar33)) {
        iVar30 = 0;
      }
      if (iVar22 < 1) {
        iVar22 = 0;
      }
      local_1c0 = (code *)CONCAT44(local_1c0._4_4_,iVar22);
      if (iVar30 < 1) {
        iVar30 = 0;
      }
      fVar34 = local_1f8._4_4_ + 65.0;
      fVar33 = floorf(((float)local_1f8 + 65.0) / 8.666667);
      iVar22 = (int)fVar33;
      if (2.1474835e+09 < fVar33) {
        iVar22 = 0x7fffffff;
      }
      if (NAN(fVar33)) {
        iVar22 = 0;
      }
      fVar33 = floorf(fVar34 / 8.666667);
      iVar29 = (int)fVar33;
      if (2.1474835e+09 < fVar33) {
        iVar29 = 0x7fffffff;
      }
      if (NAN(fVar33)) {
        iVar29 = 0;
      }
      if (0xd < iVar22) {
        iVar22 = 0xe;
      }
      if (0xd < iVar29) {
        iVar29 = 0xe;
      }
      local_1b8 = (code *)0x0;
      pcStack_1b0 = (code *)&DAT_00000008;
      local_1a8 = (code *)0x0;
      if (iVar29 < iVar30) {
        pcVar15 = (code *)0x0;
        local_f8 = (code *)0x0;
      }
      else {
        pcVar15 = (code *)0x0;
        local_224 = iVar29;
        do {
          local_f8 = (code *)CONCAT71(local_f8._1_7_,iVar30 < local_224);
          if ((int)local_1c0 <= iVar22) {
            pcVar19 = pcVar15;
            iVar29 = (int)local_1c0;
            do {
              if (pcVar19 == local_1b8) {
                alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
              }
              pcVar15 = pcVar19 + 1;
              *(ulonglong *)(pcStack_1b0 + (longlong)pcVar19 * 8) =
                   (ulonglong)(uint)(iVar30 * 0xf + iVar29);
              local_1a8 = pcVar15;
            } while ((iVar29 < iVar22) &&
                    (iVar29 = iVar29 + (uint)(iVar29 < iVar22), pcVar19 = pcVar15, iVar29 <= iVar22)
                    );
          }
          if (local_224 <= iVar30) break;
          iVar30 = iVar30 + (uint)(byte)local_f8;
        } while (iVar30 <= local_224);
        local_f8 = local_1b8;
      }
      puVar12 = __imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
      puVar24 = puVar24 + 0xc;
      pcVar15 = pcStack_1b0 + (longlong)pcVar15 * 8;
      pcVar19 = pcStack_1b0;
      local_1c0 = pcStack_1b0;
      local_1b8 = local_f8;
      pvVar31 = local_138;
joined_r0x0001407b3304:
      local_138 = pvVar31;
      if (pcVar19 != pcVar15) {
        do {
          local_1f0[0] = *(code **)pcVar19;
          if (&DAT_000000e0 < local_1f0[0]) {
                    // WARNING: Subroutine does not return
            core::panicking::panic_bounds_check();
          }
          uVar13 = *(int *)((longlong)pvVar31 + (longlong)local_1f0[0] * 4) + 1;
          if (uVar13 < 0x32) {
            *(uint *)((longlong)pvVar31 + (longlong)local_1f0[0] * 4) = uVar13;
            if (local_f0 <= local_1f0[0]) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            if (*(ulonglong *)(local_130 + ((longlong)local_1f0[0] * 3 + 2) * 8) <=
                (ulonglong)uVar13 - 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar10 = *(longlong *)(local_130 + ((longlong)local_1f0[0] * 3 + 1) * 8);
            lVar16 = ((ulonglong)uVar13 - 1) * 0x30;
            *(undefined4 *)(lVar10 + lVar16) = uVar23;
            *(undefined4 *)(lVar10 + 4 + lVar16) = local_220;
            *(undefined4 *)(lVar10 + 8 + lVar16) = uVar2;
            *(undefined4 *)(lVar10 + 0xc + lVar16) = local_218;
            *(undefined4 *)(lVar10 + 0x10 + lVar16) = uVar3;
            *(undefined4 *)(lVar10 + 0x14 + lVar16) = local_22c;
            *(undefined4 *)(lVar10 + 0x18 + lVar16) = uVar4;
            *(undefined4 *)(lVar10 + 0x1c + lVar16) = local_228;
            *(longlong *)(lVar10 + 0x20 + lVar16) = lVar9;
            *(undefined8 *)(lVar10 + 0x28 + lVar16) = uVar25;
          }
          else if (1 < *(ulonglong *)puVar12) goto code_r0x0001407b3209;
          pcVar19 = pcVar19 + 8;
          if (pcVar19 == pcVar15) break;
        } while( true );
      }
      if (local_f8 != (code *)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
  }
  if (local_122 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_11 == 0) {
      local_108 = local_f0;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      local_110 = pcVar15;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      pcVar19 = extraout_RAX_12;
      if (extraout_RAX_12 == (code *)0x0) goto LAB_1407b3efe;
    }
    else {
      if ((int)*extraout_RAX_11 != 1) {
LAB_1407b3efe:
        local_108 = local_f0;
        local_118 = local_100;
        local_e1 = 1;
        local_e2 = 1;
        local_110 = pcVar15;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      pcVar19 = (code *)(extraout_RAX_11 + 1);
    }
    if (*(longlong *)pcVar19 != 0) {
      local_108 = local_f0;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      local_110 = pcVar15;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *(longlong *)pcVar19 = -1;
    local_f8 = pcVar19;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
    pcVar15 = local_130;
    pvVar31 = local_138;
  }
  local_108 = local_f0;
  local_118 = local_100;
  local_e1 = 1;
  local_e2 = 1;
  local_120 = pvVar31;
  local_110 = pcVar15;
  puffin::are_scopes_on();
  if (extraout_AL_02 == '\0') {
    local_1f0[0] = (code *)0x0;
  }
  else {
    pcVar28 = "alloc vectors";
    do {
      bVar6 = pcVar28[-1];
      uVar13 = (uint)(char)bVar6;
      if ((char)bVar6 < '\0') {
        bVar7 = pcVar28[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = pcVar28[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = (byte *)(pcVar28 + -4);
            pcVar28 = pcVar28 + -4;
            uVar13 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar28 = pcVar28 + -3;
            uVar13 = bVar8 & 0xf;
          }
          uVar13 = bVar7 & 0x3f | uVar13 << 6;
        }
        else {
          pcVar28 = pcVar28 + -2;
          uVar13 = bVar7 & 0x1f;
        }
        uVar13 = (uint)(bVar6 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_1407b34c4;
LAB_1407b34df:
        if (pcVar28 == "") break;
        if (pcVar28 + -0x142a83acf < (char *)0x2c) {
          if (-0x41 < pcVar28[1]) break;
        }
        else if (pcVar28 + -0x142a83acf == (char *)0x2c) break;
        local_108 = local_f0;
        local_118 = local_100;
        local_e1 = 1;
        local_e2 = 1;
        local_110 = pcVar15;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar28 = pcVar28 + -1;
      if (uVar13 == 0x5c) goto LAB_1407b34df;
LAB_1407b34c4:
      if (uVar13 == 0x2f) goto LAB_1407b34df;
    } while (pcVar28 != "crates/systems/decorator/src/dormer_holes.rsalloc vectors");
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_13 == 0) {
      local_108 = local_f0;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      local_120 = pvVar31;
      local_110 = pcVar15;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      pcVar19 = extraout_RAX_14;
      if (extraout_RAX_14 == (code *)0x0) goto LAB_1407b3f47;
    }
    else {
      if ((int)*extraout_RAX_13 != 1) {
LAB_1407b3f47:
        local_108 = local_f0;
        local_118 = local_100;
        local_e1 = 1;
        local_e2 = 1;
        local_110 = pcVar15;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      pcVar19 = (code *)(extraout_RAX_13 + 1);
    }
    if (*(longlong *)pcVar19 != 0) {
      local_108 = local_f0;
      local_118 = local_100;
      local_e1 = 1;
      local_e2 = 1;
      local_110 = pcVar15;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *(longlong *)pcVar19 = -1;
    local_f8 = pcVar19;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
    local_1f0[0] = (code *)0x1;
  }
  local_e3 = 1;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX_15 != (undefined8 *)0x0) {
    local_278 = (code *)&anon_8b5f636f5ecef8ae31f545d005a6bf08_0_llvm_10917514084104779652;
    pcStack_270 = (code *)0x43;
    if ((extraout_RAX_15[3] != 0x43) ||
       (iVar22 = memcmp(&anon_8b5f636f5ecef8ae31f545d005a6bf08_0_llvm_10917514084104779652,
                        (void *)extraout_RAX_15[2],0x43), iVar22 != 0)) {
      local_1b8 = (code *)0x0;
      local_e3 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_e3 = 1;
    puVar17 = (undefined8 *)(**(code **)(extraout_RAX_15[1] + 0x18))(*extraout_RAX_15);
    if (puVar17 != (undefined8 *)0x0) {
      local_e3 = 1;
      country_core::render::impl$0::deref();
      if ((ulonglong)puVar17[4] < 900) {
        local_1b8 = (code *)&DAT_0000000b;
LAB_1407b3fd0:
        pcStack_1b0 = local_328;
        local_1a8 = pcStack_320;
        ppcStack_1a0 = local_318;
        local_198 = pcStack_310;
        lStack_190 = local_308;
        local_188 = uStack_300;
        local_180 = local_2f8;
        uStack_178 = uStack_2f0;
        local_170 = local_2e8;
                    // WARNING: Subroutine does not return
        core::result::unwrap_failed();
      }
      local_278 = (code *)0x0;
      pcStack_270 = (code *)0x384;
      local_e3 = 1;
      rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
      if (local_1b8 != (code *)&DAT_0000000b) {
        local_2e8 = local_170;
        local_2f8 = local_180;
        uStack_2f0 = uStack_178;
        local_308 = lStack_190;
        uStack_300 = local_188;
        local_318 = ppcStack_1a0;
        pcStack_310 = local_198;
        local_328 = pcStack_1b0;
        pcStack_320 = local_1a8;
        goto LAB_1407b3fd0;
      }
      *puVar17 = 0xe1;
    }
  }
  local_e3 = 1;
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  uVar25 = CONCAT71((int7)((ulonglong)pcVar15 >> 8),1);
  uVar23 = (undefined4)uVar25;
  pcVar15 = local_130;
  if (extraout_RAX_16 != (undefined8 *)0x0) {
    local_278 = (code *)&anon_8b5f636f5ecef8ae31f545d005a6bf08_2_llvm_10917514084104779652;
    pcStack_270 = (code *)0x37;
    if ((extraout_RAX_16[3] != 0x37) ||
       (iVar22 = memcmp(&anon_8b5f636f5ecef8ae31f545d005a6bf08_2_llvm_10917514084104779652,
                        (void *)extraout_RAX_16[2],0x37), iVar22 != 0)) {
      local_1b8 = (code *)0x0;
      local_e3 = 1;
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    local_e3 = 1;
    plVar18 = (longlong *)(**(code **)(extraout_RAX_16[1] + 0x18))(*extraout_RAX_16);
    pcVar15 = local_130;
    if (plVar18 != (longlong *)0x0) {
      ppcStack_1a0 = (code **)(local_130 + (longlong)local_f0 * 0x18);
      local_1b8 = local_130;
      pcStack_1b0 = local_130;
      local_1a8 = local_100;
      local_198 = (code *)0x0;
      uStack_178 = 0;
      local_e3 = 0;
      alloc::vec::spec_from_iter::impl$0::
      from_iter<country_core::resources::walls::dormers::DormerHoleSsbo,core::iter::adapters::flatten::Flatten<alloc::vec::into_iter::IntoIter<alloc::vec::Vec<country_core::resources::walls::dormers::DormerHoleSsbo,alloc::alloc::Global>,alloc::alloc::Global>_>_>
                ();
      country_core::render::impl$0::deref();
      lVar9 = CONCAT44(uStack_1c4,local_1c8);
      if (lVar9 != 0) {
        if ((ulonglong)plVar18[4] < (ulonglong)(lVar9 * 0x30)) {
          local_1b8 = (code *)&DAT_0000000b;
LAB_1407b408c:
          local_170 = local_238;
          local_180 = local_248;
          uStack_178 = uStack_240;
          lStack_190 = local_258;
          local_188 = uStack_250;
          ppcStack_1a0 = local_268;
          local_198 = pcStack_260;
          pcStack_1b0 = local_278;
          local_1a8 = pcStack_270;
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
        local_200 = 0;
        local_1f8 = lVar9 * 0x30;
        rhapsody::raw::device::GraphicsDevice::update_buffer_bytes();
        if (local_1b8 != (code *)&DAT_0000000b) {
          local_238 = local_170;
          local_248 = local_180;
          uStack_240 = uStack_178;
          local_258 = lStack_190;
          uStack_250 = local_188;
          local_268 = ppcStack_1a0;
          pcStack_260 = local_198;
          local_278 = pcStack_1b0;
          pcStack_270 = local_1a8;
          goto LAB_1407b408c;
        }
      }
      pcVar15 = local_130;
      *plVar18 = lVar9;
      if (local_1d8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      uVar25 = 0;
    }
    uVar23 = (undefined4)uVar25;
  }
  cVar21 = (char)uVar25;
  if (local_1f0[0] != (code *)0x0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    uVar20 = (undefined1)uVar23;
    if (*extraout_RAX_17 == 0) {
      local_120 = local_138;
      local_108 = local_f0;
      local_118 = local_100;
      local_e2 = 1;
      local_110 = pcVar15;
      local_e1 = uVar20;
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      pcVar19 = extraout_RAX_18;
      if (extraout_RAX_18 == (code *)0x0) goto LAB_1407b3e4e;
    }
    else {
      if ((int)*extraout_RAX_17 != 1) {
LAB_1407b3e4e:
        local_120 = local_138;
        local_108 = local_f0;
        local_118 = local_100;
        local_e2 = 1;
        local_110 = pcVar15;
        local_e1 = uVar20;
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      pcVar19 = (code *)(extraout_RAX_17 + 1);
    }
    if (*(longlong *)pcVar19 != 0) {
      local_120 = local_138;
      local_108 = local_f0;
      local_118 = local_100;
      local_e2 = 1;
      local_110 = pcVar15;
      local_e1 = uVar20;
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    local_f8 = (code *)CONCAT44(local_f8._4_4_,uVar23);
    *(longlong *)pcVar19 = -1;
    local_1c0 = pcVar19;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_1c0 = *(longlong *)local_1c0 + 1;
    cVar21 = (char)local_f8;
    pcVar15 = local_130;
  }
  std::alloc::__default_lib_allocator::__rust_dealloc();
  if (cVar21 != '\0') {
    if (local_f0 != (code *)0x0) {
      pcVar15 = pcVar15 + 8;
      do {
        if (*(longlong *)(pcVar15 + -8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        pcVar15 = pcVar15 + 0x18;
        local_f0 = local_f0 + -1;
      } while (local_f0 != (code *)0x0);
    }
    if (local_100 != (code *)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if (local_121 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_19 == 1) {
      plVar18 = extraout_RAX_19 + 1;
    }
    else if ((*extraout_RAX_19 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_21, extraout_RAX_21 == (longlong *)0x0)) {
LAB_1407b3b52:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_1407b3ba0:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    local_f8 = (code *)plVar18;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
  }
  return;
code_r0x0001407b3209:
  local_278 = (code *)&DAT_142a83bc0;
  pcStack_270 = core::fmt::num::imp::impl$9::fmt;
  local_268 = local_1f0;
  pcStack_260 = core::fmt::num::imp::impl$9::fmt;
  toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
  local_198 = (code *)*extraout_RAX_10;
  lStack_190 = extraout_RAX_10[1];
  local_188 = 2;
  local_180 = "system_decorator::dormer_holes";
  uStack_178 = 0x1e;
  local_168 = &PTR_s_Exceeded_maximum_count_of_dormer_142a83c18;
  local_160 = 2;
  local_150 = 2;
  local_148 = 0;
  local_1b8 = (code *)0x0;
  pcStack_1b0 = (code *)0x142a83c50;
  local_1a8 = (code *)&DAT_0000001e;
  ppcStack_1a0 = (code **)0x0;
  local_170 = CONCAT44((int)extraout_RAX_10[2],1);
  local_158 = &local_278;
  ::log::__private_api::impl$2::log();
  pcVar19 = pcVar19 + 8;
  pvVar31 = local_138;
  goto joined_r0x0001407b3304;
}


