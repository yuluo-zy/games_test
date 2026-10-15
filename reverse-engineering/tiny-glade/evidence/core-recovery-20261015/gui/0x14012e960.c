
void __cdecl
bevy_app::app::App::insert_resource<country_core::resources::gui::TimeOfDayScreen>(void)

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
  undefined8 *in_RDX;
  ushort uVar7;
  undefined1 uVar8;
  uint uVar9;
  ulonglong uVar10;
  longlong lVar11;
  longlong lVar12;
  ulonglong uVar13;
  ushort uVar14;
  longlong lVar15;
  ulonglong uVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined8 local_268;
  undefined8 uStack_260;
  longlong local_190;
  longlong *local_188;
  longlong lStack_180;
  undefined8 local_178;
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
  longlong local_c0 [5];
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined *local_80;
  undefined8 local_78;
  undefined8 local_70;
  undefined2 local_68;
  undefined1 local_66;
  longlong local_60;
  longlong local_58;
  void *local_50;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  lVar6 = in_RCX + 0x38;
  local_58 = in_RCX + 0x5a8;
  local_60 = lVar6;
  if (*(longlong *)(in_RCX + 0x88) != 0) {
    lVar15 = *(longlong *)(in_RCX + 0x70);
    uVar13 = *(ulonglong *)(in_RCX + 0x78);
    uVar10 = uVar13 & 0x9fea1ff84878cb90;
    lVar12 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar15 + uVar10);
      auVar21[0] = -(*pcVar1 == 'O');
      auVar21[1] = -(pcVar1[1] == 'O');
      auVar21[2] = -(pcVar1[2] == 'O');
      auVar21[3] = -(pcVar1[3] == 'O');
      auVar21[4] = -(pcVar1[4] == 'O');
      auVar21[5] = -(pcVar1[5] == 'O');
      auVar21[6] = -(pcVar1[6] == 'O');
      auVar21[7] = -(pcVar1[7] == 'O');
      auVar21[8] = -(pcVar1[8] == 'O');
      auVar21[9] = -(pcVar1[9] == 'O');
      auVar21[10] = -(pcVar1[10] == 'O');
      auVar21[0xb] = -(pcVar1[0xb] == 'O');
      auVar21[0xc] = -(pcVar1[0xc] == 'O');
      auVar21[0xd] = -(pcVar1[0xd] == 'O');
      auVar21[0xe] = -(pcVar1[0xe] == 'O');
      auVar21[0xf] = -(pcVar1[0xf] == 'O');
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
        uVar16 = uVar4 + uVar10 & uVar13;
        if ((*(longlong *)(lVar15 + -0x18 + uVar16 * -0x18) == -0x4fcfa754035525d4) &&
           (*(longlong *)(lVar15 + -0x10 + uVar16 * -0x18) == -0x6015e007b7873470)) {
          lVar15 = *(longlong *)(lVar15 + uVar16 * -0x18 + -8);
          goto LAB_14012ed47;
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
  lVar15 = *(longlong *)(in_RCX + 0xd8);
  uVar13 = *(ulonglong *)(in_RCX + 0xe0);
  uVar10 = uVar13 & 0x9fea1ff84878cb90;
  lVar12 = 0;
  while( true ) {
    pcVar1 = (char *)(lVar15 + uVar10);
    auVar22[0] = -(*pcVar1 == 'O');
    auVar22[1] = -(pcVar1[1] == 'O');
    auVar22[2] = -(pcVar1[2] == 'O');
    auVar22[3] = -(pcVar1[3] == 'O');
    auVar22[4] = -(pcVar1[4] == 'O');
    auVar22[5] = -(pcVar1[5] == 'O');
    auVar22[6] = -(pcVar1[6] == 'O');
    auVar22[7] = -(pcVar1[7] == 'O');
    auVar22[8] = -(pcVar1[8] == 'O');
    auVar22[9] = -(pcVar1[9] == 'O');
    auVar22[10] = -(pcVar1[10] == 'O');
    auVar22[0xb] = -(pcVar1[0xb] == 'O');
    auVar22[0xc] = -(pcVar1[0xc] == 'O');
    auVar22[0xd] = -(pcVar1[0xd] == 'O');
    auVar22[0xe] = -(pcVar1[0xe] == 'O');
    auVar22[0xf] = -(pcVar1[0xf] == 'O');
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
LAB_14012ea9c:
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
LAB_14012eb9d:
      lVar15 = *(longlong *)(in_RCX + 0x5a8);
      *(ulonglong *)(in_RCX + 0x5a8) = lVar15 + 1U;
      local_88 = 0x8000000000000000;
      local_80 = &anon_893984a47b10772a2f3aa373d073e074_301_llvm_8286655925340897655;
      local_78 = 0x2d;
      local_68 = 0x100;
      local_c0[0] = 1;
      local_c0[1] = 0xb03058abfcaada2c;
      local_c0[2] = 0x9fea1ff84878cb90;
      local_98 = 4;
      local_90 = 0x10;
      local_70 = 0;
      local_66 = 1;
      local_c0[3] = 0;
      bevy_ecs::component::ComponentInfo::new();
      if (*(ulonglong *)(in_RCX + 0x48) < lVar15 + 1U) {
        local_41 = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar12 = *(longlong *)(in_RCX + 0x40);
      local_50 = (void *)(lVar12 + lVar15 * 0xd0);
      memcpy(&local_190,&local_268,0xd0);
      if (*(int *)(lVar12 + lVar15 * 0xd0) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      memcpy(local_50,&local_190,0xd0);
      hashbrown::map::HashMap::
      insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
                ();
LAB_14012ed47:
      local_268 = *in_RDX;
      uStack_260 = in_RDX[1];
      uVar2 = *(undefined4 *)(in_RCX + 0x5b0);
      bevy_ecs::world::World::flush_components();
      lStack_180 = in_RCX + 0xf8;
      local_188 = local_c0;
      local_190 = lVar6;
      local_c0[0] = lVar15;
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
                       *(longlong *)(extraout_RAX_00 + 0x10)),&local_268,
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
LAB_14012eacd:
  uVar4 = 0;
  for (uVar5 = uVar9; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
    uVar4 = uVar4 + 1;
  }
  uVar16 = uVar4 + uVar10 & uVar13;
  lVar11 = uVar16 * -0x88;
  if ((*(longlong *)(lVar15 + -0x88 + lVar11) != -0x4fcfa754035525d4) ||
     (*(longlong *)(lVar15 + -0x80 + lVar11) != -0x6015e007b7873470)) goto LAB_14012eac0;
  lVar12 = ((longlong)(uVar16 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
  uVar13 = lVar12 - 0x10U & uVar13;
  pcVar1 = (char *)(lVar15 + uVar13);
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
  pcVar1 = (char *)(lVar15 + lVar12);
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
    uVar14 = 0x10;
  }
  else {
    uVar14 = 0xf;
    if (uVar7 != 0) {
      for (; uVar7 >> uVar14 == 0; uVar14 = uVar14 - 1) {
      }
    }
    uVar14 = uVar14 ^ 0xf;
  }
  lVar11 = lVar11 + lVar15;
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
  if ((ushort)(uVar14 + sVar3) < 0x10) {
    *(longlong *)(in_RCX + 0xe8) = *(longlong *)(in_RCX + 0xe8) + 1;
    uVar8 = 0xff;
  }
  *(undefined1 *)(lVar15 + lVar12) = uVar8;
  *(undefined1 *)(lVar15 + 0x10 + uVar13) = uVar8;
  *(longlong *)(in_RCX + 0xf0) = *(longlong *)(in_RCX + 0xf0) + -1;
  if (*(longlong *)(lVar11 + -0x78) != 2) {
    local_128 = *(undefined8 *)(lVar11 + -0x10);
    uStack_120 = *(undefined8 *)(lVar11 + -8);
    local_138 = *(undefined8 *)(lVar11 + -0x20);
    uStack_130 = *(undefined8 *)(lVar11 + -0x18);
    local_148 = *(undefined8 *)(lVar11 + -0x30);
    uStack_140 = *(undefined8 *)(lVar11 + -0x28);
    local_188 = *(longlong **)(lVar11 + -0x70);
    lStack_180 = *(undefined8 *)(lVar11 + -0x68);
    local_178 = *(undefined8 *)(lVar11 + -0x60);
    uStack_170 = *(undefined8 *)(lVar11 + -0x58);
    local_168 = *(undefined8 *)(lVar11 + -0x50);
    uStack_160 = *(undefined8 *)(lVar11 + -0x48);
    local_158 = *(undefined8 *)(lVar11 + -0x40);
    uStack_150 = *(undefined8 *)(lVar11 + -0x38);
    local_190 = *(longlong *)(lVar11 + -0x78);
    bevy_ecs::component::QueuedRegistration::register();
    lVar15 = extraout_RAX;
    goto LAB_14012ed47;
  }
  goto LAB_14012eb9d;
LAB_14012eac0:
  uVar7 = (ushort)(uVar9 - 1) & (ushort)uVar9;
  uVar9 = CONCAT22((short)(uVar9 - 1 >> 0x10),uVar7);
  if (uVar7 == 0) goto LAB_14012ea9c;
  goto LAB_14012eacd;
}


