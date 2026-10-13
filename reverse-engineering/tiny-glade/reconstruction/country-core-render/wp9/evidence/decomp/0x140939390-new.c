// signature: void __cdecl new(void)

void __cdecl country_core::resources::roofs::roof_tile_sim::RoofTileSimulation::new(void)

{
  undefined4 *in_RCX;
  undefined4 in_XMM1_Da;
  
  *(undefined8 *)((longlong)in_RCX + 0xd) = 0;
  *(undefined8 *)(in_RCX + 2) = 0;
  *in_RCX = 0;
  in_RCX[1] = in_XMM1_Da;
  return;
}

