// 0x14067e8d0 weight
// signature: void __cdecl weight(void)

void __cdecl toolbar::mei::widgets::MenuButtonView::weight(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::MenuButtonContent>();
  tg_text::TextFragment::set_weight();
  in_RCX[2] = in_RDX[2];
  uVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar1;
  return;
}

