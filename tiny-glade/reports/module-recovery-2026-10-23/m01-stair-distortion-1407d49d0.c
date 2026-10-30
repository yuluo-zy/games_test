
void __cdecl
system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph
          (void)

{
  byte *pbVar1;
  ulonglong *puVar2;
  char *pcVar3;
  float fVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  byte bVar7;
  void *_Src;
  uint uVar8;
  uint uVar9;
  uint uVar10;
  undefined5 uVar11;
  undefined1 (*pauVar12) [16];
  undefined1 (*pauVar13) [16];
  undefined1 *puVar14;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  uint uVar15;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  float *extraout_RAX_03;
  undefined8 extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  ulonglong extraout_RAX_13;
  longlong extraout_RAX_14;
  ulonglong extraout_RAX_15;
  ulonglong extraout_RAX_16;
  ulonglong extraout_RAX_17;
  undefined8 *puVar16;
  longlong *extraout_RAX_18;
  longlong *extraout_RAX_19;
  longlong *extraout_RAX_20;
  longlong extraout_RAX_21;
  ulonglong extraout_RAX_22;
  ulonglong extraout_RAX_23;
  longlong *extraout_RAX_24;
  ulonglong extraout_RAX_25;
  longlong *extraout_RAX_26;
  longlong *extraout_RAX_27;
  longlong extraout_RAX_28;
  ulonglong extraout_RAX_29;
  ulonglong extraout_RAX_30;
  longlong *extraout_RAX_31;
  longlong *extraout_RAX_32;
  longlong *extraout_RAX_33;
  longlong *extraout_RAX_34;
  longlong extraout_RAX_35;
  ulonglong extraout_RAX_36;
  ulonglong extraout_RAX_37;
  ulonglong uVar17;
  ulonglong extraout_RAX_38;
  ulonglong extraout_RAX_39;
  ulonglong extraout_RAX_40;
  ulonglong extraout_RAX_41;
  undefined1 (*extraout_RAX_42) [16];
  ulonglong extraout_RAX_43;
  undefined1 (*extraout_RAX_44) [16];
  ulonglong extraout_RAX_45;
  longlong *extraout_RAX_46;
  longlong *extraout_RAX_47;
  longlong *extraout_RAX_48;
  longlong *extraout_RAX_49;
  longlong *extraout_RAX_50;
  float *pfVar18;
  undefined4 *puVar19;
  longlong *in_RDX;
  ulonglong uVar20;
  undefined1 (*pauVar21) [16];
  undefined1 (**ppauVar22) [16];
  uint uVar23;
  undefined1 (*pauVar24) [16];
  longlong *plVar25;
  ushort uVar26;
  longlong lVar27;
  float *pfVar28;
  undefined8 *puVar29;
  ulonglong uVar30;
  undefined8 *puVar31;
  ulonglong uVar32;
  undefined **ppuVar33;
  ulonglong uVar34;
  longlong in_R9;
  undefined1 (*pauVar35) [16];
  ulonglong uVar36;
  longlong lVar37;
  longlong lVar38;
  ulonglong uVar39;
  longlong lVar40;
  undefined1 (*pauVar41) [16];
  byte bVar42;
  longlong *plVar43;
  char cVar44;
  byte bVar45;
  char cVar47;
  char cVar48;
  char cVar49;
  undefined1 auVar46 [16];
  float fVar50;
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  undefined1 auVar60 [16];
  undefined1 auVar61 [16];
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  undefined1 auVar65 [16];
  undefined1 auVar66 [16];
  undefined1 auVar67 [16];
  undefined1 auVar68 [16];
  undefined1 auVar69 [16];
  undefined1 auVar70 [16];
  undefined1 auVar71 [16];
  undefined1 auVar72 [16];
  undefined1 auVar73 [16];
  undefined1 auVar74 [16];
  undefined1 auVar75 [16];
  undefined1 auVar76 [16];
  undefined1 auVar77 [16];
  int iVar100;
  int iVar102;
  undefined1 auVar78 [16];
  undefined1 auVar79 [16];
  undefined1 auVar80 [16];
  undefined1 auVar81 [16];
  undefined1 auVar82 [16];
  undefined1 auVar83 [16];
  undefined1 auVar84 [16];
  undefined1 auVar85 [16];
  undefined1 auVar86 [16];
  undefined1 auVar87 [16];
  undefined1 auVar88 [16];
  undefined1 auVar89 [16];
  int iVar101;
  undefined1 auVar90 [16];
  undefined1 auVar91 [16];
  undefined1 auVar92 [16];
  undefined1 auVar93 [16];
  undefined1 auVar94 [16];
  undefined1 auVar95 [16];
  undefined1 auVar96 [16];
  undefined1 auVar97 [16];
  undefined1 auVar98 [16];
  undefined1 auVar99 [16];
  float fVar103;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  undefined1 (*local_330) [16];
  undefined4 local_328;
  undefined8 local_320;
  undefined4 local_318;
  longlong *local_310;
  longlong local_308;
  undefined1 (*local_300) [16];
  undefined1 (*local_2f8) [16];
  undefined1 (*local_2f0) [16];
  undefined1 (*pauStack_2e8) [16];
  longlong local_2e0;
  undefined1 (*local_2d8) [16];
  undefined8 uStack_2d0;
  undefined1 (*local_2c8) [16];
  longlong *plStack_2c0;
  longlong local_2b8;
  longlong lStack_2b0;
  longlong local_290;
  longlong local_278;
  undefined1 (*local_258) [16];
  uint local_250;
  longlong local_248;
  longlong *local_240;
  undefined1 (*local_238) [16];
  longlong local_230;
  longlong local_228;
  undefined1 (*local_218) [16];
  undefined8 uStack_210;
  undefined1 (*local_208) [16];
  longlong *plStack_200;
  longlong local_1f8;
  longlong lStack_1f0;
  undefined1 (*local_1d8) [16];
  undefined4 local_1d0;
  undefined8 local_1c8;
  undefined1 (*local_1c0) [16];
  uint local_1b8;
  undefined1 (*local_1b0) [16];
  undefined1 (**local_1a8) [16];
  undefined4 local_1a0;
  longlong local_198;
  undefined1 (*local_190) [16];
  undefined4 local_188;
  undefined2 local_180;
  undefined1 uStack_17e;
  undefined4 uStack_17d;
  undefined1 (*local_178) [16];
  longlong local_170;
  undefined1 (*local_168) [16];
  undefined4 local_160;
  undefined1 (**local_158) [16];
  undefined8 local_150;
  undefined2 local_144;
  undefined1 local_142;
  undefined2 local_140;
  undefined1 local_13e;
  undefined2 local_13c;
  undefined1 local_13a;
  undefined1 (*local_138) [16];
  undefined8 uStack_130;
  undefined1 *local_128;
  ushort local_120;
  longlong *local_110;
  undefined1 (*local_108) [16];
  char local_f9;
  float *local_f8;
  longlong *local_f0;
  longlong local_e8;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  local_230 = *in_stack_00000030;
  local_310 = in_RDX;
  puffin::are_scopes_on();
  local_f9 = extraout_AL;
  if (extraout_AL != '\0') {
    uVar20 = 0x5f;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar20 != 0) {
        if (uVar20 < 0x5f) {
          if ((char)(&DAT_142a87b29)[uVar20] < -0x40) {
LAB_1407d4ab4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 != 0x5f) goto LAB_1407d4ab4;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar20 != 0xfffffffffffffffe)) {
        if (uVar20 + 2 < 0x5f) {
          if ((char)(&UNK_142a87b2b)[uVar20] < -0x40) {
LAB_1407d4b0b:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar20 + 2 != 0x5f) goto LAB_1407d4b0b;
      }
    }
    ppuVar33 = &PTR_DAT_142a87f40;
    do {
      bVar42 = *(byte *)((longlong)ppuVar33 + -1);
      uVar15 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar45 = *(byte *)((longlong)ppuVar33 + -2);
        if ((char)bVar45 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar33 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar33 + -4);
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
            uVar15 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
            uVar15 = bVar7 & 0xf;
          }
          uVar15 = bVar45 & 0x3f | uVar15 << 6;
        }
        else {
          ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
          uVar15 = bVar45 & 0x1f;
        }
        uVar15 = (uint)(bVar42 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_1407d4bc4;
LAB_1407d4bd5:
        if (ppuVar33 == (undefined **)&UNK_142a87eff) break;
        if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar33 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
      if (uVar15 == 0x5c) goto LAB_1407d4bd5;
LAB_1407d4bc4:
      if (uVar15 == 0x2f) goto LAB_1407d4bd5;
    } while (ppuVar33 != (undefined **)&DAT_142a87f00);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar43 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar43 = extraout_RAX_49, extraout_RAX_49 == (longlong *)0x0)) {
LAB_1407d7c1b:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar43 != 0) {
LAB_1407d7c62:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar43 = -1;
    local_f0 = plVar43;
    puffin::ThreadProfiler::begin_scope();
    *local_f0 = *local_f0 + 1;
  }
  *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  local_e8 = *in_stack_00000028;
  **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
  tabloid::Subscriber::take<country_core::systems::collision::world::ColliderDiff>();
  if (local_2f0 != (undefined1 (*) [16])0x0) {
    local_128 = (undefined1 *)local_2e0;
    local_138 = local_2f0;
    uStack_130 = pauStack_2e8;
    puffin::are_scopes_on();
    local_f0 = (longlong *)CONCAT71(local_f0._1_7_,extraout_AL_00);
    if (extraout_AL_00 == '\0') {
      local_158 = (undefined1 (**) [16])0x0;
    }
    else {
      ppuVar33 = &PTR_DAT_142a87f40;
      do {
        bVar42 = *(byte *)((longlong)ppuVar33 + -1);
        uVar15 = (uint)(char)bVar42;
        if ((char)bVar42 < '\0') {
          bVar45 = *(byte *)((longlong)ppuVar33 + -2);
          if ((char)bVar45 < -0x40) {
            bVar7 = *(byte *)((longlong)ppuVar33 + -3);
            if ((char)bVar7 < -0x40) {
              pbVar1 = (byte *)((longlong)ppuVar33 + -4);
              ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
              uVar15 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
            }
            else {
              ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
              uVar15 = bVar7 & 0xf;
            }
            uVar15 = bVar45 & 0x3f | uVar15 << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
            uVar15 = bVar45 & 0x1f;
          }
          uVar15 = (uint)(bVar42 & 0x3f) | uVar15 << 6;
          if (uVar15 != 0x5c) goto LAB_1407d4d84;
LAB_1407d4d9c:
          if (ppuVar33 == (undefined **)&UNK_142a87eff) break;
          if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
            if (-0x41 < *(char *)((longlong)ppuVar33 + 1)) break;
          }
          else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
          core::str::slice_error_fail();
        }
        ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
        if (uVar15 == 0x5c) goto LAB_1407d4d9c;
LAB_1407d4d84:
        if (uVar15 == 0x2f) goto LAB_1407d4d9c;
      } while (ppuVar33 != (undefined **)&DAT_142a87f00);
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_02 == 0) {
        std::sys::thread_local::native::lazy::Storage::
        initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                  ();
        local_f8 = extraout_RAX_03;
        if (extraout_RAX_03 == (float *)0x0) goto LAB_1407d7dbf;
      }
      else {
        if ((int)*extraout_RAX_02 != 1) {
LAB_1407d7dbf:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        local_f8 = (float *)(extraout_RAX_02 + 1);
      }
      if (*(longlong *)local_f8 != 0) {
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      local_f8[0] = -NAN;
      local_f8[1] = -NAN;
      puffin::ThreadProfiler::begin_scope();
      *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
      local_158 = (undefined1 (**) [16])0x1;
      local_150 = extraout_RAX_04;
    }
    pauVar41 = uStack_130;
    pauVar35 = *(undefined1 (**) [16])uStack_130[3];
    lVar40 = *(longlong *)(uStack_130[4] + 8);
    auVar46 = *pauVar35;
    uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                             (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
    pauVar21 = pauVar35 + 1;
    while (lVar40 != 0) {
      uVar23 = uVar15;
      if ((short)uVar15 == 0) {
        do {
          auVar46 = *pauVar21;
          pauVar35 = pauVar35 + -0xc;
          pauVar21 = pauVar21 + 1;
          uVar23 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                  (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
        } while (uVar23 == 0xffff);
        uVar23 = ~uVar23;
      }
      uVar15 = uVar23 - 1 & uVar23;
      lVar40 = lVar40 + -1;
      uVar8 = 0;
      for (; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      uVar20 = (ulonglong)uVar8;
      uVar23 = *(uint *)((longlong)pauVar35 + uVar20 * -0xc + -4);
      local_13a = *(undefined1 *)((longlong)pauVar35 + uVar20 * -0xc + -1);
      local_13c = *(undefined2 *)((longlong)pauVar35 + uVar20 * -0xc + -3);
      if ((0x1a203U >> (uVar23 & 0x1f) & 1) != 0) {
        uStack_2d0._0_3_ = CONCAT21(local_13c,(char)uVar23);
        uStack_2d0 = (undefined1 (*) [16])
                     CONCAT53(CONCAT41(uStack_2d0._4_4_,local_13a),(undefined3)uStack_2d0);
        local_2d8 = *(undefined1 (**) [16])((longlong)pauVar35 + uVar20 * -0xc + -0xc);
        country_core::resources::stairs::structural_graph::StairsStructuralGraph::
        remove_node_by_collider();
      }
    }
    pauVar35 = *(undefined1 (**) [16])*pauVar41;
    lVar40 = *(longlong *)(pauVar41[1] + 8);
    auVar46 = *pauVar35;
    uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                             (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
    pauVar21 = pauVar35 + 1;
    while (lVar40 != 0) {
      uVar23 = uVar15;
      if ((short)uVar15 == 0) {
        do {
          auVar46 = *pauVar21;
          pauVar35 = pauVar35 + -0xc;
          pauVar21 = pauVar21 + 1;
          uVar23 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                  (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
        } while (uVar23 == 0xffff);
        uVar23 = ~uVar23;
      }
      uVar15 = uVar23 - 1 & uVar23;
      lVar40 = lVar40 + -1;
      uVar8 = 0;
      for (; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      uVar20 = (ulonglong)uVar8;
      uVar23 = *(uint *)((longlong)pauVar35 + uVar20 * -0xc + -4);
      local_13e = *(undefined1 *)((longlong)pauVar35 + uVar20 * -0xc + -1);
      local_140 = *(undefined2 *)((longlong)pauVar35 + uVar20 * -0xc + -3);
      if ((0x1a203U >> (uVar23 & 0x1f) & 1) != 0) {
        uStack_2d0._0_3_ = CONCAT21(local_140,(char)uVar23);
        uStack_2d0 = (undefined1 (*) [16])
                     CONCAT53(CONCAT41(uStack_2d0._4_4_,local_13e),(undefined3)uStack_2d0);
        local_2d8 = *(undefined1 (**) [16])((longlong)pauVar35 + uVar20 * -0xc + -0xc);
        country_core::resources::stairs::structural_graph::StairsStructuralGraph::
        disconnect_node_by_collider();
      }
    }
    pauVar35 = *(undefined1 (**) [16])*pauVar41;
    lVar40 = *(longlong *)(pauVar41[1] + 8);
    auVar46 = *pauVar35;
    uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                             (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
    pauVar21 = pauVar35 + 1;
    while (lVar40 != 0) {
      uVar23 = uVar15;
      if ((short)uVar15 == 0) {
        do {
          auVar46 = *pauVar21;
          pauVar35 = pauVar35 + -0xc;
          pauVar21 = pauVar21 + 1;
          uVar23 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                  (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
        } while (uVar23 == 0xffff);
        uVar23 = ~uVar23;
      }
      uVar15 = uVar23 - 1 & uVar23;
      lVar40 = lVar40 + -1;
      uVar8 = 0;
      for (; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      local_1b8 = *(uint *)((longlong)pauVar35 + (ulonglong)uVar8 * -0xc + -4);
      local_1c0 = *(undefined1 (**) [16])((longlong)pauVar35 + (ulonglong)uVar8 * -0xc + -0xc);
      if ((byte)local_1b8 == 0x10 || (0xa203U >> (local_1b8 & 0x1f) & 1) != 0) {
        local_2d8 = local_1c0;
        if ((1 << ((byte)local_1b8 & 0x1f) & 0xa203U) == 0) {
          uStack_2d0 = (undefined1 (*) [16])CONCAT44(uStack_2d0._4_4_,local_1b8);
          country_core::systems::collision::world::Rapier::get_rapier_collider();
          if (extraout_RAX_05 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          country_core::resources::walls::decorator::DecoratorRank::new();
          country_core::systems::collision::world::RaycastWorld::get_collider_mesh();
          toolbar::mei::widgets::ActionHintView::into_inner();
          if ((char)local_218 != '\x10') {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          local_1c8 = (ulonglong)uStack_210;
          local_2d8 = (undefined1 (*) [16])&local_1c8;
          local_2c8 = pauVar41;
          local_250 = local_1b8;
          local_258 = local_1c0;
          uStack_2d0 = (undefined1 (*) [16])&local_1c0;
          country_core::systems::collision::world::RaycastWorld::
          iter_shape_overlaps<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_2,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_3>
                    ();
        }
        else {
          uStack_2d0 = (undefined1 (*) [16])CONCAT44(uStack_2d0._4_4_,local_1b8);
          country_core::systems::collision::world::RaycastWorld::
          iter_shape_overlaps<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_0,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_1>
                    ();
        }
      }
    }
    if ((char)local_f0 != '\0') {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_06 == 0) {
        std::sys::thread_local::native::lazy::Storage::
        initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                  ();
        plVar43 = extraout_RAX_07;
        if (extraout_RAX_07 == (longlong *)0x0) goto LAB_1407d7dcd;
      }
      else {
        if ((int)*extraout_RAX_06 != 1) {
LAB_1407d7dcd:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        plVar43 = extraout_RAX_06 + 1;
      }
      if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar43 = -1;
      local_f0 = plVar43;
      puffin::ThreadProfiler::end_scope();
      *local_f0 = *local_f0 + 1;
    }
    pauVar21 = local_138;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    uVar20 = *(ulonglong *)*pauVar21;
    *(longlong *)*pauVar21 = *(longlong *)*pauVar21 + -0x10;
    UNLOCK();
    if ((uVar20 & 0xfffffffffffffff2) == 0x12) {
      parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
    }
    puVar14 = local_128;
    local_128 = (undefined1 *)0x0;
    local_2d8 = (undefined1 (*) [16])((longlong)puVar14 + -0x10);
    LOCK();
    plVar43 = (longlong *)((longlong)puVar14 + -0x10);
    *plVar43 = *plVar43 + -1;
    UNLOCK();
    if (*plVar43 == 0) {
      alloc::sync::Arc::
      drop_slow<lock_api::rwlock::RwLock<parking_lot::raw_rwlock::RawRwLock,tabloid::Publication<country_core::systems::collision::world::ColliderDiff>_>,alloc::alloc::Global>
                ();
    }
  }
  local_1b0 = (undefined1 (*) [16])(local_e8 + 0x150);
  lVar40 = *(longlong *)(local_e8 + 0x168);
  if (lVar40 != 0) {
    pauVar35 = *(undefined1 (**) [16])*local_1b0;
    pauVar21 = pauVar35 + 1;
    auVar46 = *pauVar35;
    uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                             (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar15 == 0) {
        do {
          auVar46 = *pauVar21;
          pauVar35 = pauVar35 + -0xc;
          pauVar21 = pauVar21 + 1;
          uVar15 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                  (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
        } while (uVar15 == 0xffff);
        uVar15 = ~uVar15;
      }
      uVar23 = 0;
      for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
        uVar23 = uVar23 + 1;
      }
      uVar20 = (ulonglong)uVar23;
      if (*(char *)((longlong)pauVar35 + uVar20 * -0xc + -4) == '\x10') {
        uStack_2d0 = (undefined1 (*) [16])
                     CONCAT44(uStack_2d0._4_4_,
                              *(undefined4 *)((longlong)pauVar35 + uVar20 * -0xc + -4));
        local_2d8 = *(undefined1 (**) [16])((longlong)pauVar35 + uVar20 * -0xc + -0xc);
        indexmap::map::IndexMap::
        get_index_of<country_core::systems::collision::world::SubworldColliderHandle,alloc::vec::Vec<tuple_<country_core::systems::collision::world::SubworldColliderHandle,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::world::SubworldColliderHandle>
                  ();
        if (extraout_RAX_08 != 1) goto LAB_1407d5300;
      }
      else {
LAB_1407d5300:
        hashbrown::raw::RawTable::
        erase<tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>_>,alloc::alloc::Global>
                  ();
      }
      uVar15 = uVar15 - 1 & uVar15;
      lVar40 = lVar40 + -1;
    } while (lVar40 != 0);
  }
  puffin::are_scopes_on();
  if (extraout_AL_01 == '\0') {
    lVar27 = 0;
    lVar40 = local_e8;
  }
  else {
    ppuVar33 = &PTR_DAT_142a87f40;
    do {
      bVar42 = *(byte *)((longlong)ppuVar33 + -1);
      uVar15 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar45 = *(byte *)((longlong)ppuVar33 + -2);
        if ((char)bVar45 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar33 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar33 + -4);
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
            uVar15 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
            uVar15 = bVar7 & 0xf;
          }
          uVar15 = bVar45 & 0x3f | uVar15 << 6;
        }
        else {
          ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
          uVar15 = bVar45 & 0x1f;
        }
        uVar15 = (uint)(bVar42 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_1407d5434;
LAB_1407d5453:
        if (ppuVar33 == (undefined **)&UNK_142a87eff) break;
        if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
          if (-0x41 < *(char *)((longlong)ppuVar33 + 1)) break;
        }
        else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) == (undefined *)0x40) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
      if (uVar15 == 0x5c) goto LAB_1407d5453;
LAB_1407d5434:
      if (uVar15 == 0x2f) goto LAB_1407d5453;
    } while (ppuVar33 != (undefined **)&DAT_142a87f00);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_09 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar43 = extraout_RAX_10;
      if (extraout_RAX_10 == (longlong *)0x0) goto LAB_1407d7d87;
    }
    else {
      if ((int)*extraout_RAX_09 != 1) {
LAB_1407d7d87:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar43 = extraout_RAX_09 + 1;
    }
    lVar40 = local_e8;
    if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar43 = -1;
    local_f0 = plVar43;
    puffin::ThreadProfiler::begin_scope();
    *local_f0 = *local_f0 + 1;
    lVar27 = 1;
  }
  local_2f0 = (undefined1 (*) [16])lVar27;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_11 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar38 = *extraout_RAX_11;
  lVar37 = extraout_RAX_11[1];
  *extraout_RAX_11 = lVar38 + 1;
  local_218 = (undefined1 (*) [16])
              &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
  uStack_210 = (undefined1 (*) [16])0x0;
  local_208 = (undefined1 (*) [16])0x0;
  plStack_200 = (longlong *)0x0;
  local_1f8 = lVar38;
  lStack_1f0 = lVar37;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  local_228 = lVar27;
  if (extraout_RAX_12 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_2b8 = *extraout_RAX_12;
  lStack_2b0 = extraout_RAX_12[1];
  *extraout_RAX_12 = *extraout_RAX_12 + 1;
  local_2c8 = (undefined1 (*) [16])0x0;
  plStack_2c0 = (longlong *)0x0;
  local_2d8 = (undefined1 (*) [16])
              &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
  uStack_2d0 = (undefined1 (*) [16])0x0;
  local_248 = lVar40 + 0xf0;
  local_308 = lVar40 + 0x120;
  local_110 = (longlong *)(lVar40 + 0x140);
  local_f0 = (longlong *)(lVar40 + 0x170);
  local_f8 = *(float **)(lVar40 + 0x168);
  pauVar35 = (undefined1 (*) [16])0x0;
  pauVar21 = (undefined1 (*) [16])0x0;
  local_108 = (undefined1 (*) [16])
              &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
  pfVar18 = (float *)(longlong *)0x0;
  pauVar41 = *(undefined1 (**) [16])(lVar40 + 0x150);
LAB_1407d5693:
  auVar46 = *pauVar41;
  uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                           (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
  pauVar24 = pauVar41 + 1;
  pfVar28 = local_f8;
  local_300 = (undefined1 (*) [16])lVar37;
  local_2f8 = local_108;
  local_240 = (longlong *)pfVar18;
  local_238 = pauVar35;
  local_198 = lVar38;
  local_178 = pauVar21;
LAB_1407d56d7:
  while (pauVar13 = local_178, lVar37 = lStack_2b0, lVar38 = local_2b8, plVar43 = plStack_2c0,
        pauVar35 = local_2c8, pauVar21 = uStack_2d0, pauVar12 = local_2d8, pfVar28 != (float *)0x0)
  {
    if ((short)uVar15 == 0) {
      do {
        auVar46 = *pauVar24;
        pauVar41 = pauVar41 + -0xc;
        pauVar24 = pauVar24 + 1;
        uVar15 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
      } while (uVar15 == 0xffff);
      uVar15 = ~uVar15;
    }
    uVar23 = 0;
    for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar23 = uVar23 + 1;
    }
    local_1d0 = *(undefined4 *)((longlong)pauVar41 + (ulonglong)uVar23 * -0xc + -4);
    local_1d8 = *(undefined1 (**) [16])((longlong)pauVar41 + (ulonglong)uVar23 * -0xc + -0xc);
    if ((char)local_1d0 == '\x10') {
      uStack_130 = (undefined1 (*) [16])CONCAT44(uStack_130._4_4_,local_1d0);
      local_138 = local_1d8;
      hashbrown::map::HashMap::
      insert<country_core::systems::collision::world::SubworldColliderHandle,f32,std::hash::random::RandomState,alloc::alloc::Global>
                ();
    }
    uVar15 = uVar15 - 1 & uVar15;
    pfVar28 = (float *)((longlong)pfVar28 + -1);
    if (*(longlong *)(lVar40 + 0x138) != 0) {
      local_f8 = pfVar28;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
                ();
      lVar38 = local_e8;
      lVar27 = *(longlong *)(lVar40 + 0x120);
      puVar2 = (ulonglong *)(lVar40 + 0x128);
      bVar42 = (byte)(extraout_RAX_13 >> 0x39);
      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar46 = pshuflw(auVar46,auVar46,0);
      lVar37 = 0;
      uVar20 = extraout_RAX_13;
      while( true ) {
        uVar20 = *puVar2 & uVar20;
        pcVar3 = (char *)(lVar27 + uVar20);
        cVar44 = auVar46[0];
        auVar78[0] = -(*pcVar3 == cVar44);
        cVar47 = auVar46[1];
        auVar78[1] = -(pcVar3[1] == cVar47);
        cVar48 = auVar46[2];
        auVar78[2] = -(pcVar3[2] == cVar48);
        cVar49 = auVar46[3];
        auVar78[3] = -(pcVar3[3] == cVar49);
        auVar78[4] = -(pcVar3[4] == cVar44);
        auVar78[5] = -(pcVar3[5] == cVar47);
        auVar78[6] = -(pcVar3[6] == cVar48);
        auVar78[7] = -(pcVar3[7] == cVar49);
        auVar78[8] = -(pcVar3[8] == cVar44);
        auVar78[9] = -(pcVar3[9] == cVar47);
        auVar78[10] = -(pcVar3[10] == cVar48);
        auVar78[0xb] = -(pcVar3[0xb] == cVar49);
        auVar78[0xc] = -(pcVar3[0xc] == cVar44);
        auVar78[0xd] = -(pcVar3[0xd] == cVar47);
        auVar78[0xe] = -(pcVar3[0xe] == cVar48);
        auVar78[0xf] = -(pcVar3[0xf] == cVar49);
        uVar26 = (ushort)(SUB161(auVar78 >> 7,0) & 1) | (ushort)(SUB161(auVar78 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar78 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar78 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar78 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar78 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar78 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar78 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar78 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar78 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar78 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar78 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar78 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar78 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar78 >> 0x77,0) & 1) << 0xe | (ushort)(auVar78[0xf] >> 7) << 0xf
        ;
        uVar23 = (uint)uVar26;
        while (uVar26 != 0) {
          uVar8 = 0;
          for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar8 = uVar8 + 1;
          }
          uVar30 = uVar8 + uVar20 & *puVar2;
          puVar29 = (undefined8 *)(lVar27 + -0x10 + uVar30 * -0x10);
          if (((char)local_1d0 == *(char *)(puVar29 + 1)) &&
             (auVar79._0_4_ = -(uint)((int)*puVar29 == (int)local_1d8),
             iVar100 = -(uint)((int)((ulonglong)*puVar29 >> 0x20) ==
                              (int)((ulonglong)local_1d8 >> 0x20)), auVar79._4_4_ = auVar79._0_4_,
             auVar79._8_4_ = iVar100, auVar79._12_4_ = iVar100,
             iVar100 = movmskpd((int)puVar29,auVar79), iVar100 == 3)) {
            fVar103 = *(float *)(lVar27 + -4 + uVar30 * -0x10) + 0.046875;
            if (1.0 <= fVar103) {
              fVar103 = 1.0;
            }
            uStack_130 = (undefined1 (*) [16])CONCAT44(uStack_130._4_4_,local_1d0);
            local_138 = local_1d8;
            ppauVar22 = &local_138;
            indexmap::map::IndexMap::
            get_index_of<country_core::systems::collision::world::SubworldColliderHandle,alloc::vec::Vec<tuple_<country_core::systems::collision::world::SubworldColliderHandle,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::world::SubworldColliderHandle>
                      ();
            puVar31 = (undefined8 *)&DAT_00000004;
            puVar29 = (undefined8 *)&DAT_00000004;
            lVar40 = lVar38;
            if (extraout_RAX_14 == 1) {
              if (*(undefined1 (***) [16])(lVar38 + 0x10) <= ppauVar22) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              puVar29 = *(undefined8 **)(*(longlong *)(lVar38 + 8) + 8 + (longlong)ppauVar22 * 0x30)
              ;
              puVar31 = puVar29 + *(longlong *)
                                   (*(longlong *)(lVar38 + 8) + 0x10 + (longlong)ppauVar22 * 0x30) *
                                  2;
            }
            goto LAB_1407d59a3;
          }
          uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
          uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
          lVar40 = local_e8;
        }
        auVar58[0] = -(*pcVar3 == -1);
        auVar58[1] = -(pcVar3[1] == -1);
        auVar58[2] = -(pcVar3[2] == -1);
        auVar58[3] = -(pcVar3[3] == -1);
        auVar58[4] = -(pcVar3[4] == -1);
        auVar58[5] = -(pcVar3[5] == -1);
        auVar58[6] = -(pcVar3[6] == -1);
        auVar58[7] = -(pcVar3[7] == -1);
        auVar58[8] = -(pcVar3[8] == -1);
        auVar58[9] = -(pcVar3[9] == -1);
        auVar58[10] = -(pcVar3[10] == -1);
        auVar58[0xb] = -(pcVar3[0xb] == -1);
        auVar58[0xc] = -(pcVar3[0xc] == -1);
        auVar58[0xd] = -(pcVar3[0xd] == -1);
        auVar58[0xe] = -(pcVar3[0xe] == -1);
        auVar58[0xf] = -(pcVar3[0xf] == -1);
        pfVar28 = local_f8;
        if ((((((((((((((((SUB161(auVar58 >> 7,0) & 1) != 0 || (SUB161(auVar58 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar58 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar58 >> 0x1f,0) & 1) != 0) || (SUB161(auVar58 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar58 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar58 >> 0x37,0) & 1) != 0) || (SUB161(auVar58 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar58 >> 0x47,0) & 1) != 0) || (SUB161(auVar58 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar58 >> 0x57,0) & 1) != 0) || (SUB161(auVar58 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar58 >> 0x67,0) & 1) != 0) || (SUB161(auVar58 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar58 >> 0x77,0) & 1) != 0) || auVar58[0xf] < '\0') break;
        uVar20 = uVar20 + lVar37 + 0x10;
        lVar37 = lVar37 + 0x10;
      }
    }
  }
  if (plStack_2c0 != (longlong *)0x0) {
    local_2d8 = local_2f8;
    local_218 = pauVar12;
    uStack_2d0 = local_178;
    uStack_210 = pauVar21;
    local_2c8 = local_238;
    local_208 = pauVar35;
    plStack_2c0 = local_240;
    plStack_200 = plVar43;
    local_2b8 = local_198;
    local_1f8 = lVar38;
    lStack_2b0 = (longlong)local_300;
    lStack_1f0 = lVar37;
    local_108 = pauVar12;
    auVar46 = *pauVar12;
    local_f8 = (float *)plVar43;
    plVar25 = (longlong *)((longlong)plVar43 + 1U >> 1);
    if (*(longlong *)(lVar40 + 0x168) == 0) {
      plVar25 = plVar43;
    }
    if (*(longlong **)(lVar40 + 0x160) < plVar25) {
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState>_>
                ();
    }
    local_128 = *pauVar21 + (longlong)(*local_108 + 1);
    local_120 = ~((ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1
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
                  (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
    uStack_130 = local_108 + 1;
    local_258 = local_1b0;
    local_158 = &local_258;
    local_138 = local_108;
    hashbrown::raw::RawIterRange::
    fold_impl<tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>_>,hashbrown::map::impl_44::fold::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,tuple_<>,hashbrown::map::impl_58::fold::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,tuple_<>,core::iter::adapters::copied::copy_fold::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,core::iter::adapters::map::map_fold::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>_>,tuple_<>,hashbrown::set::impl_10::extend::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::copied::Copied<std::collections::hash::set::Iter<country_core::systems::collision::world::SubworldColliderHandle>_>_>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>_>,hashbrown::map::impl_82::extend::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::copied::Copied<std::collections::hash::set::Iter<country_core::systems::collision::world::SubworldColliderHandle>_>,hashbrown::set::impl_10::extend::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::copied::Copied<std::collections::hash::set::Iter<country_core::systems::collision::world::SubworldColliderHandle>_>_>_>_>_>_>_>_>_>,tuple_<>_>
              ();
    pauVar24 = uStack_2d0;
    pfVar18 = local_f8;
    pauVar41 = local_108;
    if (plStack_2c0 != (longlong *)0x0) {
      if (uStack_2d0 != (undefined1 (*) [16])0x0) {
        memset(local_2d8,(int)CONCAT71((int7)((ulonglong)pauVar21 >> 8),0xff),
               (size_t)(uStack_2d0[1] + 1));
      }
      plStack_2c0 = (longlong *)0x0;
      pfVar18 = local_f8;
      pauVar41 = local_108;
      local_2c8 = (undefined1 (*) [16])
                  (((ulonglong)(*pauVar24 + 1) & 0xfffffffffffffff8) -
                  ((ulonglong)(*pauVar24 + 1) >> 3));
      if (pauVar24 < (undefined1 (*) [16])&DAT_00000008) {
        local_2c8 = pauVar24;
      }
    }
    goto LAB_1407d5693;
  }
  if ((uStack_2d0 != (undefined1 (*) [16])0x0) &&
     (*uStack_2d0 + ((longlong)(*uStack_2d0 + 1) * 0xc + 0xfU & 0xfffffffffffffff0) !=
      (undefined1 *)0xffffffffffffffef)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  pauVar21 = (undefined1 (*) [16])local_228;
  if ((pauVar13 != (undefined1 (*) [16])0x0) &&
     (*pauVar13 + ((longlong)(*pauVar13 + 1) * 0xc + 0xfU & 0xfffffffffffffff0) !=
      (undefined1 *)0xffffffffffffffef)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
    pauVar21 = local_2f0;
  }
  if (pauVar21 != (undefined1 (*) [16])0x0) {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_18 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar43 = extraout_RAX_19;
      if (extraout_RAX_19 == (longlong *)0x0) goto LAB_1407d7d79;
    }
    else {
      if ((int)*extraout_RAX_18 != 1) {
LAB_1407d7d79:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar43 = extraout_RAX_18 + 1;
    }
    if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar43 = -1;
    local_110 = plVar43;
    puffin::ThreadProfiler::end_scope();
    *local_110 = *local_110 + 1;
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_20 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  *extraout_RAX_20 = *extraout_RAX_20 + 1;
  priority_queue::store::Store::
  with_capacity_and_hasher<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32>,std::hash::random::RandomState>
            ();
  pauVar35 = *(undefined1 (**) [16])(local_e8 + 0x150);
  auVar46 = *pauVar35;
  uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                           (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
  pauVar21 = pauVar35 + 1;
  local_110 = (longlong *)(local_e8 + 0x110);
  pfVar18 = (float *)*(longlong **)(local_e8 + 0x168);
  lVar40 = local_e8;
LAB_1407d5f97:
  if (pfVar18 != (float *)0x0) {
    if ((short)uVar15 == 0) {
      do {
        auVar46 = *pauVar21;
        pauVar35 = pauVar35 + -0xc;
        pauVar21 = pauVar21 + 1;
        uVar15 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
      } while (uVar15 == 0xffff);
      uVar15 = ~uVar15;
    }
    uVar23 = 0;
    for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar23 = uVar23 + 1;
    }
    local_318 = *(undefined4 *)((longlong)pauVar35 + (ulonglong)uVar23 * -0xc + -4);
    local_320 = *(undefined8 *)((longlong)pauVar35 + (ulonglong)uVar23 * -0xc + -0xc);
    puVar29 = &local_320;
    local_f8 = (float *)((longlong)pfVar18 + -1);
    indexmap::map::IndexMap::
    get_index_of<country_core::systems::collision::world::SubworldColliderHandle,alloc::vec::Vec<tuple_<country_core::systems::collision::world::SubworldColliderHandle,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::world::SubworldColliderHandle>
              ();
    puVar16 = (undefined8 *)&DAT_00000004;
    puVar31 = (undefined8 *)&DAT_00000004;
    if (extraout_RAX_21 == 1) {
      if (*(undefined8 **)(lVar40 + 0x10) <= puVar29) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      puVar31 = *(undefined8 **)(*(longlong *)(lVar40 + 8) + 8 + (longlong)puVar29 * 0x30);
      puVar16 = puVar31 + *(longlong *)(*(longlong *)(lVar40 + 8) + 0x10 + (longlong)puVar29 * 0x30)
                          * 2;
    }
    uVar15 = uVar15 - 1 & uVar15;
    for (; pfVar18 = local_f8, puVar31 != puVar16; puVar31 = puVar31 + 2) {
      pauVar41 = (undefined1 (*) [16])*puVar31;
      cVar44 = *(char *)(puVar31 + 1);
      local_138 = (undefined1 (*) [16])
                  CONCAT62(CONCAT51(local_138._3_5_,*(undefined1 *)((longlong)puVar31 + 0xb)),
                           *(undefined2 *)((longlong)puVar31 + 9));
      if (cVar44 == '\x11') break;
      local_188 = CONCAT13(*(undefined1 *)((longlong)puVar31 + 0xb),
                           CONCAT21(*(undefined2 *)((longlong)puVar31 + 9),cVar44));
      local_190 = pauVar41;
      if (cVar44 == '\x10') {
        if (*(longlong *)(lVar40 + 0x168) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
                    ();
          lVar27 = *(longlong *)(lVar40 + 0x150);
          bVar42 = (byte)(extraout_RAX_22 >> 0x39);
          auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
          auVar46 = pshuflw(auVar46,auVar46,0);
          lVar38 = 0;
          uVar20 = extraout_RAX_22;
          while( true ) {
            uVar20 = *(ulonglong *)(lVar40 + 0x158) & uVar20;
            pcVar3 = (char *)(lVar27 + uVar20);
            cVar44 = auVar46[0];
            auVar86[0] = -(*pcVar3 == cVar44);
            cVar47 = auVar46[1];
            auVar86[1] = -(pcVar3[1] == cVar47);
            cVar48 = auVar46[2];
            auVar86[2] = -(pcVar3[2] == cVar48);
            cVar49 = auVar46[3];
            auVar86[3] = -(pcVar3[3] == cVar49);
            auVar86[4] = -(pcVar3[4] == cVar44);
            auVar86[5] = -(pcVar3[5] == cVar47);
            auVar86[6] = -(pcVar3[6] == cVar48);
            auVar86[7] = -(pcVar3[7] == cVar49);
            auVar86[8] = -(pcVar3[8] == cVar44);
            auVar86[9] = -(pcVar3[9] == cVar47);
            auVar86[10] = -(pcVar3[10] == cVar48);
            auVar86[0xb] = -(pcVar3[0xb] == cVar49);
            auVar86[0xc] = -(pcVar3[0xc] == cVar44);
            auVar86[0xd] = -(pcVar3[0xd] == cVar47);
            auVar86[0xe] = -(pcVar3[0xe] == cVar48);
            auVar86[0xf] = -(pcVar3[0xf] == cVar49);
            uVar26 = (ushort)(SUB161(auVar86 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar86 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar86 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar86 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar86 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar86 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar86 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar86 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar86 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar86 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar86 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar86 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar86 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar86 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar86 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar86[0xf] >> 7) << 0xf;
            uVar23 = (uint)uVar26;
            while (uVar26 != 0) {
              uVar8 = 0;
              for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar8 = uVar8 + 1;
              }
              uVar30 = uVar8 + uVar20 & *(ulonglong *)(lVar40 + 0x158);
              if (((char)local_188 == *(char *)(lVar27 + -4 + uVar30 * -0xc)) &&
                 (puVar29 = (undefined8 *)(lVar27 + -0xc + uVar30 * -0xc), uVar5 = *puVar29,
                 auVar87._0_4_ = -(uint)((int)uVar5 == (int)local_190),
                 iVar100 = -(uint)((int)((ulonglong)uVar5 >> 0x20) ==
                                  (int)((ulonglong)local_190 >> 0x20)),
                 auVar87._4_4_ = auVar87._0_4_, auVar87._8_4_ = iVar100, auVar87._12_4_ = iVar100,
                 iVar100 = movmskpd((int)puVar29,auVar87), iVar100 == 3)) goto LAB_1407d607b;
              uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
              uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
            }
            auVar62[0] = -(*pcVar3 == -1);
            auVar62[1] = -(pcVar3[1] == -1);
            auVar62[2] = -(pcVar3[2] == -1);
            auVar62[3] = -(pcVar3[3] == -1);
            auVar62[4] = -(pcVar3[4] == -1);
            auVar62[5] = -(pcVar3[5] == -1);
            auVar62[6] = -(pcVar3[6] == -1);
            auVar62[7] = -(pcVar3[7] == -1);
            auVar62[8] = -(pcVar3[8] == -1);
            auVar62[9] = -(pcVar3[9] == -1);
            auVar62[10] = -(pcVar3[10] == -1);
            auVar62[0xb] = -(pcVar3[0xb] == -1);
            auVar62[0xc] = -(pcVar3[0xc] == -1);
            auVar62[0xd] = -(pcVar3[0xd] == -1);
            auVar62[0xe] = -(pcVar3[0xe] == -1);
            auVar62[0xf] = -(pcVar3[0xf] == -1);
            if ((((((((((((((((SUB161(auVar62 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar62 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar62 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar62 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar62 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar62 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar62 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar62 >> 0x3f,0) & 1) != 0) || (SUB161(auVar62 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar62 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar62 >> 0x57,0) & 1) != 0) || (SUB161(auVar62 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar62 >> 0x67,0) & 1) != 0) || (SUB161(auVar62 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar62 >> 0x77,0) & 1) != 0) || auVar62[0xf] < '\0') break;
            uVar20 = uVar20 + lVar38 + 0x10;
            lVar38 = lVar38 + 0x10;
          }
        }
        if (*(longlong *)(lVar40 + 0x108) != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
                    ();
          plVar43 = (longlong *)(lVar40 + 0xf0);
          puVar2 = (ulonglong *)(lVar40 + 0xf8);
          bVar42 = (byte)(extraout_RAX_23 >> 0x39);
          auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
          auVar46 = pshuflw(auVar46,auVar46,0);
          lVar27 = 0;
          uVar20 = extraout_RAX_23;
          while( true ) {
            uVar20 = *puVar2 & uVar20;
            pcVar3 = (char *)(*plVar43 + uVar20);
            cVar44 = auVar46[0];
            auVar88[0] = -(*pcVar3 == cVar44);
            cVar47 = auVar46[1];
            auVar88[1] = -(pcVar3[1] == cVar47);
            cVar48 = auVar46[2];
            auVar88[2] = -(pcVar3[2] == cVar48);
            cVar49 = auVar46[3];
            auVar88[3] = -(pcVar3[3] == cVar49);
            auVar88[4] = -(pcVar3[4] == cVar44);
            auVar88[5] = -(pcVar3[5] == cVar47);
            auVar88[6] = -(pcVar3[6] == cVar48);
            auVar88[7] = -(pcVar3[7] == cVar49);
            auVar88[8] = -(pcVar3[8] == cVar44);
            auVar88[9] = -(pcVar3[9] == cVar47);
            auVar88[10] = -(pcVar3[10] == cVar48);
            auVar88[0xb] = -(pcVar3[0xb] == cVar49);
            auVar88[0xc] = -(pcVar3[0xc] == cVar44);
            auVar88[0xd] = -(pcVar3[0xd] == cVar47);
            auVar88[0xe] = -(pcVar3[0xe] == cVar48);
            auVar88[0xf] = -(pcVar3[0xf] == cVar49);
            uVar26 = (ushort)(SUB161(auVar88 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar88 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar88 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar88 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar88 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar88 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar88 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar88 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar88 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar88 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar88 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar88 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar88 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar88 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar88 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar88[0xf] >> 7) << 0xf;
            uVar23 = (uint)uVar26;
            while (uVar26 != 0) {
              uVar8 = 0;
              for (uVar9 = uVar23; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar8 = uVar8 + 1;
              }
              puVar29 = (undefined8 *)(*plVar43 + -0x10 + (uVar8 + uVar20 & *puVar2) * -0x10);
              local_170 = lVar27;
              if (((char)local_188 == *(char *)(puVar29 + 1)) &&
                 (auVar89._0_4_ = -(uint)((int)*puVar29 == (int)local_190),
                 iVar100 = -(uint)((int)((ulonglong)*puVar29 >> 0x20) ==
                                  (int)((ulonglong)local_190 >> 0x20)),
                 auVar89._4_4_ = auVar89._0_4_, auVar89._8_4_ = iVar100, auVar89._12_4_ = iVar100,
                 iVar100 = movmskpd((int)puVar29,auVar89), iVar100 == 3)) {
                uStack_210 = (undefined1 (*) [16])CONCAT44(uStack_210._4_4_,local_188);
                local_218 = local_190;
                priority_queue::priority_queue::PriorityQueue::
                push<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32>,std::hash::random::RandomState>
                          ();
                lVar40 = local_e8;
                goto LAB_1407d607b;
              }
              uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
              uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
              lVar40 = local_e8;
            }
            auVar63[0] = -(*pcVar3 == -1);
            auVar63[1] = -(pcVar3[1] == -1);
            auVar63[2] = -(pcVar3[2] == -1);
            auVar63[3] = -(pcVar3[3] == -1);
            auVar63[4] = -(pcVar3[4] == -1);
            auVar63[5] = -(pcVar3[5] == -1);
            auVar63[6] = -(pcVar3[6] == -1);
            auVar63[7] = -(pcVar3[7] == -1);
            auVar63[8] = -(pcVar3[8] == -1);
            auVar63[9] = -(pcVar3[9] == -1);
            auVar63[10] = -(pcVar3[10] == -1);
            auVar63[0xb] = -(pcVar3[0xb] == -1);
            auVar63[0xc] = -(pcVar3[0xc] == -1);
            auVar63[0xd] = -(pcVar3[0xd] == -1);
            auVar63[0xe] = -(pcVar3[0xe] == -1);
            auVar63[0xf] = -(pcVar3[0xf] == -1);
            if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar63 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar63 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar63 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar63 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar63 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar63 >> 0x57,0) & 1) != 0) || (SUB161(auVar63 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar63 >> 0x67,0) & 1) != 0) || (SUB161(auVar63 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar63 >> 0x77,0) & 1) != 0) || auVar63[0xf] < '\0') break;
            uVar20 = uVar20 + lVar27 + 0x10;
            lVar27 = lVar27 + 0x10;
          }
        }
      }
      else {
        uStack_210 = (undefined1 (*) [16])
                     CONCAT53(CONCAT41(uStack_210._4_4_,*(undefined1 *)((longlong)puVar31 + 0xb)),
                              *(undefined3 *)(puVar31 + 1));
        local_218 = pauVar41;
        priority_queue::priority_queue::PriorityQueue::
        push<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32>,std::hash::random::RandomState>
                  ();
      }
LAB_1407d607b:
    }
    goto LAB_1407d5f97;
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_24 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_1f8 = *extraout_RAX_24;
  lStack_1f0 = extraout_RAX_24[1];
  *extraout_RAX_24 = *extraout_RAX_24 + 1;
  local_208 = (undefined1 (*) [16])0x0;
  plStack_200 = (longlong *)0x0;
  local_218 = (undefined1 (*) [16])
              &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
  uStack_210 = (undefined1 (*) [16])0x0;
  pauVar35 = *(undefined1 (**) [16])(lVar40 + 0x150);
  lVar27 = *(longlong *)(lVar40 + 0x168);
  auVar46 = *pauVar35;
  uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                           (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
  pauVar21 = pauVar35 + 1;
  local_f0 = (longlong *)(lVar40 + 0xb0);
LAB_1407d6435:
  if (lVar27 != 0) {
    uVar23 = uVar15;
    if ((short)uVar15 == 0) {
      do {
        auVar46 = *pauVar21;
        pauVar35 = pauVar35 + -0xc;
        pauVar21 = pauVar21 + 1;
        uVar23 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
      } while (uVar23 == 0xffff);
      uVar23 = ~uVar23;
    }
    uVar15 = uVar23 - 1 & uVar23;
    lVar27 = lVar27 + -1;
    if (*(longlong *)(local_e8 + 0xa8) != 0) {
      uVar8 = 0;
      for (; (uVar23 & 1) == 0; uVar23 = uVar23 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
                ();
      lVar40 = *(longlong *)(local_e8 + 0x90);
      bVar42 = (byte)(extraout_RAX_25 >> 0x39);
      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar46 = pshuflw(auVar46,auVar46,0);
      plVar43 = (longlong *)0x0;
      uVar20 = extraout_RAX_25;
      while( true ) {
        uVar20 = *(ulonglong *)(local_e8 + 0x98) & uVar20;
        pcVar3 = (char *)(lVar40 + uVar20);
        cVar44 = auVar46[0];
        auVar64[0] = -(*pcVar3 == cVar44);
        cVar47 = auVar46[1];
        auVar64[1] = -(pcVar3[1] == cVar47);
        cVar48 = auVar46[2];
        auVar64[2] = -(pcVar3[2] == cVar48);
        cVar49 = auVar46[3];
        auVar64[3] = -(pcVar3[3] == cVar49);
        auVar64[4] = -(pcVar3[4] == cVar44);
        auVar64[5] = -(pcVar3[5] == cVar47);
        auVar64[6] = -(pcVar3[6] == cVar48);
        auVar64[7] = -(pcVar3[7] == cVar49);
        auVar64[8] = -(pcVar3[8] == cVar44);
        auVar64[9] = -(pcVar3[9] == cVar47);
        auVar64[10] = -(pcVar3[10] == cVar48);
        auVar64[0xb] = -(pcVar3[0xb] == cVar49);
        auVar64[0xc] = -(pcVar3[0xc] == cVar44);
        auVar64[0xd] = -(pcVar3[0xd] == cVar47);
        auVar64[0xe] = -(pcVar3[0xe] == cVar48);
        auVar64[0xf] = -(pcVar3[0xf] == cVar49);
        uVar26 = (ushort)(SUB161(auVar64 >> 7,0) & 1) | (ushort)(SUB161(auVar64 >> 0xf,0) & 1) << 1
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
        uVar23 = (uint)uVar26;
        if (uVar26 != 0) {
          do {
            uVar9 = 0;
            for (uVar10 = uVar23; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
              uVar9 = uVar9 + 1;
            }
            uVar30 = uVar9 + uVar20 & *(ulonglong *)(local_e8 + 0x98);
            local_f8 = (float *)plVar43;
            if ((*(char *)((longlong)pauVar35 + (ulonglong)uVar8 * -0xc + -4) ==
                 *(char *)(lVar40 + -0x10 + uVar30 * -0x18)) &&
               (puVar29 = (undefined8 *)(lVar40 + -0x18 + uVar30 * -0x18),
               uVar5 = *(undefined8 *)((longlong)pauVar35 + (ulonglong)uVar8 * -0xc + -0xc),
               uVar6 = *puVar29, iVar100 = -(uint)((int)uVar6 == (int)uVar5),
               iVar101 = -(uint)((int)((ulonglong)uVar6 >> 0x20) == (int)((ulonglong)uVar5 >> 0x20))
               , auVar65._4_4_ = iVar100, auVar65._0_4_ = iVar100, auVar65._8_4_ = iVar101,
               auVar65._12_4_ = iVar101, iVar100 = movmskpd((int)puVar29,auVar65), iVar100 == 3)) {
              hashbrown::map::HashMap::
              insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              goto LAB_1407d6435;
            }
            uVar26 = (ushort)(uVar23 - 1) & (ushort)uVar23;
            uVar23 = CONCAT22((short)(uVar23 - 1 >> 0x10),uVar26);
          } while (uVar26 != 0);
        }
        auVar51[0] = -(*pcVar3 == -1);
        auVar51[1] = -(pcVar3[1] == -1);
        auVar51[2] = -(pcVar3[2] == -1);
        auVar51[3] = -(pcVar3[3] == -1);
        auVar51[4] = -(pcVar3[4] == -1);
        auVar51[5] = -(pcVar3[5] == -1);
        auVar51[6] = -(pcVar3[6] == -1);
        auVar51[7] = -(pcVar3[7] == -1);
        auVar51[8] = -(pcVar3[8] == -1);
        auVar51[9] = -(pcVar3[9] == -1);
        auVar51[10] = -(pcVar3[10] == -1);
        auVar51[0xb] = -(pcVar3[0xb] == -1);
        auVar51[0xc] = -(pcVar3[0xc] == -1);
        auVar51[0xd] = -(pcVar3[0xd] == -1);
        auVar51[0xe] = -(pcVar3[0xe] == -1);
        auVar51[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar51 >> 7,0) & 1) != 0 || (SUB161(auVar51 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar51 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar51 >> 0x1f,0) & 1) != 0) || (SUB161(auVar51 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar51 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar51 >> 0x37,0) & 1) != 0) || (SUB161(auVar51 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar51 >> 0x47,0) & 1) != 0) || (SUB161(auVar51 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar51 >> 0x57,0) & 1) != 0) || (SUB161(auVar51 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar51 >> 0x67,0) & 1) != 0) || (SUB161(auVar51 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar51 >> 0x77,0) & 1) != 0) || auVar51[0xf] < '\0') break;
        uVar20 = (longlong)plVar43 + uVar20 + 0x10;
        plVar43 = plVar43 + 2;
      }
    }
    goto LAB_1407d6435;
  }
  puffin::are_scopes_on();
  if (extraout_AL_02 == '\0') {
    local_2f0 = (undefined1 (*) [16])0x0;
  }
  else {
    ppuVar33 = &PTR_DAT_142a87f40;
    do {
      bVar42 = *(byte *)((longlong)ppuVar33 + -1);
      uVar15 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar45 = *(byte *)((longlong)ppuVar33 + -2);
        if ((char)bVar45 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar33 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar33 + -4);
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
            uVar15 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
            uVar15 = bVar7 & 0xf;
          }
          uVar15 = bVar45 & 0x3f | uVar15 << 6;
        }
        else {
          ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
          uVar15 = bVar45 & 0x1f;
        }
        uVar15 = (uint)(bVar42 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_1407d6584;
LAB_1407d659c:
        if (ppuVar33 != (undefined **)&UNK_142a87eff) {
          if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
            if (*(char *)((longlong)ppuVar33 + 1) < -0x40) {
LAB_1407d65b8:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) != (undefined *)0x40)
          goto LAB_1407d65b8;
        }
        break;
      }
      ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
      if (uVar15 == 0x5c) goto LAB_1407d659c;
LAB_1407d6584:
      if (uVar15 == 0x2f) goto LAB_1407d659c;
    } while (ppuVar33 != (undefined **)&DAT_142a87f00);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_26 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar43 = extraout_RAX_27;
      if (extraout_RAX_27 == (longlong *)0x0) goto LAB_1407d7d95;
    }
    else {
      if ((int)*extraout_RAX_26 != 1) {
LAB_1407d7d95:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar43 = extraout_RAX_26 + 1;
    }
    if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar43 = -1;
    local_f8 = (float *)plVar43;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_f8 = *(longlong *)local_f8 + 1;
    local_2f0 = (undefined1 (*) [16])0x1;
  }
  local_f8 = (float *)CONCAT71(local_f8._1_7_,extraout_AL_02);
LAB_1407d66a0:
  priority_queue::priority_queue::PriorityQueue::
  pop<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32>,std::hash::random::RandomState>
            ();
  if ((char)uStack_130 != '\x11') {
    local_328 = (int)uStack_130;
    local_330 = local_138;
    fVar103 = uStack_130._4_4_;
    ppauVar22 = &local_330;
    indexmap::map::IndexMap::
    get_index_of<country_core::systems::collision::world::SubworldColliderHandle,alloc::vec::Vec<tuple_<country_core::systems::collision::world::SubworldColliderHandle,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::systems::collision::world::SubworldColliderHandle>
              ();
    plVar25 = (longlong *)&DAT_00000004;
    plVar43 = (longlong *)&DAT_00000004;
    if (extraout_RAX_28 == 1) {
      if (*(undefined1 (***) [16])(local_e8 + 0x10) <= ppauVar22) {
                    // WARNING: Subroutine does not return
        core::panicking::panic_bounds_check();
      }
      plVar43 = *(longlong **)(*(longlong *)(local_e8 + 8) + 8 + (longlong)ppauVar22 * 0x30);
      plVar25 = plVar43 + *(longlong *)
                           (*(longlong *)(local_e8 + 8) + 0x10 + (longlong)ppauVar22 * 0x30) * 2;
    }
    fVar103 = 0.046875 - fVar103;
    if (1.0 <= fVar103) {
      fVar103 = 1.0;
    }
LAB_1407d67e8:
    if (plVar43 != plVar25) {
      cVar44 = (char)plVar43[1];
      local_142 = *(undefined1 *)((longlong)plVar43 + 0xb);
      local_144 = *(undefined2 *)((longlong)plVar43 + 9);
      if (cVar44 == '\x11') goto LAB_1407d66a0;
      local_1a0 = CONCAT13(*(undefined1 *)((longlong)plVar43 + 0xb),
                           CONCAT21(*(undefined2 *)((longlong)plVar43 + 9),cVar44));
      local_150 = CONCAT53(CONCAT41(local_150._4_4_,local_142),(int3)plVar43[1]);
      local_1a8 = (undefined1 (**) [16])*plVar43;
      local_158 = (undefined1 (**) [16])*plVar43;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
                ();
      plVar43 = plVar43 + 2;
      bVar42 = (byte)(extraout_RAX_29 >> 0x38);
      bVar45 = bVar42 >> 1;
      auVar46 = ZEXT216(CONCAT11(bVar45,bVar45));
      auVar46 = pshuflw(auVar46,auVar46,0);
      lVar40 = 0;
      uVar20 = extraout_RAX_29;
      while( true ) {
        uVar20 = *(ulonglong *)(local_e8 + 0xf8) & uVar20;
        pcVar3 = (char *)(*(longlong *)(local_e8 + 0xf0) + uVar20);
        cVar44 = auVar46[0];
        auVar92[0] = -(*pcVar3 == cVar44);
        cVar47 = auVar46[1];
        auVar92[1] = -(pcVar3[1] == cVar47);
        cVar48 = auVar46[2];
        auVar92[2] = -(pcVar3[2] == cVar48);
        cVar49 = auVar46[3];
        auVar92[3] = -(pcVar3[3] == cVar49);
        auVar92[4] = -(pcVar3[4] == cVar44);
        auVar92[5] = -(pcVar3[5] == cVar47);
        auVar92[6] = -(pcVar3[6] == cVar48);
        auVar92[7] = -(pcVar3[7] == cVar49);
        auVar92[8] = -(pcVar3[8] == cVar44);
        auVar92[9] = -(pcVar3[9] == cVar47);
        auVar92[10] = -(pcVar3[10] == cVar48);
        auVar92[0xb] = -(pcVar3[0xb] == cVar49);
        auVar92[0xc] = -(pcVar3[0xc] == cVar44);
        auVar92[0xd] = -(pcVar3[0xd] == cVar47);
        auVar92[0xe] = -(pcVar3[0xe] == cVar48);
        auVar92[0xf] = -(pcVar3[0xf] == cVar49);
        uVar26 = (ushort)(SUB161(auVar92 >> 7,0) & 1) | (ushort)(SUB161(auVar92 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar92 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar92 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar92 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar92 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar92 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar92 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar92 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar92 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar92 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar92 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar92 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar92 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar92 >> 0x77,0) & 1) << 0xe | (ushort)(auVar92[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar26;
        while (uVar26 != 0) {
          uVar23 = 0;
          for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
            uVar23 = uVar23 + 1;
          }
          uVar30 = uVar23 + uVar20 & *(ulonglong *)(local_e8 + 0xf8);
          lVar27 = *(longlong *)(local_e8 + 0xf0) + uVar30 * -0x10;
          if ((*(char *)(lVar27 + -8) == (char)local_150) &&
             (auVar93._0_4_ = -(uint)((int)*(undefined8 *)(lVar27 + -0x10) == (int)local_158),
             iVar100 = -(uint)((int)((ulonglong)*(undefined8 *)(lVar27 + -0x10) >> 0x20) ==
                              (int)((ulonglong)local_158 >> 0x20)), auVar93._4_4_ = auVar93._0_4_,
             auVar93._8_4_ = iVar100, auVar93._12_4_ = iVar100,
             iVar100 = movmskpd((int)uVar30 * 0x10,auVar93), iVar100 == 3)) goto LAB_1407d69a1;
          uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
          uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
        }
        auVar67[0] = -(*pcVar3 == -1);
        auVar67[1] = -(pcVar3[1] == -1);
        auVar67[2] = -(pcVar3[2] == -1);
        auVar67[3] = -(pcVar3[3] == -1);
        auVar67[4] = -(pcVar3[4] == -1);
        auVar67[5] = -(pcVar3[5] == -1);
        auVar67[6] = -(pcVar3[6] == -1);
        auVar67[7] = -(pcVar3[7] == -1);
        auVar67[8] = -(pcVar3[8] == -1);
        auVar67[9] = -(pcVar3[9] == -1);
        auVar67[10] = -(pcVar3[10] == -1);
        auVar67[0xb] = -(pcVar3[0xb] == -1);
        auVar67[0xc] = -(pcVar3[0xc] == -1);
        auVar67[0xd] = -(pcVar3[0xd] == -1);
        auVar67[0xe] = -(pcVar3[0xe] == -1);
        auVar67[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar67 >> 7,0) & 1) != 0 || (SUB161(auVar67 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar67 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar67 >> 0x1f,0) & 1) != 0) || (SUB161(auVar67 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar67 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar67 >> 0x37,0) & 1) != 0) || (SUB161(auVar67 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar67 >> 0x47,0) & 1) != 0) || (SUB161(auVar67 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar67 >> 0x57,0) & 1) != 0) || (SUB161(auVar67 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar67 >> 0x67,0) & 1) != 0) || (SUB161(auVar67 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar67 >> 0x77,0) & 1) != 0) || auVar67[0xf] < '\0') break;
        uVar20 = uVar20 + lVar40 + 0x10;
        lVar40 = lVar40 + 0x10;
      }
      cVar44 = (char)local_150;
      if (*(longlong *)(local_e8 + 0x100) == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::systems::collision::world::SubworldColliderHandle,f32>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::systems::collision::world::SubworldColliderHandle,f32,std::hash::random::RandomState>_>
                  ();
        cVar44 = (char)local_150;
      }
      ppauVar22 = local_158;
      local_180 = local_150._1_2_;
      uStack_17e = local_150._3_1_;
      lVar27 = local_248;
      if (cVar44 == '\x11') {
LAB_1407d69a1:
        if (fVar103 < *(float *)(lVar27 + -4)) {
          *(float *)(lVar27 + -4) = fVar103;
          local_150 = CONCAT44(local_150._4_4_,local_1a0);
          local_158 = local_1a8;
          priority_queue::priority_queue::PriorityQueue::
          push<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32>,std::hash::random::RandomState>
                    ();
        }
        lVar40 = *(longlong *)(local_e8 + 0xa8);
      }
      else {
        uVar11 = CONCAT14(local_1c8._7_1_,uStack_17d);
        local_1c8 = CONCAT44((int)((uint5)uVar11 >> 8),
                             CONCAT13((undefined1)uStack_17d,(int3)((ulonglong)local_150 >> 8)));
        local_150 = CONCAT44(local_150._4_4_,local_1a0);
        local_158 = local_1a8;
        priority_queue::priority_queue::PriorityQueue::
        push<country_core::systems::collision::world::SubworldColliderHandle,ordered_float::OrderedFloat<f32>,std::hash::random::RandomState>
                  ();
        pauVar21 = *(undefined1 (**) [16])(local_e8 + 0xf0);
        uVar20 = *(ulonglong *)(local_e8 + 0xf8);
        uVar30 = extraout_RAX_29 & uVar20;
        auVar46 = *(undefined1 (*) [16])(*pauVar21 + uVar30);
        uVar26 = (ushort)(SUB161(auVar46 >> 7,0) & 1) | (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1
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
                 (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar46[0xf] >> 7) << 0xf;
        uVar15 = (uint)uVar26;
        if (uVar26 == 0) {
          lVar40 = 0x10;
          do {
            uVar30 = uVar30 + lVar40 & uVar20;
            auVar46 = *(undefined1 (*) [16])(*pauVar21 + uVar30);
            lVar40 = lVar40 + 0x10;
            uVar15 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                    (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                    (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar46[0xf] >> 7) << 0xf);
          } while (uVar15 == 0);
        }
        uVar23 = 0;
        for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
          uVar23 = uVar23 + 1;
        }
        uVar30 = uVar23 + uVar30 & uVar20;
        bVar45 = (*pauVar21)[uVar30];
        if (-1 < (char)bVar45) {
          auVar46 = *pauVar21;
          uVar15 = 0;
          for (uVar23 = (uint)(ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
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
                                       (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar46[0xf] >> 7) << 0xf); (uVar23 & 1) == 0;
              uVar23 = uVar23 >> 1 | 0x80000000) {
            uVar15 = uVar15 + 1;
          }
          uVar30 = (ulonglong)uVar15;
          bVar45 = (*pauVar21)[uVar30];
        }
        bVar42 = bVar42 >> 1;
        (*pauVar21)[uVar30] = bVar42;
        pauVar21[1][uVar30 - 0x10 & uVar20] = bVar42;
        *(longlong *)(local_e8 + 0x100) = *(longlong *)(local_e8 + 0x100) - (ulonglong)(bVar45 & 1);
        *(undefined1 (***) [16])pauVar21[-1 - uVar30] = ppauVar22;
        pauVar21[-1 - uVar30][8] = cVar44;
        *(undefined2 *)(pauVar21[-1 - uVar30] + 9) = (undefined2)local_1c8;
        pauVar21[-1 - uVar30][0xb] = local_1c8._2_1_;
        *(float *)(pauVar21[-1 - uVar30] + 0xc) = fVar103;
        *(longlong *)(local_e8 + 0x108) = *(longlong *)(local_e8 + 0x108) + 1;
        lVar40 = *(longlong *)(local_e8 + 0xa8);
      }
      if (lVar40 != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
                  ();
        lVar40 = *(longlong *)(local_e8 + 0x90);
        bVar42 = (byte)(extraout_RAX_30 >> 0x39);
        auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
        auVar46 = pshuflw(auVar46,auVar46,0);
        lVar27 = 0;
        uVar20 = extraout_RAX_30;
        while( true ) {
          uVar20 = *(ulonglong *)(local_e8 + 0x98) & uVar20;
          pcVar3 = (char *)(lVar40 + uVar20);
          cVar44 = auVar46[0];
          auVar90[0] = -(*pcVar3 == cVar44);
          cVar47 = auVar46[1];
          auVar90[1] = -(pcVar3[1] == cVar47);
          cVar48 = auVar46[2];
          auVar90[2] = -(pcVar3[2] == cVar48);
          cVar49 = auVar46[3];
          auVar90[3] = -(pcVar3[3] == cVar49);
          auVar90[4] = -(pcVar3[4] == cVar44);
          auVar90[5] = -(pcVar3[5] == cVar47);
          auVar90[6] = -(pcVar3[6] == cVar48);
          auVar90[7] = -(pcVar3[7] == cVar49);
          auVar90[8] = -(pcVar3[8] == cVar44);
          auVar90[9] = -(pcVar3[9] == cVar47);
          auVar90[10] = -(pcVar3[10] == cVar48);
          auVar90[0xb] = -(pcVar3[0xb] == cVar49);
          auVar90[0xc] = -(pcVar3[0xc] == cVar44);
          auVar90[0xd] = -(pcVar3[0xd] == cVar47);
          auVar90[0xe] = -(pcVar3[0xe] == cVar48);
          auVar90[0xf] = -(pcVar3[0xf] == cVar49);
          uVar26 = (ushort)(SUB161(auVar90 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar90 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar90 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar90 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar90 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar90 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar90 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar90 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar90 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar90 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar90 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar90 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar90 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar90 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar90 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar90[0xf] >> 7) << 0xf;
          uVar15 = (uint)uVar26;
          while (uVar26 != 0) {
            uVar23 = 0;
            for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar23 = uVar23 + 1;
            }
            uVar30 = uVar23 + uVar20 & *(ulonglong *)(local_e8 + 0x98);
            if (((char)local_1a0 == *(char *)(lVar40 + -0x10 + uVar30 * -0x18)) &&
               (puVar29 = (undefined8 *)(lVar40 + -0x18 + uVar30 * -0x18), uVar5 = *puVar29,
               auVar91._0_4_ = -(uint)((int)uVar5 == (int)local_1a8),
               iVar100 = -(uint)((int)((ulonglong)uVar5 >> 0x20) ==
                                (int)((ulonglong)local_1a8 >> 0x20)), auVar91._4_4_ = auVar91._0_4_,
               auVar91._8_4_ = iVar100, auVar91._12_4_ = iVar100,
               iVar100 = movmskpd((int)puVar29,auVar91), iVar100 == 3)) {
              hashbrown::map::HashMap::
              insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              goto LAB_1407d67e8;
            }
            uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
          }
          auVar66[0] = -(*pcVar3 == -1);
          auVar66[1] = -(pcVar3[1] == -1);
          auVar66[2] = -(pcVar3[2] == -1);
          auVar66[3] = -(pcVar3[3] == -1);
          auVar66[4] = -(pcVar3[4] == -1);
          auVar66[5] = -(pcVar3[5] == -1);
          auVar66[6] = -(pcVar3[6] == -1);
          auVar66[7] = -(pcVar3[7] == -1);
          auVar66[8] = -(pcVar3[8] == -1);
          auVar66[9] = -(pcVar3[9] == -1);
          auVar66[10] = -(pcVar3[10] == -1);
          auVar66[0xb] = -(pcVar3[0xb] == -1);
          auVar66[0xc] = -(pcVar3[0xc] == -1);
          auVar66[0xd] = -(pcVar3[0xd] == -1);
          auVar66[0xe] = -(pcVar3[0xe] == -1);
          auVar66[0xf] = -(pcVar3[0xf] == -1);
          if ((((((((((((((((SUB161(auVar66 >> 7,0) & 1) != 0 || (SUB161(auVar66 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar66 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar66 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar66 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar66 >> 0x2f,0) & 1) != 0) || (SUB161(auVar66 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar66 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar66 >> 0x47,0) & 1) != 0) || (SUB161(auVar66 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar66 >> 0x57,0) & 1) != 0) || (SUB161(auVar66 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar66 >> 0x67,0) & 1) != 0) || (SUB161(auVar66 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar66 >> 0x77,0) & 1) != 0) || auVar66[0xf] < '\0') break;
          uVar20 = uVar20 + lVar27 + 0x10;
          lVar27 = lVar27 + 0x10;
        }
      }
      goto LAB_1407d67e8;
    }
    goto LAB_1407d66a0;
  }
  if ((char)local_f8 != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_31 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar43 = extraout_RAX_32;
      if (extraout_RAX_32 == (longlong *)0x0) goto LAB_1407d7da3;
    }
    else {
      if ((int)*extraout_RAX_31 != 1) {
LAB_1407d7da3:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar43 = extraout_RAX_31 + 1;
    }
    if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar43 = -1;
    local_f0 = plVar43;
    puffin::ThreadProfiler::end_scope();
    *local_f0 = *local_f0 + 1;
  }
  puffin::are_scopes_on();
  lVar40 = local_e8;
  if ((char)extraout_EAX != '\0') {
    ppuVar33 = &PTR_DAT_142a87f40;
    do {
      bVar42 = *(byte *)((longlong)ppuVar33 + -1);
      uVar15 = (uint)(char)bVar42;
      if ((char)bVar42 < '\0') {
        bVar45 = *(byte *)((longlong)ppuVar33 + -2);
        if ((char)bVar45 < -0x40) {
          bVar7 = *(byte *)((longlong)ppuVar33 + -3);
          if ((char)bVar7 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar33 + -4);
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -4);
            uVar15 = bVar7 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar33 = (undefined **)((longlong)ppuVar33 + -3);
            uVar15 = bVar7 & 0xf;
          }
          uVar15 = bVar45 & 0x3f | uVar15 << 6;
        }
        else {
          ppuVar33 = (undefined **)((longlong)ppuVar33 + -2);
          uVar15 = bVar45 & 0x1f;
        }
        uVar15 = (uint)(bVar42 & 0x3f) | uVar15 << 6;
        if (uVar15 != 0x5c) goto LAB_1407d6cc4;
LAB_1407d6cd5:
        if (ppuVar33 != (undefined **)&UNK_142a87eff) {
          if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) < (undefined *)0x40) {
            if (*(char *)((longlong)ppuVar33 + 1) < -0x40) {
LAB_1407d6cf1:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if ((undefined *)((longlong)ppuVar33 + -0x142a87eff) != (undefined *)0x40)
          goto LAB_1407d6cf1;
        }
        break;
      }
      ppuVar33 = (undefined **)((longlong)ppuVar33 + -1);
      if (uVar15 == 0x5c) goto LAB_1407d6cd5;
LAB_1407d6cc4:
      if (uVar15 == 0x2f) goto LAB_1407d6cd5;
    } while (ppuVar33 != (undefined **)&DAT_142a87f00);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_33 == 0) {
      std::sys::thread_local::native::lazy::Storage::
      initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                ();
      plVar43 = extraout_RAX_34;
      lVar40 = local_e8;
      if (extraout_RAX_34 == (longlong *)0x0) goto LAB_1407d7db1;
    }
    else {
      if ((int)*extraout_RAX_33 != 1) {
LAB_1407d7db1:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      plVar43 = extraout_RAX_33 + 1;
    }
    if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar43 = -1;
    local_f0 = plVar43;
    puffin::ThreadProfiler::begin_scope();
    *local_f0 = *local_f0 + 1;
    local_228 = extraout_RAX_35;
  }
  local_300 = *(undefined1 (**) [16])(local_230 + 8) + *(longlong *)(local_230 + 0x10) * 0x15;
  local_240 = (longlong *)*local_310;
  local_2f8 = (undefined1 (*) [16])(local_240 + 4);
  local_170 = lVar40 + 0xe0;
  local_238 = (undefined1 (*) [16])(local_230 + 0x158);
  local_198 = CONCAT71(local_198._1_7_,(char)extraout_EAX);
  pauVar21 = *(undefined1 (**) [16])(local_230 + 8);
  uVar15 = extraout_EAX;
LAB_1407d6e54:
  do {
    if (pauVar21 == local_300) {
      if ((char)uVar15 != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_46 == 0) {
          std::sys::thread_local::native::lazy::Storage::
          initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                    ();
          plVar43 = extraout_RAX_47;
          if (extraout_RAX_47 == (longlong *)0x0) goto LAB_1407d7ddb;
        }
        else {
          if ((int)*extraout_RAX_46 != 1) {
LAB_1407d7ddb:
                    // WARNING: Subroutine does not return
            std::thread::local::panic_access_error();
          }
          plVar43 = extraout_RAX_46 + 1;
        }
        if (*plVar43 != 0) {
                    // WARNING: Subroutine does not return
          core::cell::panic_already_borrowed();
        }
        *plVar43 = -1;
        local_f0 = plVar43;
        puffin::ThreadProfiler::end_scope();
        *local_f0 = *local_f0 + 1;
      }
      lVar27 = local_248;
      hashbrown::raw::impl$13::
      clone_from<tuple_<country_core::systems::collision::world::SubworldColliderHandle,f32>,alloc::alloc::Global>
                ();
      lVar40 = local_e8;
      *(undefined8 *)(local_e8 + 0x140) = *(undefined8 *)(local_e8 + 0x110);
      *(undefined8 *)(local_e8 + 0x148) = *(undefined8 *)(local_e8 + 0x118);
      if (*(longlong *)(local_e8 + 0x168) != 0) {
        uVar20 = *(ulonglong *)(local_e8 + 0x158);
        if (uVar20 != 0) {
          memset(*(void **)*local_1b0,(int)CONCAT71((int7)((ulonglong)lVar27 >> 8),0xff),
                 uVar20 + 0x11);
        }
        *(undefined8 *)(lVar40 + 0x168) = 0;
        uVar30 = (uVar20 + 1 & 0xfffffffffffffff8) - (uVar20 + 1 >> 3);
        if (uVar20 < 8) {
          uVar30 = uVar20;
        }
        *(ulonglong *)(lVar40 + 0x160) = uVar30;
      }
      if ((uStack_210 != (undefined1 (*) [16])0x0) &&
         ((longlong)uStack_210 + ((longlong)uStack_210 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11))
      {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((local_2b8 != 0) && (local_2b8 + (local_2b8 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_2d8 != (undefined1 (*) [16])0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_290 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_278 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_f9 == '\0') {
        return;
      }
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_48 == 1) {
        plVar43 = extraout_RAX_48 + 1;
      }
      else if ((*extraout_RAX_48 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar43 = extraout_RAX_50, extraout_RAX_50 == (longlong *)0x0))
      goto LAB_1407d7c1b;
      if (*plVar43 == 0) {
        *plVar43 = -1;
        local_f0 = plVar43;
        puffin::ThreadProfiler::end_scope();
        *local_f0 = *local_f0 + 1;
        return;
      }
      goto LAB_1407d7c62;
    }
    local_178 = pauVar21 + 0x15;
    local_f0 = (longlong *)(pauVar21[0x12] + 8);
    local_108 = pauVar21;
    if (plStack_200 != (longlong *)0x0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                ();
      bVar42 = (byte)(extraout_RAX_36 >> 0x39);
      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar46 = pshuflw(auVar46,auVar46,0);
      lVar40 = 0;
      uVar20 = extraout_RAX_36;
      while( true ) {
        uVar20 = (ulonglong)uStack_210 & uVar20;
        pcVar3 = (char *)((longlong)local_218 + uVar20);
        cVar44 = auVar46[0];
        auVar68[0] = -(*pcVar3 == cVar44);
        cVar47 = auVar46[1];
        auVar68[1] = -(pcVar3[1] == cVar47);
        cVar48 = auVar46[2];
        auVar68[2] = -(pcVar3[2] == cVar48);
        cVar49 = auVar46[3];
        auVar68[3] = -(pcVar3[3] == cVar49);
        auVar68[4] = -(pcVar3[4] == cVar44);
        auVar68[5] = -(pcVar3[5] == cVar47);
        auVar68[6] = -(pcVar3[6] == cVar48);
        auVar68[7] = -(pcVar3[7] == cVar49);
        auVar68[8] = -(pcVar3[8] == cVar44);
        auVar68[9] = -(pcVar3[9] == cVar47);
        auVar68[10] = -(pcVar3[10] == cVar48);
        auVar68[0xb] = -(pcVar3[0xb] == cVar49);
        auVar68[0xc] = -(pcVar3[0xc] == cVar44);
        auVar68[0xd] = -(pcVar3[0xd] == cVar47);
        auVar68[0xe] = -(pcVar3[0xe] == cVar48);
        auVar68[0xf] = -(pcVar3[0xf] == cVar49);
        uVar26 = (ushort)(SUB161(auVar68 >> 7,0) & 1) | (ushort)(SUB161(auVar68 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar68 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar68 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar68 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar68 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar68 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar68 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar68 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar68 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar68 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar68 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar68 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar68 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar68 >> 0x77,0) & 1) << 0xe | (ushort)(auVar68[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar26;
        if (uVar26 != 0) {
          do {
            uVar23 = 0;
            for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar23 = uVar23 + 1;
            }
            uVar30 = uVar23 + uVar20 & (ulonglong)uStack_210;
            auVar94._0_4_ =
                 -(uint)((int)*(undefined8 *)((longlong)local_218 + uVar30 * -8 + -8) ==
                        (int)*local_f0);
            iVar100 = -(uint)((int)((ulonglong)
                                    *(undefined8 *)((longlong)local_218 + uVar30 * -8 + -8) >> 0x20)
                             == (int)((ulonglong)*local_f0 >> 0x20));
            auVar94._4_4_ = auVar94._0_4_;
            auVar94._8_4_ = iVar100;
            auVar94._12_4_ = iVar100;
            iVar100 = movmskpd((int)uVar30 * 8,auVar94);
            if (iVar100 == 3) goto LAB_1407d7050;
            uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
          } while (uVar26 != 0);
        }
        auVar52[0] = -(*pcVar3 == -1);
        auVar52[1] = -(pcVar3[1] == -1);
        auVar52[2] = -(pcVar3[2] == -1);
        auVar52[3] = -(pcVar3[3] == -1);
        auVar52[4] = -(pcVar3[4] == -1);
        auVar52[5] = -(pcVar3[5] == -1);
        auVar52[6] = -(pcVar3[6] == -1);
        auVar52[7] = -(pcVar3[7] == -1);
        auVar52[8] = -(pcVar3[8] == -1);
        auVar52[9] = -(pcVar3[9] == -1);
        auVar52[10] = -(pcVar3[10] == -1);
        auVar52[0xb] = -(pcVar3[0xb] == -1);
        auVar52[0xc] = -(pcVar3[0xc] == -1);
        auVar52[0xd] = -(pcVar3[0xd] == -1);
        auVar52[0xe] = -(pcVar3[0xe] == -1);
        auVar52[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar52 >> 7,0) & 1) != 0 || (SUB161(auVar52 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar52 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar52 >> 0x1f,0) & 1) != 0) || (SUB161(auVar52 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar52 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar52 >> 0x37,0) & 1) != 0) || (SUB161(auVar52 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar52 >> 0x47,0) & 1) != 0) || (SUB161(auVar52 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar52 >> 0x57,0) & 1) != 0) || (SUB161(auVar52 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar52 >> 0x67,0) & 1) != 0) || (SUB161(auVar52 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar52 >> 0x77,0) & 1) != 0) || auVar52[0xf] < '\0') break;
        uVar20 = uVar20 + lVar40 + 0x10;
        lVar40 = lVar40 + 0x10;
      }
    }
    if (local_240[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                ();
      bVar42 = (byte)(extraout_RAX_37 >> 0x39);
      auVar46 = ZEXT216(CONCAT11(bVar42,bVar42));
      auVar46 = pshuflw(auVar46,auVar46,0);
      lVar40 = 0;
      uVar20 = extraout_RAX_37;
      while( true ) {
        uVar20 = local_240[1] & uVar20;
        pcVar3 = (char *)(*local_240 + uVar20);
        cVar44 = auVar46[0];
        auVar69[0] = -(*pcVar3 == cVar44);
        cVar47 = auVar46[1];
        auVar69[1] = -(pcVar3[1] == cVar47);
        cVar48 = auVar46[2];
        auVar69[2] = -(pcVar3[2] == cVar48);
        cVar49 = auVar46[3];
        auVar69[3] = -(pcVar3[3] == cVar49);
        auVar69[4] = -(pcVar3[4] == cVar44);
        auVar69[5] = -(pcVar3[5] == cVar47);
        auVar69[6] = -(pcVar3[6] == cVar48);
        auVar69[7] = -(pcVar3[7] == cVar49);
        auVar69[8] = -(pcVar3[8] == cVar44);
        auVar69[9] = -(pcVar3[9] == cVar47);
        auVar69[10] = -(pcVar3[10] == cVar48);
        auVar69[0xb] = -(pcVar3[0xb] == cVar49);
        auVar69[0xc] = -(pcVar3[0xc] == cVar44);
        auVar69[0xd] = -(pcVar3[0xd] == cVar47);
        auVar69[0xe] = -(pcVar3[0xe] == cVar48);
        auVar69[0xf] = -(pcVar3[0xf] == cVar49);
        uVar26 = (ushort)(SUB161(auVar69 >> 7,0) & 1) | (ushort)(SUB161(auVar69 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar69 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar69 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar69 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar69 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar69 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar69 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar69 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar69 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar69 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar69 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar69 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar69 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar69 >> 0x77,0) & 1) << 0xe | (ushort)(auVar69[0xf] >> 7) << 0xf
        ;
        uVar15 = (uint)uVar26;
        if (uVar26 != 0) {
          do {
            uVar23 = 0;
            for (uVar8 = uVar15; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
              uVar23 = uVar23 + 1;
            }
            uVar30 = uVar23 + uVar20 & local_240[1];
            uVar5 = *(undefined8 *)(*local_240 + uVar30 * -8 + -8);
            auVar95._0_4_ = -(uint)((int)uVar5 == (int)*local_f0);
            iVar100 = -(uint)((int)((ulonglong)uVar5 >> 0x20) == (int)((ulonglong)*local_f0 >> 0x20)
                             );
            auVar95._4_4_ = auVar95._0_4_;
            auVar95._8_4_ = iVar100;
            auVar95._12_4_ = iVar100;
            iVar100 = movmskpd((int)uVar30 * 8,auVar95);
            if (iVar100 == 3) goto LAB_1407d7050;
            uVar26 = (ushort)(uVar15 - 1) & (ushort)uVar15;
            uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar26);
          } while (uVar26 != 0);
        }
        auVar53[0] = -(*pcVar3 == -1);
        auVar53[1] = -(pcVar3[1] == -1);
        auVar53[2] = -(pcVar3[2] == -1);
        auVar53[3] = -(pcVar3[3] == -1);
        auVar53[4] = -(pcVar3[4] == -1);
        auVar53[5] = -(pcVar3[5] == -1);
        auVar53[6] = -(pcVar3[6] == -1);
        auVar53[7] = -(pcVar3[7] == -1);
        auVar53[8] = -(pcVar3[8] == -1);
        auVar53[9] = -(pcVar3[9] == -1);
        auVar53[10] = -(pcVar3[10] == -1);
        auVar53[0xb] = -(pcVar3[0xb] == -1);
        auVar53[0xc] = -(pcVar3[0xc] == -1);
        auVar53[0xd] = -(pcVar3[0xd] == -1);
        auVar53[0xe] = -(pcVar3[0xe] == -1);
        auVar53[0xf] = -(pcVar3[0xf] == -1);
        if ((((((((((((((((SUB161(auVar53 >> 7,0) & 1) != 0 || (SUB161(auVar53 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar53 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar53 >> 0x1f,0) & 1) != 0) || (SUB161(auVar53 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar53 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar53 >> 0x37,0) & 1) != 0) || (SUB161(auVar53 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar53 >> 0x47,0) & 1) != 0) || (SUB161(auVar53 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar53 >> 0x57,0) & 1) != 0) || (SUB161(auVar53 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar53 >> 0x67,0) & 1) != 0) || (SUB161(auVar53 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar53 >> 0x77,0) & 1) != 0) || auVar53[0xf] < '\0') break;
        uVar20 = uVar20 + lVar40 + 0x10;
        lVar40 = lVar40 + 0x10;
      }
    }
    country_core::resources::stairs::stairs_assembly_params::impl$2::from();
    uVar15 = (uint)(byte)local_198;
    pauVar21 = local_178;
    local_138 = extraout_RAX_44;
    if (*(longlong *)(local_230 + 0x150) != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar40 = *(longlong *)(local_230 + 0x138);
      bVar42 = (byte