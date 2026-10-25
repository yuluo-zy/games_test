// new_wall_from_curve  size=355
// void __cdecl new_wall_from_curve(void)

void __cdecl country_core::resources::walls::inner_walls::InnerWalls::new_wall_from_curve(void)

{
  longlong in_RCX;
  longlong local_80;
  longlong local_68;
  
  *(longlong *)(in_RCX + 0xc0) = *(longlong *)(in_RCX + 0xc0) + 1;
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (!SBORROW8(0,local_80)) {
    if (local_80 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    if (local_68 != 0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  }
  hashbrown::map::HashMap::
  insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  return;
}



// CALLEES: [{'name': '__rust_dealloc', 'address': '140613c20'}, {'name': 'insert<country_core::resources::walls::WallId,country_core::resources::walls::inner_wall_state::WallState,std::hash::random::RandomState,alloc::alloc::Global>', 'address': '141529bb0'}, {'name': 'insert<country_core::resources::walls::WallId,tuple$<>,std::hash::random::RandomState,alloc::alloc::Global>', 'address': '1402ca2f0'}]