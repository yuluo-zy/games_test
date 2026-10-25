// addr=0x14092e680 name=is_active
// signature=void __cdecl is_active(void)


void __cdecl country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active(void)

{
  longlong *in_RCX;
  
  if ((*(char *)(*in_RCX + 0x18) != '\0') && (*(longlong *)(*in_RCX + 0x38) == in_RCX[1])) {
    return;
  }
  return;
}


