// ADDRESS: 0x140a07a70
// SIZE: 181
// SIGNATURE: void __cdecl collision_bbx(void)

void __cdecl country_core::render::mesh::Mesh::collision_bbx(void)

{
  longlong in_RCX;
  undefined4 local_4c;
  undefined4 local_40;
  
  aabb_bounds();
  ::utils::geometry::aabb::Aabb3::extent();
  ::utils::geometry::aabb::Aabb3::extent();
  ::utils::geometry::aabb::Aabb3::center();
  ::utils::geometry::rectangle::Rectangle2d::new_axis_aligned();
  *(undefined4 *)(in_RCX + 0x18) = local_4c;
  *(undefined4 *)(in_RCX + 0x1c) = local_40;
  return;
}

