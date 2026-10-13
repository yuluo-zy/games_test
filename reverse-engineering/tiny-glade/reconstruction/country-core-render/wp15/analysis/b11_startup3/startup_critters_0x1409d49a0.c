// addr=0x1409d49a0 name=startup_critters
// signature=void __cdecl startup_critters(void)


void __cdecl country_core::startup::startup_critters::startup_critters(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 *extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  longlong lVar5;
  longlong *plVar6;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar7;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000040;
  longlong *in_stack_00000048;
  longlong local_228;
  longlong *plStack_220;
  longlong *local_218;
  undefined4 uStack_210;
  undefined4 uStack_20c;
  undefined4 uStack_208;
  undefined4 uStack_204;
  longlong *local_178;
  
  if (*(char *)(*in_stack_00000040 + 0x1f) == '\x01') {
    plVar6 = (longlong *)*in_stack_00000030;
    lVar5 = 0x18;
    do {
      if (*plVar6 == 0) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        if (extraout_RAX == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        builtin_strncpy((char *)((longlong)extraout_RAX + 0xb),"Butterfl",8);
        builtin_strncpy((char *)((longlong)extraout_RAX + 0x13),"y/Shoosh",8);
        *extraout_RAX = 0x532f3a746e657665;
        extraout_RAX[1] = 0x65747475422f5846;
        local_228 = 0;
        local_218 = (longlong *)&DAT_0000001b;
        uStack_210 = SUB84(extraout_RAX,0);
        uStack_20c = (undefined4)((ulonglong)extraout_RAX >> 0x20);
        uStack_208 = 0x1b;
        uStack_204 = 0;
      }
      else {
        resources::audio_resources::AudioCoreInner::instantiate_audio();
        if (local_228 == 2) {
                    // WARNING: Subroutine does not return
          core::result::unwrap_failed();
        }
      }
      fastrand::Rng::f32();
      fastrand::Rng::f32();
      ::utils::components::transform::impl$1::default();
      fastrand::Rng::f32();
      bevy_ecs::system::commands::Commands::
      spawn<tuple_<utils::components::transform::Transform,country_core::resources::critters::Butterfly,country_core::resources::critters::ButterflyAudio,country_core::resources::critters::Critter,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier>_>
                ();
      lVar5 = lVar5 + -1;
    } while (lVar5 != 0);
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_228)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    ::utils::components::transform::impl$1::default();
    local_178 = local_218;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = plStack_220;
    if (plStack_220 == (longlong *)0x0) {
      plVar6 = local_218;
    }
    lVar5 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar5) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar6[1];
    *(code **)(lVar3 + lVar5) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    plVar1 = (longlong *)(lVar3 + 8 + lVar5);
    *plVar1 = local_228;
    plVar1[1] = (longlong)plStack_220;
    *(longlong **)(lVar3 + 0x18 + lVar5) = local_218;
    *(undefined4 *)(lVar3 + 0x20 + lVar5) = uStack_210;
    *(undefined8 *)(lVar3 + 0x24 + lVar5) = 0x3f19999a3f19999a;
    *(undefined4 *)(lVar3 + 0x2c + lVar5) = 0x3f19999a;
    *(undefined8 *)(lVar3 + 0x38 + lVar5) = extraout_RAX_00;
    *(undefined8 *)(lVar3 + 0x40 + lVar5) = extraout_RAX_01;
    *(undefined1 *)(lVar3 + 0x48 + lVar5) = 0;
    *(ulonglong *)(lVar3 + 0x58 + lVar5) = CONCAT44(uStack_204,uStack_208);
    *(code **)(lVar3 + 0x68 + lVar5) = bevy_ecs::error::handler::panic;
    plVar6[2] = lVar5 + 0x78;
  }
  if (*(char *)(*in_stack_00000048 + 0x58) != '\x04') {
    lVar5 = 0;
    do {
      ::utils::radical_inverse();
      fastrand::Rng::f32();
      fVar7 = 1.0;
      fmodf(extraout_XMM0_Da_00 * 0.25 + extraout_XMM0_Da,1.0);
      fastrand::Rng::f32();
      ::utils::uniform_sample_disk();
      fastrand::Rng::f32();
      ::utils::components::transform::impl$1::default();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar6 = plStack_220;
      if (plStack_220 == (longlong *)0x0) {
        plVar6 = local_218;
      }
      lVar3 = plVar6[2];
      if ((ulonglong)(*plVar6 - lVar3) < 0xa8) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar5 = lVar5 + 1;
      lVar4 = plVar6[1];
      *(code **)(lVar4 + lVar3) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<utils::components::transform::Transform,country_core::resources::critters::Firefly,country_core::resources::critters::Critter,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(undefined4 *)(lVar4 + 8 + lVar3) = 0;
      plVar1 = (longlong *)(lVar4 + 0x18 + lVar3);
      *plVar1 = local_228;
      plVar1[1] = (longlong)plStack_220;
      *(float *)(lVar4 + 0x28 + lVar3) = extraout_XMM0_Da_01 * 65.0;
      *(float *)(lVar4 + 0x2c + lVar3) = extraout_XMM0_Da_02 + 1.0;
      *(float *)(lVar4 + 0x30 + lVar3) = fVar7 * 65.0;
      *(ulonglong *)(lVar4 + 0x34 + lVar3) = CONCAT44(uStack_208,uStack_20c);
      *(undefined4 *)(lVar4 + 0x3c + lVar3) = uStack_204;
      puVar2 = (undefined8 *)(lVar4 + 0x40 + lVar3);
      *puVar2 = local_178;
      puVar2[1] = 0;
      *(undefined4 *)(lVar4 + 0x50 + lVar3) = 0;
      *(undefined8 *)(lVar4 + 0x54 + lVar3) = 0x33d4ccccd;
      puVar2 = (undefined8 *)(lVar4 + 0x60 + lVar3);
      *puVar2 = 0;
      puVar2[1] = 0;
      *(undefined4 *)(lVar4 + 0x70 + lVar3) = 0x3f800000;
      *(undefined1 *)(lVar4 + 0x78 + lVar3) = 0;
      *(ulonglong *)(lVar4 + 0x88 + lVar3) = CONCAT44(uStack_204,uStack_208);
      *(code **)(lVar4 + 0x98 + lVar3) = bevy_ecs::error::handler::panic;
      plVar6[2] = lVar3 + 0xa8;
    } while (lVar5 != 0x60);
    geometry::sphere::impl$2::try_from();
    if (SBORROW8(0,local_228)) {
                    // WARNING: Subroutine does not return
      core::result::unwrap_failed();
    }
    country_core::render::mesh::Mesh::add_color_self();
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar6 = plStack_220;
    if (plStack_220 == (longlong *)0x0) {
      plVar6 = local_218;
    }
    lVar5 = plVar6[2];
    if ((ulonglong)(*plVar6 - lVar5) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar6[1];
    *(code **)(lVar3 + lVar5) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar2 = (undefined8 *)(lVar3 + 8 + lVar5);
    *puVar2 = 0;
    puVar2[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar5) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar5) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar5) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar5) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar5) = extraout_RAX_02;
    *(undefined8 *)(lVar3 + 0x40 + lVar5) = extraout_RAX_03;
    *(undefined1 *)(lVar3 + 0x48 + lVar5) = 0;
    *(ulonglong *)(lVar3 + 0x58 + lVar5) = CONCAT44(uStack_204,uStack_208);
    *(code **)(lVar3 + 0x68 + lVar5) = bevy_ecs::error::handler::panic;
    plVar6[2] = lVar5 + 0x78;
    geometry::plane::impl$1::from();
    country_core::render::mesh::Mesh::add_color_self();
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    if (plStack_220 == (longlong *)0x0) {
      plStack_220 = local_218;
    }
    lVar5 = plStack_220[2];
    if ((ulonglong)(*plStack_220 - lVar5) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plStack_220[1];
    *(code **)(lVar3 + lVar5) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar2 = (undefined8 *)(lVar3 + 8 + lVar5);
    *puVar2 = 0;
    puVar2[1] = 0x3f80000000000000;
    *(undefined8 *)(lVar3 + 0x18 + lVar5) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar5) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar5) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar5) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar5) = extraout_RAX_04;
    *(undefined8 *)(lVar3 + 0x40 + lVar5) = extraout_RAX_05;
    *(undefined4 *)(lVar3 + 0x48 + lVar5) = 0x11;
    *(undefined1 *)(lVar3 + 0x58 + lVar5) = 0;
    *(ulonglong *)(lVar3 + 0x68 + lVar5) = CONCAT44(uStack_204,uStack_208);
    *(code **)(lVar3 + 0x78 + lVar5) = bevy_ecs::error::handler::panic;
    plStack_220[2] = lVar5 + 0x88;
  }
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_228)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  load_mesh_into_library();
  load_shader_into_library<ref_<str_>_>();
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (!SBORROW8(0,local_228)) {
    load_mesh_into_library();
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (!SBORROW8(0,local_228)) {
      load_mesh_into_library();
      load_shader_into_library<ref_<str_>_>();
      return;
    }
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}


