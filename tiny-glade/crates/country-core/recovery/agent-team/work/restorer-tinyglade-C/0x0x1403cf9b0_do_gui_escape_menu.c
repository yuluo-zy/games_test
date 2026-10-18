// do_gui_escape_menu @ 0x0x1403cf9b0
// sig: void __cdecl do_gui_escape_menu(void)
// size: 3634


void __cdecl tiny_glade::gui::gui_escape_menu::do_gui_escape_menu(void)

{
  undefined8 *puVar1;
  char *pcVar2;
  longlong *plVar3;
  longlong lVar4;
  undefined **ppuVar5;
  undefined *puVar6;
  undefined *puVar7;
  longlong lVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  bool bVar12;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  char extraout_AL_03;
  undefined4 extraout_EAX;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  longlong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  longlong extraout_RAX_08;
  ulonglong extraout_RAX_09;
  longlong extraout_RAX_10;
  ulonglong extraout_RAX_11;
  longlong extraout_RAX_12;
  ulonglong extraout_RAX_13;
  longlong extraout_RAX_14;
  undefined8 *in_RDX;
  longlong *plVar13;
  ulonglong uVar14;
  ulonglong uVar15;
  undefined8 *in_R8;
  longlong *in_R9;
  longlong *plVar16;
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong *in_stack_00000058;
  longlong *in_stack_00000068;
  longlong *in_stack_00000078;
  byte local_156;
  char local_155;
  undefined4 local_154;
  undefined **local_150;
  longlong *local_148;
  longlong *local_140;
  byte *local_138;
  undefined4 *local_130;
  longlong *local_128;
  undefined *local_120;
  undefined8 uStack_118;
  undefined4 local_c0;
  undefined4 local_bc;
  longlong local_b8;
  undefined4 *local_b0;
  undefined8 *local_a8;
  int *local_a0;
  undefined *local_98;
  undefined8 local_90;
  longlong local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined *local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  
  local_a0 = (int *)*in_RDX;
  pcVar2 = (char *)in_RDX[4];
  plVar3 = (longlong *)in_RDX[8];
  lVar4 = in_RDX[0xc];
  local_68 = (undefined *)*in_R8;
  uStack_60 = in_R8[1];
  local_58 = in_R8[2];
  uStack_50 = in_R8[3];
  local_b8 = in_R8[4];
  local_a8 = (undefined8 *)in_stack_00000068[4];
  local_b0 = (undefined4 *)in_stack_00000068[6];
  local_c0 = *(undefined4 *)((longlong)in_stack_00000068 + 0x3c);
  local_156 = 0;
  local_155 = '\0';
  local_154 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  local_130 = (undefined4 *)in_stack_00000028[2];
  *local_130 = local_154;
  ppuVar5 = (undefined **)*in_stack_00000028;
  local_120 = &DAT_1429d1b49;
  uStack_118 = 10;
  toolbar::mei::core::impl$0::as_id();
  plVar16 = (longlong *)(extraout_RAX ^ 0xb1695320004fb443);
  plVar13 = plVar16;
  toolbar::mei::core::MeiContext::reuse_or_create_node();
  *(undefined1 *)(extraout_RAX_00 + 0x5d) = 0;
  if (((ulonglong)plVar13 & 1) == 0) {
    puVar6 = ppuVar5[2];
    if (puVar6 == *ppuVar5) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    puVar7 = ppuVar5[1];
    *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 2;
    *(longlong **)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = plVar16;
    ppuVar5[2] = puVar6 + 1;
  }
  local_140 = (longlong *)0x0;
  local_150 = ppuVar5;
  local_148 = plVar16;
  toolbar::mei::config::impl$18::template();
  local_98 = &DAT_1429d1b5c;
  local_90 = 6;
  local_88 = 1;
  local_80 = 0;
  local_78 = 1;
  local_70 = 0;
  local_bc = 0x41a00000;
  if (*(char *)(lVar4 + 1) == '\0') {
    local_150 = &local_98;
    local_148 = &local_88;
    local_140 = (longlong *)&local_bc;
    local_138 = (byte *)&local_78;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_0,ref_<str_>_>();
  }
  if (*pcVar2 == '\x01') {
    country_core::input::action_set::DigitalActionState::just_pressed();
    if (extraout_AL == '\0') {
LAB_1403cfc78:
      bVar12 = false;
    }
    else {
      country_core::systems::ui::resources::ui_state::DragState::is_some();
      if (extraout_AL_00 == '\0') {
        country_core::resources::gui::EscMenuState::is_closed();
        if (extraout_AL_01 == '\0') {
          local_155 = '\x01';
          goto LAB_1403cfc78;
        }
        local_156 = 1;
        bVar12 = true;
      }
      else {
        local_128 = in_R9;
        country_core::systems::history::history_process::HistoryCmd::interrupt();
        *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
        lVar8 = *in_stack_00000030;
        lVar9 = *(longlong *)(lVar8 + 0x30);
        uVar10 = *(undefined8 *)(lVar8 + 0x40);
        if (lVar9 == *(longlong *)(lVar8 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar11 = *(longlong *)(lVar8 + 0x28);
        *(undefined8 *)(lVar11 + lVar9 * 0x10) = uVar10;
        *(undefined4 *)(lVar11 + 8 + lVar9 * 0x10) = extraout_EAX;
        *(longlong *)(lVar8 + 0x30) = lVar9 + 1;
        *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
        bVar12 = false;
        in_R9 = local_128;
      }
    }
    country_core::input::action_set::DigitalActionState::just_pressed();
    if ((extraout_AL_02 != '\0') && ((int)*plVar3 != 8)) {
      *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
      lVar8 = *in_stack_00000040;
      lVar9 = *(longlong *)(lVar8 + 0x30);
      uVar10 = *(undefined8 *)(lVar8 + 0x40);
      if (lVar9 == *(longlong *)(lVar8 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar8 + 0x28) + lVar9 * 8) = uVar10;
      *(longlong *)(lVar8 + 0x30) = lVar9 + 1;
      *(longlong *)(lVar8 + 0x40) = *(longlong *)(lVar8 + 0x40) + 1;
      *(int *)in_R9[2] = *(int *)((longlong)in_R9 + 0x1c);
      uStack_118 = CONCAT71(uStack_118._1_7_,1);
      local_120 = (undefined *)0x8;
      country_core::resources::gui::NextEscMenuState::set();
    }
  }
  else {
    bVar12 = false;
  }
  country_core::resources::gui::EscMenuState::is_closed();
  if ((extraout_AL_03 == '\0') || (*(char *)(lVar4 + 1) != '\0')) {
    if ((int)*plVar3 == 3) {
      *local_130 = local_154;
      local_120 = &DAT_1429d1b6a;
      uStack_118 = 4;
      toolbar::mei::core::impl$0::as_id();
      plVar16 = (longlong *)(extraout_RAX_01 ^ 0xb1695320004fb443);
      plVar13 = plVar16;
      toolbar::mei::core::MeiContext::reuse_or_create_node();
      *(undefined1 *)(extraout_RAX_02 + 0x5d) = 0;
      if (((ulonglong)plVar13 & 1) == 0) {
        puVar6 = ppuVar5[2];
        if (puVar6 == *ppuVar5) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        puVar7 = ppuVar5[1];
        *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 2;
        *(longlong **)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = plVar16;
        ppuVar5[2] = puVar6 + 1;
      }
      local_140 = (longlong *)0x0;
      local_150 = ppuVar5;
      local_148 = plVar16;
      toolbar::mei::config::impl$18::template();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_2,ref_<str_>_>();
    }
    if (!bVar12) goto joined_r0x0001403d01af;
  }
  else {
    *local_130 = local_154;
    local_120 = &DAT_1429d1b6a;
    uStack_118 = 4;
    toolbar::mei::core::impl$0::as_id();
    uVar15 = extraout_RAX_03 ^ 0xb1695320004fb443;
    uVar14 = uVar15;
    toolbar::mei::core::MeiContext::reuse_or_create_node();
    *(undefined1 *)(extraout_RAX_04 + 0x5d) = 0;
    if ((uVar14 & 1) == 0) {
      puVar6 = ppuVar5[2];
      if (puVar6 == *ppuVar5) {
        alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
      }
      puVar7 = ppuVar5[1];
      *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 2;
      *(ulonglong *)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = uVar15;
      ppuVar5[2] = puVar6 + 1;
    }
    local_140 = (longlong *)0x0;
    local_150 = ppuVar5;
    local_148 = (longlong *)uVar15;
    toolbar::mei::config::impl$18::template();
    local_150 = &local_68;
    local_148 = in_stack_00000040;
    local_138 = &local_156;
    local_140 = in_R9;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_1,ref_<str_>_>();
    if ((local_156 & 1) == 0) goto joined_r0x0001403d01af;
  }
  *(int *)in_R9[2] = *(int *)((longlong)in_R9 + 0x1c);
  local_120 = (undefined *)0x3;
  local_128 = plVar3;
  country_core::resources::gui::NextEscMenuState::set();
  if (*local_a0 != 0x13) {
    if (*local_a0 == 0xd) {
      *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
      lVar4 = *in_stack_00000030;
      lVar8 = *(longlong *)(lVar4 + 0x30);
      uVar10 = *(undefined8 *)(lVar4 + 0x40);
      if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar9 = *(longlong *)(lVar4 + 0x28);
      *(undefined8 *)(lVar9 + lVar8 * 0x10) = uVar10;
      *(undefined4 *)(lVar9 + 8 + lVar8 * 0x10) = 0x3000300;
      *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
      *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
      country_core::systems::mode_manager::impl$0::default();
      *local_b0 = local_c0;
      *local_a8 = local_120;
      local_a8[1] = uStack_118;
    }
    else {
      country_core::systems::mode_manager::impl$0::default();
      *(undefined4 *)in_stack_00000068[2] = *(undefined4 *)((longlong)in_stack_00000068 + 0x1c);
      lVar4 = *in_stack_00000068;
      lVar8 = *(longlong *)(lVar4 + 0x30);
      uVar10 = *(undefined8 *)(lVar4 + 0x40);
      if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar9 = *(longlong *)(lVar4 + 0x28);
      puVar1 = (undefined8 *)(lVar9 + lVar8 * 0x18);
      *puVar1 = local_120;
      puVar1[1] = uStack_118;
      *(undefined8 *)(lVar9 + 0x10 + lVar8 * 0x18) = uVar10;
      *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
      *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
    }
  }
  *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  lVar4 = *in_stack_00000038;
  lVar8 = *(longlong *)(lVar4 + 0x30);
  uVar10 = *(undefined8 *)(lVar4 + 0x40);
  if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(lVar4 + 0x28) + lVar8 * 8) = uVar10;
  *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
  *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
  lVar4 = *in_stack_00000048;
  lVar8 = *(longlong *)(lVar4 + 0x30);
  uVar10 = *(undefined8 *)(lVar4 + 0x40);
  if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
              ();
  }
  lVar9 = *(longlong *)(lVar4 + 0x28);
  *(undefined8 *)(lVar9 + lVar8 * 0x10) = uVar10;
  *(undefined1 *)(lVar9 + 8 + lVar8 * 0x10) = 10;
  *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
  *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  lVar4 = *in_stack_00000040;
  lVar8 = *(longlong *)(lVar4 + 0x30);
  uVar10 = *(undefined8 *)(lVar4 + 0x40);
  if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(lVar4 + 0x28) + lVar8 * 8) = uVar10;
  *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
  *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  *(undefined4 *)in_stack_00000058[2] = *(undefined4 *)((longlong)in_stack_00000058 + 0x1c);
  lVar4 = *in_stack_00000058;
  lVar8 = *(longlong *)(lVar4 + 0x30);
  uVar10 = *(undefined8 *)(lVar4 + 0x40);
  if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 *)(*(longlong *)(lVar4 + 0x28) + lVar8 * 8) = uVar10;
  *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
  *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
  plVar3 = local_128;
