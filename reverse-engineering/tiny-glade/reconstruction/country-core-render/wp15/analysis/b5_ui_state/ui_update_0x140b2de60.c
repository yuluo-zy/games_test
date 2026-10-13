// addr=0x140b2de60 name=ui_update
// signature=void __cdecl ui_update(void)


// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000142aeba71) overlaps instruction at (ram,0x000142aeba70)
// 
// WARNING: Type propagation algorithm not settling

void __cdecl country_core::systems::ui::ui_update::ui_update(void)

{
  byte *pbVar1;
  float fVar2;
  byte bVar3;
  undefined1 (*pauVar4) [16];
  undefined1 (*pauVar5) [16];
  longlong *plVar6;
  code *pcVar7;
  uint uVar8;
  uint uVar9;
  bool bVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  bool bVar13;
  bool bVar14;
  char extraout_AL;
  byte bVar15;
  char cVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  undefined *puVar17;
  longlong *extraout_RAX_01;
  ulonglong *extraout_RAX_02;
  ulonglong *extraout_RAX_03;
  ulonglong *extraout_RAX_04;
  undefined1 *extraout_RAX_05;
  undefined1 (*pauVar18) [16];
  longlong *extraout_RAX_06;
  ulonglong *extraout_RAX_07;
  char *pcVar19;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  ulonglong extraout_RAX_10;
  ulonglong extraout_RAX_11;
  ulonglong *extraout_RAX_12;
  undefined *puVar20;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  longlong *extraout_RAX_15;
  longlong *extraout_RAX_16;
  char *pcVar21;
  uint uVar22;
  undefined1 (*in_RCX) [16];
  undefined1 *puVar23;
  longlong lVar24;
  ulonglong *puVar25;
  ulonglong *in_RDX;
  ulonglong uVar26;
  undefined1 (*pauVar27) [16];
  longlong *plVar28;
  undefined8 *in_R8;
  undefined1 (*pauVar29) [16];
  longlong lVar30;
  ushort uVar31;
  undefined8 *in_R9;
  undefined1 (*pauVar32) [16];
  undefined1 (*pauVar33) [16];
  ulonglong uVar34;
  undefined1 *puVar35;
  longlong lVar36;
  uint *puVar37;
  byte bVar38;
  undefined4 uVar39;
  int iVar40;
  longlong *plVar41;
  undefined8 uVar42;
  longlong lVar43;
  undefined8 uVar44;
  bool bVar45;
  bool bVar46;
  bool bVar47;
  bool bVar48;
  bool bVar49;
  bool bVar50;
  bool bVar51;
  bool bVar52;
  bool bVar53;
  char cVar54;
  char cVar56;
  char cVar57;
  char cVar58;
  char cVar59;
  char cVar60;
  char cVar61;
  undefined1 auVar55 [16];
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  undefined1 auVar65 [16];
  undefined1 auVar66 [16];
  undefined1 auVar67 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  char unaff_XMM6_Bh;
  float fVar68;
  float fVar69;
  float unaff_XMM9_Da;
  undefined8 *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  ulonglong *in_stack_00000040;
  ulonglong local_178;
  int *piStack_170;
  ulonglong *local_168;
  undefined1 (*pauStack_160) [16];
  longlong local_158;
  longlong local_150;
  undefined1 (*local_148) [16];
  uint local_140;
  undefined4 uStack_13c;
  ulonglong *local_128;
  undefined1 (*pauStack_120) [16];
  ulonglong *local_118;
  undefined1 (*pauStack_110) [16];
  undefined8 local_f8;
  int *piStack_f0;
  ulonglong *local_e8;
  undefined1 (*pauStack_e0) [16];
  ulonglong *local_d8;
  undefined1 (*pauStack_d0) [16];
  undefined8 *local_c8;
  undefined7 uStack_c0;
  undefined1 uStack_b9;
  undefined4 uStack_b8;
  undefined4 uStack_b4;
  ulonglong *local_a8;
  undefined1 (*local_a0) [16];
  ulonglong *local_98;
  int local_8c;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined1 (*local_78) [16];
  undefined4 local_68;
  
  local_68 = CONCAT13(unaff_XMM6_Bd,CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba)));
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    bVar49 = false;
  }
  else {
    uVar26 = 0x2f;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar26 != 0) {
        if (uVar26 < 0x2f) {
          if ((char)(&DAT_142aeb334)[uVar26] < -0x40) {
LAB_140b2def4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar26 != 0x2f) goto LAB_140b2def4;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar26 != 0xfffffffffffffffe)) {
        if (uVar26 + 2 < 0x2f) {
          if ((char)(&UNK_142aeb336)[uVar26] < -0x40) {
LAB_140b2df52:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar26 + 2 != 0x2f) goto LAB_140b2df52;
      }
    }
    puVar17 = &DAT_142aeb8bf;
    do {
      bVar38 = puVar17[-1];
      uVar22 = (uint)(char)bVar38;
      if ((char)bVar38 < '\0') {
        bVar15 = puVar17[-2];
        if ((char)bVar15 < -0x40) {
          bVar3 = puVar17[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = puVar17 + -4;
            puVar17 = puVar17 + -4;
            uVar22 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar17 = puVar17 + -3;
            uVar22 = bVar3 & 0xf;
          }
          uVar22 = bVar15 & 0x3f | uVar22 << 6;
        }
        else {
          puVar17 = puVar17 + -2;
          uVar22 = bVar15 & 0x1f;
        }
        uVar22 = (uint)(bVar38 & 0x3f) | uVar22 << 6;
        if (uVar22 != 0x5c) goto LAB_140b2e019;
LAB_140b2e02e:
        if (puVar17 == &UNK_142aeb88f) break;
        if (puVar17 + -0x142aeb88f < (undefined *)0x2f) {
          if (-0x41 < (char)puVar17[1]) break;
        }
        else if (puVar17 + -0x142aeb88f == (undefined *)0x2f) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar17 = puVar17 + -1;
      if (uVar22 == 0x5c) goto LAB_140b2e02e;
LAB_140b2e019:
      if (uVar22 == 0x2f) goto LAB_140b2e02e;
    } while (puVar17 != &DAT_142aeb890);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar41 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar41 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0))
    goto LAB_140b2f28d;
    if (*plVar41 != 0) goto LAB_140b2f299;
    *plVar41 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar41 = *plVar41 + 1;
    bVar49 = true;
  }
  lVar43 = in_stack_00000028[1];
  uVar26 = *(ulonglong *)*in_stack_00000028;
  pauVar18 = (undefined1 (*) [16])0x0;
  pauVar33 = (undefined1 (*) [16])(uVar26 - *(ulonglong *)(lVar43 + 0x18));
  if (uVar26 < *(ulonglong *)(lVar43 + 0x18)) {
    pauVar33 = pauVar18;
  }
  pauVar32 = (undefined1 (*) [16])(uVar26 - *(ulonglong *)(lVar43 + 0x38));
  if (uVar26 < *(ulonglong *)(lVar43 + 0x38)) {
    pauVar32 = pauVar18;
  }
  bVar45 = pauVar32 <= *(undefined1 (**) [16])(lVar43 + 0x30);
  pauVar32 = (undefined1 (*) [16])
             ((longlong)*(undefined1 (**) [16])(lVar43 + 0x30) - (longlong)pauVar32);
  bVar50 = pauVar32 != (undefined1 (*) [16])0x0;
  if (!bVar45) {
    pauVar32 = pauVar18;
  }
  bVar46 = pauVar33 <= *(undefined1 (**) [16])(lVar43 + 0x10);
  pauVar33 = (undefined1 (*) [16])
             ((longlong)*(undefined1 (**) [16])(lVar43 + 0x10) - (longlong)pauVar33);
  bVar51 = pauVar33 != (undefined1 (*) [16])0x0;
  if (!bVar46) {
    pauVar33 = pauVar18;
  }
  pauVar33 = (undefined1 (*) [16])(*pauVar32 + (longlong)*pauVar33);
  if (bVar46 && bVar51 || bVar45 && bVar50) {
    pauVar33 = pauVar18;
  }
  *(ulonglong *)*in_stack_00000028 = *(longlong *)(lVar43 + 0x40) - (longlong)pauVar33;
  lVar43 = in_R9[1];
  uVar26 = *(ulonglong *)*in_R9;
  pauVar33 = (undefined1 (*) [16])(uVar26 - *(ulonglong *)(lVar43 + 0x18));
  if (uVar26 < *(ulonglong *)(lVar43 + 0x18)) {
    pauVar33 = pauVar18;
  }
  pauVar32 = (undefined1 (*) [16])(uVar26 - *(ulonglong *)(lVar43 + 0x38));
  if (uVar26 < *(ulonglong *)(lVar43 + 0x38)) {
    pauVar32 = pauVar18;
  }
  bVar47 = pauVar32 <= *(undefined1 (**) [16])(lVar43 + 0x30);
  pauVar32 = (undefined1 (*) [16])
             ((longlong)*(undefined1 (**) [16])(lVar43 + 0x30) - (longlong)pauVar32);
  bVar52 = pauVar32 != (undefined1 (*) [16])0x0;
  if (!bVar47) {
    pauVar32 = pauVar18;
  }
  bVar48 = pauVar33 <= *(undefined1 (**) [16])(lVar43 + 0x10);
  pauVar33 = (undefined1 (*) [16])
             ((longlong)*(undefined1 (**) [16])(lVar43 + 0x10) - (longlong)pauVar33);
  bVar53 = pauVar33 != (undefined1 (*) [16])0x0;
  if (!bVar48) {
    pauVar33 = pauVar18;
  }
  bVar10 = bVar48 && bVar53 || bVar47 && bVar52;
  local_150 = CONCAT71(local_150._1_7_,bVar10);
  pauVar33 = (undefined1 (*) [16])(*pauVar32 + (longlong)*pauVar33);
  if (bVar48 && bVar53 || bVar47 && bVar52) {
    pauVar33 = pauVar18;
  }
  *(ulonglong *)*in_R9 = *(longlong *)(lVar43 + 0x40) - (longlong)pauVar33;
  puVar25 = (ulonglong *)*in_R8;
  lVar43 = in_R8[1];
  uVar26 = *puVar25;
  pauVar33 = (undefined1 (*) [16])(uVar26 - *(ulonglong *)(lVar43 + 0x18));
  if (uVar26 < *(ulonglong *)(lVar43 + 0x18)) {
    pauVar33 = pauVar18;
  }
  pauVar32 = (undefined1 (*) [16])(uVar26 - *(ulonglong *)(lVar43 + 0x38));
  if (uVar26 < *(ulonglong *)(lVar43 + 0x38)) {
    pauVar32 = pauVar18;
  }
  pauVar4 = *(undefined1 (**) [16])(lVar43 + 0x10);
  pauVar5 = *(undefined1 (**) [16])(lVar43 + 0x30);
  pauVar29 = (undefined1 (*) [16])((longlong)pauVar5 - (longlong)pauVar32);
  if (pauVar5 < pauVar32) {
    pauVar29 = pauVar18;
  }
  pauVar27 = (undefined1 (*) [16])((longlong)pauVar4 - (longlong)pauVar33);
  if (pauVar4 < pauVar33) {
    pauVar27 = pauVar18;
  }
  uVar26 = *(ulonglong *)(lVar43 + 0x40);
  lVar36 = *(longlong *)(lVar43 + 8);
  lVar43 = *(longlong *)(lVar43 + 0x28);
  *puVar25 = (uVar26 - (longlong)pauVar29) - (longlong)pauVar27;
  if (pauVar4 >= pauVar33 &&
      (undefined1 (*) [16])((longlong)pauVar4 - (longlong)pauVar33) != (undefined1 (*) [16])0x0 ||
      pauVar5 >= pauVar32 &&
      (undefined1 (*) [16])((longlong)pauVar5 - (longlong)pauVar32) != (undefined1 (*) [16])0x0) {
    lVar43 = lVar43 + (longlong)pauVar32 * 0x10;
    lVar36 = lVar36 + (longlong)pauVar33 * 0x10;
    pauVar27 = (undefined1 (*) [16])((longlong)pauVar27 * 0x10);
    if (pauVar4 < pauVar33) {
      lVar36 = 8;
    }
    puVar35 = (undefined1 *)0x0;
    if (pauVar33 < pauVar4) {
      puVar35 = pauVar27[-1] + lVar36;
    }
    if (pauVar5 < pauVar32) {
      lVar43 = 8;
    }
    puVar23 = (undefined1 *)(lVar43 + -0x10 + (longlong)pauVar29 * 0x10);
    if (pauVar5 <= pauVar32) {
      puVar23 = puVar35;
    }
    *puVar25 = uVar26;
    pauVar18 = (undefined1 (*) [16])(ulonglong)(puVar23[8] ^ 1);
  }
  local_78 = *(undefined1 (**) [16])*in_RCX;
  if (*(longlong *)local_78[1] == 0) {
    plVar41 = (longlong *)0x2;
    cVar16 = local_78[1][8];
    if (cVar16 == '\x02') goto LAB_140b2e362;
LAB_140b2e29d:
    iVar40 = (int)plVar41;
    if (cVar16 == '\x01') {
      if (local_78[1][9] == '\0') {
LAB_140b2e64c:
        bVar13 = true;
        bVar14 = false;
        goto LAB_140b2e652;
      }
      if (iVar40 != 2) {
        if (*(undefined1 (**) [16])(local_78[3] + 8) == pauVar27) {
          get_ui_volume();
          **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
          local_a8 = extraout_RAX_03;
          if (local_78[1][8] != '\0') {
            plVar28 = *(longlong **)local_78[2];
            LOCK();
            *plVar28 = *plVar28 + -1;
            UNLOCK();
            if (*plVar28 == 0) {
              alloc::sync::Arc::
              drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                        ();
            }
          }
        }
        else {
          get_ui_volume();
          uVar26 = extraout_RAX_04[7];
          uVar11 = extraout_RAX_04[5];
          uVar12 = extraout_RAX_04[6];
          *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
          uVar34 = *in_stack_00000040;
          lVar43 = *(longlong *)(uVar34 + 0x30);
          uVar42 = *(undefined8 *)(uVar34 + 0x40);
          if (lVar43 == *(longlong *)(uVar34 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
          lVar36 = *(longlong *)(uVar34 + 0x28);
          lVar24 = lVar43 * 0x20;
          *(ulonglong *)(lVar36 + 0x10 + lVar24) = uVar26;
          *(ulonglong *)(lVar36 + lVar24) = uVar11;
          ((ulonglong *)(lVar36 + lVar24))[1] = uVar12;
          *(undefined8 *)(lVar36 + 0x18 + lVar24) = uVar42;
          *(longlong *)(uVar34 + 0x30) = lVar43 + 1;
          *(longlong *)(uVar34 + 0x40) = *(longlong *)(uVar34 + 0x40) + 1;
          **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
          local_a8 = extraout_RAX_04;
          local_88 = extraout_RAX_04;
          if (local_78[1][8] != '\0') {
            plVar28 = *(longlong **)local_78[2];
            LOCK();
            *plVar28 = *plVar28 + -1;
            UNLOCK();
            if (*plVar28 == 0) {
              alloc::sync::Arc::
              drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                        ();
            }
          }
        }
        goto LAB_140b2e5e4;
      }
      local_a8 = (ulonglong *)(local_78[1] + 9);
      **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
      if (local_78[1][8] != '\0') {
        plVar28 = *(longlong **)local_78[2];
        LOCK();
        *plVar28 = *plVar28 + -1;
        UNLOCK();
        if (*plVar28 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                    ();
        }
      }
      local_78[1][8] = 0;
      *(ulonglong *)(local_78[5] + 8) =
           CONCAT35((undefined3)uStack_b4,CONCAT41(uStack_b8,uStack_b9));
      *(undefined8 **)(local_78[4] + 9) = local_c8;
      *(ulonglong *)(local_78[5] + 1) = CONCAT17(uStack_b9,uStack_c0);
      *(ulonglong **)(local_78[3] + 9) = local_d8;
      *(undefined1 (**) [16])(local_78[4] + 1) = pauStack_d0;
      *(ulonglong **)(local_78[2] + 9) = local_e8;
      *(undefined1 (**) [16])(local_78[3] + 1) = pauStack_e0;
      *local_a8 = local_f8;
      *(int **)(local_78[2] + 1) = piStack_f0;
    }
    else if (iVar40 != 2) {
      get_ui_volume();
      uVar26 = extraout_RAX_02[7];
      uVar11 = extraout_RAX_02[5];
      uVar12 = extraout_RAX_02[6];
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      uVar34 = *in_stack_00000040;
      lVar43 = *(longlong *)(uVar34 + 0x30);
      uVar42 = *(undefined8 *)(uVar34 + 0x40);
      if (lVar43 == *(longlong *)(uVar34 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      lVar36 = *(longlong *)(uVar34 + 0x28);
      lVar24 = lVar43 * 0x20;
      *(ulonglong *)(lVar36 + 0x10 + lVar24) = uVar26;
      *(ulonglong *)(lVar36 + lVar24) = uVar11;
      ((ulonglong *)(lVar36 + lVar24))[1] = uVar12;
      *(undefined8 *)(lVar36 + 0x18 + lVar24) = uVar42;
      *(longlong *)(uVar34 + 0x30) = lVar43 + 1;
      *(longlong *)(uVar34 + 0x40) = *(longlong *)(uVar34 + 0x40) + 1;
      **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
      local_a8 = extraout_RAX_02;
      local_88 = extraout_RAX_02;
      if (local_78[1][8] != '\0') {
        plVar28 = *(longlong **)local_78[2];
        LOCK();
        *plVar28 = *plVar28 + -1;
        UNLOCK();
        if (*plVar28 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                    ();
        }
      }
LAB_140b2e5e4:
      pauStack_e0 = pauStack_160;
      local_e8 = local_168;
      piStack_f0 = piStack_170;
      local_f8 = local_178;
      *(undefined2 *)(local_78[1] + 8) = 0x101;
      *(undefined4 *)(local_78[1] + 10) = (undefined4)local_88;
      *(undefined2 *)(local_78[1] + 0xe) = local_88._4_2_;
      *(ulonglong **)local_78[2] = local_a8;
      *(int *)(local_78[2] + 8) = iVar40;
      *(undefined1 (**) [16])(local_78[2] + 0xc) = local_148;
      *(uint *)(local_78[3] + 4) = local_140;
      *(undefined1 (**) [16])(local_78[3] + 8) = pauVar27;
      *(ulonglong *)local_78[4] = local_178;
      *(int **)(local_78[4] + 8) = piStack_170;
      *(ulonglong **)local_78[5] = local_168;
      *(undefined1 (**) [16])(local_78[5] + 8) = pauStack_160;
      local_a0 = pauVar27;
      local_8c = iVar40;
      goto LAB_140b2e64c;
    }
    bVar13 = true;
    bVar14 = true;
  }
  else {
    puVar37 = *(uint **)(*local_78 + 8);
    plVar41 = (longlong *)(ulonglong)*puVar37;
    local_140 = puVar37[3];
    local_148 = *(undefined1 (**) [16])(puVar37 + 1);
    pauVar27 = *(undefined1 (**) [16])(puVar37 + 4);
    local_178 = *(ulonglong *)(puVar37 + 6);
    piStack_170 = *(int **)(puVar37 + 8);
    local_168 = *(ulonglong **)(puVar37 + 10);
    pauStack_160 = *(undefined1 (**) [16])(puVar37 + 0xc);
    cVar16 = local_78[1][8];
    if (cVar16 != '\x02') goto LAB_140b2e29d;
LAB_140b2e362:
    bVar13 = false;
    bVar14 = false;
  }
LAB_140b2e652:
  puVar37 = (uint *)*in_RDX;
  uVar22 = *puVar37;
  bVar38 = (byte)pauVar18 | ((bVar46 && bVar51 || bVar45 && bVar50) || (bVar10 || uVar22 == 3));
  pauVar33 = (undefined1 (*) [16])(ulonglong)bVar38;
  if (bVar38 == 0) {
                    // WARNING: Could not recover jumptable at 0x000140b2e6a9. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_142aeba5c + *(int *)(&DAT_142aeba5c + (ulonglong)uVar22 * 4)))
              (&DAT_142aeba5c + *(int *)(&DAT_142aeba5c + (ulonglong)uVar22 * 4));
    return;
  }
  pauVar32 = pauVar27;
  local_98 = in_RDX;
  if (bVar13) {
    if (!bVar14) goto LAB_140b2ece3;
LAB_140b2e86a:
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    lVar43 = *in_stack_00000030;
    lVar36 = *(longlong *)(lVar43 + 0x30);
    uVar42 = *(undefined8 *)(lVar43 + 0x40);
    if (lVar36 == *(longlong *)(lVar43 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar24 = *(longlong *)(lVar43 + 0x28);
    lVar30 = lVar36 * 0x20;
    *(undefined8 *)(lVar24 + lVar30) = uVar42;
    *(undefined4 *)(lVar24 + 8 + lVar30) = 0;
    *(undefined1 *)(lVar24 + 0x18 + lVar30) = 0;
    *(longlong *)(lVar43 + 0x30) = lVar36 + 1;
    *(longlong *)(lVar43 + 0x40) = *(longlong *)(lVar43 + 0x40) + 1;
    goto LAB_140b2f051;
  }
  bVar15 = local_78[1][9] - 2;
  bVar38 = 3;
  if (bVar15 < 3) {
    bVar38 = bVar15;
  }
  pauVar4 = local_78 + 2;
  iVar40 = (int)plVar41;
  if (bVar38 - 1 < 2) {
    if (((ulonglong)pauVar18 & 1) == 0 &&
        (((!bVar46 || !bVar51) && (!bVar45 || !bVar50)) &&
        (((!bVar48 || !bVar53) && (!bVar47 || !bVar52)) && uVar22 != 3))) {
      pauVar27 = local_78 + 6;
      uVar26 = *(ulonglong *)local_78[6];
      if (uVar26 < 0xfffffffffffffff0 && (char)(((uint)uVar26 & 8) >> 3) == '\0') {
        LOCK();
        bVar45 = uVar26 == *(ulonglong *)*pauVar27;
        if (bVar45) {
          *(ulonglong *)*pauVar27 = uVar26 + 0x10;
        }
        UNLOCK();
        if (!bVar45) goto LAB_140b2e9d6;
      }
      else {
LAB_140b2e9d6:
        parking_lot::raw_rwlock::RawRwLock::lock_shared_slow();
      }
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      plVar41 = (longlong *)((ulonglong)pauVar27 | 1);
      parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
      if (*(longlong *)local_78[8] == 0) {
        lVar43 = 0;
        pauVar33 = local_78;
      }
      else {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
        lVar43 = *(longlong *)(local_78[6] + 8);
        bVar38 = (byte)((ulonglong)extraout_RAX_05 >> 0x39);
        auVar55 = ZEXT216(CONCAT11(bVar38,bVar38));
        auVar55 = pshuflw(auVar55,auVar55,0);
        lVar36 = 0;
        pauVar33 = (undefined1 (*) [16])0x0;
        puVar35 = extraout_RAX_05;
        while( true ) {
          uVar26 = *(ulonglong *)local_78[7] & (ulonglong)puVar35;
          pcVar19 = (char *)(lVar43 + uVar26);
          cVar16 = auVar55[0];
          auVar67[0] = -(*pcVar19 == cVar16);
          cVar56 = auVar55[1];
          auVar67[1] = -(pcVar19[1] == cVar56);
          cVar58 = auVar55[2];
          auVar67[2] = -(pcVar19[2] == cVar58);
          cVar60 = auVar55[3];
          auVar67[3] = -(pcVar19[3] == cVar60);
          auVar67[4] = -(pcVar19[4] == cVar16);
          auVar67[5] = -(pcVar19[5] == cVar56);
          auVar67[6] = -(pcVar19[6] == cVar58);
          auVar67[7] = -(pcVar19[7] == cVar60);
          auVar67[8] = -(pcVar19[8] == cVar16);
          auVar67[9] = -(pcVar19[9] == cVar56);
          auVar67[10] = -(pcVar19[10] == cVar58);
          auVar67[0xb] = -(pcVar19[0xb] == cVar60);
          auVar67[0xc] = -(pcVar19[0xc] == cVar16);
          auVar67[0xd] = -(pcVar19[0xd] == cVar56);
          auVar67[0xe] = -(pcVar19[0xe] == cVar58);
          auVar67[0xf] = -(pcVar19[0xf] == cVar60);
          uVar31 = (ushort)(SUB161(auVar67 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar67 >> 0xf,0) & 1) << 1 |
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
                   (ushort)(SUB161(auVar67 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar67[0xf] >> 7) << 0xf;
          uVar22 = (uint)uVar31;
          if (uVar31 != 0) {
            local_150 = *(longlong *)(local_78[3] + 8);
            do {
              uVar8 = 0;
              for (uVar9 = uVar22; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar8 = uVar8 + 1;
              }
              uVar34 = uVar8 + uVar26 & *(ulonglong *)local_78[7];
              local_a0 = pauVar33;
              if (local_150 == *(longlong *)(lVar43 + -0x10 + uVar34 * -0x10)) {
                lVar36 = lVar43 + uVar34 * -0x10;
                pauVar33 = in_RCX;
                goto LAB_140b2ebba;
              }
              uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
              uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
            } while (uVar31 != 0);
          }
          auVar64[0] = -(*pcVar19 == -1);
          auVar64[1] = -(pcVar19[1] == -1);
          auVar64[2] = -(pcVar19[2] == -1);
          auVar64[3] = -(pcVar19[3] == -1);
          auVar64[4] = -(pcVar19[4] == -1);
          auVar64[5] = -(pcVar19[5] == -1);
          auVar64[6] = -(pcVar19[6] == -1);
          auVar64[7] = -(pcVar19[7] == -1);
          auVar64[8] = -(pcVar19[8] == -1);
          auVar64[9] = -(pcVar19[9] == -1);
          auVar64[10] = -(pcVar19[10] == -1);
          auVar64[0xb] = -(pcVar19[0xb] == -1);
          auVar64[0xc] = -(pcVar19[0xc] == -1);
          auVar64[0xd] = -(pcVar19[0xd] == -1);
          auVar64[0xe] = -(pcVar19[0xe] == -1);
          auVar64[0xf] = -(pcVar19[0xf] == -1);
          if ((((((((((((((((SUB161(auVar64 >> 7,0) & 1) != 0 || (SUB161(auVar64 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar64 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar64 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar64 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar64 >> 0x2f,0) & 1) != 0) || (SUB161(auVar64 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar64 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar64 >> 0x47,0) & 1) != 0) || (SUB161(auVar64 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar64 >> 0x57,0) & 1) != 0) || (SUB161(auVar64 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar64 >> 0x67,0) & 1) != 0) || (SUB161(auVar64 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar64 >> 0x77,0) & 1) != 0) || auVar64[0xf] < '\0') break;
          puVar35 = pauVar33[1] + uVar26;
          pauVar33 = pauVar33 + 1;
        }
LAB_140b2ebba:
        lVar43 = lVar36 + -0x10;
        if (lVar36 == 0) {
          lVar43 = 0;
        }
      }
      pauVar18 = (undefined1 (*) [16])(lVar43 + 8);
      if (lVar43 == 0) {
        pauVar18 = pauVar4;
      }
      plVar28 = *(longlong **)*pauVar18;
      LOCK();
      lVar43 = *plVar28;
      *plVar28 = *plVar28 + 1;
      UNLOCK();
      if (*plVar28 == 0 || SCARRY8(lVar43,1) != *plVar28 < 0) {
LAB_140b2f2c5:
                    // WARNING: Does not return
        pcVar7 = (code *)invalidInstructionException();
        (*pcVar7)();
      }
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      uVar26 = *(ulonglong *)*pauVar27;
      *(longlong *)*pauVar27 = *(longlong *)*pauVar27 + -0x10;
      UNLOCK();
      if ((uVar26 & 0xfffffffffffffff2) == 0x12) {
        parking_lot::raw_rwlock::RawRwLock::unlock_shared_slow();
      }
      uStack_80 = (undefined *)CONCAT17(1,(undefined7)uStack_80);
      local_98 = (ulonglong *)(local_78[2] + 8);
      local_c8 = *(undefined8 **)(local_78[5] + 8);
      local_f8 = *local_98;
      piStack_f0 = *(int **)local_78[3];
      local_e8 = *(ulonglong **)(local_78[3] + 8);
      pauStack_e0 = *(undefined1 (**) [16])local_78[4];
      local_d8 = *(ulonglong **)(local_78[4] + 8);
      pauStack_d0 = *(undefined1 (**) [16])local_78[5];
      **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
      if (local_78[1][8] != '\0') {
        plVar6 = *(longlong **)*pauVar4;
        LOCK();
        *plVar6 = *plVar6 + -1;
        UNLOCK();
        if (*plVar6 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                    ();
        }
      }
      *(undefined2 *)(local_78[1] + 8) = 0x402;
      *(undefined4 *)(local_78[1] + 10) = (undefined4)local_88;
      *(undefined2 *)(local_78[1] + 0xe) = local_88._4_2_;
      *(longlong **)local_78[2] = plVar28;
      *(undefined8 **)(local_78[5] + 8) = local_c8;
    }
    else {
      local_c8 = *(undefined8 **)(local_78[5] + 8);
      uVar34 = *(ulonglong *)(local_78[2] + 8);
      piStack_f0 = *(int **)local_78[3];
      local_128 = *(ulonglong **)(local_78[3] + 8);
      pauStack_120 = *(undefined1 (**) [16])local_78[4];
      local_118 = *(ulonglong **)(local_78[4] + 8);
      pauStack_110 = *(undefined1 (**) [16])local_78[5];
      local_98 = *(ulonglong **)local_78[2];
      pauVar18 = *(undefined1 (**) [16])(local_78[3] + 8);
      LOCK();
      uVar26 = *local_98;
      *local_98 = *local_98 + 1;
      UNLOCK();
      if (*local_98 == 0 || SCARRY8(uVar26,1) != (longlong)*local_98 < 0) goto LAB_140b2f2c5;
      bVar45 = pauVar27 == pauVar18 && iVar40 != 2;
      local_150 = CONCAT71(local_150._1_7_,bVar45);
      **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
      if (local_78[1][8] != '\0') {
        plVar28 = *(longlong **)*pauVar4;
        LOCK();
        *plVar28 = *plVar28 + -1;
        UNLOCK();
        if (*plVar28 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                    ();
        }
      }
      local_78[1][8] = 2;
      local_78[1][9] = bVar45;
      local_f8._0_4_ = (undefined4)uVar34;
      *(undefined4 *)(local_78[1] + 10) = (undefined4)local_f8;
      local_f8._4_2_ = (undefined2)(uVar34 >> 0x20);
      *(undefined2 *)(local_78[1] + 0xe) = local_f8._4_2_;
      *(ulonglong **)local_78[2] = local_98;
      *(undefined8 **)(local_78[5] + 8) = local_c8;
      local_f8 = uVar34;
      local_e8 = local_128;
      pauStack_e0 = pauStack_120;
      local_d8 = local_118;
      pauStack_d0 = pauStack_110;
    }
    *(ulonglong **)(local_78[4] + 8) = local_d8;
    *(undefined1 (**) [16])local_78[5] = pauStack_d0;
    *(ulonglong **)(local_78[3] + 8) = local_e8;
    *(undefined1 (**) [16])local_78[4] = pauStack_e0;
    *(ulonglong *)(local_78[2] + 8) = local_f8;
    *(int **)local_78[3] = piStack_f0;
    pauVar32 = pauVar27;
LAB_140b2ece3:
    pcVar19 = *(char **)local_78[2];
    cVar16 = pcVar19[0x88];
    if (cVar16 == '\x02') goto LAB_140b2ef19;
LAB_140b2ecf7:
    if (cVar16 == '\x01') {
      bVar38 = pcVar19[0x89];
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar36 = *in_stack_00000030;
      lVar43 = *(longlong *)(lVar36 + 0x30);
      uVar42 = *(undefined8 *)(lVar36 + 0x40);
      if (lVar43 == *(longlong *)(lVar36 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      plVar41 = (longlong *)(lVar36 + 0x40);
      lVar24 = *(longlong *)(lVar36 + 0x28);
      plVar28 = (longlong *)(lVar36 + 0x30);
      lVar30 = lVar43 * 0x20;
      lVar36 = lVar24 + lVar30;
      *(undefined8 *)(lVar24 + lVar30) = uVar42;
      *(undefined4 *)(lVar24 + 8 + lVar30) = 0;
    }
    else {
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar36 = *in_stack_00000030;
      lVar43 = *(longlong *)(lVar36 + 0x30);
      uVar42 = *(undefined8 *)(lVar36 + 0x40);
      if (lVar43 == *(longlong *)(lVar36 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                  ();
      }
      plVar41 = (longlong *)(lVar36 + 0x40);
      lVar24 = *(longlong *)(lVar36 + 0x28);
      plVar28 = (longlong *)(lVar36 + 0x30);
      lVar30 = lVar43 * 0x20;
      lVar36 = lVar24 + lVar30;
      *(undefined8 *)(lVar24 + lVar30) = uVar42;
      *(undefined4 *)(lVar24 + 8 + lVar30) = 0;
      bVar38 = 0;
    }
    goto LAB_140b2eff3;
  }
  if (bVar38 == 0) goto LAB_140b2ece3;
  if (iVar40 == 2) {
    **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
    if (local_78[1][8] != '\0') {
      plVar41 = *(longlong **)*pauVar4;
      LOCK();
      *plVar41 = *plVar41 + -1;
      UNLOCK();
      if (*plVar41 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                  ();
      }
    }
    local_78[1][8] = 0;
    *(ulonglong *)(local_78[5] + 8) = CONCAT35((undefined3)uStack_b4,CONCAT41(uStack_b8,uStack_b9));
    *(undefined8 **)(local_78[4] + 9) = local_c8;
    *(ulonglong *)(local_78[5] + 1) = CONCAT17(uStack_b9,uStack_c0);
    *(ulonglong **)(local_78[3] + 9) = local_d8;
    *(undefined1 (**) [16])(local_78[4] + 1) = pauStack_d0;
    *(ulonglong **)(local_78[2] + 9) = local_e8;
    *(undefined1 (**) [16])(local_78[3] + 1) = pauStack_e0;
    *(ulonglong *)(local_78[1] + 9) = local_f8;
    *(int **)(local_78[2] + 1) = piStack_f0;
    goto LAB_140b2e86a;
  }
  if (pauVar27 == *(undefined1 (**) [16])(local_78[3] + 8)) {
    local_98 = *(ulonglong **)*pauVar4;
    LOCK();
    uVar26 = *local_98;
    *local_98 = *local_98 + 1;
    UNLOCK();
    if (*local_98 == 0 || SCARRY8(uVar26,1) != (longlong)*local_98 < 0) goto LAB_140b2f2c5;
    **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
    if (local_78[1][8] != '\0') {
      plVar28 = *(longlong **)*pauVar4;
      LOCK();
      *plVar28 = *plVar28 + -1;
      UNLOCK();
      if (*plVar28 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                  ();
      }
    }
  }
  else {
    get_ui_volume();
    uVar26 = extraout_RAX_07[7];
    uVar34 = extraout_RAX_07[5];
    uVar11 = extraout_RAX_07[6];
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    pauVar32 = (undefined1 (*) [16])*in_stack_00000040;
    lVar43 = *(longlong *)pauVar32[3];
    pauVar33 = *(undefined1 (**) [16])pauVar32[4];
    if (lVar43 == *(longlong *)pauVar32[2]) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    lVar36 = *(longlong *)(pauVar32[2] + 8);
    lVar24 = lVar43 * 0x20;
    *(ulonglong *)(lVar36 + 0x10 + lVar24) = uVar26;
    *(ulonglong *)(lVar36 + lVar24) = uVar34;
    ((ulonglong *)(lVar36 + lVar24))[1] = uVar11;
    *(undefined1 (**) [16])(lVar36 + 0x18 + lVar24) = pauVar33;
    plVar41 = (longlong *)(lVar43 + 1);
    *(longlong **)pauVar32[3] = plVar41;
    *(longlong *)pauVar32[4] = *(longlong *)pauVar32[4] + 1;
    **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
    local_98 = extraout_RAX_07;
    local_88 = extraout_RAX_07;
    if (local_78[1][8] != '\0') {
      plVar28 = *(longlong **)*pauVar4;
      LOCK();
      *plVar28 = *plVar28 + -1;
      UNLOCK();
      if (*plVar28 == 0) {
        alloc::sync::Arc::
        drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                  ();
      }
    }
  }
  pauStack_e0 = pauStack_160;
  local_e8 = local_168;
  piStack_f0 = piStack_170;
  local_f8 = local_178;
  *(undefined2 *)(local_78[1] + 8) = 0x101;
  *(undefined4 *)(local_78[1] + 10) = (undefined4)local_88;
  *(undefined2 *)(local_78[1] + 0xe) = local_88._4_2_;
  *(ulonglong **)local_78[2] = local_98;
  *(int *)(local_78[2] + 8) = iVar40;
  *(undefined1 (**) [16])(local_78[2] + 0xc) = local_148;
  *(uint *)(local_78[3] + 4) = local_140;
  *(undefined1 (**) [16])(local_78[3] + 8) = pauVar27;
  *(ulonglong *)local_78[4] = local_178;
  *(int **)(local_78[4] + 8) = piStack_170;
  *(ulonglong **)local_78[5] = local_168;
  *(undefined1 (**) [16])(local_78[5] + 8) = pauStack_160;
  pcVar19 = *(char **)local_78[2];
  cVar16 = pcVar19[0x88];
  local_a0 = pauVar27;
  local_8c = iVar40;
  if (cVar16 != '\x02') goto LAB_140b2ecf7;
LAB_140b2ef19:
  bVar38 = pcVar19[0x89];
  puVar20 = (undefined *)(ulonglong)bVar38;
  fVar68 = (float)*(undefined8 *)(pcVar19 + 0x8c);
  fVar69 = (float)((ulonglong)*(undefined8 *)(pcVar19 + 0x8c) >> 0x20);
  fVar2 = *(float *)(pcVar19 + 0x94);
  cVar16 = SUB41(fVar2,0);
  cVar56 = (char)((uint)fVar2 >> 8);
  cVar58 = (char)((uint)fVar2 >> 0x10);
  cVar60 = (char)((uint)fVar2 >> 0x18);
  pbVar1 = (byte *)(pcVar19 + 0x98);
  puVar17 = &DAT_142aeba70;
  switch(*pbVar1) {
  case 0:
    lVar43 = 8;
    goto code_r0x000140b2ef80;
  case 1:
    if (*(ulonglong *)(pcVar19 + 0xb0) <= *(ulonglong *)local_78[4]) {
                    // WARNING: Subroutine does not return
      core::panicking::panic_bounds_check();
    }
    pcVar19 = (char *)(*(ulonglong *)local_78[4] * 0xc + *(longlong *)(pcVar19 + 0xa8));
    goto code_r0x000140b2ef89;
  case 2:
  case 3:
  case 4:
  case 5:
  case 7:
    uVar39 = 0;
    goto code_r0x000140b2ef9b;
  case 6:
    lVar43 = 4;
code_r0x000140b2ef80:
    pcVar19 = pcVar19 + lVar43 + 0x98;
code_r0x000140b2ef89:
    fVar68 = fVar68 + (float)*(undefined8 *)pcVar19;
    fVar69 = fVar69 + (float)((ulonglong)*(undefined8 *)pcVar19 >> 0x20);
    fVar2 = fVar2 + *(float *)(pcVar19 + 8);
    cVar16 = SUB41(fVar2,0);
    cVar56 = (char)((uint)fVar2 >> 8);
    cVar58 = (char)((uint)fVar2 >> 0x10);
    cVar60 = (char)((uint)fVar2 >> 0x18);
    uVar39 = 1;
code_r0x000140b2ef9b:
    *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
    lVar36 = *in_stack_00000030;
    lVar43 = *(longlong *)(lVar36 + 0x30);
    uVar42 = *(undefined8 *)(lVar36 + 0x40);
    if (lVar43 == *(longlong *)(lVar36 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                ();
    }
    plVar41 = (longlong *)(lVar36 + 0x40);
    lVar24 = *(longlong *)(lVar36 + 0x28);
    plVar28 = (longlong *)(lVar36 + 0x30);
    lVar30 = lVar43 * 0x20;
    lVar36 = lVar24 + lVar30;
    *(undefined8 *)(lVar24 + lVar30) = uVar42;
    *(undefined4 *)(lVar24 + 8 + lVar30) = uVar39;
    *(ulonglong *)(lVar24 + 0xc + lVar30) = CONCAT44(fVar69,fVar68);
    *(uint *)(lVar24 + 0x14 + lVar30) = CONCAT13(cVar60,CONCAT12(cVar58,CONCAT11(cVar56,cVar16)));
LAB_140b2eff3:
    *(byte *)(lVar36 + 0x18) = bVar38;
    *plVar28 = lVar43 + 1;
    *plVar41 = *plVar41 + 1;
    if (((local_78[5][9] & 8) == 0) || (2 < *puVar37)) {
LAB_140b2f051:
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar43 = *in_stack_00000038;
      lVar36 = *(longlong *)(lVar43 + 0x30);
      uVar42 = *(undefined8 *)(lVar43 + 0x40);
      uVar44 = 0x100000003;
      if (lVar36 == *(longlong *)(lVar43 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
    }
    else {
      *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
      lVar43 = *in_stack_00000038;
      lVar36 = *(longlong *)(lVar43 + 0x30);
      uVar42 = *(undefined8 *)(lVar43 + 0x40);
      uVar44 = 0x100000002;
      if (lVar36 == *(longlong *)(lVar43 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
    }
    lVar24 = *(longlong *)(lVar43 + 0x28);
    *(undefined8 *)(lVar24 + lVar36 * 0x10) = uVar44;
    *(undefined8 *)(lVar24 + 8 + lVar36 * 0x10) = uVar42;
    *(longlong *)(lVar43 + 0x30) = lVar36 + 1;
    *(longlong *)(lVar43 + 0x40) = *(longlong *)(lVar43 + 0x40) + 1;
    if (1 < (byte)local_78[1][8]) {
      bVar38 = local_78[1][9] - 2;
      bVar15 = 3;
      if (bVar38 < 3) {
        bVar15 = bVar38;
      }
      if (bVar15 == 1) {
        lVar43 = *(longlong *)local_78[2];
        local_e8 = *(ulonglong **)(lVar43 + 0x50);
        local_f8 = *(undefined8 *)(lVar43 + 0x40);
        piStack_f0 = *(int **)(lVar43 + 0x48);
        *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
        uVar26 = *in_stack_00000040;
        lVar43 = *(longlong *)(uVar26 + 0x30);
        uVar42 = *(undefined8 *)(uVar26 + 0x40);
        if (lVar43 == *(longlong *)(uVar26 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
      }
      else {
        if (bVar15 != 3) goto LAB_140b2f1fe;
        if ((local_78[1][9] & 1) == 0) {
          lVar43 = *(longlong *)local_78[2];
          local_e8 = *(ulonglong **)(lVar43 + 0x68);
          local_f8 = *(undefined8 *)(lVar43 + 0x58);
          piStack_f0 = *(int **)(lVar43 + 0x60);
          *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
          uVar26 = *in_stack_00000040;
          lVar43 = *(longlong *)(uVar26 + 0x30);
          uVar42 = *(undefined8 *)(uVar26 + 0x40);
          if (lVar43 == *(longlong *)(uVar26 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
        }
        else {
          lVar43 = *(longlong *)local_78[2];
          local_e8 = *(ulonglong **)(lVar43 + 0x80);
          local_f8 = *(undefined8 *)(lVar43 + 0x70);
          piStack_f0 = *(int **)(lVar43 + 0x78);
          *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
          uVar26 = *in_stack_00000040;
          lVar43 = *(longlong *)(uVar26 + 0x30);
          uVar42 = *(undefined8 *)(uVar26 + 0x40);
          if (lVar43 == *(longlong *)(uVar26 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                      ();
          }
        }
      }
      lVar36 = *(longlong *)(uVar26 + 0x28);
      lVar24 = lVar43 * 0x20;
      *(ulonglong **)(lVar36 + 0x10 + lVar24) = local_e8;
      *(undefined8 *)(lVar36 + lVar24) = local_f8;
      ((undefined8 *)(lVar36 + lVar24))[1] = piStack_f0;
      *(undefined8 *)(lVar36 + 0x18 + lVar24) = uVar42;
      *(longlong *)(uVar26 + 0x30) = lVar43 + 1;
      *(longlong *)(uVar26 + 0x40) = *(longlong *)(uVar26 + 0x40) + 1;
    }
LAB_140b2f1fe:
    **(undefined4 **)in_RCX[1] = *(undefined4 *)(in_RCX[1] + 0xc);
    hashbrown::raw::RawTable::
    clear<tuple_<country_core::systems::ui::resources::ui_state::WidgetId,alloc::sync::Arc<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>_>,alloc::alloc::Global>
              ();
    if (bVar49) {
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_08 == 1) {
        plVar41 = extraout_RAX_08 + 1;
      }
      else if ((*extraout_RAX_08 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), plVar41 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0)) {
LAB_140b2f28d:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      if (*plVar41 != 0) {
LAB_140b2f299:
                    // WARNING: Subroutine does not return
        core::cell::panic_already_borrowed();
      }
      *plVar41 = -1;
      puffin::ThreadProfiler::end_scope();
      *plVar41 = *plVar41 + 1;
    }
    return;
  case 8:
    uVar22 = (int)pcVar19 + 0xe9000000;
    pcVar19 = (char *)(ulonglong)uVar22;
    *pcVar19 = *pcVar19 + (char)uVar22;
    pcVar21 = &DAT_142aeba70;
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
    if ((byte)pcVar19[0x18] < 2) goto LAB_140b30577;
    goto code_r0x000140b30567;
  case 9:
    goto code_r0x000140b30481;
  case 10:
    if ((longlong)(&DAT_142aeba70 + *(int *)(&DAT_142aeba70 + (ulonglong)*pbVar1 * 4)) < 0) {
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    pcVar21 = 
    "/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/bevy_ecs-0.16.0/src/component.rs";
code_r0x000140b30567:
    if ((*(char **)(pcVar19 + 0x38) == pcVar21) && (puVar17 = puVar20, pcVar19[0x19] != '\x02'))
    goto LAB_140b3043b;
LAB_140b30577:
LAB_140b3006b:
    if (local_78 == (undefined1 (*) [16])0x0) {
      if (unaff_XMM6_Bh != '\0') {
        puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
        if (*extraout_RAX_13 == 1) {
          local_78 = (undefined1 (*) [16])(extraout_RAX_13 + 1);
        }
        else if ((*extraout_RAX_13 != 0) ||
                (std::sys::thread_local::native::lazy::Storage::
                 initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                           (), local_78 = (undefined1 (*) [16])extraout_RAX_14,
                extraout_RAX_14 == (longlong *)0x0)) {
LAB_140b307c4:
                    // WARNING: Subroutine does not return
          std::thread::local::panic_access_error();
        }
        if (*(longlong *)local_78 != 0) {
LAB_140b3078a:
                    // WARNING: Subroutine does not return
          core::cell::panic_already_borrowed();
        }
        *(longlong *)local_78 = -1;
        puffin::ThreadProfiler::end_scope();
LAB_140b306a0:
        *(longlong *)local_78 = *(longlong *)local_78 + 1;
      }
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
      return;
    }
    if ((short)local_68 == 0) {
      do {
        auVar55 = *pauVar33;
        uVar31 = (ushort)(SUB161(auVar55 >> 7,0) & 1) | (ushort)(SUB161(auVar55 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar55 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar55 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar55 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar55 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar55 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar55 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar55 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar55 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar55 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar55 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar55 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar55 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar55 >> 0x77,0) & 1) << 0xe |
                 (ushort)(byte)(auVar55[0xf] >> 7) << 0xf;
        puVar37 = puVar37 + -0xaa0;
        pauVar33 = pauVar33 + 1;
      } while (uVar31 == 0xffff);
      uVar22 = ~(uint)uVar31;
      local_68 = uVar22 & -(uint)uVar31 - 2;
    }
    else {
      uVar22 = local_68;
      local_68 = local_68 - 1 & local_68;
    }
    local_78 = (undefined1 (*) [16])(local_78[-1] + 0xf);
    uVar8 = 0;
    for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    in_RCX = *(undefined1 (**) [16])(puVar37 + (ulonglong)uVar8 * -0xaa + -0xaa);
    if (plVar41[9] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar43 = plVar41[6];
      bVar38 = (byte)(extraout_RAX_10 >> 0x39);
      auVar55 = ZEXT216(CONCAT11(bVar38,bVar38));
      auVar55 = pshuflw(auVar55,auVar55,0);
      lVar36 = 0;
      uVar26 = extraout_RAX_10;
      while( true ) {
        uVar26 = plVar41[7] & uVar26;
        pcVar19 = (char *)(lVar43 + uVar26);
        cVar54 = auVar55[0];
        auVar65[0] = -(*pcVar19 == cVar54);
        cVar57 = auVar55[1];
        auVar65[1] = -(pcVar19[1] == cVar57);
        cVar59 = auVar55[2];
        auVar65[2] = -(pcVar19[2] == cVar59);
        cVar61 = auVar55[3];
        auVar65[3] = -(pcVar19[3] == cVar61);
        auVar65[4] = -(pcVar19[4] == cVar54);
        auVar65[5] = -(pcVar19[5] == cVar57);
        auVar65[6] = -(pcVar19[6] == cVar59);
        auVar65[7] = -(pcVar19[7] == cVar61);
        auVar65[8] = -(pcVar19[8] == cVar54);
        auVar65[9] = -(pcVar19[9] == cVar57);
        auVar65[10] = -(pcVar19[10] == cVar59);
        auVar65[0xb] = -(pcVar19[0xb] == cVar61);
        auVar65[0xc] = -(pcVar19[0xc] == cVar54);
        auVar65[0xd] = -(pcVar19[0xd] == cVar57);
        auVar65[0xe] = -(pcVar19[0xe] == cVar59);
        auVar65[0xf] = -(pcVar19[0xf] == cVar61);
        uVar31 = (ushort)(SUB161(auVar65 >> 7,0) & 1) | (ushort)(SUB161(auVar65 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar65 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar65 >> 0x77,0) & 1) << 0xe | (ushort)(auVar65[0xf] >> 7) << 0xf
        ;
        uVar22 = (uint)uVar31;
        while (uVar31 != 0) {
          uVar8 = 0;
          for (uVar9 = uVar22; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar8 = uVar8 + 1;
          }
          uVar34 = uVar8 + uVar26 & plVar41[7];
          if (*(undefined1 (**) [16])(lVar43 + -0x10 + uVar34 * -0x10) == in_RCX) {
            uVar26 = *(ulonglong *)(lVar43 + -8 + uVar34 * -0x10);
            if (uVar26 >> 0x20 == 0) goto LAB_140b30290;
            uVar34 = uVar26 & 0xffffffff;
            if ((uVar34 < *(ulonglong *)(local_158 + 0x10)) &&
               (*(int *)(*(longlong *)(local_158 + 8) + uVar34 * 0x14) == (int)(uVar26 >> 0x20))) {
              lVar43 = *(longlong *)(local_158 + 8) + uVar34 * 0x14;
              uVar26 = (ulonglong)*(uint *)(lVar43 + 4);
              if ((uVar26 != 0xffffffff) &&
                 ((uVar26 < *(ulonglong *)(local_150 + 0x38) &&
                  ((*(ulonglong *)
                     (*(longlong *)(local_150 + 0x28) + (ulonglong)(*(uint *)(lVar43 + 4) >> 6) * 8)
                    >> (uVar26 & 0x3f) & 1) != 0)))) {
                fVar2 = *(float *)(*(longlong *)
                                    (*(longlong *)
                                      (*(longlong *)(local_158 + 0x1a8) + 0x18 +
                                      (ulonglong)*(uint *)(lVar43 + 0xc) * 0x48) + 0x10 +
                                    ~*(ulonglong *)
                                      (*(longlong *)
                                        (*(longlong *)(local_158 + 0x1a8) + 0x38 +
                                        (ulonglong)*(uint *)(lVar43 + 0xc) * 0x48) +
                                      *(longlong *)(local_150 + 0x110) * 8) * 0x30) +
                                  (ulonglong)*(uint *)(lVar43 + 0x10) * 8);
                if ((fVar2 != unaff_XMM9_Da) || (NAN(fVar2) || NAN(unaff_XMM9_Da)))
                goto LAB_140b3006b;
                goto LAB_140b30290;
              }
            }
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
          uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
        }
        auVar62[0] = -(*pcVar19 == cVar16);
        auVar62[1] = -(pcVar19[1] == cVar56);
        auVar62[2] = -(pcVar19[2] == cVar58);
        auVar62[3] = -(pcVar19[3] == cVar60);
        auVar62[4] = -(pcVar19[4] == '\0');
        auVar62[5] = -(pcVar19[5] == '\0');
        auVar62[6] = -(pcVar19[6] == '\0');
        auVar62[7] = -(pcVar19[7] == '\0');
        auVar62[8] = -(pcVar19[8] == '\0');
        auVar62[9] = -(pcVar19[9] == '\0');
        auVar62[10] = -(pcVar19[10] == '\0');
        auVar62[0xb] = -(pcVar19[0xb] == '\0');
        auVar62[0xc] = -(pcVar19[0xc] == '\0');
        auVar62[0xd] = -(pcVar19[0xd] == '\0');
        auVar62[0xe] = -(pcVar19[0xe] == '\0');
        auVar62[0xf] = -(pcVar19[0xf] == '\0');
        if ((((((((((((((((SUB161(auVar62 >> 7,0) & 1) != 0 || (SUB161(auVar62 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar62 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar62 >> 0x1f,0) & 1) != 0) || (SUB161(auVar62 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar62 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar62 >> 0x37,0) & 1) != 0) || (SUB161(auVar62 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar62 >> 0x47,0) & 1) != 0) || (SUB161(auVar62 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar62 >> 0x57,0) & 1) != 0) || (SUB161(auVar62 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar62 >> 0x67,0) & 1) != 0) || (SUB161(auVar62 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar62 >> 0x77,0) & 1) != 0) || auVar62[0xf] < '\0') break;
        uVar26 = uVar26 + lVar36 + 0x10;
        lVar36 = lVar36 + 0x10;
      }
    }
LAB_140b30290:
    if (plVar41[3] == 0) {
LAB_140b306dc:
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar43 = *plVar41;
    bVar38 = (byte)(extraout_RAX_11 >> 0x39);
    auVar55 = ZEXT216(CONCAT11(bVar38,bVar38));
    auVar55 = pshuflw(auVar55,auVar55,0);
    lVar36 = 0;
    uVar26 = extraout_RAX_11;
    while( true ) {
      uVar26 = plVar41[1] & uVar26;
      pcVar19 = (char *)(lVar43 + uVar26);
      cVar54 = auVar55[0];
      auVar66[0] = -(*pcVar19 == cVar54);
      cVar57 = auVar55[1];
      auVar66[1] = -(pcVar19[1] == cVar57);
      cVar59 = auVar55[2];
      auVar66[2] = -(pcVar19[2] == cVar59);
      cVar61 = auVar55[3];
      auVar66[3] = -(pcVar19[3] == cVar61);
      auVar66[4] = -(pcVar19[4] == cVar54);
      auVar66[5] = -(pcVar19[5] == cVar57);
      auVar66[6] = -(pcVar19[6] == cVar59);
      auVar66[7] = -(pcVar19[7] == cVar61);
      auVar66[8] = -(pcVar19[8] == cVar54);
      auVar66[9] = -(pcVar19[9] == cVar57);
      auVar66[10] = -(pcVar19[10] == cVar59);
      auVar66[0xb] = -(pcVar19[0xb] == cVar61);
      auVar66[0xc] = -(pcVar19[0xc] == cVar54);
      auVar66[0xd] = -(pcVar19[0xd] == cVar57);
      auVar66[0xe] = -(pcVar19[0xe] == cVar59);
      auVar66[0xf] = -(pcVar19[0xf] == cVar61);
      uVar31 = (ushort)(SUB161(auVar66 >> 7,0) & 1) | (ushort)(SUB161(auVar66 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar66 >> 0x77,0) & 1) << 0xe | (ushort)(auVar66[0xf] >> 7) << 0xf;
      uVar22 = (uint)uVar31;
      if (uVar31 != 0) break;
LAB_140b30320:
      auVar63[0] = -(*pcVar19 == cVar16);
      auVar63[1] = -(pcVar19[1] == cVar56);
      auVar63[2] = -(pcVar19[2] == cVar58);
      auVar63[3] = -(pcVar19[3] == cVar60);
      auVar63[4] = -(pcVar19[4] == '\0');
      auVar63[5] = -(pcVar19[5] == '\0');
      auVar63[6] = -(pcVar19[6] == '\0');
      auVar63[7] = -(pcVar19[7] == '\0');
      auVar63[8] = -(pcVar19[8] == '\0');
      auVar63[9] = -(pcVar19[9] == '\0');
      auVar63[10] = -(pcVar19[10] == '\0');
      auVar63[0xb] = -(pcVar19[0xb] == '\0');
      auVar63[0xc] = -(pcVar19[0xc] == '\0');
      auVar63[0xd] = -(pcVar19[0xd] == '\0');
      auVar63[0xe] = -(pcVar19[0xe] == '\0');
      auVar63[0xf] = -(pcVar19[0xf] == '\0');
      if ((((((((((((((((SUB161(auVar63 >> 7,0) & 1) != 0 || (SUB161(auVar63 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar63 >> 0x17,0) & 1) != 0) || (SUB161(auVar63 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar63 >> 0x27,0) & 1) != 0) || (SUB161(auVar63 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar63 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar63 >> 0x3f,0) & 1) != 0) || (SUB161(auVar63 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar63 >> 0x4f,0) & 1) != 0) || (SUB161(auVar63 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar63 >> 0x5f,0) & 1) != 0) || (SUB161(auVar63 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar63 >> 0x6f,0) & 1) != 0) || (SUB161(auVar63 >> 0x77,0) & 1) != 0) ||
          auVar63[0xf] < '\0') goto LAB_140b306dc;
      uVar26 = uVar26 + lVar36 + 0x10;
      lVar36 = lVar36 + 0x10;
    }
LAB_140b302f6:
    uVar8 = 0;
    for (uVar9 = uVar22; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    lVar24 = (uVar8 + uVar26 & plVar41[1]) * -0x2a8;
    if (*(undefined1 (**) [16])(lVar43 + -0x2a8 + lVar24) != in_RCX) goto code_r0x000140b30311;
    if ((*(ulonglong *)(lVar43 + -0x290 + lVar24) < 2) ||
       (*(float *)(lVar43 + -0x270 + lVar24) <= fVar68)) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    if (*(char *)(lVar43 + lVar24 + -9) != '\0') goto LAB_140b3006b;
    if ((int)piStack_170 - 1U < 2) {
      if ((local_148 != in_RCX) || (*(uint *)(lVar43 + lVar24 + -0x48) < 2)) goto LAB_140b3006b;
    }
    else if ((int)piStack_170 != 0) {
      if (unaff_XMM6_Bh == '\0') {
        return;
      }
      puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
      if (*extraout_RAX_15 == 1) {
        local_78 = (undefined1 (*) [16])(extraout_RAX_15 + 1);
      }
      else if ((*extraout_RAX_15 != 0) ||
              (std::sys::thread_local::native::lazy::Storage::
               initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                         (), local_78 = (undefined1 (*) [16])extraout_RAX_16,
              extraout_RAX_16 == (longlong *)0x0)) goto LAB_140b307c4;
      if (*(longlong *)local_78 != 0) goto LAB_140b3078a;
      *(longlong *)local_78 = -1;
      puffin::ThreadProfiler::end_scope();
      goto LAB_140b306a0;
    }
    country_core::resources::walls::public_walls::PublicWalls::roof_component();
    uStack_80 = (undefined *)CONCAT44(uStack_13c,local_140);
    wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::widget();
    puVar17 = (undefined *)(lVar43 + lVar24 + -0x2a0);
    local_88 = extraout_RAX_12;
LAB_140b3043b:
    wall::adjust_wall_height::adjust_wall_ui_mode::UiWallHeight::signifier();
    cVar54 = '\x02';
    if ((1 < (byte)local_88[3]) && (puVar17 = (undefined *)local_88[7], puVar17 == uStack_80)) {
      cVar54 = *(char *)((longlong)local_88 + 0x19);
    }
    pcVar19 = (char *)(ulonglong)(byte)(cVar54 - 2);
code_r0x000140b30481:
    bVar38 = (byte)pcVar19;
    if (2 < (byte)pcVar19) {
      bVar38 = 3;
    }
    bVar49 = 0xfffffffebd51456f <
             (ulonglong)(longlong)*(int *)(&DAT_142aeba90 + (ulonglong)bVar38 * 4);
    puVar20 = &DAT_142aeba90 + *(int *)(&DAT_142aeba90 + (ulonglong)bVar38 * 4);
    switch(bVar38) {
    case 0:
      goto LAB_140b30577;
    case 1:
      *piStack_f0 = local_8c;
      uVar26 = *local_a8;
      uVar34 = *(ulonglong *)*local_a0;
      if (uVar34 == *local_98) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar43 = *(longlong *)(local_f8 + 0x28);
      *(ulonglong *)(lVar43 + uVar34 * 0x10) = uVar26;
      *(undefined1 (**) [16])(lVar43 + 8 + uVar34 * 0x10) = in_RCX;
      *(ulonglong *)(local_f8 + 0x30) = uVar34 + 1;
      *(longlong *)(local_f8 + 0x40) = *(longlong *)(local_f8 + 0x40) + 1;
      *(undefined4 *)local_118 = uStack_b4;
      uVar26 = *local_d8;
      puVar20 = *(undefined **)*pauStack_d0;
      if (puVar20 == *(undefined **)CONCAT17(uStack_b9,uStack_c0)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(ulonglong *)(*(longlong *)(pauStack_110[2] + 8) + (longlong)puVar20 * 8) = uVar26;
      pauVar18 = pauStack_d0;
      puVar25 = local_d8;
      break;
    case 2:
      *piStack_f0 = local_8c;
      pauVar32 = (undefined1 (*) [16])*local_a8;
      puVar20 = *(undefined **)*local_a0;
      if (puVar20 == (undefined *)*local_98) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      pcVar19 = *(char **)(local_f8 + 0x28);
code_r0x000140b305c9:
      *(undefined1 (**) [16])(pcVar19 + (longlong)puVar20 * 0x10) = pauVar32;
      *(undefined1 (**) [16])(pcVar19 + (longlong)puVar20 * 0x10 + 8) = in_RCX;
      pauVar18 = local_a0;
      puVar25 = local_a8;
      break;
    case 3:
      *(undefined4 *)*pauStack_120 = uStack_b8;
      uVar26 = *local_e8;
      puVar20 = *(undefined **)*pauStack_e0;
      if (puVar20 == (undefined *)*local_c8) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      uVar34 = local_128[5];
      *(ulonglong *)(uVar34 + (longlong)puVar20 * 0x10) = uVar26;
      *(undefined1 (**) [16])(uVar34 + 8 + (longlong)puVar20 * 0x10) = in_RCX;
      pauVar18 = pauStack_e0;
      puVar25 = local_e8;
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
      goto code_r0x000142aeba8f;
    default:
      goto code_r0x000142aeba90;
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
      goto code_r0x000142aeba92;
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
      puVar20 = (undefined *)CONCAT71((int7)((ulonglong)puVar20 >> 8),(byte)puVar20 - 2);
      goto code_r0x000142aeba90;
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
      goto code_r0x000142aeba8d;
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
      bVar49 = 1 < (byte)puVar20;
    case 0x31:
    case 0x53:
    case 0x83:
    case 0x84:
    case 0xa4:
    case 0xc0:
    case 199:
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
      goto code_r0x000142aeba8d;
    case 0xcc:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0xdf:
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    *(undefined **)*pauVar18 = puVar20 + 1;
    *puVar25 = *puVar25 + 1;
    goto LAB_140b3006b;
  case 0xb:
    goto code_r0x000140b305c9;
  case 0xc:
  case 0xd:
  case 0x11:
  case 0x28:
  case 0x3f:
  case 0x5f:
  case 100:
  case 0x70:
  case 0x73:
  case 0x74:
  case 0x78:
  case 0x93:
  case 0x97:
  case 0x9c:
  case 0xb0:
  case 199:
  case 0xe8:
  case 0xec:
  case 0xf5:
    break;
  default:
    halt_baddata();
  case 0x10:
  case 0x19:
  case 0x1c:
  case 0x1d:
  case 0x30:
  case 0x4c:
  case 0x4f:
  case 0x7b:
  case 0x7c:
  case 0x90:
  case 0xa4:
  case 0xab:
  case 0xb7:
  case 0xc0:
  case 0xd0:
  case 0xdf:
  case 0xf0:
    pcVar19 = (char *)CONCAT71((int7)((ulonglong)pcVar19 >> 8),(char)pcVar19 + -2);
  case 0x39:
  case 0x5b:
  case 0x8b:
  case 0x8c:
  case 0xac:
  case 200:
  case 0xcf:
    if ((longlong)*(int *)(&DAT_142aeba70 + (ulonglong)*pbVar1 * 4) != -0x142aeba70) {
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    cVar16 = (char)pcVar19;
    *pcVar19 = *pcVar19 + cVar16;
    *pcVar19 = *pcVar19 + cVar16;
    *pcVar19 = *pcVar19 + cVar16;
    pcVar21 = (char *)CONCAT71((int7)((ulonglong)pcVar19 >> 8),cVar16 + *pcVar19);
    *pcVar21 = *pcVar21 + cVar16 + *pcVar19;
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  case 0x14:
  case 0x15:
  case 0x18:
  case 0x35:
  case 0x48:
  case 0x4b:
  case 0x50:
  case 0x77:
  case 0x84:
  case 0x94:
  case 0xa7:
  case 0xb4:
  case 0xb8:
  case 0xdb:
  case 0xe3:
  case 0xed:
  case 0xfd:
    halt_baddata();
  case 0x21:
  case 0x31:
  case 0x34:
  case 0x54:
  case 0x6b:
  case 0x6c:
  case 0x87:
  case 0x98:
  case 0x9b:
  case 0x9f:
  case 0xbf:
  case 0xcb:
  case 0xcc:
  case 0xf8:
    break;
  case 0x24:
  case 0x29:
  case 0x2d:
  case 0x6f:
  case 0x80:
  case 0x83:
  case 0x8f:
  case 0xa3:
  case 0xa8:
  case 0xbb:
  case 0xbc:
  case 0xe0:
  case 0xf4:
  case 0xfc:
                    // WARNING: Bad instruction - Truncating control flow here
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
    halt_baddata();
  case 0x25:
  case 0x2c:
  case 0x38:
  case 0x3c:
  case 0x40:
  case 0x43:
  case 0x44:
  case 0x53:
  case 0x68:
  case 0xa0:
  case 0xaf:
  case 0xb3:
  case 0xd3:
  case 0xdc:
  case 0xe9:
  case 0xf1:
  case 0xf9:
    halt_baddata();
  case 0x57:
  case 0x60:
  case 99:
  case 0x67:
  case 0x88:
  case 0xc3:
  case 0xc4:
  case 0xd7:
  case 0xe4:
    break;
  case 0xd4:
    puVar20[0x2c] = puVar20[0x2c] + -1;
    halt_baddata();
                    // WARNING: Read-only address (ram,0x000142aeba90) is written
  case 0xe7:
                    // WARNING: Bad instruction - Truncating control flow here
    halt_baddata();
  }
                    // WARNING: Bad instruction - Truncating control flow here
  halt_baddata();
code_r0x000140b30311:
  uVar31 = (ushort)(uVar22 - 1) & (ushort)uVar22;
  uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar31);
  if (uVar31 == 0) goto LAB_140b30320;
  goto LAB_140b302f6;
code_r0x000142aeba90:
  out(0x4a,(int)puVar20);
code_r0x000142aeba92:
  bVar49 = 1 < (byte)puVar20;
code_r0x000142aeba94:
  *(uint *)(puVar17 + 4) = *(int *)(puVar17 + 4) + 0x42aeba90 + (uint)bVar49;
                    // WARNING: Bad instruction - Truncating control flow here
  halt_baddata();
code_r0x000142aeba8d:
code_r0x000142aeba8f:
                    // WARNING: Bad instruction - Truncating control flow here
  halt_baddata();
}


