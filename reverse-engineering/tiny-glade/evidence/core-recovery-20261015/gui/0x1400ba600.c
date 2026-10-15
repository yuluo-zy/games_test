
void __cdecl
core::ops::function::FnOnce::
call_once<tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_env_0,tuple_<ref_mut_<egui::ui::Ui>_>_>
          (void)

{
  longlong *plVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  char extraout_AL;
  char extraout_AL_00;
  longlong *in_RCX;
  ulonglong uVar8;
  longlong *plVar9;
  longlong *local_f0;
  char local_ad;
  char local_ac;
  longlong local_a0;
  longlong *local_98;
  ulonglong local_90;
  longlong *local_80;
  
  plVar9 = (longlong *)*in_RCX;
  plVar1 = (longlong *)in_RCX[1];
  convert::impl$3::into<ref_<str_>,enum2_<egui::widget_text::WidgetText>_>();
  egui::widgets::button::Button::opt_image_and_text();
  egui::widgets::button::impl$1::ui();
  if ((local_ac == '\0') &&
     ((local_ad == '\0' ||
      (egui::context::Context::
       write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                 (), extraout_AL == '\0')))) {
    LOCK();
    *local_f0 = *local_f0 + -1;
    UNLOCK();
    if (*local_f0 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
  }
  else {
    LOCK();
    *local_f0 = *local_f0 + -1;
    UNLOCK();
    if (*local_f0 == 0) {
      ::alloc::sync::Arc::
      drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                ();
    }
    *(undefined4 *)plVar9[2] = *(undefined4 *)((longlong)plVar9 + 0x1c);
    lVar2 = *plVar9;
    lVar3 = *(longlong *)(lVar2 + 0x30);
    uVar4 = *(undefined8 *)(lVar2 + 0x40);
    if (lVar3 == *(longlong *)(lVar2 + 0x20)) {
      ::alloc::raw_vec::RawVec::
      grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                ();
    }
    *(undefined8 *)(*(longlong *)(lVar2 + 0x28) + lVar3 * 8) = uVar4;
    *(longlong *)(lVar2 + 0x30) = lVar3 + 1;
    *(longlong *)(lVar2 + 0x40) = *(longlong *)(lVar2 + 0x40) + 1;
  }
  _ZN100__LT_country_core__components__drawable__WireframePass_u20_as_u20_bevy_ecs__component__Component_GT_14clone_behavior17h98754d446e2f5bc0E
            ();
  ::alloc::vec::spec_from_iter::impl$0::
  from_iter<tuple_<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,core::iter::adapters::filter_map::FilterMap<enum_iterator::All<country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_0::closure_env_0>_>
            ();
  if (1 < local_90) {
    if (local_90 < 0x15) {
      plVar9 = local_98 + 4;
      do {
        slice::sort::shared::smallsort::
        insert_tail<tuple_<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,alloc::slice::impl_0::sort_by::closure_env_0<tuple_<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_0::closure_env_1>_>
                  ();
        plVar9 = plVar9 + 4;
      } while (plVar9 != local_98 + local_90 * 4);
    }
    else {
      slice::sort::stable::
      driftsort_main<tuple_<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,alloc::slice::impl_0::sort_by::closure_env_0<tuple_<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,tiny_glade::gui::gui_debug::do_gui_debug::closure_0::closure_0::closure_0::closure_env_1>,alloc::vec::Vec<tuple_<alloc::string::String,country_core::resources::clutter_meshes::ClutterMeshTy>,alloc::alloc::Global>_>
                ();
    }
  }
  local_80 = local_98;
  while (plVar9 = local_80, local_80 != local_98 + local_90 * 4) {
    plVar9 = local_80 + 4;
    lVar2 = *local_80;
    if (SBORROW8(0,lVar2)) break;
    lVar3 = local_80[3];
    ::alloc::fmt::format::format_inner();
    egui::widgets::button::Button::opt_image_and_text();
    egui::widgets::button::impl$1::ui();
    if ((local_ac == '\0') &&
       ((local_ad == '\0' ||
        (egui::context::Context::
         write<bool,egui::context::impl_13::input::closure_env_0<bool,egui::response::impl_0::clicked_by::closure_env_0>_>
                   (), extraout_AL_00 == '\0')))) {
      LOCK();
      *local_f0 = *local_f0 + -1;
      UNLOCK();
      if (*local_f0 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
    }
    else {
      LOCK();
      *local_f0 = *local_f0 + -1;
      UNLOCK();
      if (*local_f0 == 0) {
        ::alloc::sync::Arc::
        drop_slow<epaint::mutex::rw_lock_impl::RwLock<egui::context::ContextImpl>,alloc::alloc::Global>
                  ();
      }
      *(undefined4 *)plVar1[2] = *(undefined4 *)((longlong)plVar1 + 0x1c);
      lVar5 = *plVar1;
      lVar6 = *(longlong *)(lVar5 + 0x30);
      uVar4 = *(undefined8 *)(lVar5 + 0x40);
      if (lVar6 == *(longlong *)(lVar5 + 0x20)) {
        ::alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      lVar7 = *(longlong *)(lVar5 + 0x28);
      *(undefined8 *)(lVar7 + lVar6 * 0x10) = uVar4;
      *(char *)(lVar7 + 8 + lVar6 * 0x10) = (char)lVar3;
      *(longlong *)(lVar5 + 0x30) = lVar6 + 1;
      *(longlong *)(lVar5 + 0x40) = *(longlong *)(lVar5 + 0x40) + 1;
    }
    local_80 = plVar9;
    if (lVar2 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  local_80 = plVar9;
  uVar8 = (longlong)(local_98 + local_90 * 4) - (longlong)local_80;
  if (uVar8 != 0) {
    uVar8 = uVar8 >> 5;
    local_80 = local_80 + 1;
    do {
      if (local_80[-1] != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_80 = local_80 + 4;
      uVar8 = uVar8 - 1;
    } while (uVar8 != 0);
  }
  if (local_a0 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}


