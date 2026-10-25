// addr=0x140a21620 name=startup_grass
// signature=void __cdecl startup_grass(void)


void __cdecl country_core::startup::startup_grass::startup_grass(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 *puVar4;
  longlong *plVar5;
  undefined8 local_108;
  longlong *plStack_100;
  longlong *local_f8;
  undefined8 local_f0;
  undefined8 local_e8;
  undefined8 local_d8;
  undefined *local_d0;
  undefined8 uStack_c8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  longlong local_b0;
  undefined8 local_a8;
  undefined4 uStack_a0;
  undefined4 local_9c;
  undefined4 uStack_98;
  uint local_94;
  undefined8 local_90;
  undefined8 local_80;
  longlong *plStack_78;
  longlong *local_70;
  undefined8 local_68;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  load_shader_into_library<ref_<str_>_>();
  local_108 = 0;
  local_d8 = 0;
  local_a8 = (undefined8 *)((ulonglong)local_a8._4_4_ << 0x20);
  local_9c = 0;
  uStack_98 = 5;
  local_94 = local_94 & 0xffffff00;
  local_90 = 0x1b;
  puVar4 = &local_108;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<u32,core::iter::adapters::take::Take<core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::ops::range::RangeInclusive<u32>,tuple_<u32,u32>_>,array_<u32,6>,country_core::startup::startup_grass::startup_grass::closure_env_0>_>_>
            ();
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(undefined8 **)(extraout_RAX + 0x10) = puVar4;
    local_b0 = extraout_RAX_00;
  }
  else {
    local_b0 = *(longlong *)(extraout_RAX + 8);
    puVar4 = *(undefined8 **)(extraout_RAX + 0x10);
  }
  *(longlong *)(extraout_RAX + 8) = local_b0 + 1;
  local_f8 = local_70;
  local_108 = local_80;
  plStack_100 = plStack_78;
  local_f0 = 0x8000000000000000;
  local_d0 = &anon_087c9b6d159f545cda87747878cb277f_0_llvm_10190951805537787575;
  uStack_c8 = 0;
  local_c0 = 0;
  uStack_b8 = 0;
  uStack_a0 = 0x42ac96c7;
  local_9c = 1;
  uStack_98 = 0xb;
  local_94 = 0;
  local_a8 = puVar4;
  load_mesh_into_library();
  local_68 = extraout_RAX_01;
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  uVar3 = local_e8;
  plVar5 = plStack_100;
  if (plStack_100 == (longlong *)0x0) {
    plVar5 = local_f8;
  }
  lVar1 = plVar5[2];
  if ((ulonglong)(*plVar5 - lVar1) < 0x88) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar2 = plVar5[1];
  *(code **)(lVar2 + lVar1) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar4 = (undefined8 *)(lVar2 + 8 + lVar1);
  *puVar4 = 0;
  puVar4[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar2 + 0x18 + lVar1) = 0;
  *(undefined4 *)(lVar2 + 0x20 + lVar1) = 0;
  *(undefined8 *)(lVar2 + 0x24 + lVar1) = 0x3f8000003f800000;
  *(undefined4 *)(lVar2 + 0x2c + lVar1) = 0x3f800000;
  *(undefined8 *)(lVar2 + 0x38 + lVar1) = local_68;
  *(undefined8 *)(lVar2 + 0x40 + lVar1) = extraout_RAX_02;
  *(undefined4 *)(lVar2 + 0x48 + lVar1) = 7;
  *(undefined1 *)(lVar2 + 0x58 + lVar1) = 0;
  *(undefined8 *)(lVar2 + 0x68 + lVar1) = uVar3;
  *(code **)(lVar2 + 0x78 + lVar1) = bevy_ecs::error::handler::panic;
  plVar5[2] = lVar1 + 0x88;
  load_shader_into_library<ref_<str_>_>();
  bevy_ecs::system::commands::Commands::spawn_empty();
  plVar5 = plStack_100;
  if (plStack_100 == (longlong *)0x0) {
    plVar5 = local_f8;
  }
  lVar1 = plVar5[2];
  if ((ulonglong)(*plVar5 - lVar1) < 0x88) {
    alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
  }
  lVar2 = plVar5[1];
  *(code **)(lVar2 + lVar1) =
       core::ops::function::FnOnce::
       call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
  ;
  puVar4 = (undefined8 *)(lVar2 + 8 + lVar1);
  *puVar4 = 0;
  puVar4[1] = 0x3f80000000000000;
  *(undefined8 *)(lVar2 + 0x18 + lVar1) = 0;
  *(undefined4 *)(lVar2 + 0x20 + lVar1) = 0;
  *(undefined8 *)(lVar2 + 0x24 + lVar1) = 0x3f8000003f800000;
  *(undefined4 *)(lVar2 + 0x2c + lVar1) = 0x3f800000;
  *(undefined8 *)(lVar2 + 0x38 + lVar1) = local_68;
  *(undefined8 *)(lVar2 + 0x40 + lVar1) = extraout_RAX_03;
  *(undefined4 *)(lVar2 + 0x48 + lVar1) = 7;
  *(undefined1 *)(lVar2 + 0x58 + lVar1) = 0;
  *(undefined8 *)(lVar2 + 0x68 + lVar1) = local_e8;
  *(code **)(lVar2 + 0x78 + lVar1) = bevy_ecs::error::handler::panic;
  plVar5[2] = lVar1 + 0x88;
  return;
}


