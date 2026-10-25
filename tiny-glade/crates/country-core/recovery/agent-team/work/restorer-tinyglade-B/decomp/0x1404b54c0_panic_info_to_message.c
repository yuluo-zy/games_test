// panic_info_to_message @ 0x1404b54c0
// sig: void __cdecl panic_info_to_message(void)


void __cdecl tiny_glade::panic_reporter::sentry_reporter::panic_info_to_message(void)

{
  undefined8 uVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  uVar1 = *in_RCX;
  lVar2 = in_RCX[1];
  lVar3 = (**(code **)(lVar2 + 0x18))(uVar1);
  if (in_RDX != 0x63eb502cd6cb5d6d || lVar3 != -0x4674e48ea859be88) {
    (**(code **)(lVar2 + 0x18))(uVar1);
  }
  return;
}

