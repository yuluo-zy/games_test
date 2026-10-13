// name: load size: 55
// void __cdecl load(void)

void __cdecl country_core::resources::session::NextSession::load(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  uVar1 = in_RDX[1];
  in_RCX[4] = *in_RDX;
  in_RCX[5] = uVar1;
  in_RCX[3] = &DAT_8000000000000002;
  *in_RCX = 0;
  in_RCX[1] = 8;
  in_RCX[2] = 0;
  *(undefined1 *)(in_RCX + 0x13) = 0;
  return;
}


