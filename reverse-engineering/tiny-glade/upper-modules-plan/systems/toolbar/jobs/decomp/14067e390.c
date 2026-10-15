// 0x14067e390 max_width
// signature: void __cdecl max_width(void)

void __cdecl toolbar::mei::widgets::LabelView::max_width(void)

{
  undefined8 *puVar1;
  undefined8 uVar2;
  undefined8 *in_RCX;
  undefined1 in_DL;
  undefined7 in_register_00000011;
  
  puVar1 = (undefined8 *)CONCAT71(in_register_00000011,in_DL);
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
  tg_text::TextFragment::set_max_width();
  in_RCX[2] = puVar1[2];
  uVar2 = puVar1[1];
  *in_RCX = *puVar1;
  in_RCX[1] = uVar2;
  return;
}

