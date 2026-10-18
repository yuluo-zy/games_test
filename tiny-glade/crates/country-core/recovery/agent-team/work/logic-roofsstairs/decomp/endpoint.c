
void __cdecl country_core::resources::stairs::stairs_state::StairsState::endpoint(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  in_RCX[2] = *(undefined8 *)(in_RDX + 0x20);
  uVar1 = *(undefined8 *)(in_RDX + 0x18);
  *in_RCX = *(undefined8 *)(in_RDX + 0x10);
  in_RCX[1] = uVar1;
  return;
}

