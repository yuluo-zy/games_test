// 0x14067e9e0 force_highlight
// signature: void __cdecl force_highlight(void)

void __cdecl toolbar::mei::widgets::MenuButtonView::force_highlight(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  undefined1 in_DL;
  undefined7 in_register_00000011;
  undefined1 in_R8B;
  
  puVar1 = (undefined8 *)CONCAT71(in_register_00000011,in_DL);
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::MenuButtonContent>();
  *(undefined1 *)(extraout_RAX + 0x25c) = in_R8B;
  uVar2 = puVar1[1];
  *in_RCX = *puVar1;
  in_RCX[1] = uVar2;
  in_RCX[2] = puVar1[2];
  return;
}

