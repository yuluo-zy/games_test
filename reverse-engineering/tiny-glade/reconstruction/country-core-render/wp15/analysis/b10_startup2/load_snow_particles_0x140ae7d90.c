// addr=0x140ae7d90 name=load_snow_particles
// signature=void __cdecl load_snow_particles(void)


void __cdecl country_core::startup::load_glade_tod_and_weather::load_snow_particles(void)

{
  longlong lVar1;
  longlong lVar2;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  undefined8 extraout_RAX_01;
  ulonglong extraout_RAX_02;
  undefined8 *puVar3;
  char *pcVar4;
  longlong *plVar5;
  longlong *in_stack_00000038;
  undefined8 local_d8;
  longlong *plStack_d0;
  longlong *local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_a8;
  undefined *local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  longlong local_80;
  undefined8 *local_78;
  undefined4 uStack_70;
  undefined4 local_6c;
  undefined4 uStack_68;
  uint local_64;
  undefined8 local_58;
  longlong *plStack_50;
  longlong *local_48;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  if (*(char *)(*in_stack_00000038 + 0x58) == '\x04') {
    local_d8 = 0;
    local_a8 = 0;
    local_78 = (undefined8 *)((ulonglong)local_78 & 0xffffffff00000000);
    local_6c = 0;
    uStack_68 = 1;
    local_64 = local_64 & 0xffffff00;
    puVar3 = &local_d8;
    alloc::vec::spec_from_iter::impl$0::
    from_iter<u32,core::iter::adapters::flatten::FlatMap<itertools::tuple_impl::TupleWindows<core::ops::range::RangeInclusive<u32>,tuple_<u32,u32>_>,array_<u32,6>,country_core::systems::render_loop::draw_passes::draw_shaders::clouds::startup_clouds::closure_env_0>_>
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
      *(undefined8 **)(extraout_RAX + 0x10) = puVar3;
      local_80 = extraout_RAX_00;
    }
    else {
      local_80 = *(longlong *)(extraout_RAX + 8);
      puVar3 = *(undefined8 **)(extraout_RAX + 0x10);
    }
    *(longlong *)(extraout_RAX + 8) = local_80 + 1;
    local_c8 = local_48;
    local_d8 = local_58;
    plStack_d0 = plStack_50;
    local_c0 = 0x8000000000000000;
    local_a0 = &anon_e5eae2fa848f610737f81e1b032ceca1_0_llvm_3675082498294580552;
    uStack_98 = 0;
    local_90 = 0;
    uStack_88 = 0;
    uStack_70 = 0x42ae35c8;
    local_6c = 1;
    uStack_68 = 0xe;
    local_64 = 0;
    local_78 = puVar3;
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    pcVar4 = "snow_particles_shader";
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar5 = plStack_d0;
    if (plStack_d0 == (longlong *)0x0) {
      plVar5 = local_c8;
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
    puVar3 = (undefined8 *)(lVar2 + 8 + lVar1);
    *puVar3 = 0;
    puVar3[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar2 + 0x18 + lVar1) = 0;
    *(undefined4 *)(lVar2 + 0x20 + lVar1) = 0;
    *(undefined8 *)(lVar2 + 0x24 + lVar1) = 0x3f8000003f800000;
    *(undefined4 *)(lVar2 + 0x2c + lVar1) = 0x3f800000;
    *(undefined8 *)(lVar2 + 0x38 + lVar1) = extraout_RAX_01;
    *(char **)(lVar2 + 0x40 + lVar1) = pcVar4;
    *(undefined4 *)(lVar2 + 0x48 + lVar1) = 0xc;
    *(undefined1 *)(lVar2 + 0x58 + lVar1) = 0;
    *(undefined8 *)(lVar2 + 0x68 + lVar1) = local_b8;
    *(code **)(lVar2 + 0x78 + lVar1) = bevy_ecs::error::handler::panic;
    plVar5[2] = lVar1 + 0x88;
  }
  return;
}


