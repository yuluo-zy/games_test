
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::startup::startup_critters::startup_critters(void)

{
  longlong *plVar1;
  undefined8 *puVar2;
  longlong lVar3;
  longlong lVar4;
  undefined8 uVar5;
  float _Y;
  undefined8 uVar6;
  float fVar7;
  undefined8 *extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  longlong lVar8;
  longlong *plVar9;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float fVar10;
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
    plVar9 = (longlong *)*in_stack_00000030;
    lVar8 = 0x18;
    do {
      if (*plVar9 == 0) {
        std::alloc::__default_lib_allocator::__rust_alloc();
        uVar5 = s_Butterfly_Shoosh_142ac22f4._8_8_;
        if (extraout_RAX == (undefined8 *)0x0) {
                    /* WARNING: Subroutine does not return */
          alloc::raw_vec::handle_error();
        }
        *(ulonglong *)((longlong)extraout_RAX + 0xb) =
             CONCAT35(s_Butterfly_Shoosh_142ac22f4._5_3_,s_Butterfly_Shoosh_142ac22f4._0_5_);
        *(undefined8 *)((longlong)extraout_RAX + 0x13) = uVar5;
        uVar5 = CONCAT53(s_Butterfly_Shoosh_142ac22f4._0_5_,_UNK_142ac22f1);
        *extraout_RAX = _DAT_142ac22e9;
        extraout_RAX[1] = uVar5;
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
                    /* WARNING: Subroutine does not return */
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
      lVar8 = lVar8 + -1;
    } while (lVar8 != 0);
    country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
    if (SBORROW8(0,local_228)) {
                    /* WARNING: Subroutine does not return */
      core::result::unwrap_failed();
    }
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    ::utils::components::transform::impl$1::default();
    local_178 = local_218;
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar9 = plStack_220;
    if (plStack_220 == (longlong *)0x0) {
      plVar9 = local_218;
    }
    lVar8 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar8) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar9[1];
    *(code **)(lVar3 + lVar8) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::components::drawable::DrawableMeshBundle>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    plVar1 = (longlong *)(lVar3 + 8 + lVar8);
    *plVar1 = local_228;
    plVar1[1] = (longlong)plStack_220;
    *(longlong **)(lVar3 + 0x18 + lVar8) = local_218;
    *(undefined4 *)(lVar3 + 0x20 + lVar8) = uStack_210;
    *(undefined8 *)(lVar3 + 0x24 + lVar8) = __xmm_00000000000000003f19999a3f19999a;
    *(undefined4 *)(lVar3 + 0x2c + lVar8) = 0x3f19999a;
    *(undefined8 *)(lVar3 + 0x38 + lVar8) = extraout_RAX_00;
    *(undefined8 *)(lVar3 + 0x40 + lVar8) = extraout_RAX_01;
    *(undefined1 *)(lVar3 + 0x48 + lVar8) = 0;
    *(ulonglong *)(lVar3 + 0x58 + lVar8) = CONCAT44(uStack_204,uStack_208);
    *(code **)(lVar3 + 0x68 + lVar8) = bevy_ecs::error::handler::panic;
    plVar9[2] = lVar8 + 0x78;
  }
  fVar7 = __real_42820000;
  _Y = __real_3f800000;
  if (*(char *)(*in_stack_00000048 + 0x58) != '\x04') {
    lVar8 = 0;
    do {
      ::utils::radical_inverse();
      fastrand::Rng::f32();
      fVar10 = _Y;
      fmodf(extraout_XMM0_Da_00 * __real_3e800000 + extraout_XMM0_Da,_Y);
      fastrand::Rng::f32();
      ::utils::uniform_sample_disk();
      fastrand::Rng::f32();
      ::utils::components::transform::impl$1::default();
      bevy_ecs::system::commands::Commands::spawn_empty();
      plVar9 = plStack_220;
      if (plStack_220 == (longlong *)0x0) {
        plVar9 = local_218;
      }
      lVar3 = plVar9[2];
      if ((ulonglong)(*plVar9 - lVar3) < 0xa8) {
        alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
      }
      lVar8 = lVar8 + 1;
      lVar4 = plVar9[1];
      *(code **)(lVar4 + lVar3) =
           core::ops::function::FnOnce::
           call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<utils::components::transform::Transform,country_core::resources::critters::Firefly,country_core::resources::critters::Critter,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
      ;
      *(undefined4 *)(lVar4 + 8 + lVar3) = 0;
      plVar1 = (longlong *)(lVar4 + 0x18 + lVar3);
      *plVar1 = local_228;
      plVar1[1] = (longlong)plStack_220;
      *(float *)(lVar4 + 0x28 + lVar3) = extraout_XMM0_Da_01 * fVar7;
      *(float *)(lVar4 + 0x2c + lVar3) = extraout_XMM0_Da_02 + _Y;
      *(float *)(lVar4 + 0x30 + lVar3) = fVar10 * fVar7;
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
      plVar9[2] = lVar3 + 0xa8;
    } while (lVar8 != 0x60);
    geometry::sphere::impl$2::try_from();
    if (SBORROW8(0,local_228)) {
                    /* WARNING: Subroutine does not return */
      core::result::unwrap_failed();
    }
    country_core::render::mesh::Mesh::add_color_self();
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    plVar9 = plStack_220;
    if (plStack_220 == (longlong *)0x0) {
      plVar9 = local_218;
    }
    lVar8 = plVar9[2];
    if ((ulonglong)(*plVar9 - lVar8) < 0x78) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plVar9[1];
    *(code **)(lVar3 + lVar8) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DontCastShadow>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    uVar6 = DAT_14293e0c8;
    uVar5 = __xmm_3f800000000000000000000000000000;
    puVar2 = (undefined8 *)(lVar3 + 8 + lVar8);
    *puVar2 = __xmm_3f800000000000000000000000000000;
    puVar2[1] = uVar6;
    *(undefined8 *)(lVar3 + 0x18 + lVar8) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar8) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar8) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar8) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar8) = extraout_RAX_02;
    *(undefined8 *)(lVar3 + 0x40 + lVar8) = extraout_RAX_03;
    *(undefined1 *)(lVar3 + 0x48 + lVar8) = 0;
    *(ulonglong *)(lVar3 + 0x58 + lVar8) = CONCAT44(uStack_204,uStack_208);
    *(code **)(lVar3 + 0x68 + lVar8) = bevy_ecs::error::handler::panic;
    plVar9[2] = lVar8 + 0x78;
    geometry::plane::impl$1::from();
    country_core::render::mesh::Mesh::add_color_self();
    load_mesh_into_library();
    load_shader_into_library<ref_<str_>_>();
    bevy_ecs::system::commands::Commands::spawn_empty();
    if (plStack_220 == (longlong *)0x0) {
      plStack_220 = local_218;
    }
    lVar8 = plStack_220[2];
    if ((ulonglong)(*plStack_220 - lVar8) < 0x88) {
      alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
    }
    lVar3 = plStack_220[1];
    *(code **)(lVar3 + lVar8) =
         core::ops::function::FnOnce::
         call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<country_core::components::drawable::DrawableMeshBundle,country_core::components::drawable::DesignatedRenderPass>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
    ;
    puVar2 = (undefined8 *)(lVar3 + 8 + lVar8);
    *puVar2 = uVar5;
    puVar2[1] = uVar6;
    *(undefined8 *)(lVar3 + 0x18 + lVar8) = 0;
    *(undefined4 *)(lVar3 + 0x20 + lVar8) = 0;
    *(undefined8 *)(lVar3 + 0x24 + lVar8) = 0x3f8000003f800000;
    *(undefined4 *)(lVar3 + 0x2c + lVar8) = 0x3f800000;
    *(undefined8 *)(lVar3 + 0x38 + lVar8) = extraout_RAX_04;
    *(undefined8 *)(lVar3 + 0x40 + lVar8) = extraout_RAX_05;
    *(undefined4 *)(lVar3 + 0x48 + lVar8) = 0x11;
    *(undefined1 *)(lVar3 + 0x58 + lVar8) = 0;
    *(ulonglong *)(lVar3 + 0x68 + lVar8) = CONCAT44(uStack_204,uStack_208);
    *(code **)(lVar3 + 0x78 + lVar8) = bevy_ecs::error::handler::panic;
    plStack_220[2] = lVar8 + 0x88;
  }
  country_core::utils::load_json::load_json_as_mesh<ref_<str_>_>();
  if (SBORROW8(0,local_228)) {
                    /* WARNING: Subroutine does not return */
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
                    /* WARNING: Subroutine does not return */
    core::result::unwrap_failed();
  }
                    /* WARNING: Subroutine does not return */
  core::result::unwrap_failed();
}

