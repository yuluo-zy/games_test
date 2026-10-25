
void __cdecl country_core::resources::roofs::roof_ridge::RoofRidgeDims::from(void)

{
  longlong in_RCX;
  
  if (*(int *)(in_RCX + 8) != 1) {
    return;
  }
  if (*(char *)(in_RCX + 0x3c) != '\0') {
    return;
  }
  return;
}

