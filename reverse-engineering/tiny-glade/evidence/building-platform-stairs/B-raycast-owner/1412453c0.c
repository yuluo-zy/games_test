
void __cdecl
country_core::systems::collision::world::RaycastWorld::
upsert_collider<country_core::resources::walls::WallId,tuple_<glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>_>
          (void)

{
  char *pcVar1;
  undefined8 uVar2;
  undefined1 (*pauVar3) [16];
  uint uVar4;
  float fVar5;
  float fVar6;
  float fVar7;
  float fVar8;
  float fVar9;
  char extraout_AL;
  char extraout_AL_00;
  undefined4 extraout_EAX;
  undefined4 extraout_EAX_00;
  int iVar10;
  undefined4 extraout_EAX_01;
  ulonglong extraout_RAX;
  undefined8 *extraout_RAX_00;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *extraout_RAX_03;
  undefined *extraout_RAX_04;
  undefined *puVar11;
  undefined8 *extraout_RAX_05;
  longlong extraout_RAX_06;
  undefined8 *extraout_RAX_07;
  undefined8 *extraout_RAX_08;
  longlong extraout_RAX_09;
  longlong extraout_RAX_10;
  undefined8 *in_RCX;
  uint uVar12;
  uint uVar13;
  ulonglong uVar14;
  undefined8 *puVar15;
  undefined8 *puVar16;
  byte bVar17;
  ushort uVar18;
  longlong *in_R8;
  ulonglong uVar19;
  longlong *in_R9;
  longlong lVar20;
  ulonglong uVar21;
  longlong lVar22;
  longlong *plVar23;
  byte bVar24;
  char cVar25;
  char cVar28;
  char cVar29;
  char cVar30;
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  float fVar33;
  float fVar34;
  float fVar35;
  float fVar36;
  undefined8 *in_stack_00000028;
  undefined8 *in_stack_00000030;
  float local_3c8;
  float fStack_3c4;
  float fStack_3c0;
  float fStack_3bc;
  undefined8 local_3b8;
  float local_3b0;
  int local_3a8;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  longlong *local_2f8;
  undefined8 uStack_2f0;
  longlong *local_2e8;
  longlong *plStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined **local_2c8;
  undefined8 uStack_2c0;
  longlong **local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  longlong local_278;
  ulonglong local_270;
  longlong *local_268;
  code *pcStack_260;
  longlong *local_258;
  code *pcStack_250;
  ulonglong local_248;
  undefined8 local_240;
  undefined **local_238;
  undefined8 uStack_230;
  longlong **local_228;
  undefined8 uStack_220;
  undefined8 local_218;
  undefined8 uStack_210;
  undefined8 local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  undefined8 local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  longlong *plStack_1d0;
  undefined8 local_1bc;
  undefined4 local_1b4;
  longlong *local_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  longlong *local_198;
  longlong *plStack_190;
  longlong *local_188;
  undefined8 uStack_180;
  undefined **local_178;
  longlong *plStack_170;
  longlong **local_168;
  undefined8 uStack_160;
  undefined **local_158;
  undefined8 uStack_150;
  longlong **local_148;
  undefined1 auStack_140 [16];
  undefined8 uStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  undefined8 local_f8;
  float local_f0;
  undefined1 local_e6;
  undefined1 local_e5;
  undefined1 local_e4;
  undefined1 local_e3;
  undefined1 local_e2;
  undefined1 local_e1;
  undefined8 local_e0;
  
  local_e0 = 0xfffffffffffffffe;
  local_e5 = 1;
  local_1b0 = in_R9;
  local_1a8 = in_R9;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  lVar20 = *in_R8;
  bVar24 = (byte)(extraout_RAX >> 0x39);
  auVar26 = ZEXT216(CONCAT11(bVar24,bVar24));
  auVar26 = pshuflw(auVar26,auVar26,0);
  lVar22 = 0;
  uVar14 = extraout_RAX;
  while( true ) {
    uVar14 = in_R8[1] & uVar14;
    pcVar1 = (char *)(lVar20 + uVar14);
    cVar25 = auVar26[0];
    auVar32[0] = -(*pcVar1 == cVar25);
    cVar28 = auVar26[1];
    auVar32[1] = -(pcVar1[1] == cVar28);
    cVar29 = auVar26[2];
    auVar32[2] = -(pcVar1[2] == cVar29);
    cVar30 = auVar26[3];
    auVar32[3] = -(pcVar1[3] == cVar30);
    auVar32[4] = -(pcVar1[4] == cVar25);
    auVar32[5] = -(pcVar1[5] == cVar28);
    auVar32[6] = -(pcVar1[6] == cVar29);
    auVar32[7] = -(pcVar1[7] == cVar30);
    auVar32[8] = -(pcVar1[8] == cVar25);
    auVar32[9] = -(pcVar1[9] == cVar28);
    auVar32[10] = -(pcVar1[10] == cVar29);
    auVar32[0xb] = -(pcVar1[0xb] == cVar30);
    auVar32[0xc] = -(pcVar1[0xc] == cVar25);
    auVar32[0xd] = -(pcVar1[0xd] == cVar28);
    auVar32[0xe] = -(pcVar1[0xe] == cVar29);
    auVar32[0xf] = -(pcVar1[0xf] == cVar30);
    uVar18 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar32 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar32 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar32 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar32 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar32 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar32 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar32 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar32 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar32 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar32 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar32 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar32 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe | (ushort)(auVar32[0xf] >> 7) << 0xf;
    uVar12 = (uint)uVar18;
    while (uVar18 != 0) {
      uVar13 = 0;
      for (uVar4 = uVar12; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar13 = uVar13 + 1;
      }
      uVar21 = uVar13 + uVar14 & in_R8[1];
      if (*(longlong **)(lVar20 + -0x20 + uVar21 * -0x20) == local_1b0) {
        local_270 = lVar20 + uVar21 * -0x20;
        plVar23 = (longlong *)0x0;
        local_1b0 = in_R8;
        goto LAB_141245555;
      }
      uVar18 = (ushort)(uVar12 - 1) & (ushort)uVar12;
      uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar18);
    }
    auVar31[0] = -(*pcVar1 == -1);
    auVar31[1] = -(pcVar1[1] == -1);
    auVar31[2] = -(pcVar1[2] == -1);
    auVar31[3] = -(pcVar1[3] == -1);
    auVar31[4] = -(pcVar1[4] == -1);
    auVar31[5] = -(pcVar1[5] == -1);
    auVar31[6] = -(pcVar1[6] == -1);
    auVar31[7] = -(pcVar1[7] == -1);
    auVar31[8] = -(pcVar1[8] == -1);
    auVar31[9] = -(pcVar1[9] == -1);
    auVar31[10] = -(pcVar1[10] == -1);
    auVar31[0xb] = -(pcVar1[0xb] == -1);
    auVar31[0xc] = -(pcVar1[0xc] == -1);
    auVar31[0xd] = -(pcVar1[0xd] == -1);
    auVar31[0xe] = -(pcVar1[0xe] == -1);
    auVar31[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar31 >> 0x17,0) & 1) != 0) || (SUB161(auVar31 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar31 >> 0x27,0) & 1) != 0) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar31 >> 0x37,0) & 1) != 0) || (SUB161(auVar31 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar31 >> 0x47,0) & 1) != 0) || (SUB161(auVar31 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar31 >> 0x57,0) & 1) != 0) || (SUB161(auVar31 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar31 >> 0x67,0) & 1) != 0) || (SUB161(auVar31 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar31 >> 0x77,0) & 1) != 0) || auVar31[0xf] < '\0') break;
    uVar14 = uVar14 + lVar22 + 0x10;
    lVar22 = lVar22 + 0x10;
  }
  plVar23 = in_R8;
  local_270 = extraout_RAX;
  if (in_R8[2] == 0) {
    local_e5 = 1;
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,alloc::vec::Vec<country_core::resources::walls::wall_stone_columns::StoneColumn,alloc::alloc::Global>,std::hash::random::RandomState>_>
              ();
    local_1b0 = local_1a8;
  }
