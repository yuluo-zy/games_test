// addr=0x14090df90 name=upgrade
// signature=void __cdecl upgrade(void)


void __cdecl country_core::game_config::GameConfig::upgrade(void)

{
  float fVar1;
  int iVar2;
  longlong in_RCX;
  
  if ((*(byte *)(in_RCX + 0x88) & 1) == 0) {
    fVar1 = *(float *)(in_RCX + 0x84);
    iVar2 = *(int *)(in_RCX + 0x80);
    *(undefined4 *)(in_RCX + 0x80) = 0;
    if ((iVar2 != 0) && (((fVar1 != 1.0 || (NAN(fVar1))) && (0.125 < fVar1)))) {
      *(undefined4 *)(in_RCX + 0x88) = 1;
      *(float *)(in_RCX + 0x8c) = fVar1;
      return;
    }
  }
  return;
}


