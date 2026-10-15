// 0x14068bf00 label
// signature: void __cdecl label(void)

void __cdecl toolbar::mei::widgets::impl$26::label(void)

{
  undefined8 uVar1;
  undefined8 extraout_RAX;
  undefined8 *in_RCX;
  longlong in_RDX;
  
  ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  uVar1 = *(undefined8 *)(in_RDX + 8);
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::LabelContent>();
  tg_text::TextFragment::set_text<ref_<str_>_>();
  in_RCX[2] = 0xce69dbf0aee13da2;
  *in_RCX = uVar1;
  in_RCX[1] = extraout_RAX;
  return;
}

