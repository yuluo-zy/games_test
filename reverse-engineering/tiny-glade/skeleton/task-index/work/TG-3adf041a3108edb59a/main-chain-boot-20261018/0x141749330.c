
void __cdecl
_ZN120__LT_bevy_ecs__system__schedule_system__InfallibleSystemWrapper_LT_S_GT__u20_as_u20_bevy_ecs__system__system__System_GT_10initialize17h71ae826f3b773633E_llvm_13877059707774801069
          (void)

{
  longlong *plVar1;
  undefined8 extraout_RAX;
  undefined8 extraout_RAX_00;
  undefined8 extraout_RAX_01;
  undefined8 extraout_RAX_02;
  undefined8 extraout_RAX_03;
  undefined8 extraout_RAX_04;
  undefined8 extraout_RAX_05;
  undefined8 extraout_RAX_06;
  undefined8 extraout_RAX_07;
  undefined8 extraout_RAX_08;
  undefined8 extraout_RAX_09;
  undefined8 extraout_RAX_10;
  undefined8 extraout_RAX_11;
  undefined8 extraout_RAX_12;
  undefined8 extraout_RAX_13;
  longlong in_RCX;
  longlong in_RDX;
  undefined1 local_588 [160];
  undefined1 local_4e8 [280];
  longlong local_3d0 [36];
  undefined8 local_2b0;
  undefined **local_288;
  undefined8 uStack_280;
  undefined8 local_278;
  undefined8 uStack_270;
  undefined8 local_268;
  undefined8 uStack_260;
  undefined8 local_258;
  undefined1 local_250 [328];
  undefined **local_108;
  undefined8 uStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  undefined8 local_d8;
  undefined8 local_d0;
  undefined8 local_c8;
  undefined8 local_c0;
  undefined8 local_b8;
  undefined8 local_b0;
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined8 local_68;
  longlong local_60;
  undefined8 local_58;
  longlong *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_60 = in_RCX;
  if (SBORROW8(0,*(longlong *)(in_RCX + 0x138))) {
    bevy_ecs::system::system_param::impl$9::
    init_state<enum2_<country_core::systems::mode_manager::AppMode>_>();
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::systems::save::load_game::LoadGameManager>();
    local_d0 = extraout_RAX_00;
    bevy_ecs::system::system_param::impl$12::init_state<country_core::meta_commander::MetaCommander>
              ();
    local_c8 = extraout_RAX_01;
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::resources::session::ActiveSession>();
    local_c0 = extraout_RAX_02;
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::systems::save::save_game::AutoSaveState>();
    local_b8 = extraout_RAX_03;
    bevy_ecs::system::system_param::impl$12::init_state<country_core::game_profile::GameProfile>();
    local_b0 = extraout_RAX_04;
    bevy_ecs::system::system_param::impl$12::
    init_state<enum2_<bevy_state::state::resources::NextState<country_core::app_loading_state::AppLoadingState>_>_>
              ();
    local_a8 = extraout_RAX_05;
    bevy_ecs::system::commands::_::impl$0::init_state();
    country_core::systems::save::snapshot::_::impl$0::init_state();
    bevy_ecs::system::system_param::impl$9::init_state<country_core::game_config::GameConfig>();
    local_a0 = extraout_RAX_06;
    bevy_ecs::system::system_param::impl$9::
    init_state<country_core::resources::glade_settings::GladeSettings>();
    local_98 = extraout_RAX_07;
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::systems::camera_rig::MainCamera>();
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::resources::time_of_day::TimeOfDay>();
    local_50 = (longlong *)extraout_RAX_08;
    bevy_ecs::system::system_param::impl$12::init_state<country_core::resources::history::History>()
    ;
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::resources::walls::inner_walls::InnerWalls>();
    bevy_ecs::query::state::QueryState::
    new_with_access<ref_mut_<country_core::systems::debug::sheep_controls::SheepControl>,tuple_<>_>
              ();
    local_90 = extraout_RAX;
    bevy_ecs::system::system_param::
    init_query_param<ref_mut_<country_core::systems::debug::sheep_controls::SheepControl>,tuple_<>_>
              ();
    memcpy(local_4e8,&local_288,0x118);
    bevy_ecs::system::system_param::impl$12::
    init_state<country_core::resources::glade_color_customization::GladeColorCustomization>();
    bevy_ecs::system::system_param::impl$12::
    init_state<bevy_ecs::event::collections::Events<country_core::resources::critters::sheep::CmdUnfreezeSheep>_>
              ();
    local_2b0 = local_50;
    memcpy(local_3d0,local_4e8,0x118);
    bevy_ecs::system::system_param::impl$9::
    init_state<bevy_ecs::event::collections::Events<country_core::resources::events::ModeJustChanged>_>
              ();
    local_88 = extraout_RAX_09;
    bevy_ecs::system::system_param::impl$12::
    init_state<bevy_ecs::event::collections::Events<country_core::systems::history::history_process::HistoryCmd>_>
              ();
    local_80 = extraout_RAX_10;
    bevy_ecs::system::system_param::impl$12::
    init_state<bevy_ecs::event::collections::Events<country_core::resources::history::tree_edit::SpawnPreplacedTrees>_>
              ();
    local_78 = extraout_RAX_11;
    bevy_ecs::system::system_param::impl$12::
    init_state<bevy_ecs::event::collections::Events<country_core::systems::save::load_game::LoadingGameComplete>_>
              ();
    local_70 = extraout_RAX_12;
    bevy_ecs::system::system_param::impl$12::
    init_state<bevy_ecs::event::collections::Events<country_core::systems::save::load_completion_checker::DisplayCorruptedSaveFileWarning>_>
              ();
    plVar1 = (longlong *)(in_RCX + 0x138);
    local_258 = local_d8;
    local_268 = local_e8;
    uStack_260 = uStack_e0;
    local_278 = local_f8;
    uStack_270 = uStack_f0;
    local_288 = local_108;
    uStack_280 = uStack_100;
    local_68 = extraout_RAX_13;
    memcpy(local_4e8,local_588,0xa0);
    memcpy(local_250,local_3d0,0x148);
    local_58 = *(undefined8 *)(in_RDX + 0x5a0);
    local_50 = plVar1;
    if (!SBORROW8(0,*plVar1)) {
      _ZN4core3ptr759drop_in_place_LT_bevy_ecs__system__function_system__FunctionSystemState_LT__LP_bevy_ecs__change_detection__ResMut_LT_country_core__systems__ui__resources__ui_state__UiState_GT__C_bevy_ecs__change_detection__Res_LT_country_core__asset_libraries__mesh_library__AssetMeshLibrary_GT__C_bevy_ecs__change_detection__Res_LT_country_core__asset_libraries__shader_library__AssetShaderLibrary_GT__C_bevy_ecs__system__commands__Commands_C_bevy_ecs__system__query__Query_LT__RF_country_core__systems__ui__signifiers__SignifierId_C_bevy_ecs__query__filter__Without_LT_country_core__systems__ui__signifiers__SignifierMarkedForDeletion_GT__GT__C_bevy_ecs__change_detection__ResMut_LT_country_core__systems__ui__signifiers__signifier_create__UiSignifierStream_GT__RP__GT__GT_17h47b89f388bc7e078E
                ();
    }
    memcpy(local_50,&local_288,0x180);
    *(undefined8 *)(local_60 + 0x2b8) = local_90;
    *(undefined8 *)(local_60 + 0x2c0) = local_d0;
    *(undefined8 *)(local_60 + 0x2c8) = local_c8;
    *(undefined8 *)(local_60 + 0x2d0) = local_c0;
    *(undefined8 *)(local_60 + 0x2d8) = local_b8;
    *(undefined8 *)(local_60 + 0x2e0) = local_b0;
    *(undefined8 *)(local_60 + 0x2e8) = local_a8;
    memcpy((void *)(local_60 + 0x2f0),local_4e8,0xa0);
    *(undefined8 *)(local_60 + 0x390) = local_a0;
    *(undefined8 *)(local_60 + 0x398) = local_98;
    *(undefined8 *)(local_60 + 0x3a0) = 0;
    *(undefined8 *)(local_60 + 0x3a8) = local_88;
    *(undefined8 *)(local_60 + 0x3b0) = local_80;
    *(undefined8 *)(local_60 + 0x3b8) = local_78;
    *(undefined8 *)(local_60 + 0x3c0) = local_70;
    *(undefined8 *)(local_60 + 0x3c8) = local_68;
    *(undefined8 *)(local_60 + 0x3d0) = local_58;
  }
  else {
    local_3d0[0] = *(longlong *)(in_RDX + 0x5a0);
    if (*(longlong *)(in_RCX + 0x3d0) != local_3d0[0]) {
      local_288 = &PTR_s_System_built_with_a_different_wo_142cd2c20;
      uStack_280 = 1;
      local_278 = 8;
      uStack_270 = 0;
      local_268 = 0;
                    // WARNING: Subroutine does not return
      core::panicking::
      assert_failed<bevy_ecs::world::identifier::WorldId,bevy_ecs::world::identifier::WorldId>();
    }
  }
  *(int *)(local_60 + 0x130) = *(int *)(in_RDX + 0x5b0) + 0x3dcc5000;
  return;
}

