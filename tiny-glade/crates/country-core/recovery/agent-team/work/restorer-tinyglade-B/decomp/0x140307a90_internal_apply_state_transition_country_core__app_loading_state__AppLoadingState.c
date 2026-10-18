// internal_apply_state_transition<country_core::app_loading_state::AppLoadingState> @ 0x140307a90
// sig: void __cdecl internal_apply_state_transition<country_core::app_loading_state::AppLoadingState>(void)


void __cdecl
bevy_state::state::transitions::
internal_apply_state_transition<country_core::app_loading_state::AppLoadingState>(void)

{
  char *pcVar1;
  longlong lVar2;
  longlong *in_RCX;
  longlong lVar3;
  longlong in_RDX;
  char cVar4;
  longlong *plVar5;
  longlong lVar6;
  undefined8 *in_R8;
  char in_R9B;
  longlong lVar7;
  undefined8 uVar8;
  
  pcVar1 = (char *)*in_R8;
  if (in_R9B == '\x03') {
    if (pcVar1 == (char *)0x0) {
      return;
    }
    plVar5 = *(longlong **)(in_RDX + 8);
    if (plVar5 == (longlong *)0x0) {
      plVar5 = *(longlong **)(in_RDX + 0x10);
    }
    lVar6 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar6) < 8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    *(code **)(plVar5[1] + lVar6) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::remove_resource::closure_env_0<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    plVar5[2] = lVar6 + 8;
    cVar4 = *pcVar1;
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    lVar6 = *in_RCX;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    in_R9B = '\x03';
    if (lVar7 != *(longlong *)(lVar6 + 0x20)) goto LAB_140307ab1;
  }
  else if (pcVar1 == (char *)0x0) {
    plVar5 = *(longlong **)(in_RDX + 8);
    if (plVar5 == (longlong *)0x0) {
      plVar5 = *(longlong **)(in_RDX + 0x10);
    }
    lVar6 = plVar5[2];
    if ((ulonglong)(*plVar5 - lVar6) < 9) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar7 = plVar5[1];
    *(code **)(lVar7 + lVar6) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<bevy_state::state::resources::State<country_core::app_loading_state::AppLoadingState>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(char *)(lVar7 + 8 + lVar6) = in_R9B;
    plVar5[2] = lVar6 + 9;
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    lVar6 = *in_RCX;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    cVar4 = '\x03';
    if (lVar7 != *(longlong *)(lVar6 + 0x20)) goto LAB_140307ab1;
  }
  else {
    cVar4 = in_R9B;
    if (*pcVar1 != in_R9B) {
      *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
      cVar4 = *pcVar1;
      *pcVar1 = in_R9B;
    }
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    lVar6 = *in_RCX;
    lVar7 = *(longlong *)(lVar6 + 0x30);
    uVar8 = *(undefined8 *)(lVar6 + 0x40);
    if (lVar7 != *(longlong *)(lVar6 + 0x20)) goto LAB_140307ab1;
  }
  alloc::raw_vec::RawVec::
  grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
            ();
LAB_140307ab1:
  lVar2 = *(longlong *)(lVar6 + 0x28);
  lVar3 = lVar7 * 0x10;
  *(undefined8 *)(lVar2 + lVar3) = uVar8;
  *(char *)(lVar2 + 8 + lVar3) = cVar4;
  *(char *)(lVar2 + 9 + lVar3) = in_R9B;
  *(longlong *)(lVar6 + 0x30) = lVar7 + 1;
  *(longlong *)(lVar6 + 0x40) = *(longlong *)(lVar6 + 0x40) + 1;
  return;
}

