
void __cdecl toolbar::mei::core::mei_pre_update(void)

{
  byte *pbVar1;
  undefined **_Dst;
  longlong *plVar2;
  undefined8 *puVar3;
  undefined8 uVar4;
  byte bVar5;
  byte bVar6;
  ulonglong *puVar7;
  ulonglong uVar8;
  ushort uVar9;
  undefined8 uVar10;
  ulonglong uVar11;
  undefined ***pppuVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char cVar13;
  char extraout_AL_02;
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  uint uVar14;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar15;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  undefined ***pppuVar16;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  byte bVar17;
  uint uVar18;
  int iVar19;
  longlong *in_RCX;
  ulonglong uVar20;
  undefined8 *in_RDX;
  ulonglong uVar21;
  longlong lVar22;
  longlong lVar23;
  longlong lVar24;
  undefined8 *puVar25;
  undefined1 (*pauVar26) [16];
  undefined1 (*pauVar27) [16];
  longlong lVar28;
  undefined8 *in_R8;
  undefined *puVar29;
  longlong *plVar30;
  ulonglong uVar31;
  uint uVar32;
  longlong *in_R9;
  longlong lVar33;
  longlong lVar34;
  longlong lVar35;
  ulonglong uVar36;
  undefined8 *puVar37;
  bool bVar38;
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined4 uVar43;
  undefined4 uVar45;
  undefined1 auVar44 [16];
  float fVar46;
  float fVar47;
  float fVar48;
  float fVar49;
  float fVar50;
  float unaff_XMM8_Dc;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  longlong *in_stack_00000028;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000048;
  undefined1 local_4f0 [368];
  undefined8 local_380;
  undefined8 uStack_378;
  undefined8 local_370;
  undefined **local_368;
  undefined8 uStack_360;
  undefined8 uStack_358;
  undefined **ppuStack_350;
  undefined **local_348;
  undefined8 uStack_340;
  undefined8 local_338;
  undefined8 uStack_330;
  undefined **local_328;
  undefined **ppuStack_320;
  undefined **local_318;
  undefined8 *local_2a0;
  longlong local_298;
  longlong local_290;
  undefined ***local_288;
  undefined ***local_280;
  longlong local_278;
  longlong local_270;
  longlong local_268;
  undefined **local_228;
  undefined **ppuStack_220;
  undefined ****local_218;
  undefined **ppuStack_210;
  undefined **local_208;
  undefined4 uStack_200;
  undefined4 uStack_1fc;
  undefined4 local_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined **local_1e8;
  undefined **ppuStack_1e0;
  undefined **local_1d8;
  int local_1d0;
  undefined8 local_1cc;
  undefined8 uStack_1c4;
  undefined4 local_1bc;
  undefined8 *local_178;
  longlong local_170;
  longlong local_168;
  undefined4 *local_158;
  longlong local_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  int local_12c;
  longlong local_128;
  undefined *local_120;
  longlong local_118;
  longlong local_110;
  undefined8 *local_108;
  longlong *local_100;
  undefined4 local_f4;
  undefined8 *local_f0;
  longlong *local_e8;
  longlong local_e0;
  longlong local_d8;
  longlong local_d0;
  undefined8 *local_c8;
  longlong *local_c0;
  undefined1 local_b3;
  undefined1 local_b1;
  undefined8 local_b0;
  
  local_b0 = 0xfffffffffffffffe;
  puffin::are_scopes_on();
  local_f0 = in_R8;
  if (extraout_AL == '\0') {
    lVar35 = 0;
LAB_140684582:
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    local_e0 = *in_RCX;
    *(undefined4 *)(local_e0 + 0x270) = *(undefined4 *)(*in_stack_00000028 + 0x58);
    local_168 = lVar35;
    if (*(longlong *)(local_e0 + 0x1e8) != 0) {
      uVar21 = *(ulonglong *)(local_e0 + 0x1d8);
      if (uVar21 != 0) {
        memset(*(void **)(local_e0 + 0x1d0),(int)CONCAT71((int7)((ulonglong)local_e0 >> 8),0xff),
               uVar21 + 0x11);
      }
      *(undefined8 *)(local_e0 + 0x1e8) = 0;
      uVar15 = (uVar21 + 1 & 0xfffffffffffffff8) - (uVar21 + 1 >> 3);
      if (uVar21 < 8) {
        uVar15 = uVar21;
      }
      *(ulonglong *)(local_e0 + 0x1e0) = uVar15;
    }
    local_100 = (longlong *)(local_e0 + 0x268);
    local_290 = local_e0 + 0x1a0;
    pauVar26 = *(undefined1 (**) [16])(local_e0 + 0x1a0);
    auVar40 = *pauVar26;
    pauVar27 = pauVar26 + 1;
    plVar30 = *(longlong **)(local_e0 + 0x1b8);
    uVar14 = ~(uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
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
                            (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
    local_e8 = in_R9;
    local_c8 = in_RDX;
    while (plVar30 != (longlong *)0x0) {
      if ((short)uVar14 == 0) {
        do {
          auVar40 = *pauVar27;
          uVar9 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                  (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
          pauVar26 = pauVar26 + -0xc0;
          pauVar27 = pauVar27 + 1;
        } while (uVar9 == 0xffff);
        uVar14 = ~(uint)uVar9;
        uVar32 = uVar14 & -(uint)uVar9 - 2;
      }
      else {
        uVar32 = uVar14 - 1 & uVar14;
      }
      plVar30 = (longlong *)((longlong)plVar30 + -1);
      uVar18 = 0;
      for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
        uVar18 = uVar18 + 1;
      }
      uVar14 = uVar32;
      if (*(longlong *)(pauVar26[(ulonglong)uVar18 * -0xc + -0xb] + 8) != 0) {
        local_d8 = CONCAT44(local_d8._4_4_,uVar32);
        lVar28 = *(longlong *)pauVar26[(ulonglong)uVar18 * -0xc + -0xb];
        lVar24 = lVar28 + *(longlong *)(pauVar26[(ulonglong)uVar18 * -0xc + -0xb] + 8) * 8;
        local_c0 = plVar30;
        do {
          local_128 = lVar35;
          hashbrown::map::HashMap::
          insert<country_core::systems::ui::GlobalId,country_core::systems::ui::GlobalId,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
          lVar28 = lVar28 + 8;
        } while (lVar28 != lVar24);
        plVar30 = local_c0;
        uVar14 = (uint)local_d8;
      }
    }
    fVar46 = 0.0;
    fVar47 = 0.0;
    fVar48 = 0.0;
    fVar49 = 0.0;
    if ((undefined8 *)*in_stack_00000040 != (undefined8 *)0x0) {
      uVar4 = *(undefined8 *)*in_stack_00000040;
      fVar48 = (float)uVar4;
      fVar49 = (float)((ulonglong)uVar4 >> 0x20);
    }
    uVar14 = *(uint *)(*local_e8 + 4);
    puVar7 = (ulonglong *)*local_c8;
    lVar24 = local_c8[1];
    uVar21 = *puVar7;
    uVar36 = 0;
    uVar15 = uVar21 - *(ulonglong *)(lVar24 + 0x18);
    if (uVar21 < *(ulonglong *)(lVar24 + 0x18)) {
      uVar15 = uVar36;
    }
    uVar20 = uVar21 - *(ulonglong *)(lVar24 + 0x38);
    if (uVar21 < *(ulonglong *)(lVar24 + 0x38)) {
      uVar20 = uVar36;
    }
    lVar28 = uVar15 * 0x20 + *(longlong *)(lVar24 + 8);
    uVar21 = *(ulonglong *)(lVar24 + 0x10);
    lVar22 = uVar20 * 0x20 + *(longlong *)(lVar24 + 0x28);
    uVar31 = uVar21 - uVar15;
    if (uVar21 < uVar15) {
      uVar31 = uVar36;
    }
    uVar8 = *(ulonglong *)(lVar24 + 0x30);
    if (uVar21 < uVar15) {
      lVar28 = 8;
      uVar15 = uVar21;
    }
    uVar11 = uVar8 - uVar20;
    if (uVar8 < uVar20) {
      lVar22 = 8;
      uVar11 = uVar36;
    }
    lVar24 = *(longlong *)(lVar24 + 0x40);
    if (uVar8 < uVar20) {
      uVar20 = uVar8;
    }
    *puVar7 = (lVar24 - uVar11) - uVar31;
    uVar21 = ((uVar20 + lVar24 + uVar15) - uVar8) - uVar21;
    lVar24 = lVar22;
    lVar33 = lVar28;
    do {
      uVar21 = uVar21 + 1;
      if ((lVar33 == 0) || (lVar33 == uVar31 * 0x20 + lVar28)) {
        if ((lVar24 == 0) || (lVar24 == uVar11 * 0x20 + lVar22)) goto LAB_140684826;
        lVar34 = 0;
        lVar23 = lVar24 + 0x20;
        lVar33 = lVar24;
      }
      else {
        lVar34 = lVar33 + 0x20;
        lVar23 = lVar24;
      }
      *puVar7 = uVar21;
      fVar46 = fVar46 + (float)*(undefined8 *)(lVar33 + 0x10);
      fVar47 = fVar47 + (float)((ulonglong)*(undefined8 *)(lVar33 + 0x10) >> 0x20);
      lVar24 = lVar23;
      lVar33 = lVar34;
    } while( true );
  }
  uVar21 = 0x22;
  ::core::str::impl$0::rfind<ref_<str_>_>();
  if (extraout_RAX == 1) {
    if (uVar21 != 0) {
      if (uVar21 < 0x22) {
        if ((char)(&DAT_142a17976)[uVar21] < -0x40) {
LAB_140684390:
                    // WARNING: Subroutine does not return
          ::core::str::slice_error_fail();
        }
      }
      else if (uVar21 != 0x22) goto LAB_140684390;
    }
    ::core::str::impl$0::rfind<ref_<str_>_>();
    if ((extraout_RAX_00 == 1) && (uVar21 != 0xfffffffffffffffe)) {
      if (uVar21 + 2 < 0x22) {
        if ((char)(&UNK_142a17978)[uVar21] < -0x40) {
LAB_1406843ef:
                    // WARNING: Subroutine does not return
          ::core::str::slice_error_fail();
        }
      }
      else if (uVar21 + 2 != 0x22) goto LAB_1406843ef;
    }
  }
  puVar29 = &DAT_142a17c1e;
  do {
    bVar5 = puVar29[-1];
    uVar14 = (uint)(char)bVar5;
    if ((char)bVar5 < '\0') {
      bVar17 = puVar29[-2];
      if ((char)bVar17 < -0x40) {
        bVar6 = puVar29[-3];
        if ((char)bVar6 < -0x40) {
          pbVar1 = puVar29 + -4;
          puVar29 = puVar29 + -4;
          uVar14 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
        }
        else {
          puVar29 = puVar29 + -3;
          uVar14 = bVar6 & 0xf;
        }
        uVar14 = bVar17 & 0x3f | uVar14 << 6;
      }
      else {
        puVar29 = puVar29 + -2;
        uVar14 = bVar17 & 0x1f;
      }
      uVar14 = (uint)(bVar5 & 0x3f) | uVar14 << 6;
      if (uVar14 != 0x5c) goto LAB_1406844b4;
LAB_1406844c5:
      if (puVar29 == &UNK_142a17bf7) break;
      if (puVar29 + -0x142a17bf7 < (undefined *)0x26) {
        if (-0x41 < (char)puVar29[1]) break;
      }
      else if (puVar29 + -0x142a17bf7 == (undefined *)0x26) break;
                    // WARNING: Subroutine does not return
      ::core::str::slice_error_fail();
    }
    puVar29 = puVar29 + -1;
    if (uVar14 == 0x5c) goto LAB_1406844c5;
LAB_1406844b4:
    if (uVar14 == 0x2f) goto LAB_1406844c5;
  } while (puVar29 != &anon_3649a92b8b6597a9c033fd5390173f61_115_llvm_5378289450998296588);
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_01 == 1) {
    plVar30 = extraout_RAX_01 + 1;
  }
  else if ((*extraout_RAX_01 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar30 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
  goto LAB_1406855e8;
  if (*plVar30 == 0) {
    *plVar30 = -1;
    local_c0 = plVar30;
    puffin::ThreadProfiler::begin_scope();
    *local_c0 = *local_c0 + 1;
    lVar35 = 1;
    goto LAB_140684582;
  }
  goto LAB_1406855f4;
LAB_140684826:
  local_128 = lVar35;
  country_core::input::action_set::MainActionSet::deconstruct();
  puVar25 = local_f0;
  fVar50 = (float)uVar14 / 1000.0;
  auVar39._4_4_ = fVar49;
  auVar39._0_4_ = fVar48;
  auVar39._8_8_ = 0;
  auVar40._4_4_ = fVar50;
  auVar40._0_4_ = fVar50;
  auVar40._8_4_ = unaff_XMM8_Dc;
  auVar40._12_4_ = unaff_XMM8_Dc;
  auVar40 = divps(auVar39,auVar40);
  *local_100 = auVar40._0_8_;
  local_c0 = (longlong *)*local_f0;
  if (((int)local_c0[2] == 2) && (local_c8 = local_108, local_108 != (undefined8 *)0x0)) {
    local_d8 = local_110;
    lVar35 = 0;
    puVar37 = (undefined8 *)0x0;
    do {
      local_170 = 0;
      local_178 = puVar37;
      process_input_action_handlers();
      puVar37 = (undefined8 *)((longlong)puVar37 + 1);
      if (extraout_AL_00 != '\0') {
        lVar35 = 1;
        if (puVar37 != local_c8) {
          do {
            puVar3 = (undefined8 *)(local_d8 + (longlong)puVar37 * 0xc);
            local_178 = puVar37;
            local_170 = lVar35;
            process_input_action_handlers();
            if (extraout_AL_01 == '\0') {
              *(undefined4 *)(local_d8 + 8 + ((longlong)puVar37 - lVar35) * 0xc) =
                   *(undefined4 *)(puVar3 + 1);
              *(undefined8 *)(local_d8 + ((longlong)puVar37 - lVar35) * 0xc) = *puVar3;
            }
            else {
              lVar35 = lVar35 + 1;
            }
            puVar37 = (undefined8 *)((longlong)puVar37 + 1);
          } while (puVar37 != local_c8);
        }
        break;
      }
    } while (puVar37 != local_c8);
    local_108 = (undefined8 *)((longlong)local_c8 - lVar35);
  }
  local_288 = (undefined ***)CONCAT71(local_288._1_7_,(char)local_c0[6]);
  local_368 = (undefined **)((ulonglong)local_368 & 0xffffffffffffff00);
  iVar19 = (int)&local_368;
  alloc::vec::Vec::
  retain<enum2_<country_core::input::action_set::ActionSetAction>,alloc::alloc::Global,toolbar::mei::core::mei_pre_update::closure_env_2>
            ();
  if ((char)local_368 == '\0') {
    iVar19 = (int)local_c0[5];
    if (iVar19 != 1000000000) goto LAB_1406849ee;
LAB_140684a2d:
    bVar38 = false;
  }
  else {
    std::time::Instant::now();
    *(undefined4 *)puVar25[2] = *(undefined4 *)((longlong)puVar25 + 0x1c);
    local_c0[4] = extraout_RAX_02;
    *(int *)(local_c0 + 5) = iVar19;
    if (iVar19 == 1000000000) goto LAB_140684a2d;
LAB_1406849ee:
    local_228 = (undefined **)local_c0[4];
    ppuStack_220 = (undefined **)CONCAT44(ppuStack_220._4_4_,iVar19);
    std::time::Instant::elapsed();
    bVar38 = extraout_RAX_03 < 0x78;
  }
  *(undefined4 *)puVar25[2] = *(undefined4 *)((longlong)puVar25 + 0x1c);
  *(bool *)(local_c0 + 6) = bVar38;
  country_core::input::action_set::MainActionSet::reconstruct();
  lVar35 = local_e0;
  _Dst = (undefined **)(local_e0 + 0x30);
  memcpy(_Dst,local_4f0,0x170);
  pauVar26 = *(undefined1 (**) [16])(lVar35 + 0x1a0);
  lVar35 = *(longlong *)(lVar35 + 0x1b8);
  auVar40 = *pauVar26;
  uVar14 = ~(uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
  pauVar27 = pauVar26 + 1;
  while (lVar35 != 0) {
    if ((short)uVar14 == 0) {
      do {
        auVar40 = *pauVar27;
        uVar9 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
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
                (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
        pauVar26 = pauVar26 + -0xc0;
        pauVar27 = pauVar27 + 1;
      } while (uVar9 == 0xffff);
      uVar32 = ~(uint)uVar9;
      uVar14 = uVar32 & -(uint)uVar9 - 2;
    }
    else {
      uVar32 = uVar14;
      uVar14 = uVar14 - 1 & uVar14;
    }
    lVar35 = lVar35 + -1;
    uVar18 = 0;
    for (; (uVar32 & 1) == 0; uVar32 = uVar32 >> 1 | 0x80000000) {
      uVar18 = uVar18 + 1;
    }
    uVar21 = (ulonglong)uVar18;
    if (*(longlong *)(pauVar26[uVar21 * -0xc + -4] + 8) != 0) {
      ppuStack_210 = (undefined **)CONCAT44(ppuStack_210._4_4_,(int)local_100[1]);
      local_218 = (undefined ****)*local_100;
      local_228 = (undefined **)(pauVar26[uVar21 * -0xc + -1] + 0xe);
      ppuStack_220 = _Dst;
      (**(code **)(*(longlong *)pauVar26[uVar21 * -0xc + -3] + 0x58))
                (*(longlong *)(pauVar26[uVar21 * -0xc + -4] + 8),&local_228);
    }
  }
  pauVar26 = *(undefined1 (**) [16])(local_e0 + 0x1a0);
  auVar40 = *pauVar26;
  pauVar27 = pauVar26 + 1;
  local_d8 = *in_stack_00000038;
  local_298 = local_e0 + 0x18;
  local_12c = -(uint)(fVar46 == 0.0 && fVar47 == 0.0);
  local_f4 = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  local_e8 = (longlong *)*in_stack_00000048;
  local_158 = (undefined4 *)in_stack_00000048[2];
  local_150 = (longlong)local_e8 + 0x20;
  plVar30 = *(longlong **)(local_e0 + 0x1b8);
  uVar14 = ~(uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
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
                          (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
  while (plVar30 != (longlong *)0x0) {
    if ((short)uVar14 == 0) {
      do {
        auVar40 = *pauVar27;
        uVar9 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
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
                (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
        pauVar26 = pauVar26 + -0xc0;
        pauVar27 = pauVar27 + 1;
      } while (uVar9 == 0xffff);
      uVar14 = ~(uint)uVar9;
      uVar32 = uVar14 & -(uint)uVar9 - 2;
    }
    else {
      uVar32 = uVar14 - 1 & uVar14;
    }
    uVar18 = 0;
    for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
      uVar18 = uVar18 + 1;
    }
    uVar21 = (ulonglong)uVar18;
    local_120 = *(undefined **)pauVar26[uVar21 * -0xc + -0xc];
    bVar5 = pauVar26[uVar21 * -0xc + -1][0xe];
    bVar17 = pauVar26[uVar21 * -0xc + -1][0xf];
    uVar18 = (uint)bVar17;
    uVar14 = (uint)bVar17;
    if ((bVar17 & 2) == 0) {
      cVar13 = '\x01';
      if ((bVar17 & 1) == 0) {
        lVar35 = *(longlong *)(pauVar26[uVar21 * -0xc + -4] + 8);
        uVar14 = 0;
        if (lVar35 == 0) goto LAB_140684cd0;
        cVar13 = (**(code **)(*(longlong *)pauVar26[uVar21 * -0xc + -3] + 0x40))();
        uVar18 = (uint)lVar35;
      }
    }
    else {
LAB_140684cd0:
      uVar18 = uVar14;
      cVar13 = '\0';
    }
    fVar51 = (float)*(undefined8 *)pauVar26[uVar21 * -0xc + -4] * fVar50;
    fVar52 = (float)((ulonglong)*(undefined8 *)pauVar26[uVar21 * -0xc + -4] >> 0x20) * fVar50;
    fVar53 = (float)*(undefined8 *)(pauVar26[uVar21 * -0xc + -5] + 8) * fVar50 + fVar51;
    fVar54 = (float)((ulonglong)*(undefined8 *)(pauVar26[uVar21 * -0xc + -5] + 8) >> 0x20) * fVar50
             + fVar52;
    auVar41._4_4_ = -(uint)(fVar49 < fVar52);
    auVar41._0_4_ = -(uint)(fVar48 < fVar51);
    auVar41._8_4_ = -(uint)(0.0 < unaff_XMM8_Dc * 0.0);
    auVar41._12_4_ = -(uint)(0.0 < unaff_XMM8_Dc * 0.0);
    auVar44._4_4_ = -(uint)(fVar54 < fVar49);
    auVar44._0_4_ = -(uint)(fVar53 < fVar48);
    auVar44._8_4_ = -(uint)(unaff_XMM8_Dc * 0.0 + unaff_XMM8_Dc * 0.0 < 0.0);
    auVar44._12_4_ = -(uint)(unaff_XMM8_Dc * 0.0 + unaff_XMM8_Dc * 0.0 < 0.0);
    uVar43 = SUB164(auVar44 | auVar41,0);
    uVar45 = SUB164(auVar44 | auVar41,4);
    auVar42._4_4_ = uVar43;
    auVar42._0_4_ = uVar43;
    auVar42._8_4_ = uVar45;
    auVar42._12_4_ = uVar45;
    iVar19 = movmskpd(uVar18,auVar42);
    if ((iVar19 == 0) ||
       ((((int)local_c0[2] == 2 && ((undefined *)local_c0[3] == local_120)) &&
        ((char)local_c0[6] != '\0')))) {
      bVar17 = pauVar26[uVar21 * -0xc + -1][0xe] | 1;
    }
    else {
      bVar17 = pauVar26[uVar21 * -0xc + -1][0xe] & 0xfe;
    }
    plVar30 = (longlong *)((longlong)plVar30 + -1);
    pauVar26[uVar21 * -0xc + -1][0xe] = bVar17;
    uVar14 = uVar32;
    if (cVar13 != '\0') {
      local_f0 = (undefined8 *)CONCAT71(local_f0._1_7_,iVar19 != 0);
      local_c8 = (undefined8 *)CONCAT44(local_c8._4_4_,uVar32);
      plVar2 = (longlong *)(pauVar26[uVar21 * -0xc + -4] + 8);
      local_100 = plVar30;
      if (local_d8 == 0) {
        if (*plVar2 == 0) {
          local_368 = (undefined **)0x142a17c68;
          uStack_360 = (code *)&DAT_0000000c;
        }
        else {
          local_368 = (undefined **)
                      (**(code **)(*(longlong *)pauVar26[uVar21 * -0xc + -3] + 0x30))();
          uStack_360 = (code *)plVar30;
        }
        local_288 = &local_368;
        local_280 = (undefined ***)::core::fmt::impl$74::fmt<str_>;
        local_228 = (undefined **)&DAT_142a17ca8;
        ppuStack_220 = (undefined **)0x1;
        local_208 = (undefined **)0x0;
        local_218 = &local_288;
        ppuStack_210 = (undefined **)0x1;
                    // WARNING: Subroutine does not return
        ::core::panicking::panic_fmt();
      }
      local_368 = &local_120;
      uStack_360 = ::core::fmt::num::impl$64::fmt;
      local_228 = &PTR_DAT_142a17c58;
      ppuStack_220 = (undefined **)0x1;
      ppuStack_210 = (undefined **)0x1;
      local_208 = (undefined **)0x0;
      local_218 = (undefined ****)&local_368;
      alloc::fmt::format::format_inner();
      local_148 = local_380;
      uStack_140 = uStack_378;
      local_138 = local_370;
      uStack_360 = (code *)CONCAT44(fVar52,fVar51);
      uStack_358 = (undefined ***)CONCAT44(fVar54,fVar53);
      uVar4 = CONCAT44(*(undefined4 *)(pauVar26[uVar21 * -0xc + -1] + 4),local_368._0_4_);
      local_368 = (undefined **)CONCAT71((uint7)((ulonglong)uVar4 >> 8) ^ 0x80000000000000,3);
      local_288 = (undefined ***)((ulonglong)local_288 & 0xffffffffffffff00);
      local_b1 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::new();
      local_b1 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording();
      local_b1 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
      local_b1 = 1;
      country_core::systems::ui::resources::widget_params::WidgetParams::allow_negative_distance();
      local_b3 = 1;
      country_core::systems::ui::resources::widget_params::UiSounds::empty();
      local_b3 = 0;
      country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
      local_b1 = 0;
      puVar25 = &local_148;
      country_core::systems::ui::resources::ui_state::UiState::add<alloc::string::String>();
      local_2a0 = puVar25;
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      country_core::systems::ui::resources::ui_state::DragState::is_some();
      pauVar26[uVar21 * -0xc + -1][0xe] =
           extraout_AL_02 << 2 | pauVar26[uVar21 * -0xc + -1][0xe] & 0xfb;
      country_core::systems::ui::resources::ui_state::DragState::is_started();
      pauVar26[uVar21 * -0xc + -1][0xe] =
           extraout_AL_03 * '\x02' | pauVar26[uVar21 * -0xc + -1][0xe] & 0xfd;
      country_core::systems::ui::resources::ui_state::DragState::is_finished();
      pauVar26[uVar21 * -0xc + -1][0xe] =
           extraout_AL_04 << 3 | pauVar26[uVar21 * -0xc + -1][0xe] & 0xf7;
      country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
      if ((extraout_AL_05 != '\0') ||
         ((((int)local_c0[2] == 2 && ((undefined *)local_c0[3] == local_120)) &&
          (country_core::input::action_set::DigitalActionState::just_pressed(),
          extraout_AL_06 != '\0')))) {
        pauVar26[uVar21 * -0xc + -1][0xe] = pauVar26[uVar21 * -0xc + -1][0xe] | 0x10;
      }
      if (*plVar2 != 0) {
        if ((pauVar26[uVar21 * -0xc + -1][0xe] & 0x40) != 0) {
          local_270 = *(longlong *)(local_e0 + 0x28);
          *(undefined8 *)(local_e0 + 0x28) = 0;
          local_280 = *(undefined ****)(local_e0 + 0x20) + local_270 * 0xe;
          local_278 = local_298;
          local_268 = 0;
          pppuVar16 = *(undefined ****)(local_e0 + 0x20);
          while (lVar35 = local_268, pppuVar12 = local_280, pppuVar16 != local_280) {
            local_368 = *pppuVar16;
            uStack_360 = (code *)pppuVar16[1];
            uStack_358 = (undefined ***)pppuVar16[2];
            ppuStack_350 = pppuVar16[3];
            local_348 = pppuVar16[4];
            uStack_340 = pppuVar16[5];
            local_338 = pppuVar16[6];
            uStack_330 = pppuVar16[7];
            local_328 = pppuVar16[8];
            ppuStack_320 = pppuVar16[9];
            local_318 = pppuVar16[10];
            local_138 = CONCAT44(local_138._4_4_,*(undefined4 *)((longlong)pppuVar16 + 0x6c));
            local_148 = *(undefined8 *)((longlong)pppuVar16 + 0x5c);
            uStack_140 = *(undefined8 *)((longlong)pppuVar16 + 100);
            if (*(int *)(pppuVar16 + 0xb) == 0xc6) {
              local_288 = (undefined ***)&DAT_00000008;
              local_280 = (undefined ***)&DAT_00000008;
              if (pppuVar12 != pppuVar16 + 0xe) {
                local_d0 = local_278;
                ::core::ptr::drop_in_place<slice2_<enum2_<toolbar::mei::core::MeiInputEvent>_>_>();
                lVar35 = local_268;
                if (local_268 != 0) {
                  lVar24 = *(longlong *)(local_d0 + 0x10);
                  if (local_270 != lVar24) {
                    memmove((void *)(lVar24 * 0x70 + *(longlong *)(local_d0 + 8)),
                            (void *)(local_270 * 0x70 + *(longlong *)(local_d0 + 8)),
                            local_268 * 0x70);
                  }
                  *(longlong *)(local_d0 + 0x10) = lVar24 + lVar35;
                }
                goto LAB_1406852e5;
              }
              break;
            }
            uStack_340._4_4_ = (undefined4)((ulonglong)uStack_340 >> 0x20);
            local_338._4_4_ = (undefined4)((ulonglong)local_338 >> 0x20);
            uStack_330._4_4_ = (undefined4)((ulonglong)uStack_330 >> 0x20);
            local_1f8 = (undefined4)local_338;
            uStack_1f4 = local_338._4_4_;
            uStack_1f0 = (undefined4)uStack_330;
            uStack_1ec = uStack_330._4_4_;
            uStack_200 = (undefined4)uStack_340;
            uStack_1fc = uStack_340._4_4_;
            local_288 = pppuVar16 + 0xe;
            local_228 = local_368;
            ppuStack_220 = (undefined **)uStack_360;
            local_218 = (undefined ****)uStack_358;
            ppuStack_210 = ppuStack_350;
            local_208 = local_348;
            local_1e8 = local_328;
            ppuStack_1e0 = ppuStack_320;
            local_1d8 = local_318;
            local_1d0 = *(int *)(pppuVar16 + 0xb);
            local_1cc = local_148;
            uStack_1c4 = uStack_140;
            local_1bc = *(undefined4 *)((longlong)pppuVar16 + 0x6c);
            (**(code **)(*(longlong *)pauVar26[uVar21 * -0xc + -3] + 0x48))(*plVar2,&local_228);
            uVar14 = local_1d0 - 0xc3;
            if (2 < uVar14) {
              uVar14 = 1;
            }
            pppuVar16 = local_288;
            if (uVar14 != 0) {
              if (uVar14 == 1) {
                ::core::ptr::drop_in_place<bevy_input::keyboard::KeyboardInput>();
                pppuVar16 = local_288;
                if (((char)local_228 != '\x1b') && ((char)local_228 == '\x18')) {
                  LOCK();
                  *ppuStack_220 = *ppuStack_220 + -1;
                  UNLOCK();
                  if (*ppuStack_220 == (undefined *)0x0) {
                    alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
                    pppuVar16 = local_288;
                  }
                }
              }
              else if (local_228 != (undefined **)0x0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
                pppuVar16 = local_288;
              }
            }
          }
          if (local_268 != 0) {
            lVar24 = *(longlong *)(local_278 + 0x10);
            lVar28 = local_278;
            if (local_270 != lVar24) {
              local_d0 = local_278;
              memmove((void *)(lVar24 * 0x70 + *(longlong *)(local_278 + 8)),
                      (void *)(local_270 * 0x70 + *(longlong *)(local_278 + 8)),local_268 * 0x70);
              lVar28 = local_d0;
            }
            *(longlong *)(lVar28 + 0x10) = lVar24 + lVar35;
          }
        }
LAB_1406852e5:
        if ((((byte)local_f0 | (byte)local_12c) & 1) == 0) {
          local_228 = (undefined **)CONCAT44(fVar47,fVar46);
          local_1d0 = 0xc3;
          (**(code **)(*(longlong *)pauVar26[uVar21 * -0xc + -3] + 0x48))(*plVar2,&local_228);
          uVar14 = local_1d0 - 0xc3;
          if (2 < uVar14) {
            uVar14 = 1;
          }
          if (uVar14 != 0) {
            if (uVar14 == 1) {
              ::core::ptr::drop_in_place<bevy_input::keyboard::KeyboardInput>();
              if (((char)local_228 != '\x1b') && ((char)local_228 == '\x18')) {
                LOCK();
                *ppuStack_220 = *ppuStack_220 + -1;
                UNLOCK();
                if (*ppuStack_220 == (undefined *)0x0) {
                  alloc::sync::Arc::drop_slow<str_,alloc::alloc::Global>();
                }
              }
            }
            else if (local_228 != (undefined **)0x0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
          }
        }
      }
      bVar17 = pauVar26[uVar21 * -0xc + -1][0xe];
      if ((((bVar5 & 1) == 0 & bVar17) != 0) &&
         (cVar13 = pauVar26[uVar21 * -0xc + -1][0xc], cVar13 != '\r')) {
        *local_158 = local_f4;
        lVar35 = *(longlong *)((longlong)local_e8 + 0x30);
        local_f0 = *(undefined8 **)((longlong)local_e8 + 0x40);
        if (lVar35 == *(longlong *)((longlong)local_e8 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar24 = *(longlong *)((longlong)local_e8 + 0x28);
        *(undefined8 **)(lVar24 + lVar35 * 0x10) = local_f0;
        *(char *)(lVar24 + 8 + lVar35 * 0x10) = cVar13;
        *(longlong *)((longlong)local_e8 + 0x30) = lVar35 + 1;
        *(longlong *)((longlong)local_e8 + 0x40) = *(longlong *)((longlong)local_e8 + 0x40) + 1;
      }
      if (((bVar5 & 0x30) == 0 && (bVar17 & 0x30) != 0) &&
         (cVar13 = pauVar26[uVar21 * -0xc + -1][0xd], cVar13 != '\r')) {
        *local_158 = local_f4;
        lVar35 = *(longlong *)((longlong)local_e8 + 0x30);
        uVar4 = *(undefined8 *)((longlong)local_e8 + 0x40);
        if (lVar35 == *(longlong *)((longlong)local_e8 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar24 = *(longlong *)((longlong)local_e8 + 0x28);
        *(undefined8 *)(lVar24 + lVar35 * 0x10) = uVar4;
        *(char *)(lVar24 + 8 + lVar35 * 0x10) = cVar13;
        *(longlong *)((longlong)local_e8 + 0x30) = lVar35 + 1;
        *(longlong *)((longlong)local_e8 + 0x40) = *(longlong *)((longlong)local_e8 + 0x40) + 1;
      }
      plVar30 = local_100;
      uVar14 = (uint)local_c8;
    }
  }
  *(undefined8 *)(local_e0 + 0x10) = 0;
  uVar4 = *(undefined8 *)(local_e0 + 0x200);
  uVar10 = *(undefined8 *)(local_e0 + 0x208);
  *(undefined8 *)(local_e0 + 0x200) = *(undefined8 *)(local_e0 + 0x1a0);
  *(undefined8 *)(local_e0 + 0x208) = *(undefined8 *)(local_e0 + 0x1a8);
  *(undefined8 *)(local_e0 + 0x1a0) = uVar4;
  *(undefined8 *)(local_e0 + 0x1a8) = uVar10;
  uVar4 = *(undefined8 *)(local_e0 + 0x210);
  uVar10 = *(undefined8 *)(local_e0 + 0x218);
  *(undefined8 *)(local_e0 + 0x210) = *(undefined8 *)(local_e0 + 0x1b0);
  *(undefined8 *)(local_e0 + 0x218) = *(undefined8 *)(local_e0 + 0x1b8);
  *(undefined8 *)(local_e0 + 0x1b0) = uVar4;
  *(undefined8 *)(local_e0 + 0x1b8) = uVar10;
  uVar4 = *(undefined8 *)(local_e0 + 0x220);
  uVar10 = *(undefined8 *)(local_e0 + 0x228);
  *(undefined8 *)(local_e0 + 0x220) = *(undefined8 *)(local_e0 + 0x1c0);
  *(undefined8 *)(local_e0 + 0x228) = *(undefined8 *)(local_e0 + 0x1c8);
  *(undefined8 *)(local_e0 + 0x1c0) = uVar4;
  *(undefined8 *)(local_e0 + 0x1c8) = uVar10;
  hashbrown::raw::RawTable::
  clear<tuple_<country_core::systems::ui::GlobalId,toolbar::mei::core::Node>,alloc::alloc::Global>()
  ;
  if (local_118 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_168 != 0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar30 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0)) {
LAB_1406855e8:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar30 != 0) {
LAB_1406855f4:
                    // WARNING: Subroutine does not return
      ::core::cell::panic_already_borrowed();
    }
    *plVar30 = -1;
    local_c0 = plVar30;
    puffin::ThreadProfiler::end_scope();
    *local_c0 = *local_c0 + 1;
  }
  return;
}

