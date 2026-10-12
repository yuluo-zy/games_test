
void __cdecl country_core::resources::walls::public_wall_state::PublicWallState::is_rectangle(void)

{
  undefined8 uVar1;
  uint *in_RCX;
  longlong in_RDX;
  uint uVar2;
  bool bVar3;
  
  uVar2 = 0;
  if (*(uint *)(in_RDX + 600) < 2) {
    uVar2 = *(uint *)(in_RDX + 600);
  }
  bVar3 = (uVar2 & 1) != 0;
  if (bVar3) {
    *(undefined8 *)(in_RCX + 5) = *(undefined8 *)(in_RDX + 0x26c);
    uVar1 = *(undefined8 *)(in_RDX + 0x264);
    *(undefined8 *)(in_RCX + 1) = *(undefined8 *)(in_RDX + 0x25c);
    *(undefined8 *)(in_RCX + 3) = uVar1;
  }
  *in_RCX = (uint)bVar3;
  return;
}

