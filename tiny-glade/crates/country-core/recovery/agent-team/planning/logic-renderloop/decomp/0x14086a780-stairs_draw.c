// 0x14086a780 stairs_draw size=1140

void __cdecl
country_core::systems::render_loop::draw_passes::draw_shaders::stairs_draw::stairs_draw(void)

{
  uint *puVar1;
  char *pcVar2;
  uint uVar3;
  longlong lVar4;
  ulonglong uVar5;
  uint uVar6;
  longlong extraout_RAX;
  void *in_RCX;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  uint uVar10;
  ushort uVar11;
  ulonglong uVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined4 local_248 [55];
  char local_16a;
  undefined4 local_150 [2];
  uint local_148;
  longlong local_140;
  int local_134;
  char local_72;
  void *local_58;
  undefined8 local_50;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  local_41 = 1;
  local_58 = in_RCX;
  bevy_ecs::world::entity_fetch::impl$1::fetch_ref();
  if (local_134 == 0) {
LAB_14086a9f2:
    memcpy(local_150,local_58,0xf8);
    local_41 = 0;
    country_core::render::pass_builder::draw_builder::impl$2::
    read_storage_buffer<country_core::systems::render_loop::compute_passes::stairs::StairsTransformSsbo>
              ();
    local_50 = 0x84a037e697de0324;
    local_150[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TextureFromDisk>();
    memcpy(local_150,local_248,0xf8);
    if (local_72 != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::compute_pass_builder::impl_3::uniform::closure_env_0<i32>,i32>
                ();
    }
    memcpy(local_248,local_150,0xf8);
    if (local_16a != '\b') {
      country_core::render::pass_builder::uniform_binder::UniformBinder::
      write_uniform<country_core::render::pass_builder::draw_builder::impl_2::uniform::closure_env_0<bool>,bool>
                ();
    }
    memcpy(local_150,local_248,0xf8);
    local_50 = CONCAT71(local_50._1_7_,0xc);
    local_248[0] = 2;
    country_core::render::pass_builder::draw_builder::impl$3::
    bind_texture_to_draw_builder<country_core::resources::texture_presets::TexturePreset>();
    memcpy(local_248,local_150,0xf8);
    local_41 = 0;
    country_core::render::pass_builder::draw_builder::DrawBuilder::multi_draw_indexed_indirect();
    return;
  }
  if (*(longlong *)(local_140 + 0x68) != 0) {
    lVar4 = *(longlong *)(local_140 + 0x50);
    uVar5 = *(ulonglong *)(local_140 + 0x58);
    uVar7 = uVar5 & 0x3bad471c3be19d00;
    lVar9 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar4 + uVar7);
      auVar15[0] = -(*pcVar2 == '\x1d');
      auVar15[1] = -(pcVar2[1] == '\x1d');
      auVar15[2] = -(pcVar2[2] == '\x1d');
      auVar15[3] = -(pcVar2[3] == '\x1d');
      auVar15[4] = -(pcVar2[4] == '\x1d');
      auVar15[5] = -(pcVar2[5] == '\x1d');
      auVar15[6] = -(pcVar2[6] == '\x1d');
      auVar15[7] = -(pcVar2[7] == '\x1d');
      auVar15[8] = -(pcVar2[8] == '\x1d');
      auVar15[9] = -(pcVar2[9] == '\x1d');
      auVar15[10] = -(pcVar2[10] == '\x1d');
      auVar15[0xb] = -(pcVar2[0xb] == '\x1d');
      auVar15[0xc] = -(pcVar2[0xc] == '\x1d');
      auVar15[0xd] = -(pcVar2[0xd] == '\x1d');
      auVar15[0xe] = -(pcVar2[0xe] == '\x1d');
      auVar15[0xf] = -(pcVar2[0xf] == '\x1d');
      uVar11 = (ushort)(SUB161(auVar15 >> 7,0) & 1) | (ushort)(SUB161(auVar15 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar15 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar15 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar15 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar15 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar15 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar15 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar15 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar15 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar15 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar15 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar15 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar15 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar15 >> 0x77,0) & 1) << 0xe | (ushort)(auVar15[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar3 = 0;
        for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        uVar12 = uVar3 + uVar7 & uVar5;
        if ((*(longlong *)(lVar4 + -0x18 + uVar12 * -0x18) == 0x775c6361f3425a3a) &&
           (*(longlong *)(lVar4 + -0x10 + uVar12 * -0x18) == 0x3bad471c3be19d00))
        goto LAB_14086a9bc;
        uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
      }
      auVar13[0] = -(*pcVar2 == -1);
      auVar13[1] = -(pcVar2[1] == -1);
      auVar13[2] = -(pcVar2[2] == -1);
      auVar13[3] = -(pcVar2[3] == -1);
      auVar13[4] = -(pcVar2[4] == -1);
      auVar13[5] = -(pcVar2[5] == -1);
      auVar13[6] = -(pcVar2[6] == -1);
      auVar13[7] = -(pcVar2[7] == -1);
      auVar13[8] = -(pcVar2[8] == -1);
      auVar13[9] = -(pcVar2[9] == -1);
      auVar13[10] = -(pcVar2[10] == -1);
      auVar13[0xb] = -(pcVar2[0xb] == -1);
      auVar13[0xc] = -(pcVar2[0xc] == -1);
      auVar13[0xd] = -(pcVar2[0xd] == -1);
      auVar13[0xe] = -(pcVar2[0xe] == -1);
      auVar13[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar13 >> 7,0) & 1) != 0 || (SUB161(auVar13 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar13 >> 0x17,0) & 1) != 0) || (SUB161(auVar13 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar13 >> 0x27,0) & 1) != 0) || (SUB161(auVar13 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar13 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar13 >> 0x3f,0) & 1) != 0) || (SUB161(auVar13 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar13 >> 0x4f,0) & 1) != 0) || (SUB161(auVar13 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar13 >> 0x5f,0) & 1) != 0) || (SUB161(auVar13 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar13 >> 0x6f,0) & 1) != 0) || (SUB161(auVar13 >> 0x77,0) & 1) != 0) ||
          auVar13[0xf] < '\0') break;
      lVar8 = uVar7 + lVar9;
      lVar9 = lVar9 + 0x10;
      uVar7 = lVar8 + 0x10U & uVar5;
    }
  }
  puVar1 = (uint *)(local_140 + 0x90);
  uVar10 = *(uint *)(local_140 + 0x90);
  if (uVar10 < 0x3ffffffe) {
    LOCK();
    uVar3 = *puVar1;
    if (uVar10 == uVar3) {
      *puVar1 = uVar10 + 1;
    }
    UNLOCK();
    if (uVar10 == uVar3) goto LAB_14086a8e7;
  }
  local_41 = 1;
  std::sys::sync::rwlock::futex::RwLock::read_contended();
