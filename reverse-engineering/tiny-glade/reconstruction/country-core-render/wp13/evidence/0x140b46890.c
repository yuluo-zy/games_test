// name: is_halloween_any size: 41
// void __cdecl is_halloween_any(void)

void __cdecl country_core::resources::session::SessionMeta::is_halloween_any(void)

{
  byte *pbVar1;
  longlong in_RCX;
  longlong lVar2;
  
  lVar2 = 0;
  do {
    if (*(longlong *)(in_RCX + 0x10) == lVar2) {
      return;
    }
    pbVar1 = (byte *)(*(longlong *)(in_RCX + 8) + lVar2);
    lVar2 = lVar2 + 1;
  } while (1 < *pbVar1);
  return;
}


