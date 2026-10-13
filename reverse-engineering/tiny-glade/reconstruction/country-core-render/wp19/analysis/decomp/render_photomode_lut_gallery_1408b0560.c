// render_photomode_lut_gallery  0x1408b0560
// signature: void __cdecl render_photomode_lut_gallery(void)
// callees: [{"name": "__chkstk", "address": "14289aa40"}, {"name": "__rust_dealloc", "address": "140613c20"}, {"name": "acquire_resource", "address": "140c81c70"}, {"name": "bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>", "address": "140908610"}, {"name": "bind_texture_to_draw_builder<ref$<rhapsody::raw::texture::Texture>_>", "address": "1408fd6c0"}, {"name": "call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64>_>_>_>_>_>_>_>_>", "address": "1400d9b20"}, {"name": "compute", "address": "140c9cd80"}, {"name": "default", "address": "140be6750"}, {"name": "draw_no_vao", "address": "140906a90"}, {"name": "extent_2d", "address": "140bd5790"}, {"name": "format_inner", "address": "140d79f90"}, {"name": "from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<u32>,country_core::resources::gui::impl$6::positions::closure_env$0>_>", "address": "141304fb0"}, {"name": "get_raster_handle_by_name", "address": "140937aa0"}, {"name": "get_resource_id", "address": "1400dd6f0"}, {"name": "lock_slow", "address": "1428cd210"}, {"name": "memcmp", "address": "142923bf0"}, {"name": "memcpy", "address": "142923c00"}, {"name": "new", "address": "140905be0"}, {"name": "new_impl", "address": "140867970"}, {"name": "optional_uniform<array$<u32,2>_>", "address": "141629d20"}, {"name": "panic_access_error", "address": "1428d80d0"}, {"name": "read_contended", "address": "1428d8a30"}, {"name": "release_resource", "address": "140c81e10"}, {"name": "render", "address": "140868370"}, {"name": "unlock_slow", "address": "1428cdb90"}, {"name": "unwrap_failed", "address": "1428d9310"}, {"name": "wake_writer_or_readers", "address": "1428d8d40"}, {"name": "write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<f32>,f32>", "address": "14096ba30"}, {"name": "write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>", "address": "14096b940"}, {"name": "write_uniform<country_core::render::pass_builder::draw_builder::impl$2::uniform::closure_env$0<bool>,bool>", "address": "14096b850"}]


// WARNING: Function: __chkstk replaced with injection: alloca_probe

void __cdecl country_core::systems::render_loop::lut_gallery::render_photomode_lut_gallery(void)

