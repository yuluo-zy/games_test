
void __cdecl
country_core::systems::ui::resources::ui_state::UiState::clearing_canvas_w_layer_and_sound(void)

{
  longlong *plVar1;
  void *_Dst;
  longlong *extraout_RAX;
  longlong in_RCX;
  bool bVar2;
  undefined8 *in_stack_00000030;
  undefined *local_1d8;
  undefined8 local_1d0;
  undefined1 **local_1c8;
  undefined8 local_1c0;
  undefined8 local_1b8;
  longlong local_118;
  undefined *local_110;
  undefined8 local_108;
  undefined8 local_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined4 local_e8;
  undefined8 local_e4;
  undefined8 uStack_dc;
  undefined4 local_d4;
  undefined4 local_d0;
  undefined8 local_cc;
  undefined8 uStack_c4;
  undefined4 local_bc;
  undefined4 local_b8;
  undefined8 local_b4;
  undefined8 uStack_ac;
  undefined4 local_a4;
  undefined1 local_a0;
  undefined1 local_90;
  undefined8 local_8c;
  undefined2 local_70;
  undefined1 *local_68;
  code *local_60;
  undefined1 local_58 [16];
  longlong *local_48;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  local_68 = local_58;
  local_60 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_1d8 = &DAT_142aafe50;
  local_1d0 = 2;
  local_1c8 = &local_68;
  local_1c0 = 1;
  local_1b8 = 0;
  alloc::fmt::format::format_inner();
  local_48 = (longlong *)local_110;
  local_1d8 = local_110;
  local_1d0 = local_108;
  ::utils::calculate_hash<tuple_<ref_<str_>,ref_<tuple_<>_>_>_>();
  if (local_118 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_e8 = *(undefined4 *)(in_stack_00000030 + 3);
  local_d4 = *(undefined4 *)((longlong)in_stack_00000030 + 0x2c);
  local_e4 = *(undefined8 *)((longlong)in_stack_00000030 + 0x1c);
  uStack_dc = *(undefined8 *)((longlong)in_stack_00000030 + 0x24);
  local_d0 = *(undefined4 *)(in_stack_00000030 + 6);
  local_cc = *(undefined8 *)((longlong)in_stack_00000030 + 0x34);
  uStack_c4 = *(undefined8 *)((longlong)in_stack_00000030 + 0x3c);
  local_bc = *(undefined4 *)((longlong)in_stack_00000030 + 0x44);
  local_b8 = *(undefined4 *)(in_stack_00000030 + 9);
  local_b4 = *(undefined8 *)((longlong)in_stack_00000030 + 0x4c);
  uStack_ac = *(undefined8 *)((longlong)in_stack_00000030 + 0x54);
  local_a4 = *(undefined4 *)((longlong)in_stack_00000030 + 0x5c);
  local_118 = 0;
  local_110 = (undefined *)0x8;
  local_108 = 0;
  local_100 = *in_stack_00000030;
  local_f8 = in_stack_00000030[1];
  uStack_f0 = in_stack_00000030[2];
  local_a0 = 1;
  local_90 = 4;
  local_8c = 0x4302000043020000;
  local_70 = 0x100;
  LOCK();
  bVar2 = *(longlong *)(in_RCX + 0x60) == 0;
  if (bVar2) {
    *(longlong *)(in_RCX + 0x60) = 8;
  }
  UNLOCK();
  if (!bVar2) {
    local_39 = 1;
    parking_lot::raw_rwlock::RawRwLock::lock_exclusive_slow();
  }
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_39 = 1;
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_48 = (longlong *)(in_RCX + 0x60);
  memcpy(&local_1c8,&local_118,0xb0);
  local_1d8 = (undefined *)0x1;
  local_1d0 = 1;
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
    alloc::alloc::handle_alloc_error();
  }
  memcpy(_Dst,&local_1d8,0xc0);
  hashbrown::map::HashMap::
  insert<country_core::systems::ui::resources::ui_state::WidgetId,alloc::sync::Arc<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (extraout_RAX != (longlong *)0x0) {
    LOCK();
    *extraout_RAX = *extraout_RAX + -1;
    UNLOCK();
    if (*extraout_RAX == 0) {
      alloc::sync::Arc::
      drop_slow<country_core::systems::ui::resources::widget_params::WidgetParams,alloc::alloc::Global>
                ();
    }
  }
  plVar1 = local_48;
  local_39 = 0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  local_39 = 0;
  parking_lot_core::parking_lot::deadlock_impl::release_resource();
  LOCK();
  bVar2 = *plVar1 == 8;
  if (bVar2) {
    *plVar1 = 0;
  }
  UNLOCK();
  if (!bVar2) {
    local_39 = 0;
    parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow();
  }
  return;
}

