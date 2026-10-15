// 0x14032f240 tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>_>
// signature: void __cdecl tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>_>(void)

// WARNING: Removing unreachable block (ram,0x00014032f6e2)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl
toolbar::mei::widgets::tweaker::impl$13::
tweaker<toolbar::mei::widgets::tweaker::TweakableEnum<country_core::resources::ui_settings::ToolbarAlignment>_>
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
  char extraout_AL_03;
  char extraout_AL_04;
  char extraout_AL_05;
  char extraout_AL_06;
  int extraout_EAX;
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  ushort uVar5;
  longlong lVar6;
  uint uVar7;
  int iVar8;
  ulonglong uVar9;
  longlong lVar10;
  byte bVar11;
  char cVar12;
  char cVar14;
  char cVar15;
  char cVar16;
  undefined1 auVar13 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  float in_stack_00000028;
  char *in_stack_00000030;
  
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
  if (extraout_EAX < 1) {
    iVar8 = extraout_EAX;
    if (extraout_EAX < 0) {
      do {
        country_core::resources::ui_settings::_::impl$0::previous();
        if (extraout_AL_01 != '\x04') {
          *in_stack_00000030 = extraout_AL_01;
        }
        iVar8 = iVar8 + 1;
      } while (iVar8 != 0);
    }
  }
  else {
    uVar7 = extraout_EAX + 1;
    do {
      country_core::resources::clutter_meshes::_::impl$0::next();
      if (extraout_AL_00 != '\x04') {
        *in_stack_00000030 = extraout_AL_00;
      }
      uVar7 = uVar7 - 1;
    } while (1 < uVar7);
  }
  widgets::impl$26::menu_button_no_nav();
  MenuButtonView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  MenuButtonView::on_click();
  impl$37::deref();
  core::Node::used();
  if ((extraout_AL_02 != '\0') &&
     (country_core::resources::ui_settings::_::impl$0::previous(), extraout_AL_03 != '\x04')) {
    *in_stack_00000030 = extraout_AL_03;
  }
  country_core::resources::ui_settings::impl$1::fmt();
  if (extraout_AL_04 != '\0') {
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
    bVar11 = (byte)(extraout_RAX_00 >> 0x39);
    auVar13 = ZEXT216(CONCAT11(bVar11,bVar11));
    auVar13 = pshuflw(auVar13,auVar13,0);
    lVar10 = 0;
    uVar9 = extraout_RAX_00;
    while( true ) {
      uVar9 = _DAT_000001a8 & uVar9;
      pcVar1 = (char *)(lRam00000000000001a0 + uVar9);
      cVar12 = auVar13[0];
      auVar18[0] = -(*pcVar1 == cVar12);
      cVar14 = auVar13[1];
      auVar18[1] = -(pcVar1[1] == cVar14);
      cVar15 = auVar13[2];
      auVar18[2] = -(pcVar1[2] == cVar15);
      cVar16 = auVar13[3];
      auVar18[3] = -(pcVar1[3] == cVar16);
      auVar18[4] = -(pcVar1[4] == cVar12);
      auVar18[5] = -(pcVar1[5] == cVar14);
      auVar18[6] = -(pcVar1[6] == cVar15);
      auVar18[7] = -(pcVar1[7] == cVar16);
      auVar18[8] = -(pcVar1[8] == cVar12);
      auVar18[9] = -(pcVar1[9] == cVar14);
      auVar18[10] = -(pcVar1[10] == cVar15);
      auVar18[0xb] = -(pcVar1[0xb] == cVar16);
      auVar18[0xc] = -(pcVar1[0xc] == cVar12);
      auVar18[0xd] = -(pcVar1[0xd] == cVar14);
      auVar18[0xe] = -(pcVar1[0xe] == cVar15);
      auVar18[0xf] = -(pcVar1[0xf] == cVar16);
      uVar5 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
      uVar7 = (uint)uVar5;
      while (uVar5 != 0) {
        uVar2 = 0;
        for (uVar3 = uVar7; (uVar3 & 1) == 0; uVar3 = uVar3 >> 1 | 0x80000000) {
          uVar2 = uVar2 + 1;
        }
        lVar6 = (uVar2 + uVar9 & _DAT_000001a8) * -0xc0;
        if (extraout_RAX == *(longlong *)(lRam00000000000001a0 + -0xc0 + lVar6)) {
          *(undefined4 *)(lRam00000000000001a0 + -0x78 + lVar6) = 0;
          *(float *)(lVar4 + -0x74 + lVar6) = in_stack_00000028 * 0.6;
          widgets::impl$26::menu_button_no_nav();
          MenuButtonView::on_click();
          MenuButtonView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          impl$37::deref();
          core::Node::used();
          if ((extraout_AL_05 != '\0') &&
             (country_core::resources::clutter_meshes::_::impl$0::next(), extraout_AL_06 != '\x04'))
          {
            *in_stack_00000030 = extraout_AL_06;
          }
          return;
        }
        uVar5 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar5);
      }
      auVar17[0] = -(*pcVar1 == -1);
      auVar17[1] = -(pcVar1[1] == -1);
      auVar17[2] = -(pcVar1[2] == -1);
      auVar17[3] = -(pcVar1[3] == -1);
      auVar17[4] = -(pcVar1[4] == -1);
      auVar17[5] = -(pcVar1[5] == -1);
      auVar17[6] = -(pcVar1[6] == -1);
      auVar17[7] = -(pcVar1[7] == -1);
      auVar17[8] = -(pcVar1[8] == -1);
      auVar17[9] = -(pcVar1[9] == -1);
      auVar17[10] = -(pcVar1[10] == -1);
      auVar17[0xb] = -(pcVar1[0xb] == -1);
      auVar17[0xc] = -(pcVar1[0xc] == -1);
      auVar17[0xd] = -(pcVar1[0xd] == -1);
      auVar17[0xe] = -(pcVar1[0xe] == -1);
      auVar17[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      uVar9 = uVar9 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