LAB_141245555:
  local_1e8 = *in_stack_00000028;
  uStack_1e0 = in_stack_00000028[1];
  local_1d8 = in_stack_00000028[2];
  plStack_1d0 = (longlong *)in_stack_00000028[3];
  local_318 = *in_stack_00000030;
  uStack_310 = in_stack_00000030[1];
  local_308 = in_stack_00000030[2];
  uStack_300 = in_stack_00000030[3];
  local_e6 = 1;
  RaycastGeoSource::subworld();
  local_e6 = 1;
  world::impl$21::to_offset_collider_builder();
  uVar14 = local_270;
  fVar5 = (float)local_1a8;
  fVar6 = local_1a8._4_4_;
  fVar7 = (float)uStack_1a0;
  fVar8 = uStack_1a0._4_4_;
  fVar33 = SUB84(local_198,0);
  fVar34 = (float)((ulonglong)local_198 >> 0x20);
  fVar9 = plStack_190._0_4_;
  local_288 = local_118;
  uStack_280 = uStack_110;
  local_298 = local_128;
  uStack_290 = uStack_120;
  local_2a8 = auStack_140._8_8_;
  uStack_2a0 = uStack_130;
  local_2b8 = local_148;
  uStack_2b0 = auStack_140._0_8_;
  local_2c8 = local_158;
  uStack_2c0 = uStack_150;
  local_2d8 = local_168;
  uStack_2d0 = uStack_160;
  local_2e8 = (longlong *)local_178;
  plStack_2e0 = plStack_170;
  local_2f8 = local_188;
  uStack_2f0 = uStack_180;
  if (plVar23 != (longlong *)0x0) {
    if ((*
         __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
         != '\0') && (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
      local_e1 = 1;
      local_e2 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_188 = (longlong *)*extraout_RAX_00;
      uStack_180 = extraout_RAX_00[1];
      local_178 = (undefined **)0x3;
      plStack_170 = (longlong *)s_country_core__systems__collision_142c1fca0;
      local_168 = (longlong **)0x27;
      local_158 = &PTR_s_Inserting_a_collider_142c1fd98;
      uStack_150 = 1;
      local_148 = (longlong **)&DAT_00000008;
      local_1a8 = (longlong *)0x0;
      auStack_140 = (undefined1  [16])0x0;
      uStack_1a0 = s_country_core__systems__collision_142c1fca0;
      local_198 = (longlong *)0x27;
      plStack_190 = (longlong *)0x0;
      uStack_160 = CONCAT44(*(undefined4 *)(extraout_RAX_00 + 2),1);
      local_e1 = 1;
      local_e2 = 1;
      ::log::__private_api::impl$2::log();
    }
    local_198 = (longlong *)local_1d8;
    plStack_190 = plStack_1d0;
    local_1a8 = (longlong *)local_1e8;
    uStack_1a0 = (char *)uStack_1e0;
    local_e1 = 1;
    local_e2 = 0;
    puVar15 = &local_1a8;
    slotmap::basic::SlotMap::
    insert<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh>
              ();
    local_f8 = (longlong *)CONCAT44((int)puVar15,extraout_EAX);
    uStack_130 = uStack_280;
    local_148 = (longlong **)local_298;
    auStack_140._0_8_ = uStack_290;
    auStack_140._8_8_ = local_288;
    local_158 = (undefined **)local_2a8;
    uStack_150 = uStack_2a0;
    local_168 = local_2b8;
    uStack_160 = uStack_2b0;
    local_178 = local_2c8;
    plStack_170 = (longlong *)uStack_2c0;
    local_188 = (longlong *)local_2d8;
    uStack_180 = uStack_2d0;
    local_198 = local_2e8;
    plStack_190 = plStack_2e0;
    local_1a8 = local_2f8;
    uStack_1a0 = (char *)uStack_2f0;
    impl$62::data();
    local_248 = (ulonglong)puVar15 & 0xffffffff | extraout_RAX_01 << 0x20;
    uStack_180 = 0;
    local_268 = local_1a8;
    pcStack_260 = (code *)uStack_1a0;
    local_258 = local_198;
    pcStack_250 = (code *)plStack_190;
    local_238 = local_178;
    uStack_230 = plStack_170;
    local_228 = local_168;
    uStack_220 = uStack_160;
    local_218 = local_158;
    uStack_210 = uStack_150;
    local_208 = local_148;
    uStack_200 = auStack_140._0_8_;
    local_1f8 = auStack_140._8_8_;
    uStack_1f0 = uStack_130;
    local_240 = 0;
    local_188 = (longlong *)local_248;
    rapier3d::geometry::collider::ColliderBuilder::build();
    LOCK();
    *local_258 = *local_258 + -1;
    UNLOCK();
    if (*local_258 == 0) {
      alloc::sync::Arc::
      drop_slow<dyn_<core::convert::AsRef<slice2_<u8>_>,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
                ();
    }
    if (1 < (uint)local_268) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
    local_e4 = 1;
    world::impl$1::index();
    *(undefined1 *)(extraout_RAX_02 + 0x388) = 1;
    local_e4 = 1;
    rapier3d::geometry::collider::Collider::compute_aabb();
    local_e4 = 1;
    xz_flatten_aabb();
    local_e4 = 1;
    country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect();
    memcpy(&local_1a8,&local_3c8,0xb0);
    local_e4 = 0;
    rapier3d::geometry::collider_set::ColliderSet::insert<rapier3d::geometry::collider::Collider>();
    local_e4 = 0;
    SubworldColliderHandle::new();
    pauVar3 = (undefined1 (*) [16])*plVar23;
    uVar21 = plVar23[1];
    uVar19 = uVar21 & uVar14;
    auVar26 = *(undefined1 (*) [16])(*pauVar3 + uVar19);
    uVar18 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar26[0xf] >> 7) << 0xf;
    uVar12 = (uint)uVar18;
    if (uVar18 == 0) {
      lVar20 = 0x10;
      do {
        uVar19 = uVar19 + lVar20 & uVar21;
        auVar26 = *(undefined1 (*) [16])(*pauVar3 + uVar19);
        lVar20 = lVar20 + 0x10;
        uVar12 = (uint)(ushort)((ushort)(SUB161(auVar26 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar26[0xf] >> 7) << 0xf);
      } while (uVar12 == 0);
    }
    uVar13 = 0;
    for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      uVar13 = uVar13 + 1;
    }
    uVar19 = uVar13 + uVar19 & uVar21;
    bVar24 = (*pauVar3)[uVar19];
    if (-1 < (char)bVar24) {
      auVar26 = *pauVar3;
      uVar12 = 0;
      for (uVar13 = (uint)(ushort)((ushort)(SUB161(auVar26 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
                                   (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
                                   (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
                                   (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
                                   (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
                                   (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
                                   (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
                                   (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
                                   (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
                                   (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
                                   (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
                                   (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
                                   (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
                                   (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar26[0xf] >> 7) << 0xf); (uVar13 & 1) == 0;
          uVar13 = uVar13 >> 1 | 0x80000000) {
        uVar12 = uVar12 + 1;
      }
      uVar19 = (ulonglong)uVar12;
      bVar24 = (*pauVar3)[uVar19];
    }
    bVar17 = (byte)(uVar14 >> 0x39);
    (*pauVar3)[uVar19] = bVar17;
    pauVar3[1][uVar19 - 0x10 & uVar21] = bVar17;
    plVar23[2] = plVar23[2] - (ulonglong)(bVar24 & 1);
    *(longlong **)pauVar3[uVar19 * -2 + -2] = local_1b0;
    *(longlong **)(pauVar3[uVar19 * -2 + -2] + 8) = local_f8;
    *(undefined4 *)(pauVar3[uVar19 * -2 + -1] + 8) = local_1b4;
    *(undefined8 *)pauVar3[uVar19 * -2 + -1] = local_1bc;
    plVar23[3] = plVar23[3] + 1;
    uStack_1a0 = (char *)CONCAT44(uStack_1a0._4_4_,local_1b4);
    local_1a8 = (longlong *)local_1bc;
    local_e4 = 0;
    Rapier::on_collider_upserted();
    *(undefined4 *)(in_RCX + 1) = local_1b4;
    *in_RCX = local_1bc;
    *(undefined4 *)((longlong)in_RCX + 0xc) = 0;
    return;
  }
  uVar2 = *(undefined8 *)(local_270 - 0x10);
  uStack_1a0 = (char *)CONCAT44(uStack_1a0._4_4_,*(undefined4 *)(local_270 - 8));
  local_1a8 = *(longlong **)(local_270 - 0x10);
  local_e1 = 1;
  local_e2 = 1;
  get_collider();
  puVar15 = (undefined8 *)(uVar14 - 0x10);
  if (local_3a8 == 0) goto LAB_141246026;
  fVar35 = (float)local_3b8;
  fVar36 = (float)((ulonglong)local_3b8 >> 0x20);
  if ((fVar35 != fVar33) || (NAN(fVar35) || NAN(fVar33))) {
    if (*
        __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
        == '\0') goto LAB_141246026;
LAB_141245b18:
    puVar11 = *(undefined **)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E;
    if ((undefined *)0x2 < *(undefined **)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
      local_f0 = fVar9 - local_3b0;
      local_f8 = (longlong *)CONCAT44(fVar34 - fVar36,fVar33 - fVar35);
      local_268 = &local_f8;
      pcStack_260 = glam::f32::vec3::impl$92::fmt;
      local_e1 = 1;
      local_e2 = 1;
      toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
      local_188 = (longlong *)*extraout_RAX_03;
      uStack_180 = extraout_RAX_03[1];
      local_178 = (undefined **)0x3;
      plStack_170 = (longlong *)s_country_core__systems__collision_142c1fca0;
      local_168 = (longlong **)0x27;
      local_158 = &PTR_s_Pos_diff__142c1fc10;
      uStack_150 = 1;
      local_148 = &local_268;
      auStack_140 = ZEXT816(1);
      local_1a8 = (longlong *)0x0;
      uStack_1a0 = s_country_core__systems__collision_142c1fca0;
      local_198 = (longlong *)0x27;
      plStack_190 = (longlong *)0x0;
      uStack_160 = CONCAT44(*(undefined4 *)(extraout_RAX_03 + 2),1);
      local_e1 = 1;
      local_e2 = 1;
      ::log::__private_api::impl$2::log();
      puVar11 = extraout_RAX_04;
    }
  }
  else {
    if ((fVar36 == fVar34) && (!NAN(fVar36) && !NAN(fVar34))) {
      if ((local_3b0 == fVar9) &&
         ((!NAN(local_3b0) && !NAN(fVar9) &&
          (auVar26._4_4_ = -(uint)(fStack_3c4 == fVar6), auVar26._0_4_ = -(uint)(local_3c8 == fVar5)
          , auVar26._8_4_ = -(uint)(fStack_3c0 == fVar7),
          auVar26._12_4_ = -(uint)(fStack_3bc == fVar8), iVar10 = movmskps(extraout_EAX_00,auVar26),
          iVar10 == 0xf)))) {
        local_e1 = 1;
        local_e2 = 1;
        get_collider_mesh();
        local_e1 = 1;
        local_e2 = 1;
        world::impl$18::eq();
        if (extraout_AL != '\0') {
          *(undefined4 *)(in_RCX + 1) = *(undefined4 *)(uVar14 - 8);
          *in_RCX = *puVar15;
          *(undefined4 *)((longlong)in_RCX + 0xc) = 2;
          in_RCX[2] = *puVar15;
          local_e6 = 1;
          core::ptr::drop_in_place<rapier3d::geometry::collider::ColliderBuilder>();
          LOCK();
          *plStack_1d0 = *plStack_1d0 + -1;
          UNLOCK();
          if (*plStack_1d0 != 0) {
            return;
          }
          local_e5 = 0;
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
          return;
        }
      }
    }
    if (*
        __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
        == '\0') goto LAB_141246026;
    if ((((fVar34 != fVar36) || (NAN(fVar34) || NAN(fVar36))) || (fVar9 != local_3b0)) ||
       (puVar11 = 
        __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
       , NAN(fVar9) || NAN(local_3b0))) goto LAB_141245b18;
  }
  auVar27._4_4_ = -(uint)(fVar6 == fStack_3c4);
  auVar27._0_4_ = -(uint)(fVar5 == local_3c8);
  auVar27._8_4_ = -(uint)(fVar7 == fStack_3c0);
  auVar27._12_4_ = -(uint)(fVar8 == fStack_3bc);
  iVar10 = movmskps((int)puVar11,auVar27);
  if ((iVar10 != 0xf) && (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)
     ) {
    local_f0 = fVar7 - fStack_3c0;
    local_f8 = (longlong *)CONCAT44(fVar6 - fStack_3c4,fVar5 - local_3c8);
    local_268 = &local_f8;
    pcStack_260 = glam::f32::vec3::impl$92::fmt;
    local_e1 = 1;
    local_e2 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_188 = (longlong *)*extraout_RAX_05;
    uStack_180 = extraout_RAX_05[1];
    local_178 = (undefined **)0x3;
    plStack_170 = (longlong *)s_country_core__systems__collision_142c1fca0;
    local_168 = (longlong **)0x27;
    local_158 = &PTR_s_Pos_diff__142c1fc10;
    uStack_150 = 1;
    local_148 = &local_268;
    auStack_140 = ZEXT816(1);
    local_1a8 = (longlong *)0x0;
    uStack_1a0 = s_country_core__systems__collision_142c1fca0;
    local_198 = (longlong *)0x27;
    plStack_190 = (longlong *)0x0;
    uStack_160 = CONCAT44(*(undefined4 *)(extraout_RAX_05 + 2),1);
    local_e1 = 1;
    local_e2 = 1;
    ::log::__private_api::impl$2::log();
  }
  local_e1 = 1;
  local_e2 = 1;
  get_collider_mesh();
  local_e1 = 1;
  local_e2 = 1;
  world::impl$18::eq();
  if ((extraout_AL_00 == '\0') &&
     (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
    local_e1 = 1;
    local_e2 = 1;
    get_collider_mesh();
    local_278 = *(longlong *)(extraout_RAX_06 + 0x18) + 0x10;
    local_f8 = plStack_1d0 + 2;
    local_268 = &local_278;
    pcStack_260 = core::fmt::num::impl$64::fmt;
    local_258 = &local_f8;
    pcStack_250 = core::fmt::num::impl$64::fmt;
    local_e1 = 1;
    local_e2 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_188 = (longlong *)*extraout_RAX_07;
    uStack_180 = extraout_RAX_07[1];
    local_178 = (undefined **)0x3;
    plStack_170 = (longlong *)s_country_core__systems__collision_142c1fca0;
    local_168 = (longlong **)0x27;
    local_158 = &PTR_s_Mesh_diff__vs_142c1fcf0;
    uStack_150 = 2;
    local_148 = &local_268;
    auStack_140 = ZEXT816(2);
    local_1a8 = (longlong *)0x0;
    uStack_1a0 = s_country_core__systems__collision_142c1fca0;
    local_198 = (longlong *)0x27;
    plStack_190 = (longlong *)0x0;
    uStack_160 = CONCAT44(*(undefined4 *)(extraout_RAX_07 + 2),1);
    local_e1 = 1;
    local_e2 = 1;
    ::log::__private_api::impl$2::log();
  }
LAB_141246026:
  if ((*
       __imp__ZN12country_core7systems9collision5world24PHYSICS_DEBUG_LOG_ENABLE17h8c0b6c8a40689d94E
       != '\0') && (2 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E)) {
    local_e1 = 1;
    local_e2 = 1;
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    local_188 = (longlong *)*extraout_RAX_08;
    uStack_180 = extraout_RAX_08[1];
    local_178 = (undefined **)0x3;
    plStack_170 = (longlong *)s_country_core__systems__collision_142c1fca0;
    local_168 = (longlong **)0x27;
    local_158 = &PTR_s_Updating_a_collider_142c1fd40;
    uStack_150 = 1;
    local_148 = (longlong **)&DAT_00000008;
    local_1a8 = (longlong *)0x0;
    auStack_140 = (undefined1  [16])0x0;
    uStack_1a0 = s_country_core__systems__collision_142c1fca0;
    local_198 = (longlong *)0x27;
    plStack_190 = (longlong *)0x0;
    uStack_160 = CONCAT44(*(undefined4 *)(extraout_RAX_08 + 2),1);
    local_e1 = 1;
    local_e2 = 1;
    ::log::__private_api::impl$2::log();
  }
  local_e1 = 1;
  local_e2 = 1;
  dispose_collider_mesh_instance();
  LOCK();
  *plStack_170 = *plStack_170 + -1;
  UNLOCK();
  if (*plStack_170 == 0) {
    local_e1 = 1;
    local_e2 = 1;
    alloc::sync::Arc::
    drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>();
  }
  local_198 = (longlong *)local_1d8;
  plStack_190 = plStack_1d0;
  local_1a8 = (longlong *)local_1e8;
  uStack_1a0 = (char *)uStack_1e0;
  local_e1 = 1;
  local_e2 = 0;
  puVar16 = &local_1a8;
  slotmap::basic::SlotMap::
  insert<country_core::systems::collision::world::RaycastMeshKey,country_core::systems::collision::world::RaycastMesh>
            ();
  *(undefined4 *)(uVar14 - 0x18) = extraout_EAX_01;
  *(int *)(uVar14 - 0x14) = (int)puVar16;
  uStack_130 = uStack_280;
  local_148 = (longlong **)local_298;
  auStack_140._0_8_ = uStack_290;
  auStack_140._8_8_ = local_288;
  local_158 = (undefined **)local_2a8;
  uStack_150 = uStack_2a0;
  local_168 = local_2b8;
  uStack_160 = uStack_2b0;
  local_178 = local_2c8;
  plStack_170 = (longlong *)uStack_2c0;
  local_188 = (longlong *)local_2d8;
  uStack_180 = uStack_2d0;
  local_198 = local_2e8;
  plStack_190 = plStack_2e0;
  local_1a8 = local_2f8;
  uStack_1a0 = (char *)uStack_2f0;
  impl$62::data();
  local_248 = (ulonglong)puVar16 & 0xffffffff | extraout_RAX_09 << 0x20;
  uStack_180 = 0;
  local_268 = local_1a8;
  pcStack_260 = (code *)uStack_1a0;
  local_258 = local_198;
  pcStack_250 = (code *)plStack_190;
  local_238 = local_178;
  uStack_230 = plStack_170;
  local_228 = local_168;
  uStack_220 = uStack_160;
  local_218 = local_158;
  uStack_210 = uStack_150;
  local_208 = local_148;
  uStack_200 = auStack_140._0_8_;
  local_1f8 = auStack_140._8_8_;
  uStack_1f0 = uStack_130;
  local_240 = 0;
  local_188 = (longlong *)local_248;
  rapier3d::geometry::collider::ColliderBuilder::build();
  LOCK();
  *local_258 = *local_258 + -1;
  UNLOCK();
  if (*local_258 == 0) {
    alloc::sync::Arc::
    drop_slow<dyn_<core::convert::AsRef<slice2_<u8>_>,core::marker::Send,core::marker::Sync>,alloc::alloc::Global>
              ();
  }
  if (1 < (uint)local_268) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  local_e3 = 1;
  world::impl$1::index();
  *(undefined1 *)(extraout_RAX_10 + 0x388) = 1;
  local_e3 = 1;
  rapier3d::geometry::collider::Collider::compute_aabb();
  local_e3 = 1;
  xz_flatten_aabb();
  local_e3 = 1;
  country_core::utils::dirty_tiles::DirtyTiles::rasterize_world_space_rect();
  memcpy(&local_1a8,&local_3c8,0xb0);
  local_e3 = 0;
  rapier3d::geometry::collider_set::ColliderSet::insert<rapier3d::geometry::collider::Collider>();
  local_e3 = 0;
  SubworldColliderHandle::new();
  *(undefined4 *)(uVar14 - 8) = pcStack_260._0_4_;
  *puVar15 = local_268;
  uStack_1a0 = (char *)CONCAT44(uStack_1a0._4_4_,pcStack_260._0_4_);
  local_1a8 = local_268;
  local_e3 = 0;
  Rapier::on_collider_upserted();
  *(undefined4 *)(in_RCX + 1) = pcStack_260._0_4_;
  *in_RCX = local_268;
  *(undefined4 *)((longlong)in_RCX + 0xc) = 1;
  in_RCX[2] = uVar2;
  return;
}

