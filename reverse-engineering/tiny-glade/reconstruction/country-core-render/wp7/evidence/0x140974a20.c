
void __cdecl
country_core::render::pass_builder::pass_resource_access::
write_access_buffer<country_core::resources::water_mask::WaterMaskSsbo>(void)

{
  undefined8 *puVar1;
  ulonglong uVar2;
  ulonglong uVar3;
  longlong lVar4;
  ulonglong *in_R9;
  undefined8 local_7c;
  undefined8 uStack_74;
  undefined8 local_6c;
  undefined8 uStack_64;
  undefined8 local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined8 uStack_44;
  undefined4 local_3c;
  
  asset_libraries::ssbo_library::AssetSsboLibrary::
  get_rg_buffer<country_core::resources::water_mask::WaterMaskSsbo>();
  bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
  rhapsody::rg::pass_builder::PassBuilder::
  add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
  uVar2 = in_R9[2];
  if (uVar2 < 0x100) {
    rhapsody::rg::pass_api::impl$5::bind();
    if (uVar2 == *in_R9) {
      alloc::raw_vec::RawVec::
      grow_one<enum2_<rhapsody::rg::pass_api::RenderPassBinding>,alloc::alloc::Global>();
    }
    uVar3 = in_R9[1];
    lVar4 = uVar2 * 0x44;
    *(undefined4 *)(uVar3 + 0x40 + lVar4) = local_3c;
    puVar1 = (undefined8 *)(uVar3 + 0x30 + lVar4);
    *puVar1 = local_4c;
    puVar1[1] = uStack_44;
    puVar1 = (undefined8 *)(uVar3 + 0x20 + lVar4);
    *puVar1 = local_5c;
    puVar1[1] = uStack_54;
    puVar1 = (undefined8 *)(uVar3 + 0x10 + lVar4);
    *puVar1 = local_6c;
    puVar1[1] = uStack_64;
    *(undefined8 *)(uVar3 + lVar4) = local_7c;
    ((undefined8 *)(uVar3 + lVar4))[1] = uStack_74;
    in_R9[2] = uVar2 + 1;
    return;
  }
                    // WARNING: Subroutine does not return
  core::result::unwrap_failed();
}

