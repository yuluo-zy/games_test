// display_link_to_official_game @ 0x1403d0c70
// sig: void __cdecl display_link_to_official_game(void)


void __cdecl tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game(void)

{
  ulonglong *puVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong *plVar4;
  longlong lVar5;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 *in_RCX;
  byte *in_RDX;
  ulonglong uVar6;
  ulonglong uVar7;
  ulonglong uVar8;
  longlong *in_R8;
  ulonglong uVar9;
  ulonglong uVar10;
  ulonglong uVar11;
  
  puVar1 = (ulonglong *)*in_RCX;
  lVar2 = in_RCX[1];
  uVar6 = *puVar1;
  uVar10 = 0;
  uVar11 = uVar6 - *(ulonglong *)(lVar2 + 0x18);
  if (uVar6 < *(ulonglong *)(lVar2 + 0x18)) {
    uVar11 = uVar10;
  }
  uVar9 = uVar6 - *(ulonglong *)(lVar2 + 0x38);
  if (uVar6 < *(ulonglong *)(lVar2 + 0x38)) {
    uVar9 = uVar10;
  }
  uVar6 = *(ulonglong *)(lVar2 + 0x30);
  uVar8 = uVar6 - uVar9;
  if (uVar9 > uVar6) {
    uVar8 = uVar10;
  }
  uVar3 = *(ulonglong *)(lVar2 + 0x10);
  uVar7 = uVar3 - uVar11;
  if (uVar11 > uVar3) {
    uVar7 = uVar10;
  }
  uVar10 = *(ulonglong *)(lVar2 + 0x40);
  *puVar1 = uVar10 - (uVar7 + uVar8);
  if (uVar11 <= uVar3 && uVar3 - uVar11 != 0 || uVar9 <= uVar6 && uVar6 - uVar9 != 0) {
    *puVar1 = uVar10;
    *in_RDX = 1;
  }
  else if ((*in_RDX & 1) == 0) {
    return;
  }
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  plVar4 = (longlong *)*in_R8;
  toolbar::mei::core::impl$0::as_id();
  uVar6 = extraout_RAX ^ 0xb1695320004fb443;
  toolbar::mei::core::MeiContext::reuse_or_create_node();
  *(undefined1 *)(extraout_RAX_00 + 0x5d) = 0;
  *(undefined1 *)(extraout_RAX_00 + 0xb7) = 1;
  if ((uVar6 & 1) == 0) {
    lVar2 = plVar4[2];
    if (lVar2 == *plVar4) {
      alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
    }
    lVar5 = plVar4[1];
    *(undefined4 *)(lVar5 + lVar2 * 0x38) = 0;
    *(ulonglong *)(lVar5 + 0x30 + lVar2 * 0x38) = extraout_RAX ^ 0xb1695320004fb443;
    plVar4[2] = lVar2 + 1;
  }
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  toolbar::mei::core::NodeContext::child_with_global_id();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  toolbar::mei::core::NodeContext::child_with_global_id();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  toolbar::mei::core::NodeContext::child_with_global_id();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  toolbar::mei::core::NodeContext::child_with_global_id();
  toolbar::mei::config::impl$18::template();
  toolbar::mei::config::TemplateView::
  with_slot<tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game::closure_env_0,ref_<str_>_>
            ();
  toolbar::mei::config::TemplateView::
  with_slot<tiny_glade::gui::gui_link_to_official_game::display_link_to_official_game::closure_env_1,ref_<str_>_>
            ();
  return;
}

