
void __cdecl
system_decorator::decorator_visual::maintain_visual_entities::clear_out_wall_holes(void)

{
  longlong in_RCX;
  longlong in_RDX;
  
  **(undefined4 **)(in_RCX + 0x10) = *(undefined4 *)(in_RCX + 0x1c);
  **(undefined4 **)(in_RDX + 0x10) = *(undefined4 *)(in_RDX + 0x1c);
  country_core::resources::walls::wall_holes::WallHoles::clear_for_next_frame();
  return;
}

