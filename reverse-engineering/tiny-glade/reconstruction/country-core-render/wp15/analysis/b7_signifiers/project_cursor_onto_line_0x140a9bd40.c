// addr=0x140a9bd40 name=project_cursor_onto_line
// signature=void __cdecl project_cursor_onto_line(void)


void __cdecl
country_core::systems::ui::resources::slider3d::Slider3dState::project_cursor_onto_line(void)

{
  ::utils::mouse_ss_to_cs();
  country_core::utils::project_cursor_onto_ray::ProportionalScreenSpaceCoord::project_onto_ray();
  return;
}


