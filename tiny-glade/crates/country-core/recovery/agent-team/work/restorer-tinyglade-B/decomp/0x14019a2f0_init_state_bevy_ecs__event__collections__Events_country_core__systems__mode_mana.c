// init_state<bevy_ecs::event::collections::Events<country_core::systems::mode_manager::RequestHistoryCommand>_> @ 0x14019a2f0
// sig: void __cdecl init_state<bevy_ecs::event::collections::Events<country_core::systems::mode_manager::RequestHistoryCommand>_>(void)


void __cdecl
bevy_ecs::system::system_param::impl$9::
init_state<bevy_ecs::event::collections::Events<country_core::systems::mode_manager::RequestHistoryCommand>_>
          (void)

{
  undefined *puVar1;
  char *pcVar2;
  ulonglong *puVar3;
  longlong lVar4;
  short sVar5;
  uint uVar6;
  uint uVar7;
  undefined **extraout_RAX;
  longlong extraout_RAX_00;
  longlong in_RCX;
  longlong in_RDX;
  ushort uVar8;
  undefined1 uVar9;
  uint uVar10;
  ulonglong uVar11;
  longlong lVar12;
  longlong lVar13;
  ulonglong uVar14;
  ushort uVar15;
  undefined **ppuVar16;
  ulonglong uVar17;
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined **local_260;
  undefined8 local_258;
  undefined8 **local_250;
  undefined8 local_248;
  longlong *local_240;
  undefined8 local_238;
  undefined *local_190;
  undefined ***local_188;
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
  undefined8 local_110;
  undefined8 local_100;
  undefined8 local_f8;
  undefined1 local_f0;
  undefined **local_c0;
  code *local_b8;
  longlong local_b0;
  code *local_a8;
  undefined8 local_98;
  undefined8 local_90;
  undefined8 local_88;
  undefined *local_80;
  undefined8 local_78;
  code *local_70;
  undefined2 local_68;
  undefined1 local_66;
  undefined *local_60;
  longlong local_58;
  void *local_50;
  undefined1 local_41;
  undefined8 local_40;
  
  local_40 = 0xfffffffffffffffe;
  puVar1 = (undefined *)(in_RCX + 0x38);
  local_58 = in_RCX + 0x5a8;
  local_60 = puVar1;
  if (*(longlong *)(in_RCX + 0x88) != 0) {
    lVar4 = *(longlong *)(in_RCX + 0x70);
    uVar14 = *(ulonglong *)(in_RCX + 0x78);
    uVar11 = uVar14 & 0xf073d18fea2ba51e;
    lVar13 = 0;
    while( true ) {
      pcVar2 = (char *)(lVar4 + uVar11);
      auVar22[0] = -(*pcVar2 == 'x');
      auVar22[1] = -(pcVar2[1] == 'x');
      auVar22[2] = -(pcVar2[2] == 'x');
      auVar22[3] = -(pcVar2[3] == 'x');
      auVar22[4] = -(pcVar2[4] == 'x');
      auVar22[5] = -(pcVar2[5] == 'x');
      auVar22[6] = -(pcVar2[6] == 'x');
      auVar22[7] = -(pcVar2[7] == 'x');
      auVar22[8] = -(pcVar2[8] == 'x');
      auVar22[9] = -(pcVar2[9] == 'x');
      auVar22[10] = -(pcVar2[10] == 'x');
      auVar22[0xb] = -(pcVar2[0xb] == 'x');
      auVar22[0xc] = -(pcVar2[0xc] == 'x');
      auVar22[0xd] = -(pcVar2[0xd] == 'x');
      auVar22[0xe] = -(pcVar2[0xe] == 'x');
      auVar22[0xf] = -(pcVar2[0xf] == 'x');
      uVar8 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
      uVar10 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar6 = 0;
        for (uVar7 = uVar10; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        uVar17 = uVar6 + uVar11 & uVar14;
        if ((*(longlong *)(lVar4 + -0x18 + uVar17 * -0x18) == -0x2e21936580d493b1) &&
           (*(longlong *)(lVar4 + -0x10 + uVar17 * -0x18) == -0xf8c2e7015d45ae2)) {
          ppuVar16 = *(undefined ***)(lVar4 + uVar17 * -0x18 + -8);
          goto LAB_14019a17a;
        }
        uVar8 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar8);
      }
      auVar20[0] = -(*pcVar2 == -1);
      auVar20[1] = -(pcVar2[1] == -1);
      auVar20[2] = -(pcVar2[2] == -1);
      auVar20[3] = -(pcVar2[3] == -1);
      auVar20[4] = -(pcVar2[4] == -1);
      auVar20[5] = -(pcVar2[5] == -1);
      auVar20[6] = -(pcVar2[6] == -1);
      auVar20[7] = -(pcVar2[7] == -1);
      auVar20[8] = -(pcVar2[8] == -1);
      auVar20[9] = -(pcVar2[9] == -1);
      auVar20[10] = -(pcVar2[10] == -1);
      auVar20[0xb] = -(pcVar2[0xb] == -1);
      auVar20[0xc] = -(pcVar2[0xc] == -1);
      auVar20[0xd] = -(pcVar2[0xd] == -1);
      auVar20[0xe] = -(pcVar2[0xe] == -1);
      auVar20[0xf] = -(pcVar2[0xf] == -1);
      if ((((((((((((((((SUB161(auVar20 >> 7,0) & 1) != 0 || (SUB161(auVar20 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar20 >> 0x17,0) & 1) != 0) || (SUB161(auVar20 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar20 >> 0x27,0) & 1) != 0) || (SUB161(auVar20 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar20 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar20 >> 0x3f,0) & 1) != 0) || (SUB161(auVar20 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar20 >> 0x4f,0) & 1) != 0) || (SUB161(auVar20 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar20 >> 0x5f,0) & 1) != 0) || (SUB161(auVar20 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar20 >> 0x6f,0) & 1) != 0) || (SUB161(auVar20 >> 0x77,0) & 1) != 0) ||
          auVar20[0xf] < '\0') break;
      lVar12 = uVar11 + lVar13;
      lVar13 = lVar13 + 0x10;
      uVar11 = lVar12 + 0x10U & uVar14;
    }
  }
  lVar4 = *(longlong *)(in_RCX + 0xd8);
  uVar14 = *(ulonglong *)(in_RCX + 0xe0);
  uVar11 = uVar14 & 0xf073d18fea2ba51e;
  lVar13 = 0;
  while( true ) {
    pcVar2 = (char *)(lVar4 + uVar11);
    auVar23[0] = -(*pcVar2 == 'x');
    auVar23[1] = -(pcVar2[1] == 'x');
    auVar23[2] = -(pcVar2[2] == 'x');
    auVar23[3] = -(pcVar2[3] == 'x');
    auVar23[4] = -(pcVar2[4] == 'x');
    auVar23[5] = -(pcVar2[5] == 'x');
    auVar23[6] = -(pcVar2[6] == 'x');
    auVar23[7] = -(pcVar2[7] == 'x');
    auVar23[8] = -(pcVar2[8] == 'x');
    auVar23[9] = -(pcVar2[9] == 'x');
    auVar23[10] = -(pcVar2[10] == 'x');
    auVar23[0xb] = -(pcVar2[0xb] == 'x');
    auVar23[0xc] = -(pcVar2[0xc] == 'x');
    auVar23[0xd] = -(pcVar2[0xd] == 'x');
    auVar23[0xe] = -(pcVar2[0xe] == 'x');
    auVar23[0xf] = -(pcVar2[0xf] == 'x');
    uVar8 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
    uVar10 = (uint)uVar8;
    if (uVar8 != 0) break;
LAB_140199ecc:
    auVar21[0] = -(*pcVar2 == -1);
    auVar21[1] = -(pcVar2[1] == -1);
    auVar21[2] = -(pcVar2[2] == -1);
    auVar21[3] = -(pcVar2[3] == -1);
    auVar21[4] = -(pcVar2[4] == -1);
    auVar21[5] = -(pcVar2[5] == -1);
    auVar21[6] = -(pcVar2[6] == -1);
    auVar21[7] = -(pcVar2[7] == -1);
    auVar21[8] = -(pcVar2[8] == -1);
    auVar21[9] = -(pcVar2[9] == -1);
    auVar21[10] = -(pcVar2[10] == -1);
    auVar21[0xb] = -(pcVar2[0xb] == -1);
    auVar21[0xc] = -(pcVar2[0xc] == -1);
    auVar21[0xd] = -(pcVar2[0xd] == -1);
    auVar21[0xe] = -(pcVar2[0xe] == -1);
    auVar21[0xf] = -(pcVar2[0xf] == -1);
    if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar21 >> 0x37,0) & 1) != 0) || (SUB161(auVar21 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar21 >> 0x47,0) & 1) != 0) || (SUB161(auVar21 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar21 >> 0x57,0) & 1) != 0) || (SUB161(auVar21 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar21 >> 0x67,0) & 1) != 0) || (SUB161(auVar21 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar21 >> 0x77,0) & 1) != 0) || auVar21[0xf] < '\0') {
LAB_140199fcd:
      ppuVar16 = *(undefined ***)(in_RCX + 0x5a8);
      *(ulonglong *)(in_RCX + 0x5a8) = (longlong)ppuVar16 + 1U;
      local_88 = 0x8000000000000000;
      local_80 = &anon_893984a47b10772a2f3aa373d073e074_278_llvm_8286655925340897655;
      local_78 = 0x60;
      local_68 = 0x100;
      local_c0 = (undefined **)0x1;
      local_b8 = (code *)0xd1de6c9a7f2b6c4f;
      local_b0 = 0xf073d18fea2ba51e;
      local_98 = 8;
      local_90 = 0x48;
      local_70 = component::ComponentDescriptor::
                 drop_ptr<bevy_ecs::event::collections::Events<country_core::resources::history::tree_edit::DeleteTreeCmd>_>
      ;
      local_66 = 1;
      local_a8 = (code *)0x0;
      component::ComponentInfo::new();
      if (*(ulonglong *)(in_RCX + 0x48) < (longlong)ppuVar16 + 1U) {
        local_41 = 1;
        alloc::vec::Vec::
        resize_with<enum2_<core::option::Option<bevy_ecs::component::ComponentInfo>_>,alloc::alloc::Global,bevy_ecs::component::impl_19::register_component_inner::closure_env_0>
                  ();
      }
      lVar4 = *(longlong *)(in_RCX + 0x40);
      local_50 = (void *)(lVar4 + (longlong)ppuVar16 * 0xd0);
      memcpy(&local_190,&local_260,0xd0);
      if (*(int *)(lVar4 + (longlong)ppuVar16 * 0xd0) != 2) {
        core::ptr::drop_in_place<bevy_ecs::component::ComponentInfo>();
      }
      memcpy(local_50,&local_190,0xd0);
      hashbrown::map::HashMap::
      insert<core::any::TypeId,bevy_ecs::component::ComponentId,bevy_platform::hash::NoOpHash,allocator_api2::stable::alloc::global::Global>
                ();
LAB_14019a17a:
      world::World::flush_components();
      lStack_180 = in_RCX + 0xf8;
      local_188 = &local_260;
      local_260 = ppuVar16;
      local_190 = puVar1;
      storage::sparse_set::SparseSet::
      get_or_insert_with<bevy_ecs::component::ComponentId,bevy_ecs::storage::resource::ResourceData<true>,bevy_ecs::storage::resource::impl_2::initialize_with::closure_env_0<true,bevy_ecs::world::impl_2::initialize_resource_internal::closure_env_0>_>
                ();
      if ((*(char *)(in_RDX + 0x93) == '\0') &&
         ((*(undefined ***)(in_RDX + 0x70) <= ppuVar16 ||
          ((*(ulonglong *)(*(longlong *)(in_RDX + 0x60) + ((ulonglong)ppuVar16 >> 6) * 8) >>
            ((ulonglong)ppuVar16 & 0x3f) & 1) == 0)))) {
        uVar14 = *(ulonglong *)(extraout_RAX_00 + 0x48);
        bevy_ecs::query::access::FilteredAccessSet::
        add_unfiltered_resource_read<bevy_ecs::component::ComponentId>();
        if (*(ulonglong *)(in_RDX + 0xf0) < uVar14 + 1) {
          fixedbitset::impl$2::grow::do_grow();
        }
        puVar3 = (ulonglong *)(*(longlong *)(in_RDX + 0xe0) + (uVar14 >> 6) * 8);
        *puVar3 = *puVar3 | 1L << ((byte)uVar14 & 0x3f);
        return;
      }
      local_60 = &DAT_14296370c;
      local_58 = 0x60;
      local_b0 = in_RDX + 0x98;
      local_c0 = &local_60;
      local_b8 = anyhow::context::impl$3::fmt<ref_<str_>,alloc::ffi::c_str::FromVecWithNulError>;
      local_a8 = alloc::string::impl$23::fmt;
      local_190 = (undefined *)0x2;
      lStack_180 = 2;
      uStack_170 = 0;
      local_168 = 0x20;
      uStack_160 = CONCAT71(uStack_160._1_7_,3);
      local_158 = 2;
      local_148 = 2;
      local_138 = 1;
      uStack_130 = 0x20;
      local_128 = CONCAT71(local_128._1_7_,3);
      uStack_120 = 2;
      local_110 = 2;
      local_100 = 0;
      local_f8 = 0x20;
      local_f0 = 3;
      local_260 = &PTR_DAT_1429617f0;
      local_258 = 4;
      local_238 = 3;
      local_250 = &local_c0;
      local_248 = 2;
      local_240 = (longlong *)&local_190;
                    // WARNING: Subroutine does not return
      core::panicking::panic_fmt();
    }
    lVar12 = uVar11 + lVar13;
    lVar13 = lVar13 + 0x10;
    uVar11 = lVar12 + 0x10U & uVar14;
  }
