// no_history_recording 0x1409a8f10

void __cdecl
country_core::systems::ui::resources::widget_params::WidgetParams::no_history_recording(void)

{
  void *in_RCX;
  void *in_RDX;
  
  *(byte *)((longlong)in_RDX + 0xa9) = *(byte *)((longlong)in_RDX + 0xa9) & 0xfe;
  memcpy(in_RCX,in_RDX,0xb0);
  return;
}

