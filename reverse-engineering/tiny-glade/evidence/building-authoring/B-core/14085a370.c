
void __cdecl country_core::resources::walls::inner_walls::InnerWalls::new_wall_w_params(void)

{
  longlong in_RCX;
  undefined8 local_80;
  undefined8 local_68;
  
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

