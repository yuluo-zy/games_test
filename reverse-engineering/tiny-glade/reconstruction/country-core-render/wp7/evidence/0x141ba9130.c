
void __cdecl system_water::draw_water::draw_water(void)

{
  uint uVar1;
  undefined4 uVar2;
  undefined4 *puVar3;
  longlong lVar4;
  longlong lVar5;
  undefined8 uVar6;
  bool bVar7;
  char extraout_AL;
  undefined1 extraout_AL_00;
  char extraout_AL_01;
  undefined8 *extraout_RAX;
  undefined8 *in_RCX;
  char *pcVar8;
  longlong in_R9;
  longlong lVar9;
  char cVar10;
  float extraout_XMM0_Da;
  float fVar11;
  longlong *in_stack_00000040;
  longlong in_stack_00000058;
  undefined1 *in_stack_00000060;
  longlong in_stack_00000078;
  undefined4 local_110;
  float fStack_10c;
  undefined4 *local_108;
  ulonglong *local_f8;
  undefined8 uStack_f0;
  undefined8 **local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  char *local_c8;
  longlong local_b8;
  undefined8 *local_b0;
  undefined4 local_a8;
  undefined4 local_a4;
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 **local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined4 *local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  if (*__imp__ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE17h6f2a49c135369f5aE == '\0') {
    uVar1 = *(uint *)*in_RCX;
    lVar9 = 0;
    if (uVar1 - 6 < 0xf) {
      lVar9 = (ulonglong)uVar1 - 5;
    }
    local_b8 = in_R9;
    if (lVar9 == 4) {
      if ((((uint *)*in_RCX)[1] & 3) != 0) {
        return;
      }
      ::utils::inverse_lerp();
      fVar11 = extraout_XMM0_Da * 4.0 + (1.0 - extraout_XMM0_Da);
      if (0x7f7fffff < (uint)ABS(fVar11)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      pcVar8 = "eraser";
      country_core::systems::ui::resources::ui_state::UiState::clearing_canvas();
      local_c8 = pcVar8;
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      country_core::resources::mouse_terrain_raycast::CameraCursorRay::intersect_with_xz_plane();
      uStack_f0 = glam::f32::vec3::impl$92::fmt;
      local_a0 = (ulonglong *)&DAT_142eebc40;
      uStack_98 = 1;
      local_90 = &local_f8;
      uStack_88 = 1;
      local_80 = 0;
      local_f8 = (ulonglong *)&local_b0;
      alloc::fmt::format::format_inner();
      uStack_98 = CONCAT44(uStack_98._4_4_,local_a8);
      local_a0 = local_b0;
      std::alloc::__default_lib_allocator::__rust_alloc();
      local_68 = local_108;
      if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX = 1;
      extraout_RAX[1] = 1;
      extraout_RAX[2] = local_a0;
      *(undefined4 *)(extraout_RAX + 3) = (undefined4)uStack_98;
      *(float *)((longlong)extraout_RAX + 0x1c) = fVar11 * 0.5;
      country_core::systems::ui::signifiers::SignifierFactory::new();
      local_d8 = local_80;
      local_e8 = local_90;
      uStack_e0 = uStack_88;
      local_f8 = local_a0;
      uStack_f0 = (code *)uStack_98;
      country_core::systems::ui::signifiers::signifier_create::UiSignifierStream::write();
      if (CONCAT44(fStack_10c,local_110) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      bVar7 = true;
      cVar10 = '\0';
    }
    else {
      if (lVar9 != 10) {
        return;
      }
      fVar11 = *(float *)(*(longlong *)(in_stack_00000078 + 0x20) + 4);
      if (0x7f7fffff < (uint)ABS(fVar11)) {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      lVar9 = *(longlong *)(in_stack_00000058 + 0x20);
      puVar3 = *(undefined4 **)(in_stack_00000058 + 0x30);
      uVar2 = *(undefined4 *)(in_stack_00000058 + 0x3c);
      lVar4 = *(longlong *)(in_stack_00000058 + 0x40);
      local_68 = *(undefined4 **)(in_stack_00000058 + 0x50);
      local_a4 = *(undefined4 *)(in_stack_00000058 + 0x5c);
      pcVar8 = "watereraser_water_";
      country_core::systems::ui::resources::ui_state::UiState::clearing_canvas();
      local_c8 = pcVar8;
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
      ::utils::geometry::circle::Circle2d::new();
      local_a0 = (ulonglong *)((ulonglong)local_a0._4_4_ << 0x20);
      country_core::resources::glade_border::GladeBorder::is_shape_inside();
      cVar10 = '\x01';
      if (extraout_AL == '\0') {
        *puVar3 = uVar2;
        lVar5 = *(longlong *)(lVar9 + 0x30);
        uVar6 = *(undefined8 *)(lVar9 + 0x40);
        if (lVar5 == *(longlong *)(lVar9 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(lVar9 + 0x28) + lVar5 * 8) = uVar6;
        *(longlong *)(lVar9 + 0x30) = lVar5 + 1;
        *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
        country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
        local_a0 = (ulonglong *)CONCAT71(local_a0._1_7_,extraout_AL_00);
        country_core::systems::ui::resources::ui_state::DragState::is_some();
        if (extraout_AL_01 != '\0') {
          *local_68 = local_a4;
          lVar9 = *(longlong *)(lVar4 + 0x30);
          uVar6 = *(undefined8 *)(lVar4 + 0x40);
          if (lVar9 == *(longlong *)(lVar4 + 0x20)) {
            alloc::raw_vec::RawVec::
            grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                      ();
          }
          *(undefined8 *)(*(longlong *)(lVar4 + 0x28) + lVar9 * 8) = uVar6;
          *(longlong *)(lVar4 + 0x30) = lVar9 + 1;
          *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
        }
        bVar7 = false;
      }
      else {
        bVar7 = true;
      }
    }
    country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
    country_core::systems::paths::draw_path::DrawState::update();
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*in_stack_00000060) {
    case 0:
      goto LAB_141ba9640;
    case 1:
      local_f8 = (ulonglong *)((ulonglong)local_f8 & 0xffffffff00000000);
      uStack_f0._0_5_ = CONCAT14(cVar10,fVar11);
      break;
    case 2:
      if (!bVar7) {
        return;
      }
      if (cVar10 == '\0') {
        local_f8 = (ulonglong *)CONCAT44(local_f8._4_4_,2);
      }
      else {
        country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
        ::utils::geometry::circle::Circle2d::new();
        country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position();
        local_80 = 0;
        local_90 = (undefined8 **)((ulonglong)local_90 & 0xffffffff);
        uStack_88 = CONCAT44(uStack_88._4_4_,fStack_10c + 1.5);
        local_a0 = (ulonglong *)CONCAT62(local_a0._2_6_,0x100);
        bevy_ecs::event::writer::EventWriter::
        write<enum2_<country_core::systems::ui_clutter::decoclutter_show_pins::ShowPinnedDecoClutter>_>
                  ();
        local_f8 = (ulonglong *)CONCAT44(local_f8._4_4_,2);
      }
      break;
    case 3:
      local_f8 = (ulonglong *)CONCAT44(local_f8._4_4_,3);
    }
    *(undefined4 *)in_stack_00000040[2] = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
    lVar9 = *in_stack_00000040;
    lVar4 = *(longlong *)(lVar9 + 0x30);
    uVar6 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar4 == *(longlong *)(lVar9 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar9 + 0x28) + lVar4 * 8) = uVar6;
    *(longlong *)(lVar9 + 0x30) = lVar4 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    country_core::systems::terrain::delayed_terrain_changed_msg::ScheduledTerrainReadback::
    on_terrain_modified();
    uStack_98 = CONCAT44(uStack_98._4_4_,local_a8);
    local_a0 = local_b0;
    **(undefined4 **)(local_b8 + 0x10) = *(undefined4 *)(local_b8 + 0x1c);
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    country_core::systems::world_raster::
    draw_raster<country_core::resources::world_raster_defs::WaterRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,system_water::draw_water::draw_water::closure_env_1,system_water::draw_water::draw_water::closure_env_2,system_water::draw_water::draw_water::closure_env_3>
              ();
  }
LAB_141ba9640:
  return;
}

