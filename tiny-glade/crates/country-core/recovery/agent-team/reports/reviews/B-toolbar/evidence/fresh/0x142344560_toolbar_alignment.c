
// WARNING: Instruction at (ram,0x000142344d3b) overlaps instruction at (ram,0x000142344d3a)
// 
// WARNING: Removing unreachable block (ram,0x000142375bd1)
// WARNING: Removing unreachable block (ram,0x000142375f41)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl toolbar::toolbar_systems::toolbar_alignment(void)

{
  int *piVar1;
  byte bVar2;
  char cVar3;
  int iVar4;
  longlong *plVar5;
  undefined8 *puVar6;
  ulonglong uVar7;
  byte bVar8;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  byte **ppbVar9;
  byte *pbVar10;
  longlong lVar11;
  undefined8 *in_RCX;
  byte *pbVar12;
  byte *pbVar13;
  longlong *plVar14;
  longlong lVar15;
  undefined *puVar16;
  undefined8 *in_RDX;
  byte *pbVar17;
  byte *pbVar18;
  undefined4 *puVar19;
  byte *pbVar20;
  undefined8 uVar21;
  longlong *unaff_RBP;
  longlong lVar22;
  longlong lVar23;
  byte *in_R8;
  byte *pbVar24;
  uint uVar25;
  uint uVar26;
  undefined4 uVar27;
  longlong *in_R9;
  byte *pbVar28;
  longlong *unaff_R15;
  bool bVar29;
  float in_XMM0_Da;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float extraout_XMM0_Da_03;
  float extraout_XMM0_Da_04;
  float extraout_XMM0_Da_05;
  float fVar30;
  int in_XMM1_Da;
  float fVar31;
  int in_XMM1_Db;
  undefined1 in_XMM2 [16];
  undefined1 auVar32 [16];
  float fVar34;
  float unaff_XMM9_Da;
  longlong *in_stack_00000098;
  int iStack_1e8;
  byte *local_50;
  byte *local_48;
  undefined4 uVar33;
  
  pbVar17 = *(byte **)in_R8;
  local_48 = (byte *)*in_RDX;
  local_50 = pbVar17;
  uVar33 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  plVar5 = (longlong *)*in_R9;
  puVar19 = (undefined4 *)in_R9[2];
  puVar6 = (undefined8 *)*in_RCX;
  ppbVar9 = &local_48;
  pbVar12 = (byte *)0x0;
  do {
    uVar21 = in_XMM2._0_8_;
    pbVar20 = *ppbVar9;
    pbVar28 = (byte *)(ulonglong)*(uint *)(pbVar20 + 0x18);
    pbVar18 = (byte *)(ulonglong)*(uint *)(pbVar20 + 0x1c);
    uVar26 = *(uint *)(pbVar20 + 0x20);
    pbVar24 = (byte *)(ulonglong)*(uint *)(pbVar20 + 0x24);
    *puVar19 = uVar33;
    if (plVar5[0x15] != 0) {
      lVar22 = plVar5[0x14];
      pbVar17 = (byte *)(plVar5[0x15] * 0x28);
      in_R8 = (byte *)0x0;
      do {
        if (in_R8[lVar22 + 0x24] != 4) {
          uVar25 = 0x1020300 >> ((*(byte *)(puVar6 + 0x1e) & 3) << 3);
          in_R9 = (longlong *)(ulonglong)uVar25;
          in_R8[lVar22 + 0x24] = (byte)uVar25;
        }
        in_R8 = in_R8 + 0x28;
      } while (pbVar17 != in_R8);
    }
    uVar27 = SUB84(in_R9,0);
    pbVar13 = &DAT_14304e6d8;
    pbVar10 = &DAT_14304e6d8 + *(int *)(&DAT_14304e6d8 + (ulonglong)*(byte *)(puVar6 + 0x1e) * 4);
    bVar29 = pbVar10 == (byte *)0x0;
    bVar8 = (byte)pbVar10;
    plVar14 = plVar5;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*(byte *)(puVar6 + 0x1e)) {
    case 0:
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_12>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_11>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_10>();
      *puVar19 = uVar33;
      pbVar17 = (byte *)(ulonglong)uVar26;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_11>();
      in_R8 = pbVar24;
      in_XMM0_Da = extraout_XMM0_Da;
      break;
    case 1:
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_0>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env_1>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_2>();
      *puVar19 = uVar33;
      pbVar17 = (byte *)(ulonglong)uVar26;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_13>();
      in_R8 = pbVar24;
      in_XMM0_Da = extraout_XMM0_Da_02;
      break;
    case 2:
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env_0>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::undo_redo_alignment::closure_env_1>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_2>();
      *puVar19 = uVar33;
      pbVar17 = (byte *)(ulonglong)uVar26;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_11>();
      in_R8 = pbVar24;
      in_XMM0_Da = extraout_XMM0_Da_00;
      break;
    case 3:
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_12>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_13>();
      *puVar19 = uVar33;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_10>();
      *puVar19 = uVar33;
      pbVar17 = (byte *)(ulonglong)uVar26;
      resouces_and_components::taffy_utils::
      taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_13>();
      in_R8 = pbVar24;
      in_XMM0_Da = extraout_XMM0_Da_01;
      break;
    default:
code_r0x00014234485d:
                    // WARNING: Could not recover jumptable at 0x00014234486b. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&UNK_14304e6e8 + *(int *)(&UNK_14304e6e8 + (longlong)pbVar10 * 4)))();
      return;
    case 5:
    case 7:
      out(0xeb,bVar8);
      goto code_r0x0001423447d0;
    case 8:
      pbVar20 = pbVar20 + 0x50c139bda;
      bVar2 = *pbVar20;
      *pbVar20 = *pbVar20 + bVar8;
      *(int *)pbVar24 = (int)unaff_R15;
                    // WARNING: Could not recover jumptable at 0x0001423448e4. Too many branches
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_14304e6f8 +
                *(int *)(&DAT_14304e6f8 +
                        CONCAT71((int7)((ulonglong)pbVar10 >> 8),
                                 (bVar8 + 0xb4) - CARRY1(bVar2,bVar8)) * 4)))();
      return;
    case 9:
      *(char *)((longlong)unaff_RBP + -0x77) = *(char *)((longlong)unaff_RBP + -0x77) + bVar8;
      goto code_r0x000142375b40;
    case 10:
      *(char *)((longlong)unaff_RBP + -0x77) = *(char *)((longlong)unaff_RBP + -0x77) + bVar8;