{
  uint *puVar1;
  char cVar2;
  uint uVar3;
  undefined4 uVar4;
  longlong *plVar5;
  int *piVar6;
  uint uVar7;
  undefined **ppuVar8;
  void *pvVar9;
  int iVar10;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong extraout_RAX_03;
  undefined8 ***pppuVar11;
  longlong *extraout_RAX_04;
  longlong lVar12;
  longlong in_RDX;
  ulonglong uVar13;
  ulonglong uVar14;
  longlong lVar15;
  char *pcVar16;
  longlong in_R8;
  longlong lVar17;
  uint uVar18;
  ushort uVar19;
  ulonglong uVar20;
  bool bVar21;
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined1 auVar25 [16];
  undefined8 ***local_1050;
  void *local_1048;
  size_t local_1040;
  undefined1 local_c68 [222];
  char local_b8a;
  undefined1 local_b70 [222];
  char local_a92;
  undefined1 local_a78 [222];
  char local_99a;
  undefined8 **local_980;
  undefined8 uStack_978;
  char local_8a2;
  undefined **local_888;
  longlong lStack_880;
  undefined8 ****local_878;
  undefined8 local_870;
  undefined8 local_868;
  byte local_7e0;
  undefined2 local_7df;
  byte local_7dd;
  undefined8 local_7dc;
  undefined8 uStack_7d4;
  undefined8 local_7cc;
  undefined8 uStack_7c4;
  undefined8 local_7bc;
  undefined8 uStack_7b4;
  undefined8 uStack_798;
  undefined **local_4a0;
  undefined1 local_498 [160];
  byte local_3f8;
  undefined2 local_3f7;
  byte local_3f5;
  undefined8 local_3f4;
  undefined8 uStack_3ec;
  undefined8 local_3e4;
  undefined8 uStack_3dc;
  undefined8 local_3d4;
  undefined8 uStack_3cc;
  char cStack_3c2;
  undefined8 uStack_3b0;
  undefined8 ***local_3a8;
  code *pcStack_3a0;
  char local_2ca;
  undefined8 ***local_2b0;
  code *local_2a8;
  size_t sStack_2a0;
  undefined8 local_298;
  undefined8 local_290;
  undefined8 local_288;
  undefined8 local_280;
  undefined8 local_278;
  undefined8 local_270;
  undefined *local_240;
  undefined8 uStack_238;
  undefined8 local_230;
  undefined8 uStack_228;
  longlong local_220;
  longlong lStack_218;
  char *local_210;
  undefined4 local_204;
  undefined2 local_200;
  undefined8 ***local_1b8;
  void *local_1b0;
  size_t sStack_1a8;
  undefined4 local_1a0;
  undefined4 uStack_19c;
  uint *local_198;
  uint *local_190;
  longlong local_188;
  longlong local_180;
  undefined **local_178;
  longlong local_170;
  undefined1 local_168 [16];
  undefined1 local_158 [16];
  longlong local_148;
  longlong local_140;
  char *local_138;
  longlong local_130;
  uint *local_128;
  undefined **local_120;
  longlong local_118;
  longlong local_110;
  void *local_108;
  size_t local_100;
  undefined8 ***local_f8;
  char *local_f0;
  char *local_e8;
  undefined8 ***local_e0;
  longlong local_d8;
  undefined8 ***local_d0;
  void *local_c8;
  longlong local_c0;
  undefined8 ***local_b8;
  undefined4 local_ac;
  void *local_a8;
  uint *local_a0;
  undefined1 local_97;
  undefined1 local_96;
  undefined1 local_95;
  undefined1 local_94;
  undefined1 local_93;
  undefined1 local_92;
  undefined1 local_91;
  undefined8 local_90;
  undefined8 uStack_48;
  
  uStack_48 = 0x1408b0576;
  local_90 = 0xfffffffffffffffe;
  if (_DEV_OVERRIDE_IS_DEMO_MODE != '\0') {
    return;
  }
  uVar13 = 0xaca0d6d443d3ae2e;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar13)) ||
     (uVar13 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar13 * 8), uVar13 == 0)) {
LAB_1408b150e:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar12 = ~uVar13 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar12) == 0) goto LAB_1408b150e;
  plVar5 = *(longlong **)(*(longlong *)(in_RDX + 0x1e0) + lVar12 + 0x10);
  uVar13 = 0xd38dae617fb79c7a;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_00 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar13)) ||
     (uVar13 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar13 * 8), uVar13 == 0)) {
LAB_1408b151a:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar12 = ~uVar13 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar12) == 0) goto LAB_1408b151a;
  piVar6 = *(int **)(*(longlong *)(in_RDX + 0x1e0) + lVar12 + 0x10);
  if (((*piVar6 != 10) || ((char)piVar6[2] != '\x01')) || (lVar12 = plVar5[1], lVar12 == 0)) {
    return;
  }
  pcVar16 = (char *)(lVar12 + 0x10);
  LOCK();
  bVar21 = *(char *)(lVar12 + 0x10) == '\0';
  if (bVar21) {
    *(char *)(lVar12 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar21) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  local_148 = lVar12 + 0x18;
  if (*(char *)(lVar12 + 0x4e) == '\a') {
    local_148 = 0;
    parking_lot_core::parking_lot::deadlock_impl::release_resource();
    LOCK();
    cVar2 = *pcVar16;
    if (cVar2 == '\x01') {
      *pcVar16 = '\0';
    }
    UNLOCK();
    if (cVar2 == '\x01') {
      return;
    }
LAB_1408b14fb:
    parking_lot::raw_mutex::RawMutex::unlock_slow();
    return;
  }
  uVar13 = 0x4a142e09642375b9;
  local_f0 = pcVar16;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_01 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar13)) ||
     (uVar13 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar13 * 8), uVar13 == 0)) {
LAB_1408b1534:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar12 = ~uVar13 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar12) == 0) goto LAB_1408b1534;
  local_a8 = *(void **)(*(longlong *)(in_RDX + 0x1e0) + lVar12 + 0x10);
  if (*(longlong *)(in_RDX + 0x88) != 0) {
    lVar12 = *(longlong *)(in_RDX + 0x70);
    uVar13 = *(ulonglong *)(in_RDX + 0x78);
    uVar14 = uVar13 & 0x8b297ec5ae15f15c;
    lVar17 = 0;
    while( true ) {
      pcVar16 = (char *)(lVar12 + uVar14);
      auVar24[0] = -(*pcVar16 == 'E');
      auVar24[1] = -(pcVar16[1] == 'E');
      auVar24[2] = -(pcVar16[2] == 'E');
      auVar24[3] = -(pcVar16[3] == 'E');
      auVar24[4] = -(pcVar16[4] == 'E');
      auVar24[5] = -(pcVar16[5] == 'E');
      auVar24[6] = -(pcVar16[6] == 'E');
      auVar24[7] = -(pcVar16[7] == 'E');
      auVar24[8] = -(pcVar16[8] == 'E');
      auVar24[9] = -(pcVar16[9] == 'E');
      auVar24[10] = -(pcVar16[10] == 'E');
      auVar24[0xb] = -(pcVar16[0xb] == 'E');
      auVar24[0xc] = -(pcVar16[0xc] == 'E');
      auVar24[0xd] = -(pcVar16[0xd] == 'E');
      auVar24[0xe] = -(pcVar16[0xe] == 'E');
      auVar24[0xf] = -(pcVar16[0xf] == 'E');
      uVar19 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
      uVar18 = (uint)uVar19;
      while (uVar19 != 0) {
        uVar3 = 0;
        for (uVar7 = uVar18; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar20 = uVar3 + uVar14 & uVar13;
        if ((*(longlong *)(lVar12 + -0x18 + uVar20 * -0x18) == -0x7a6811f8d84b9e4d) &&
           (*(longlong *)(lVar12 + -0x10 + uVar20 * -0x18) == -0x74d6813a51ea0ea4)) {
          uVar13 = *(ulonglong *)(lVar12 + uVar20 * -0x18 + -8);
          goto LAB_1408b0952;
        }
        uVar19 = (ushort)(uVar18 - 1) & (ushort)uVar18;
        uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar19);
      }
      auVar22[0] = -(*pcVar16 == -1);
      auVar22[1] = -(pcVar16[1] == -1);
      auVar22[2] = -(pcVar16[2] == -1);
      auVar22[3] = -(pcVar16[3] == -1);
      auVar22[4] = -(pcVar16[4] == -1);
      auVar22[5] = -(pcVar16[5] == -1);
      auVar22[6] = -(pcVar16[6] == -1);
      auVar22[7] = -(pcVar16[7] == -1);
      auVar22[8] = -(pcVar16[8] == -1);
      auVar22[9] = -(pcVar16[9] == -1);
      auVar22[10] = -(pcVar16[10] == -1);
      auVar22[0xb] = -(pcVar16[0xb] == -1);
      auVar22[0xc] = -(pcVar16[0xc] == -1);
      auVar22[0xd] = -(pcVar16[0xd] == -1);
      auVar22[0xe] = -(pcVar16[0xe] == -1);
      auVar22[0xf] = -(pcVar16[0xf] == -1);
      if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar22 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar22 >> 0x3f,0) & 1) != 0) || (SUB161(auVar22 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar22 >> 0x4f,0) & 1) != 0) || (SUB161(auVar22 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar22 >> 0x5f,0) & 1) != 0) || (SUB161(auVar22 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar22 >> 0x6f,0) & 1) != 0) || (SUB161(auVar22 >> 0x77,0) & 1) != 0) ||
          auVar22[0xf] < '\0') break;
      lVar15 = uVar14 + lVar17;
      lVar17 = lVar17 + 0x10;
      uVar14 = lVar15 + 0x10U & uVar13;
    }
  }
  local_a0 = (uint *)(in_RDX + 0x90);
  uVar18 = *(uint *)(in_RDX + 0x90);
  if (uVar18 < 0x3ffffffe) {
    LOCK();
    uVar3 = *local_a0;
    if (uVar18 == uVar3) {
      *local_a0 = uVar18 + 1;
    }
    UNLOCK();
    if (uVar18 == uVar3) goto LAB_1408b0877;
  }
  std::sys::sync::rwlock::futex::RwLock::read_contended();
