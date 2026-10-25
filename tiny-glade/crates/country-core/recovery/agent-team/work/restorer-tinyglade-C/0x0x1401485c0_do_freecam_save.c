// do_freecam_save @ 0x0x1401485c0
// sig: void __cdecl do_freecam_save(void)
// size: 1081


// WARNING: Restarted to delay deadcode elimination for space: stack

void __cdecl tiny_glade::gui::gui_debug::do_freecam_save(void)

{
  longlong *plVar1;
  char extraout_AL;
  char extraout_AL_00;
  undefined8 *extraout_RAX;
  int *in_RCX;
  longlong in_R8;
  int iVar2;
  int extraout_XMM0_Da;
  int iVar3;
  ulonglong local_360;
  int iStack_358;
  int local_350;
  undefined4 uStack_34c;
  undefined8 uStack_348;
  undefined4 uStack_318;
  undefined4 uStack_314;
  undefined4 local_310;
  undefined4 uStack_30c;
  undefined8 uStack_308;
  undefined1 local_300;
  undefined4 uStack_2ff;
  undefined4 uStack_2fb;
  undefined3 uStack_2f7;
  undefined4 uStack_2f4;
  ulonglong local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  ulonglong *local_b0;
  code *local_a8;
  longlong *local_a0;
  char local_5d;
  char local_5c;
  undefined8 local_50;
  int local_48;
  undefined1 local_42;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  iVar2 = *in_RCX;
  if (iVar2 == 1) {
    local_118 = *(undefined8 *)(in_RCX + 5);
    local_128 = *(ulonglong *)(in_RCX + 1);
    uStack_120 = *(undefined8 *)(in_RCX + 3);
    local_b0 = &local_128;
    local_a8 = core::fmt::impl$98::fmt<glam::f32::vec3::Vec3,f32,f32,f32>;
    alloc::fmt::format::format_inner();
    local_300 = 0;
    uStack_2fb = 0;
    uStack_2f7 = 0;
    local_360 = 3;
    uStack_348 = 6;
    uStack_318 = 0;
    local_310 = 0;
    uStack_308 = 0;
    egui::widgets::label::impl$2::ui();
    LOCK();
    local_b0 = (ulonglong *)((longlong)local_b0 + -1);
    UNLOCK();
    if (local_b0 == (ulonglong *)0x0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  local_41 = 0;
  core::convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  local_c8 = CONCAT35((undefined3)uStack_2fb,CONCAT41(uStack_2ff,local_300));
  uStack_c0 = CONCAT44(uStack_2f4,CONCAT31(uStack_2f7,uStack_2fb._3_1_));
  local_d8 = CONCAT44(uStack_30c,local_310);
  uStack_d0 = uStack_308;
  uStack_e0 = CONCAT44(uStack_314,uStack_318);
  uStack_110 = uStack_348;
  local_128 = local_360;
  local_41 = 1;
  iVar3 = local_350;
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if (local_5c == '\0') {
    local_360 = local_360 & 0xffffffffffffff00;
    if (local_5d != '\0') {
      egui::context::Context::
      write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                ();
      if (extraout_AL != '\0') goto LAB_14014879d;
    }
    LOCK();
    *local_a0 = *local_a0 + -1;
    UNLOCK();
    if (*local_a0 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
LAB_14014879d:
    LOCK();
    *local_a0 = *local_a0 + -1;
    UNLOCK();
    if (*local_a0 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    country_core::systems::camera_rig::free::FreeCamera::get_position();
    country_core::systems::camera_rig::free::FreeCamera::get_yaw_pitch();
    iVar2 = *(int *)(in_R8 + 0x2c8);
    *in_RCX = 1;
    in_RCX[3] = iStack_358;
    *(ulonglong *)(in_RCX + 1) = local_360;
    in_RCX[4] = extraout_XMM0_Da;
    in_RCX[5] = iVar3;
    in_RCX[6] = iVar2;
    iVar2 = 1;
  }
  local_42 = 0;
  core::convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  local_c8 = CONCAT35((undefined3)uStack_2fb,CONCAT41(uStack_2ff,local_300));
  uStack_c0 = CONCAT44(uStack_2f4,CONCAT31(uStack_2f7,uStack_2fb._3_1_));
  local_d8 = CONCAT44(uStack_30c,local_310);
  uStack_d0 = uStack_308;
  uStack_e0 = CONCAT44(uStack_314,uStack_318);
  uStack_110 = uStack_348;
  local_128 = local_360;
  local_42 = 1;
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if (local_5c == '\0') {
    if (local_5d != '\0') {
      egui::context::Context::
      write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                ();
      if (extraout_AL_00 != '\0') goto LAB_1401488ee;
    }
    LOCK();
    *local_a0 = *local_a0 + -1;
    UNLOCK();
    if (*local_a0 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
LAB_1401488ee:
    LOCK();
    *local_a0 = *local_a0 + -1;
    UNLOCK();
    if (*local_a0 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    if ((char)iVar2 != '\0') {
      local_48 = in_RCX[3];
      local_50 = *(undefined8 *)(in_RCX + 1);
      iVar2 = in_RCX[6];
      country_core::systems::camera_rig::free::FreeCamera::set();
      *(int *)(in_R8 + 0x2cc) = iVar2;
    }
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (undefined8 *)0x0) {
    *extraout_RAX = in_RCX;
    egui::ui::Ui::horizontal_with_main_wrap_dyn<tuple_<>_>();
    plVar1 = (longlong *)CONCAT44(uStack_34c,local_350);
    LOCK();
    *plVar1 = *plVar1 + -1;
    UNLOCK();
    if (*plVar1 == 0) {
      alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::alloc::handle_alloc_error();
}

