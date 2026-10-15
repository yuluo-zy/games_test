
void __cdecl tiny_glade::gui::gui_debug::do_gui_debug(void)

{
  longlong lVar1;
  char extraout_AL;
  byte bVar2;
  char extraout_AL_00;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *in_RCX;
  undefined8 *in_RDX;
  longlong in_R8;
  longlong in_R9;
  undefined1 auVar3 [16];
  longlong in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  longlong in_stack_00000040;
  longlong in_stack_00000050;
  undefined1 local_600 [392];
  longlong local_478;
  longlong local_470;
  longlong local_468;
  longlong *local_2e0;
  int local_2b0;
  undefined8 local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined8 uStack_250;
  undefined8 local_248;
  undefined8 uStack_240;
  undefined8 local_238;
  undefined8 uStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  longlong local_208;
  longlong lStack_200;
  longlong local_1f8;
  longlong lStack_1f0;
  longlong local_1e8;
  longlong lStack_1e0;
  longlong local_1d8;
  longlong lStack_1d0;
  longlong local_1c8;
  longlong lStack_1c0;
  longlong local_1b8;
  longlong lStack_1b0;
  longlong local_1a8;
  longlong lStack_1a0;
  longlong local_198;
  longlong lStack_190;
  longlong local_188;
  longlong lStack_180;
  longlong local_178;
  longlong lStack_170;
  longlong local_168;
  longlong lStack_160;
  longlong local_158;
  longlong lStack_150;
  longlong local_148;
  longlong lStack_140;
  longlong local_138;
  longlong lStack_130;
  longlong local_128;
  longlong lStack_120;
  longlong local_118;
  longlong lStack_110;
  longlong local_108;
  longlong lStack_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  longlong local_c8;
  longlong lStack_c0;
  longlong local_b8;
  longlong lStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  undefined8 local_80;
  longlong local_78;
  longlong lStack_70;
  undefined4 *local_68;
  undefined8 uStack_60;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_78 = *in_RCX;
  lStack_70 = in_RCX[1];
  local_68 = (undefined4 *)in_RCX[2];
  uStack_60 = in_RCX[3];
  local_e8 = in_RCX[4];
  lStack_e0 = in_RCX[5];
  local_d8 = in_RCX[6];
  lStack_d0 = in_RCX[7];
  local_108 = in_RCX[8];
  lStack_100 = in_RCX[9];
  local_f8 = in_RCX[10];
  lStack_f0 = in_RCX[0xb];
  lVar1 = in_RCX[0xc];
  local_128 = in_RCX[0x10];
  lStack_120 = in_RCX[0x11];
  local_118 = in_RCX[0x12];
  lStack_110 = in_RCX[0x13];
  local_148 = in_RCX[0x14];
  lStack_140 = in_RCX[0x15];
  local_138 = in_RCX[0x16];
  lStack_130 = in_RCX[0x17];
  local_a8 = in_RCX[0x18];
  lStack_a0 = in_RCX[0x19];
  local_98 = in_RCX[0x1a];
  lStack_90 = in_RCX[0x1b];
  local_168 = in_RCX[0x1c];
  lStack_160 = in_RCX[0x1d];
  local_158 = in_RCX[0x1e];
  lStack_150 = in_RCX[0x1f];
  local_188 = in_RCX[0x20];
  lStack_180 = in_RCX[0x21];
  local_178 = in_RCX[0x22];
  lStack_170 = in_RCX[0x23];
  local_1a8 = in_RCX[0x24];
  lStack_1a0 = in_RCX[0x25];
  local_198 = in_RCX[0x26];
  lStack_190 = in_RCX[0x27];
  local_c8 = in_RCX[0x28];
  lStack_c0 = in_RCX[0x29];
  local_b8 = in_RCX[0x2a];
  lStack_b0 = in_RCX[0x2b];
  local_1c8 = in_RCX[0x2c];
  lStack_1c0 = in_RCX[0x2d];
  local_1b8 = in_RCX[0x2e];
  lStack_1b0 = in_RCX[0x2f];
  local_1e8 = in_RCX[0x34];
  lStack_1e0 = in_RCX[0x35];
  local_1d8 = in_RCX[0x36];
  lStack_1d0 = in_RCX[0x37];
  local_208 = in_RCX[0x38];
  lStack_200 = in_RCX[0x39];
  local_1f8 = in_RCX[0x3a];
  lStack_1f0 = in_RCX[0x3b];
  local_228 = *in_RDX;
  uStack_220 = in_RDX[1];
  local_218 = in_RDX[2];
  uStack_210 = in_RDX[3];
  local_248 = in_RDX[4];
  uStack_240 = in_RDX[5];
  local_238 = in_RDX[6];
  uStack_230 = in_RDX[7];
  local_268 = in_RDX[0xc];
  uStack_260 = in_RDX[0xd];
  local_258 = in_RDX[0xe];
  uStack_250 = in_RDX[0xf];
  local_288 = in_RDX[0x10];
  uStack_280 = in_RDX[0x11];
  local_278 = in_RDX[0x12];
  uStack_270 = in_RDX[0x13];
  local_80 = in_RDX[0x18];
  if (in_RDX[0x14] != 0) {
    *(undefined4 *)in_RCX[0xe] = *(undefined4 *)((longlong)in_RCX + 0x7c);
    country_core::input::action_set::DigitalActionState::just_pressed();
    if (extraout_AL == '\0') {
      bVar2 = *(byte *)(local_78 + 0xf0);
    }
    else {
      *local_68 = uStack_60._4_4_;
      bVar2 = *(byte *)(local_78 + 0xf0) ^ 1;
      *(byte *)(local_78 + 0xf0) = bVar2;
    }
    if ((bVar2 & 1) != 0) {
      egui::containers::window::Window::new<ref_<str_>_>();
      local_49 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      memcpy(&local_478,local_600,0x188);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      auVar3._8_4_ = (int)lVar1;
      auVar3._0_8_ = lVar1;
      auVar3._12_4_ = (int)((ulonglong)lVar1 >> 0x20);
      *extraout_RAX = (longlong)&local_1e8;
      extraout_RAX[1] = (longlong)&local_208;
      extraout_RAX[2] = (longlong)&stack0x00000048;
      extraout_RAX[3] = (longlong)&local_a8;
      extraout_RAX[4] = (longlong)&local_1a8;
      extraout_RAX[5] = (longlong)&local_78;
      extraout_RAX[6] = (longlong)&local_268;
      extraout_RAX[7] = (longlong)&local_80;
      extraout_RAX[8] = (longlong)&local_288;
      extraout_RAX[9] = in_stack_00000040;
      extraout_RAX[10] = (longlong)&stack0x00000038;
      extraout_RAX[0xb] = (longlong)&local_c8;
      extraout_RAX[0xc] = (longlong)&local_1c8;
      extraout_RAX[0xd] = (longlong)&local_168;
      extraout_RAX[0x10] = lVar1 + 0x10;
      extraout_RAX[0x11] = auVar3._8_8_ + 0x28;
      extraout_RAX[0xe] = lVar1 + 8;
      extraout_RAX[0xf] = auVar3._8_8_ + 0xc;
      extraout_RAX[0x12] = lVar1 + 0x14;
      extraout_RAX[0x13] = lVar1 + 0x18;
      extraout_RAX[0x14] = lVar1 + 0x1c;
      extraout_RAX[0x15] = in_R8;
      extraout_RAX[0x16] = in_stack_00000050;
      extraout_RAX[0x17] = (longlong)&local_128;
      extraout_RAX[0x18] = (longlong)&local_148;
      extraout_RAX[0x19] = in_stack_00000030;
      extraout_RAX[0x1a] = (longlong)&local_e8;
      extraout_RAX[0x1b] = (longlong)&local_248;
      extraout_RAX[0x1c] = (longlong)&local_228;
      extraout_RAX[0x1d] = in_R9;
      extraout_RAX[0x1e] = in_stack_00000028;
      extraout_RAX[0x1f] = (longlong)&local_188;
      extraout_RAX[0x20] = (longlong)&local_108;
      egui::containers::window::Window::show_dyn<tuple_<>_>();
      if (local_2b0 != 2) {
        LOCK();
        *local_2e0 = *local_2e0 + -1;
        UNLOCK();
        if (*local_2e0 == 0) {
          alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
      }
    }
    country_core::input::action_set::DigitalActionState::just_pressed();
    if (extraout_AL_00 != '\0') {
      bVar2 = *(byte *)(local_78 + 0x101);
      *local_68 = uStack_60._4_4_;
      *(byte *)(local_78 + 0x101) = bVar2 ^ 1;
    }
    bVar2 = 0;
    if (*(char *)(local_78 + 0x101) != '\0') {
      egui::containers::window::Window::new<ref_<str_>_>();
      local_4a = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      memcpy(&local_478,local_600,0x188);
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_00 = (longlong)&local_a8;
      extraout_RAX_00[1] = (longlong)&local_78;
      extraout_RAX_00[2] = in_R9;
      egui::containers::window::Window::show_dyn<tuple_<>_>();
      if (local_2b0 != 2) {
        LOCK();
        *local_2e0 = *local_2e0 + -1;
        UNLOCK();
        if (*local_2e0 == 0) {
          alloc::sync::Arc::
          drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                    ();
        }
      }
      bVar2 = *(byte *)(local_78 + 0x101);
    }
    if (((bVar2 & 1) != 0) || (*(char *)(local_78 + 0xf0) != '\0')) {
      country_core::render::impl$0::deref();
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_470 = in_stack_00000038 + 0x48;
      local_478 = in_stack_00000038;
      local_468 = in_stack_00000038 + 0x49;
      rhapsody::raw::device::GraphicsDevice::
      with_global_allocator<tiny_glade::gui::gui_debug::render_gpu_allocator_gui::closure_env_0,tuple_<>_>
                ();
    }
  }
  return;
}


