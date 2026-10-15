// 0x1406803f0 draw
// signature: void __cdecl draw(void)

void __cdecl toolbar::mei::widgets::FrameView::draw(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  longlong *extraout_RAX;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  longlong *in_R8;
  longlong lVar6;
  longlong lVar7;
  
  core::NodeContext::get_content_mut_impl<toolbar::mei::widgets::FrameContent>();
  lVar1 = in_R8[2];
  lVar3 = *in_R8;
  lVar4 = in_R8[1];
  lVar7 = extraout_RAX[2];
  if (lVar7 != 0) {
    lVar6 = extraout_RAX[1] + 8;
    do {
      lVar2 = *(longlong *)(lVar6 + -8);
      if (((-0x7fffffffffffffff < lVar2) &&
          (2 < lVar2 + 0x7ffffffffffffffeU || lVar2 + 0x7ffffffffffffffeU == 1)) && (lVar2 != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar6 = lVar6 + 0x20;
      lVar7 = lVar7 + -1;
    } while (lVar7 != 0);
  }
  if (*extraout_RAX != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  extraout_RAX[2] = lVar1;
  *extraout_RAX = lVar3;
  extraout_RAX[1] = lVar4;
  in_RCX[2] = in_RDX[2];
  uVar5 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar5;
  return;
}

