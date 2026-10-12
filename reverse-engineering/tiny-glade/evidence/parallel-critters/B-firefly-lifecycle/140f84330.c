
void __cdecl
bevy_app::app::App::insert_resource<country_core::resources::critters::CritterRng>(void)

{
  char *pcVar1;
  undefined4 uVar2;
  short sVar3;
  uint uVar4;
  uint uVar5;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong in_RCX;
  longlong lVar6;
  ushort uVar7;
  undefined1 uVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  ulonglong uVar13;
  ulonglong uVar14;
  ushort uVar15;
  longlong lVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 local_268 [208];
  longlong local_198;
  longlong *local_190;
  longlong lStack_188;
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
  longlong local_c8 [5];
  undefined8 local_a0;
  undefined8 local_98;
  undefined8 local_90;
  undefined *local_88;
  undefined8 local_80;
  undefined8 local_78;
  undefined2 local_70;
  undefined1 local_6e;
  longlong local_68;
  longlong local_60;
  void *local_58;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar6 = in_RCX + 0x38;
  local_60 = in_RCX + 0x5a8;
  local_68 = lVar6;
  if (*(longlong *)(in_RCX + 0x88) != 0) {
    lVar16 = *(longlong *)(in_RCX + 0x70);
    uVar13 = *(ulonglong *)(in_RCX + 0x78);
    uVar10 = uVar13 & 0x39a4850b05a38e02;
    lVar12 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar16 + uVar10);
      auVar21[0] = -(*pcVar1 == __xmm_1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c);
      auVar21[1] = -(pcVar1[1] == UNK_1429504c1);
      auVar21[2] = -(pcVar1[2] == UNK_1429504c2);
      auVar21[3] = -(pcVar1[3] == UNK_1429504c3);
      auVar21[4] = -(pcVar1[4] == UNK_1429504c4);
      auVar21[5] = -(pcVar1[5] == UNK_1429504c5);
      auVar21[6] = -(pcVar1[6] == UNK_1429504c6);
      auVar21[7] = -(pcVar1[7] == UNK_1429504c7);
      auVar21[8] = -(pcVar1[8] == UNK_1429504c8);
      auVar21[9] = -(pcVar1[9] == UNK_1429504c9);
      auVar21[10] = -(pcVar1[10] == UNK_1429504ca);
      auVar21[0xb] = -(pcVar1[0xb] == UNK_1429504cb);
      auVar21[0xc] = -(pcVar1[0xc] == UNK_1429504cc);
      auVar21[0xd] = -(pcVar1[0xd] == UNK_1429504cd);
      auVar21[0xe] = -(pcVar1[0xe] == UNK_1429504ce);
      auVar21[0xf] = -(pcVar1[0xf] == UNK_1429504cf);
      uVar7 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
      uVar9 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar14 = uVar4 + uVar10 & uVar13;
        if ((*(longlong *)(lVar16 + -0x18 + uVar14 * -0x18) == 0x47cdbf1fe95adfd7) &&
           (*(longlong *)(lVar16 + -0x10 + uVar14 * -0x18) == 0x39a4850b05a38e02)) {
          lVar16 = *(longlong *)(lVar16 + uVar14 * -0x18 + -8);
          goto LAB_140f84714;
        }
        uVar7 = (ushort)(uVar9 - 1) & (ushort)uVar9;
        uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar7);
      }
      auVar19[0] = -(*pcVar1 == -1);
      auVar19[1] = -(pcVar1[1] == -1);
      auVar19[2] = -(pcVar1[2] == -1);
      auVar19[3] = -(pcVar1[3] == -1);
      auVar19[4] = -(pcVar1[4] == -1);
      auVar19[5] = -(pcVar1[5] == -1);
      auVar19[6] = -(pcVar1[6] == -1);
      auVar19[7] = -(pcVar1[7] == -1);
      auVar19[8] = -(pcVar1[8] == -1);
      auVar19[9] = -(pcVar1[9] == -1);
      auVar19[10] = -(pcVar1[10] == -1);
      auVar19[0xb] = -(pcVar1[0xb] == -1);
      auVar19[0xc] = -(pcVar1[0xc] == -1);
      auVar19[0xd] = -(pcVar1[0xd] == -1);
      auVar19[0xe] = -(pcVar1[0xe] == -1);
      auVar19[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      lVar11 = uVar10 + lVar12;
      lVar12 = lVar12 + 0x10;
      uVar10 = lVar11 + 0x10U & uVar13;
    }
  }
  lVar16 = *(longlong *)(in_RCX + 0xd8);
  uVar13 = *(ulonglong *)(in_RCX + 0xe0);
  uVar10 = uVar13 & 0x39a4850b05a38e02;
  lVar12 = 0;
  while( true ) {
    pcVar1 = (char *)(lVar16 + uVar10);
    auVar22[0] = -(*pcVar1 == __xmm_1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c1c);
    auVar22[1] = -(pcVar1[1] == UNK_1429504c1);
    auVar22[2] = -(pcVar1[2] == UNK_1429504c2);
    auVar22[3] = -(pcVar1[3] == UNK_1429504c3);
    auVar22[4] = -(pcVar1[4] == UNK_1429504c4);
    auVar22[5] = -(pcVar1[5] == UNK_1429504c5);
    auVar22[6] = -(pcVar1[6] == UNK_1429504c6);
    auVar22[7] = -(pcVar1[7] == UNK_1429504c7);
    auVar22[8] = -(pcVar1[8] == UNK_1429504c8);
    auVar22[9] = -(pcVar1[9] == UNK_1429504c9);
    auVar22[10] = -(pcVar1[10] == UNK_1429504ca);
    auVar22[0xb] = -(pcVar1[0xb] == UNK_1429504cb);
    auVar22[0xc] = -(pcVar1[0xc] == UNK_1429504cc);
    auVar22[0xd] = -(pcVar1[0xd] == UNK_1429504cd);
    auVar22[0xe] = -(pcVar1[0xe] == UNK_1429504ce);
    auVar22[0xf] = -(pcVar1[0xf] == UNK_1429504cf);
    uVar7 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar22 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar22 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar22 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar22 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar22 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar22 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar22 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar22 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar22 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar22 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar22 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(auVar22[0xf] >> 7) << 0xf;
    uVar9 = (uint)uVar7;
    if (uVar7 != 0) break;
LAB_140f8446c:
    auVar20[0] = -(*pcVar1 == -1);
    auVar20[1] = -(pcVar1[1] == -1);
    auVar20[2] = -(pcVar1[2] == -1);
    auVar20[3] = -(pcVar1[3] == -1);
    auVar20[4] = -(pcVar1[4] == -1);
    auVar20[5] = -(pcVar1[5] == -1);
    auVar20[6] = -(pcVar1[6] == -1);
    auVar20[7] = -(pcVar1[7] == -1);
    auVar20[8] = -(pcVar1[8] == -1);
    auVar20[9] = -(pcVar1[9] == -1);
    auVar20[10] = -(pcVar1[10] == -1);
    auVar20[0xb] = -(pcVar1[0xb] == -1);
    auVar20[0xc] = -(pcVar1[0xc] == -1);
    auVar20[0xd] = -(pcVar1[0xd] == -1);
    auVar20[0xe] = -(pcVar1[0xe] == -1);
    auVar20[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar20 >> 0x37,0) & 1) != 0) || (SUB161(auVar20 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar20 >> 0x47,0) & 1) != 0) || (SUB161(auVar20 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar20 >> 0x57,0) & 1) != 0) || (SUB161(auVar20 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar20 >> 0x67,0) & 1) != 0) || (SUB161(auVar20 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar20 >> 0x77,0) & 1) != 0) || auVar20[0xf] < '\0') {
LAB_140f8456d:
      lVar16 = *(longlong *)(in_RCX + 0x5a8);
      *(ulonglong *)(in_RCX + 0x5a8) = lVar16 + 1U;
      local_90 = 0x8000000000000000;
      local_88 = &anon_c53613bba7dfa2b84e0308c59f8a73e8_43_llvm_8002602149119224474;
      local_80 = 0x2d;
      local_70 = 0x100;
      local_c8[0] = 1;
      local_c8[1] = 0x47cdbf1fe95adfd7;
      local_c8[2] = 0x39a4850b05a38e02;
      local_a0 = 0x10;
      local_98 = 0x10;
      local_78 = 0;
      local_6e = 1;
      local_c8[3] = 0;
      bevy_ecs::component::ComponentInfo::new();
      if (*(ulonglong *)(in_RCX + 0x48) < lVar16 + 1U) {
        local_49 = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar12 = *(longlong *)(in_RCX + 0x40);
      local_58 = (void *)(lVar12 + lVar16 * 0xd0);
      memcpy(&local_198,local_268,0xd0);
      if (*(int *)(lVar12 + lVar16 * 0xd0) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      memcpy(local_58,&local_198,0xd0);
      hashbrown::map::HashMap::
      insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
                ();
LAB_140f84714:
      uVar2 = *(undefined4 *)(in_RCX + 0x5b0);
      bevy_ecs::world::World::flush_components();
      lStack_188 = in_RCX + 0xf8;
      local_190 = local_c8;
      local_198 = lVar6;
      local_c8[0] = lVar16;
      bevy_ecs::storage::sparse_set::SparseSet::
      get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl_2::initialize_with::closure_env_0<true,bevy_ecs::world::impl_2::initialize_resource_internal::closure_env_0>_>
                ();
      if (*(longlong *)(extraout_RAX_00 + 0x20) == 0) {
        if (*(longlong *)(extraout_RAX_00 + 0x18) == 0) {
          bevy_ecs::storage::blob_vec::impl$1::reserve::do_reserve();
          lVar6 = *(longlong *)(extraout_RAX_00 + 0x20);
        }
        else {
          lVar6 = 0;
        }
        *(longlong *)(extraout_RAX_00 + 0x20) = lVar6 + 1;
        memcpy((void *)(lVar6 * *(size_t *)(extraout_RAX_00 + 8) +
                       *(longlong *)(extraout_RAX_00 + 0x10)),local_268,
               *(size_t *)(extraout_RAX_00 + 8));
        *(undefined4 *)(extraout_RAX_00 + 0x58) = uVar2;
      }
      else {
        bevy_ecs::storage::blob_vec::BlobVec::replace_unchecked();
      }
      *(undefined4 *)(extraout_RAX_00 + 0x5c) = uVar2;
      return;
    }
    lVar11 = uVar10 + lVar12;
    lVar12 = lVar12 + 0x10;
    uVar10 = lVar11 + 0x10U & uVar13;
  }
LAB_140f8449d:
  uVar4 = 0;
  for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
    uVar4 = uVar4 + 1;
  }
  uVar14 = uVar4 + uVar10 & uVar13;
  lVar11 = uVar14 * -0x88;
  if ((*(longlong *)(lVar16 + -0x88 + lVar11) != 0x47cdbf1fe95adfd7) ||
     (*(longlong *)(lVar16 + -0x80 + lVar11) != 0x39a4850b05a38e02)) goto LAB_140f84490;
  lVar12 = ((longlong)(uVar14 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
  uVar13 = lVar12 - 0x10U & uVar13;
  pcVar1 = (char *)(lVar16 + uVar13);
  auVar17[0] = -(*pcVar1 == -1);
  auVar17[1] = -(pcVar1[1] == -1);
  auVar17[2] = -(pcVar1[2] == -1);
  auVar17[3] = -(pcVar1[3] == -1);
  auVar17[4] = -(pcVar1[4] == -1);
  auVar17[5] = -(pcVar1[5] == -1);
  auVar17[6] = -(pcVar1[6] == -1);
  auVar17[7] = -(pcVar1[7] == -1);
  auVar17[8] = -(pcVar1[8] == -1);
  auVar17[9] = -(pcVar1[9] == -1);
  auVar17[10] = -(pcVar1[10] == -1);
  auVar17[0xb] = -(pcVar1[0xb] == -1);
  auVar17[0xc] = -(pcVar1[0xc] == -1);
  auVar17[0xd] = -(pcVar1[0xd] == -1);
  auVar17[0xe] = -(pcVar1[0xe] == -1);
  auVar17[0xf] = -(pcVar1[0xf] == -1);
  uVar7 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
          (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
          (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
          (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
          (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
          (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
          (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
          (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
          (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
          (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
          (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
          (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
          (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
          (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(auVar17[0xf] >> 7) << 0xf;
  pcVar1 = (char *)(lVar16 + lVar12);
  auVar18[0] = -(*pcVar1 == -1);
  auVar18[1] = -(pcVar1[1] == -1);
  auVar18[2] = -(pcVar1[2] == -1);
  auVar18[3] = -(pcVar1[3] == -1);
  auVar18[4] = -(pcVar1[4] == -1);
  auVar18[5] = -(pcVar1[5] == -1);
  auVar18[6] = -(pcVar1[6] == -1);
  auVar18[7] = -(pcVar1[7] == -1);
  auVar18[8] = -(pcVar1[8] == -1);
  auVar18[9] = -(pcVar1[9] == -1);
  auVar18[10] = -(pcVar1[10] == -1);
  auVar18[0xb] = -(pcVar1[0xb] == -1);
  auVar18[0xc] = -(pcVar1[0xc] == -1);
  auVar18[0xd] = -(pcVar1[0xd] == -1);
  auVar18[0xe] = -(pcVar1[0xe] == -1);
  auVar18[0xf] = -(pcVar1[0xf] == -1);
  if (uVar7 == 0) {
    uVar15 = 0x10;
  }
  else {
    uVar15 = 0xf;
    if (uVar7 != 0) {
      for (; uVar7 >> uVar15 == 0; uVar15 = uVar15 - 1) {
      }
    }
    uVar15 = uVar15 ^ 0xf;
  }
  lVar11 = lVar11 + lVar16;
  sVar3 = 0;
  for (uVar9 = (ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
                        (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
                        (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
                        (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
                        (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
                        (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
                        (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
                        (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
                        (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
                        (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
                        (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
                        (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
                        (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
                        (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
                        (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar18[0xf] >> 7) << 0xf) | 0x10000; (uVar9 & 1) == 0;
      uVar9 = uVar9 >> 1 | 0x80000000) {
    sVar3 = sVar3 + 1;
  }
  uVar8 = 0x80;
  if ((ushort)(uVar15 + sVar3) < 0x10) {
    *(longlong *)(in_RCX + 0xe8) = *(longlong *)(in_RCX + 0xe8) + 1;
    uVar8 = 0xff;
  }
  *(undefined1 *)(lVar16 + lVar12) = uVar8;
  *(undefined1 *)(lVar16 + 0x10 + uVar13) = uVar8;
  *(longlong *)(in_RCX + 0xf0) = *(longlong *)(in_RCX + 0xf0) + -1;
  if (*(longlong *)(lVar11 + -0x78) != 2) {
    local_130 = *(undefined8 *)(lVar11 + -0x10);
    uStack_128 = *(undefined8 *)(lVar11 + -8);
    local_140 = *(undefined8 *)(lVar11 + -0x20);
    uStack_138 = *(undefined8 *)(lVar11 + -0x18);
    local_150 = *(undefined8 *)(lVar11 + -0x30);
    uStack_148 = *(undefined8 *)(lVar11 + -0x28);
    local_190 = *(longlong **)(lVar11 + -0x70);
    lStack_188 = *(undefined8 *)(lVar11 + -0x68);
    local_180 = *(undefined8 *)(lVar11 + -0x60);
    uStack_178 = *(undefined8 *)(lVar11 + -0x58);
    local_170 = *(undefined8 *)(lVar11 + -0x50);
    uStack_168 = *(undefined8 *)(lVar11 + -0x48);
    local_160 = *(undefined8 *)(lVar11 + -0x40);
    uStack_158 = *(undefined8 *)(lVar11 + -0x38);
    local_198 = *(longlong *)(lVar11 + -0x78);
    bevy_ecs::component::QueuedRegistration::register();
    lVar16 = extraout_RAX;
    goto LAB_140f84714;
  }
  goto LAB_140f8456d;
LAB_140f84490:
  uVar7 = (ushort)(uVar9 - 1) & (ushort)uVar9;
  uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar7);
  if (uVar7 == 0) goto LAB_140f8446c;
  goto LAB_140f8449d;
}

