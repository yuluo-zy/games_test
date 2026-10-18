// allow_negative_distance 0x1409a8f40

void __cdecl
country_core::systems::ui::resources::widget_params::WidgetParams::allow_negative_distance(void)

{
  void *in_RCX;
  void *in_RDX;
  char in_R8B;
  
  *(byte *)((longlong)in_RDX + 0xa9) = in_R8B << 2 | *(byte *)((longlong)in_RDX + 0xa9) & 0xfb;
  memcpy(in_RCX,in_RDX,0xb0);
  return;
}

