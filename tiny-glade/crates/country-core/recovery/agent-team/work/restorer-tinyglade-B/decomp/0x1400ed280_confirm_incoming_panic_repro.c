// confirm_incoming_panic_repro @ 0x1400ed280
// sig: void __cdecl confirm_incoming_panic_repro(void)


void __cdecl tiny_glade::gui::confirm_incoming_panic_repro::confirm_incoming_panic_repro(void)

{
  char *pcVar1;
  longlong lVar2;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  void *_Dst;
  longlong extraout_RAX_01;
  longlong *in_RCX;
  undefined8 *in_RDX;
  longlong *local_1b8;
  code *local_1b0;
  longlong local_130 [6];
  longlong local_100;
  undefined8 local_f8;
  undefined **local_e8;
  undefined8 local_e0;
  undefined8 **local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined4 local_88;
  undefined1 local_68;
  longlong local_60 [2];
  undefined8 local_50;
  undefined8 local_48;
  longlong local_40;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  pcVar1 = (char *)*in_RDX;
  lVar2 = *in_RCX;
  if (*pcVar1 == '\x02') {
    country_core::input::input_stream::InputStream::sync_point_count();
    if (extraout_RAX < 2) {
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::raw_vec::handle_error();
      }
      memcpy(_Dst,&DAT_14293d588,0x8c);
      local_60[0] = 0x8c;
      local_50 = 0x8c;
    }
    else {
      country_core::input::input_stream::InputStream::sync_point_count();
      local_130[0] = extraout_RAX_00 + -1;
      local_1b8 = local_130;
      local_1b0 = core::fmt::num::imp::impl$9::fmt;
      local_e8 = &PTR_DAT_14293d700;
      local_e0 = 2;
      local_d8 = &local_1b8;
      uStack_d0 = 1;
      local_c8 = 0;
      alloc::fmt::format::format_inner();
    }
    local_1b8 = local_60;
    local_1b0 = alloc::string::impl$23::fmt;
    local_e8 = &PTR_DAT_14293d848;
    local_e0 = 2;
    local_d8 = &local_1b8;
    uStack_d0 = 1;
    local_c8 = 0;
    alloc::fmt::format::format_inner();
    local_40 = local_100;
    local_48 = local_f8;
    local_d8 = (undefined8 **)0x0;
    uStack_d0 = 0;
    local_e8 = (undefined **)0x0;
    local_e0 = 1;
    local_c8 = 1;
    local_c0 = 0;
    local_b8 = 0x8000000000000000;
    local_88 = 0xd;
    local_68 = 0;
    rfd::message_dialog::MessageDialog::set_title();
    rfd::message_dialog::MessageDialog::set_description();
    local_130[0] = -0x8000000000000000;
    rfd::message_dialog::MessageDialog::set_buttons();
    rfd::message_dialog::MessageDialog::show();
    *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
    *pcVar1 = '\x03';
    if (local_40 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_60[0] != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  if ((*(undefined1 **)(lVar2 + 0x40) == &DAT_8000000000000002) && (*pcVar1 == '\x01')) {
    lVar2 = *(longlong *)(lVar2 + 0x48);
    country_core::input::input_stream::InputStream::frame_count();
    if (lVar2 + 1 == extraout_RAX_01) {
      local_d8 = (undefined8 **)0x0;
      uStack_d0 = 0;
      local_e8 = (undefined **)0x0;
      local_e0 = 1;
      local_c8 = 1;
      local_c0 = 0;
      local_b8 = 0x8000000000000000;
      local_88 = 0xd;
      local_68 = 0;
      rfd::message_dialog::MessageDialog::set_title();
      rfd::message_dialog::MessageDialog::set_description();
      local_130[0] = -0x8000000000000000;
      rfd::message_dialog::MessageDialog::set_buttons();
      rfd::message_dialog::MessageDialog::show();
      *(undefined4 *)in_RDX[2] = *(undefined4 *)((longlong)in_RDX + 0x1c);
      *pcVar1 = '\x02';
    }
  }
  return;
}

