// ADDR 0x140812b20
// SIG void __cdecl derive_decorator_subtypes(void)
// ORIG system::derive_decorator_subtypes


void __cdecl system_decorator::derive_decorator_subtype::derive_decorator_subtypes(void)

{
  byte *pbVar1;
  float fVar2;
  float fVar3;
  float fVar4;
  float fVar5;
  undefined1 auVar6 [16];
  byte bVar7;
  byte bVar8;
  byte bVar9;
  int iVar10;
  ushort uVar11;
  uint uVar12;
  char extraout_AL;
  char extraout_AL_00;
  undefined1 extraout_AL_01;
  undefined1 extraout_AL_02;
  uint uVar13;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  undefined8 *puVar14;
  undefined1 (*extraout_RAX_02) [16];
  ulonglong uVar15;
  longlong extraout_RAX_03;
  float *extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  uint uVar16;
  undefined8 *in_RCX;
  longlong *plVar17;
  undefined1 (*pauVar18) [16];
  undefined1 (*pauVar19) [16];
  longlong lVar20;
  longlong *plVar21;
  undefined1 uVar22;
  longlong *in_RDX;
  undefined ***pppuVar23;
  undefined1 *puVar24;
  undefined1 (*pauVar25) [16];
  undefined1 (*pauVar26) [16];
  undefined1 (*pauVar27) [16];
  undefined8 *in_R8;
  undefined *puVar28;
  undefined1 (*in_R9) [16];
  longlong lVar29;
  undefined1 (*pauVar30) [16];
  undefined1 (*pauVar31) [16];
  bool bVar32;
  float extraout_XMM0_Da;
  undefined4 extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar33;
  float fVar34;
  float fVar35;
  undefined8 *in_stack_00000030;
  undefined1 (*local_2a0) [16];
  undefined8 *local_298;
  undefined8 local_290;
  undefined8 local_270;
  undefined8 local_268;
  char *local_260;
  undefined8 local_258;
  undefined8 *local_248;
  undefined1 *local_240;
  undefined1 *local_238;
  undefined **local_228;
  code *local_220;
  undefined8 *local_218;
  undefined8 local_210;
  undefined8 local_208;
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined4 *local_1e8;
  undefined1 (*local_1e0) [16];
  undefined8 local_1d8;
  undefined8 local_1d0;
  undefined4 local_1c4;
  undefined8 local_1c0;
  undefined8 local_1b8;
  undefined1 (*local_1b0) [16];
  undefined ***local_1a8;
  undefined1 (*local_1a0) [16];
  undefined1 (**local_198) [16];
  undefined1 (*local_190) [16];
  undefined8 **local_188;
  code *local_180;
  undefined8 *local_178;
  code *local_170;
  undefined1 *local_168;
  code *local_160;
  undefined1 *local_158;
  code *local_150;
  longlong local_148;
  undefined4 local_140;
  uint local_13c;
  undefined8 *local_138;
  longlong *local_130;
  undefined1 (*local_128) [16];
  undefined1 (*local_120) [16];
  undefined8 local_118;
  undefined8 *local_110;
  longlong local_108;
  undefined1 (*local_100) [16];
  undefined1 local_f4;
  undefined1 local_f3;
  undefined1 local_f2;
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  plVar21 = in_RDX;
  puffin::are_scopes_on();
  local_f1 = extraout_AL;
  if (extraout_AL == '\0') {
    local_290 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    plVar17 = (longlong *)&DAT_142a8f170;
    if (extraout_RAX == 1) {
      if (plVar21 != (longlong *)0x0) {
        if (&DAT_00000044 < plVar21) {
          if (plVar21 != (longlong *)0x45) goto LAB_140812bed;
        }
        else if ((char)plVar21[0x28551e2e] < -0x40) {
LAB_140812bed:
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (plVar21 != (longlong *)0xfffffffffffffffe) {
          if ((undefined1 *)((longlong)plVar21 + 2) < (undefined1 *)0x45) {
            if (*(char *)((longlong)plVar21 + 0x142a8f172) < -0x40) {
LAB_140812c41:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if ((undefined1 *)((longlong)plVar21 + 2) != (undefined1 *)0x45) goto LAB_140812c41;
        }
        plVar17 = (longlong *)((longlong)plVar21 + 0x142a8f172);
      }
    }
    plVar21 = plVar17;
    puVar28 = &DAT_142a8f316;
    do {
      bVar7 = puVar28[-1];
      uVar13 = (uint)(char)bVar7;
      if ((char)bVar7 < '\0') {
        bVar8 = puVar28[-2];
        if ((char)bVar8 < -0x40) {
          bVar9 = puVar28[-3];
          if ((char)bVar9 < -0x40) {
            pbVar1 = puVar28 + -4;
            puVar28 = puVar28 + -4;
            uVar13 = bVar9 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar28 = puVar28 + -3;
            uVar13 = bVar9 & 0xf;
          }
          uVar13 = bVar8 & 0x3f | uVar13 << 6;
        }
        else {
          puVar28 = puVar28 + -2;
          uVar13 = bVar8 & 0x1f;
        }
        uVar13 = (uint)(bVar7 & 0x3f) | uVar13 << 6;
        if (uVar13 != 0x5c) goto LAB_140812d04;
LAB_140812d15:
        if (puVar28 == &DAT_142a8f2dd) break;
        if (puVar28 + -0x142a8f2dd < (undefined *)0x38) {
          if (-0x41 < (char)puVar28[1]) break;
        }
        else if (puVar28 + -0x142a8f2dd == (undefined *)0x38) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar28 = puVar28 + -1;
      if (uVar13 == 0x5c) goto LAB_140812d15;
LAB_140812d04:
      if (uVar13 == 0x2f) goto LAB_140812d15;
    } while (puVar28 != &DAT_142a8f2de);
    local_100 = in_R9;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar17 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar17 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
    goto LAB_1408137cf;
    if (*plVar17 != 0) goto LAB_1408138b8;
    *plVar17 = -1;
    local_130 = plVar17;
    puffin::ThreadProfiler::begin_scope();
    *local_130 = *local_130 + 1;
    local_290 = 1;
    in_R9 = local_100;
  }
  local_140 = *(undefined4 *)((longlong)in_RCX + 0x1c);
  local_1e8 = (undefined4 *)in_RCX[2];
  *local_1e8 = local_140;
  plVar17 = (longlong *)*in_RCX;
  if (plVar17[3] != 0) {
    in_RCX = (undefined8 *)plVar17[1];
    if (in_RCX != (undefined8 *)0x0) {
      memset((void *)*plVar17,(int)CONCAT71((int7)((ulonglong)plVar21 >> 8),0xff),
             (size_t)((longlong)in_RCX + 0x11));
    }
    plVar17[3] = 0;
    puVar14 = (undefined8 *)
              (((ulonglong)((longlong)in_RCX + 1) & 0xfffffffffffffff8) -
              ((ulonglong)((longlong)in_RCX + 1) >> 3));
    if (in_RCX < &DAT_00000008) {
      puVar14 = in_RCX;
    }
    plVar17[2] = (longlong)puVar14;
  }
  local_148 = *in_RDX;
  local_130 = plVar17;
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter();
  local_1c0 = CONCAT71(local_1c0._1_7_,(undefined1)local_1b8);
  local_108 = (longlong)local_198;
  local_138 = local_178;
  local_1f8 = *in_R8;
  local_1f0 = *(undefined8 *)*in_R9;
  pppuVar23 = local_1a8;
  pauVar27 = local_1b0;
  pauVar19 = local_190;
  pauVar18 = local_1a0;
  uVar13 = (uint)(ushort)local_180;
  do {
    pauVar25 = local_120;
    puVar14 = local_110;
    if (pppuVar23 == (undefined ***)0x2) {
LAB_140813045:
      local_110 = puVar14;
      local_120 = pauVar25;
      pauVar26 = pauVar27;
      if (((byte)local_1c0 & pauVar26 != (undefined1 (*) [16])0x0) == 0) {
        country_core::resources::walls::decorator_storage::Decorators::iter_removed_ids();
        local_118 = *in_stack_00000030;
        pauVar19 = local_1b0;
        puVar24 = (undefined1 *)local_1b8;
        uVar13 = (uint)(ushort)local_1a0;
        break;
      }
      local_110 = (undefined8 *)0x2;
      local_118 = CONCAT44(local_118._4_4_,uVar13);
      local_120 = (undefined1 (*) [16])0x0;
      pauVar30 = (undefined1 (*) [16])0x2;
      puVar14 = in_RCX;
      local_128 = pauVar18;
      local_100 = pauVar19;
    }
    else {
      pauVar26 = pauVar18;
      if (local_108 == 0) {
joined_r0x000140813035:
        local_120 = pauVar27;
        local_110 = pppuVar23;
        pauVar27 = local_120;
        if (((ulonglong)local_110 & 1) == 0) {
          local_108 = 0;
          pauVar18 = pauVar26;
        }
        else {
          pauVar30 = (undefined1 (*) [16])0x1;
          local_108 = 0;
          pauVar18 = (undefined1 (*) [16])0x0;
          if (pauVar26 != (undefined1 (*) [16])0x0) {
            local_118._4_4_ = (undefined4)((ulonglong)local_118 >> 0x20);
            local_118 = CONCAT44(local_118._4_4_,uVar13);
            local_108 = 0;
            local_128 = (undefined1 (*) [16])0x0;
            puVar14 = in_RCX;
            local_100 = pauVar19;
            goto LAB_140813088;
          }
        }
        goto LAB_140813045;
      }
      if (local_138 == (undefined8 *)0x0) {
        local_138 = (undefined8 *)0x0;
        goto joined_r0x000140813035;
      }
      if ((short)uVar13 == 0) {
        do {
          auVar6 = *pauVar19;
          uVar11 = (ushort)(SUB161(auVar6 >> 7,0) & 1) | (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar6[0xf] >> 7) << 0xf;
          local_108 = local_108 + -0x980;
          pauVar19 = pauVar19 + 1;
        } while (uVar11 == 0xffff);
        uVar13 = ~(uint)uVar11;
        uVar16 = uVar13 & -(uint)uVar11 - 2;
        puVar14 = (undefined8 *)(ulonglong)uVar16;
        local_100 = pauVar19;
      }
      else {
        uVar16 = uVar13 - 1 & uVar13;
        puVar14 = pppuVar23;
        local_100 = pauVar19;
      }
      local_118 = CONCAT44(local_118._4_4_,uVar16);
      uVar16 = 0;
      for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
        uVar16 = uVar16 + 1;
      }
      local_128 = pauVar18;
      local_120 = pauVar27;
      local_110 = pppuVar23;
      country_core::resources::walls::decorator_storage::impl$16::from();
      local_138 = (undefined8 *)((longlong)local_138 + -1);
      if (extraout_RAX_02 == (undefined1 (*) [16])0x3) {
        pppuVar23 = (undefined ***)local_110;
        pauVar27 = local_120;
        pauVar19 = local_100;
        pauVar26 = local_128;
        uVar13 = (uint)local_118;
        pauVar25 = local_120;
        puVar14 = local_110;
        goto joined_r0x000140813035;
      }
      pauVar26 = (undefined1 (*) [16])((ulonglong)uVar16 * -0x98 + local_108 + -0x90);
      pauVar30 = extraout_RAX_02;
    }
LAB_140813088:
    pauVar31 = *(undefined1 (**) [16])pauVar26[3];
    lVar29 = *(longlong *)(pauVar26[4] + 8);
    auVar6 = *pauVar31;
    pauVar25 = pauVar31 + 1;
    uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar6 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar6[0xf] >> 7) << 0xf);
    local_298 = puVar14;
    local_2a0 = pauVar30;
    pauVar18 = local_128;
    pauVar27 = local_120;
    pppuVar23 = (undefined ***)local_110;
    pauVar19 = local_100;
    while (local_128 = pauVar18, local_120 = pauVar27, local_110 = pppuVar23, local_100 = pauVar19,
          lVar29 != 0) {
      lVar29 = lVar29 + -1;
      if ((short)uVar13 == 0) {
        do {
          auVar6 = *pauVar25;
          uVar11 = (ushort)(SUB161(auVar6 >> 7,0) & 1) | (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1
                   | (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar6[0xf] >> 7) << 0xf;
          pauVar31 = pauVar31 + -0x50;
          pauVar25 = pauVar25 + 1;
        } while (uVar11 == 0xffff);
        uVar13 = ~(uint)uVar11;
        uVar16 = uVar13 & -(uint)uVar11 - 2;
      }
      else {
        uVar16 = uVar13 - 1 & uVar13;
      }
      uVar12 = 0;
      for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
        uVar12 = uVar12 + 1;
      }
      local_240 = pauVar31[(ulonglong)uVar12 * -5 + -5] + 8;
      local_1c4 = *(undefined4 *)pauVar31[(ulonglong)uVar12 * -5 + -5];
      local_1b8 = (undefined **)local_240;
      local_1b0 = pauVar30;
      local_1a8 = (undefined ***)puVar14;
      derive_decorator_subtype();
      *local_1e8 = local_140;
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_subtype::DecoratorSubtype,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (extraout_AL_00 != '\x1a') {
        country_core::resources::walls::decorator_storage::DecoratorStorage::iter();
        core::iter::adapters::chain::impl$1::
        try_fold<core::iter::adapters::chain::Chain<core::iter::adapters::map::Map<std::collections::hash::map::Iter<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl_17::iter::closure_env_0>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl_17::iter::closure_env_1>_>,core::iter::adapters::map::Map<core::option::Iter<country_core::resources::walls::decorator_storage::Decorators>,country_core::resources::walls::decorator_storage::impl_17::iter::closure_env_2>,tuple_<>,core::iter::traits::iterator::Iterator::find_map::check::closure_env_0<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref_<country_core::resources::walls::decorator_storage::Decorators>_>,tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref_<country_core::resources::walls::decorator::DecoratorInfo>_>,system_decorator::derive_decorator_subtype::derive_decorator_subtypes::closure_env_0>,enum2_<core::ops::control_flow::ControlFlow<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,ref_<country_core::resources::walls::decorator::DecoratorInfo>_>,tuple_<>_>_>_>
                  ();
        if (local_228 != (undefined **)0x3) {
          local_1d8 = local_228;
          local_1d0 = local_220;
          local_248 = local_218;
          if (*(longlong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E != 0) {
            local_1b0 = (undefined1 (*) [16])country_core::resources::walls::decorator::impl$27::fmt
            ;
            local_1a8 = (undefined ***)&local_240;
            local_1a0 = (undefined1 (*) [16])
                        core::fmt::impl$74::
                        fmt<country_core::resources::walls::decorator::DecoratorInfo>;
            local_198 = &local_2a0;
            local_190 = (undefined1 (*) [16])
                        country_core::resources::walls::decorator_storage::impl$25::fmt;
            local_188 = &local_248;
            local_180 = core::fmt::impl$74::
                        fmt<country_core::resources::walls::decorator::DecoratorInfo>;
            local_178 = &local_1d8;
            local_170 = country_core::resources::walls::decorator_storage::impl$25::fmt;
            local_168 = &local_f2;
            local_160 = country_core::resources::walls::decorator_subtype::impl$15::fmt;
            local_158 = &local_f4;
            local_150 = country_core::resources::walls::decorator_subtype::impl$15::fmt;
            local_228 = &PTR_DAT_142a8f3e0;
            local_220 = (code *)0x7;
            local_208 = 0;
            local_210 = 7;
            local_218 = &local_1b8;
            local_1b8 = (undefined **)&local_1c4;
            toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
            local_270 = "system_decorator::derive_decorator_subtype";
            local_268 = 0x2a;
            local_260 = "system_decorator::derive_decorator_subtype";
            local_258 = 0x2a;
            ::log::__private_api::log<tuple_<>,log::__private_api::GlobalLogger>();
          }
          local_228 = (undefined **)&local_f3;
          local_220 = core::option::impl$46::
                      fmt<country_core::resources::walls::decorator_subtype::DecoratorSubtype>;
          local_1b8 = &PTR_DAT_142a8f498;
          local_1b0 = (undefined1 (*) [16])0x1;
          local_198 = (undefined1 (**) [16])0x0;
          local_1a0 = (undefined1 (*) [16])0x1;
          local_1a8 = &local_228;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      local_f3 = 0x1a;
      uVar13 = uVar16;
      pauVar18 = local_128;
      pauVar27 = local_120;
      pppuVar23 = (undefined ***)local_110;
      pauVar19 = local_100;
    }
    in_RCX = puVar14;
    uVar13 = (uint)local_118;
  } while( true );
LAB_140813215:
  if (local_198 == (undefined1 (**) [16])0x0) goto LAB_1408136ea;
  local_148 = (longlong)local_198 + -1;
  if ((short)uVar13 == 0) {
    do {
      auVar6 = *pauVar19;
      uVar11 = (ushort)(SUB161(auVar6 >> 7,0) & 1) | (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
               (ushort)(byte)(auVar6[0xf] >> 7) << 0xf;
      puVar24 = puVar24 + -0x980;
      pauVar19 = pauVar19 + 1;
    } while (uVar11 == 0xffff);
    uVar13 = ~(uint)uVar11;
    local_13c = uVar13 & -(uint)uVar11 - 2;
    local_1e0 = pauVar19;
  }
  else {
    if (puVar24 == (undefined1 *)0x0) goto LAB_1408136ea;
    local_13c = uVar13 - 1 & uVar13;
    local_1e0 = pauVar19;
  }
  uVar16 = 0;
  for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
    uVar16 = uVar16 + 1;
  }
  lVar29 = (ulonglong)uVar16 * -0x98;
  local_138 = (undefined8 *)(puVar24 + lVar29);
  local_1c0 = *(undefined8 *)(puVar24 + lVar29 + -0x98);
  pauVar27 = *(undefined1 (**) [16])(puVar24 + lVar29 + -0x60);
  auVar6 = *pauVar27;
  pauVar18 = pauVar27 + 1;
  local_110 = *(undefined8 **)(puVar24 + lVar29 + -0x48);
  uVar16 = ~(uint)(ushort)((ushort)(SUB161(auVar6 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar6[0xf] >> 7) << 0xf);
  local_238 = puVar24;
  while (pauVar19 = local_1e0, puVar24 = local_238, local_198 = (undefined1 (**) [16])local_148,
        uVar13 = local_13c, local_110 != (undefined8 *)0x0) {
    if ((short)uVar16 == 0) {
      do {
        auVar6 = *pauVar18;
        uVar11 = (ushort)(SUB161(auVar6 >> 7,0) & 1) | (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1 |
                 (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar6[0xf] >> 7) << 0xf;
        pauVar27 = pauVar27 + -0x50;
        pauVar18 = pauVar18 + 1;
      } while (uVar11 == 0xffff);
      uVar16 = ~(uint)uVar11;
      uVar13 = uVar16 & -(uint)uVar11 - 2;
    }
    else {
      uVar13 = uVar16 - 1 & uVar16;
    }
    local_110 = (undefined8 *)((longlong)local_110 + -1);
    uVar12 = 0;
    for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
      uVar12 = uVar12 + 1;
    }
    uVar15 = (ulonglong)uVar12;
    local_108 = CONCAT44(local_108._4_4_,uVar13);
    uVar16 = uVar13;
    if ((pauVar27[uVar15 * -5 + -1][0xe] == ';') && ((byte)pauVar27[uVar15 * -5 + -3][10] < 2)) {
      local_100 = (undefined1 (*) [16])
                  CONCAT44(local_100._4_4_,*(undefined4 *)pauVar27[uVar15 * -5 + -5]);
      fVar2 = *(float *)(pauVar27[uVar15 * -5 + -5] + 0xc);
      country_core::resources::walls::public_walls::PublicWalls::get();
      if (extraout_RAX_03 == 0) {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      country_core::resources::walls::public_wall_state::PublicWallState::max_y();
      country_core::resources::walls::decorator::DecoratorExtraInfo::as_chimney();
      fVar3 = *extraout_RAX_04;
      country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
      fVar34 = extraout_XMM0_Da - fVar2;
      if (fVar34 <= 0.0) {
        fVar34 = 0.0;
      }
      ::utils::wall_space::impl$1::project_unbounded();
      country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
      local_128 = (undefined1 (*) [16])CONCAT44(local_128._4_4_,extraout_EAX);
      fVar33 = extraout_XMM0_Da;
      bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
      local_1b8 = (undefined **)CONCAT44(fVar33,extraout_XMM0_Da_01);
      ::utils::wall_space::WallSpaceCoord::to_world_space();
      fVar35 = (fVar34 + 3.0 + fVar3) * 0.5;
      pauVar25 = (undefined1 (*) [16])local_138[-0xc];
      lVar29 = local_138[-9];
      auVar6 = *pauVar25;
      uVar13 = ~(uint)(ushort)((ushort)(SUB161(auVar6 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1 |
                               (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                               (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                               (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                               (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                               (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                               (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                               (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                               (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                               (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                               (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                               (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                               (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                               (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar6[0xf] >> 7) << 0xf);
      pauVar19 = pauVar25 + 1;
      fVar34 = local_270._4_4_;
      fVar33 = (float)local_268;
      fVar3 = (float)local_270;
LAB_1408134d0:
      do {
        bVar32 = lVar29 == 0;
        lVar29 = lVar29 + -1;
        if (bVar32) break;
        if ((short)uVar13 == 0) {
          do {
            auVar6 = *pauVar19;
            uVar11 = (ushort)(SUB161(auVar6 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar6 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar6 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar6 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar6 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar6 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar6 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar6 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar6 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar6 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar6 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar6 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar6 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar6 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar6 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar6[0xf] >> 7) << 0xf;
            pauVar25 = pauVar25 + -0x50;
            pauVar19 = pauVar19 + 1;
          } while (uVar11 == 0xffff);
          uVar16 = ~(uint)uVar11;
          uVar13 = uVar16 & -(uint)uVar11 - 2;
        }
        else {
          uVar16 = uVar13;
          uVar13 = uVar13 - 1 & uVar13;
        }
        uVar12 = 0;
        for (; (uVar16 & 1) == 0; uVar16 = uVar16 >> 1 | 0x80000000) {
          uVar12 = uVar12 + 1;
        }
        uVar15 = (ulonglong)uVar12;
        iVar10 = *(int *)pauVar25[uVar15 * -5 + -5];
        lVar20 = (ulonglong)(byte)pauVar25[uVar15 * -5 + -3][10] - 1;
        if (2 < (byte)(pauVar25[uVar15 * -5 + -3][10] - 2)) {
          lVar20 = 0;
        }
        if (lVar20 == 0) {
          fVar4 = *(float *)(pauVar25[uVar15 * -5 + -5] + 0xc);
          local_120 = (undefined1 (*) [16])CONCAT44(local_120._4_4_,iVar10);
          country_core::resources::walls::decorator_subtype::impl$6::into_decorator_subtype();
          uVar22 = extraout_AL_01;
          country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
          local_1d8._0_2_ = CONCAT11(uVar22,extraout_AL_02);
          std::collections::hash::map::HashMap::
          get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                    ();
          if (extraout_RAX_05 == 0) {
            local_228 = (undefined **)&local_1d8;
            local_220 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
            local_1b8 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
            local_1b0 = (undefined1 (*) [16])0x1;
            local_198 = (undefined1 (**) [16])0x0;
            local_1a8 = &local_228;
            local_1a0 = (undefined1 (*) [16])0x1;
                    // WARNING: Subroutine does not return
            core::panicking::panic_fmt();
          }
          fVar5 = *(float *)(extraout_RAX_05 + 0x30);
          local_1b8 = (undefined **)CONCAT44(extraout_XMM0_Da_00,(int)local_128);
          if ((*(float *)(extraout_RAX_05 + 0x3c) - *(float *)(extraout_RAX_05 + 0x38)) * 0.5 +
              fVar35 <= ABS(fVar4 - (fVar2 + fVar35))) {
            bVar32 = false;
            if ((int)local_120 == (int)local_100) goto LAB_1408134d0;
          }
          else {
            ::utils::wall_space::impl$1::distance();
            bVar32 = extraout_XMM0_Da_02 < fVar5 * 0.5 + 0.425;
            if ((int)local_120 == (int)local_100) goto LAB_1408134d0;
          }
        }
        else {
          if (lVar20 != 1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms::
          position();
          bVar32 = SQRT((local_1b0._0_4_ - fVar33) * (local_1b0._0_4_ - fVar33) +
                        (local_1b8._4_4_ - fVar34) * (local_1b8._4_4_ - fVar34) +
                        ((float)local_1b8 - fVar3) * ((float)local_1b8 - fVar3)) < 1.0;
          if (iVar10 == (int)local_100) goto LAB_1408134d0;
        }
      } while (!bVar32);
      *local_1e8 = local_140;
      hashbrown::map::HashMap::
      insert<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator_subtype::DecoratorSubtype,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      uVar16 = (uint)local_108;
    }
  }
  goto LAB_140813215;
LAB_1408136ea:
  if (local_f1 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_06 == 1) {
      plVar21 = extraout_RAX_06 + 1;
    }
    else if ((*extraout_RAX_06 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar21 = extraout_RAX_08, extraout_RAX_08 == (longlong *)0x0)) {
LAB_1408137cf:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar21 != 0) {
LAB_1408138b8:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar21 = -1;
    local_100 = (undefined1 (*) [16])plVar21;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_100 = *(longlong *)local_100 + 1;
  }
  return;
}


