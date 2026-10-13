// ADDRESS: 0x14093bd80
// SIZE: 209
// SIGNATURE: void __cdecl create_preset_textures(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::create_preset_textures(void)

{
  longlong lVar1;
  longlong lVar2;
  longlong lVar3;
  longlong *extraout_RAX;
  longlong *in_RCX;
  
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX != (longlong *)0x0) {
    lVar2 = *extraout_RAX;
    lVar3 = extraout_RAX[1];
    *extraout_RAX = *extraout_RAX + 1;
    core::iter::adapters::map::impl$2::
    fold<tuple_<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl_1::create_preset_textures::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,hashbrown::map::impl_82::extend::closure_env_0<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<enum_iterator::All<country_core::resources::texture_presets::TexturePreset>,country_core::asset_libraries::texture_library::impl_1::create_preset_textures::closure_env_0>_>_>_>
              ();
    *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
    lVar1 = *in_RCX;
    hashbrown::raw::RawTableInner::
    drop_inner_table<tuple_<country_core::resources::texture_presets::TexturePreset,country_core::asset_libraries::texture_library::PresetTextureState>,alloc::alloc::Global>
              ();
    *(longlong *)(lVar1 + 0x70) = lVar2;
    *(longlong *)(lVar1 + 0x78) = lVar3;
    *(undefined8 *)(lVar1 + 0x60) = 0;
    *(undefined8 *)(lVar1 + 0x68) = 0;
    *(undefined **)(lVar1 + 0x50) =
         &anon_2ad33cd064e2b7a85895a07d96050156_27_llvm_3451300626392288586;
    *(undefined8 *)(lVar1 + 0x58) = 0;
    return;
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

