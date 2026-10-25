
void __cdecl
bevy_app::app::App::insert_resource<country_core::systems::camera_rig::MainCamera>(void)

{
  char *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  short sVar4;
  uint uVar5;
  uint uVar6;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong in_RCX;
  longlong lVar7;
  void *in_RDX;
  ulonglong uVar8;
  ushort uVar9;
  undefined1 uVar10;
  uint uVar11;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  ushort uVar15;
  ulonglong uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  longlong local_688;
  undefined8 local_680;
  undefined8 uStack_678;
  undefined8 local_670;
  undefined8 uStack_668;
  undefined8 local_660;
  undefined8 uStack_658;
  undefined8 local_650;
  undefined8 uStack_648;
  undefined8 local_640;
  undefined8 uStack_638;
  undefined8 local_630;
  undefined8 uStack_628;
  undefined8 local_620;
  undefined8 uStack_618;
  longlong local_1a0;
  ulonglong *local_198;
  longlong local_190;
  ulonglong local_d0 [5];
  undefined8 local_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined *local_90;
  undefined8 local_88;
  code *local_80;
  undefined2 local_78;
  undefined1 local_76;
  longlong local_70;
  ulonglong *local_68;
  void *local_60;
  void *local_58;
  undefined1 local_4a;
  undefined1 local_49;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  lVar7 = in_RCX + 0x38;
  local_68 = (ulonglong *)(in_RCX + 0x5a8);
  local_70 = lVar7;
  if (*(longlong *)(in_RCX + 0x88) != 0) {
    lVar3 = *(longlong *)(in_RCX + 0x70);
    uVar8 = *(ulonglong *)(in_RCX + 0x78);
    uVar12 = uVar8 & 0x8d008d15febcf29d;
    lVar14 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar3 + uVar12);
      auVar21[0] = -(*pcVar1 == 'F');
      auVar21[1] = -(pcVar1[1] == 'F');
      auVar21[2] = -(pcVar1[2] == 'F');
      auVar21[3] = -(pcVar1[3] == 'F');
      auVar21[4] = -(pcVar1[4] == 'F');
      auVar21[5] = -(pcVar1[5] == 'F');
      auVar21[6] = -(pcVar1[6] == 'F');
      auVar21[7] = -(pcVar1[7] == 'F');
      auVar21[8] = -(pcVar1[8] == 'F');
      auVar21[9] = -(pcVar1[9] == 'F');
      auVar21[10] = -(pcVar1[10] == 'F');
      auVar21[0xb] = -(pcVar1[0xb] == 'F');
      auVar21[0xc] = -(pcVar1[0xc] == 'F');
      auVar21[0xd] = -(pcVar1[0xd] == 'F');
      auVar21[0xe] = -(pcVar1[0xe] == 'F');
      auVar21[0xf] = -(pcVar1[0xf] == 'F');
      uVar9 = (ushort)(SUB161(auVar21 >> 7,0) & 1) | (ushort)(SUB161(auVar21 >> 0xf,0) & 1) << 1 |
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
      uVar11 = (uint)uVar9;
      while (uVar9 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar16 = uVar5 + uVar12 & uVar8;
        if ((*(longlong *)(lVar3 + -0x18 + uVar16 * -0x18) == -0x445bae1195f75c01) &&
           (*(longlong *)(lVar3 + -0x10 + uVar16 * -0x18) == -0x72ff72ea01430d63)) {
          uVar8 = *(ulonglong *)(lVar3 + uVar16 * -0x18 + -8);
          goto LAB_140133901;
        }
        uVar9 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar9);
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
      lVar13 = uVar12 + lVar14;
      lVar14 = lVar14 + 0x10;
      uVar12 = lVar13 + 0x10U & uVar8;
    }
  }
  lVar3 = *(longlong *)(in_RCX + 0xd8);
  uVar8 = *(ulonglong *)(in_RCX + 0xe0);
  uVar12 = uVar8 & 0x8d008d15febcf29d;
  lVar14 = 0;
  while( true ) {
    pcVar1 = (char *)(lVar3 + uVar12);
    auVar22[0] = -(*pcVar1 == 'F');
    auVar22[1] = -(pcVar1[1] == 'F');
    auVar22[2] = -(pcVar1[2] == 'F');
    auVar22[3] = -(pcVar1[3] == 'F');
    auVar22[4] = -(pcVar1[4] == 'F');
    auVar22[5] = -(pcVar1[5] == 'F');
    auVar22[6] = -(pcVar1[6] == 'F');
    auVar22[7] = -(pcVar1[7] == 'F');
    auVar22[8] = -(pcVar1[8] == 'F');
    auVar22[9] = -(pcVar1[9] == 'F');
    auVar22[10] = -(pcVar1[10] == 'F');
    auVar22[0xb] = -(pcVar1[0xb] == 'F');
    auVar22[0xc] = -(pcVar1[0xc] == 'F');
    auVar22[0xd] = -(pcVar1[0xd] == 'F');
    auVar22[0xe] = -(pcVar1[0xe] == 'F');
    auVar22[0xf] = -(pcVar1[0xf] == 'F');
    uVar9 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
    uVar11 = (uint)uVar9;
    local_60 = in_RDX;
    if (uVar9 != 0) break;
LAB_14013364c:
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
LAB_14013374d:
      uVar8 = *local_68;
      *local_68 = uVar8 + 1;
      local_98 = 0x8000000000000000;
      local_90 = &anon_893984a47b10772a2f3aa373d073e074_273_llvm_8286655925340897655;
      local_88 = 0x2d;
      local_78 = 0x100;
      local_d0[0] = 1;
      local_d0[1] = 0xbba451ee6a08a3ff;
      local_d0[2] = 0x8d008d15febcf29d;
      local_a8 = 0x10;
      local_a0 = 0x4e0;
      local_80 = bevy_ecs::component::ComponentDescriptor::
                 drop_ptr<country_core::systems::camera_rig::MainCamera>;
      local_76 = 1;
      local_d0[3] = 0;
      local_49 = 1;
      bevy_ecs::component::ComponentInfo::new();
      if (*(ulonglong *)(in_RCX + 0x48) < uVar8 + 1) {
        local_4a = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar3 = *(longlong *)(in_RCX + 0x40);
      local_58 = (void *)(lVar3 + uVar8 * 0xd0);
      memcpy(&local_688,&local_1a0,0xd0);
      if (*(int *)(lVar3 + uVar8 * 0xd0) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      memcpy(local_58,&local_688,0xd0);
      local_49 = 1;
      hashbrown::map::HashMap::
      insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
                ();
      in_RDX = local_60;
LAB_140133901:
      memcpy(&local_688,in_RDX,0x4e0);
      uVar2 = *(undefined4 *)(in_RCX + 0x5b0);
      local_49 = 0;
      bevy_ecs::world::World::flush_components();
      local_190 = in_RCX + 0xf8;
      local_198 = local_d0;
      local_49 = 0;
      local_1a0 = lVar7;
      local_d0[0] = uVar8;
      bevy_ecs::storage::sparse_set::SparseSet::
      get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl_2::initialize_with::closure_env_0<true,bevy_ecs::world::impl_2::initialize_resource_internal::closure_env_0>_>
                ();
      if (*(longlong *)(extraout_RAX_00 + 0x20) == 0) {
        if (*(longlong *)(extraout_RAX_00 + 0x18) == 0) {
          local_49 = 0;
          bevy_ecs::storage::blob_vec::impl$1::reserve::do_reserve();
          lVar7 = *(longlong *)(extraout_RAX_00 + 0x20);
        }
        else {
          lVar7 = 0;
        }
        *(longlong *)(extraout_RAX_00 + 0x20) = lVar7 + 1;
        memcpy((void *)(lVar7 * *(size_t *)(extraout_RAX_00 + 8) +
                       *(longlong *)(extraout_RAX_00 + 0x10)),&local_688,
               *(size_t *)(extraout_RAX_00 + 8));
        *(undefined4 *)(extraout_RAX_00 + 0x58) = uVar2;
      }
      else {
        local_49 = 0;
        bevy_ecs::storage::blob_vec::BlobVec::replace_unchecked();
      }
      *(undefined4 *)(extraout_RAX_00 + 0x5c) = uVar2;
      return;
    }
    lVar13 = uVar12 + lVar14;
    lVar14 = lVar14 + 0x10;
    uVar12 = lVar13 + 0x10U & uVar8;
  }
LAB_14013367d:
  uVar5 = 0;
  for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
    uVar5 = uVar5 + 1;
  }
  uVar16 = uVar5 + uVar12 & uVar8;
  lVar13 = uVar16 * -0x88;
  if ((*(longlong *)(lVar3 + -0x88 + lVar13) != -0x445bae1195f75c01) ||
     (*(longlong *)(lVar3 + -0x80 + lVar13) != -0x72ff72ea01430d63)) goto LAB_140133670;
  lVar14 = ((longlong)(uVar16 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
  uVar8 = lVar14 - 0x10U & uVar8;
  pcVar1 = (char *)(lVar3 + uVar8);
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
  uVar9 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
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
  pcVar1 = (char *)(lVar3 + lVar14);
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
  if (uVar9 == 0) {
    uVar15 = 0x10;
  }
  else {
    uVar15 = 0xf;
    if (uVar9 != 0) {
      for (; uVar9 >> uVar15 == 0; uVar15 = uVar15 - 1) {
      }
    }
    uVar15 = uVar15 ^ 0xf;
  }
  lVar13 = lVar13 + lVar3;
  sVar4 = 0;
  for (uVar11 = (ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
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
                        (ushort)(auVar18[0xf] >> 7) << 0xf) | 0x10000; (uVar11 & 1) == 0;
      uVar11 = uVar11 >> 1 | 0x80000000) {
    sVar4 = sVar4 + 1;
  }
  uVar10 = 0x80;
  if ((ushort)(uVar15 + sVar4) < 0x10) {
    *(longlong *)(in_RCX + 0xe8) = *(longlong *)(in_RCX + 0xe8) + 1;
    uVar10 = 0xff;
  }
  *(undefined1 *)(lVar3 + lVar14) = uVar10;
  *(undefined1 *)(lVar3 + 0x10 + uVar8) = uVar10;
  *(longlong *)(in_RCX + 0xf0) = *(longlong *)(in_RCX + 0xf0) + -1;
  if (*(longlong *)(lVar13 + -0x78) != 2) {
    local_620 = *(undefined8 *)(lVar13 + -0x10);
    uStack_618 = *(undefined8 *)(lVar13 + -8);
    local_630 = *(undefined8 *)(lVar13 + -0x20);
    uStack_628 = *(undefined8 *)(lVar13 + -0x18);
    local_640 = *(undefined8 *)(lVar13 + -0x30);
    uStack_638 = *(undefined8 *)(lVar13 + -0x28);
    local_680 = *(undefined8 *)(lVar13 + -0x70);
    uStack_678 = *(undefined8 *)(lVar13 + -0x68);
    local_670 = *(undefined8 *)(lVar13 + -0x60);
    uStack_668 = *(undefined8 *)(lVar13 + -0x58);
    local_660 = *(undefined8 *)(lVar13 + -0x50);
    uStack_658 = *(undefined8 *)(lVar13 + -0x48);
    local_650 = *(undefined8 *)(lVar13 + -0x40);
    uStack_648 = *(undefined8 *)(lVar13 + -0x38);
    local_49 = 1;
    local_688 = *(longlong *)(lVar13 + -0x78);
    bevy_ecs::component::QueuedRegistration::register();
    in_RDX = local_60;
    uVar8 = extraout_RAX;
    goto LAB_140133901;
  }
  goto LAB_14013374d;
LAB_140133670:
  uVar9 = (ushort)(uVar11 - 1) & (ushort)uVar11;
  uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar9);
  if (uVar9 == 0) goto LAB_14013364c;
  goto LAB_14013367d;
}

