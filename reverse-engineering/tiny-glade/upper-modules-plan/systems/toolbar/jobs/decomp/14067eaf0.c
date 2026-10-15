// 0x14067eaf0 blur_extend
// signature: void __cdecl blur_extend(void)

void __cdecl toolbar::mei::widgets::ImageView::blur_extend(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  longlong extraout_RAX;
  undefined8 *in_RCX;
  undefined1 in_DL;
  undefined7 in_register_00000011;
  
  puVar1 = (undefined8 *)CONCAT71(in_register_00000011,in_DL);
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::ImageContent>();
  *(undefined1 *)(extraout_RAX + 0x44) = 1;
  uVar2 = puVar1[1];
  *in_RCX = *puVar1;
  in_RCX[1] = uVar2;
  in_RCX[2] = puVar1[2];
  return;
}

