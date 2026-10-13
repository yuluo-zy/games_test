
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl system_tree::impl$0::build(void)

{
  longlong extraout_RAX;
  byte *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  longlong extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong lVar1;
  longlong in_RDX;
  ulonglong uVar2;
  undefined8 **ppuVar3;
  char **local_1e8;
  code *pcStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 uStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined8 uStack_190;
  undefined8 local_188;
  undefined8 uStack_180;
  char *local_178;
  undefined8 uStack_170;
  undefined8 local_168;
  undefined8 uStack_160;
  undefined8 local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  undefined8 local_138;
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 *local_108;
  undefined8 uStack_100;
  undefined8 **local_f8;
  undefined8 uStack_f0;
  longlong local_e8;
  undefined **ppuStack_e0;
  undefined8 local_d8;
  undefined8 uStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  char *local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  undefined8 local_38;
  undefined8 uStack_30;
  undefined1 local_21;
  undefined8 local_20;
  
  local_20 = 0xfffffffffffffffe;
  uVar2 = 0x8e1bd251962ba784;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX != 0) && (uVar2 < *(ulonglong *)(in_RDX + 0x218))) &&
     (uVar2 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar2 * 8), uVar2 != 0)) {
    lVar1 = ~uVar2 * 0x60;
    if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar1) != 0) {
      *(undefined4 *)(*(longlong *)(in_RDX + 0x1e0) + lVar1 + 0x5c) =
           *(undefined4 *)(in_RDX + 0x5b0);
      bevy_ecs::schedule::schedule::Schedules::
      add_systems<tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)>_>,bevy_state::state::transitions::OnExit<country_core::app_loading_state::AppLoadingState>,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::tree_storage::TreeStorage>)>
                ();
      bevy_ecs::schedule::config::IntoScheduleConfigs::
      in_set<void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>),alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>,tuple_<bevy_ecs::schedule::config::Infallible,tuple_<bevy_ecs::system::function_system::IsFunctionSystem,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_buffer::ReplayBuffer>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::InsertTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::tree_storage::MoveTreeCmd>,bevy_ecs::event::writer::EventWriter<country_core::resources::history::tree_edit::DeleteTreeCmd>,bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>)>_>,country_core::systems::GameSystemSet>
                ();
      _ZN8bevy_ecs8schedule6config19IntoScheduleConfigs21chain_ignore_deferred17h76a87a28934c2e1dE()
      ;
      local_21 = 0x1b;
      bevy_ecs::intern::Interner::intern<dyn_<bevy_ecs::schedule::set::SystemSet>_>();
      bevy_ecs::schedule::config::ScheduleConfigs::
      in_set_inner<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>
                ();
      local_38 = local_118;
      uStack_30 = uStack_110;
      local_48 = local_128;
      uStack_40 = uStack_120;
      local_58 = local_138;
      uStack_50 = uStack_130;
      local_68 = local_148;
      uStack_60 = uStack_140;
      local_78 = local_158;
      uStack_70 = uStack_150;
      local_88 = local_168;
      uStack_80 = uStack_160;
      local_98 = local_178;
      uStack_90 = uStack_170;
      local_108 = local_1e8;
      uStack_100 = pcStack_1e0;
      local_f8 = (undefined8 **)local_1d8;
      uStack_f0 = uStack_1d0;
      local_e8 = local_1c8;
      ppuStack_e0 = (undefined **)uStack_1c0;
      local_d8 = local_1b8;
      uStack_d0 = uStack_1b0;
      local_c8 = local_1a8;
      uStack_c0 = uStack_1a0;
      local_b8 = local_198;
      uStack_b0 = uStack_190;
      local_a8 = local_188;
      uStack_a0 = uStack_180;
      ppuVar3 = &local_108;
      bevy_app::sub_app::SubApp::
      add_systems<tuple_<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple_<>,tuple_<>_>,bevy_app::main_schedule::Update,tuple_<enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>,enum2_<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn_<bevy_ecs::system::system::System<assoc_<In,tuple_<>_>,assoc_<Out,enum2_<core::result::Result<tuple_<>,bevy_ecs::error::bevy_error::BevyError>_>_>_>_>,alloc::alloc::Global>_>_>_>_>
                ();
      std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
      if ((*extraout_RAX_00 & 1) == 0) {
        std::sys::random::hashmap_random_keys();
        extraout_RAX_00[0] = 1;
        extraout_RAX_00[1] = 0;
        extraout_RAX_00[2] = 0;
        extraout_RAX_00[3] = 0;
        extraout_RAX_00[4] = 0;
        extraout_RAX_00[5] = 0;
        extraout_RAX_00[6] = 0;
        extraout_RAX_00[7] = 0;
        *(longlong *)(extraout_RAX_00 + 8) = extraout_RAX_01;
        *(undefined8 ***)(extraout_RAX_00 + 0x10) = ppuVar3;
        local_e8 = extraout_RAX_01;
      }
      else {
        local_e8 = *(longlong *)(extraout_RAX_00 + 8);
        ppuVar3 = *(undefined8 ***)(extraout_RAX_00 + 0x10);
      }
      *(longlong *)(extraout_RAX_00 + 8) = local_e8 + 1;
      local_f8 = (undefined8 **)_DAT_142a110d0;
      uStack_f0 = _UNK_142a110d8;
      local_108 = (undefined8 *)PTR_DAT_142a110c0;
      uStack_100 = _UNK_142a110c8;
      local_d8 = 0;
      ppuStack_e0 = (undefined **)ppuVar3;
      bevy_app::app::App::insert_resource<country_core::resources::tree_storage::TreeStorage>();
      uVar2 = 0xb02b76b45f2998a1;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_03 == 0) || (*(ulonglong *)(extraout_RAX_02 + 0x218) <= uVar2)) ||
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<system_tree::ui_place_tree::PlaceTreeCmd>();
      }
      uVar2 = 0x69b920782825a2aa;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_04 == 0) || (*(ulonglong *)(extraout_RAX_02 + 0x218) <= uVar2)) ||
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::tree_storage::MoveTreeCmd>();
      }
      uVar2 = 0x6fc458aeefff4088;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_05 == 0) || (*(ulonglong *)(extraout_RAX_02 + 0x218) <= uVar2)) ||
          (uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::history::tree_edit::DeleteTreeCmd>();
      }
      uVar2 = 0x1717ac7cb3b711a2;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_06 == 0) || (*(ulonglong *)(extraout_RAX_02 + 0x218) <= uVar2)) ||
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::history::tree_edit::InsertTreeCmd>();
      }
      uVar2 = 0xfdf3bb7477d0754d;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_07 == 0) || (*(ulonglong *)(extraout_RAX_02 + 0x218) <= uVar2)) ||
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 == 0 ||
          (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)))) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::history::tree_edit::SpawnPreplacedTrees>();
      }
      uVar2 = 0x73b63ddf2555564f;
      bevy_ecs::component::Components::get_resource_id();
      if ((((extraout_RAX_08 == 0) || (*(ulonglong *)(extraout_RAX_02 + 0x218) <= uVar2)) ||
          (uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 == 0))
         || (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) == 0)) {
        bevy_ecs::event::registry::EventRegistry::
        register_event<country_core::resources::tree_storage::HideTreesinAreaCmd>();
      }
      uVar2 = 0x8fee802165add7a6;
      bevy_ecs::component::Components::get_resource_id();
      if (((extraout_RAX_09 != 0) && (uVar2 < *(ulonglong *)(extraout_RAX_02 + 0x218))) &&
         ((uVar2 = *(ulonglong *)(*(longlong *)(extraout_RAX_02 + 0x210) + uVar2 * 8), uVar2 != 0 &&
          (*(longlong *)(*(longlong *)(extraout_RAX_02 + 0x1e0) + 0x20 + ~uVar2 * 0x60) != 0)))) {
        return;
      }
      bevy_ecs::event::registry::EventRegistry::
      register_event<country_core::resources::tree_storage::HideTreesNextToWallCmd>();
      return;
    }
  }
  local_178 = 
  "bevy_ecs::schedule::schedule::SchedulesRequested resource  does not exist in the `World`.\n                Did you forget to add it using `app.insert_resource` / `app.init_resource`?\n                Resources are also implicitly added via `app.add_event`,\n                and can be added by plugins."
  ;
  uStack_170 = 0x27;
  local_1e8 = &local_178;
  pcStack_1e0 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_108 = (undefined8 *)&anon_53ecfc27c5a872a92699da0087a74362_5_llvm_7307910103716307508;
  uStack_100 = 2;
  local_e8 = 0;
  local_f8 = &local_1e8;
  uStack_f0 = 1;
                    /* WARNING: Subroutine does not return */
  core::panicking::panic_fmt();
}

