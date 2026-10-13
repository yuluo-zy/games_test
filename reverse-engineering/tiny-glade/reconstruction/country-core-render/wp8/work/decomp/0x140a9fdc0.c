
void __cdecl country_core::systems::terrain::ui_draw_terrain::ui_draw_terrain(void)

{
  undefined4 uVar1;
  longlong lVar2;
  undefined4 *puVar3;
  longlong lVar4;
  undefined4 *puVar5;
  byte *pbVar6;
  longlong lVar7;
  undefined8 uVar8;
  char cVar9;
  uint extraout_EAX;
  uint uVar10;
  uint uVar11;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  char *pcVar12;
  float extraout_XMM0_Da;
  undefined4 uVar13;
  undefined8 in_stack_00000038;
  longlong in_stack_00000060;
  char *in_stack_00000068;
  undefined4 *in_stack_00000070;
  
  if (*(int *)*in_RCX != 10) {
    return;
  }
  lVar2 = *(longlong *)(in_stack_00000060 + 0x20);
  puVar3 = *(undefined4 **)(in_stack_00000060 + 0x30);
  uVar13 = *(undefined4 *)(in_stack_00000060 + 0x3c);
  lVar4 = *(longlong *)(in_stack_00000060 + 0x40);
  puVar5 = *(undefined4 **)(in_stack_00000060 + 0x50);
  uVar1 = *(undefined4 *)(in_stack_00000060 + 0x5c);
  pbVar6 = (byte *)*in_RDX;
  ::utils::float_grid::impl$0::
  bilinear_sample_at_world_xz<country_core::resources::world_raster::RasterBufferRef<512>_>();
  if ((0.0 < extraout_XMM0_Da) && (((*pbVar6 & 1) == 0 || (*(float *)(pbVar6 + 8) <= 0.5)))) {
    systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound();
  }
  pcVar12 = "terrain_draw";
  systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound();
  ::utils::geometry::circle::Circle2d::new();
  resources::glade_border::GladeBorder::is_shape_inside();
  if ((char)extraout_EAX == '\0') {
    *puVar3 = uVar13;
    lVar7 = *(longlong *)(lVar2 + 0x30);
    uVar8 = *(undefined8 *)(lVar2 + 0x40);
    if (lVar7 == *(longlong *)(lVar2 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar2 + 0x28) + lVar7 * 8) = uVar8;
    *(longlong *)(lVar2 + 0x30) = lVar7 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
    if (((1 < *(byte *)(extraout_RAX + 0x18)) && (*(char **)(extraout_RAX + 0x38) == pcVar12)) &&
       (*(char *)(extraout_RAX + 0x19) != '\x02')) {
      *puVar5 = uVar1;
      lVar2 = *(longlong *)(lVar4 + 0x30);
      uVar8 = *(undefined8 *)(lVar4 + 0x40);
      if (lVar2 == *(longlong *)(lVar4 + 0x20)) {
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(lVar4 + 0x28) + lVar2 * 8) = uVar8;
      *(longlong *)(lVar4 + 0x30) = lVar2 + 1;
      *(longlong *)(lVar4 + 0x40) = *(longlong *)(lVar4 + 0x40) + 1;
    }
  }
  cVar9 = '\x02';
  if ((1 < *(byte *)(extraout_RAX + 0x18)) && (*(char **)(extraout_RAX + 0x38) == pcVar12)) {
    cVar9 = *(char *)(extraout_RAX + 0x19);
  }
  uVar10 = 3;
  if ((byte)(cVar9 - 2U) < 3) {
    uVar10 = (uint)(byte)(cVar9 - 2U);
  }
  if (uVar10 - 1 < 2) {
    uVar11 = 2;
    if (1 < (byte)(*in_stack_00000068 - 1U)) {
      uVar11 = extraout_EAX & 0xff;
    }
  }
  else {
    uVar11 = 0;
    if (uVar10 != 0) {
      uVar11 = (uint)((byte)(*in_stack_00000068 - 1U) < 2) * 3;
    }
  }
  *in_stack_00000068 = (char)uVar11;
  if (((1 < *(byte *)(extraout_RAX + 0x18)) && (*(char **)(extraout_RAX + 0x38) == pcVar12)) &&
     (*(char *)(extraout_RAX + 0x19) == '\x03')) {
    uVar13 = 0;
    if ((*pbVar6 & 1) != 0) {
      uVar13 = *(undefined4 *)(pbVar6 + 8);
    }
    *in_stack_00000070 = uVar13;
  }
                    // WARNING: Could not recover jumptable at 0x000140aa00e7. Too many branches
                    // WARNING: Treating indirect jump as call
  (*(code *)(&DAT_142adc498 + *(int *)(&DAT_142adc498 + (ulonglong)uVar11 * 4)))(in_stack_00000038);
  return;
}


