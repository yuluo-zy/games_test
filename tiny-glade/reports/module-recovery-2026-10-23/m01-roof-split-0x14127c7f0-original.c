
void __cdecl
system_decorator::ui_systems::handle_splitting_roof_decorators::
handle_moving_out_roof_decorator_part::closure_1(void)

{
  float fVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined1 extraout_AL;
  undefined8 *in_RCX;
  longlong *in_RDX;
  ulonglong in_R8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float in_XMM3_Da;
  float fVar4;
  
  if ((in_R8 & 1) == 0) {
    *in_RCX = 3;
  }
  else {
    if ((*(byte *)(*in_RDX + 8) & 1) == 0) {
      country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
      country_core::systems::wall::valid_enclosure::ShapeParameters::perimeter();
      fVar4 = in_XMM3_Da / extraout_XMM0_Da_00;
    }
    else {
      country_core::resources::roofs::roof_shape::Roof::get_roof_shape_from_pos_ws_clamped();
      country_core::systems::wall::valid_enclosure::ShapeParameters::as_rectangle();
      ::utils::geometry::rectangle::Rectangle2d::get_closest_edge_id();
      ::utils::geometry::rectangle::Rectangle2d::edge_length();
      fVar4 = (in_XMM3_Da / extraout_XMM0_Da) * 0.25;
    }
    fVar4 = fVar4 + *(float *)in_RDX[2];
    fVar1 = ((float *)in_RDX[2])[1];
    if (fVar4 <= 1.0) {
      if (fVar4 < 0.0) {
        fVar4 = fVar4 + 1.0;
      }
    }
    else {
      fVar4 = fVar4 + -1.0;
    }
    uVar2 = *(undefined8 *)in_RDX[3];
    uVar3 = ((undefined8 *)in_RDX[3])[1];
    bevy_app::plugin::Plugin::ready<country_core::systems::main_camera::MainCameraPlugin>();
    *in_RCX = uVar2;
    in_RCX[1] = uVar3;
    *(float *)(in_RCX + 2) = fVar4;
    *(float *)((longlong)in_RCX + 0x14) = fVar1;
    *(undefined4 *)(in_RCX + 3) = 0;
    *(undefined1 *)((longlong)in_RCX + 0x32) = 2;
    *(undefined1 *)(in_RCX + 7) = extraout_AL;
  }
  return;
}


