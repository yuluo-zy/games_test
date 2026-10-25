// addr=0x14089dc30 name=rotation
// signature=void __cdecl rotation(void)


void __cdecl country_core::systems::ui::signifiers::SignifierEntityBuilder::rotation(void)

{
  undefined8 uVar1;
  void *in_RCX;
  undefined8 *in_RDX;
  undefined8 *in_R8;
  
  uVar1 = in_R8[1];
  *in_RDX = *in_R8;
  in_RDX[1] = uVar1;
  memcpy(in_RCX,in_RDX,0x90);
  return;
}


