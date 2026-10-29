
void __cdecl country_core::systems::ivy::impl$0::insert_ivy_resources(void)

{
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX != (longlong *)0x0) {
    *extraout_RAX = *extraout_RAX + 1;
    bevy_app::app::App::insert_resource<country_core::resources::ivy::ivy_storage::IvyStorage>();
    bracket_noise::fastnoise::FastNoise::seeded();
    bracket_noise::fastnoise::FastNoise::set_fractal_octaves();
    bracket_noise::fastnoise::FastNoise::set_fractal_gain();
    bracket_noise::fastnoise::FastNoise::set_seed();
    bevy_app::app::App::
    insert_resource<country_core::resources::ivy::ivy_direction_proposer::IvySpawnNoise>();
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_00 != (longlong *)0x0) {
      *extraout_RAX_00 = *extraout_RAX_00 + 1;
      bevy_app::app::App::
      insert_resource<country_core::resources::ivy::ivy_leaf_storage::IvyLeafStorage>();
      bevy_app::app::App::
      insert_resource<country_core::resources::ivy::ivy_leaf_storage::IvyLeafRng>();
      return;
    }
  }
                    // WARNING: Subroutine does not return
  std::thread::local::panic_access_error();
}

