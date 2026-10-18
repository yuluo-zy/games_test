
void __cdecl country_core::systems::garden::draw_garden::draw_garden(void)

{
  undefined8 *puVar1;
  ulonglong *puVar2;
  uint uVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  byte *pbVar8;
  longlong lVar9;
  undefined8 uVar10;
  longlong lVar11;
  bool bVar12;
  char extraout_AL;
  char cVar13;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  byte bVar14;
  undefined8 *in_RCX;
  longlong lVar15;
  byte bVar16;
  longlong lVar17;
  char *pcVar18;
  longlong in_R9;
  float extraout_XMM0_Da;
  float fVar19;
  undefined8 *in_stack_00000040;
  longlong in_stack_00000048;
  char *in_stack_00000050;
  longlong in_stack_00000058;
  longlong *in_stack_00000060;
  undefined4 uStack_b4;
  undefined8 uStack_b0;
  uint local_a8;
  undefined4 uStack_9c;
  undefined8 uStack_90;
  uint uStack_84;
  
  uVar3 = *(uint *)*in_RCX;
  lVar17 = 0;
  if (uVar3 - 6 < 0xf) {
    lVar17 = (ulonglong)uVar3 - 5;
  }
  if (lVar17 != 0xc) {
    if (lVar17 != 4) {
      return;
    }
    cVar13 = (char)((uint *)*in_RCX)[1];
    if ((cVar13 != '\0') && (cVar13 != '\x05')) {
      return;
    }
    ::utils::inverse_lerp();
    if (0x7f7fffff < (uint)ABS(extraout_XMM0_Da * 4.0 + (1.0 - extraout_XMM0_Da))) {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    pcVar18 = "eraser";
    systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound();
    cVar13 = '\x01';
    bVar12 = false;
    bVar14 = *(byte *)(extraout_RAX + 0x18);
    lVar17 = extraout_RAX;
    goto joined_r0x000140ae4741;
  }
  if (0x7f7fffff < (*(uint *)(*(longlong *)(in_stack_00000058 + 0x20) + 8) & 0x7fffffff)) {
                    // WARNING: Subroutine does not return
    core::panicking::panic();
  }
  lVar9 = *(longlong *)(in_stack_00000048 + 0x20);
  puVar6 = *(undefined4 **)(in_stack_00000048 + 0x30);
  uVar4 = *(undefined4 *)(in_stack_00000048 + 0x3c);
  lVar11 = *(longlong *)(in_stack_00000048 + 0x40);
  puVar7 = *(undefined4 **)(in_stack_00000048 + 0x50);
  uVar5 = *(undefined4 *)(in_stack_00000048 + 0x5c);
  pcVar18 = &DAT_142ae2ea0;
  systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound();
  ::utils::geometry::circle::Circle2d::new();
  resources::glade_border::GladeBorder::is_shape_inside();
  lVar17 = extraout_RAX_00;
  if (extraout_AL == '\0') {
    *puVar6 = uVar4;
    lVar15 = *(longlong *)(lVar9 + 0x30);
    uVar10 = *(undefined8 *)(lVar9 + 0x40);
    if (lVar15 == *(longlong *)(lVar9 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    bVar12 = true;
    *(undefined8 *)(*(longlong *)(lVar9 + 0x28) + lVar15 * 8) = uVar10;
    *(longlong *)(lVar9 + 0x30) = lVar15 + 1;
    *(longlong *)(lVar9 + 0x40) = *(longlong *)(lVar9 + 0x40) + 1;
    if (*(byte *)(extraout_RAX_00 + 0x18) < 2) {
      cVar13 = '\0';
      goto LAB_140ae48a1;
    }
    if (*(char **)(extraout_RAX_00 + 0x38) != pcVar18) {
      cVar13 = '\0';
      bVar14 = *(byte *)(extraout_RAX_00 + 0x18);
      goto joined_r0x000140ae4741;
    }
    if (*(char *)(extraout_RAX_00 + 0x19) != '\x02') {
      *puVar7 = uVar5;
      lVar9 = *(longlong *)(lVar11 + 0x30);
      uVar10 = *(undefined8 *)(lVar11 + 0x40);
      if (lVar9 == *(longlong *)(lVar11 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      bVar12 = true;
      *(undefined8 *)(*(longlong *)(lVar11 + 0x28) + lVar9 * 8) = uVar10;
      *(longlong *)(lVar11 + 0x30) = lVar9 + 1;
      *(longlong *)(lVar11 + 0x40) = *(longlong *)(lVar11 + 0x40) + 1;
      cVar13 = '\0';
      bVar14 = *(byte *)(extraout_RAX_00 + 0x18);
      goto joined_r0x000140ae4741;
    }
    cVar13 = '\0';
    bVar14 = *(byte *)(extraout_RAX_00 + 0x18);
  }
  else {
    cVar13 = '\x01';
LAB_140ae48a1:
    bVar14 = *(byte *)(extraout_RAX_00 + 0x18);
  }
  bVar12 = true;
joined_r0x000140ae4741:
  if ((bVar14 < 2) || (*(char **)(lVar17 + 0x38) != pcVar18)) {
    bVar14 = 0;
  }
  else {
    bVar14 = *(char *)(lVar17 + 0x19) - 2;
  }
  bVar16 = 3;
  if (bVar14 < 3) {
    bVar16 = bVar14;
  }
  if (bVar16 - 1 < 2) {
    if ((byte)(*in_stack_00000050 - 1U) < 2) {
      *in_stack_00000050 = '\x02';
      if (cVar13 == '\0') {
        return;
      }
      if (bVar12) {
        pbVar8 = (byte *)*in_stack_00000040;
        ::utils::geometry::circle::Circle2d::new();
        if ((*pbVar8 & 1) == 0) {
          fVar19 = 1.5;
        }
        else {
          fVar19 = *(float *)(pbVar8 + 8) + 1.5;
        }
        *(undefined4 *)in_stack_00000060[2] = *(undefined4 *)((longlong)in_stack_00000060 + 0x1c);
        lVar17 = *in_stack_00000060;
        lVar9 = *(longlong *)(lVar17 + 0x30);
        uVar10 = *(undefined8 *)(lVar17 + 0x40);
        if (lVar9 == *(longlong *)(lVar17 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
        }
        lVar11 = *(longlong *)(lVar17 + 0x28);
        lVar15 = lVar9 * 0x40;
        *(ulonglong *)(lVar11 + 0x30 + lVar15) = (ulonglong)uStack_84 << 0x20;
        puVar1 = (undefined8 *)(lVar11 + 0x20 + lVar15);
        *puVar1 = 0;
        puVar1[1] = uStack_90;
        puVar2 = (ulonglong *)(lVar11 + 0x10 + lVar15);
        *puVar2 = (ulonglong)local_a8;
        puVar2[1] = CONCAT44(uStack_9c,fVar19);
        *(undefined8 *)(lVar11 + lVar15) = CONCAT44(uStack_b4,0x100);
        ((undefined8 *)(lVar11 + lVar15))[1] = uStack_b0;
        *(undefined8 *)(lVar11 + 0x38 + lVar15) = uVar10;
        *(longlong *)(lVar17 + 0x30) = lVar9 + 1;
        *(longlong *)(lVar17 + 0x40) = *(longlong *)(lVar17 + 0x40) + 1;
      }
    }
    else {
      *in_stack_00000050 = cVar13;
      if (cVar13 == '\0') {
        return;
      }
    }
  }
  else {
    if ((bVar16 == 0) || (1 < (byte)(*in_stack_00000050 - 1U))) {
      *in_stack_00000050 = '\0';
      return;
    }
    *in_stack_00000050 = '\x03';
  }
  **(undefined4 **)(in_R9 + 0x10) = *(undefined4 *)(in_R9 + 0x1c);
  world_raster::
  draw_raster<country_core::resources::world_raster_defs::GardenRasterStagingSsbo,country_core::resources::world_raster::WorldRaster<512>,country_core::systems::garden::draw_garden::draw_garden::closure_env_0,country_core::systems::garden::draw_garden::draw_garden::closure_env_1,country_core::systems::garden::draw_garden::draw_garden::closure_env_2>
            ();
  return;
}

