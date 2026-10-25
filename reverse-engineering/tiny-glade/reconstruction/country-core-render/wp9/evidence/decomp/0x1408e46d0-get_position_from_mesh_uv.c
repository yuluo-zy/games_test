// signature: void __cdecl get_position_from_mesh_uv(void)

void __cdecl
country_core::resources::roofs::roof_trigger_volume::RoofTriggerVolume::get_position_from_mesh_uv
          (void)

{
  uint *in_RCX;
  float in_stack_00000028;
  int local_18;
  undefined8 local_14;
  float local_c;
  
  ::utils::geometry::position_from_mesh_uvs::get_position_from_mesh_uv();
  if (local_18 == 1) {
    *(ulonglong *)(in_RCX + 1) =
         CONCAT44(in_stack_00000028 + (float)((ulonglong)local_14 >> 0x20),
                  in_stack_00000028 * 0.0 + (float)local_14);
    in_RCX[3] = (uint)(local_c + in_stack_00000028 * 0.0);
  }
  *in_RCX = (uint)(local_18 == 1);
  return;
}

