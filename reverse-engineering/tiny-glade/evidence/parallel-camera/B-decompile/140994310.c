
void __cdecl country_core::systems::camera_rig::walk::WalkCamera::rotation(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  uVar1 = *(undefined8 *)(in_RDX + 0x20);
  *in_RCX = *(undefined8 *)(in_RDX + 0x18);
  in_RCX[1] = uVar1;
  return;
}

