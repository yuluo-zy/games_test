// 0x1409ba540 build_draw_lists size=11408

void __cdecl country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists(void)

{
  byte *pbVar1;
  size_t *psVar2;
  byte bVar3;
  byte bVar4;
  byte bVar5;
  ulonglong uVar6;
  ushort uVar7;
  bool bVar8;
  bool bVar9;
  bool bVar10;
  char extraout_AL;
  uint uVar11;
  int iVar12;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar13;
  longlong extraout_RAX_02;
  ulonglong *_Buf1;
  uint7 uVar14;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  undefined4 uVar15;
  undefined4 uVar16;
  longlong in_RCX;
  longlong *plVar17;
  longlong lVar18;
  undefined8 uVar19;
  ulonglong *puVar20;
  char cVar21;
  ulonglong in_RDX;
  ulonglong uVar22;
  size_t _Size;
  undefined8 uVar23;
  undefined4 *puVar24;
  char cVar25;
  undefined4 uVar26;
  undefined *puVar27;
  ulonglong uVar28;
  size_t sVar29;
  char cVar30;
  bool bVar31;
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  undefined1 auVar41 [16];
  undefined1 auVar42 [16];
  undefined1 auVar43 [16];
  undefined1 auVar44 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 auVar51 [16];
  undefined1 auVar52 [16];
  undefined1 auVar53 [16];
  undefined1 auVar54 [16];
  undefined1 auVar55 [16];
  undefined1 auVar56 [16];
  undefined1 auVar57 [16];
  undefined1 auVar58 [16];
  undefined1 auVar59 [16];
  undefined1 auVar60 [16];
  undefined1 auVar61 [16];
  undefined1 auVar62 [16];
  undefined1 auVar63 [16];
  undefined1 auVar64 [16];
  undefined1 auVar65 [16];
  undefined1 auVar66 [16];
  undefined1 auVar67 [16];
  undefined1 auVar68 [16];
  undefined1 auVar69 [16];
  undefined1 auVar70 [16];
  undefined1 auVar71 [16];
  undefined1 auVar72 [16];
  longlong local_4d0;
  ulonglong local_3c0;
  ulonglong local_3b8;
  ulonglong local_3b0;
  ulonglong local_3a8;
  ulonglong local_3a0;
  ulonglong local_398;
  ulonglong local_390;
  ulonglong local_388;
  ulonglong local_380;
  uint *local_320;
  uint *local_318;
  undefined8 local_310;
  longlong local_300;
  ulonglong local_2f0;
  ulonglong local_2e8;
  longlong local_2e0;
  byte local_2d8;
  longlong local_2d0;
  byte local_2c8;
  longlong local_2c0;
  byte local_2b8;
  longlong local_2b0;
  byte local_2a8;
  undefined8 local_2a0;
  byte local_298;
  longlong local_290;
  longlong local_288;
  undefined8 local_280;
  byte local_270;
  byte local_268;
  undefined1 local_220 [8];
  size_t local_218;
  longlong local_210;
  undefined8 local_1f8;
  longlong local_1f0;
  uint *local_1e8;
  longlong local_1e0;
  longlong local_1d8;
  undefined8 local_1d0;
  longlong local_1c8;
  longlong local_1c0;
  longlong local_1b8;
  longlong local_1b0;
  ulonglong local_1a8;
  longlong local_1a0;
  longlong local_198;
  undefined8 local_190;
  undefined4 local_188;
  undefined4 local_184;
  longlong *local_180;
  undefined4 local_174;
  uint local_170;
  undefined4 local_168;
  uint local_164;
  uint local_160;
  uint local_15c;
  undefined4 local_158;
  undefined4 local_154;
  uint local_150;
  uint local_14c;
  longlong local_148;
  undefined4 *local_140;
  undefined8 local_138;
  undefined8 local_130;
  undefined4 local_124;
  undefined4 local_120;
  undefined4 local_11c;
  uint local_118;
  uint local_114;
  undefined4 local_110;
  undefined4 local_10c;
  undefined8 local_108;
  code *local_100;
  undefined4 local_f8;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  local_180 = (longlong *)in_RCX;
  puffin::are_scopes_on();
  if (extraout_AL == '\0') {
    local_210 = 0;
  }
  else {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (in_RDX != 0) {
        if (in_RDX < 0x4d) {
          if ((char)(&DAT_142abe3f3)[in_RDX] < -0x40) {
LAB_1409ba610:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX != 0x4d) goto LAB_1409ba610;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (in_RDX != 0xfffffffffffffffe)) {
        if (in_RDX + 2 < 0x4d) {
          if ("untry_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::f"
              [in_RDX] < -0x40) {
LAB_1409ba664:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (in_RDX + 2 != 0x4d) goto LAB_1409ba664;
      }
    }
    puVar27 = &DAT_142abe645;
    do {
      bVar3 = puVar27[-1];
      uVar11 = (uint)(char)bVar3;
      if ((char)bVar3 < '\0') {
        bVar4 = puVar27[-2];
        if ((char)bVar4 < -0x40) {
          bVar5 = puVar27[-3];
          if ((char)bVar5 < -0x40) {
            pbVar1 = puVar27 + -4;
            puVar27 = puVar27 + -4;
            uVar11 = bVar5 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar27 = puVar27 + -3;
            uVar11 = bVar5 & 0xf;
          }
          uVar11 = bVar4 & 0x3f | uVar11 << 6;
        }
        else {
          puVar27 = puVar27 + -2;
          uVar11 = bVar4 & 0x1f;
        }
        uVar11 = (uint)(bVar3 & 0x3f) | uVar11 << 6;
        if (uVar11 != 0x5c) goto LAB_1409ba724;
LAB_1409ba735:
        if (puVar27 == &UNK_142abe5ff) break;
        if (puVar27 + -0x142abe5ff < (undefined *)0x45) {
          if (-0x41 < (char)puVar27[1]) break;
        }
        else if (puVar27 + -0x142abe5ff == (undefined *)0x45) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar27 = puVar27 + -1;
      if (uVar11 == 0x5c) goto LAB_1409ba735;
LAB_1409ba724:
      if (uVar11 == 0x2f) goto LAB_1409ba735;
    } while (puVar27 != &DAT_142abe600);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar17 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar17 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_1409bd622;
    if (*plVar17 != 0) goto LAB_1409bd62e;
    *plVar17 = -1;
    local_1e8 = (uint *)plVar17;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_1e8 = *(longlong *)local_1e8 + 1;
    local_210 = 1;
  }
  bevy_ecs::query::state::QueryState::
  new<tuple_<bevy_ecs::entity::Entity,ref_<country_core::asset_libraries::Handle<country_core::render::vao::Vao>_>,ref_<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>_>,ref_<utils::components::transform::Transform>,enum2_<core::option::Option<ref_<enum2_<country_core::components::drawable::DrawTopology>_>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::DesignatedRenderPass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::UiPass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::WireframePass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::DontCastShadow>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::OcclusionCulled>_>_>_>,tuple_<>_>
            ();
  local_148 = 0;
  local_140 = (undefined4 *)&DAT_00000004;
  local_138 = 0;
  bevy_ecs::query::state::QueryState::
  update_archetypes_unsafe_world_cell<tuple_<bevy_ecs::entity::Entity,ref_<country_core::asset_libraries::Handle<country_core::render::vao::Vao>_>,ref_<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>_>,ref_<utils::components::transform::Transform>,enum2_<core::option::Option<ref_<enum2_<country_core::components::drawable::DrawTopology>_>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::DesignatedRenderPass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::UiPass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::WireframePass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::DontCastShadow>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::OcclusionCulled>_>_>_>,tuple_<>_>
            ();
  if (local_4d0 != *(longlong *)((longlong)local_180 + 0x5a0)) {
                    // WARNING: Subroutine does not return
    bevy_ecs::query::state::impl$2::validate_world::panic_mismatched();
  }
  bevy_ecs::query::iter::QueryIterationCursor::
  init<tuple_<bevy_ecs::entity::Entity,ref_<country_core::asset_libraries::Handle<country_core::render::vao::Vao>_>,ref_<country_core::asset_libraries::Handle<country_core::render::shader::RasterShaderProgram>_>,ref_<utils::components::transform::Transform>,enum2_<core::option::Option<ref_<enum2_<country_core::components::drawable::DrawTopology>_>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::DesignatedRenderPass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::UiPass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::WireframePass>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::DontCastShadow>_>_>,enum2_<core::option::Option<ref_<country_core::components::drawable::OcclusionCulled>_>_>_>,tuple_<>_>
            ();
  local_1f8 = local_310;
  local_1f0 = local_300;
  local_1a8 = local_2e8;
  local_1c8 = local_2e0;
  local_150 = (uint)local_2d8;
  local_1c0 = local_2d0;
  local_14c = (uint)local_2c8;
  local_1e0 = local_2c0;
  local_164 = (uint)local_2b8;
  local_1d8 = local_2b0;
  local_160 = (uint)local_2a8;
  local_1d0 = local_2a0;
  local_15c = (uint)local_298;
  local_1a0 = local_290;
  local_198 = local_288;
  local_190 = local_280;
  local_170 = (uint)local_270;
  if ((local_268 & 1) == 0) {
    if (local_2e8 == local_2f0) {
      do {
        if (local_320 == local_318) goto LAB_1409bd50e;
        uVar11 = *local_320;
        local_320 = local_320 + 1;
      } while (*(longlong *)
                (*(longlong *)((longlong)local_180 + 0x100) + 0x58 + (ulonglong)uVar11 * 0xa0) == 0)
      ;
      lVar13 = *(longlong *)((longlong)local_180 + 0x100) + (ulonglong)uVar11 * 0xa0;
      lVar18 = *(longlong *)((longlong)local_180 + 0x1a8) +
               (ulonglong)*(uint *)(lVar13 + 0x94) * 0x48;
      uVar22 = *(ulonglong *)
                (*(longlong *)((longlong)local_180 + 0x1a8) + 0x40 +
                (ulonglong)*(uint *)(lVar13 + 0x94) * 0x48);
      if ((local_3c0 < uVar22) &&
         (uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3c0 * 8), uVar28 != 0)) {
        local_1a0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        if (uVar22 <= local_3b8) goto LAB_1409bac52;
LAB_1409babbb:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3b8 * 8);
        if (uVar28 == 0) goto LAB_1409bac52;
        local_198 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        if (local_3b0 < uVar22) goto LAB_1409bac6d;
LAB_1409bad19:
        local_190 = 0;
        uVar28 = *(ulonglong *)(lVar13 + 0x88);
        if (uVar28 <= local_3a8) goto LAB_1409badf5;
LAB_1409bad3b:
        if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + local_3a8 * 8) == 0) goto LAB_1409badf5;
        local_150 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar13 + 0x80) >> 8),1);
        if ((uVar22 <= local_3a8) ||
           (uVar6 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3a8 * 8), uVar6 == 0)) {
          local_1c8 = 0;
          goto joined_r0x0001409bae09;
        }
        local_1c8 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar6 * 0x30);
        if (local_3a0 < uVar28) goto LAB_1409baf8f;
LAB_1409bafe6:
        local_14c = 0;
joined_r0x0001409bb013:
        if (uVar28 <= local_398) goto LAB_1409bb06b;
LAB_1409bb015:
        if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + local_398 * 8) == 0) goto LAB_1409bb06b;
        local_164 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar13 + 0x80) >> 8),1);
        if ((uVar22 <= local_398) ||
           (uVar6 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_398 * 8), uVar6 == 0)) {
          local_1e0 = 0;
          goto joined_r0x0001409bb07f;
        }
        local_1e0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar6 * 0x30);
        if (local_390 < uVar28) goto LAB_1409bb09a;
LAB_1409bb0f0:
        local_160 = 0;
joined_r0x0001409bb11d:
        if (uVar28 <= local_388) goto LAB_1409bb172;
LAB_1409bb11f:
        if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + local_388 * 8) == 0) goto LAB_1409bb172;
        local_15c = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar13 + 0x80) >> 8),1);
        if ((uVar22 <= local_388) ||
           (uVar22 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_388 * 8), uVar22 == 0)) {
          local_1d0 = 0;
          goto joined_r0x0001409bb186;
        }
        local_1d0 = *(undefined8 *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar22 * 0x30);
        if (local_380 < uVar28) goto LAB_1409bb1a1;
