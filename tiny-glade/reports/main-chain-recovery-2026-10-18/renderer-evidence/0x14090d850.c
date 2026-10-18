
void __cdecl country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain(void)

{
  longlong extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong in_RDX;
  ulonglong uVar1;
  
  rhapsody::raw::texture::TextureDesc::extent_2d();
  uVar1 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if ((((extraout_RAX != 0) && (uVar1 < *(ulonglong *)(in_RDX + 0x218))) &&
      (uVar1 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar1 * 8), uVar1 != 0)) &&
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar1 * 0x60) != 0)) {
    asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
    if ((extraout_RAX_00 & 1) != 0) {
      country_core::render::raster_single::
      raster_single_object<1,array_<enum2_<country_core::render::pass_builder::framebuffer_binding::ColorAttachment>,1>,country_core::systems::render_loop::draw_passes::final_post::resolve_to_swapchain::closure_env_0>
                ();
      return;
    }
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

