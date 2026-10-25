// 0x140a59800 as_texture size=83

void __cdecl country_core::render::pass_builder::pass_resource_access::impl$0::as_texture(void)

{
  undefined8 extraout_RAX;
  undefined4 *in_RCX;
  undefined8 local_2c;
  undefined8 uStack_24;
  undefined8 local_1c;
  undefined8 uStack_14;
  undefined4 local_c;
  
  asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture();
  rhapsody::raw::texture::TextureDesc::extent_2d();
  *(undefined8 *)(in_RCX + 3) = local_2c;
  *(undefined8 *)(in_RCX + 5) = uStack_24;
  *(undefined8 *)(in_RCX + 7) = local_1c;
  *(undefined8 *)(in_RCX + 9) = uStack_14;
  in_RCX[0xb] = local_c;
  *(undefined8 *)(in_RCX + 1) = extraout_RAX;
  *in_RCX = 0;
  return;
}

