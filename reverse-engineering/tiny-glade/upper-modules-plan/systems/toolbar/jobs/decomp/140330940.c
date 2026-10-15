// 0x140330940 tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
// signature: void __cdecl tweaker<toolbar::mei::widgets::tweaker::TweakableBool>(void)

void __cdecl
toolbar::mei::widgets::tweaker::impl$13::tweaker<toolbar::mei::widgets::tweaker::TweakableBool>
          (void)

{
  char *pcVar1;
  longlong lVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  int extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  uint uVar6;
  int iVar7;
  ushort uVar8;
  longlong lVar9;
  ulonglong uVar10;
  longlong lVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  float in_stack_00000028;
  longlong local_d8;
  
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
  if (extraout_EAX < 1) {
    iVar7 = extraout_EAX;
    if (extraout_EAX < 0) {
      do {
        tweaker::impl$6::decrease();
        iVar7 = iVar7 + 1;
      } while (iVar7 != 0);
    }
  }
  else {
    uVar6 = extraout_EAX + 1;
    do {
      tweaker::impl$6::increase();
      uVar6 = uVar6 - 1;
    } while (1 < uVar6);
  }
  widgets::impl$26::menu_button_no_nav();
  MenuButtonView::content_mut();
  tg_text::TextFragment::set_metrics<f32>();
  MenuButtonView::on_click();
  impl$37::deref();
  core::Node::used();
  if (extraout_AL_00 != '\0') {
    tweaker::impl$6::decrease();
  }
  tweaker::impl$6::format();
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
    bVar12 = (byte)(extraout_RAX_01 >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar11 = 0;
    uVar10 = extraout_RAX_01;
    while( true ) {
      uVar10 = *(ulonglong *)(lVar2 + 0x1a8) & uVar10;
      pcVar1 = (char *)(lVar3 + uVar10);
      cVar13 = auVar14[0];
      auVar19[0] = -(*pcVar1 == cVar13);
      cVar15 = auVar14[1];
      auVar19[1] = -(pcVar1[1] == cVar15);
      cVar16 = auVar14[2];
      auVar19[2] = -(pcVar1[2] == cVar16);
      cVar17 = auVar14[3];
      auVar19[3] = -(pcVar1[3] == cVar17);
      auVar19[4] = -(pcVar1[4] == cVar13);
      auVar19[5] = -(pcVar1[5] == cVar15);
      auVar19[6] = -(pcVar1[6] == cVar16);
      auVar19[7] = -(pcVar1[7] == cVar17);
      auVar19[8] = -(pcVar1[8] == cVar13);
      auVar19[9] = -(pcVar1[9] == cVar15);
      auVar19[10] = -(pcVar1[10] == cVar16);
      auVar19[0xb] = -(pcVar1[0xb] == cVar17);
      auVar19[0xc] = -(pcVar1[0xc] == cVar13);
      auVar19[0xd] = -(pcVar1[0xd] == cVar15);
      auVar19[0xe] = -(pcVar1[0xe] == cVar16);
      auVar19[0xf] = -(pcVar1[0xf] == cVar17);
      uVar8 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar6 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar6; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar9 = (uVar4 + uVar10 & *(ulonglong *)(lVar2 + 0x1a8)) * -0xc0;
        if (extraout_RAX_00 == *(longlong *)(lVar3 + -0xc0 + lVar9)) {
          *(undefined4 *)(lVar3 + -0x78 + lVar9) = 0;
          *(float *)(lVar3 + -0x74 + lVar9) = in_stack_00000028 * 0.6;
          widgets::impl$26::menu_button_no_nav();
          MenuButtonView::on_click();
          MenuButtonView::content_mut();
          tg_text::TextFragment::set_metrics<f32>();
          impl$37::deref();
          core::Node::used();
          if (extraout_AL_01 != '\0') {
            tweaker::impl$6::increase();
          }
          if (local_d8 != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          return;
        }
        uVar8 = (ushort)(uVar6 - 1) & (ushort)uVar6;
        uVar6 = CONCAT22((short)(uVar6 - 1 >> 0x10),uVar8);
      }
      auVar18[0] = -(*pcVar1 == -1);
      auVar18[1] = -(pcVar1[1] == -1);
      auVar18[2] = -(pcVar1[2] == -1);
      auVar18[3] = -(pcVar1[3] == -1);
      auVar18[4] = -(pcVar1[4] == -1);
      auVar18[5] = -(pcVar1[5] == -1);
      auVar18[6] = -(pcVar1[6] == -1);
      auVar18[7] = -(pcVar1[7] == -1);
      auVar18[8] = -(pcVar1[8] == -1);
      auVar18[9] = -(pcVar1[9] == -1);
      auVar18[10] = -(pcVar1[10] == -1);
      auVar18[0xb] = -(pcVar1[0xb] == -1);
      auVar18[0xc] = -(pcVar1[0xc] == -1);
      auVar18[0xd] = -(pcVar1[0xd] == -1);
      auVar18[0xe] = -(pcVar1[0xe] == -1);
      auVar18[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      uVar10 = uVar10 + lVar11 + 0x10;
      lVar11 = lVar11 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  ::core::option::unwrap_failed();
}

