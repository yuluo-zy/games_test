// position 0x140ad51a0 size=22
// signature: void __cdecl position(void)


void __cdecl country_core::resources::placement_data::PlacementDataSsbo::position(void)

{
  undefined4 uVar1;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  uVar1 = *(undefined4 *)(in_RDX + 1);
  *in_RCX = *in_RDX;
  *(undefined4 *)(in_RCX + 1) = uVar1;
  return;
}

