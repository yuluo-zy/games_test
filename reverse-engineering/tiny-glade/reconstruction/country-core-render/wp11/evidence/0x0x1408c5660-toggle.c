// 0x1408c5660 toggle
// sig: void __cdecl toggle(void)

void __cdecl country_core::resources::walls::inner_wall_state::FreehandHeightType::toggle(void)

{
  byte *in_RCX;
  
  *in_RCX = *in_RCX ^ 1;
  return;
}

