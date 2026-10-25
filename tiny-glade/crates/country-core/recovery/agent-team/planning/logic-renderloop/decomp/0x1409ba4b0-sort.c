// 0x1409ba4b0 sort size=135

void __cdecl country_core::systems::render_loop::draw_passes::draw_lists::DrawLists::sort(void)

{
  ulonglong uVar1;
  longlong in_RCX;
  longlong lVar2;
  undefined1 local_29;
  undefined1 *local_28;
  
  lVar2 = 0;
  do {
    while (uVar1 = *(ulonglong *)(in_RCX + 0x10 + lVar2), uVar1 < 2) {
      lVar2 = lVar2 + 0x18;
      if (lVar2 == 0x1f8) {
        return;
      }
    }
    local_28 = &local_29;
    if (uVar1 < 0x15) {
      core::slice::sort::shared::smallsort::
      insertion_sort_shift_left<tuple_<country_core::render::pass_builder::draw_list::DrawListItem,void_(*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)>,alloc::slice::impl_0::sort_by_key::closure_env_0<tuple_<country_core::render::pass_builder::draw_list::DrawListItem,void_(*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)>,i32,country_core::systems::render_loop::draw_passes::draw_lists::impl_0::sort::closure_env_0>_>
                ();
    }
    else {
      core::slice::sort::stable::
      driftsort_main<tuple_<country_core::render::pass_builder::draw_list::DrawListItem,void_(*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)>,alloc::slice::impl_0::sort_by_key::closure_env_0<tuple_<country_core::render::pass_builder::draw_list::DrawListItem,void_(*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)>,i32,country_core::systems::render_loop::draw_passes::draw_lists::impl_0::sort::closure_env_0>,alloc::vec::Vec<tuple_<country_core::render::pass_builder::draw_list::DrawListItem,void_(*)(country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell)>,alloc::alloc::Global>_>
                ();
    }
    lVar2 = lVar2 + 0x18;
  } while (lVar2 != 0x1f8);
  return;
}