code_r0x000142375b40:
      local_50 = (byte *)CONCAT44(uVar26,uVar33);
      local_48 = pbVar12;
      taffy::node::Taffy::style();
      if (iStack_1e8 == 4) {
        taffy::style::impl$16::clone();
        taffy::node::Taffy::set_style();
        return;
      }
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    case 0xb:
      *(char *)((longlong)unaff_RBP + -0x77) = *(char *)((longlong)unaff_RBP + -0x77) + bVar8;
      local_50 = (byte *)CONCAT44(uVar26,uVar33);
      local_48 = pbVar12;
      taffy::node::Taffy::style();
      if (iStack_1e8 == 4) {
        taffy::style::impl$16::clone();
        taffy::node::Taffy::set_style();
        return;
      }
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    case 0xc:
      *(int *)puVar6[2] = (int)pbVar10;
      pbVar24 = (byte *)*puVar6;
      pbVar24[0x10] = 0;
      pbVar24[0x11] = 0;
      pbVar24[0x12] = 0;
      pbVar24[0x13] = 0;
    case 0xe:
LAB_142344cd5:
      ::utils::easing_functions::EasingFunction::apply();
      in_XMM0_Da = extraout_XMM0_Da_05 * unaff_XMM9_Da * 100.0;
      *(undefined4 *)unaff_R15[2] = *(undefined4 *)((longlong)unaff_R15 + 0x1c);
      in_XMM1_Da = *(int *)pbVar24;
      in_XMM1_Db = *(int *)(pbVar24 + 4);
      pbVar10 = *(byte **)(*unaff_R15 + 0xa0);
      puVar16 = (undefined *)(*(longlong *)(*unaff_R15 + 0xa8) << 3);
