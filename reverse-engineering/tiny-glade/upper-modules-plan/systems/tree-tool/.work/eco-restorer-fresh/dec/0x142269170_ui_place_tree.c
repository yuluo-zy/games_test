// ADDRESS 0x142269170 SIZE 1215 SIGNATURE void __cdecl ui_place_tree(void)

void __cdecl system_tree::ui_place_tree::ui_place_tree(void)

{
  undefined4 uVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  char *pcVar5;
  longlong lVar6;
  undefined4 uVar7;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000050;
  longlong in_stack_00000058;
  undefined8 local_2a8;
  code *pcStack_2a0;
  undefined8 *local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined1 local_1f0;
  undefined7 uStack_1ef;
  undefined4 *local_1e8;
  undefined *local_190;
  undefined8 uStack_188;
  undefined8 *local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  int local_160;
  int local_e0;
  undefined4 *local_c8;
  char *local_b8;
  undefined8 local_b0;
  undefined4 local_a4;
  float fStack_a0;
  undefined4 local_9c;
  undefined4 local_98;
  undefined4 local_94;
  undefined4 *local_90;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  if ((*(int *)*in_RCX == 0x10) &&
     (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0')) {
    lVar6 = *(longlong *)(in_stack_00000058 + 0x20);
    local_90 = *(undefined4 **)(in_stack_00000058 + 0x30);
    local_98 = *(undefined4 *)(in_stack_00000058 + 0x3c);
    lVar2 = *(longlong *)(in_stack_00000058 + 0x40);
    local_c8 = *(undefined4 **)(in_stack_00000058 + 0x50);
    local_94 = *(undefined4 *)(in_stack_00000058 + 0x5c);
    local_2a8 = (undefined8 *)CONCAT44(0x7f7fffff,(undefined4)local_2a8);
    local_2a8 = (undefined8 *)CONCAT71(local_2a8._1_7_,5);
    local_1f0 = 0;
    country_core::systems::ui::resources::widget_params::WidgetParams::new();
    country_core::systems::ui::resources::widget_params::WidgetParams::occlude_by_world();
    local_81 = 1;
    country_core::systems::ui::resources::widget_params::UiSounds::empty();
    local_81 = 0;
    country_core::systems::ui::resources::widget_params::WidgetParams::sounds();
    pcVar5 = "tree_canvascrates/systems/tree-tool/src/ui_place_tree.rsdecorator_";
    country_core::systems::ui::resources::ui_state::UiState::add<ref_<str_>_>();
    local_b8 = pcVar5;
    country_core::systems::collision::world::RaycastWorld::legacy_raycast();
    if (local_160 != 2) {
      local_298 = local_180;
      local_2a8 = (undefined8 *)local_190;
      pcStack_2a0 = (code *)uStack_188;
      country_core::systems::ui_edit_focus::geo_src_to_edit_subject();
      if (local_e0 != 0) {
        return;
      }
    }
    country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
    uVar7 = local_190._0_4_;
    country_core::resources::glade_border::GladeBorder::is_pos_inside();
    if (extraout_AL == '\0') {
      *local_90 = local_98;
      lVar4 = *(longlong *)(lVar6 + 0x30);
      uVar3 = *(undefined8 *)(lVar6 + 0x40);
      if (lVar4 == *(longlong *)(lVar6 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar6 + 0x28) + lVar4 * 8) = uVar3;
      *(longlong *)(lVar6 + 0x30) = lVar4 + 1;
      *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
      country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
      if (extraout_AL_02 == '\0') {
        return;
      }
      *local_c8 = local_94;
      lVar6 = *(longlong *)(lVar2 + 0x30);
      uVar3 = *(undefined8 *)(lVar2 + 0x40);
      if (lVar6 == *(longlong *)(lVar2 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar2 + 0x28) + lVar6 * 8) = uVar3;
    }
    else {
      country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
      if (extraout_AL_00 != '\0') {
        local_b0 = *(undefined8 *)*in_stack_00000040;
        country_core::resources::window::WindowSize::as_vec2();
        uVar1 = *(undefined4 *)(*in_stack_00000050 + 0xe8);
        local_2a8 = &local_b0;
        pcStack_2a0 = glam::f32::vec2::impl$93::fmt;
        local_190 = &DAT_143035150;
        uStack_188 = 1;
        local_180 = &local_2a8;
        uStack_178 = 1;
        local_170 = 0;
        alloc::fmt::format::format_inner();
        uVar3 = local_b0;
        std::alloc::__default_lib_allocator::__rust_alloc();
        local_90 = local_1e8;
        if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::alloc::handle_alloc_error();
        }
        *extraout_RAX = 1;
        extraout_RAX[1] = 1;
        extraout_RAX[2] = uVar3;
        *(undefined4 *)(extraout_RAX + 3) = uVar7;
        *(undefined4 *)((longlong)extraout_RAX + 0x1c) = uVar1;
        country_core::systems::ui::signifiers::SignifierFactory::new();
        local_288 = local_170;
        local_298 = local_180;
        uStack_290 = uStack_178;
        local_2a8 = (undefined8 *)local_190;
        pcStack_2a0 = (code *)uStack_188;
        country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
        if (CONCAT71(uStack_1ef,local_1f0) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
      }
      country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked();
      if (extraout_AL_01 == '\0') {
        return;
      }
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      if (fStack_a0 < 0.0) {
        fStack_a0 = (fStack_a0 / -2.5) * -1.2 + (1.0 - fStack_a0 / -2.5) * 0.0;
      }
      *(undefined4 *)in_stack_00000028[2] = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
      lVar2 = *in_stack_00000028;
      lVar6 = *(longlong *)(lVar2 + 0x30);
      uVar3 = *(undefined8 *)(lVar2 + 0x40);
      if (lVar6 == *(longlong *)(lVar2 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<tuple_<ref_<str_>,enum2_<country_core::resources::gui::SettingsMenuState>_>,alloc::alloc::Global>
                  ();
      }
      lVar4 = *(longlong *)(lVar2 + 0x28);
      *(undefined8 *)(lVar4 + lVar6 * 0x18) = uVar3;
      *(ulonglong *)(lVar4 + 8 + lVar6 * 0x18) = CONCAT44(fStack_a0,local_a4);
      *(undefined4 *)(lVar4 + 0x10 + lVar6 * 0x18) = local_9c;
    }
    *(longlong *)(lVar2 + 0x30) = lVar6 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
  }
  return;
}


// LOCALS:
[
 {
  "name": "in_stack_00000050",
  "type": "longlong *",
  "storage": "Stack[0x50]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000040",
  "type": "undefined8 *",
  "storage": "Stack[0x40]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000058",
  "type": "longlong",
  "storage": "Stack[0x58]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_AL_02",
  "type": "char",
  "storage": "AL:1",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_AL_01",
  "type": "char",
  "storage": "AL:1",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_1f0",
  "type": "undefined1",
  "storage": "Stack[-0x1f0]:1",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_RCX",
  "type": "undefined8 *",
  "storage": "RCX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_1e8",
  "type": "undefined4 *",
  "storage": "Stack[-0x1e8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_RAX",
  "type": "undefined8 *",
  "storage": "RAX:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "lVar6",
  "type": "longlong",
  "storage": "RSI:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_190",
  "type": "undefined *",
  "storage": "Stack[-0x190]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_180",
  "type": "undefined8 *",
  "storage": "Stack[-0x180]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcVar5",
  "type": "char *",
  "storage": "RDX:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "in_stack_00000028",
  "type": "longlong *",
  "storage": "Stack[0x28]:8",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_170",
  "type": "undefined8",
  "storage": "Stack[-0x170]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_160",
  "type": "int",
  "storage": "Stack[-0x160]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar7",
  "type": "undefined4",
  "storage": "XMM1_Da:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar2",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar1",
  "type": "undefined4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "lVar4",
  "type": "longlong",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_2a8",
  "type": "undefined8",
  "storage": "Stack[-0x2a8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uVar3",
  "type": "undefined8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_AL_00",
  "type": "char",
  "storage": "AL:1",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_298",
  "type": "undefined8 *",
  "storage": "Stack[-0x298]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "extraout_AL",
  "type": "char",
  "storage": "AL:1",
  "is_phantom": true,
  "in_decompiled_code": true
 },
 {
  "name": "local_288",
  "type": "undefined8",
  "storage": "Stack[-0x288]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_98",
  "type": "undefined4",
  "storage": "Stack[-0x98]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_94",
  "type": "undefined4",
  "storage": "Stack[-0x94]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_90",
  "type": "undefined4 *",
  "storage": "Stack[-0x90]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_81",
  "type": "undefined1",
  "storage": "Stack[-0x81]:1",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_80",
  "type": "undefined8",
  "storage": "Stack[-0x80]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_e0",
  "type": "int",
  "storage": "Stack[-0xe0]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_c8",
  "type": "undefined4 *",
  "storage": "Stack[-0xc8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_b8",
  "type": "char *",
  "storage": "Stack[-0xb8]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_b0",
  "type": "undefined8",
  "storage": "Stack[-0xb0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "fStack_a0",
  "type": "float",
  "storage": "Stack[-0xa0]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_a4",
  "type": "undefined4",
  "storage": "Stack[-0xa4]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "local_9c",
  "type": "undefined4",
  "storage": "Stack[-0x9c]:4",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_1ef",
  "type": "undefined7",
  "storage": "Stack[-0x1ef]:7",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_188",
  "type": "undefined8",
  "storage": "Stack[-0x188]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_178",
  "type": "undefined8",
  "storage": "Stack[-0x178]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "pcStack_2a0",
  "type": "undefined *",
  "storage": "Stack[-0x2a0]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 },
 {
  "name": "uStack_290",
  "type": "undefined8",
  "storage": "Stack[-0x290]:8",
  "is_phantom": false,
  "in_decompiled_code": true
 }
]