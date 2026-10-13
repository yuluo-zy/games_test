// name: is_xmas2024 size: 41
// void __cdecl is_xmas2024(void)

void __cdecl country_core::resources::session::SessionMeta::is_xmas2024(void)

{
  char *pcVar1;
  longlong in_RCX;
  longlong lVar2;
  
  lVar2 = 0;
  do {
    if (*(longlong *)(in_RCX + 0x10) == lVar2) {
      return;
    }
    pcVar1 = (char *)(*(longlong *)(in_RCX + 8) + lVar2);
    lVar2 = lVar2 + 1;
  } while (*pcVar1 != '\x02');
  return;
}


