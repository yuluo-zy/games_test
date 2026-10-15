// 0x1403319f0 tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>_>
// signature: void __cdecl tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>_>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$13::
tweaker<toolbar::mei::widgets::tweaker::TweakableList<alloc::string::String>_>(void)

{
  char *pcVar1;
  longlong lVar2;
  ulonglong *puVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  ulonglong uVar7;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  uint extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong uVar8;
  ulonglong uVar9;
  ushort uVar10;
  longlong lVar11;
  int iVar12;
  ulonglong uVar13;
  uint uVar14;
  longlong lVar15;
  bool bVar16;
  byte bVar17;
  char cVar18;
  char cVar20;
  char cVar21;
  char cVar22;
  undefined1 auVar19 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
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
      uVar13 = **(ulonglong **)(in_stack_00000030 + 0x10);
      uVar14 = extraout_EAX;
      if ((-extraout_EAX & 3) != 0) {
        iVar12 = 0;
        do {
          bVar16 = uVar13 != 0;
          uVar9 = uVar13 - 1;
          uVar13 = 0;
          if (bVar16) {
            uVar13 = uVar9;
          }
          iVar12 = iVar12 + -1;
        } while (-(-extraout_EAX & 3) != iVar12);
        uVar14 = extraout_EAX - iVar12;
      }
      if (extraout_EAX < 0xfffffffd) {
        do {
          bVar16 = uVar13 < 4;
          uVar13 = uVar13 - 4;
          if (bVar16) {
            uVar13 = 0;
          }
          uVar14 = uVar14 + 4;
        } while (uVar14 != 0);
      }
      **(ulonglong **)(in_stack_00000030 + 0x10) = uVar13;
    }
  }
  else {
    uVar13 = *(ulonglong *)(in_stack_00000030 + 8);
    uVar8 = uVar13 - 1;
    uVar9 = **(ulonglong **)(in_stack_00000030 + 0x10);
    uVar14 = extraout_EAX;
    if ((extraout_EAX & 3) != 0) {
      uVar14 = 0;
      do {
        uVar9 = uVar9 + 1;
        if (uVar8 < uVar9) {
          uVar9 = uVar8;
        }
        uVar14 = uVar14 + 1;
      } while ((extraout_EAX & 3) != uVar14);
      uVar14 = extraout_EAX - uVar14;
    }
    if (3 < extraout_EAX) {
      uVar14 = uVar14 - 1;
      do {
        uVar7 = uVar13;
        if (uVar9 + 1 <= uVar8) {
          uVar7 = uVar9 + 2;
        }
        uVar9 = uVar13;
        if (uVar7 <= uVar8) {
          uVar9 = uVar7 + 1;
        }
        uVar7 = uVar13;
        if (uVar9 <= uVar8) {
          uVar7 = uVar9 + 1;
        }
        uVar9 = uVar8;
        if (uVar7 <= uVar8) {
          uVar9 = uVar7;
        }
        uVar14 = uVar14 - 4;
      } while (uVar14 < 0xfffffffe);
    }
    **(ulonglong **)(in_stack_00000030 + 0x10) = uVar9;
  }
  widgets::impl$26::menu_button_no_nav();
  MenuButtonView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  MenuButtonView::on_click();
  impl$37::deref();
  core::Node::used();
  puVar3 = *(ulonglong **)(in_stack_00000030 + 0x10);
  uVar13 = *puVar3;
  if (extraout_AL_00 != '\0') {
    bVar16 = uVar13 == 0;
    uVar13 = uVar13 - 1;
    if (bVar16) {
      uVar13 = 0;
    }
    *puVar3 = uVar13;
  }
  uVar9 = *(ulonglong *)(in_stack_00000030 + 8);
  if (uVar9 <= uVar13) {
                    // WARNING: Subroutine does not return
    ::core::panicking::panic_bounds_check();
  }
  alloc::string::impl$6::clone();
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
    lVar4 = *(longlong *)(lVar2 + 0x1a0);
    bVar17 = (byte)(extraout_RAX_01 >> 0x39);
    auVar19 = ZEXT216(CONCAT11(bVar17,bVar17));
    auVar19 = pshuflw(auVar19,auVar19,0);
    lVar15 = 0;
    uVar13 = extraout_RAX_01;
    while( true ) {
      uVar13 = *(ulonglong *)(lVar2 + 0x1a8) & uVar13;
      pcVar1 = (char *)(lVar4 + uVar13);
      cVar18 = auVar19[0];
      auVar24[0] = -(*pcVar1 == cVar18);
      cVar20 = auVar19[1];
      auVar24[1] = -(pcVar1[1] == cVar20);
      cVar21 = auVar19[2];
      auVar24[2] = -(pcVar1[2] == cVar21);
      cVar22 = auVar19[3];
      auVar24[3] = -(pcVar1[3] == cVar22);
      auVar24[4] = -(pcVar1[4] == cVar18);
      auVar24[5] = -(pcVar1[5] == cVar20);
      auVar24[6] = -(pcVar1[6] == cVar21);
      auVar24[7] = -(pcVar1[7] == cVar22);
      auVar24[8] = -(pcVar1[8] == cVar18);
      auVar24[9] = -(pcVar1[9] == cVar20);
      auVar24[10] = -(pcVar1[10] == cVar21);
      auVar24[0xb] = -(pcVar1[0xb] == cVar22);
      auVar24[0xc] = -(pcVar1[0xc] == cVar18);
      auVar24[0xd] = -(pcVar1[0xd] == cVar20);
      auVar24[0xe] = -(pcVar1[0xe] == cVar21);
      auVar24[0xf] = -(pcVar1[0xf] == cVar22);
      uVar10 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
      uVar14 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar14; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar11 = (uVar5 + uVar13 & *(ulonglong *)(lVar2 + 0x1a8)) * -0xc0;
        if (extraout_RAX_00 == *(longlong *)(lVar4 + -0xc0 + lVar11)) {
          *(undefined4 *)(lVar4 + -0x78 + lVar11) = 0;
          *(float *)(lVar4 + -0x74 + lVar11) = in_stack_00000028 * 0.6;
          widgets::impl$26::menu_button_no_nav();
          MenuButtonView::on_click();
          MenuButtonView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          impl$37::deref();
          core::Node::used();
          if (extraout_AL_01 != '\0') {
            uVar9 = uVar9 - 1;
            if (*puVar3 + 1 <= uVar9) {
              uVar9 = *puVar3 + 1;
            }
            *puVar3 = uVar9;
          }
          if (local_e8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          return;
        }
        uVar10 = (ushort)(uVar14 - 1) & (ushort)uVar14;
        uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar10);
      }
      auVar23[0] = -(*pcVar1 == -1);
      auVar23[1] = -(pcVar1[1] == -1);
      auVar23[2] = -(pcVar1[2] == -1);
      auVar23[3] = -(pcVar1[3] == -1);
      auVar23[4] = -(pcVar1[4] == -1);
      auVar23[5] = -(pcVar1[5] == -1);
      auVar23[6] = -(pcVar1[6] == -1);
      auVar23[7] = -(pcVar1[7] == -1);
      auVar23[8] = -(pcVar1[8] == -1);
      auVar23[9] = -(pcVar1[9] == -1);
      auVar23[10] = -(pcVar1[10] == -1);
      auVar23[0xb] = -(pcVar1[0xb] == -1);
      auVar23[0xc] = -(pcVar1[0xc] == -1);
      auVar23[0xd] = -(pcVar1[0xd] == -1);
      auVar23[0xe] = -(pcVar1[0xe] == -1);
      auVar23[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar23 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar23 >> 0x3f,0) & 1) != 0) || (SUB161(auVar23 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar23 >> 0x4f,0) & 1) != 0) || (SUB161(auVar23 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar23 >> 0x5f,0) & 1) != 0) || (SUB161(auVar23 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar23 >> 0x6f,0) & 1) != 0) || (SUB161(auVar23 >> 0x77,0) & 1) != 0) ||
          auVar23[0xf] < '\0') break;
      uVar13 = uVar13 + lVar15 + 0x10;
      lVar15 = lVar15 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

