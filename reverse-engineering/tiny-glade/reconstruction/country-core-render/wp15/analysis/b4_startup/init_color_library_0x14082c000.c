// addr=0x14082c000 name=init_color_library
// signature=void __cdecl init_color_library(void)


void __cdecl country_core::startup::init_color_library::init_color_library(void)

{
  longlong lVar1;
  longlong lVar2;
  char *extraout_RAX;
  longlong *plVar3;
  longlong in_R8;
  longlong in_R9;
  undefined8 local_4d0 [2];
  undefined8 local_4c0;
  undefined1 local_280 [592];
  
  asset_libraries::color_library::impl$3::default();
  memcpy(local_4d0,local_280,0x250);
  asset_libraries::ssbo_library::AssetSsboLibrary::
  add_new_with_data<country_core::asset_libraries::color_library::ColorLibrarySsbo>();
  plVar3 = *(longlong **)(in_R9 + 8);
  if (plVar3 == (longlong *)0x0) {
    plVar3 = *(longlong **)(in_R9 + 0x10);
  }
  lVar1 = plVar3[2];
  if ((ulonglong)(*plVar3 - lVar1) < 600) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar2 = plVar3[1];
  *(code **)(lVar2 + lVar1) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<country_core::asset_libraries::color_library::ColorLibrarySsbo>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  memcpy((void *)(lVar2 + lVar1 + 8),local_280,0x250);
  plVar3[2] = lVar1 + 600;
  **(undefined4 **)(in_R8 + 0x10) = *(undefined4 *)(in_R8 + 0x1c);
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX != (char *)0x0) {
    builtin_strncpy(extraout_RAX,"assets/colors.json",0x12);
    local_4d0[0] = 0x12;
    local_4c0 = 0x12;
    asset_libraries::configwatch::ConfigWatch::watch();
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


