// do_time_of_day_screen @ 0x1405cc7f0
// sig: void __cdecl do_time_of_day_screen(void)


void __cdecl tiny_glade::gui::gui_tod::do_time_of_day_screen(void)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong lVar3;
  longlong *plVar4;
  char extraout_AL;
  char extraout_AL_00;
  char cVar5;
  byte bVar6;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *in_RCX;
  longlong *in_RDX;
  ulonglong uVar7;
  ulonglong uVar8;
  undefined4 uVar9;
  longlong *in_R8;
  longlong *in_R9;
  undefined4 *puVar10;
  longlong lVar11;
  float in_XMM1_Da;
  undefined8 *in_stack_00000038;
  undefined8 in_stack_00000050;
  longlong *in_stack_00000058;
  undefined8 in_stack_00000060;
  undefined8 in_stack_00000068;
  undefined8 in_stack_00000070;
  undefined1 local_b6;
  undefined1 local_b5;
  float local_b4;
  longlong *local_b0;
  undefined1 *local_a8;
  undefined1 *local_a0;
  float *local_98;
  undefined8 local_90;
  undefined1 *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined *local_68;
  undefined8 local_60;
  
  country_core::resources::gui::EscMenuState::is_open();
  if (extraout_AL == '\0') {
    in_XMM1_Da = *(float *)(*in_stack_00000058 + 0x94);
    country_core::input::action_set::DigitalActionState::just_pressed();
    if (extraout_AL_00 != '\0') goto LAB_1405cc851;
    lVar11 = *in_RDX;
    if ((*(byte *)(lVar11 + 10) & 1) != 0) {
      bVar6 = *(byte *)(lVar11 + 9) ^ 1;
      goto LAB_1405cc864;
    }
  }
  else {
LAB_1405cc851:
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    lVar11 = *in_RDX;
    *(undefined1 *)(lVar11 + 10) = 0;
  }
  bVar6 = 0;
LAB_1405cc864:
  uVar9 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar10 = (undefined4 *)in_RDX[2];
  *puVar10 = uVar9;
  *(byte *)(lVar11 + 8) = bVar6;
  country_core::resources::window::WindowSize::as_vec2();
  local_b4 = in_XMM1_Da / 1000.0;
  cVar5 = *(char *)(lVar11 + 10);
  if (cVar5 == '\x01') {
    if (*(char *)(lVar11 + 8) != '\0') {
      if (*(int *)*in_stack_00000038 == 0xd) {
        *(undefined4 *)in_R9[2] = *(undefined4 *)((longlong)in_R9 + 0x1c);
        lVar11 = *in_R9;
        lVar1 = *(longlong *)(lVar11 + 0x30);
        uVar2 = *(undefined8 *)(lVar11 + 0x40);
        if (lVar1 == *(longlong *)(lVar11 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                    ();
        }
        lVar3 = *(longlong *)(lVar11 + 0x28);
        *(undefined8 *)(lVar3 + lVar1 * 0x10) = uVar2;
        *(undefined4 *)(lVar3 + 8 + lVar1 * 0x10) = 0x3000300;
        *(longlong *)(lVar11 + 0x30) = lVar1 + 1;
        *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      }
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      lVar11 = *in_R8;
      lVar1 = *(longlong *)(lVar11 + 0x30);
      uVar2 = *(undefined8 *)(lVar11 + 0x40);
      if (lVar1 == *(longlong *)(lVar11 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar11 + 0x28) + lVar1 * 8) = uVar2;
      *(longlong *)(lVar11 + 0x30) = lVar1 + 1;
      *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
    }
    uVar2 = in_stack_00000050;
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    plVar4 = (longlong *)*in_RCX;
    local_68 = &DAT_142a06cf9;
    local_60 = 3;
    toolbar::mei::core::impl$0::as_id();
    uVar8 = extraout_RAX ^ 0xb1695320004fb443;
    uVar7 = uVar8;
    toolbar::mei::core::MeiContext::reuse_or_create_node();
    *(undefined1 *)(extraout_RAX_00 + 0x5d) = 0;
    if ((uVar7 & 1) == 0) {
      lVar11 = plVar4[2];
      if (lVar11 == *plVar4) {
        alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
      }
      lVar1 = plVar4[1];
      *(undefined4 *)(lVar1 + lVar11 * 0x38) = 1;
      *(ulonglong *)(lVar1 + 0x30 + lVar11 * 0x38) = uVar8;
      plVar4[2] = lVar11 + 1;
    }
    local_a0 = (undefined1 *)0x0;
    local_b0 = plVar4;
    local_a8 = (undefined1 *)uVar8;
    toolbar::mei::config::impl$18::template();
    local_b6 = 0;
    local_b5 = 0;
    local_a8 = &local_b6;
    local_90 = uVar2;
    local_80 = in_stack_00000070;
    local_78 = in_stack_00000068;
    local_70 = in_stack_00000060;
    local_b0 = (longlong *)&local_b5;
    local_98 = &local_b4;
    local_88 = &stack0x00000030;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_tod::do_time_of_day_screen::closure_env_0,ref_<str_>_>();
    local_b5 = 1;
    local_90 = in_stack_00000050;
    local_80 = in_stack_00000070;
    local_78 = in_stack_00000068;
    local_70 = in_stack_00000060;
    local_b0 = (longlong *)&local_b5;
    local_a8 = &local_b6;
    local_98 = &local_b4;
    local_88 = &stack0x00000030;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_tod::do_time_of_day_screen::closure_env_0,ref_<str_>_>();
    local_a8 = (undefined1 *)in_stack_00000060;
    local_90 = in_stack_00000070;
    local_b0 = (longlong *)&stack0x00000030;
    local_a0 = &local_b6;
    toolbar::mei::config::TemplateView::
    with_slot<tiny_glade::gui::gui_tod::do_time_of_day_screen::closure_env_1,ref_<str_>_>();
    lVar11 = *in_RDX;
    puVar10 = (undefined4 *)in_RDX[2];
    cVar5 = *(char *)(lVar11 + 10);
    uVar9 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  }
  *puVar10 = uVar9;
  *(char *)(lVar11 + 9) = cVar5;
  return;
}

