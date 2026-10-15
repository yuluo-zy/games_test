// 0x14032eba0 tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory>_>
// signature: void __cdecl tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory>_>(void)

// WARNING: Removing unreachable block (ram,0x00014032f0ec)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
toolbar::mei::widgets::tweaker::impl$13::
tweaker_w_font_size<toolbar::mei::widgets::tweaker::TweakableEnum<tiny_glade::gui::gui_controls::GuiKeyBindingsCategory>_>
          (void)

{
  char *pcVar1;
  uint uVar2;
  uint uVar3;
  longlong lVar4;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  int extraout_EAX;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  ushort uVar5;
  longlong lVar6;
  ulonglong uVar7;
  longlong lVar8;
  uint uVar9;
  byte bVar10;
  char cVar11;
  char cVar13;
  char cVar14;
  char cVar15;
  undefined1 auVar12 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  float in_stack_00000028;
  longlong *in_stack_00000038;
  
  tweaker_impl<ref_<str_>_>();
  TweakerView::delta();
  TweakerView::as_inner_mut();
  impl$37::deref();
  core::Node::hover();
  if (extraout_AL != '\0') {
    MaybeHint::confirm();
  }
  widgets::impl$26::label();
  LabelView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  LabelView::family();
  LabelView::max_width();
  LabelView::weight();
  LabelView::color();
  ActionHintView::into_inner();
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  layout::horizontal();
  core::NodeContext::layout();
  if (0 < extraout_EAX) {
                    // WARNING: Could not recover jumptable at 0x00014032ed65. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_1429bf7dc + *(int *)(&DAT_1429bf7dc + *in_stack_00000038 * 4)))
              (1,*in_stack_00000038,
               &DAT_1429bf7dc + *(int *)(&DAT_1429bf7dc + *in_stack_00000038 * 4));
    return;
  }
  if (extraout_EAX < 0) {
                    // WARNING: Could not recover jumptable at 0x00014032ed83. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_1429bf7c0 + *(int *)(&DAT_1429bf7c0 + *in_stack_00000038 * 4)))
              (0,*in_stack_00000038,
               &DAT_1429bf7c0 + *(int *)(&DAT_1429bf7c0 + *in_stack_00000038 * 4));
    return;
  }
  widgets::impl$26::menu_button_no_nav();
  MenuButtonView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  MenuButtonView::on_click();
  impl$37::deref();
  core::Node::used();
  if (extraout_AL_00 != '\0') {
                    // WARNING: Could not recover jumptable at 0x00014032ee50. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)(&DAT_1429bf7f8 + *(int *)(&DAT_1429bf7f8 + *in_stack_00000038 * 4)))();
    return;
  }
  tiny_glade::gui::gui_controls::impl$0::fmt();
  if (extraout_AL_01 != '\0') {
                    // WARNING: Subroutine does not return
    ::core::result::unwrap_failed();
  }
  widgets::impl$26::label();
  LabelView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  LabelView::family();
  LabelView::color();
  LabelView::align();
  ActionHintView::into_inner();
  if (lRam00000000000001b8 != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar4 = lRam00000000000001a0;
    bVar10 = (byte)(extraout_RAX_00 >> 0x39);
    auVar12 = ZEXT216(CONCAT11(bVar10,bVar10));
    auVar12 = pshuflw(auVar12,auVar12,0);
    lVar8 = 0;
    uVar7 = extraout_RAX_00;
    while( true ) {
      uVar7 = _DAT_000001a8 & uVar7;
      pcVar1 = (char *)(lRam00000000000001a0 + uVar7);
      cVar11 = auVar12[0];
      auVar17[0] = -(*pcVar1 == cVar11);
      cVar13 = auVar12[1];
      auVar17[1] = -(pcVar1[1] == cVar13);
      cVar14 = auVar12[2];
      auVar17[2] = -(pcVar1[2] == cVar14);
      cVar15 = auVar12[3];
      auVar17[3] = -(pcVar1[3] == cVar15);
      auVar17[4] = -(pcVar1[4] == cVar11);
      auVar17[5] = -(pcVar1[5] == cVar13);
      auVar17[6] = -(pcVar1[6] == cVar14);
      auVar17[7] = -(pcVar1[7] == cVar15);
      auVar17[8] = -(pcVar1[8] == cVar11);
      auVar17[9] = -(pcVar1[9] == cVar13);
      auVar17[10] = -(pcVar1[10] == cVar14);
      auVar17[0xb] = -(pcVar1[0xb] == cVar15);
      auVar17[0xc] = -(pcVar1[0xc] == cVar11);
      auVar17[0xd] = -(pcVar1[0xd] == cVar13);
      auVar17[0xe] = -(pcVar1[0xe] == cVar14);
      auVar17[0xf] = -(pcVar1[0xf] == cVar15);
      uVar5 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
      uVar9 = (uint)uVar5;
      while (uVar5 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar9; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        lVar6 = (uVar2 + uVar7 & _DAT_000001a8) * -0xc0;
        if (extraout_RAX == *(longlong *)(lRam00000000000001a0 + -0xc0 + lVar6)) {
          *(undefined4 *)(lRam00000000000001a0 + -0x78 + lVar6) = 0;
          *(float *)(lVar4 + -0x74 + lVar6) = in_stack_00000028 * 0.6;
          widgets::impl$26::menu_button_no_nav();
          MenuButtonView::on_click();
          MenuButtonView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          impl$37::deref();
          core::Node::used();
          if (extraout_AL_02 != '\0') {
                    // WARNING: Could not recover jumptable at 0x00014032f0ac. Too many branches
                    // WARNING: Treating indirect jump as call
            (*(code *)(&DAT_1429bf814 + *(int *)(&DAT_1429bf814 + *in_stack_00000038 * 4)))();
            return;
          }
          return;
        }
        uVar5 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar5);
      }
      auVar16[0] = -(*pcVar1 == -1);
      auVar16[1] = -(pcVar1[1] == -1);
      auVar16[2] = -(pcVar1[2] == -1);
      auVar16[3] = -(pcVar1[3] == -1);
      auVar16[4] = -(pcVar1[4] == -1);
      auVar16[5] = -(pcVar1[5] == -1);
      auVar16[6] = -(pcVar1[6] == -1);
      auVar16[7] = -(pcVar1[7] == -1);
      auVar16[8] = -(pcVar1[8] == -1);
      auVar16[9] = -(pcVar1[9] == -1);
      auVar16[10] = -(pcVar1[10] == -1);
      auVar16[0xb] = -(pcVar1[0xb] == -1);
      auVar16[0xc] = -(pcVar1[0xc] == -1);
      auVar16[0xd] = -(pcVar1[0xd] == -1);
      auVar16[0xe] = -(pcVar1[0xe] == -1);
      auVar16[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar16 >> 7,0) & 1) != 0 || (SUB161(auVar16 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar16 >> 0x17,0) & 1) != 0) || (SUB161(auVar16 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar16 >> 0x27,0) & 1) != 0) || (SUB161(auVar16 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar16 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar16 >> 0x3f,0) & 1) != 0) || (SUB161(auVar16 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar16 >> 0x4f,0) & 1) != 0) || (SUB161(auVar16 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar16 >> 0x5f,0) & 1) != 0) || (SUB161(auVar16 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar16 >> 0x6f,0) & 1) != 0) || (SUB161(auVar16 >> 0x77,0) & 1) != 0) ||
          auVar16[0xf] < '\0') break;
      uVar7 = uVar7 + lVar8 + 0x10;
      lVar8 = lVar8 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