code_r0x000142344d1b:
      pbVar17 = (byte *)((longlong)puVar16 * 5);
      pbVar13 = (byte *)0x0;
      in_R8 = pbVar10;
      do {
        uVar27 = SUB84(in_R9,0);
        bVar29 = pbVar17 == pbVar13;
code_r0x000142344d33:
        if (bVar29) {
          return;
        }
        pbVar12 = in_R8 + 0x10;
        in_R8 = in_R8 + 0x28;
        uVar21 = CONCAT44(-(uint)((int)((ulonglong)*(undefined8 *)pbVar12 >> 0x20) == in_XMM1_Db),
                          -(uint)((int)*(undefined8 *)pbVar12 == in_XMM1_Da));
code_r0x000142344d43:
        auVar32._0_4_ = (undefined4)uVar21;
        uVar33 = (undefined4)((ulonglong)uVar21 >> 0x20);
        auVar32._4_4_ = auVar32._0_4_;
        auVar32._8_4_ = uVar33;
        auVar32._12_4_ = uVar33;
        uVar26 = movmskpd(uVar27,auVar32);
        in_R9 = (longlong *)(ulonglong)uVar26;
        pbVar13 = pbVar13 + 0x28;
      } while (uVar26 != 3);
      fVar31 = 0.0;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
      switch(pbVar28[0xf0]) {
      case 0:
        in_XMM0_Da = -in_XMM0_Da;
      case 3:
        fVar30 = 0.0;
        fVar31 = in_XMM0_Da;
        break;
      case 2:
        in_XMM0_Da = -in_XMM0_Da;
      case 1:
        fVar30 = in_XMM0_Da;
      }
      *(float *)(pbVar10 + -0x20 + (longlong)pbVar13) = fVar30;
      *(float *)(pbVar10 + -0x1c + (longlong)pbVar13) = fVar31;
                    // WARNING: Read-only address (ram,0x00014304e65b) is written
                    // WARNING: Read-only address (ram,0x00014304e6d7) is written
      return;
    case 0xd:
      pbVar24 = (byte *)*puVar6;
      fVar31 = *(float *)(pbVar24 + 0x10);
      country_core::resources::gui::EscMenuState::is_closed();
      if (extraout_AL == '\0') {
code_r0x000142344be2:
        fVar30 = *(float *)(*plVar5 + 0x58) / 0.3;
code_r0x000142344bf2:
        uVar33 = *(undefined4 *)((longlong)puVar6 + 0x1c);
        puVar19 = (undefined4 *)puVar6[2];
        *puVar19 = uVar33;
        fVar30 = fVar30 + *(float *)(pbVar24 + 0x10);
      }
      else {
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        ::utils::geometry::aabb::Aabb2::contains();
        if (extraout_AL_00 == '\0') goto code_r0x000142344be2;
        country_core::input::action_set::DigitalActionState::pressed();
        if (extraout_AL_01 == '\0') {
          fVar30 = *(float *)(*plVar5 + 0x58) / -0.3;
          goto code_r0x000142344bf2;
        }
        fVar30 = *(float *)(pbVar24 + 0x10);
        uVar33 = *(undefined4 *)((longlong)puVar6 + 0x1c);
        puVar19 = (undefined4 *)puVar6[2];
      }
      fVar34 = 0.0;
      if (0.0 <= fVar30) {
        fVar34 = fVar30;
      }
      fVar30 = 1.0;
      if (fVar34 <= 1.0) {
        fVar30 = fVar34;
      }
      *puVar19 = uVar33;
      *(float *)(pbVar24 + 0x10) = fVar30;
      if ((fVar31 != 0.0) || (fVar30 <= 0.0)) {
        if ((fVar31 != 1.0) || ((NAN(fVar31) || (1.0 <= fVar30)))) goto LAB_142344cd5;
        *(undefined4 *)in_stack_00000098[2] = *(undefined4 *)((longlong)in_stack_00000098 + 0x1c);
        lVar22 = *in_stack_00000098;
        lVar23 = *(longlong *)(lVar22 + 0x30);
        uVar21 = *(undefined8 *)(lVar22 + 0x40);
        if (lVar23 == *(longlong *)(lVar22 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar11 = *(longlong *)(lVar22 + 0x28);
        lVar15 = lVar23 * 0x20;
        *(undefined4 *)(lVar11 + lVar15) = 0x11;
      }
      else {
        *(undefined4 *)in_stack_00000098[2] = *(undefined4 *)((longlong)in_stack_00000098 + 0x1c);
        lVar22 = *in_stack_00000098;
        lVar23 = *(longlong *)(lVar22 + 0x30);
        uVar21 = *(undefined8 *)(lVar22 + 0x40);
        if (lVar23 == *(longlong *)(lVar22 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<system_decorator::decorator_visual::stairs::PreferredNodeAlignment,alloc::alloc::Global>_>,alloc::alloc::Global>
                    ();
        }
        lVar11 = *(longlong *)(lVar22 + 0x28);
        lVar15 = lVar23 * 0x20;
        *(undefined4 *)(lVar11 + lVar15) = 0x10;
      }
      *(undefined8 *)(lVar11 + 0x18 + lVar15) = uVar21;
      *(longlong *)(lVar22 + 0x30) = lVar23 + 1;
      *(longlong *)(lVar22 + 0x40) = *(longlong *)(lVar22 + 0x40) + 1;
      goto LAB_142344cd5;
    case 0xf:
      *pbVar10 = *pbVar10 << 1 | (char)*pbVar10 < '\0';
                    // WARNING: Could not emulate address calculation at 0x000142344aea
                    // WARNING: Treating indirect jump as call
      (*(code *)(&DAT_14304e6d8 + *(int *)(&DAT_14304e6d8 + (longlong)pbVar10 * 4)))();
      return;
    case 0x10:
      goto code_r0x000142344d33;
    case 0x11:
      uVar27 = movmskps(uVar27,in_XMM2);
      goto code_r0x000142344d43;
    case 0x12:
      piVar1 = (int *)((longlong)plVar5 + 0xf);
      iVar4 = *piVar1;
      *piVar1 = *piVar1 << 1;
      if (iVar4 < 0 == *piVar1 < 0) goto code_r0x000142344d43;
      puVar16 = &UNK_14304e6d7;
      if (*piVar1 == 0) {
                    // WARNING (jumptable): Read-only address (ram,0x00014304e6d7) is written
                    // WARNING: Read-only address (ram,0x00014304e6d7) is written
        _UNK_14304e6d7 = SUB84(plVar5,0);
      }
      goto code_r0x000142344d1b;
    case 0x13:
                    // WARNING (jumptable): Read-only address (ram,0x00014304e65b) is written
                    // WARNING: Read-only address (ram,0x00014304e65b) is written
      UNK_14304e65b =
           (0xfffffffebcfb1927U <
           (ulonglong)(longlong)*(int *)(&DAT_14304e6d8 + (ulonglong)*(byte *)(puVar6 + 0x1e) * 4))
           + 'K';
      *pbVar10 = *pbVar10 >> 6;
      goto code_r0x000142344d43;
    }
    pbVar24 = *(byte **)(pbVar20 + 8);
    unaff_RBP = (longlong *)(*(longlong *)(pbVar20 + 0x10) << 5);
    pbVar28 = pbVar24;
    for (unaff_R15 = (longlong *)0x0; unaff_RBP != unaff_R15; unaff_R15 = unaff_R15 + 4) {
      pbVar18 = (byte *)(ulonglong)*(uint *)(pbVar28 + 0x10);
      if (*(uint *)(pbVar28 + 0x10) != 0) {
        uVar26 = *(uint *)(pbVar24 + 0x14 + (longlong)unaff_R15);
        pbVar20 = (byte *)(ulonglong)uVar26;
        cVar3 = *(char *)(puVar6 + 0x1e);
        *puVar19 = uVar33;
        if (1 < (byte)(cVar3 - 1U)) {
          resouces_and_components::taffy_utils::
          taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_10>();
          pbVar10 = (byte *)(ulonglong)*(byte *)(puVar6 + 0x1e);
          goto code_r0x00014234485d;
        }
        resouces_and_components::taffy_utils::
        taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_2>();
        switch(*(undefined1 *)(puVar6 + 0x1e)) {
        case 1:
code_r0x0001423447d0:
          *puVar19 = uVar33;
          in_R8 = (byte *)((ulonglong)pbVar20 & 0xffffffff);
          plVar14 = plVar5;
          resouces_and_components::taffy_utils::
          taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_13>();
          pbVar17 = pbVar18;
          in_XMM0_Da = extraout_XMM0_Da_03;
          break;
        case 2:
          *puVar19 = uVar33;
          in_R8 = (byte *)(ulonglong)uVar26;
          plVar14 = plVar5;
          resouces_and_components::taffy_utils::
          taffy_mut_style<toolbar::toolbar_systems::toolbar_alignment::closure_env_11>();
          pbVar17 = pbVar18;
          in_XMM0_Da = extraout_XMM0_Da_04;
        }
      }
      pbVar28 = pbVar28 + 0x20;
    }
    ppbVar9 = &local_50;
    uVar7 = (ulonglong)pbVar12 & 1;
    pbVar12 = (byte *)CONCAT71((int7)((ulonglong)plVar14 >> 8),1);
    if (uVar7 != 0) {
      return;
    }
  } while( true );
}

