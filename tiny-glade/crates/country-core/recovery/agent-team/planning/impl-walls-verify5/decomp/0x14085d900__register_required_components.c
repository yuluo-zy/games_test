// register_required_components  size=110
// void __cdecl register_required_components(void)

void __cdecl
country_core::systems::wall::adjust_wall_height::signifier_animation::impl$1::
register_required_components(void)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 extraout_RAX;
  longlong *in_stack_00000028;
  
  lVar1 = in_stack_00000028[1];
  lVar2 = in_stack_00000028[2];
  bevy_ecs::component::enforce_no_required_components_recursion();
  bevy_ecs::component::ComponentsRegistrator::
  register_component<country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier>
            ();
  if (lVar2 == *in_stack_00000028) {
    alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
    *(undefined8 *)(in_stack_00000028[1] + lVar2 * 8) = extraout_RAX;
  }
  else {
    *(undefined8 *)(lVar1 + lVar2 * 8) = extraout_RAX;
    in_stack_00000028[2] = lVar2 + 1;
    if (lVar2 + 1 == 0) {
      return;
    }
  }
  in_stack_00000028[2] = lVar2;
  return;
}



// CALLEES: [{'name': 'enforce_no_required_components_recursion', 'address': '140d40d70'}, {'name': 'grow_one<usize,alloc::alloc::Global>', 'address': '140b81df0'}, {'name': 'register_component<country_core::systems::wall::adjust_wall_height::signifier_animation::WallHeightSignifier>', 'address': '1408580e0'}]