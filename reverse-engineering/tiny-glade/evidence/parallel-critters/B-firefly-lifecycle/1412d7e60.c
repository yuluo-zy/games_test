
void __cdecl
core::ops::function::FnOnce::
call_once<bevy_ecs::world::command_queue::impl_4::push::closure_env_0<bevy_ecs::error::command_handling::impl_0::handle_error_with::closure_env_0<bevy_ecs::error::command_handling::impl_3::with_entity::closure_env_0<bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<utils::components::transform::Transform,country_core::resources::critters::Firefly,country_core::resources::critters::Critter,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier>_>_>,tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>,tuple_<bevy_ptr::OwningPtr<bevy_ptr::Unaligned>,enum2_<core::option::Option<core::ptr::non_null::NonNull<bevy_ecs::world::World>_>_>,ref_mut_<usize>_>_>
          (void)

{
  code *pcVar1;
  undefined8 extraout_RAX;
  longlong *in_RCX;
  longlong in_RDX;
  longlong *in_R8;
  longlong local_118;
  longlong lStack_110;
  undefined *local_108;
  longlong lStack_100;
  longlong local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  longlong local_c8;
  longlong lStack_c0;
  longlong local_b8;
  longlong lStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  longlong local_98;
  longlong lStack_90;
  undefined *local_88;
  longlong lStack_80;
  longlong local_78;
  longlong lStack_70;
  longlong local_68;
  longlong lStack_60;
  longlong local_58;
  longlong lStack_50;
  longlong local_48;
  longlong lStack_40;
  longlong local_38;
  longlong lStack_30;
  longlong local_28;
  longlong lStack_20;
  
  *in_R8 = *in_R8 + 0xa0;
  if (in_RDX != 0) {
    pcVar1 = (code *)in_RCX[0x12];
    local_28 = in_RCX[0xe];
    lStack_20 = in_RCX[0xf];
    local_38 = in_RCX[0xc];
    lStack_30 = in_RCX[0xd];
    local_48 = in_RCX[10];
    lStack_40 = in_RCX[0xb];
    local_58 = in_RCX[8];
    lStack_50 = in_RCX[9];
    local_98 = *in_RCX;
    lStack_90 = in_RCX[1];
    local_88 = (undefined *)in_RCX[2];
    lStack_80 = in_RCX[3];
    local_78 = in_RCX[4];
    lStack_70 = in_RCX[5];
    local_68 = in_RCX[6];
    lStack_60 = in_RCX[7];
    bevy_ecs::world::entity_fetch::impl$1::fetch_mut();
    if (local_118 == 0) {
      if (lStack_110 != 2) {
        bevy_ecs::error::bevy_error::impl$1::
        from<enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>();
        lStack_110 = -0x8000000000000000;
        local_108 = &anon_fc24da5738a8178533c5abfa9a6b7a26_71_llvm_15983634890098048951;
        lStack_100 = 0x26c;
        local_118 = CONCAT44(local_118._4_4_,2);
        (*pcVar1)(extraout_RAX,&local_118);
      }
    }
    else {
      local_a8 = local_28;
      lStack_a0 = lStack_20;
      local_b8 = local_38;
      lStack_b0 = lStack_30;
      local_c8 = local_48;
      lStack_c0 = lStack_40;
      local_d8 = local_58;
      lStack_d0 = lStack_50;
      local_e8 = local_68;
      lStack_e0 = lStack_60;
      local_f8 = local_78;
      lStack_f0 = lStack_70;
      local_108 = local_88;
      lStack_100 = lStack_80;
      local_118 = local_98;
      lStack_110 = lStack_90;
      bevy_ecs::system::commands::entity_command::impl$0::
      apply<tuple_<>,bevy_ecs::system::commands::entity_command::insert::closure_env_0<tuple_<utils::components::transform::Transform,country_core::resources::critters::Firefly,country_core::resources::critters::Critter,enum2_<country_core::resources::critters::CritterLocomotion>,country_core::resources::critters::CritterGoal,country_core::resources::critters::CritterVelocityTarget,country_core::resources::critters::Velocity,country_core::resources::critters::CritterVelocityMultiplier>_>_>
                ();
    }
    bevy_ecs::world::World::flush_entities();
    bevy_ecs::world::World::flush_components();
    bevy_ecs::world::World::flush_commands();
    return;
  }
  return;
}

