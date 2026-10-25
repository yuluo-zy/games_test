
void __cdecl country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast::position(void)

{
  undefined8 *in_RCX;
  byte *in_RDX;
  
  if ((*in_RDX & 1) != 0) {
    *(undefined4 *)(in_RCX + 1) = *(undefined4 *)(in_RDX + 0xc);
    *in_RCX = *(undefined8 *)(in_RDX + 4);
    return;
  }
  *(undefined4 *)(in_RCX + 1) = 0;
  *in_RCX = 0;
  return;
}


