/* random_tree_prefab @0x0x14226a170 size 225 */

void __cdecl system_tree::ui_place_tree::random_tree_prefab(void)

{
  char extraout_AL;
  char extraout_AL_00;
  char extraout_AL_01;
  char extraout_AL_02;
  byte in_CL;
  byte bVar1;
  
  country_core::systems::save::load_completion_checker::SessionLoadReadiness::deny_load();
  if ((extraout_AL == '\0') &&
     (country_core::resources::session::SessionMeta::is_halloween_any(), extraout_AL_00 == '\0')) {
    country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
    if (extraout_AL_02 != '\0') {
      bVar1 = 0;
joined_r0x00014226a1d3:
      if ((in_CL & 1) == 0) goto LAB_14226a245;
      goto LAB_14226a1d5;
    }
  }
  else {
    bVar1 = in_CL ^ 1;
    country_core::systems::save::save_backup_list::BackupSaveFileKind::is_corrupted();
    if (extraout_AL_01 != '\0') goto joined_r0x00014226a1d3;
LAB_14226a1d5:
    if ((bVar1 & 1) != 0) goto LAB_14226a245;
  }
  country_core::systems::ui::resources::ui_state::DragState::is_started();
LAB_14226a245:
  asset::prefab::PrefabName::new();
  return;
}

