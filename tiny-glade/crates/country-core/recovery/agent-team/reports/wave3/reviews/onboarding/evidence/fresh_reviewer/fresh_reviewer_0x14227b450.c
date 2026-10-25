
void __cdecl system_onboarding::hints_startup::hints_startup(void)

{
  undefined8 *puVar1;
  undefined1 uVar2;
  char cVar3;
  longlong lVar4;
  longlong lVar5;
  longlong lVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong *in_RCX;
  longlong in_RDX;
  longlong *plVar9;
  undefined4 uVar10;
  undefined4 uVar11;
  undefined4 uVar12;
  undefined8 local_68;
  undefined8 uStack_60;
  
  lVar4 = *in_RCX;
  plVar9 = *(longlong **)(in_RDX + 8);
  if (plVar9 == (longlong *)0x0) {
    plVar9 = *(longlong **)(in_RDX + 0x10);
  }
  uVar2 = *(undefined1 *)(lVar4 + 500);
  lVar5 = plVar9[2];
  if ((ulonglong)(*plVar9 - lVar5) < 0x18) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_deselect_tool::DeselectToolHintConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar6 + 8 + lVar5) = 0;
  *(undefined1 *)(lVar6 + 0x10 + lVar5) = uVar2;
  *(undefined1 *)(lVar6 + 0x11 + lVar5) = 0;
  plVar9[2] = lVar5 + 0x18;
  uVar2 = *(undefined1 *)(lVar4 + 0x1f5);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x18)) < 0x18) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x18 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_focused_edit::FocusedEditHintConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar6 + 0x20 + lVar5) = 0;
  *(undefined1 *)(lVar6 + 0x28 + lVar5) = uVar2;
  *(undefined1 *)(lVar6 + 0x29 + lVar5) = 0;
  plVar9[2] = lVar5 + 0x30;
  uVar2 = *(undefined1 *)(lVar4 + 0x1f9);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x30)) < 0x18) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x30 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_terrain::HintTerrainConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar6 + 0x38 + lVar5) = 0;
  *(undefined1 *)(lVar6 + 0x40 + lVar5) = uVar2;
  plVar9[2] = lVar5 + 0x48;
  uVar2 = *(undefined1 *)(lVar4 + 0x1fa);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x48)) < 10) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x48 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_terrain_steepness::HintTerrainSteepnessConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined1 *)(lVar6 + 0x50 + lVar5) = uVar2;
  *(undefined1 *)(lVar6 + 0x51 + lVar5) = uVar2;
  plVar9[2] = lVar5 + 0x52;
  uVar2 = *(undefined1 *)(lVar4 + 0x1fc);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x52)) < 9) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x52 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_disable_snapping::HintDisableSnappingConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined1 *)(lVar6 + 0x5a + lVar5) = uVar2;
  plVar9[2] = lVar5 + 0x5b;
  if ((ulonglong)(*plVar9 - (lVar5 + 0x5b)) < 0x10) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x5b + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_undo::UndoHintConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined4 *)(lVar6 + 99 + lVar5) = 0;
  *(undefined1 *)(lVar6 + 0x67 + lVar5) = 0;
  plVar9[2] = lVar5 + 0x6b;
  uVar2 = *(undefined1 *)(lVar4 + 0x1ff);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x6b)) < 10) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x6b + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_roof_shape::HintRoofShapeConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined1 *)(lVar6 + 0x73 + lVar5) = uVar2;
  *(undefined1 *)(lVar6 + 0x74 + lVar5) = uVar2;
  plVar9[2] = lVar5 + 0x75;
  uVar2 = *(undefined1 *)(lVar4 + 0x200);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x75)) < 10) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x75 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_clutter_focus_edit::FocusedClutterHintConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined1 *)(lVar6 + 0x7d + lVar5) = uVar2;
  *(undefined1 *)(lVar6 + 0x7e + lVar5) = 0;
  lVar6 = lVar5 + 0x7f;
  plVar9[2] = lVar6;
  if (((*(char *)(lVar4 + 0x1f6) != '\x01') || (*(char *)(lVar4 + 0x1f7) == '\0')) ||
     (uVar10 = 2, *(char *)(lVar4 + 0x1f8) == '\0')) {
    local_68 = 0;
    uStack_60 = 0;
    uVar10 = 0;
  }
  if ((ulonglong)(*plVar9 - lVar6) < 0x1c) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar7 = plVar9[1];
  *(code **)(lVar7 + 0x7f + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<enum2_<system_onboarding::hint_camera_controls::CameraHintsState>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined4 *)(lVar7 + 0x87 + lVar5) = uVar10;
  puVar1 = (undefined8 *)(lVar7 + 0xc + lVar6);
  *puVar1 = local_68;
  puVar1[1] = uStack_60;
  plVar9[2] = lVar5 + 0x9b;
  uVar2 = *(undefined1 *)(lVar4 + 0x1fb);
  if ((ulonglong)(*plVar9 - (lVar5 + 0x9b)) < 9) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar6 = plVar9[1];
  *(code **)(lVar6 + 0x9b + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_terrain_focus::HintFocusTerrainConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined1 *)(lVar6 + 0xa3 + lVar5) = uVar2;
  lVar6 = lVar5 + 0xa4;
  plVar9[2] = lVar6;
  uVar10 = 0x3f800000;
  uVar11 = 0x3f800000;
  if (*(char *)(lVar4 + 0x1f6) == '\0') {
    uVar11 = 0;
    cVar3 = *(char *)(lVar4 + 0x1f8);
  }
  else {
    cVar3 = *(char *)(lVar4 + 0x1f8);
  }
  if (cVar3 == '\0') {
    uVar12 = 0;
    cVar3 = *(char *)(lVar4 + 0x1f7);
  }
  else {
    uVar12 = 0x3f800000;
    cVar3 = *(char *)(lVar4 + 0x1f7);
  }
  if (cVar3 == '\0') {
    uVar10 = 0;
    uVar8 = *plVar9 - lVar6;
  }
  else {
    uVar8 = *plVar9 - lVar6;
  }
  if (uVar8 < 0x2c) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar4 = plVar9[1];
  *(code **)(lVar4 + 0xa4 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_camera_progress::CameraHintProgress>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined4 *)(lVar4 + 0xac + lVar5) = uVar11;
  *(undefined4 *)(lVar4 + 0xb0 + lVar5) = uVar10;
  *(undefined4 *)(lVar4 + 0xb4 + lVar5) = uVar12;
  *(undefined4 *)(lVar4 + 0xb8 + lVar5) = uVar11;
  *(undefined4 *)(lVar4 + 0xbc + lVar5) = uVar10;
  *(undefined4 *)(lVar4 + 0xc0 + lVar5) = uVar12;
  *(undefined8 *)(lVar4 + 0xc4 + lVar5) = 0x3f3333333f333333;
  *(undefined4 *)(lVar4 + 0xcc + lVar5) = 0x42700000;
  plVar9[2] = lVar5 + 0xd0;
  if ((ulonglong)(*plVar9 - (lVar5 + 0xd0)) < 0x18) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar4 = plVar9[1];
  *(code **)(lVar4 + 0xd0 + lVar5) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<system_onboarding::hint_stairs::StairsHintConditions>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  *(undefined8 *)(lVar4 + 0xd8 + lVar5) = 0;
  *(undefined4 *)(lVar4 + 0xe0 + lVar5) = 0;
  plVar9[2] = lVar5 + 0xe8;
  return;
}