joined_r0x0001403d01af:
  if (local_155 != '\0') {
    *(undefined4 *)in_stack_00000048[2] = *(undefined4 *)((longlong)in_stack_00000048 + 0x1c);
    lVar4 = *in_stack_00000048;
    lVar8 = *(longlong *)(lVar4 + 0x30);
    uVar10 = *(undefined8 *)(lVar4 + 0x40);
    if (lVar8 == *(longlong *)(lVar4 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                ();
    }
    lVar9 = *(longlong *)(lVar4 + 0x28);
    *(undefined8 *)(lVar9 + lVar8 * 0x10) = uVar10;
    *(undefined1 *)(lVar9 + 8 + lVar8 * 0x10) = 0xb;
    *(longlong *)(lVar4 + 0x30) = lVar8 + 1;
    *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
    *(int *)in_R9[2] = *(int *)((longlong)in_R9 + 0x1c);
    local_120 = (undefined *)0x2;
    country_core::resources::gui::NextEscMenuState::set();
  }
  if (*plVar3 == 3) {
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
      *local_130 = local_154;
      local_120 = &DAT_1429d1b91;
      uStack_118 = 0xd;
      toolbar::mei::core::impl$0::as_id();
      uVar15 = extraout_RAX_11 ^ 0xb1695320004fb443;
      uVar14 = uVar15;
      toolbar::mei::core::MeiContext::reuse_or_create_node();
      *(undefined1 *)(extraout_RAX_12 + 0x5d) = 0;
      *(undefined1 *)(extraout_RAX_12 + 0xb7) = 1;
      if ((uVar14 & 1) == 0) {
        puVar6 = ppuVar5[2];
        if (puVar6 == *ppuVar5) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        puVar7 = ppuVar5[1];
        *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 2;
        *(ulonglong *)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = uVar15;
        ppuVar5[2] = puVar6 + 1;
      }
      local_140 = (longlong *)0x0;
      local_150 = ppuVar5;
      local_148 = (longlong *)uVar15;
      toolbar::mei::config::impl$18::template();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_5,ref_<str_>_>();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_6,ref_<str_>_>();
    }
    else {
      *local_130 = local_154;
      local_120 = &DAT_1429d1bbb;
      uStack_118 = 8;
      toolbar::mei::core::impl$0::as_id();
      uVar15 = extraout_RAX_07 ^ 0xb1695320004fb443;
      uVar14 = uVar15;
      toolbar::mei::core::MeiContext::reuse_or_create_node();
      *(undefined1 *)(extraout_RAX_08 + 0x5d) = 0;
      *(undefined1 *)(extraout_RAX_08 + 0xb7) = 1;
      if ((uVar14 & 1) == 0) {
        puVar6 = ppuVar5[2];
        if (puVar6 == *ppuVar5) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        puVar7 = ppuVar5[1];
        *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 2;
        *(ulonglong *)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = uVar15;
        ppuVar5[2] = puVar6 + 1;
      }
      local_140 = (longlong *)0x0;
      local_150 = ppuVar5;
      local_148 = (longlong *)uVar15;
      toolbar::mei::config::impl$18::template();
      if ((600.0 < *(float *)(local_b8 + 0x1f0)) || (2 < *(ulonglong *)(*in_stack_00000078 + 0x10)))
      {
        toolbar::mei::config::TemplateView::
        with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_3,ref_<str_>_>()
        ;
      }
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_4,ref_<str_>_>();
    }
    *local_130 = local_154;
    local_120 = &DAT_1429d1bd0;
    uStack_118 = 10;
    toolbar::mei::core::impl$0::as_id();
    uVar15 = extraout_RAX_13 ^ 0xb1695320004fb443;
    uVar14 = uVar15;
    toolbar::mei::core::MeiContext::reuse_or_create_node();
    *(undefined1 *)(extraout_RAX_14 + 0x5d) = 0;
    *(undefined1 *)(extraout_RAX_14 + 0xb7) = 1;
    if ((uVar14 & 1) == 0) {
      puVar6 = ppuVar5[2];
      if (puVar6 == *ppuVar5) {
        alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
      }
      puVar7 = ppuVar5[1];
      *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 1;
      *(ulonglong *)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = uVar15;
      ppuVar5[2] = puVar6 + 1;
    }
    local_140 = (longlong *)0x0;
    local_150 = ppuVar5;
    local_148 = (longlong *)uVar15;
    toolbar::mei::config::impl$18::template();
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_7,ref_<str_>_>();
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_8,ref_<str_>_>();
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_9,ref_<str_>_>();
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_10,ref_<str_>_>();
  }
  else if ((int)*plVar3 == 4) {
    if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
      *local_130 = local_154;
      local_120 = &DAT_1429d1bd0;
      uStack_118 = 10;
      toolbar::mei::core::impl$0::as_id();
      uVar15 = extraout_RAX_09 ^ 0xb1695320004fb443;
      uVar14 = uVar15;
      toolbar::mei::core::MeiContext::reuse_or_create_node();
      *(undefined1 *)(extraout_RAX_10 + 0x5d) = 0;
      *(undefined1 *)(extraout_RAX_10 + 0xb7) = 1;
      if ((uVar14 & 1) == 0) {
        puVar6 = ppuVar5[2];
        if (puVar6 == *ppuVar5) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        puVar7 = ppuVar5[1];
        *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 1;
        *(ulonglong *)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = uVar15;
        ppuVar5[2] = puVar6 + 1;
      }
      local_140 = (longlong *)0x0;
      local_150 = ppuVar5;
      local_148 = (longlong *)uVar15;
      toolbar::mei::config::impl$18::template();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_15,ref_<str_>_>();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_16,ref_<str_>_>();
    }
    else {
      *local_130 = local_154;
      local_120 = &DAT_1429d1bd0;
      uStack_118 = 10;
      toolbar::mei::core::impl$0::as_id();
      uVar15 = extraout_RAX_05 ^ 0xb1695320004fb443;
      uVar14 = uVar15;
      toolbar::mei::core::MeiContext::reuse_or_create_node();
      *(undefined1 *)(extraout_RAX_06 + 0x5d) = 0;
      *(undefined1 *)(extraout_RAX_06 + 0xb7) = 1;
      if ((uVar14 & 1) == 0) {
        puVar6 = ppuVar5[2];
        if (puVar6 == *ppuVar5) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        puVar7 = ppuVar5[1];
        *(undefined4 *)(puVar7 + (longlong)puVar6 * 0x38) = 1;
        *(ulonglong *)(puVar7 + (longlong)puVar6 * 0x38 + 0x30) = uVar15;
        ppuVar5[2] = puVar6 + 1;
      }
      local_140 = (longlong *)0x0;
      local_150 = ppuVar5;
      local_148 = (longlong *)uVar15;
      toolbar::mei::config::impl$18::template();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_11,ref_<str_>_>();
      if ((600.0 < *(float *)(local_b8 + 0x1f0)) || (2 < *(ulonglong *)(*in_stack_00000078 + 0x10)))
      {
        toolbar::mei::config::TemplateView::
        with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_12,ref_<str_>_>
                  ();
      }
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_13,ref_<str_>_>();
      toolbar::mei::config::TemplateView::
      with_slot<tiny_glade::gui::gui_escape_menu::do_gui_escape_menu::closure_env_16,ref_<str_>_>();
    }
  }
  return;
}

