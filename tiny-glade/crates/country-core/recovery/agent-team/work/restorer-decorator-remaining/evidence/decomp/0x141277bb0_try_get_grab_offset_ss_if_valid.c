
void __cdecl
system_decorator::ui_systems::calculate_decorator_grab_offset::try_get_grab_offset_ss_if_valid(void)

{
  longlong in_R8;
  longlong lVar1;
  
  lVar1 = 0;
  if ((byte)(*(byte *)(in_R8 + 0x22) - 2) < 3) {
    lVar1 = (ulonglong)*(byte *)(in_R8 + 0x22) - 1;
  }
                    // WARNING: Could not emulate address calculation at 0x000141277c08
                    // WARNING: Treating indirect jump as call
  (*(code *)((longlong)&DAT_142c23990 + (longlong)(int)(&DAT_142c23990)[lVar1]))
            ((code *)((longlong)&DAT_142c23990 + (longlong)(int)(&DAT_142c23990)[lVar1]));
  return;
}

