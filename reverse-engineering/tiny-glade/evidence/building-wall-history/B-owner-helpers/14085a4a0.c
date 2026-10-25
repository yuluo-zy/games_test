
void __cdecl country_core::resources::walls::inner_walls::InnerWalls::new_wall_with_id(void)

{
  undefined1 local_98 [8];
  undefined1 *local_90;
  code *local_88;
  undefined **local_80;
  undefined8 local_78;
  undefined1 **local_70;
  undefined8 local_68;
  undefined8 local_60;
  
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (SBORROW8(0,(longlong)local_80)) {
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    return;
  }
  bevy_ecs::component::ComponentDescriptor::
  drop_ptr<country_core::systems::wall::finish_wall::ContinueWallOriginalState>();
  local_90 = local_98;
  local_88 = walls::impl$7::fmt;
  local_80 = &anon_6a7339a970014eecaf19421d4b3f97a6_53_llvm_4089674097218605020;
  local_78 = 1;
  local_60 = 0;
  local_70 = &local_90;
  local_68 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

