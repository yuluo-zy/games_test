// 0x140331400 tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>_>
// signature: void __cdecl tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>_>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$13::
tweaker<toolbar::mei::widgets::tweaker::TweakableList<tiny_glade::gui::settings::Resolution>_>(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  ulonglong uVar6;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong uVar7;
  ulonglong uVar8;
  ushort uVar9;
  longlong lVar10;
  int iVar11;
  ulonglong uVar12;
  uint uVar13;
  longlong lVar14;
  bool bVar15;
  byte bVar16;
  char cVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  float in_stack_00000028;
  longlong in_stack_00000030;
  longlong local_e8;
  
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
  lVar2 = *(longlong *)(extraout_RAX + 8);
  minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
  ::utils::calculate_hash<tuple_<u64,core::any::TypeId>_>();
  ::utils::calculate_hash<tuple_<u64,u64>_>();
  core::NodeContext::child_with_global_id();
  layout::horizontal();
  core::NodeContext::layout();
  if ((int)extraout_EAX < 1) {
    if ((int)extraout_EAX < 0) {
      uVar12 = **(ulonglong **)(in_stack_00000030 + 0x10);
      uVar13 = extraout_EAX;
      if ((-extraout_EAX & 3) != 0) {
        iVar11 = 0;
        do {
          bVar15 = uVar12 != 0;
          uVar8 = uVar12 - 1;
          uVar12 = 0;
          if (bVar15) {
            uVar12 = uVar8;
          }
          iVar11 = iVar11 + -1;
        } while (-(-extraout_EAX & 3) != iVar11);
        uVar13 = extraout_EAX - iVar11;
      }
      if (extraout_EAX < 0xfffffffd) {
        do {
          bVar15 = uVar12 < 4;
          uVar12 = uVar12 - 4;
          if (bVar15) {
            uVar12 = 0;
          }
          uVar13 = uVar13 + 4;
        } while (uVar13 != 0);
      }
      **(ulonglong **)(in_stack_00000030 + 0x10) = uVar12;
    }
  }
  else {
    uVar12 = *(ulonglong *)(in_stack_00000030 + 8);
    uVar7 = uVar12 - 1;
    uVar8 = **(ulonglong **)(in_stack_00000030 + 0x10);
    uVar13 = extraout_EAX;
    if ((extraout_EAX & 3) != 0) {
      uVar13 = 0;
      do {
        uVar8 = uVar8 + 1;
        if (uVar7 < uVar8) {
          uVar8 = uVar7;
        }
        uVar13 = uVar13 + 1;
      } while ((extraout_EAX & 3) != uVar13);
      uVar13 = extraout_EAX - uVar13;
    }
    if (3 < extraout_EAX) {
      uVar13 = uVar13 - 1;
      do {
        uVar6 = uVar12;
        if (uVar8 + 1 <= uVar7) {
          uVar6 = uVar8 + 2;
        }
        uVar8 = uVar12;
        if (uVar6 <= uVar7) {
          uVar8 = uVar6 + 1;
        }
        uVar6 = uVar12;
        if (uVar8 <= uVar7) {
          uVar6 = uVar8 + 1;
        }
        uVar8 = uVar7;
        if (uVar6 <= uVar7) {
          uVar8 = uVar6;
        }
        uVar13 = uVar13 - 4;
      } while (uVar13 < 0xfffffffe);
    }
    **(ulonglong **)(in_stack_00000030 + 0x10) = uVar8;
  }
  widgets::impl$26::menu_button_no_nav();
  MenuButtonView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  MenuButtonView::on_click();
  impl$37::deref();
  core::Node::used();
  if (extraout_AL_00 != '\0') {
    lVar3 = **(longlong **)(in_stack_00000030 + 0x10);
    lVar14 = 0;
    if (lVar3 != 0) {
      lVar14 = lVar3 + -1;
    }
    **(longlong **)(in_stack_00000030 + 0x10) = lVar14;
  }
  tweaker::impl$8::format<tiny_glade::gui::settings::Resolution>();
  widgets::impl$26::label();
  LabelView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  LabelView::family();
  LabelView::color();
  LabelView::align();
  ActionHintView::into_inner();
  if (*(longlong *)(lVar2 + 0x1b8) != 0) {
    ::core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar3 = *(longlong *)(lVar2 + 0x1a0);
    bVar16 = (byte)(extraout_RAX_01 >> 0x39);
    auVar18 = ZEXT216(CONCAT11(bVar16,bVar16));
    auVar18 = pshuflw(auVar18,auVar18,0);
    lVar14 = 0;
    uVar12 = extraout_RAX_01;
    while( true ) {
      uVar12 = *(ulonglong *)(lVar2 + 0x1a8) & uVar12;
      pcVar1 = (char *)(lVar3 + uVar12);
      cVar17 = auVar18[0];
      auVar23[0] = -(*pcVar1 == cVar17);
      cVar19 = auVar18[1];
      auVar23[1] = -(pcVar1[1] == cVar19);
      cVar20 = auVar18[2];
      auVar23[2] = -(pcVar1[2] == cVar20);
      cVar21 = auVar18[3];
      auVar23[3] = -(pcVar1[3] == cVar21);
      auVar23[4] = -(pcVar1[4] == cVar17);
      auVar23[5] = -(pcVar1[5] == cVar19);
      auVar23[6] = -(pcVar1[6] == cVar20);
      auVar23[7] = -(pcVar1[7] == cVar21);
      auVar23[8] = -(pcVar1[8] == cVar17);
      auVar23[9] = -(pcVar1[9] == cVar19);
      auVar23[10] = -(pcVar1[10] == cVar20);
      auVar23[0xb] = -(pcVar1[0xb] == cVar21);
      auVar23[0xc] = -(pcVar1[0xc] == cVar17);
      auVar23[0xd] = -(pcVar1[0xd] == cVar19);
      auVar23[0xe] = -(pcVar1[0xe] == cVar20);
      auVar23[0xf] = -(pcVar1[0xf] == cVar21);
      uVar9 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar9;
      while (uVar9 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar13; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar10 = (uVar4 + uVar12 & *(ulonglong *)(lVar2 + 0x1a8)) * -0xc0;
        if (extraout_RAX_00 == *(longlong *)(lVar3 + -0xc0 + lVar10)) {
          *(undefined4 *)(lVar3 + -0x78 + lVar10) = 0;
          *(float *)(lVar3 + -0x74 + lVar10) = in_stack_00000028 * 0.6;
          widgets::impl$26::menu_button_no_nav();
          MenuButtonView::on_click();
          MenuButtonView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          impl$37::deref();
          core::Node::used();
          if (extraout_AL_01 != '\0') {
            uVar8 = **(ulonglong **)(in_stack_00000030 + 0x10) + 1;
            uVar12 = *(longlong *)(in_stack_00000030 + 8) - 1;
            if (uVar8 <= uVar12) {
              uVar12 = uVar8;
            }
            **(ulonglong **)(in_stack_00000030 + 0x10) = uVar12;
          }
          if (local_e8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          return;
        }
        uVar9 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar9);
      }
      auVar22[0] = -(*pcVar1 == -1);
      auVar22[1] = -(pcVar1[1] == -1);
      auVar22[2] = -(pcVar1[2] == -1);
      auVar22[3] = -(pcVar1[3] == -1);
      auVar22[4] = -(pcVar1[4] == -1);
      auVar22[5] = -(pcVar1[5] == -1);
      auVar22[6] = -(pcVar1[6] == -1);
      auVar22[7] = -(pcVar1[7] == -1);
      auVar22[8] = -(pcVar1[8] == -1);
      auVar22[9] = -(pcVar1[9] == -1);
      auVar22[10] = -(pcVar1[10] == -1);
      auVar22[0xb] = -(pcVar1[0xb] == -1);
      auVar22[0xc] = -(pcVar1[0xc] == -1);
      auVar22[0xd] = -(pcVar1[0xd] == -1);
      auVar22[0xe] = -(pcVar1[0xe] == -1);
      auVar22[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      uVar12 = uVar12 + lVar14 + 0x10;
      lVar14 = lVar14 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

