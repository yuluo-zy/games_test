// signature: void __cdecl roof_audio_init_system(void)

// WARNING: Removing unreachable block (ram,0x000140aa322e)

void __cdecl country_core::systems::audio::roof_audio::roof_audio_init_system(void)

{
  undefined8 local_108;
  
  resources::audio_resources::AudioCore::instantiate_loop();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  resources::audio_resources::AudioCore::instantiate_loop();
  if (SBORROW8(0,local_108)) {
                    // WARNING: Subroutine does not return
    core::result::unwrap_failed();
  }
  resources::audio_resources::AudioCore::instantiate_loop();
  bevy_ecs::system::commands::Commands::
  insert_resource<country_core::systems::audio::roof_audio::RoofAudioState>();
  return;
}

