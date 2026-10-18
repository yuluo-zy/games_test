// new 0x1409a8e10

void __cdecl country_core::systems::ui::resources::widget_params::WidgetParams::new(void)

{
  undefined8 uVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  undefined8 *in_R8;
  
  uVar1 = *in_RDX;
  uVar2 = in_RDX[1];
  uVar3 = in_RDX[3];
  in_RCX[0x13] = in_RDX[2];
  in_RCX[0x14] = uVar3;
  in_RCX[0x11] = uVar1;
  in_RCX[0x12] = uVar2;
  uVar1 = in_R8[1];
  in_RCX[0xf] = *in_R8;
  in_RCX[0x10] = uVar1;
  *(undefined4 *)(in_RCX + 3) = 1;
  in_RCX[4] = 0;
  *(undefined4 *)(in_RCX + 6) = 2;
  *(undefined4 *)(in_RCX + 9) = 3;
  *(undefined4 *)(in_RCX + 0xc) = 3;
  *in_RCX = 0;
  in_RCX[1] = 8;
  in_RCX[2] = 0;
  *(undefined2 *)(in_RCX + 0x15) = 0x101;
  return;
}