LAB_1408b0877:
  if (*(longlong *)(in_RDX + 0xf0) == 0) {
LAB_1408b1550:
    LOCK();
    uVar18 = *local_a0;
    *local_a0 = *local_a0 - 1;
    UNLOCK();
    if (SBORROW4(0,uVar18 - 1 & 0xbfffffff)) {
      std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
    }
LAB_1408b156d:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar12 = *(longlong *)(in_RDX + 0xd8);
  uVar13 = *(ulonglong *)(in_RDX + 0xe0);
  uVar14 = uVar13 & 0x8b297ec5ae15f15c;
  lVar17 = 0;
  while( true ) {
    pcVar16 = (char *)(lVar12 + uVar14);
    auVar25[0] = -(*pcVar16 == 'E');
    auVar25[1] = -(pcVar16[1] == 'E');
    auVar25[2] = -(pcVar16[2] == 'E');
    auVar25[3] = -(pcVar16[3] == 'E');
    auVar25[4] = -(pcVar16[4] == 'E');
    auVar25[5] = -(pcVar16[5] == 'E');
    auVar25[6] = -(pcVar16[6] == 'E');
    auVar25[7] = -(pcVar16[7] == 'E');
    auVar25[8] = -(pcVar16[8] == 'E');
    auVar25[9] = -(pcVar16[9] == 'E');
    auVar25[10] = -(pcVar16[10] == 'E');
    auVar25[0xb] = -(pcVar16[0xb] == 'E');
    auVar25[0xc] = -(pcVar16[0xc] == 'E');
    auVar25[0xd] = -(pcVar16[0xd] == 'E');
    auVar25[0xe] = -(pcVar16[0xe] == 'E');
    auVar25[0xf] = -(pcVar16[0xf] == 'E');
    uVar19 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar25 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar25 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar25 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar25 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar25 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar25 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar25 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar25 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar25 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar25 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar25 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar25 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe | (ushort)(auVar25[0xf] >> 7) << 0xf;
    uVar18 = (uint)uVar19;
    if (uVar19 != 0) break;
LAB_1408b08c7:
    auVar23[0] = -(*pcVar16 == -1);
    auVar23[1] = -(pcVar16[1] == -1);
    auVar23[2] = -(pcVar16[2] == -1);
    auVar23[3] = -(pcVar16[3] == -1);
    auVar23[4] = -(pcVar16[4] == -1);
    auVar23[5] = -(pcVar16[5] == -1);
    auVar23[6] = -(pcVar16[6] == -1);
    auVar23[7] = -(pcVar16[7] == -1);
    auVar23[8] = -(pcVar16[8] == -1);
    auVar23[9] = -(pcVar16[9] == -1);
    auVar23[10] = -(pcVar16[10] == -1);
    auVar23[0xb] = -(pcVar16[0xb] == -1);
    auVar23[0xc] = -(pcVar16[0xc] == -1);
    auVar23[0xd] = -(pcVar16[0xd] == -1);
    auVar23[0xe] = -(pcVar16[0xe] == -1);
    auVar23[0xf] = -(pcVar16[0xf] == -1);
    if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar23 >> 0x37,0) & 1) != 0) || (SUB161(auVar23 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar23 >> 0x47,0) & 1) != 0) || (SUB161(auVar23 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar23 >> 0x57,0) & 1) != 0) || (SUB161(auVar23 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar23 >> 0x67,0) & 1) != 0) || (SUB161(auVar23 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar23 >> 0x77,0) & 1) != 0) || auVar23[0xf] < '\0') goto LAB_1408b1550;
    lVar15 = uVar14 + lVar17;
    lVar17 = lVar17 + 0x10;
    uVar14 = lVar15 + 0x10U & uVar13;
  }
  while( true ) {
    uVar3 = 0;
    for (uVar7 = uVar18; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
      uVar3 = uVar3 + 1;
    }
    lVar15 = (uVar3 + uVar14 & uVar13) * -0x88;
    if ((*(longlong *)(lVar12 + -0x88 + lVar15) == -0x7a6811f8d84b9e4d) &&
       (*(longlong *)(lVar12 + -0x80 + lVar15) == -0x74d6813a51ea0ea4)) break;
    uVar19 = (ushort)(uVar18 - 1) & (ushort)uVar18;
    uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar19);
    if (uVar19 == 0) goto LAB_1408b08c7;
  }
  uVar13 = *(ulonglong *)(lVar12 + lVar15 + -8);
  LOCK();
  uVar18 = *local_a0;
  *local_a0 = *local_a0 - 1;
  UNLOCK();
  if (SBORROW4(0,uVar18 - 1 & 0xbfffffff)) {
    std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
  }
