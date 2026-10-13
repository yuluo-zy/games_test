// addr=0x140b0e0f0 name=load_audio
// signature=void __cdecl load_audio(void)


void __cdecl country_core::startup::load_audio::load_audio(void)

{
  longlong lVar1;
  undefined8 *extraout_RAX;
  undefined4 *extraout_RAX_00;
  longlong in_RCX;
  undefined4 uVar2;
  longlong in_RDX;
  longlong lVar3;
  longlong *plVar4;
  undefined1 *local_b8;
  code *local_b0;
  undefined1 local_a8 [8];
  undefined4 local_a0;
  undefined8 local_98;
  char *local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 uStack_70;
  ulonglong local_68;
  char *local_60;
  undefined8 local_58;
  undefined4 local_50;
  undefined4 local_4c;
  undefined *local_48;
  undefined8 local_40;
  undefined1 **local_38;
  undefined8 local_30;
  undefined8 local_28;
  
  lVar3 = in_RDX;
  std::time::Instant::now();
  uVar2 = (undefined4)lVar3;
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  resources::audio_resources::AudioSamplesLoadedPromise::recv();
  if (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
    std::time::Instant::elapsed();
    local_b8 = local_a8;
    local_b0 = core::time::impl$12::fmt;
    local_a0 = uVar2;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_78 = *extraout_RAX;
    uStack_70 = extraout_RAX[1];
    local_4c = *(undefined4 *)(extraout_RAX + 2);
    local_68 = 3;
    local_60 = "country_core::startup::load_audio";
    local_58 = 0x21;
    local_48 = &anon_d3dffccd918481e721e2ae8aefb299e1_38_llvm_9005556892098412074;
    local_40 = 2;
    local_38 = &local_b8;
    local_30 = 1;
    local_28 = 0;
    local_98 = 0;
    local_90 = "country_core::startup::load_audio";
    local_88 = 0x21;
    local_80 = 0;
    local_50 = 1;
    ::log::__private_api::impl$2::log();
  }
  std::alloc::__default_lib_allocator::__rust_alloc();
  if (extraout_RAX_00 != (undefined4 *)0x0) {
    *extraout_RAX_00 = 0;
    extraout_RAX_00[6] = 0;
    local_98 = 1;
    local_88 = 1;
    local_80 = 1;
    bevy_ecs::system::commands::Commands::
    insert_resource<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>();
    plVar4 = *(longlong **)(in_RDX + 8);
    if (plVar4 == (longlong *)0x0) {
      plVar4 = *(longlong **)(in_RDX + 0x10);
    }
    lVar3 = plVar4[2];
    if ((ulonglong)(*plVar4 - lVar3) < 0xc) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar1 = plVar4[1];
    *(code **)(lVar1 + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::insert_resource::closure_env_0<country_core::systems::audio::terrain_stairs_audio::StairsCounter>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    *(undefined4 *)(lVar1 + 8 + lVar3) = 0;
    plVar4[2] = lVar3 + 0xc;
    local_98 = 2;
    local_68 = local_68 & 0xffffffff00000000;
    bevy_ecs::system::commands::Commands::
    insert_resource<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>();
    lVar3 = plVar4[2];
    if ((ulonglong)(*plVar4 - lVar3) < 8) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    *(code **)(plVar4[1] + lVar3) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::system::commands::command::remove_resource::closure_env_0<country_core::resources::audio_resources::AudioSamplesLoadedPromise>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    plVar4[2] = lVar3 + 8;
    return;
  }
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
}


