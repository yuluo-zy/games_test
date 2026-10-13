
void __cdecl
country_core::systems::render_loop::compute_passes::terrain_lighting_texture::
calc_terrain_lighting_texture(void)

{
  int *piVar1;
  char *pcVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  longlong extraout_RAX;
  longlong in_RDX;
  ulonglong uVar10;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint uVar14;
  ushort uVar15;
  ulonglong uVar16;
  bool bVar17;
  char in_XMM1_Ba;
  char in_XMM1_Bb;
  char in_XMM1_Bc;
  char in_XMM1_Bd;
  char in_XMM1_Be;
  char in_XMM1_Bf;
  char in_XMM1_Bg;
  char in_XMM1_Bh;
  char in_XMM1_Bi;
  char in_XMM1_Bj;
  char in_XMM1_Bk;
  char in_XMM1_Bl;
  char in_XMM1_Bm;
  char in_XMM1_Bn;
  char in_XMM1_Bo;
  char in_XMM1_Bp;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined8 local_518;
  undefined8 uStack_510;
  undefined8 local_508;
  undefined8 uStack_500;
  undefined8 local_4f8;
  undefined8 uStack_4f0;
  undefined8 local_4e8;
  undefined8 uStack_4e0;
  undefined8 local_4d8;
  undefined8 uStack_4d0;
  char local_3e2;
  undefined4 local_388 [2];
  undefined8 uStack_380;
  undefined8 local_378;
  undefined8 local_370;
  undefined8 uStack_368;
  undefined8 local_360;
  undefined8 local_358;
  undefined8 uStack_350;
  undefined8 local_348;
  undefined8 local_340;
  undefined8 uStack_338;
  undefined8 local_330;
  undefined8 local_328;
  undefined8 uStack_320;
  undefined8 local_318;
  undefined8 uStack_310;
  undefined8 local_308;
  undefined8 uStack_300;
  undefined8 local_2f8;
  undefined8 uStack_2f0;
  undefined8 local_2e8;
  undefined8 uStack_2e0;
  undefined8 local_2d8;
  undefined8 uStack_2d0;
  undefined8 local_2c8;
  undefined8 uStack_2c0;
  undefined8 local_2b8;
  undefined8 uStack_2b0;
  undefined8 local_2a8;
  undefined8 uStack_2a0;
  undefined8 local_298;
  undefined8 uStack_290;
  undefined8 local_288;
  undefined8 uStack_280;
  char local_252;
  undefined8 local_250;
  undefined8 local_208;
  undefined4 local_1f8;
  undefined4 uStack_1f4;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined4 uStack_1e8;
  undefined4 uStack_1e4;
  undefined4 uStack_1e0;
  undefined4 uStack_1dc;
  undefined4 uStack_1d8;
  undefined4 uStack_1d4;
  undefined8 local_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  undefined8 local_1b0;
  undefined8 uStack_1a8;
  undefined8 local_1a0;
  undefined4 local_198;
  undefined4 local_18c;
  undefined8 local_180;
  undefined8 uStack_178;
  undefined8 local_170;
  undefined8 uStack_168;
  undefined8 local_160;
  undefined8 uStack_158;
  undefined8 local_150;
  undefined8 uStack_148;
  undefined8 local_140;
  undefined8 uStack_138;
  undefined8 local_130;
  undefined8 uStack_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined1 local_48;
  undefined7 uStack_47;
  undefined4 local_40;
  undefined1 local_39;
  undefined8 local_38;
  
  local_38 = 0xfffffffffffffffe;
  if (*(longlong *)(in_RDX + 0x88) != 0) {
    lVar3 = *(longlong *)(in_RDX + 0x70);
    uVar12 = *(ulonglong *)(in_RDX + 0x78);
    uVar10 = uVar12 & 0xf5fd74e6faf4ed14;
    lVar13 = 0;
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    in_XMM1_Ba = (char)iVar6;
    in_XMM1_Bb = (char)((uint)iVar6 >> 8);
    in_XMM1_Bc = (char)((uint)iVar6 >> 0x10);
    in_XMM1_Bd = (char)((uint)iVar6 >> 0x18);
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    in_XMM1_Be = (char)iVar6;
    in_XMM1_Bf = (char)((uint)iVar6 >> 8);
    in_XMM1_Bg = (char)((uint)iVar6 >> 0x10);
    in_XMM1_Bh = (char)((uint)iVar6 >> 0x18);
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    in_XMM1_Bi = (char)iVar6;
    in_XMM1_Bj = (char)((uint)iVar6 >> 8);
    in_XMM1_Bk = (char)((uint)iVar6 >> 0x10);
    in_XMM1_Bl = (char)((uint)iVar6 >> 0x18);
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    in_XMM1_Bm = (char)iVar6;
    in_XMM1_Bn = (char)((uint)iVar6 >> 8);
    in_XMM1_Bo = (char)((uint)iVar6 >> 0x10);
    in_XMM1_Bp = (char)((uint)iVar6 >> 0x18);
    while( true ) {
      pcVar2 = (char *)(lVar3 + uVar10);
      auVar20[0] = -(*pcVar2 == 'z');
      auVar20[1] = -(pcVar2[1] == 'z');
      auVar20[2] = -(pcVar2[2] == 'z');
      auVar20[3] = -(pcVar2[3] == 'z');
      auVar20[4] = -(pcVar2[4] == 'z');
      auVar20[5] = -(pcVar2[5] == 'z');
      auVar20[6] = -(pcVar2[6] == 'z');
      auVar20[7] = -(pcVar2[7] == 'z');
      auVar20[8] = -(pcVar2[8] == 'z');
      auVar20[9] = -(pcVar2[9] == 'z');
      auVar20[10] = -(pcVar2[10] == 'z');
      auVar20[0xb] = -(pcVar2[0xb] == 'z');
      auVar20[0xc] = -(pcVar2[0xc] == 'z');
      auVar20[0xd] = -(pcVar2[0xd] == 'z');
      auVar20[0xe] = -(pcVar2[0xe] == 'z');
      auVar20[0xf] = -(pcVar2[0xf] == 'z');
      uVar15 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar14 = (uint)uVar15;
      while (uVar15 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar16 = uVar4 + uVar10 & uVar12;
        if ((*(longlong *)(lVar3 + -0x18 + uVar16 * -0x18) == -0x597a9e4c6987b156) &&
           (*(longlong *)(lVar3 + -0x10 + uVar16 * -0x18) == -0xa028b19050b12ec)) {
          uVar12 = *(ulonglong *)(lVar3 + uVar16 * -0x18 + -8);
          if (*(ulonglong *)(in_RDX + 0x218) <= uVar12) goto LAB_140a51aef;
          goto LAB_140a5162c;
        }
        uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
        uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
      }
      auVar18[0] = -(*pcVar2 == in_XMM1_Ba);
      auVar18[1] = -(pcVar2[1] == in_XMM1_Bb);
      auVar18[2] = -(pcVar2[2] == in_XMM1_Bc);
      auVar18[3] = -(pcVar2[3] == in_XMM1_Bd);
      auVar18[4] = -(pcVar2[4] == in_XMM1_Be);
      auVar18[5] = -(pcVar2[5] == in_XMM1_Bf);
      auVar18[6] = -(pcVar2[6] == in_XMM1_Bg);
      auVar18[7] = -(pcVar2[7] == in_XMM1_Bh);
      auVar18[8] = -(pcVar2[8] == in_XMM1_Bi);
      auVar18[9] = -(pcVar2[9] == in_XMM1_Bj);
      auVar18[10] = -(pcVar2[10] == in_XMM1_Bk);
      auVar18[0xb] = -(pcVar2[0xb] == in_XMM1_Bl);
      auVar18[0xc] = -(pcVar2[0xc] == in_XMM1_Bm);
      auVar18[0xd] = -(pcVar2[0xd] == in_XMM1_Bn);
      auVar18[0xe] = -(pcVar2[0xe] == in_XMM1_Bo);
      auVar18[0xf] = -(pcVar2[0xf] == in_XMM1_Bp);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      lVar11 = uVar10 + lVar13;
      lVar13 = lVar13 + 0x10;
      uVar10 = lVar11 + 0x10U & uVar12;
    }
  }
  uVar14 = *(uint *)(in_RDX + 0x90);
  if (uVar14 < 0x3ffffffe) {
    LOCK();
    bVar17 = uVar14 == *(uint *)(in_RDX + 0x90);
    if (bVar17) {
      *(uint *)(in_RDX + 0x90) = uVar14 + 1;
    }
    UNLOCK();
    if (bVar17) {
      lVar3 = *(longlong *)(in_RDX + 0xf0);
      goto joined_r0x000140a51558;
    }
  }
  std::sys::sync::rwlock::futex::RwLock::read_contended();
  lVar3 = *(longlong *)(in_RDX + 0xf0);
joined_r0x000140a51558:
  if (lVar3 != 0) {
    lVar3 = *(longlong *)(in_RDX + 0xd8);
    uVar12 = *(ulonglong *)(in_RDX + 0xe0);
    uVar10 = uVar12 & 0xf5fd74e6faf4ed14;
    lVar13 = 0;
    iVar6 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                   CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    do {
      pcVar2 = (char *)(lVar3 + uVar10);
      auVar21[0] = -(*pcVar2 == 'z');
      auVar21[1] = -(pcVar2[1] == 'z');
      auVar21[2] = -(pcVar2[2] == 'z');
      auVar21[3] = -(pcVar2[3] == 'z');
      auVar21[4] = -(pcVar2[4] == 'z');
      auVar21[5] = -(pcVar2[5] == 'z');
      auVar21[6] = -(pcVar2[6] == 'z');
      auVar21[7] = -(pcVar2[7] == 'z');
      auVar21[8] = -(pcVar2[8] == 'z');
      auVar21[9] = -(pcVar2[9] == 'z');
      auVar21[10] = -(pcVar2[10] == 'z');
      auVar21[0xb] = -(pcVar2[0xb] == 'z');
      auVar21[0xc] = -(pcVar2[0xc] == 'z');
      auVar21[0xd] = -(pcVar2[0xd] == 'z');
      auVar21[0xe] = -(pcVar2[0xe] == 'z');
      auVar21[0xf] = -(pcVar2[0xf] == 'z');
      uVar15 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar21 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar21 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar21 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar21 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar21 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar21 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar21 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar21 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar21 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar21 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar21 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar21 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar21 >> 0x77,0) & 1) << 0xe | (ushort)(auVar21[0xf] >> 7) << 0xf;
      uVar14 = (uint)uVar15;
      while (uVar15 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar14; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        lVar11 = (uVar4 + uVar10 & uVar12) * -0x88;
        if ((*(longlong *)(lVar3 + -0x88 + lVar11) == -0x597a9e4c6987b156) &&
           (*(longlong *)(lVar3 + -0x80 + lVar11) == -0xa028b19050b12ec)) {
          uVar12 = *(ulonglong *)(lVar3 + lVar11 + -8);
          LOCK();
          piVar1 = (int *)(in_RDX + 0x90);
          iVar6 = *piVar1;
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (SBORROW4(0,iVar6 - 1U & 0xbfffffff)) {
            std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
            if (*(ulonglong *)(in_RDX + 0x218) <= uVar12) goto LAB_140a51aef;
            goto LAB_140a5162c;
          }
          if (uVar12 < *(ulonglong *)(in_RDX + 0x218)) {
LAB_140a5162c:
            uVar12 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar12 * 8);
            if ((uVar12 != 0) &&
               (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar12 * 0x60) != 0)) {
              asset_libraries::texture_library::AssetTextureLibrary::get_rg_texture();
              country_core::render::pass_builder::compute_pass_builder::ComputePass::
              new<ref_<str_>_>();
              local_388[0] = CONCAT31(local_388[0]._1_3_,8);
              local_518 = CONCAT44(local_518._4_4_,2);
              country_core::render::pass_builder::compute_pass_builder::impl$4::
              bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                        ();
              memcpy(&local_518,&local_1f8,400);
              local_39 = 1;
              uVar12 = 0xbba451ee6a08a3ff;
              bevy_ecs::component::Components::get_resource_id();
              if ((extraout_RAX != 0) &&
                 (((uVar12 < *(ulonglong *)(in_RDX + 0x218) &&
                   (uVar12 = *(ulonglong *)(*(longlong *)(in_RDX + 0x210) + uVar12 * 8), uVar12 != 0
                   )) && (*(longlong *)(*(longlong *)(in_RDX + 0x1e0) + 0x20 + ~uVar12 * 0x60) != 0)
                  ))) {
                local_39 = 1;
                ::utils::camera::CameraMatrices::eye_position();
                if (local_3e2 != '\b') {
                  uStack_1f0 = local_40;
                  local_1f8 = (undefined4)CONCAT71(uStack_47,local_48);
                  uStack_1f4 = (undefined4)((uint7)uStack_47 >> 0x18);
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<glam::f32::vec3::Vec3>,glam::f32::vec3::Vec3>
                            ();
                }
                memcpy(local_388,&local_518,400);
                local_518 = CONCAT71(local_518._1_7_,9);
                local_1f8 = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_1f8,local_388,400);
                local_48 = 10;
                local_518 = CONCAT44(local_518._4_4_,2);
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(&local_518,&local_1f8,400);
                if (local_3e2 != '\b') {
                  country_core::render::pass_builder::uniform_binder::UniformBinder::
                  write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                            ();
                }
                memcpy(&local_1f8,&local_518,400);
                local_48 = 0x11;
                local_388[0] = 2;
                country_core::render::pass_builder::compute_pass_builder::impl$4::
                bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
                          ();
                memcpy(local_388,&local_1f8,400);
                local_4d8 = local_2e8;
                uStack_4d0 = uStack_2e0;
                local_4e8 = local_2f8;
                uStack_4e0 = uStack_2f0;
                local_4f8 = local_308;
                uStack_4f0 = uStack_300;
                local_508 = local_318;
                uStack_500 = uStack_310;
                local_518 = local_328;
                uStack_510 = uStack_320;
                uStack_1e8 = (undefined4)local_378;
                uStack_1e4 = (undefined4)((ulonglong)local_378 >> 0x20);
                local_1f8 = local_388[0];
                uStack_1f0 = (undefined4)uStack_380;
                uStack_1ec = (undefined4)((ulonglong)uStack_380 >> 0x20);
                local_1c8 = local_370;
                uStack_1c0 = uStack_368;
                local_1b8 = local_360;
                uStack_1e0 = (undefined4)local_358;
                uStack_1dc = (undefined4)((ulonglong)local_358 >> 0x20);
                uStack_1d8 = (undefined4)uStack_350;
                uStack_1d4 = (undefined4)((ulonglong)uStack_350 >> 0x20);
                local_1d0 = local_348;
                local_180 = local_2d8;
                uStack_178 = uStack_2d0;
                local_170 = local_2c8;
                uStack_168 = uStack_2c0;
                local_160 = local_2b8;
                uStack_158 = uStack_2b0;
                local_150 = local_2a8;
                uStack_148 = uStack_2a0;
                local_140 = local_298;
                uStack_138 = uStack_290;
                local_130 = local_288;
                uStack_128 = uStack_280;
                local_1a0 = local_330;
                local_1b0 = local_340;
                uStack_1a8 = uStack_338;
                local_118 = local_208;
                local_120 = local_250;
                local_198 = 1;
                local_18c = 1;
                rhapsody::rg::pass_builder::PassBuilder::
                render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl_0::dispatch_impl::closure_env_0>
                          ();
                if (local_252 != '\b') {
                  rhapsody::raw::managed::handle::impl$6::drop();
                }
                generate_mips::
                generate_mips<country_core::resources::texture_presets::TexturePreset>();
                return;
              }
              local_39 = 1;
                    // WARNING: Subroutine does not return
              core::option::unwrap_failed();
            }
          }
          goto LAB_140a51aef;
        }
        uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
        uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
      }
      auVar19[0] = -(*pcVar2 == (char)iVar6);
      auVar19[1] = -(pcVar2[1] == (char)((uint)iVar6 >> 8));
      auVar19[2] = -(pcVar2[2] == (char)((uint)iVar6 >> 0x10));
      auVar19[3] = -(pcVar2[3] == (char)((uint)iVar6 >> 0x18));
      auVar19[4] = -(pcVar2[4] == (char)iVar7);
      auVar19[5] = -(pcVar2[5] == (char)((uint)iVar7 >> 8));
      auVar19[6] = -(pcVar2[6] == (char)((uint)iVar7 >> 0x10));
      auVar19[7] = -(pcVar2[7] == (char)((uint)iVar7 >> 0x18));
      auVar19[8] = -(pcVar2[8] == (char)iVar8);
      auVar19[9] = -(pcVar2[9] == (char)((uint)iVar8 >> 8));
      auVar19[10] = -(pcVar2[10] == (char)((uint)iVar8 >> 0x10));
      auVar19[0xb] = -(pcVar2[0xb] == (char)((uint)iVar8 >> 0x18));
      auVar19[0xc] = -(pcVar2[0xc] == (char)iVar9);
      auVar19[0xd] = -(pcVar2[0xd] == (char)((uint)iVar9 >> 8));
      auVar19[0xe] = -(pcVar2[0xe] == (char)((uint)iVar9 >> 0x10));
      auVar19[0xf] = -(pcVar2[0xf] == (char)((uint)iVar9 >> 0x18));
      if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      lVar11 = uVar10 + lVar13;
      lVar13 = lVar13 + 0x10;
      uVar10 = lVar11 + 0x10U & uVar12;
    } while( true );
  }
  LOCK();
  piVar1 = (int *)(in_RDX + 0x90);
  iVar6 = *piVar1;
  *piVar1 = *piVar1 + -1;
  UNLOCK();
  if (SBORROW4(0,iVar6 - 1U & 0xbfffffff)) {
    std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
LAB_140a51aef:
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