LAB_1409bb1b9:
        local_170 = 0;
      }
      else {
        local_1a0 = 0;
        if (local_3b8 < uVar22) goto LAB_1409babbb;
LAB_1409bac52:
        local_198 = 0;
        if (uVar22 <= local_3b0) goto LAB_1409bad19;
LAB_1409bac6d:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3b0 * 8);
        if (uVar28 == 0) goto LAB_1409bad19;
        local_190 = *(undefined8 *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        uVar28 = *(ulonglong *)(lVar13 + 0x88);
        if (local_3a8 < uVar28) goto LAB_1409bad3b;
LAB_1409badf5:
        local_150 = 0;
joined_r0x0001409bae09:
        if (uVar28 <= local_3a0) goto LAB_1409bafe6;
LAB_1409baf8f:
        if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + local_3a0 * 8) == 0) goto LAB_1409bafe6;
        local_14c = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar13 + 0x80) >> 8),1);
        if ((uVar22 <= local_3a0) ||
           (uVar6 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3a0 * 8), uVar6 == 0)) {
          local_1c0 = 0;
          goto joined_r0x0001409bb013;
        }
        local_1c0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar6 * 0x30);
        if (local_398 < uVar28) goto LAB_1409bb015;
LAB_1409bb06b:
        local_164 = 0;
joined_r0x0001409bb07f:
        if (uVar28 <= local_390) goto LAB_1409bb0f0;
LAB_1409bb09a:
        if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + local_390 * 8) == 0) goto LAB_1409bb0f0;
        local_160 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar13 + 0x80) >> 8),1);
        if ((uVar22 <= local_390) ||
           (uVar6 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_390 * 8), uVar6 == 0)) {
          local_1d8 = 0;
          goto joined_r0x0001409bb11d;
        }
        local_1d8 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar6 * 0x30);
        if (local_388 < uVar28) goto LAB_1409bb11f;
LAB_1409bb172:
        local_15c = 0;
joined_r0x0001409bb186:
        if (uVar28 <= local_380) goto LAB_1409bb1b9;
LAB_1409bb1a1:
        if (*(longlong *)(*(longlong *)(lVar13 + 0x80) + local_380 * 8) == 0) goto LAB_1409bb1b9;
        local_170 = (uint)CONCAT71((int7)(local_380 >> 8),1);
      }
      local_1f0 = *(longlong *)(lVar13 + 0x50);
      local_1a8 = 0;
    }
    uVar22 = (ulonglong)*(uint *)(local_1f0 + 8 + local_1a8 * 0x10);
    local_1e8 = local_320;
  }
  else {
    if (local_2e8 == local_2f0) {
      do {
        if (local_320 == local_318) goto LAB_1409bd50e;
        uVar11 = *local_320;
        local_320 = local_320 + 1;
      } while (*(longlong *)
                (*(longlong *)((longlong)local_180 + 0x1a8) + 0x10 + (ulonglong)uVar11 * 0x48) == 0)
      ;
      lVar18 = *(longlong *)((longlong)local_180 + 0x1a8) + (ulonglong)uVar11 * 0x48;
      uVar22 = *(ulonglong *)(lVar18 + 0x40);
      if ((local_3c0 < uVar22) &&
         (uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3c0 * 8), uVar28 != 0)) {
        local_1a0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        if (uVar22 <= local_3b8) goto LAB_1409babf9;
LAB_1409bab62:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3b8 * 8);
        if (uVar28 == 0) goto LAB_1409babf9;
        local_198 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        if (local_3b0 < uVar22) goto LAB_1409bac14;
LAB_1409bacb5:
        local_190 = 0;
        if (uVar22 <= local_3a8) goto LAB_1409bada1;
LAB_1409bacd0:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3a8 * 8);
        if (uVar28 == 0) goto LAB_1409bada1;
        local_1c8 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        local_150 = (uint)CONCAT71((int7)((ulonglong)local_1c8 >> 8),1);
        if (local_3a0 < uVar22) goto LAB_1409badb7;
LAB_1409bae14:
        local_14c = 0;
        if (uVar22 <= local_398) goto LAB_1409bae68;
LAB_1409bae2a:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_398 * 8);
        if (uVar28 == 0) goto LAB_1409bae68;
        local_1e0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        local_164 = (uint)CONCAT71((int7)((ulonglong)local_1e0 >> 8),1);
        if (local_390 < uVar22) goto LAB_1409bae7e;
LAB_1409baebc:
        local_160 = 0;
        if (uVar22 <= local_388) goto LAB_1409baf1a;
LAB_1409baed2:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_388 * 8);
        if (uVar28 == 0) goto LAB_1409baf1a;
        local_1d0 = *(undefined8 *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        local_15c = (uint)CONCAT71((int7)((ulonglong)local_1d0 >> 8),1);
        if (local_380 < uVar22) goto LAB_1409baf30;
LAB_1409baf0e:
        local_170 = 0;
      }
      else {
        local_1a0 = 0;
        if (local_3b8 < uVar22) goto LAB_1409bab62;
LAB_1409babf9:
        local_198 = 0;
        if (uVar22 <= local_3b0) goto LAB_1409bacb5;
LAB_1409bac14:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3b0 * 8);
        if (uVar28 == 0) goto LAB_1409bacb5;
        local_190 = *(undefined8 *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        if (local_3a8 < uVar22) goto LAB_1409bacd0;
LAB_1409bada1:
        local_150 = 0;
        if (uVar22 <= local_3a0) goto LAB_1409bae14;
LAB_1409badb7:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_3a0 * 8);
        if (uVar28 == 0) goto LAB_1409bae14;
        local_1c0 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        local_14c = (uint)CONCAT71((int7)((ulonglong)local_1c0 >> 8),1);
        if (local_398 < uVar22) goto LAB_1409bae2a;
LAB_1409bae68:
        local_164 = 0;
        if (uVar22 <= local_390) goto LAB_1409baebc;
LAB_1409bae7e:
        uVar28 = *(ulonglong *)(*(longlong *)(lVar18 + 0x38) + local_390 * 8);
        if (uVar28 == 0) goto LAB_1409baebc;
        local_1d8 = *(longlong *)(*(longlong *)(lVar18 + 0x18) + 0x10 + ~uVar28 * 0x30);
        local_160 = (uint)CONCAT71((int7)((ulonglong)local_1d8 >> 8),1);
        if (local_388 < uVar22) goto LAB_1409baed2;
LAB_1409baf1a:
        local_15c = 0;
        if (uVar22 <= local_380) goto LAB_1409baf0e;
