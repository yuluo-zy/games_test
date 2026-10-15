// 0x14067e9a0 color
// signature: void __cdecl color(void)

void __cdecl toolbar::mei::widgets::MenuButtonView::color(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  undefined1 in_DL;
  undefined7 in_register_00000011;
  undefined4 in_R8D;
  
  puVar1 = (undefined8 *)CONCAT71(in_register_00000011,in_DL);
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::MenuButtonContent>();
  *(undefined4 *)(extraout_RAX + 600) = in_R8D;
  uVar2 = puVar1[1];
  *in_RCX = *puVar1;
  in_RCX[1] = uVar2;
  in_RCX[2] = puVar1[2];
  return;
}

