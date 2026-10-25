// addr=0x14092e640 name=intersecting
// signature=void __cdecl intersecting(void)


void __cdecl country_core::systems::ui::resources::ui_state::UiWidgetContext::intersecting(void)

{
  longlong lVar1;
  undefined8 uVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 *in_RCX;
  longlong *in_RDX;
  
  lVar1 = *in_RDX;
  if ((*(char *)(lVar1 + 0x18) != '\0') && (*(longlong *)(lVar1 + 0x38) == in_RDX[1])) {
    in_RCX[6] = *(undefined8 *)(lVar1 + 0x58);
    uVar2 = *(undefined8 *)(lVar1 + 0x28);
    uVar3 = *(undefined8 *)(lVar1 + 0x30);
    uVar4 = *(undefined8 *)(lVar1 + 0x38);
    uVar5 = *(undefined8 *)(lVar1 + 0x40);
    uVar6 = *(undefined8 *)(lVar1 + 0x50);
    in_RCX[4] = *(undefined8 *)(lVar1 + 0x48);
    in_RCX[5] = uVar6;
    in_RCX[2] = uVar4;
    in_RCX[3] = uVar5;
    *in_RCX = uVar2;
    in_RCX[1] = uVar3;
    return;
  }
  *(undefined4 *)in_RCX = 2;
  return;
}


