// ADDR 0x14080f470
// SIG void __cdecl is_decorator_location_valid(void)
// ORIG system::is_decorator_location_valid


void __cdecl system_decorator::cull_oob_decorators::is_decorator_location_valid(void)

{
  longlong lVar1;
  longlong in_R8;
  
  lVar1 = 0;
  if ((byte)(*(byte *)(in_R8 + 0x22) - 2) < 3) {
    lVar1 = (ulonglong)*(byte *)(in_R8 + 0x22) - 1;
  }
                    // WARNING: Could not emulate address calculation at 0x00014080f52b
                    // WARNING: Treating indirect jump as call
  (*(code *)((longlong)&DAT_142a8eab0 + (longlong)(int)(&DAT_142a8eab0)[lVar1]))
            ((code *)((longlong)&DAT_142a8eab0 + (longlong)(int)(&DAT_142a8eab0)[lVar1]));
  return;
}


