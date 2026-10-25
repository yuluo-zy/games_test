
void __cdecl country_core::resources::roofs::roof_shape::Roof::rotation_yaw_radians(void)

{
  longlong in_RCX;
  
  if ((*(byte *)(in_RCX + 8) & 1) != 0) {
    atan2f(-*(float *)(in_RCX + 0x10),*(float *)(in_RCX + 0xc));
    return;
  }
  ::utils::circular_f32::CircularF32::as_f32();
  return;
}