LAB_140199efd:
  uVar6 = 0;
  for (uVar7 = uVar10; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
    uVar6 = uVar6 + 1;
  }
  uVar17 = uVar6 + uVar11 & uVar14;
  lVar12 = uVar17 * -0x88;
  if ((*(longlong *)(lVar4 + -0x88 + lVar12) != -0x2e21936580d493b1) ||
     (*(longlong *)(lVar4 + -0x80 + lVar12) != -0xf8c2e7015d45ae2)) goto LAB_140199ef0;
  lVar13 = ((longlong)(uVar17 * 0x88) >> 3) * -0xf0f0f0f0f0f0f0f;
  uVar14 = lVar13 - 0x10U & uVar14;
  pcVar2 = (char *)(lVar4 + uVar14);
  auVar18[0] = -(*pcVar2 == -1);
  auVar18[1] = -(pcVar2[1] == -1);
  auVar18[2] = -(pcVar2[2] == -1);
  auVar18[3] = -(pcVar2[3] == -1);
  auVar18[4] = -(pcVar2[4] == -1);
  auVar18[5] = -(pcVar2[5] == -1);
  auVar18[6] = -(pcVar2[6] == -1);
  auVar18[7] = -(pcVar2[7] == -1);
  auVar18[8] = -(pcVar2[8] == -1);
  auVar18[9] = -(pcVar2[9] == -1);
  auVar18[10] = -(pcVar2[10] == -1);
  auVar18[0xb] = -(pcVar2[0xb] == -1);
  auVar18[0xc] = -(pcVar2[0xc] == -1);
  auVar18[0xd] = -(pcVar2[0xd] == -1);
  auVar18[0xe] = -(pcVar2[0xe] == -1);
  auVar18[0xf] = -(pcVar2[0xf] == -1);
  uVar8 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
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
          (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
  pcVar2 = (char *)(lVar4 + lVar13);
  auVar19[0] = -(*pcVar2 == -1);
  auVar19[1] = -(pcVar2[1] == -1);
  auVar19[2] = -(pcVar2[2] == -1);
  auVar19[3] = -(pcVar2[3] == -1);
  auVar19[4] = -(pcVar2[4] == -1);
  auVar19[5] = -(pcVar2[5] == -1);
  auVar19[6] = -(pcVar2[6] == -1);
  auVar19[7] = -(pcVar2[7] == -1);
  auVar19[8] = -(pcVar2[8] == -1);
  auVar19[9] = -(pcVar2[9] == -1);
  auVar19[10] = -(pcVar2[10] == -1);
  auVar19[0xb] = -(pcVar2[0xb] == -1);
  auVar19[0xc] = -(pcVar2[0xc] == -1);
  auVar19[0xd] = -(pcVar2[0xd] == -1);
  auVar19[0xe] = -(pcVar2[0xe] == -1);
  auVar19[0xf] = -(pcVar2[0xf] == -1);
  if (uVar8 == 0) {
    uVar15 = 0x10;
  }
  else {
    uVar15 = 0xf;
    if (uVar8 != 0) {
      for (; uVar8 >> uVar15 == 0; uVar15 = uVar15 - 1) {
      }
    }
    uVar15 = uVar15 ^ 0xf;
  }
  lVar12 = lVar12 + lVar4;
  sVar5 = 0;
  for (uVar10 = (ushort)((ushort)(SUB161(auVar19 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
                         (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
                         (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
                         (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
                         (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
                         (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
                         (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
                         (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
                         (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
                         (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
                         (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
                         (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
                         (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe |
                        (ushort)(auVar19[0xf] >> 7) << 0xf) | 0x10000; (uVar10 & 1) == 0;
      uVar10 = uVar10 >> 1 | 0x80000000) {
    sVar5 = sVar5 + 1;
  }
  uVar9 = 0x80;
  if ((ushort)(uVar15 + sVar5) < 0x10) {
    *(longlong *)(in_RCX + 0xe8) = *(longlong *)(in_RCX + 0xe8) + 1;
    uVar9 = 0xff;
  }
  *(undefined1 *)(lVar4 + lVar13) = uVar9;
  *(undefined1 *)(lVar4 + 0x10 + uVar14) = uVar9;
  *(longlong *)(in_RCX + 0xf0) = *(longlong *)(in_RCX + 0xf0) + -1;
  if (*(undefined **)(lVar12 + -0x78) != (undefined *)0x2) {
    local_128 = *(undefined8 *)(lVar12 + -0x10);
    uStack_120 = *(undefined8 *)(lVar12 + -8);
    local_138 = *(undefined8 *)(lVar12 + -0x20);
    uStack_130 = *(undefined8 *)(lVar12 + -0x18);
    local_148 = *(undefined8 *)(lVar12 + -0x30);
    uStack_140 = *(undefined8 *)(lVar12 + -0x28);
    local_188 = *(undefined ****)(lVar12 + -0x70);
    lStack_180 = *(undefined8 *)(lVar12 + -0x68);
    local_178 = *(undefined8 *)(lVar12 + -0x60);
    uStack_170 = *(undefined8 *)(lVar12 + -0x58);
    local_168 = *(undefined8 *)(lVar12 + -0x50);
    uStack_160 = *(undefined8 *)(lVar12 + -0x48);
    local_158 = *(undefined8 *)(lVar12 + -0x40);
    uStack_150 = *(undefined8 *)(lVar12 + -0x38);
    local_190 = *(undefined **)(lVar12 + -0x78);
    component::QueuedRegistration::register();
    ppuVar16 = extraout_RAX;
    goto LAB_14019a17a;
  }
  goto LAB_140199fcd;
LAB_140199ef0:
  uVar8 = (ushort)(uVar10 - 1) & (ushort)uVar10;
  uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar8);
  if (uVar8 == 0) goto LAB_140199ecc;
  goto LAB_140199efd;
}

