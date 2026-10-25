
void __cdecl country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts(void)

{
  ulonglong *puVar1;
  char *pcVar2;
  undefined8 uVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  uint uVar7;
  undefined8 uVar8;
  longlong *plVar9;
  char extraout_AL;
  byte *extraout_RAX;
  longlong extraout_RAX_00;
  byte *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong uVar10;
  ulonglong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  longlong extraout_RAX_05;
  ulonglong extraout_RAX_06;
  ulonglong extraout_RAX_07;
  longlong extraout_RAX_08;
  ulonglong extraout_RAX_09;
  undefined8 *in_RCX;
  longlong *in_RDX;
  undefined4 *puVar11;
  ulonglong uVar12;
  undefined1 (*pauVar13) [16];
  undefined1 (*pauVar14) [16];
  uint uVar15;
  uint uVar16;
  longlong *in_R8;
  longlong in_R9;
  longlong lVar17;
  uint uVar18;
  uint uVar19;
  ushort uVar20;
  longlong lVar21;
  uint *puVar22;
  longlong lVar23;
  uint *puVar24;
  bool bVar25;
  byte bVar26;
  char cVar27;
  char cVar29;
  char cVar30;
  char cVar31;
  undefined1 auVar28 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  undefined1 auVar35 [16];
  undefined1 auVar36 [16];
  undefined1 auVar37 [16];
  longlong *in_stack_00000028;
  longlong *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined1 (*local_248) [16];
  longlong lStack_240;
  undefined8 local_238;
  longlong lStack_230;
  longlong local_228;
  longlong *plStack_220;
  longlong *local_210;
  undefined1 local_208;
  undefined1 (*local_200) [16];
  longlong local_1f8;
  longlong local_1f0;
  undefined4 *local_1e8;
  longlong local_1e0;
  longlong local_1d8;
  undefined4 *local_1d0;
  longlong local_1c8;
  longlong local_1c0;
  longlong local_1b8;
  longlong local_1b0;
  undefined4 *local_1a8;
  longlong local_1a0;
  longlong local_198;
  longlong local_190;
  longlong local_188;
  longlong local_180;
  undefined1 (*local_178) [16];
  undefined1 (*local_170) [16];
  undefined1 (*local_168) [16];
  longlong lStack_160;
  undefined1 (**local_158) [16];
  longlong lStack_150;
  ulonglong local_148;
  longlong *plStack_140;
  undefined4 local_138;
  undefined4 local_134;
  undefined4 *local_130;
  longlong local_128;
  undefined8 local_120;
  undefined8 local_118;
  undefined1 (*pauStack_110) [16];
  longlong *local_108;
  longlong lStack_100;
  ulonglong local_f8;
  undefined1 (*pauStack_f0) [16];
  undefined8 local_e8;
  longlong *plStack_e0;
  undefined8 local_d8;
  longlong local_d0;
  undefined4 local_c8;
  uint local_c4;
  longlong *local_c0;
  undefined4 *local_b8;
  longlong *local_b0;
  undefined8 *local_a8;
  undefined1 (*local_a0) [16];
  longlong local_98;
  longlong *local_90;
  undefined1 local_83;
  undefined1 local_82;
  undefined1 local_81;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  *(undefined4 *)in_R8[2] = *(undefined4 *)((longlong)in_R8 + 0x1c);
  local_98 = *in_R8;
  pauVar13 = *(undefined1 (**) [16])(local_98 + 0x148);
  lVar23 = *(longlong *)(local_98 + 0x150);
  auVar28 = *pauVar13;
  uVar3 = *(undefined8 *)(local_98 + 0x160);
  uVar8 = *(undefined8 *)(local_98 + 0x150);
  plVar9 = *(longlong **)(local_98 + 0x158);
  *(undefined **)(local_98 + 0x148) =
       &anon_6652254e5a159f1dca07c38add8ba93c_0_llvm_3554112267207565025;
  *(undefined8 *)(local_98 + 0x150) = 0;
  *(undefined8 *)(local_98 + 0x158) = 0;
  *(undefined8 *)(local_98 + 0x160) = 0;
  local_b0 = in_RDX;
  local_a8 = in_RCX;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  local_a0 = pauVar13;
  local_90 = (longlong *)lVar23;
  if ((*extraout_RAX & 1) == 0) {
    local_83 = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX[0] = 1;
    extraout_RAX[1] = 0;
    extraout_RAX[2] = 0;
    extraout_RAX[3] = 0;
    extraout_RAX[4] = 0;
    extraout_RAX[5] = 0;
    extraout_RAX[6] = 0;
    extraout_RAX[7] = 0;
    *(longlong *)(extraout_RAX + 8) = extraout_RAX_00;
    *(longlong **)(extraout_RAX + 0x10) = in_RDX;
    local_148 = extraout_RAX_00;
  }
  else {
    local_148 = *(ulonglong *)(extraout_RAX + 8);
    in_RDX = *(longlong **)(extraout_RAX + 0x10);
  }
  local_d0 = local_98 + 0x148;
  pauStack_110 = local_a0 + 1;
  local_108 = (longlong *)(*local_a0 + (longlong)local_90 + 1);
  *(ulonglong *)(extraout_RAX + 8) = local_148 + 1;
  local_158 = (undefined1 (**) [16])0x0;
  lStack_150 = 0;
  local_168 = (undefined1 (*) [16])&DAT_142aedb10;
  lStack_160 = 0;
  lStack_100 = CONCAT62(lStack_100._2_6_,
                        ~((ushort)(SUB161(auVar28 >> 7,0) & 1) |
                          (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                          (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                          (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                          (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                          (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                          (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                          (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                          (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                          (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                          (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                          (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                          (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                          (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                          (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar28[0xf] >> 7) << 0xf));
  plStack_140 = in_RDX;
  local_118 = local_a0;
  local_f8 = uVar3;
  pauStack_f0 = local_a0;
  local_e8 = uVar8;
  plStack_e0 = plVar9;
  local_d8 = uVar3;
  hashbrown::map::impl$82::
  extend<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<std::collections::hash::set::Drain<country_core::resources::stairs::stairs_state::StairGraphEdge>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,std::collections::hash::set::Drain<country_core::resources::stairs::stairs_state::StairGraphEdge>_>_>_>
            ();
  local_248 = local_168;
  lStack_240 = lStack_160;
  local_238 = local_158;
  lStack_230 = lStack_150;
  local_228 = local_148;
  plStack_220 = plStack_140;
  local_130 = (undefined4 *)in_stack_00000028[2];
  local_a0 = (undefined1 (*) [16])
             CONCAT44(local_a0._4_4_,*(undefined4 *)((longlong)in_stack_00000028 + 0x1c));
  *local_130 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  local_90 = (longlong *)*in_stack_00000028;
  *(undefined1 *)(local_90 + 6) = 0;
  local_c8 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  puVar11 = (undefined4 *)in_stack_00000038[2];
  *puVar11 = local_c8;
  local_128 = *in_stack_00000038;
  lVar23 = *(longlong *)(local_128 + 8);
  lVar17 = *(longlong *)(local_128 + 0x10);
  local_1a8 = puVar11;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_01 & 1) == 0) {
    local_81 = 1;
    std::sys::random::hashmap_random_keys();
    extraout_RAX_01[0] = 1;
    extraout_RAX_01[1] = 0;
    extraout_RAX_01[2] = 0;
    extraout_RAX_01[3] = 0;
    extraout_RAX_01[4] = 0;
    extraout_RAX_01[5] = 0;
    extraout_RAX_01[6] = 0;
    extraout_RAX_01[7] = 0;
    *(ulonglong *)(extraout_RAX_01 + 8) = extraout_RAX_02;
    *(undefined4 **)(extraout_RAX_01 + 0x10) = puVar11;
    local_f8 = extraout_RAX_02;
  }
  else {
    local_f8 = *(ulonglong *)(extraout_RAX_01 + 8);
    puVar11 = *(undefined4 **)(extraout_RAX_01 + 0x10);
  }
  *(ulonglong *)(extraout_RAX_01 + 8) = local_f8 + 1;
  lStack_160 = lVar17 * 0x150 + lVar23;
  local_108 = (longlong *)0x0;
  lStack_100 = 0;
  local_118 = (undefined1 (*) [16])&DAT_142aedb10;
  pauStack_110 = (undefined1 (*) [16])0x0;
  local_158 = &local_248;
  local_168 = (undefined1 (*) [16])lVar23;
  pauStack_f0 = (undefined1 (*) [16])puVar11;
  core::iter::adapters::map::impl$2::
  fold<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env_0>_>,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>,hashbrown::map::impl_82::extend::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::filter_map::FilterMap<core::slice::iter::IterMut<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams>,country_core::resources::stairs::invalidate::invalidate_dirty_stair_artifacts::closure_env_0>_>_>_>_>_>
            ();
  local_158 = (undefined1 (**) [16])local_108;
  lStack_150 = lStack_100;
  local_168 = local_118;
  lStack_160 = (longlong)pauStack_110;
  local_148 = local_f8;
  plStack_140 = (longlong *)pauStack_f0;
  if (lStack_100 != 0) {
    auVar28 = *local_118;
    uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar28 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar28[0xf] >> 7) << 0xf);
    pauVar13 = local_118 + 1;
    local_c0 = (longlong *)CONCAT44(local_c0._4_4_,*(undefined4 *)(in_R9 + 0x1c));
    local_b8 = *(undefined4 **)(in_R9 + 0x10);
    lVar23 = lStack_100;
    while (lVar23 != 0) {
      lVar23 = lVar23 + -1;
      uVar16 = uVar15;
      if ((short)uVar15 == 0) {
        do {
          auVar28 = *pauVar13;
          pauVar13 = pauVar13 + 1;
          uVar15 = (uint)(ushort)((ushort)(SUB161(auVar28 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                                  (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                                  (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                                  (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                                  (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                                  (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                                  (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                                  (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                                  (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                                  (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                                  (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                                  (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                                  (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar28[0xf] >> 7) << 0xf);
        } while (uVar15 == 0xffff);
        uVar15 = ~uVar15;
        uVar16 = uVar15;
      }
      for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
      }
      *local_b8 = local_c0._0_4_;
      local_82 = 1;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                ();
      local_82 = 1;
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::stairs_state::StairGraphEdge,u32>_>
                ();
      uVar15 = uVar16 - 1 & uVar16;
    }
    *(undefined4 *)local_a8[2] = *(undefined4 *)((longlong)local_a8 + 0x1c);
    *(undefined4 *)local_b0[2] = *(undefined4 *)((longlong)local_b0 + 0x1c);
    local_118 = (undefined1 (*) [16])&local_168;
    local_82 = 1;
    hashbrown::map::HashMap::
    retain<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,country_core::systems::collision::world::ColliderMeshInstance,std::hash::random::RandomState,alloc::alloc::Global,country_core::systems::collision::world::impl_13::retain_colliders::closure_env_0<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,u32>,system_decorator::decorator_visual::stairs::maintain_stair_graph::closure_env_2>_>
              ();
  }
  auVar28 = *local_248;
  local_190 = lStack_240;
  if (lStack_240 == 0) {
    local_120 = 0;
  }
  else {
    uVar10 = lStack_240 * 8 + 0x17U & 0xfffffffffffffff0;
    local_188 = lStack_240 + uVar10 + 0x11;
    local_180 = (longlong)local_248 - uVar10;
    local_120 = 0x10;
  }
  local_138 = *(undefined4 *)((longlong)local_b0 + 0x1c);
  local_1f0 = *local_b0;
  local_1e8 = (undefined4 *)local_b0[2];
  local_b8 = (undefined4 *)CONCAT44(local_b8._4_4_,*(undefined4 *)((longlong)local_a8 + 0x1c));
  local_b0 = (longlong *)*local_a8;
  local_a8 = (undefined8 *)local_a8[2];
  local_134 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  local_1a0 = *in_stack_00000030;
  local_1d0 = (undefined4 *)in_stack_00000030[2];
  local_1b8 = local_128 + 0x198;
  local_1b0 = local_128 + 0x168;
  local_1c0 = local_128 + 0x1b8;
  local_128 = local_128 + 0x188;
  pauVar13 = local_248 + 1;
  local_1e0 = local_1f0 + 0x30;
  local_1d8 = local_1f0 + 0x60;
  local_198 = local_98 + 0x28;
  local_c0 = local_90 + 4;
  local_1c8 = local_1a0 + 0x20;
  local_1f8 = lStack_230 + -1;
  pauVar14 = local_248;
  uVar15 = ~(uint)(ushort)((ushort)(SUB161(auVar28 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                           (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                           (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                           (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                           (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                           (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                           (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                           (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                           (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                           (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                           (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                           (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                           (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar28[0xf] >> 7) << 0xf);
  if (lStack_230 != 0) {
    do {
      if ((short)uVar15 == 0) {
        do {
          auVar28 = *pauVar13;
          uVar20 = (ushort)(SUB161(auVar28 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar28 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar28 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar28 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar28 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar28 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar28 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar28 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar28 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar28 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar28 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar28 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar28 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar28 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar28 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar28[0xf] >> 7) << 0xf;
          pauVar14 = pauVar14 + -8;
          pauVar13 = pauVar13 + 1;
        } while (uVar20 == 0xffff);
        uVar15 = ~(uint)uVar20;
        local_c4 = uVar15 & -(uint)uVar20 - 2;
      }
      else {
        local_c4 = uVar15 - 1 & uVar15;
      }
      iVar4 = 0;
      for (; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
        iVar4 = iVar4 + 1;
      }
      puVar1 = (ulonglong *)((longlong)pauVar14 + (-8 - (ulonglong)(uint)(iVar4 << 3)));
      uVar15 = (uint)*puVar1;
      uVar16 = *(uint *)((longlong)pauVar14 + (-4 - (ulonglong)(uint)(iVar4 << 3)));
      uVar10 = *puVar1;
      *local_1e8 = local_138;
      *(undefined4 *)local_a8 = local_b8._0_4_;
      local_200 = pauVar14;
      systems::collision::world::RaycastWorld::
      remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
      if ((char)local_f8 != '\x11') {
        LOCK();
        *plStack_e0 = *plStack_e0 + -1;
        UNLOCK();
        if (*plStack_e0 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      *(undefined4 *)local_a8 = local_b8._0_4_;
      systems::collision::world::RaycastWorld::
      remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
      if ((char)local_f8 != '\x11') {
        LOCK();
        *plStack_e0 = *plStack_e0 + -1;
        UNLOCK();
        if (*plStack_e0 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      *(undefined4 *)local_a8 = local_b8._0_4_;
      systems::collision::world::RaycastWorld::
      remove_collider<country_core::resources::stairs::stairs_state::StairGraphEdge>();
      if ((char)local_f8 != '\x11') {
        LOCK();
        *plStack_e0 = *plStack_e0 + -1;
        UNLOCK();
        if (*plStack_e0 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      stairs_state::StairsState::contains_edge_or_stub();
      *local_130 = local_a0._0_4_;
      if (extraout_AL == '\0') {
        visual::StairsVisualState::remove_edge();
        uVar10 = *(ulonglong *)(local_98 + 0x38);
        puVar24 = (uint *)&DAT_00000004;
        if (uVar10 == 0) {
          puVar22 = (uint *)&DAT_00000004;
        }
        else if (uVar10 == 1) {
          lVar23 = *(longlong *)(local_98 + 0x30);
          puVar22 = (uint *)&DAT_00000004;
          if (*(uint *)(lVar23 + 0x20) == uVar15) {
            uVar12 = 0;
LAB_140b37b75:
            puVar22 = *(uint **)(lVar23 + 8 + uVar12 * 0x28);
            puVar24 = puVar22 + *(longlong *)(lVar23 + 0x10 + uVar12 * 0x28) * 2;
          }
        }
        else {
          local_118._0_4_ = uVar15;
          indexmap::map::IndexMap::
          hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          uVar12 = extraout_RAX_04;
          indexmap::map::core::IndexMapCore::
          get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                    ();
          if (extraout_RAX_05 == 1) {
            if (uVar10 <= uVar12) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar23 = *(longlong *)(local_98 + 0x30);
            puVar22 = (uint *)&DAT_00000004;
            if (lVar23 != 0) goto LAB_140b37b75;
          }
          else {
            puVar22 = (uint *)&DAT_00000004;
          }
        }
        for (; puVar22 != puVar24; puVar22 = puVar22 + 2) {
          uVar18 = *puVar22;
          uVar5 = uVar18;
          if (uVar15 < uVar18) {
            uVar5 = uVar15;
          }
          *local_130 = local_a0._0_4_;
          if (uVar18 < uVar15) {
            uVar18 = uVar15;
          }
          local_118._4_4_ = uVar18;
          if (local_90[3] != 0) {
            local_118._0_4_ = uVar5;
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                      ();
            lVar23 = *local_90;
            bVar26 = (byte)(extraout_RAX_06 >> 0x39);
            auVar28 = ZEXT216(CONCAT11(bVar26,bVar26));
            auVar28 = pshuflw(auVar28,auVar28,0);
            lVar17 = 0;
            uVar10 = extraout_RAX_06;
            while( true ) {
              uVar10 = local_90[1] & uVar10;
              pcVar2 = (char *)(lVar23 + uVar10);
              cVar27 = auVar28[0];
              auVar36[0] = -(*pcVar2 == cVar27);
              cVar29 = auVar28[1];
              auVar36[1] = -(pcVar2[1] == cVar29);
              cVar30 = auVar28[2];
              auVar36[2] = -(pcVar2[2] == cVar30);
              cVar31 = auVar28[3];
              auVar36[3] = -(pcVar2[3] == cVar31);
              auVar36[4] = -(pcVar2[4] == cVar27);
              auVar36[5] = -(pcVar2[5] == cVar29);
              auVar36[6] = -(pcVar2[6] == cVar30);
              auVar36[7] = -(pcVar2[7] == cVar31);
              auVar36[8] = -(pcVar2[8] == cVar27);
              auVar36[9] = -(pcVar2[9] == cVar29);
              auVar36[10] = -(pcVar2[10] == cVar30);
              auVar36[0xb] = -(pcVar2[0xb] == cVar31);
              auVar36[0xc] = -(pcVar2[0xc] == cVar27);
              auVar36[0xd] = -(pcVar2[0xd] == cVar29);
              auVar36[0xe] = -(pcVar2[0xe] == cVar30);
              auVar36[0xf] = -(pcVar2[0xf] == cVar31);
              uVar20 = (ushort)(SUB161(auVar36 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar36 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar36 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar36 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar36 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar36 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar36 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar36 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar36 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar36 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar36 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar36 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar36 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar36 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar36 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar36[0xf] >> 7) << 0xf;
              uVar19 = (uint)uVar20;
              while (uVar20 != 0) {
                uVar6 = 0;
                for (uVar7 = uVar19; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                  uVar6 = uVar6 + 1;
                }
                lVar21 = (uVar6 + uVar10 & local_90[1]) * -0x110;
                if ((*(uint *)(lVar23 + -0x110 + lVar21) == uVar5) &&
                   (*(uint *)(lVar23 + -0x10c + lVar21) == uVar18)) {
                  *(undefined1 *)(lVar23 + -8 + lVar21) = 1;
                  goto LAB_140b37c16;
                }
                uVar20 = (ushort)(uVar19 - 1) & (ushort)uVar19;
                uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar20);
              }
              auVar33[0] = -(*pcVar2 == -1);
              auVar33[1] = -(pcVar2[1] == -1);
              auVar33[2] = -(pcVar2[2] == -1);
              auVar33[3] = -(pcVar2[3] == -1);
              auVar33[4] = -(pcVar2[4] == -1);
              auVar33[5] = -(pcVar2[5] == -1);
              auVar33[6] = -(pcVar2[6] == -1);
              auVar33[7] = -(pcVar2[7] == -1);
              auVar33[8] = -(pcVar2[8] == -1);
              auVar33[9] = -(pcVar2[9] == -1);
              auVar33[10] = -(pcVar2[10] == -1);
              auVar33[0xb] = -(pcVar2[0xb] == -1);
              auVar33[0xc] = -(pcVar2[0xc] == -1);
              auVar33[0xd] = -(pcVar2[0xd] == -1);
              auVar33[0xe] = -(pcVar2[0xe] == -1);
              auVar33[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar33 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar33 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar33 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar33 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar33 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar33 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar33 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar33 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar33 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar33 >> 0x4f,0) & 1) != 0) || (SUB161(auVar33 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar33 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar33 >> 0x67,0) & 1) != 0) || (SUB161(auVar33 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar33 >> 0x77,0) & 1) != 0) || auVar33[0xf] < '\0') break;
              uVar10 = uVar10 + lVar17 + 0x10;
              lVar17 = lVar17 + 0x10;
            }
          }
LAB_140b37c16:
        }
        local_118 = (undefined1 (*) [16])CONCAT44(local_118._4_4_,uVar16);
        uVar10 = *(ulonglong *)(local_98 + 0x38);
        puVar24 = (uint *)&DAT_00000004;
        if (uVar10 == 0) {
          puVar22 = (uint *)&DAT_00000004;
        }
        else if (uVar10 == 1) {
          lVar23 = *(longlong *)(local_98 + 0x30);
          puVar22 = (uint *)&DAT_00000004;
          if (*(uint *)(lVar23 + 0x20) == uVar16) {
            uVar12 = 0;
LAB_140b37d50:
            puVar22 = *(uint **)(lVar23 + 8 + uVar12 * 0x28);
            puVar24 = puVar22 + *(longlong *)(lVar23 + 0x10 + uVar12 * 0x28) * 2;
          }
        }
        else {
          indexmap::map::IndexMap::
          hash<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,std::hash::random::RandomState,country_core::resources::walls::decorator::DecoratorId>
                    ();
          uVar12 = extraout_RAX_07;
          indexmap::map::core::IndexMapCore::
          get_index_of<country_core::resources::walls::decorator::DecoratorId,alloc::vec::Vec<tuple_<country_core::resources::walls::decorator::DecoratorId,petgraph::graphmap::CompactDirection>,alloc::alloc::Global>,country_core::resources::walls::decorator::DecoratorId>
                    ();
          if (extraout_RAX_08 == 1) {
            if (uVar10 <= uVar12) {
                    // WARNING: Subroutine does not return
              core::panicking::panic_bounds_check();
            }
            lVar23 = *(longlong *)(local_98 + 0x30);
            puVar22 = (uint *)&DAT_00000004;
            if (lVar23 != 0) goto LAB_140b37d50;
          }
          else {
            puVar22 = (uint *)&DAT_00000004;
          }
        }
        for (; puVar22 != puVar24; puVar22 = puVar22 + 2) {
          uVar18 = *puVar22;
          uVar5 = uVar18;
          if (uVar16 < uVar18) {
            uVar5 = uVar16;
          }
          *local_130 = local_a0._0_4_;
          if (uVar18 < uVar16) {
            uVar18 = uVar16;
          }
          local_118 = (undefined1 (*) [16])CONCAT44(uVar18,uVar5);
          if (local_90[3] != 0) {
            core::hash::BuildHasher::
            hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                      ();
            lVar23 = *local_90;
            bVar26 = (byte)(extraout_RAX_09 >> 0x39);
            auVar28 = ZEXT216(CONCAT11(bVar26,bVar26));
            auVar28 = pshuflw(auVar28,auVar28,0);
            lVar17 = 0;
            uVar10 = extraout_RAX_09;
            while( true ) {
              uVar10 = local_90[1] & uVar10;
              pcVar2 = (char *)(lVar23 + uVar10);
              cVar27 = auVar28[0];
              auVar37[0] = -(*pcVar2 == cVar27);
              cVar29 = auVar28[1];
              auVar37[1] = -(pcVar2[1] == cVar29);
              cVar30 = auVar28[2];
              auVar37[2] = -(pcVar2[2] == cVar30);
              cVar31 = auVar28[3];
              auVar37[3] = -(pcVar2[3] == cVar31);
              auVar37[4] = -(pcVar2[4] == cVar27);
              auVar37[5] = -(pcVar2[5] == cVar29);
              auVar37[6] = -(pcVar2[6] == cVar30);
              auVar37[7] = -(pcVar2[7] == cVar31);
              auVar37[8] = -(pcVar2[8] == cVar27);
              auVar37[9] = -(pcVar2[9] == cVar29);
              auVar37[10] = -(pcVar2[10] == cVar30);
              auVar37[0xb] = -(pcVar2[0xb] == cVar31);
              auVar37[0xc] = -(pcVar2[0xc] == cVar27);
              auVar37[0xd] = -(pcVar2[0xd] == cVar29);
              auVar37[0xe] = -(pcVar2[0xe] == cVar30);
              auVar37[0xf] = -(pcVar2[0xf] == cVar31);
              uVar20 = (ushort)(SUB161(auVar37 >> 7,0) & 1) |
                       (ushort)(SUB161(auVar37 >> 0xf,0) & 1) << 1 |
                       (ushort)(SUB161(auVar37 >> 0x17,0) & 1) << 2 |
                       (ushort)(SUB161(auVar37 >> 0x1f,0) & 1) << 3 |
                       (ushort)(SUB161(auVar37 >> 0x27,0) & 1) << 4 |
                       (ushort)(SUB161(auVar37 >> 0x2f,0) & 1) << 5 |
                       (ushort)(SUB161(auVar37 >> 0x37,0) & 1) << 6 |
                       (ushort)(SUB161(auVar37 >> 0x3f,0) & 1) << 7 |
                       (ushort)(SUB161(auVar37 >> 0x47,0) & 1) << 8 |
                       (ushort)(SUB161(auVar37 >> 0x4f,0) & 1) << 9 |
                       (ushort)(SUB161(auVar37 >> 0x57,0) & 1) << 10 |
                       (ushort)(SUB161(auVar37 >> 0x5f,0) & 1) << 0xb |
                       (ushort)(SUB161(auVar37 >> 0x67,0) & 1) << 0xc |
                       (ushort)(SUB161(auVar37 >> 0x6f,0) & 1) << 0xd |
                       (ushort)(SUB161(auVar37 >> 0x77,0) & 1) << 0xe |
                       (ushort)(auVar37[0xf] >> 7) << 0xf;
              uVar19 = (uint)uVar20;
              while (uVar20 != 0) {
                uVar6 = 0;
                for (uVar7 = uVar19; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
                  uVar6 = uVar6 + 1;
                }
                lVar21 = (uVar6 + uVar10 & local_90[1]) * -0x110;
                if ((*(uint *)(lVar23 + -0x110 + lVar21) == uVar5) &&
                   (*(uint *)(lVar23 + -0x10c + lVar21) == uVar18)) {
                  *(undefined1 *)(lVar23 + -8 + lVar21) = 1;
                  goto LAB_140b37df6;
                }
                uVar20 = (ushort)(uVar19 - 1) & (ushort)uVar19;
                uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar20);
              }
              auVar34[0] = -(*pcVar2 == -1);
              auVar34[1] = -(pcVar2[1] == -1);
              auVar34[2] = -(pcVar2[2] == -1);
              auVar34[3] = -(pcVar2[3] == -1);
              auVar34[4] = -(pcVar2[4] == -1);
              auVar34[5] = -(pcVar2[5] == -1);
              auVar34[6] = -(pcVar2[6] == -1);
              auVar34[7] = -(pcVar2[7] == -1);
              auVar34[8] = -(pcVar2[8] == -1);
              auVar34[9] = -(pcVar2[9] == -1);
              auVar34[10] = -(pcVar2[10] == -1);
              auVar34[0xb] = -(pcVar2[0xb] == -1);
              auVar34[0xc] = -(pcVar2[0xc] == -1);
              auVar34[0xd] = -(pcVar2[0xd] == -1);
              auVar34[0xe] = -(pcVar2[0xe] == -1);
              auVar34[0xf] = -(pcVar2[0xf] == -1);
              if ((((((((((((((((SUB161(auVar34 >> 7,0) & 1) != 0 ||
                               (SUB161(auVar34 >> 0xf,0) & 1) != 0) ||
                              (SUB161(auVar34 >> 0x17,0) & 1) != 0) ||
                             (SUB161(auVar34 >> 0x1f,0) & 1) != 0) ||
                            (SUB161(auVar34 >> 0x27,0) & 1) != 0) ||
                           (SUB161(auVar34 >> 0x2f,0) & 1) != 0) ||
                          (SUB161(auVar34 >> 0x37,0) & 1) != 0) ||
                         (SUB161(auVar34 >> 0x3f,0) & 1) != 0) ||
                        (SUB161(auVar34 >> 0x47,0) & 1) != 0) ||
                       (SUB161(auVar34 >> 0x4f,0) & 1) != 0) || (SUB161(auVar34 >> 0x57,0) & 1) != 0
                      ) || (SUB161(auVar34 >> 0x5f,0) & 1) != 0) ||
                    (SUB161(auVar34 >> 0x67,0) & 1) != 0) || (SUB161(auVar34 >> 0x6f,0) & 1) != 0)
                  || (SUB161(auVar34 >> 0x77,0) & 1) != 0) || auVar34[0xf] < '\0') break;
              uVar10 = uVar10 + lVar17 + 0x10;
              lVar17 = lVar17 + 0x10;
            }
          }
LAB_140b37df6:
        }
      }
      else {
        local_118 = (undefined1 (*) [16])uVar10;
        if (local_90[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                    ();
          lVar23 = *local_90;
          bVar26 = (byte)(extraout_RAX_03 >> 0x39);
          auVar28 = ZEXT216(CONCAT11(bVar26,bVar26));
          auVar28 = pshuflw(auVar28,auVar28,0);
          lVar17 = 0;
          uVar10 = extraout_RAX_03;
          while( true ) {
            uVar10 = local_90[1] & uVar10;
            pcVar2 = (char *)(lVar23 + uVar10);
            cVar27 = auVar28[0];
            auVar35[0] = -(*pcVar2 == cVar27);
            cVar29 = auVar28[1];
            auVar35[1] = -(pcVar2[1] == cVar29);
            cVar30 = auVar28[2];
            auVar35[2] = -(pcVar2[2] == cVar30);
            cVar31 = auVar28[3];
            auVar35[3] = -(pcVar2[3] == cVar31);
            auVar35[4] = -(pcVar2[4] == cVar27);
            auVar35[5] = -(pcVar2[5] == cVar29);
            auVar35[6] = -(pcVar2[6] == cVar30);
            auVar35[7] = -(pcVar2[7] == cVar31);
            auVar35[8] = -(pcVar2[8] == cVar27);
            auVar35[9] = -(pcVar2[9] == cVar29);
            auVar35[10] = -(pcVar2[10] == cVar30);
            auVar35[0xb] = -(pcVar2[0xb] == cVar31);
            auVar35[0xc] = -(pcVar2[0xc] == cVar27);
            auVar35[0xd] = -(pcVar2[0xd] == cVar29);
            auVar35[0xe] = -(pcVar2[0xe] == cVar30);
            auVar35[0xf] = -(pcVar2[0xf] == cVar31);
            uVar20 = (ushort)(SUB161(auVar35 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar35 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar35 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar35 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar35 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar35 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar35 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar35 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar35 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar35 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar35 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar35 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar35 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar35[0xf] >> 7) << 0xf;
            uVar18 = (uint)uVar20;
            while (uVar20 != 0) {
              uVar5 = 0;
              for (uVar19 = uVar18; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
                uVar5 = uVar5 + 1;
              }
              lVar21 = (uVar5 + uVar10 & local_90[1]) * -0x110;
              if ((*(uint *)(lVar23 + -0x110 + lVar21) == uVar15) &&
                 (*(uint *)(lVar23 + -0x10c + lVar21) == uVar16)) {
                *(undefined1 *)(lVar23 + -8 + lVar21) = 1;
                goto LAB_140b379d0;
              }
              uVar20 = (ushort)(uVar18 - 1) & (ushort)uVar18;
              uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar20);
            }
            auVar32[0] = -(*pcVar2 == -1);
            auVar32[1] = -(pcVar2[1] == -1);
            auVar32[2] = -(pcVar2[2] == -1);
            auVar32[3] = -(pcVar2[3] == -1);
            auVar32[4] = -(pcVar2[4] == -1);
            auVar32[5] = -(pcVar2[5] == -1);
            auVar32[6] = -(pcVar2[6] == -1);
            auVar32[7] = -(pcVar2[7] == -1);
            auVar32[8] = -(pcVar2[8] == -1);
            auVar32[9] = -(pcVar2[9] == -1);
            auVar32[10] = -(pcVar2[10] == -1);
            auVar32[0xb] = -(pcVar2[0xb] == -1);
            auVar32[0xc] = -(pcVar2[0xc] == -1);
            auVar32[0xd] = -(pcVar2[0xd] == -1);
            auVar32[0xe] = -(pcVar2[0xe] == -1);
            auVar32[0xf] = -(pcVar2[0xf] == -1);
            if ((((((((((((((((SUB161(auVar32 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar32 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar32 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar32 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar32 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar32 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar32 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar32 >> 0x3f,0) & 1) != 0) || (SUB161(auVar32 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar32 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar32 >> 0x57,0) & 1) != 0) || (SUB161(auVar32 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar32 >> 0x67,0) & 1) != 0) || (SUB161(auVar32 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar32 >> 0x77,0) & 1) != 0) || auVar32[0xf] < '\0') break;
            uVar10 = uVar10 + lVar17 + 0x10;
            lVar17 = lVar17 + 0x10;
          }
        }
      }
LAB_140b379d0:
      *local_1d0 = local_134;
      pauVar14 = (undefined1 (*) [16])CONCAT44(uVar15,uVar16);
      local_178 = (undefined1 (*) [16])0x1;
      local_170 = pauVar14;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::rt::RtMeshHandle>_>_>
                ();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<enum2_<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<enum2_<country_core::resources::rt::RtMeshHandle>,enum2_<country_core::resources::rt::RtMeshHandle>,country_core::resources::rt::DirtyCow<country_core::resources::rt::RtMesh>_>_>
                ();
      local_208 = 2;
      if (local_118 != (undefined1 (*) [16])0x3) {
        local_208 = (undefined1)lStack_100;
      }
      local_210 = local_108;
      if (local_118 != (undefined1 (*) [16])0x3) {
        LOCK();
        *local_108 = *local_108 + -1;
        UNLOCK();
        if (*local_108 == 0) {
          alloc::sync::Arc::drop_slow<country_core::resources::rt::RtMesh,alloc::alloc::Global>();
        }
        *(undefined1 *)(local_1a0 + 0x38) = 1;
      }
      *local_1a8 = local_c8;
      local_178 = pauVar14;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,smallvec::SmallVec<array_<country_core::resources::stairs::stairs_assembly_params::StairSubSegment,1>_>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,smallvec::SmallVec<array_<country_core::resources::stairs::stairs_assembly_params::StairSubSegment,1>_>_>_>
                ();
      if ((((ulonglong)local_118 & 1) != 0) && (1 < local_f8)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *local_1a8 = local_c8;
      local_118 = pauVar14;
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::StairArchWallSupportType>_>
                ();
      bVar25 = local_1f8 != 0;
      local_1f8 = local_1f8 + -1;
      pauVar14 = local_200;
      uVar15 = local_c4;
    } while (bVar25);
    local_1f8 = -1;
  }
  if ((local_190 != 0) && (local_188 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if ((lStack_160 != 0) && (lStack_160 + (lStack_160 * 8 + 0x17U & 0xfffffffffffffff0) != -0x11)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

