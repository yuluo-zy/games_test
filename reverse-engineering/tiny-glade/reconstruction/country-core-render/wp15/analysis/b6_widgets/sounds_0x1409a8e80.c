// addr=0x1409a8e80 name=sounds
// signature=void __cdecl sounds(void)


void __cdecl country_core::systems::ui::resources::widget_params::WidgetParams::sounds(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  void *in_RCX;
  void *in_RDX;
  undefined8 *in_R8;
  
  uVar1 = in_R8[0xb];
  *(undefined8 *)((longlong)in_RDX + 0x68) = in_R8[10];
  *(undefined8 *)((longlong)in_RDX + 0x70) = uVar1;
  uVar1 = in_R8[9];
  *(undefined8 *)((longlong)in_RDX + 0x58) = in_R8[8];
  *(undefined8 *)((longlong)in_RDX + 0x60) = uVar1;
  uVar1 = *in_R8;
  uVar2 = in_R8[1];
  uVar3 = in_R8[2];
  uVar4 = in_R8[3];
  uVar5 = in_R8[4];
  uVar6 = in_R8[5];
  uVar7 = in_R8[7];
  *(undefined8 *)((longlong)in_RDX + 0x48) = in_R8[6];
  *(undefined8 *)((longlong)in_RDX + 0x50) = uVar7;
  *(undefined8 *)((longlong)in_RDX + 0x38) = uVar5;
  *(undefined8 *)((longlong)in_RDX + 0x40) = uVar6;
  *(undefined8 *)((longlong)in_RDX + 0x28) = uVar3;
  *(undefined8 *)((longlong)in_RDX + 0x30) = uVar4;
  *(undefined8 *)((longlong)in_RDX + 0x18) = uVar1;
  *(undefined8 *)((longlong)in_RDX + 0x20) = uVar2;
  memcpy(in_RCX,in_RDX,0xb0);
  return;
}


