// do_gui_error_log @ 0x0x140160410
// sig: void __cdecl do_gui_error_log(void)
// size: 608


void __cdecl tiny_glade::gui::gui_error_log::do_gui_error_log(void)

{
  longlong lVar1;
  undefined8 *extraout_RAX;
  longlong *in_RCX;
  longlong *in_RDX;
  undefined8 in_R8;
  undefined1 local_4e8 [220];
  undefined2 local_40c;
  undefined2 local_40a;
  undefined8 local_408;
  undefined2 local_400;
  undefined1 local_3fe;
  byte local_3fd;
  undefined1 local_3fc [156];
  undefined8 local_360;
  undefined8 uStack_358;
  undefined8 local_350;
  undefined8 uStack_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 uStack_328;
  undefined8 local_320;
  undefined8 uStack_318;
  undefined1 local_1dc;
  undefined1 local_1d8 [16];
  longlong *local_1c8;
  int local_198;
  undefined2 local_fa;
  undefined2 local_f0;
  byte local_ed;
  undefined1 local_ec [68];
  undefined4 local_a8;
  undefined8 local_a4;
  undefined8 uStack_9c;
  undefined8 local_94;
  undefined8 uStack_8c;
  undefined8 local_84;
  undefined8 uStack_7c;
  undefined8 local_74;
  undefined8 uStack_6c;
  undefined8 local_64;
  undefined8 uStack_5c;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  lVar1 = *in_RCX;
  if (((!SBORROW8(0,*(longlong *)(lVar1 + 0x90)) ||
        (char)(!SBORROW8(0,*(longlong *)(lVar1 + 0x70)) + !SBORROW8(0,*(longlong *)(lVar1 + 0x50)) +
              !SBORROW8(0,*(longlong *)(lVar1 + 0x30)) + !SBORROW8(0,*(longlong *)(lVar1 + 0x10)))
        != '\0') && (*in_RDX != 0)) && (*(char *)(lVar1 + 0xf7) == '\x01')) {
    egui::containers::window::Window::new<ref_<str_>_>();
    local_1dc = 0;
    memcpy(local_1d8,&local_360,0x188);
    iced_core::renderer::impl$0::default();
    local_a8 = 1;
    local_a4 = local_360;
    uStack_9c = uStack_358;
    local_94 = local_350;
    uStack_8c = uStack_348;
    local_84 = local_340;
    uStack_7c = uStack_338;
    local_74 = local_330;
    uStack_6c = uStack_328;
    local_64 = local_320;
    uStack_5c = uStack_318;
    memcpy(local_4e8,local_1d8,0xdc);
    memcpy(local_3fc,local_ec,0x9c);
    local_3fd = local_ed & 1;
    local_40c = 2;
    local_40a = local_fa;
    local_408 = 0xc1a00000;
    local_400 = local_f0;
    local_3fe = 0;
    local_41 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    memcpy(&local_360,local_4e8,0x188);
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *extraout_RAX = in_RCX;
    extraout_RAX[1] = in_R8;
    egui::containers::window::Window::show_dyn<tuple_<>_>();
    if (local_198 != 2) {
      LOCK();
      *local_1c8 = *local_1c8 + -1;
      UNLOCK();
      if (*local_1c8 == 0) {
        alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
    }
  }
  return;
}

