// name: debug_set_history_start size: 4
// void __cdecl debug_set_history_start(void)

void __cdecl country_core::resources::history::History::debug_set_history_start(void)

{
  longlong in_RCX;
  undefined4 in_EDX;
  
  *(undefined4 *)(in_RCX + 0x6c) = in_EDX;
  return;
}


