// init_state  size=46
// void __cdecl init_state(void)

void __cdecl country_core::resources::walls::decorator_archivist::_::impl$0::init_state(void)

{
  bevy_ecs::system::system_param::impl$9::
  init_state<country_core::systems::collision::world::RaycastWorld>();
  bevy_ecs::system::system_param::impl$9::
  init_state<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>();
  return;
}



// CALLEES: [{'name': 'init_state<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>', 'address': '14134c460'}, {'name': 'init_state<country_core::systems::collision::world::RaycastWorld>', 'address': '14136ca50'}]