LAB_1409baf30:
        local_170 = (uint)CONCAT71((int7)((ulonglong)*(longlong *)(lVar18 + 0x38) >> 8),
                                   *(longlong *)(*(longlong *)(lVar18 + 0x38) + local_380 * 8) != 0)
        ;
      }
      local_1f8 = *(undefined8 *)(lVar18 + 8);
      local_1a8 = 0;
    }
    uVar22 = local_1a8 & 0xffffffff;
    local_1e8 = local_320;
  }
  local_1b8 = local_1a0 + uVar22 * 8;
  psVar2 = (size_t *)(local_198 + uVar22 * 8);
  local_1b0 = local_1c8 + uVar22 * 8;
  if ((local_150 & 1) == 0) {
    local_1b0 = 0;
  }
  puVar24 = (undefined4 *)(local_1c0 + uVar22 * 4);
  if ((local_14c & 1) == 0) {
    puVar24 = (undefined4 *)0x0;
  }
  uVar22 = 0x8597ee0727b461b3;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_02 == 0) || (*(ulonglong *)((longlong)local_180 + 0x218) <= uVar22)) ||
     (uVar22 = *(ulonglong *)(*(longlong *)((longlong)local_180 + 0x210) + uVar22 * 8), uVar22 == 0)
     ) {
LAB_1409bd666:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar18 = ~uVar22 * 0x60;
  if (*(longlong *)(*(longlong *)((longlong)local_180 + 0x1e0) + 0x20 + lVar18) == 0)
  goto LAB_1409bd666;
  uVar19 = *(undefined8 *)(*(longlong *)((longlong)local_180 + 0x1e0) + lVar18 + 0x10);
  _Size = *psVar2;
  asset_libraries::shader_library::AssetShaderLibrary::debug_raster_get_name();
  if (_Buf1 == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::expect_failed();
  }
  uVar23 = 0x6972625f6c6c6177;
  local_218 = _Size;
  if (_Size == 0xb) {
    uVar23 = 0x79706f6e61635f65;
    if (*(longlong *)((longlong)_Buf1 + 3) != 0x79706f6e61635f65 || *_Buf1 != 0x6e61635f65657274) {
LAB_1409bb34b:
      uVar15 = 0;
      bVar9 = false;
      if (_Size != 0x11) goto LAB_1409bb460;
      auVar32[0] = -((char)*_Buf1 == 'n');
      auVar32[1] = -(*(char *)((longlong)_Buf1 + 1) == 'a');
      auVar32[2] = -(*(char *)((longlong)_Buf1 + 2) == 'n');
      auVar32[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
      auVar32[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
      auVar32[5] = -(*(char *)((longlong)_Buf1 + 5) == 'w');
      auVar32[6] = -(*(char *)((longlong)_Buf1 + 6) == 'o');
      auVar32[7] = -(*(char *)((longlong)_Buf1 + 7) == 'o');
      auVar32[8] = -((char)_Buf1[1] == 'd');
      auVar32[9] = -(*(char *)((longlong)_Buf1 + 9) == 'e');
      auVar32[10] = -(*(char *)((longlong)_Buf1 + 10) == 'n');
      auVar32[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == '_');
      auVar32[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'p');
      auVar32[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'l');
      auVar32[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'a');
      auVar32[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'n');
      auVar53[0] = -((char)_Buf1[2] == 'k');
      auVar53[1] = 0xff;
      auVar53[2] = 0xff;
      auVar53[3] = 0xff;
      auVar53[4] = 0xff;
      auVar53[5] = 0xff;
      auVar53[6] = 0xff;
      auVar53[7] = 0xff;
      auVar53[8] = 0xff;
      auVar53[9] = 0xff;
      auVar53[10] = 0xff;
      auVar53[0xb] = 0xff;
      auVar53[0xc] = 0xff;
      auVar53[0xd] = 0xff;
      auVar53[0xe] = 0xff;
      auVar53[0xf] = 0xff;
      auVar53 = auVar53 & auVar32;
      uVar11 = (uint)CONCAT71((int7)((ulonglong)uVar23 >> 8),1);
      if ((ushort)((ushort)(SUB161(auVar53 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar53 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar53 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar53 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar53 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar53 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar53 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar53 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar53 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar53 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar53 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar53 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar53 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar53 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar53 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar53[0xf] >> 7) << 0xf) == 0xffff) {
        local_f8 = 0x157c;
        local_100 = core::ops::function::FnOnce::
                    call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_0,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
        ;
        goto LAB_1409bc700;
      }
LAB_1409bb385:
      bVar9 = false;
      local_154 = 0;
      local_114 = uVar11 ^ 1;
      bVar31 = false;
      uVar14 = 0;
      sVar29 = CONCAT71((int7)((ulonglong)local_180 >> 8),_Size == 0x10);
      uVar15 = (undefined4)sVar29;
      local_168 = 0;
joined_r0x0001409bb676:
      if (_Size != 0x10) {
        uVar15 = (undefined4)sVar29;
        if (_Size != 0xb) {
          if (_Size == 0x20) {
            auVar40[0] = -((char)*_Buf1 == 'i');
            auVar40[1] = -(*(char *)((longlong)_Buf1 + 1) == 'n');
            auVar40[2] = -(*(char *)((longlong)_Buf1 + 2) == 's');
            auVar40[3] = -(*(char *)((longlong)_Buf1 + 3) == 't');
            auVar40[4] = -(*(char *)((longlong)_Buf1 + 4) == 'a');
            auVar40[5] = -(*(char *)((longlong)_Buf1 + 5) == 'n');
            auVar40[6] = -(*(char *)((longlong)_Buf1 + 6) == 'c');
            auVar40[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
            auVar40[8] = -((char)_Buf1[1] == 'd');
            auVar40[9] = -(*(char *)((longlong)_Buf1 + 9) == '_');
            auVar40[10] = -(*(char *)((longlong)_Buf1 + 10) == 'b');
            auVar40[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'a');
            auVar40[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'c');
            auVar40[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'k');
            auVar40[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'g');
            auVar40[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'r');
            auVar60[0] = -((char)_Buf1[2] == 'o');
            auVar60[1] = -(*(char *)((longlong)_Buf1 + 0x11) == 'u');
            auVar60[2] = -(*(char *)((longlong)_Buf1 + 0x12) == 'n');
            auVar60[3] = -(*(char *)((longlong)_Buf1 + 0x13) == 'd');
            auVar60[4] = -(*(char *)((longlong)_Buf1 + 0x14) == '_');
            auVar60[5] = -(*(char *)((longlong)_Buf1 + 0x15) == 't');
            auVar60[6] = -(*(char *)((longlong)_Buf1 + 0x16) == 'r');
            auVar60[7] = -(*(char *)((longlong)_Buf1 + 0x17) == 'e');
            auVar60[8] = -((char)_Buf1[3] == 'e');
            auVar60[9] = -(*(char *)((longlong)_Buf1 + 0x19) == '_');
            auVar60[10] = -(*(char *)((longlong)_Buf1 + 0x1a) == 's');
            auVar60[0xb] = -(*(char *)((longlong)_Buf1 + 0x1b) == 'h');
            auVar60[0xc] = -(*(char *)((longlong)_Buf1 + 0x1c) == 'a');
            auVar60[0xd] = -(*(char *)((longlong)_Buf1 + 0x1d) == 'd');
            auVar60[0xe] = -(*(char *)((longlong)_Buf1 + 0x1e) == 'e');
            auVar60[0xf] = -(*(char *)((longlong)_Buf1 + 0x1f) == 'r');
            auVar60 = auVar60 & auVar40;
            if ((ushort)((ushort)(SUB161(auVar60 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar60 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar60 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar60 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar60 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar60 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar60 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar60 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar60 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar60 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar60 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar60 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar60 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar60 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar60 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar60[0xf] >> 7) << 0xf) != 0xffff) {
              if (bVar31) {
                local_110 = 0;
                goto LAB_1409bb3e1;
              }
              goto LAB_1409bbc1a;
            }
            local_f8 = 8000;
            local_100 = core::ops::function::FnOnce::
                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_4,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
            ;
            goto LAB_1409bb972;
          }
          if (_Size != 9) {
            bVar8 = false;
            local_110 = 0;
            local_130 = (size_t *)((ulonglong)local_130._4_4_ << 0x20);
            local_11c = 0;
            local_124 = 0;
            local_120 = 0;
            local_118 = uVar11;
            if (bVar31) {
LAB_1409bb7c5:
              puVar20 = _Buf1;
              sVar29 = _Size;
              iVar12 = memcmp(_Buf1,&DAT_142abe6b0,_Size);
              local_120 = (undefined4)CONCAT71((int7)((ulonglong)puVar20 >> 8),1);
              if (iVar12 == 0) {
                local_f8 = 0x1f41;
                local_100 = core::ops::function::FnOnce::
                            call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_5,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                ;
                goto LAB_1409bc146;
              }
            }
            if ((_Size == 0xd) &&
               (*(longlong *)((longlong)_Buf1 + 5) == 0x7265776f6c665f77 &&
                *_Buf1 == 0x665f776f6461656d)) {
              local_f8 = 0x1f42;
              local_100 = core::ops::function::FnOnce::
                          call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_6,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
              ;
              goto LAB_1409bc146;
            }
            local_158 = (undefined4)CONCAT71((int7)(sVar29 >> 8),_Size == 0xd);
            bVar10 = bVar8;
            if ((char)local_130 != '\0') goto LAB_1409bb830;
            uVar26 = 0;
            uVar15 = 0;
            if (_Size == 0xb) goto LAB_1409bb991;
            goto LAB_1409bb9bc;
          }
          if ((char)_Buf1[1] == 'n' && *_Buf1 == 0x6572775f696e616e) {
            local_f8 = 0x1b59;
            local_100 = draw_shaders::instanced_critters::draw_wren_shader;
            local_118 = uVar11;
            goto LAB_1409bc1e2;
          }
          bVar8 = true;
          local_110 = 0;
          local_130 = (size_t *)((ulonglong)local_130._4_4_ << 0x20);
          local_11c = 0;
          local_124 = 0;
          local_120 = 0;
          uVar26 = 0;
          local_158 = 0;
          local_118 = uVar11;
          if (bVar31) goto LAB_1409bb7c5;
LAB_1409bb9c1:
          puVar20 = _Buf1;
          local_10c = uVar26;
          iVar12 = memcmp(_Buf1,&DAT_142abe702,_Size);
          uVar16 = (undefined4)CONCAT71((int7)((ulonglong)puVar20 >> 8),1);
          uVar11 = local_118;
          uVar15 = local_110;
          if (iVar12 == 0) {
            local_f8 = 0x1f47;
            local_100 = core::ops::function::FnOnce::
                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_11,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
            ;
            goto LAB_1409bc146;
          }
joined_r0x0001409bb9ec:
          local_118 = uVar11;
          local_110 = uVar15;
          if ((_Size == 0x19) &&
             (auVar39[0] = -((char)*_Buf1 == 'i'),
             auVar39[1] = -(*(char *)((longlong)_Buf1 + 1) == 'n'),
             auVar39[2] = -(*(char *)((longlong)_Buf1 + 2) == 's'),
             auVar39[3] = -(*(char *)((longlong)_Buf1 + 3) == 't'),
             auVar39[4] = -(*(char *)((longlong)_Buf1 + 4) == 'a'),
             auVar39[5] = -(*(char *)((longlong)_Buf1 + 5) == 'n'),
             auVar39[6] = -(*(char *)((longlong)_Buf1 + 6) == 'c'),
             auVar39[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e'),
             auVar39[8] = -((char)_Buf1[1] == 'd'),
             auVar39[9] = -(*(char *)((longlong)_Buf1 + 9) == '_'),
             auVar39[10] = -(*(char *)((longlong)_Buf1 + 10) == 'i'),
             auVar39[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'v'),
             auVar39[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'y'),
             auVar39[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == '_'),
             auVar39[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'l'),
             auVar39[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e'),
             auVar59[0] = -(*(char *)((longlong)_Buf1 + 9) == '_'),
             auVar59[1] = -(*(char *)((longlong)_Buf1 + 10) == 'i'),
             auVar59[2] = -(*(char *)((longlong)_Buf1 + 0xb) == 'v'),
             auVar59[3] = -(*(char *)((longlong)_Buf1 + 0xc) == 'y'),
             auVar59[4] = -(*(char *)((longlong)_Buf1 + 0xd) == '_'),
             auVar59[5] = -(*(char *)((longlong)_Buf1 + 0xe) == 'l'),
             auVar59[6] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e'),
             auVar59[7] = -((char)_Buf1[2] == 'a'),
             auVar59[8] = -(*(char *)((longlong)_Buf1 + 0x11) == 'f'),
             auVar59[9] = -(*(char *)((longlong)_Buf1 + 0x12) == '_'),
             auVar59[10] = -(*(char *)((longlong)_Buf1 + 0x13) == 's'),
             auVar59[0xb] = -(*(char *)((longlong)_Buf1 + 0x14) == 'h'),
             auVar59[0xc] = -(*(char *)((longlong)_Buf1 + 0x15) == 'a'),
             auVar59[0xd] = -(*(char *)((longlong)_Buf1 + 0x16) == 'd'),
             auVar59[0xe] = -(*(char *)((longlong)_Buf1 + 0x17) == 'e'),
             auVar59[0xf] = -((char)_Buf1[3] == 'r'), auVar59 = auVar59 & auVar39,
             (ushort)((ushort)(SUB161(auVar59 >> 7,0) & 1) |
                      (ushort)(SUB161(auVar59 >> 0xf,0) & 1) << 1 |
                      (ushort)(SUB161(auVar59 >> 0x17,0) & 1) << 2 |
                      (ushort)(SUB161(auVar59 >> 0x1f,0) & 1) << 3 |
                      (ushort)(SUB161(auVar59 >> 0x27,0) & 1) << 4 |
                      (ushort)(SUB161(auVar59 >> 0x2f,0) & 1) << 5 |
                      (ushort)(SUB161(auVar59 >> 0x37,0) & 1) << 6 |
                      (ushort)(SUB161(auVar59 >> 0x3f,0) & 1) << 7 |
                      (ushort)(SUB161(auVar59 >> 0x47,0) & 1) << 8 |
                      (ushort)(SUB161(auVar59 >> 0x4f,0) & 1) << 9 |
                      (ushort)(SUB161(auVar59 >> 0x57,0) & 1) << 10 |
                      (ushort)(SUB161(auVar59 >> 0x5f,0) & 1) << 0xb |
                      (ushort)(SUB161(auVar59 >> 0x67,0) & 1) << 0xc |
                      (ushort)(SUB161(auVar59 >> 0x6f,0) & 1) << 0xd |
                      (ushort)(SUB161(auVar59 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar59[0xf] >> 7) << 0xf) == 0xffff)) {
            local_f8 = 9000;
            local_100 = draw_shaders::instanced_ivy::draw_instanced_ivy_leaf_shader;
            goto LAB_1409bc146;
          }
          bVar31 = _Size == 0x19;
          local_184 = uVar16;
          if (bVar9) goto LAB_1409bbc72;
          local_188 = 0;
          cVar21 = (char)uVar11;
          uVar26 = uVar15;
          local_130 = psVar2;
          goto LAB_1409bbcb8;
        }
        if (*(longlong *)((longlong)_Buf1 + 3) != 0x7265646168735f6b || *_Buf1 != 0x6168735f6b637564
           ) {
          bVar8 = false;
          local_110 = 0;
          local_130 = (size_t *)((ulonglong)local_130._4_4_ << 0x20);
          local_11c = 0;
          local_124 = 0;
          local_158 = 0;
          local_120 = 0;
          uVar15 = 0;
          local_118 = uVar11;
          if (bVar31) goto LAB_1409bb7c5;
LAB_1409bb991:
          uVar26 = uVar15;
          if (*(longlong *)((longlong)_Buf1 + 3) == 0x7265776f6c665f69 &&
              *_Buf1 == 0x6f6c665f696e616e) {
            local_f8 = 0x1f46;
            local_100 = core::ops::function::FnOnce::
                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_10,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
            ;
            goto LAB_1409bc146;
          }
LAB_1409bb9bc:
          if (bVar8) goto LAB_1409bb9c1;
          uVar16 = 0;
          uVar11 = local_118;
          uVar15 = local_110;
          local_10c = uVar26;
          goto joined_r0x0001409bb9ec;
        }
        local_f8 = 0x1b59;
        local_100 = draw_shaders::instanced_critters::draw_duck_shader;
        local_118 = uVar11;
LAB_1409bc1e2:
        local_108 = (undefined1 *)0x2;
        uVar11 = 2;
        goto LAB_1409bc717;
      }
      auVar33[0] = -((char)*_Buf1 == 'b');
      auVar33[1] = -(*(char *)((longlong)_Buf1 + 1) == 'u');
      auVar33[2] = -(*(char *)((longlong)_Buf1 + 2) == 't');
      auVar33[3] = -(*(char *)((longlong)_Buf1 + 3) == 't');
      auVar33[4] = -(*(char *)((longlong)_Buf1 + 4) == 'e');
      auVar33[5] = -(*(char *)((longlong)_Buf1 + 5) == 'r');
      auVar33[6] = -(*(char *)((longlong)_Buf1 + 6) == 'f');
      auVar33[7] = -(*(char *)((longlong)_Buf1 + 7) == 'l');
      auVar33[8] = -((char)_Buf1[1] == 'y');
      auVar33[9] = -(*(char *)((longlong)_Buf1 + 9) == '_');
      auVar33[10] = -(*(char *)((longlong)_Buf1 + 10) == 's');
      auVar33[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'h');
      auVar33[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'a');
      auVar33[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'd');
      auVar33[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'e');
      auVar33[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'r');
      if ((ushort)((ushort)(SUB161(auVar33 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                  (ushort)(auVar33[0xf] >> 7) << 0xf) != 0xffff) {
        if (bVar31) {
          local_110 = (undefined4)CONCAT71(uVar14,1);
LAB_1409bb3e1:
          bVar8 = false;
          local_130 = (size_t *)((ulonglong)local_130 & 0xffffffff00000000);
          local_11c = 0;
          local_124 = 0;
          local_118 = uVar11;
          goto LAB_1409bb7c5;
        }
LAB_1409bbc1a:
        cVar21 = (char)uVar11;
        local_120 = 0;
        local_124 = 0;
        local_11c = 0;
        local_10c = 0;
        local_184 = 0;
        local_158 = 0;
        bVar31 = false;
        local_188 = 0;
        uVar26 = uVar15;
        local_130 = psVar2;
        if (bVar9) {
LAB_1409bbc72:
          cVar21 = (char)uVar11;
          local_130 = psVar2;
          iVar12 = memcmp(_Buf1,&DAT_142abe70b,_Size);
          if (iVar12 == 0) {
            local_f8 = 0x2329;
            local_100 = draw_shaders::instanced_ivy::draw_instanced_ivy_flower_shader;
          }
          else {
            puVar20 = _Buf1;
            iVar12 = memcmp(_Buf1,&DAT_142abe726,_Size);
            local_188 = (undefined4)CONCAT71((int7)((ulonglong)puVar20 >> 8),1);
            uVar26 = uVar15;
            if (iVar12 != 0) goto LAB_1409bbcb8;
            local_f8 = 10000;
            local_100 = draw_shaders::instanced_ivy::draw_instanced_ivy_branch_shader;
          }
          local_108 = (undefined1 *)0x200000001;
LAB_1409bc10e:
          uVar11 = 1;
          goto LAB_1409bc717;
        }
LAB_1409bbcb8:
        if (_Size == 0x1e) {
          auVar61[0] = -(*(char *)((longlong)_Buf1 + 0xe) == 't');
          auVar61[1] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e');
          auVar61[2] = -((char)_Buf1[2] == 'x');
          auVar61[3] = -(*(char *)((longlong)_Buf1 + 0x11) == '_');
          auVar61[4] = -(*(char *)((longlong)_Buf1 + 0x12) == 'c');
          auVar61[5] = -(*(char *)((longlong)_Buf1 + 0x13) == 'o');
          auVar61[6] = -(*(char *)((longlong)_Buf1 + 0x14) == 'l');
          auVar61[7] = -(*(char *)((longlong)_Buf1 + 0x15) == 'o');
          auVar61[8] = -(*(char *)((longlong)_Buf1 + 0x16) == 'r');
          auVar61[9] = -(*(char *)((longlong)_Buf1 + 0x17) == '_');
          auVar61[10] = -((char)_Buf1[3] == 'b');
          auVar61[0xb] = -(*(char *)((longlong)_Buf1 + 0x19) == 'r');
          auVar61[0xc] = -(*(char *)((longlong)_Buf1 + 0x1a) == 'i');
          auVar61[0xd] = -(*(char *)((longlong)_Buf1 + 0x1b) == 'c');
          auVar61[0xe] = -(*(char *)((longlong)_Buf1 + 0x1c) == 'k');
          auVar61[0xf] = -(*(char *)((longlong)_Buf1 + 0x1d) == 's');
          auVar41[0] = -((char)*_Buf1 == 'n');
          auVar41[1] = -(*(char *)((longlong)_Buf1 + 1) == 'a');
          auVar41[2] = -(*(char *)((longlong)_Buf1 + 2) == 'n');
          auVar41[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
          auVar41[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
          auVar41[5] = -(*(char *)((longlong)_Buf1 + 5) == 's');
          auVar41[6] = -(*(char *)((longlong)_Buf1 + 6) == 'o');
          auVar41[7] = -(*(char *)((longlong)_Buf1 + 7) == 'l');
          auVar41[8] = -((char)_Buf1[1] == 'i');
          auVar41[9] = -(*(char *)((longlong)_Buf1 + 9) == 'd');
          auVar41[10] = -(*(char *)((longlong)_Buf1 + 10) == '_');
          auVar41[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'v');
          auVar41[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'e');
          auVar41[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'r');
          auVar41[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 't');
          auVar41[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e');
          auVar41 = auVar41 & auVar61;
          if ((ushort)((ushort)(SUB161(auVar41 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar41 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar41 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar41 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar41 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar41 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar41 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar41 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar41 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar41 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar41 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar41 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar41 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar41 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar41 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar41[0xf] >> 7) << 0xf) == 0xffff) {
            local_f8 = 15000;
            local_100 = core::ops::function::FnOnce::
                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_12,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
            ;
            local_108 = (undefined1 *)0x1;
            goto LAB_1409bc10e;
          }
          auVar62[0] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o');
          auVar62[1] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
          auVar62[2] = -((char)_Buf1[2] == 'o');
          auVar62[3] = -(*(char *)((longlong)_Buf1 + 0x11) == 'r');
          auVar62[4] = -(*(char *)((longlong)_Buf1 + 0x12) == '_');
          auVar62[5] = -(*(char *)((longlong)_Buf1 + 0x13) == 'w');
          auVar62[6] = -(*(char *)((longlong)_Buf1 + 0x14) == 'o');
          auVar62[7] = -(*(char *)((longlong)_Buf1 + 0x15) == 'o');
          auVar62[8] = -(*(char *)((longlong)_Buf1 + 0x16) == 'd');
          auVar62[9] = -(*(char *)((longlong)_Buf1 + 0x17) == 'e');
          auVar62[10] = -((char)_Buf1[3] == 'n');
          auVar62[0xb] = -(*(char *)((longlong)_Buf1 + 0x19) == '_');
          auVar62[0xc] = -(*(char *)((longlong)_Buf1 + 0x1a) == 'g');
          auVar62[0xd] = -(*(char *)((longlong)_Buf1 + 0x1b) == 'a');
          auVar62[0xe] = -(*(char *)((longlong)_Buf1 + 0x1c) == 't');
          auVar62[0xf] = -(*(char *)((longlong)_Buf1 + 0x1d) == 'e');
          auVar42[0] = -((char)*_Buf1 == 's');
          auVar42[1] = -(*(char *)((longlong)_Buf1 + 1) == 'o');
          auVar42[2] = -(*(char *)((longlong)_Buf1 + 2) == 'l');
          auVar42[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
          auVar42[4] = -(*(char *)((longlong)_Buf1 + 4) == 'd');
          auVar42[5] = -(*(char *)((longlong)_Buf1 + 5) == '_');
          auVar42[6] = -(*(char *)((longlong)_Buf1 + 6) == 'v');
          auVar42[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
          auVar42[8] = -((char)_Buf1[1] == 'r');
          auVar42[9] = -(*(char *)((longlong)_Buf1 + 9) == 't');
          auVar42[10] = -(*(char *)((longlong)_Buf1 + 10) == 'e');
          auVar42[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'x');
          auVar42[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == '_');
          auVar42[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c');
          auVar42[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o');
          auVar42[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
          auVar42 = auVar42 & auVar62;
          uVar7 = (ushort)(SUB161(auVar42 >> 7,0) & 1) | (ushort)(SUB161(auVar42 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar42 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar42 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar42 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar42 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar42 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar42 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar42 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar42 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar42 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar42 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar42 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar42 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar42 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar42[0xf] >> 7) << 0xf;
          if (uVar7 != 0xffff) {
            local_118 = (uint)CONCAT71((uint7)(byte)(uVar7 >> 8),1);
            goto joined_r0x0001409bbe7a;
          }
          local_f8 = 0x3a9a;
          local_100 = core::ops::function::FnOnce::
                      call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_14,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
          ;
        }
        else {
          if (!bVar31) goto LAB_1409bbdc9;
          auVar43[0] = -((char)*_Buf1 == 'n');
          auVar43[1] = -(*(char *)((longlong)_Buf1 + 1) == 'a');
          auVar43[2] = -(*(char *)((longlong)_Buf1 + 2) == 'n');
          auVar43[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
          auVar43[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
          auVar43[5] = -(*(char *)((longlong)_Buf1 + 5) == 'g');
          auVar43[6] = -(*(char *)((longlong)_Buf1 + 6) == 'o');
          auVar43[7] = -(*(char *)((longlong)_Buf1 + 7) == 't');
          auVar43[8] = -((char)_Buf1[1] == 'h');
          auVar43[9] = -(*(char *)((longlong)_Buf1 + 9) == 'i');
          auVar43[10] = -(*(char *)((longlong)_Buf1 + 10) == 'c');
          auVar43[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == '_');
          auVar43[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'w');
          auVar43[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'i');
          auVar43[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'n');
          auVar43[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'd');
          auVar63[0] = -(*(char *)((longlong)_Buf1 + 9) == 'i');
          auVar63[1] = -(*(char *)((longlong)_Buf1 + 10) == 'c');
          auVar63[2] = -(*(char *)((longlong)_Buf1 + 0xb) == '_');
          auVar63[3] = -(*(char *)((longlong)_Buf1 + 0xc) == 'w');
          auVar63[4] = -(*(char *)((longlong)_Buf1 + 0xd) == 'i');
          auVar63[5] = -(*(char *)((longlong)_Buf1 + 0xe) == 'n');
          auVar63[6] = -(*(char *)((longlong)_Buf1 + 0xf) == 'd');
          auVar63[7] = -((char)_Buf1[2] == 'o');
          auVar63[8] = -(*(char *)((longlong)_Buf1 + 0x11) == 'w');
          auVar63[9] = -(*(char *)((longlong)_Buf1 + 0x12) == '_');
          auVar63[10] = -(*(char *)((longlong)_Buf1 + 0x13) == 'b');
          auVar63[0xb] = -(*(char *)((longlong)_Buf1 + 0x14) == 'r');
          auVar63[0xc] = -(*(char *)((longlong)_Buf1 + 0x15) == 'i');
          auVar63[0xd] = -(*(char *)((longlong)_Buf1 + 0x16) == 'c');
          auVar63[0xe] = -(*(char *)((longlong)_Buf1 + 0x17) == 'k');
          auVar63[0xf] = -((char)_Buf1[3] == 's');
          auVar63 = auVar63 & auVar43;
          if ((ushort)((ushort)(SUB161(auVar63 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar63 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar63 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar63 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar63 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar63 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar63 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar63 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar63 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar63 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar63 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar63 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar63 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar63 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar63 >> 0x77,0) & 1) << 0xe |
                      (ushort)(byte)(auVar63[0xf] >> 7) << 0xf) != 0xffff) {
            local_174 = 0;
            goto joined_r0x0001409bbe2e;
          }
          local_f8 = 0x3a99;
          local_100 = core::ops::function::FnOnce::
                      call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_13,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
          ;
        }
        goto LAB_1409bc700;
      }
      local_f8 = 7000;
      local_100 = draw_shaders::instanced_critters::draw_instanced_butterfly_shader;
      goto LAB_1409bb972;
    }
    local_f8 = 1000;
    local_100 = draw_shaders::clearing_shaders::tree_canopy;
    local_108 = (undefined1 *)0x300000001;
  }
  else {
    if (_Size == 0x1b) {
      auVar35[0] = -((char)*_Buf1 == 'i');
      auVar35[1] = -(*(char *)((longlong)_Buf1 + 1) == 'n');
      auVar35[2] = -(*(char *)((longlong)_Buf1 + 2) == 's');
      auVar35[3] = -(*(char *)((longlong)_Buf1 + 3) == 't');
      auVar35[4] = -(*(char *)((longlong)_Buf1 + 4) == 'a');
      auVar35[5] = -(*(char *)((longlong)_Buf1 + 5) == 'n');
      auVar35[6] = -(*(char *)((longlong)_Buf1 + 6) == 'c');
      auVar35[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
      auVar35[8] = -((char)_Buf1[1] == 'd');
      auVar35[9] = -(*(char *)((longlong)_Buf1 + 9) == '_');
      auVar35[10] = -(*(char *)((longlong)_Buf1 + 10) == 'r');
      auVar35[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'o');
      auVar35[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'o');
      auVar35[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'f');
      auVar35[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == '_');
      auVar35[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 's');
      auVar55[0] = -(*(char *)((longlong)_Buf1 + 0xb) == 'o');
      auVar55[1] = -(*(char *)((longlong)_Buf1 + 0xc) == 'o');
      auVar55[2] = -(*(char *)((longlong)_Buf1 + 0xd) == 'f');
      auVar55[3] = -(*(char *)((longlong)_Buf1 + 0xe) == '_');
      auVar55[4] = -(*(char *)((longlong)_Buf1 + 0xf) == 's');
      auVar55[5] = -((char)_Buf1[2] == 'p');
      auVar55[6] = -(*(char *)((longlong)_Buf1 + 0x11) == 'i');
      auVar55[7] = -(*(char *)((longlong)_Buf1 + 0x12) == 'r');
      auVar55[8] = -(*(char *)((longlong)_Buf1 + 0x13) == 'e');
      auVar55[9] = -(*(char *)((longlong)_Buf1 + 0x14) == '_');
      auVar55[10] = -(*(char *)((longlong)_Buf1 + 0x15) == 's');
      auVar55[0xb] = -(*(char *)((longlong)_Buf1 + 0x16) == 'h');
      auVar55[0xc] = -(*(char *)((longlong)_Buf1 + 0x17) == 'a');
      auVar55[0xd] = -((char)_Buf1[3] == 'd');
      auVar55[0xe] = -(*(char *)((longlong)_Buf1 + 0x19) == 'e');
      auVar55[0xf] = -(*(char *)((longlong)_Buf1 + 0x1a) == 'r');
      auVar55 = auVar55 & auVar35;
      if ((ushort)((ushort)(SUB161(auVar55 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar55 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar55 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar55 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar55 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar55 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar55 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar55 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar55 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar55 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar55 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar55 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar55 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar55 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar55 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar55[0xf] >> 7) << 0xf) == 0xffff) {
        local_f8 = 0xc1c;
        local_100 = draw_shaders::instanced_roof::draw_instanced_roof_spire_shader;
      }
      else {
        uVar15 = 0;
        bVar9 = true;
LAB_1409bb460:
        local_168 = 0;
        bVar31 = false;
        uVar14 = 0;
        local_154 = uVar15;
joined_r0x0001409bb74f:
        if (_Size != 0xc) {
          if (_Size == 0x15) {
            auVar37[0] = -((char)*_Buf1 == 's');
            auVar37[1] = -(*(char *)((longlong)_Buf1 + 1) == 'n');
            auVar37[2] = -(*(char *)((longlong)_Buf1 + 2) == 'o');
            auVar37[3] = -(*(char *)((longlong)_Buf1 + 3) == 'w');
            auVar37[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
            auVar37[5] = -(*(char *)((longlong)_Buf1 + 5) == 'p');
            auVar37[6] = -(*(char *)((longlong)_Buf1 + 6) == 'a');
            auVar37[7] = -(*(char *)((longlong)_Buf1 + 7) == 'r');
            auVar37[8] = -((char)_Buf1[1] == 't');
            auVar37[9] = -(*(char *)((longlong)_Buf1 + 9) == 'i');
            auVar37[10] = -(*(char *)((longlong)_Buf1 + 10) == 'c');
            auVar37[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'l');
            auVar37[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'e');
            auVar37[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 's');
            auVar37[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == '_');
            auVar37[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 's');
            auVar57[0] = -(*(char *)((longlong)_Buf1 + 5) == 'p');
            auVar57[1] = -(*(char *)((longlong)_Buf1 + 6) == 'a');
            auVar57[2] = -(*(char *)((longlong)_Buf1 + 7) == 'r');
            auVar57[3] = -((char)_Buf1[1] == 't');
            auVar57[4] = -(*(char *)((longlong)_Buf1 + 9) == 'i');
            auVar57[5] = -(*(char *)((longlong)_Buf1 + 10) == 'c');
            auVar57[6] = -(*(char *)((longlong)_Buf1 + 0xb) == 'l');
            auVar57[7] = -(*(char *)((longlong)_Buf1 + 0xc) == 'e');
            auVar57[8] = -(*(char *)((longlong)_Buf1 + 0xd) == 's');
            auVar57[9] = -(*(char *)((longlong)_Buf1 + 0xe) == '_');
            auVar57[10] = -(*(char *)((longlong)_Buf1 + 0xf) == 's');
            auVar57[0xb] = -((char)_Buf1[2] == 'h');
            auVar57[0xc] = -(*(char *)((longlong)_Buf1 + 0x11) == 'a');
            auVar57[0xd] = -(*(char *)((longlong)_Buf1 + 0x12) == 'd');
            auVar57[0xe] = -(*(char *)((longlong)_Buf1 + 0x13) == 'e');
            auVar57[0xf] = -(*(char *)((longlong)_Buf1 + 0x14) == 'r');
            auVar57 = auVar57 & auVar37;
            uVar7 = (ushort)(SUB161(auVar57 >> 7,0) & 1) |
                    (ushort)(SUB161(auVar57 >> 0xf,0) & 1) << 1 |
                    (ushort)(SUB161(auVar57 >> 0x17,0) & 1) << 2 |
                    (ushort)(SUB161(auVar57 >> 0x1f,0) & 1) << 3 |
                    (ushort)(SUB161(auVar57 >> 0x27,0) & 1) << 4 |
                    (ushort)(SUB161(auVar57 >> 0x2f,0) & 1) << 5 |
                    (ushort)(SUB161(auVar57 >> 0x37,0) & 1) << 6 |
                    (ushort)(SUB161(auVar57 >> 0x3f,0) & 1) << 7 |
                    (ushort)(SUB161(auVar57 >> 0x47,0) & 1) << 8 |
                    (ushort)(SUB161(auVar57 >> 0x4f,0) & 1) << 9 |
                    (ushort)(SUB161(auVar57 >> 0x57,0) & 1) << 10 |
                    (ushort)(SUB161(auVar57 >> 0x5f,0) & 1) << 0xb |
                    (ushort)(SUB161(auVar57 >> 0x67,0) & 1) << 0xc |
                    (ushort)(SUB161(auVar57 >> 0x6f,0) & 1) << 0xd |
                    (ushort)(SUB161(auVar57 >> 0x77,0) & 1) << 0xe |
                    (ushort)(byte)(auVar57[0xf] >> 7) << 0xf;
            if (uVar7 == 0xffff) {
              local_f8 = 6000;
              local_100 = core::ops::function::FnOnce::
                          call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_3,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
              ;
              goto LAB_1409bc700;
            }
            local_130 = (size_t *)
                        CONCAT44(local_130._4_4_,(int)CONCAT71((uint7)(byte)(uVar7 >> 8),1));
            bVar8 = false;
            local_110 = 0;
            local_11c = 0;
            local_124 = 0;
            local_118 = 0;
            local_114 = 0;
            local_120 = 0;
            local_158 = 0;
            bVar10 = false;
            if (bVar31) goto LAB_1409bb7c5;
LAB_1409bb830:
            bVar8 = bVar10;
            iVar12 = memcmp(_Buf1,&DAT_142abe6c3,_Size);
            if (iVar12 == 0) {
              local_f8 = 0x1f43;
              local_100 = core::ops::function::FnOnce::
                          call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_7,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
              ;
            }
            else {
              iVar12 = memcmp(_Buf1,&DAT_142abe6d8,_Size);
              if (iVar12 != 0) {
                sVar29 = _Size;
                iVar12 = memcmp(_Buf1,&DAT_142abe6ed,_Size);
                uVar26 = (undefined4)CONCAT71((int7)(sVar29 >> 8),1);
                if (iVar12 != 0) {
                  uVar15 = uVar26;
                  if (_Size != 0xb) goto LAB_1409bb9bc;
                  goto LAB_1409bb991;
                }
                local_f8 = 0x1f45;
                local_100 = core::ops::function::FnOnce::
                            call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_9,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                ;
                goto LAB_1409bc700;
              }
              local_f8 = 0x1f44;
              local_100 = core::ops::function::FnOnce::
                          call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_8,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
              ;
            }
          }
          else {
            if (_Size != 0x17) {
              uVar11 = 0;
              local_114 = 0;
              sVar29 = CONCAT71((int7)((ulonglong)local_180 >> 8),_Size == 0x10);
              uVar15 = (undefined4)sVar29;
              goto joined_r0x0001409bb676;
            }
            auVar54[0] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
            auVar54[1] = -((char)_Buf1[1] == 'r');
            auVar54[2] = -(*(char *)((longlong)_Buf1 + 9) == 't');
            auVar54[3] = -(*(char *)((longlong)_Buf1 + 10) == 'e');
            auVar54[4] = -(*(char *)((longlong)_Buf1 + 0xb) == 'x');
            auVar54[5] = -(*(char *)((longlong)_Buf1 + 0xc) == '_');
            auVar54[6] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c');
            auVar54[7] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o');
            auVar54[8] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
            auVar54[9] = -((char)_Buf1[2] == 'o');
            auVar54[10] = -(*(char *)((longlong)_Buf1 + 0x11) == 'r');
            auVar54[0xb] = -(*(char *)((longlong)_Buf1 + 0x12) == '_');
            auVar54[0xc] = -(*(char *)((longlong)_Buf1 + 0x13) == 'b');
            auVar54[0xd] = -(*(char *)((longlong)_Buf1 + 0x14) == 'o');
            auVar54[0xe] = -(*(char *)((longlong)_Buf1 + 0x15) == 'a');
            auVar54[0xf] = -(*(char *)((longlong)_Buf1 + 0x16) == 't');
            auVar34[0] = -((char)*_Buf1 == 's');
            auVar34[1] = -(*(char *)((longlong)_Buf1 + 1) == 'o');
            auVar34[2] = -(*(char *)((longlong)_Buf1 + 2) == 'l');
            auVar34[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
            auVar34[4] = -(*(char *)((longlong)_Buf1 + 4) == 'd');
            auVar34[5] = -(*(char *)((longlong)_Buf1 + 5) == '_');
            auVar34[6] = -(*(char *)((longlong)_Buf1 + 6) == 'v');
            auVar34[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
            auVar34[8] = -((char)_Buf1[1] == 'r');
            auVar34[9] = -(*(char *)((longlong)_Buf1 + 9) == 't');
            auVar34[10] = -(*(char *)((longlong)_Buf1 + 10) == 'e');
            auVar34[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'x');
            auVar34[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == '_');
            auVar34[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c');
            auVar34[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o');
            auVar34[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
            auVar34 = auVar34 & auVar54;
            if ((ushort)((ushort)(SUB161(auVar34 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar34[0xf] >> 7) << 0xf) != 0xffff) {
              if (bVar31) {
                local_124 = (undefined4)CONCAT71(uVar14,1);
                local_11c = 0;
                goto LAB_1409bb7b1;
              }
              local_124 = (undefined4)CONCAT71(uVar14,1);
              local_11c = 0;
              goto LAB_1409bbb41;
            }
            local_f8 = 0x1676;
            local_100 = core::ops::function::FnOnce::
                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_1,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
            ;
          }
          goto LAB_1409bc146;
        }
        if ((int)_Buf1[1] == 0x72656461 && (*_Buf1 ^ 0x68735f7065656873) == 0) {
          local_f8 = 6000;
          local_100 = core::ops::function::FnOnce::
                      call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_2,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
          ;
          goto LAB_1409bc1e2;
        }
        local_11c = (undefined4)CONCAT71((int7)((*_Buf1 ^ 0x68735f7065656873) >> 8),1);
        if (bVar31) {
          local_124 = 0;
LAB_1409bb7b1:
          local_110 = 0;
          local_130 = (size_t *)((ulonglong)local_130._4_4_ << 0x20);
          bVar8 = false;
          local_118 = 0;
          local_114 = 0;
          goto LAB_1409bb7c5;
        }
        local_124 = 0;
LAB_1409bbb41:
        local_120 = 0;
        local_114 = 0;
        cVar21 = '\0';
        uVar11 = 0;
        local_10c = 0;
        uVar26 = 0;
        uVar15 = 0;
        local_184 = 0;
        local_158 = 0;
        bVar31 = false;
        local_188 = 0;
        local_130 = psVar2;
        if (bVar9) goto LAB_1409bbc72;
LAB_1409bbdc9:
        if (_Size == 0x16) {
          auVar44[0] = -((char)*_Buf1 == 'n');
          auVar44[1] = -(*(char *)((longlong)_Buf1 + 1) == 'a');
          auVar44[2] = -(*(char *)((longlong)_Buf1 + 2) == 'n');
          auVar44[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
          auVar44[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
          auVar44[5] = -(*(char *)((longlong)_Buf1 + 5) == 'c');
          auVar44[6] = -(*(char *)((longlong)_Buf1 + 6) == 'u');
          auVar44[7] = -(*(char *)((longlong)_Buf1 + 7) == 'r');
          auVar44[8] = -((char)_Buf1[1] == 'v');
          auVar44[9] = -(*(char *)((longlong)_Buf1 + 9) == 'e');
          auVar44[10] = -(*(char *)((longlong)_Buf1 + 10) == 'd');
          auVar44[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == '_');
          auVar44[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'h');
          auVar44[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'a');
          auVar44[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'l');
          auVar44[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'f');
          auVar64[0] = -(*(char *)((longlong)_Buf1 + 6) == 'u');
          auVar64[1] = -(*(char *)((longlong)_Buf1 + 7) == 'r');
          auVar64[2] = -((char)_Buf1[1] == 'v');
          auVar64[3] = -(*(char *)((longlong)_Buf1 + 9) == 'e');
          auVar64[4] = -(*(char *)((longlong)_Buf1 + 10) == 'd');
          auVar64[5] = -(*(char *)((longlong)_Buf1 + 0xb) == '_');
          auVar64[6] = -(*(char *)((longlong)_Buf1 + 0xc) == 'h');
          auVar64[7] = -(*(char *)((longlong)_Buf1 + 0xd) == 'a');
          auVar64[8] = -(*(char *)((longlong)_Buf1 + 0xe) == 'l');
          auVar64[9] = -(*(char *)((longlong)_Buf1 + 0xf) == 'f');
          auVar64[10] = -((char)_Buf1[2] == 't');
          auVar64[0xb] = -(*(char *)((longlong)_Buf1 + 0x11) == 'i');
          auVar64[0xc] = -(*(char *)((longlong)_Buf1 + 0x12) == 'm');
          auVar64[0xd] = -(*(char *)((longlong)_Buf1 + 0x13) == 'b');
          auVar64[0xe] = -(*(char *)((longlong)_Buf1 + 0x14) == 'e');
          auVar64[0xf] = -(*(char *)((longlong)_Buf1 + 0x15) == 'r');
          auVar64 = auVar64 & auVar44;
          uVar7 = (ushort)(SUB161(auVar64 >> 7,0) & 1) | (ushort)(SUB161(auVar64 >> 0xf,0) & 1) << 1
                  | (ushort)(SUB161(auVar64 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar64 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar64 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar64 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar64 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar64 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar64 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar64 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar64 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar64 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar64 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar64 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar64 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar64[0xf] >> 7) << 0xf;
          if (uVar7 == 0xffff) {
            local_f8 = 0x3a9a;
            local_100 = core::ops::function::FnOnce::
                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_15,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
            ;
            goto LAB_1409bc700;
          }
          local_174 = (undefined4)CONCAT71((uint7)(byte)(uVar7 >> 8),1);
joined_r0x0001409bbe2e:
          local_118 = 0;
          local_110 = uVar26;
          uVar15 = local_124;
          cVar30 = (char)local_114;
        }
        else {
          local_118 = 0;
joined_r0x0001409bbe7a:
          local_174 = 0;
          local_110 = uVar26;
          uVar15 = local_124;
          cVar30 = (char)local_114;
        }
        local_124 = uVar15;
        if ((cVar30 == '\0') || (iVar12 = memcmp(_Buf1,&DAT_142abe741,_Size), iVar12 != 0)) {
          if ((cVar21 == '\0') || (iVar12 = memcmp(_Buf1,&DAT_142abe750,_Size), iVar12 != 0)) {
            if ((_Size == 0x12) &&
               (auVar65[0] = -((char)(short)_Buf1[2] == 'o'),
               auVar65[1] = -((char)((ushort)(short)_Buf1[2] >> 8) == 'r'), auVar65[2] = 0xff,
               auVar65[3] = 0xff, auVar65[4] = 0xff, auVar65[5] = 0xff, auVar65[6] = 0xff,
               auVar65[7] = 0xff, auVar65[8] = 0xff, auVar65[9] = 0xff, auVar65[10] = 0xff,
               auVar65[0xb] = 0xff, auVar65[0xc] = 0xff, auVar65[0xd] = 0xff, auVar65[0xe] = 0xff,
               auVar65[0xf] = 0xff, auVar45[0] = -((char)*_Buf1 == 's'),
               auVar45[1] = -(*(char *)((longlong)_Buf1 + 1) == 'o'),
               auVar45[2] = -(*(char *)((longlong)_Buf1 + 2) == 'l'),
               auVar45[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i'),
               auVar45[4] = -(*(char *)((longlong)_Buf1 + 4) == 'd'),
               auVar45[5] = -(*(char *)((longlong)_Buf1 + 5) == '_'),
               auVar45[6] = -(*(char *)((longlong)_Buf1 + 6) == 'v'),
               auVar45[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e'),
               auVar45[8] = -((char)_Buf1[1] == 'r'),
               auVar45[9] = -(*(char *)((longlong)_Buf1 + 9) == 't'),
               auVar45[10] = -(*(char *)((longlong)_Buf1 + 10) == 'e'),
               auVar45[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'x'),
               auVar45[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == '_'),
               auVar45[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c'),
               auVar45[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o'),
               auVar45[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l'),
               auVar45 = auVar45 & auVar65,
               (ushort)((ushort)(SUB161(auVar45 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar45 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar45 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar45 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar45 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar45 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar45 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar45 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar45 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar45 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar45 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar45 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar45 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar45 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar45 >> 0x77,0) & 1) << 0xe |
                       (ushort)(byte)(auVar45[0xf] >> 7) << 0xf) == 0xffff)) {
              local_f8 = 0x3a9c;
              local_100 = core::ops::function::FnOnce::
                          call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_18,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
              ;
            }
            else if (((char)local_114 == '\0') ||
                    (iVar12 = memcmp(_Buf1,&DAT_142abe761,_Size), iVar12 != 0)) {
              if (((char)local_154 == '\0') ||
                 (iVar12 = memcmp(_Buf1,&DAT_142abe770,_Size), iVar12 != 0)) {
                if ((char)local_114 == '\0') {
LAB_1409bbf76:
                  if ((char)local_154 != '\0') {
                    iVar12 = memcmp(_Buf1,&DAT_142abe798,_Size);
                    if (iVar12 == 0) {
                      local_f8 = 0x3aa1;
                      local_100 = core::ops::function::FnOnce::
                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_23,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                      ;
                      goto LAB_1409bc700;
                    }
                    iVar12 = memcmp(_Buf1,&DAT_142abe7a2,_Size);
                    if (iVar12 == 0) {
                      local_f8 = 0x3aa2;
                      local_100 = core::ops::function::FnOnce::
                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_24,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                      ;
                      goto LAB_1409bc1e2;
                    }
                  }
                  cVar30 = (char)uVar15;
                  if ((cVar30 == '\0') || (iVar12 = memcmp(_Buf1,&DAT_142abe7ac,_Size), iVar12 != 0)
                     ) {
                    if (_Size == 0x1d) {
                      auVar67[0] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c');
                      auVar67[1] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o');
                      auVar67[2] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
                      auVar67[3] = -((char)_Buf1[2] == 'o');
                      auVar67[4] = -(*(char *)((longlong)_Buf1 + 0x11) == 'r');
                      auVar67[5] = -(*(char *)((longlong)_Buf1 + 0x12) == '_');
                      auVar67[6] = -(*(char *)((longlong)_Buf1 + 0x13) == 'a');
                      auVar67[7] = -(*(char *)((longlong)_Buf1 + 0x14) == 'l');
                      auVar67[8] = -(*(char *)((longlong)_Buf1 + 0x15) == 'b');
                      auVar67[9] = -(*(char *)((longlong)_Buf1 + 0x16) == 'e');
                      auVar67[10] = -(*(char *)((longlong)_Buf1 + 0x17) == 'd');
                      auVar67[0xb] = -((char)_Buf1[3] == 'o');
                      auVar67[0xc] = -(*(char *)((longlong)_Buf1 + 0x19) == '_');
                      auVar67[0xd] = -(*(char *)((longlong)_Buf1 + 0x1a) == 'm');
                      auVar67[0xe] = -(*(char *)((longlong)_Buf1 + 0x1b) == 'a');
                      auVar67[0xf] = -(*(char *)((longlong)_Buf1 + 0x1c) == 'p');
                      auVar47[0] = -((char)*_Buf1 == 's');
                      auVar47[1] = -(*(char *)((longlong)_Buf1 + 1) == 'o');
                      auVar47[2] = -(*(char *)((longlong)_Buf1 + 2) == 'l');
                      auVar47[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
                      auVar47[4] = -(*(char *)((longlong)_Buf1 + 4) == 'd');
                      auVar47[5] = -(*(char *)((longlong)_Buf1 + 5) == '_');
                      auVar47[6] = -(*(char *)((longlong)_Buf1 + 6) == 'v');
                      auVar47[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e');
                      auVar47[8] = -((char)_Buf1[1] == 'r');
                      auVar47[9] = -(*(char *)((longlong)_Buf1 + 9) == 't');
                      auVar47[10] = -(*(char *)((longlong)_Buf1 + 10) == 'e');
                      auVar47[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'x');
                      auVar47[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == '_');
                      auVar47[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c');
                      auVar47[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o');
                      auVar47[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l');
                      auVar47 = auVar47 & auVar67;
                      if ((ushort)((ushort)(SUB161(auVar47 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar47[0xf] >> 7) << 0xf) != 0xffff)
                      goto LAB_1409bc2c4;
                      local_f8 = 0x3aa5;
                      local_100 = core::ops::function::FnOnce::
                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_27,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                      ;
                    }
                    else if ((_Size == 0x1f) &&
                            (auVar66[0] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l'),
                            auVar66[1] = -((char)_Buf1[2] == 'o'),
                            auVar66[2] = -(*(char *)((longlong)_Buf1 + 0x11) == 'r'),
                            auVar66[3] = -(*(char *)((longlong)_Buf1 + 0x12) == '_'),
                            auVar66[4] = -(*(char *)((longlong)_Buf1 + 0x13) == 'g'),
                            auVar66[5] = -(*(char *)((longlong)_Buf1 + 0x14) == 'a'),
                            auVar66[6] = -(*(char *)((longlong)_Buf1 + 0x15) == 'r'),
                            auVar66[7] = -(*(char *)((longlong)_Buf1 + 0x16) == 'd'),
                            auVar66[8] = -(*(char *)((longlong)_Buf1 + 0x17) == 'e'),
                            auVar66[9] = -((char)_Buf1[3] == 'n'),
                            auVar66[10] = -(*(char *)((longlong)_Buf1 + 0x19) == '_'),
                            auVar66[0xb] = -(*(char *)((longlong)_Buf1 + 0x1a) == 's'),
                            auVar66[0xc] = -(*(char *)((longlong)_Buf1 + 0x1b) == 't'),
                            auVar66[0xd] = -(*(char *)((longlong)_Buf1 + 0x1c) == 'o'),
                            auVar66[0xe] = -(*(char *)((longlong)_Buf1 + 0x1d) == 'n'),
                            auVar66[0xf] = -(*(char *)((longlong)_Buf1 + 0x1e) == 'e'),
                            auVar46[0] = -((char)*_Buf1 == 's'),
                            auVar46[1] = -(*(char *)((longlong)_Buf1 + 1) == 'o'),
                            auVar46[2] = -(*(char *)((longlong)_Buf1 + 2) == 'l'),
                            auVar46[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i'),
                            auVar46[4] = -(*(char *)((longlong)_Buf1 + 4) == 'd'),
                            auVar46[5] = -(*(char *)((longlong)_Buf1 + 5) == '_'),
                            auVar46[6] = -(*(char *)((longlong)_Buf1 + 6) == 'v'),
                            auVar46[7] = -(*(char *)((longlong)_Buf1 + 7) == 'e'),
                            auVar46[8] = -((char)_Buf1[1] == 'r'),
                            auVar46[9] = -(*(char *)((longlong)_Buf1 + 9) == 't'),
                            auVar46[10] = -(*(char *)((longlong)_Buf1 + 10) == 'e'),
                            auVar46[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'x'),
                            auVar46[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == '_'),
                            auVar46[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'c'),
                            auVar46[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'o'),
                            auVar46[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'l'),
                            auVar46 = auVar46 & auVar66,
                            (ushort)((ushort)(SUB161(auVar46 >> 7,0) & 1) |
                                     (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                                     (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                                     (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                                     (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                                     (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                                     (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                                     (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                                     (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                                     (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                                     (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                                     (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                                     (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                                     (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                                     (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                                    (ushort)(byte)(auVar46[0xf] >> 7) << 0xf) == 0xffff)) {
                      local_f8 = 0x3aa4;
                      local_100 = core::ops::function::FnOnce::
                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_26,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                      ;
                    }
                    else {
LAB_1409bc2c4:
                      if (((char)local_184 == '\0') ||
                         (iVar12 = memcmp(_Buf1,&DAT_142abe7c3,_Size), iVar12 != 0)) {
                        if (_Size == 0x24) {
                          iVar12 = memcmp(_Buf1,&DAT_142abe7cc,0x24);
                          if (iVar12 != 0) goto LAB_1409bc3af;
                          local_f8 = 0x3aa7;
                          local_100 = core::ops::function::FnOnce::
                                      call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_29,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                          ;
LAB_1409bcde7:
                          local_108 = (undefined1 *)0x3;
                          uVar11 = 3;
                          goto LAB_1409bc717;
                        }
                        if ((char)local_118 != '\0') {
                          auVar48[0] = -((char)*_Buf1 == 'n');
                          auVar48[1] = -(*(char *)((longlong)_Buf1 + 1) == 'a');
                          auVar48[2] = -(*(char *)((longlong)_Buf1 + 2) == 'n');
                          auVar48[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
                          auVar48[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
                          auVar48[5] = -(*(char *)((longlong)_Buf1 + 5) == 's');
                          auVar48[6] = -(*(char *)((longlong)_Buf1 + 6) == 'o');
                          auVar48[7] = -(*(char *)((longlong)_Buf1 + 7) == 'l');
                          auVar48[8] = -((char)_Buf1[1] == 'i');
                          auVar48[9] = -(*(char *)((longlong)_Buf1 + 9) == 'd');
                          auVar48[10] = -(*(char *)((longlong)_Buf1 + 10) == '_');
                          auVar48[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'a');
                          auVar48[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'l');
                          auVar48[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'b');
                          auVar48[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 'e');
                          auVar48[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'd');
                          auVar68[0] = -(*(char *)((longlong)_Buf1 + 0xe) == 'e');
                          auVar68[1] = -(*(char *)((longlong)_Buf1 + 0xf) == 'd');
                          auVar68[2] = -((char)_Buf1[2] == 'o');
                          auVar68[3] = -(*(char *)((longlong)_Buf1 + 0x11) == '_');
                          auVar68[4] = -(*(char *)((longlong)_Buf1 + 0x12) == 'm');
                          auVar68[5] = -(*(char *)((longlong)_Buf1 + 0x13) == 'a');
                          auVar68[6] = -(*(char *)((longlong)_Buf1 + 0x14) == 'p');
                          auVar68[7] = -(*(char *)((longlong)_Buf1 + 0x15) == '_');
                          auVar68[8] = -(*(char *)((longlong)_Buf1 + 0x16) == 'e');
                          auVar68[9] = -(*(char *)((longlong)_Buf1 + 0x17) == 'm');
                          auVar68[10] = -((char)_Buf1[3] == 'i');
                          auVar68[0xb] = -(*(char *)((longlong)_Buf1 + 0x19) == 's');
                          auVar68[0xc] = -(*(char *)((longlong)_Buf1 + 0x1a) == 's');
                          auVar68[0xd] = -(*(char *)((longlong)_Buf1 + 0x1b) == 'i');
                          auVar68[0xe] = -(*(char *)((longlong)_Buf1 + 0x1c) == 'v');
                          auVar68[0xf] = -(*(char *)((longlong)_Buf1 + 0x1d) == 'e');
                          auVar68 = auVar68 & auVar48;
                          if ((ushort)((ushort)(SUB161(auVar68 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar68 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar68 >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auVar68 >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auVar68 >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auVar68 >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auVar68 >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auVar68 >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auVar68 >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auVar68 >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auVar68 >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auVar68 >> 0x5f,0) & 1) << 0xb |
                                       (ushort)(SUB161(auVar68 >> 0x67,0) & 1) << 0xc |
                                       (ushort)(SUB161(auVar68 >> 0x6f,0) & 1) << 0xd |
                                       (ushort)(SUB161(auVar68 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar68[0xf] >> 7) << 0xf) == 0xffff) {
                            local_f8 = 0x3aa8;
                            local_100 = core::ops::function::FnOnce::
                                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_30,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                            ;
                            goto LAB_1409bcde7;
                          }
                          auVar69[0] = -(*(char *)((longlong)_Buf1 + 0xe) == 't');
                          auVar69[1] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e');
                          auVar69[2] = -((char)_Buf1[2] == 'x');
                          auVar69[3] = -(*(char *)((longlong)_Buf1 + 0x11) == '_');
                          auVar69[4] = -(*(char *)((longlong)_Buf1 + 0x12) == 'c');
                          auVar69[5] = -(*(char *)((longlong)_Buf1 + 0x13) == 'o');
                          auVar69[6] = -(*(char *)((longlong)_Buf1 + 0x14) == 'l');
                          auVar69[7] = -(*(char *)((longlong)_Buf1 + 0x15) == 'o');
                          auVar69[8] = -(*(char *)((longlong)_Buf1 + 0x16) == 'r');
                          auVar69[9] = -(*(char *)((longlong)_Buf1 + 0x17) == '_');
                          auVar69[10] = -((char)_Buf1[3] == 'w');
                          auVar69[0xb] = -(*(char *)((longlong)_Buf1 + 0x19) == 'i');
                          auVar69[0xc] = -(*(char *)((longlong)_Buf1 + 0x1a) == 'n');
                          auVar69[0xd] = -(*(char *)((longlong)_Buf1 + 0x1b) == 'd');
                          auVar69[0xe] = -(*(char *)((longlong)_Buf1 + 0x1c) == 'o');
                          auVar69[0xf] = -(*(char *)((longlong)_Buf1 + 0x1d) == 'w');
                          auVar49[0] = -((char)*_Buf1 == 'n');
                          auVar49[1] = -(*(char *)((longlong)_Buf1 + 1) == 'a');
                          auVar49[2] = -(*(char *)((longlong)_Buf1 + 2) == 'n');
                          auVar49[3] = -(*(char *)((longlong)_Buf1 + 3) == 'i');
                          auVar49[4] = -(*(char *)((longlong)_Buf1 + 4) == '_');
                          auVar49[5] = -(*(char *)((longlong)_Buf1 + 5) == 's');
                          auVar49[6] = -(*(char *)((longlong)_Buf1 + 6) == 'o');
                          auVar49[7] = -(*(char *)((longlong)_Buf1 + 7) == 'l');
                          auVar49[8] = -((char)_Buf1[1] == 'i');
                          auVar49[9] = -(*(char *)((longlong)_Buf1 + 9) == 'd');
                          auVar49[10] = -(*(char *)((longlong)_Buf1 + 10) == '_');
                          auVar49[0xb] = -(*(char *)((longlong)_Buf1 + 0xb) == 'v');
                          auVar49[0xc] = -(*(char *)((longlong)_Buf1 + 0xc) == 'e');
                          auVar49[0xd] = -(*(char *)((longlong)_Buf1 + 0xd) == 'r');
                          auVar49[0xe] = -(*(char *)((longlong)_Buf1 + 0xe) == 't');
                          auVar49[0xf] = -(*(char *)((longlong)_Buf1 + 0xf) == 'e');
                          auVar49 = auVar49 & auVar69;
                          if ((ushort)((ushort)(SUB161(auVar49 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar49 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar49 >> 0x17,0) & 1) << 2 |
                                       (ushort)(SUB161(auVar49 >> 0x1f,0) & 1) << 3 |
                                       (ushort)(SUB161(auVar49 >> 0x27,0) & 1) << 4 |
                                       (ushort)(SUB161(auVar49 >> 0x2f,0) & 1) << 5 |
                                       (ushort)(SUB161(auVar49 >> 0x37,0) & 1) << 6 |
                                       (ushort)(SUB161(auVar49 >> 0x3f,0) & 1) << 7 |
                                       (ushort)(SUB161(auVar49 >> 0x47,0) & 1) << 8 |
                                       (ushort)(SUB161(auVar49 >> 0x4f,0) & 1) << 9 |
                                       (ushort)(SUB161(auVar49 >> 0x57,0) & 1) << 10 |
                                       (ushort)(SUB161(auVar49 >> 0x5f,0) & 1) << 0xb |
                                       (ushort)(SUB161(auVar49 >> 0x67,0) & 1) << 0xc |
                                       (ushort)(SUB161(auVar49 >> 0x6f,0) & 1) << 0xd |
                                       (ushort)(SUB161(auVar49 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar49[0xf] >> 7) << 0xf) == 0xffff) {
                            local_f8 = 0x3aa9;
                            local_100 = core::ops::function::FnOnce::
                                        call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_31,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                            ;
                            goto LAB_1409bc700;
                          }
                        }
LAB_1409bc3af:
                        if ((cVar21 == '\0') ||
                           (iVar12 = memcmp(_Buf1,&DAT_142abe7f0,_Size), iVar12 != 0)) {
                          if (((char)local_154 == '\0') ||
                             (iVar12 = memcmp(_Buf1,&DAT_142abe801,_Size), iVar12 != 0)) {
                            if (((char)local_11c == '\0') ||
                               (iVar12 = memcmp(_Buf1,&DAT_142abe80b,_Size), iVar12 != 0)) {
                              if (((char)local_120 == '\0') ||
                                 (iVar12 = memcmp(_Buf1,&DAT_142abe817,_Size), iVar12 != 0)) {
                                if (((char)local_114 != '\0') &&
                                   (iVar12 = memcmp(_Buf1,&DAT_142abe82a,_Size), iVar12 == 0)) {
                                  local_f8 = 0x3aac;
                                  local_100 = core::ops::function::FnOnce::
                                              call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_36,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                                  ;
                                  local_108 = (undefined1 *)0x400000002;
                                  uVar11 = 2;
                                  goto LAB_1409bc717;
                                }
                                if ((_Size == 0xe) &&
                                   (*(longlong *)((longlong)_Buf1 + 6) == 0x6e6f6974616d696e &&
                                    *_Buf1 == 0x696e615f6f676f6c)) {
                                  local_f8 = 0x3aad;
                                  local_100 = core::ops::function::FnOnce::
                                              call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_37,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                                  ;
                                }
                                else if ((cVar30 == '\0') ||
                                        (iVar12 = memcmp(_Buf1,&DAT_142abe839,_Size), iVar12 != 0))
                                {
                                  if (((char)local_10c != '\0') &&
                                     (iVar12 = memcmp(_Buf1,&DAT_142abe850,_Size), iVar12 == 0)) {
                                    local_f8 = 0x3aaf;
                                    local_100 = core::ops::function::FnOnce::
                                                call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_39,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                                    ;
                                    goto LAB_1409bcde7;
                                  }
                                  if ((char)local_158 == '\0') {
                                    if ((_Size == 0xe) &&
                                       (*(longlong *)((longlong)_Buf1 + 6) == 0x6e65657263735f67 &&
                                        *_Buf1 == 0x5f676e6964616f6c)) {
                                      local_f8 = 0x3c8c;
                                      local_100 = core::ops::function::FnOnce::
                                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_41,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                                      ;
                                      goto LAB_1409bb972;
                                    }
                                  }
                                  else {
                                    iVar12 = memcmp(_Buf1,&DAT_142abe865,_Size);
                                    if (iVar12 == 0) {
                                      local_f8 = 0x3ab0;
                                      local_100 = core::ops::function::FnOnce::
                                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_40,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                                      ;
                                      goto LAB_1409bc700;
                                    }
                                  }
                                  if (((char)local_110 == '\0') ||
                                     (iVar12 = memcmp(_Buf1,&DAT_142abe872,_Size), iVar12 != 0)) {
                                    cVar25 = (char)local_10c;
                                    if (_Size == 0xb) {
                                      if (*(longlong *)((longlong)_Buf1 + 3) == 0x656c626265705f68
                                          && *_Buf1 == 0x6265705f68746170) {
                                        local_f8 = 19000;
                                        local_100 = draw_shaders::path_pebble_shader::
                                                    path_pebble_draw;
                                      }
                                      else {
                                        if (*(longlong *)((longlong)_Buf1 + 3) != 0x706574735f737269
                                            || *_Buf1 != 0x735f737269617473) goto LAB_1409bce34;
                                        local_f8 = 19000;
                                        local_100 = draw_shaders::stairs_draw::stairs_draw;
                                      }
                                    }
                                    else {
                                      if ((char)local_158 == '\0') {
                                        if ((_Size == 0xe) &&
                                           (*(longlong *)((longlong)_Buf1 + 6) == 0x7265646168735f64
                                            && *_Buf1 == 0x5f646170796c696c)) {
                                          local_f8 = 19000;
                                          local_100 = draw_shaders::water_planes_draw::lilypad_draw;
                                          goto LAB_1409bc146;
                                        }
LAB_1409bce34:
                                        if ((cVar25 == '\0') ||
                                           (iVar12 = memcmp(_Buf1,&DAT_142abe89c,_Size), iVar12 != 0
                                           )) {
                                          if ((_Size == 0xb) &&
                                             (*(longlong *)((longlong)_Buf1 + 3) ==
                                              0x7265646168735f64 && *_Buf1 == 0x6168735f64656572)) {
                                            local_f8 = 19000;
                                            local_100 = draw_shaders::water_planes_draw::reed_draw;
                                          }
                                          else {
                                            if (((char)local_154 == '\0') ||
                                               (iVar12 = memcmp(_Buf1,&DAT_142abe8b1,_Size),
                                               iVar12 != 0)) {
                                              if (((char)local_11c == '\0') ||
                                                 (iVar12 = memcmp(_Buf1,&DAT_142abe8bb,_Size),
                                                 iVar12 != 0)) {
                                                if ((cVar21 == '\0') ||
                                                   (iVar12 = memcmp(_Buf1,&DAT_142abe8c7,_Size),
                                                   iVar12 != 0)) {
                                                  uVar15 = local_110;
                                                  if (((char)local_174 == '\0') ||
                                                     (iVar12 = memcmp(_Buf1,&DAT_142abe8d8,_Size),
                                                     iVar12 != 0)) {
                                                    if (((char)local_11c == '\0') ||
                                                       (iVar12 = memcmp(_Buf1,&DAT_142abe8ee,_Size),
                                                       iVar12 != 0)) {
                                                      if (((char)uVar15 != '\0') &&
                                                         (iVar12 = memcmp(_Buf1,&DAT_142abe8fa,_Size
                                                                         ), iVar12 == 0)) {
                                                        local_f8 = 32000;
                                                        goto LAB_1409bc276;
                                                      }
                                                      if ((cVar30 == '\0') ||
                                                         (iVar12 = memcmp(_Buf1,&DAT_142abe90a,_Size
                                                                         ), iVar12 != 0)) {
                                                        if ((_Size == 0x12) &&
                                                           (auVar50[0] = -((char)*_Buf1 == 'n'),
                                                           auVar50[1] = -(*(char *)((longlong)_Buf1
                                                                                   + 1) == 'a'),
                                                           auVar50[2] = -(*(char *)((longlong)_Buf1
                                                                                   + 2) == 'n'),
                                                           auVar50[3] = -(*(char *)((longlong)_Buf1
                                                                                   + 3) == 'i'),
                                                           auVar50[4] = -(*(char *)((longlong)_Buf1
                                                                                   + 4) == '_'),
                                                           auVar50[5] = -(*(char *)((longlong)_Buf1
                                                                                   + 5) == 'v'),
                                                           auVar50[6] = -(*(char *)((longlong)_Buf1
                                                                                   + 6) == 'e'),
                                                           auVar50[7] = -(*(char *)((longlong)_Buf1
                                                                                   + 7) == 'l'),
                                                           auVar50[8] = -((char)_Buf1[1] == 'o'),
                                                           auVar50[9] = -(*(char *)((longlong)_Buf1
                                                                                   + 9) == 'c'),
                                                           auVar50[10] = -(*(char *)((longlong)_Buf1
                                                                                    + 10) == 'i'),
                                                           auVar50[0xb] = -(*(char *)((longlong)
                                                                                      _Buf1 + 0xb)
                                                                           == 't'),
                                                           auVar50[0xc] = -(*(char *)((longlong)
                                                                                      _Buf1 + 0xc)
                                                                           == 'y'),
                                                           auVar50[0xd] = -(*(char *)((longlong)
                                                                                      _Buf1 + 0xd)
                                                                           == '_'),
                                                           auVar50[0xe] = -(*(char *)((longlong)
                                                                                      _Buf1 + 0xe)
                                                                           == 'o'),
                                                           auVar50[0xf] = -(*(char *)((longlong)
                                                                                      _Buf1 + 0xf)
                                                                           == 'n'),
                                                           auVar70[0] = -((char)(short)_Buf1[2] ==
                                                                         'l'),
                                                           auVar70[1] = -((char)((ushort)(short)
                                                  _Buf1[2] >> 8) == 'y'), auVar70[2] = 0xff,
                                                  auVar70[3] = 0xff, auVar70[4] = 0xff,
                                                  auVar70[5] = 0xff, auVar70[6] = 0xff,
                                                  auVar70[7] = 0xff, auVar70[8] = 0xff,
                                                  auVar70[9] = 0xff, auVar70[10] = 0xff,
                                                  auVar70[0xb] = 0xff, auVar70[0xc] = 0xff,
                                                  auVar70[0xd] = 0xff, auVar70[0xe] = 0xff,
                                                  auVar70[0xf] = 0xff, auVar70 = auVar70 & auVar50,
                                                  (ushort)((ushort)(SUB161(auVar70 >> 7,0) & 1) |
                                                           (ushort)(SUB161(auVar70 >> 0xf,0) & 1) <<
                                                           1 | (ushort)(SUB161(auVar70 >> 0x17,0) &
                                                                       1) << 2 |
                                                           (ushort)(SUB161(auVar70 >> 0x1f,0) & 1)
                                                           << 3 | (ushort)(SUB161(auVar70 >> 0x27,0)
                                                                          & 1) << 4 |
                                                           (ushort)(SUB161(auVar70 >> 0x2f,0) & 1)
                                                           << 5 | (ushort)(SUB161(auVar70 >> 0x37,0)
                                                                          & 1) << 6 |
                                                           (ushort)(SUB161(auVar70 >> 0x3f,0) & 1)
                                                           << 7 | (ushort)(SUB161(auVar70 >> 0x47,0)
                                                                          & 1) << 8 |
                                                           (ushort)(SUB161(auVar70 >> 0x4f,0) & 1)
                                                           << 9 | (ushort)(SUB161(auVar70 >> 0x57,0)
                                                                          & 1) << 10 |
                                                           (ushort)(SUB161(auVar70 >> 0x5f,0) & 1)
                                                           << 0xb | (ushort)(SUB161(auVar70 >> 0x67,
                                                                                    0) & 1) << 0xc |
                                                           (ushort)(SUB161(auVar70 >> 0x6f,0) & 1)
                                                           << 0xd | (ushort)(SUB161(auVar70 >> 0x77,
                                                                                    0) & 1) << 0xe |
                                                          (ushort)(byte)(auVar70[0xf] >> 7) << 0xf)
                                                  == 0xffff)) {
                                                    local_f8 = 55000;
                                                    local_100 = core::ops::function::FnOnce::
                                                                                                                                
                                                  call_once<country_core::systems::render_loop::draw_passes::draw_lists::build_draw_lists::closure_env_49,tuple_<country_core::render::pass_builder::draw_builder::DrawBuilder,ref_<country_core::render::pass_builder::draw_list::EntityDrawParams>,bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell>_>
                                                  ;
                                                  local_108 = (undefined1 *)
                                                              CONCAT44(local_108._4_4_,4);
                                                  uVar11 = 4;
                                                  goto LAB_1409bc717;
                                                  }
                                                  if ((((char)local_168 == '\0') ||
                                                      (iVar12 = memcmp(_Buf1,&DAT_142abe921,_Size),
                                                      iVar12 != 0)) &&
                                                     (((char)local_10c == '\0' ||
                                                      (iVar12 = memcmp(_Buf1,&DAT_142abe939,_Size),
                                                      iVar12 != 0)))) {
                                                    if (_Size == 0x14) {
                                                      auVar51[0] = -((char)*_Buf1 == 'd');
                                                      auVar51[1] = -(*(char *)((longlong)_Buf1 + 1)
                                                                    == 'e');
                                                      auVar51[2] = -(*(char *)((longlong)_Buf1 + 2)
                                                                    == 'm');
                                                      auVar51[3] = -(*(char *)((longlong)_Buf1 + 3)
                                                                    == 'o');
                                                      auVar51[4] = -(*(char *)((longlong)_Buf1 + 4)
                                                                    == '_');
                                                      auVar51[5] = -(*(char *)((longlong)_Buf1 + 5)
                                                                    == 't');
                                                      auVar51[6] = -(*(char *)((longlong)_Buf1 + 6)
                                                                    == 'e');
                                                      auVar51[7] = -(*(char *)((longlong)_Buf1 + 7)
                                                                    == 'r');
                                                      auVar51[8] = -((char)_Buf1[1] == 'r');
                                                      auVar51[9] = -(*(char *)((longlong)_Buf1 + 9)
                                                                    == 'a');
                                                      auVar51[10] = -(*(char *)((longlong)_Buf1 + 10
                                                                               ) == 'i');
                                                      auVar51[0xb] = -(*(char *)((longlong)_Buf1 +
                                                                                0xb) == 'n');
                                                      auVar51[0xc] = -(*(char *)((longlong)_Buf1 +
                                                                                0xc) == '_');
                                                      auVar51[0xd] = -(*(char *)((longlong)_Buf1 +
                                                                                0xd) == 'o');
                                                      auVar51[0xe] = -(*(char *)((longlong)_Buf1 +
                                                                                0xe) == 'v');
                                                      auVar51[0xf] = -(*(char *)((longlong)_Buf1 +
                                                                                0xf) == 'e');
                                                      iVar12 = (int)_Buf1[2];
                                                      auVar71[0] = -((char)iVar12 == 'r');
                                                      auVar71[1] = -((char)((uint)iVar12 >> 8) ==
                                                                    'l');
                                                      auVar71[2] = -((char)((uint)iVar12 >> 0x10) ==
                                                                    'a');
                                                      auVar71[3] = -((char)((uint)iVar12 >> 0x18) ==
                                                                    'y');
                                                      auVar71[4] = 0xff;
                                                      auVar71[5] = 0xff;
                                                      auVar71[6] = 0xff;
                                                      auVar71[7] = 0xff;
                                                      auVar71[8] = 0xff;
                                                      auVar71[9] = 0xff;
                                                      auVar71[10] = 0xff;
                                                      auVar71[0xb] = 0xff;
                                                      auVar71[0xc] = 0xff;
                                                      auVar71[0xd] = 0xff;
                                                      auVar71[0xe] = 0xff;
                                                      auVar71[0xf] = 0xff;
                                                      auVar71 = auVar71 & auVar51;
                                                      if ((ushort)((ushort)(SUB161(auVar71 >> 7,0) &
                                                                           1) |
                                                                   (ushort)(SUB161(auVar71 >> 0xf,0)
                                                                           & 1) << 1 |
                                                                   (ushort)(SUB161(auVar71 >> 0x17,0
                                                                                  ) & 1) << 2 |
                                                                   (ushort)(SUB161(auVar71 >> 0x1f,0
                                                                                  ) & 1) << 3 |
                                                                   (ushort)(SUB161(auVar71 >> 0x27,0
                                                                                  ) & 1) << 4 |
                                                                   (ushort)(SUB161(auVar71 >> 0x2f,0
                                                                                  ) & 1) << 5 |
                            