LAB_14086a8e7:
  if (*(longlong *)(local_140 + 0xd0) != 0) {
    lVar4 = *(longlong *)(local_140 + 0xb8);
    uVar5 = *(ulonglong *)(local_140 + 0xc0);
    uVar7 = uVar5 & 0x3bad471c3be19d00;
    lVar9 = 0;
    do {
      pcVar2 = (char *)(lVar4 + uVar7);
      auVar16[0] = -(*pcVar2 == '\x1d');
      auVar16[1] = -(pcVar2[1] == '\x1d');
      auVar16[2] = -(pcVar2[2] == '\x1d');
      auVar16[3] = -(pcVar2[3] == '\x1d');
      auVar16[4] = -(pcVar2[4] == '\x1d');
      auVar16[5] = -(pcVar2[5] == '\x1d');
      auVar16[6] = -(pcVar2[6] == '\x1d');
      auVar16[7] = -(pcVar2[7] == '\x1d');
      auVar16[8] = -(pcVar2[8] == '\x1d');
      auVar16[9] = -(pcVar2[9] == '\x1d');
      auVar16[10] = -(pcVar2[10] == '\x1d');
      auVar16[0xb] = -(pcVar2[0xb] == '\x1d');
      auVar16[0xc] = -(pcVar2[0xc] == '\x1d');
      auVar16[0xd] = -(pcVar2[0xd] == '\x1d');
      auVar16[0xe] = -(pcVar2[0xe] == '\x1d');
      auVar16[0xf] = -(pcVar2[0xf] == '\x1d');
      uVar11 = (ushort)(SUB161(auVar16 >> 7,0) & 1) | (ushort)(SUB161(auVar16 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar16 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar16 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar16 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar16 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar16 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar16 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar16 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar16 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar16 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar16 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar16 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar16 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar16 >> 0x77,0) & 1) << 0xe | (ushort)(auVar16[0xf] >> 7) << 0xf;
      uVar10 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar3 = 0;
        for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar3 = uVar3 + 1;
        }
        lVar8 = (uVar3 + uVar7 & uVar5) * -0x88;
        if ((*(longlong *)(lVar4 + -0x88 + lVar8) == 0x775c6361f3425a3a) &&
           (*(longlong *)(lVar4 + -0x80 + lVar8) == 0x3bad471c3be19d00)) {
          LOCK();
          uVar10 = *puVar1;
          *puVar1 = *puVar1 - 1;
          UNLOCK();
          if (SBORROW4(0,uVar10 - 1 & 0xbfffffff)) {
            local_41 = 1;
            std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
          }
LAB_14086a9bc:
          if ((ulonglong)local_148 < *(ulonglong *)(local_140 + 0x1b0)) {
            local_41 = 1;
            bevy_ecs::storage::table::Table::get_component();
            if (extraout_RAX == 0) goto LAB_14086abe1;
            goto LAB_14086a9f2;
          }
          goto LAB_14086abe1;
        }
        uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
      }
      auVar14[0] = -(*pcVar2 == -1);
      auVar14[1] = -(pcVar2[1] == -1);
      auVar14[2] = -(pcVar2[2] == -1);
      auVar14[3] = -(pcVar2[3] == -1);
      auVar14[4] = -(pcVar2[4] == -1);
      auVar14[5] = -(pcVar2[5] == -1);
      auVar14[6] = -(pcVar2[6] == -1);
      auVar14[7] = -(pcVar2[7] == -1);
      auVar14[8] = -(pcVar2[8] == -1);
      auVar14[9] = -(pcVar2[9] == -1);
      auVar14[10] = -(pcVar2[10] == -1);
      auVar14[0xb] = -(pcVar2[0xb] == -1);
      auVar14[0xc] = -(pcVar2[0xc] == -1);
      auVar14[0xd] = -(pcVar2[0xd] == -1);
      auVar14[0xe] = -(pcVar2[0xe] == -1);
      auVar14[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar14 >> 7,0) & 1) != 0 || (SUB161(auVar14 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar14 >> 0x17,0) & 1) != 0) || (SUB161(auVar14 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar14 >> 0x27,0) & 1) != 0) || (SUB161(auVar14 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar14 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar14 >> 0x3f,0) & 1) != 0) || (SUB161(auVar14 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar14 >> 0x4f,0) & 1) != 0) || (SUB161(auVar14 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar14 >> 0x5f,0) & 1) != 0) || (SUB161(auVar14 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar14 >> 0x6f,0) & 1) != 0) || (SUB161(auVar14 >> 0x77,0) & 1) != 0) ||
          auVar14[0xf] < '\0') break;
      lVar8 = uVar7 + lVar9;
      lVar9 = lVar9 + 0x10;
      uVar7 = lVar8 + 0x10U & uVar5;
    } while( true );
  }
  LOCK();
  uVar10 = *puVar1;
  *puVar1 = *puVar1 - 1;
  UNLOCK();
  if (SBORROW4(0,uVar10 - 1 & 0xbfffffff)) {
    local_41 = 1;
    std::sys::sync::rwlock::futex::RwLock::wake_writer_or_readers();
  }
LAB_14086abe1:
  local_41 = 1;
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}