LAB_1408b0952:
  if (((*(ulonglong *)(in_RDX + 0x218) <= uVar13) ||
      (uVar13 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar13 * 8), uVar13 == 0)) ||
     (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar13 * 0x60) == 0))
  goto LAB_1408b156d;
  pcVar16 = "lut_gallery_thumbnailcrates/country-core/src/systems/render_loop/lut_gallery.rs";
  asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name();
  local_138 = pcVar16;
  if ((extraout_RAX_02 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar13 = 0xd387f1aea6230aea;
  bevy_ecs::component::Components::get_resource_id();
  if (((extraout_RAX_03 == 0) || (*(ulonglong *)(in_RDX + 0x218) <= uVar13)) ||
     (uVar13 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar13 * 8), uVar13 == 0)) {
LAB_1408b1542:
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar12 = ~uVar13 * 0x60;
  if (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + lVar12) == 0) goto LAB_1408b1542;
  lVar12 = *(longlong *)(*(longlong *)(in_RDX + 0x1e0) + lVar12 + 0x10);
  local_d8 = *plVar5;
  pcVar16 = (char *)(local_d8 + 0x10);
  LOCK();
  bVar21 = *(char *)(local_d8 + 0x10) == '\0';
  if (bVar21) {
    *(char *)(local_d8 + 0x10) = '\x01';
  }
  UNLOCK();
  if (!bVar21) {
    parking_lot::raw_mutex::RawMutex::lock_slow();
  }
  parking_lot_core::parking_lot::deadlock_impl::acquire_resource();
  pvVar9 = local_a8;
  local_3a8 = *(undefined8 ****)((longlong)local_a8 + 8);
  pcStack_3a0 = *(code **)((longlong)local_a8 + 0x10);
  local_2b0 = &local_3a8;
  local_2a8 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_888 = &anon_ff3ecbf17b154c6586c2d7358fc9984a_75_llvm_1075954330245063046;
  lStack_880 = 1;
  local_870 = 1;
  local_868 = 0;
  local_878 = &local_2b0;
  local_e8 = pcVar16;
  alloc::fmt::format::format_inner();
  uVar4 = *(undefined4 *)((longlong)pvVar9 + 0x18);
  local_b8 = local_1050;
  local_c8 = local_1048;
  local_a0 = *(uint **)(lVar12 + 8);
  lVar12 = *(longlong *)(lVar12 + 0x10);
  local_130 = in_R8 + 4;
  local_91 = 1;
  rhapsody::raw::texture::TextureDesc::extent_2d();
  local_888 = (undefined **)0x3e3851ec00000001;
  lStack_880 = 0x3e3851ec3e3851ec;
  local_878 = (undefined8 ****)CONCAT44(local_878._4_4_,0x3f800000);
  local_2b0 = (undefined8 ***)CONCAT44(local_2b0._4_4_,2);
  local_91 = 1;
  country_core::render::pass_builder::draw_stream::RasterPass::new_impl();
  plVar5 = (longlong *)
           ((longlong)local_a8 + (ulonglong)*(byte *)((longlong)local_a8 + 0xc0) * 0x40 + 8);
  local_3a8 = (undefined8 ***)*plVar5;
  pcStack_3a0 = (code *)plVar5[1];
  local_2b0 = &local_3a8;
  local_2a8 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
  local_888 = &anon_ff3ecbf17b154c6586c2d7358fc9984a_75_llvm_1075954330245063046;
  lStack_880 = 1;
  local_870 = 1;
  local_868 = 0;
  local_95 = 1;
  local_96 = 1;
  local_878 = &local_2b0;
  alloc::fmt::format::format_inner();
  local_2b0 = (undefined8 ***)(local_d8 + 0x14);
  local_2a8 = (code *)((ulonglong)*(uint *)(local_d8 + 0x3c) << 0x20);
  local_92 = 1;
  local_93 = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<glam::f32::vec2::Vec2,core::iter::adapters::map::Map<core::ops::range::Range<u32>,country_core::resources::gui::impl_6::positions::closure_env_0>_>
            ();
  local_190 = local_a0 + lVar12 * 10;
  local_170 = lStack_880 + (longlong)local_878 * 8;
  local_168 = (undefined1  [16])0x0;
  local_158 = (undefined1  [16])0x0;
  local_1b8 = local_b8;
  local_1b0 = local_c8;
  sStack_1a8 = local_1040;
  local_198 = local_a0;
  local_188 = lStack_880;
  local_120 = local_888;
  local_178 = local_888;
  local_d0 = local_b8;
  lVar17 = 0;
  local_118 = lStack_880;
  pppuVar11 = local_b8;
  lVar12 = lStack_880;
  local_1a0 = uVar4;
  local_140 = local_170;
  local_128 = local_190;
  do {
    if (lVar12 == local_140) {
LAB_1408b13b4:
      if (local_120 != (undefined **)0x0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((-0x7fffffffffffffff < (longlong)local_d0) && (local_d0 != (undefined8 ***)0x0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      memcpy(&local_888,&local_1050,1000);
      local_92 = 0;
      local_93 = 0;
      country_core::render::pass_builder::draw_stream::RasterPass::render();
      if (local_110 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      pcVar16 = local_e8;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar21 = *pcVar16 == '\x01';
      if (bVar21) {
        *pcVar16 = '\0';
      }
      UNLOCK();
      if (!bVar21) {
        parking_lot::raw_mutex::RawMutex::unlock_slow();
      }
      pcVar16 = local_f0;
      parking_lot_core::parking_lot::deadlock_impl::release_resource();
      LOCK();
      bVar21 = *pcVar16 == '\x01';
      if (bVar21) {
        *pcVar16 = '\0';
      }
      UNLOCK();
      if (bVar21) {
        return;
      }
      goto LAB_1408b14fb;
    }
    lVar12 = lVar12 + 8;
    local_180 = lVar12;
    if (pppuVar11 == (undefined8 ***)0x8000000000000001) {
LAB_1408b0d7c:
      if ((local_a0 != (uint *)0x0) && (local_a0 != local_128)) {
        puVar1 = local_a0 + 10;
        local_980 = *(undefined8 ***)(local_a0 + 2);
        uStack_978 = *(undefined8 *)(local_a0 + 4);
        local_3a8 = &local_980;
        pcStack_3a0 = anyhow::context::impl$3::
                      fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
        local_888 = &anon_ff3ecbf17b154c6586c2d7358fc9984a_75_llvm_1075954330245063046;
        lStack_880 = 1;
        local_878 = &local_3a8;
        local_870 = 1;
        local_868 = 0;
        local_198 = puVar1;
        alloc::fmt::format::format_inner();
        if (!SBORROW8(0,(longlong)local_2b0)) {
          local_f8 = (undefined8 ***)0x8000000000000001;
          local_a0 = puVar1;
          pppuVar11 = local_2b0;
          goto LAB_1408b0e37;
        }
      }
      goto LAB_1408b13b4;
    }
    local_1b8 = (undefined8 ***)0x8000000000000000;
    if (SBORROW8(0,(longlong)pppuVar11)) {
      local_1b8 = (undefined8 ***)0x8000000000000001;
      local_d0 = (undefined8 ***)0x8000000000000001;
      goto LAB_1408b0d7c;
    }
    local_298 = CONCAT44(uStack_19c,local_1a0);
    local_2a8 = local_1b0;
    sStack_2a0 = sStack_1a8;
    local_d0 = (undefined8 ***)0x8000000000000000;
    local_f8 = (undefined8 ***)0x8000000000000000;
LAB_1408b0e37:
    lVar17 = lVar17 + 1;
    local_158._8_8_ = lVar17;
    local_a8 = local_2a8;
    local_e0 = pppuVar11;
    if (local_100 == sStack_2a0) {
      iVar10 = memcmp(local_108,local_2a8,sStack_2a0);
      local_ac = CONCAT31((int3)((uint)iVar10 >> 8),iVar10 == 0);
    }
    else {
      local_ac = 0;
    }
    rhapsody::raw::shader::impl$7::default();
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_200 = 0;
    local_204 = 0x3f800000;
    local_2b0 = (undefined8 ***)0x0;
    local_210 = local_138;
    sStack_2a0 = 0;
    local_298 = 4;
    local_290 = 0;
    local_220 = *extraout_RAX_04;
    lStack_218 = extraout_RAX_04[1];
    *extraout_RAX_04 = *extraout_RAX_04 + 1;
    local_230 = 0;
    uStack_228 = 0;
    local_240 = &anon_65ceffb69360efd323e48f4f8756afaa_2_llvm_13316938065337964131;
    uStack_238 = 0;
    local_270 = 0x100000003;
    local_288 = 0;
    local_280 = 1;
    local_278 = 0;
    country_core::render::pass_builder::draw_builder::DrawBuilder::new();
    local_c0 = local_148;
    local_2b0._0_4_ = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<ref_<rhapsody::raw::texture::Texture>_>();
    memcpy(&local_2b0,&local_888,0xf8);
    local_c0 = 0xa647e80af4e9395f;
    local_888 = (undefined **)CONCAT44(local_888._4_4_,2);
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(&local_888,&local_2b0,0xf8);
    local_94 = 1;
    ::utils::name_hash::NameHash::compute();
    local_2b0 = (undefined8 ***)CONCAT44(local_2b0._4_4_,2);
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(&local_2b0,&local_888,0xf8);
    local_97 = 1;
    rhapsody::raw::texture::TextureDesc::extent_2d();
    local_97 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::optional_uniform<array_<u32,2>_>();
    if (local_2ca != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<f32>,f32>
                ();
    }
    memcpy(&local_980,&local_3a8,0xf8);
    if (local_8a2 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
                ();
    }
    memcpy(local_c68,&local_980,0xf8);
    if (local_b8a != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<glam::f32::vec2::Vec2>,glam::f32::vec2::Vec2>
                ();
    }
    memcpy(local_b70,local_c68,0xf8);
    if (local_a92 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(local_a78,local_b70,0xf8);
    if (local_99a != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(&local_4a0,local_a78,0xf8);
    if (cStack_3c2 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    ppuVar8 = local_4a0;
    memcpy(&lStack_880,local_498,0xa0);
    local_7dc = local_3f4;
    uStack_7d4 = uStack_3ec;
    local_7cc = local_3e4;
    uStack_7c4 = uStack_3dc;
    local_7bc = local_3d4;
    uStack_7b4 = uStack_3cc;
    uStack_798 = uStack_3b0;
    local_7e0 = local_3f8 & 0x7f;
    local_7dd = local_3f5 & 0x9f | 0x40;
    local_888 = ppuVar8;
    local_7df = local_3f7;
    country_core::render::pass_builder::draw_builder::DrawBuilder::draw_no_vao();
    pppuVar11 = local_f8;
    if (local_e0 != (undefined8 ***)0x0) {
      std::alloc::__default_lib_allocator::__rust_dealloc();
    }
  } while( true );
}

