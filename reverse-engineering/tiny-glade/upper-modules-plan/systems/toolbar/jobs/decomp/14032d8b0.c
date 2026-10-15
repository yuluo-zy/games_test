// 0x14032d8b0 with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>
// signature: void __cdecl with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env$0>(void)

void __cdecl
toolbar::mei::core::NodeContext::
with<tiny_glade::gui::language_menu::do_gui_language_menu::closure_env_0>(void)

{
  undefined1 *puVar1;
  size_t *psVar2;
  void *_Src;
  size_t _Size;
  undefined8 uVar3;
  char extraout_AL;
  longlong extraout_RAX;
  void *extraout_RAX_00;
  void *_Dst;
  undefined8 *in_RCX;
  longlong lVar4;
  undefined8 *in_RDX;
  longlong *in_R8;
  longlong lVar5;
  
  lVar4 = ((longlong *)*in_R8)[1];
  if (lVar4 != 0) {
    lVar5 = *(longlong *)*in_R8;
    lVar4 = lVar4 * 0x20 + lVar5;
    puVar1 = (undefined1 *)in_R8[1];
    psVar2 = (size_t *)in_R8[2];
    do {
      _Src = *(void **)(lVar5 + 0x10);
      _Size = *(size_t *)(lVar5 + 0x18);
      widgets::impl$26::menu_button();
      widgets::MenuButtonView::weight();
      widgets::MenuButtonView::color();
      widgets::impl$37::deref();
      Node::used();
      if (extraout_AL != '\0') {
        lang_lib::load_locale();
        if (extraout_RAX != 0) {
                    // WARNING: Subroutine does not return
          ::core::result::unwrap_failed();
        }
        *puVar1 = 1;
        if ((longlong)_Size < 0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        if (_Size == 0) {
          _Dst = (void *)0x1;
        }
        else {
          std::alloc::__default_lib_allocator::__rust_alloc();
          _Dst = extraout_RAX_00;
          if (extraout_RAX_00 == (void *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::raw_vec::handle_error();
          }
        }
        memcpy(_Dst,_Src,_Size);
        if ((!SBORROW8(0,*psVar2)) && (*psVar2 != 0)) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        *psVar2 = _Size;
        psVar2[1] = (size_t)_Dst;
        psVar2[2] = _Size;
      }
      lVar5 = lVar5 + 0x20;
    } while (lVar5 != lVar4);
  }
  in_RCX[2] = in_RDX[2];
  uVar3 = in_RDX[1];
  *in_RCX = *in_RDX;
  in_RCX[1] = uVar3;
  return;
}

