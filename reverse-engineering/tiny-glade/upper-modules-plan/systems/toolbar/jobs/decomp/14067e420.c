// 0x14067e420 family
// signature: void __cdecl family(void)

void __cdecl toolbar::mei::widgets::LabelView::family(void)

{
  undefined8 uVar1;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
  tg_text::TextFragment::set_family();
  in_RCX[2] = in_RDX[2];
  uVar1 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar1;
  return;
}

