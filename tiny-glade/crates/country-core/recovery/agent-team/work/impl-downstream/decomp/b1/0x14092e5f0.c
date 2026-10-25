
void __cdecl country_core::systems::ui::resources::ui_state::UiWidgetContext::clicked(void)

{
  byte bVar1;
  longlong *in_RCX;
  
  if (((1 < *(byte *)(*in_RCX + 0x18)) && (bVar1 = *(byte *)(*in_RCX + 0x19), bVar1 < 2)) &&
     ((bVar1 & 1) != 0)) {
    return;
  }
  return;
}

