
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl
country_core::resources::walls::decorator_storage::DecoratorStorage::get_or_insert_default(void)

{
  char *pcVar1;
  longlong *plVar2;
  undefined8 *puVar3;
  undefined1 (*pauVar4) [16];
  uint uVar5;
  longlong lVar6;
  longlong lVar7;
  undefined *puVar8;
  longlong lVar9;
  longlong lVar10;
  ulonglong extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  longlong *extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *in_RCX;
  undefined **ppuVar11;
  uint uVar12;
  uint uVar13;
  longlong in_RDX;
  longlong lVar14;
  longlong in_R8;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong uVar17;
  ushort uVar18;
  byte bVar19;
  byte bVar20;
  char cVar21;
  char cVar23;
  char cVar24;
  char cVar25;
  undefined1 auVar22 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  int local_148;
  undefined *local_138;
  longlong local_118;
  longlong lStack_110;
  undefined *local_f8;
  longlong lStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  undefined *local_c8;
  undefined *puStack_c0;
  undefined *puStack_b8;
  undefined *puStack_b0;
  longlong local_a8;
  longlong lStack_a0;
  undefined *local_98;
  longlong lStack_90;
  longlong local_88;
  longlong lStack_80;
  longlong local_78;
  longlong lStack_70;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  if ((char)in_RCX[0x36] != '\0') {
                    /* WARNING: Subroutine does not return */
    core::panicking::panic();
  }
  if (in_RDX == 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar19 = (byte)(extraout_RAX >> 0x38);
    bVar20 = bVar19 >> 1;
    auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
    auVar22 = pshuflw(auVar22,auVar22,0);
    lVar16 = 0;
    uVar15 = extraout_RAX;
    while( true ) {
      uVar15 = in_RCX[1] & uVar15;
      pcVar1 = (char *)(*in_RCX + uVar15);
      cVar21 = auVar22[0];
      auVar27[0] = -(*pcVar1 == cVar21);
      cVar23 = auVar22[1];
      auVar27[1] = -(pcVar1[1] == cVar23);
      cVar24 = auVar22[2];
      auVar27[2] = -(pcVar1[2] == cVar24);
      cVar25 = auVar22[3];
      auVar27[3] = -(pcVar1[3] == cVar25);
      auVar27[4] = -(pcVar1[4] == cVar21);
      auVar27[5] = -(pcVar1[5] == cVar23);
      auVar27[6] = -(pcVar1[6] == cVar24);
      auVar27[7] = -(pcVar1[7] == cVar25);
      auVar27[8] = -(pcVar1[8] == cVar21);
      auVar27[9] = -(pcVar1[9] == cVar23);
      auVar27[10] = -(pcVar1[10] == cVar24);
      auVar27[0xb] = -(pcVar1[0xb] == cVar25);
      auVar27[0xc] = -(pcVar1[0xc] == cVar21);
      auVar27[0xd] = -(pcVar1[0xd] == cVar23);
      auVar27[0xe] = -(pcVar1[0xe] == cVar24);
      auVar27[0xf] = -(pcVar1[0xf] == cVar25);
      uVar18 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar18;
      while (uVar18 != 0) {
        uVar13 = 0;
        for (uVar5 = uVar12; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar13 = uVar13 + 1;
        }
        if (*(longlong *)(*in_RCX + -0x98 + (uVar13 + uVar15 & in_RCX[1]) * -0x98) == in_R8) {
          return;
        }
        uVar18 = (ushort)(uVar12 - 1) & (ushort)uVar12;
        uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar18);
      }
      auVar26[0] = -(*pcVar1 == -1);
      auVar26[1] = -(pcVar1[1] == -1);
      auVar26[2] = -(pcVar1[2] == -1);
      auVar26[3] = -(pcVar1[3] == -1);
      auVar26[4] = -(pcVar1[4] == -1);
      auVar26[5] = -(pcVar1[5] == -1);
      auVar26[6] = -(pcVar1[6] == -1);
      auVar26[7] = -(pcVar1[7] == -1);
      auVar26[8] = -(pcVar1[8] == -1);
      auVar26[9] = -(pcVar1[9] == -1);
      auVar26[10] = -(pcVar1[10] == -1);
      auVar26[0xb] = -(pcVar1[0xb] == -1);
      auVar26[0xc] = -(pcVar1[0xc] == -1);
      auVar26[0xd] = -(pcVar1[0xd] == -1);
      auVar26[0xe] = -(pcVar1[0xe] == -1);
      auVar26[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar26 >> 0x4f,0) & 1) != 0) || (SUB161(auVar26 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar26 >> 0x5f,0) & 1) != 0) || (SUB161(auVar26 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar26 >> 0x6f,0) & 1) != 0) || (SUB161(auVar26 >> 0x77,0) & 1) != 0) ||
          auVar26[0xf] < '\0') break;
      uVar15 = uVar15 + lVar16 + 0x10;
      lVar16 = lVar16 + 0x10;
    }
    if (in_RCX[2] == 0) {
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,std::hash::random::RandomState>_>
                ();
    }
    local_98 = (undefined *)0x0;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>_>_>
              ();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::DecoratorId,std::hash::random::RandomState>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::DecoratorId,std::hash::random::RandomState>_>_>
              ();
    local_148 = (int)in_R8;
    if ((local_148 == 3) || (local_138 == (undefined *)0x0)) {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_08 == (longlong *)0x0) {
LAB_140a1d839:
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      local_118 = *extraout_RAX_08;
      lStack_110 = extraout_RAX_08[1];
      *extraout_RAX_08 = *extraout_RAX_08 + 1;
      local_138 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      ppuVar11 = (undefined **)&DAT_142c6bf08;
    }
    else {
      ppuVar11 = &local_f8;
    }
    puStack_b0 = ppuVar11[2];
    puStack_c0 = *ppuVar11;
    puStack_b8 = ppuVar11[1];
    local_c8 = local_138;
    local_a8 = local_118;
    lStack_a0 = lStack_110;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_09 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    local_78 = *extraout_RAX_09;
    lStack_70 = extraout_RAX_09[1];
    *extraout_RAX_09 = *extraout_RAX_09 + 1;
    local_88 = _DAT_142aa9140;
    lStack_80 = _UNK_142aa9148;
    local_98 = anon_c925c76b833903a453982b6800d1a02b_1_llvm_8881321364459548795;
    lStack_90 = _UNK_142aa9138;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_10 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    local_d8 = *extraout_RAX_10;
    lStack_d0 = extraout_RAX_10[1];
    *extraout_RAX_10 = *extraout_RAX_10 + 1;
    lVar10 = DAT_142c6bf18;
    lVar9 = _DAT_142c6bf10;
    lVar16 = _DAT_142c6bf08;
    puVar8 = anon_b07311dcbeea092d4c88647b99c4ec26_9_llvm_12982147591757793021;
    local_f8 = anon_b07311dcbeea092d4c88647b99c4ec26_9_llvm_12982147591757793021;
    lStack_f0 = _DAT_142c6bf08;
    local_e8 = _DAT_142c6bf10;
    lStack_e0 = DAT_142c6bf18;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_11 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    lVar6 = *extraout_RAX_11;
    lVar7 = extraout_RAX_11[1];
    *extraout_RAX_11 = *extraout_RAX_11 + 1;
    pauVar4 = (undefined1 (*) [16])*in_RCX;
    uVar15 = in_RCX[1];
    uVar17 = extraout_RAX & uVar15;
    auVar22 = *(undefined1 (*) [16])(*pauVar4 + uVar17);
    uVar18 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
             (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar22[0xf] >> 7) << 0xf;
    uVar12 = (uint)uVar18;
    if (uVar18 == 0) {
      lVar14 = 0x10;
      do {
        uVar17 = uVar17 + lVar14 & uVar15;
        auVar22 = *(undefined1 (*) [16])(*pauVar4 + uVar17);
        lVar14 = lVar14 + 0x10;
        uVar12 = (uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
                                (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar22[0xf] >> 7) << 0xf);
      } while (uVar12 == 0);
    }
    uVar13 = 0;
    for (; (uVar12 & 1) == 0; uVar12 = uVar12 >> 1 | 0x80000000) {
      uVar13 = uVar13 + 1;
    }
    uVar17 = uVar13 + uVar17 & uVar15;
    bVar20 = (*pauVar4)[uVar17];
    if (-1 < (char)bVar20) {
      auVar22 = *pauVar4;
      uVar12 = 0;
      for (uVar13 = (uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
                                   (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar22[0xf] >> 7) << 0xf); (uVar13 & 1) == 0;
          uVar13 = uVar13 >> 1 | 0x80000000) {
        uVar12 = uVar12 + 1;
      }
      uVar17 = (ulonglong)uVar12;
      bVar20 = (*pauVar4)[uVar17];
    }
    bVar19 = bVar19 >> 1;
    (*pauVar4)[uVar17] = bVar19;
    pauVar4[1][uVar17 - 0x10 & uVar15] = bVar19;
    in_RCX[2] = in_RCX[2] - (ulonglong)(bVar20 & 1);
    lVar14 = uVar17 * -0x98;
    *(longlong *)((longlong)pauVar4 + lVar14 + -0x98) = in_R8;
    puVar3 = (undefined8 *)((longlong)pauVar4 + lVar14 + -0x90);
    *puVar3 = local_c8;
    puVar3[1] = puStack_c0;
    puVar3 = (undefined8 *)((longlong)pauVar4 + lVar14 + -0x80);
    *puVar3 = puStack_b8;
    puVar3[1] = puStack_b0;
    plVar2 = (longlong *)((longlong)pauVar4 + lVar14 + -0x70);
    *plVar2 = local_a8;
    plVar2[1] = lStack_a0;
    puVar3 = (undefined8 *)((longlong)pauVar4 + lVar14 + -0x60);
    *puVar3 = local_98;
    puVar3[1] = lStack_90;
    plVar2 = (longlong *)((longlong)pauVar4 + lVar14 + -0x50);
    *plVar2 = local_88;
    plVar2[1] = lStack_80;
    plVar2 = (longlong *)((longlong)pauVar4 + lVar14 + -0x40);
    *plVar2 = local_78;
    plVar2[1] = lStack_70;
    puVar3 = (undefined8 *)((longlong)pauVar4 + lVar14 + -0x30);
    *puVar3 = puVar8;
    puVar3[1] = lVar16;
    plVar2 = (longlong *)((longlong)pauVar4 + lVar14 + -0x20);
    *plVar2 = lVar9;
    plVar2[1] = lVar10;
    plVar2 = (longlong *)((longlong)pauVar4 + lVar14 + -0x10);
    *plVar2 = lVar6;
    plVar2[1] = lVar7;
    in_RCX[3] = in_RCX[3] + 1;
  }
  else if (in_RDX == 1) {
    if (in_RCX[0x12] == 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>_>_>
                ();
      hashbrown::raw::RawTable::
      remove_entry<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::DecoratorId,std::hash::random::RandomState>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::DecoratorId,std::hash::random::RandomState>_>_>
                ();
      if ((local_148 == 3) || (local_138 == (undefined *)0x0)) {
        core::ops::function::FnOnce::
        call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                  ();
        if (extraout_RAX_00 == (longlong *)0x0) goto LAB_140a1d839;
        local_118 = *extraout_RAX_00;
        lStack_110 = extraout_RAX_00[1];
        *extraout_RAX_00 = *extraout_RAX_00 + 1;
        local_138 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
        ppuVar11 = (undefined **)&DAT_142c6bf08;
      }
      else {
        ppuVar11 = &local_f8;
      }
      puStack_b0 = ppuVar11[2];
      puStack_c0 = *ppuVar11;
      puStack_b8 = ppuVar11[1];
      local_c8 = local_138;
      local_a8 = local_118;
      lStack_a0 = lStack_110;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_01 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      local_78 = *extraout_RAX_01;
      lStack_70 = extraout_RAX_01[1];
      *extraout_RAX_01 = *extraout_RAX_01 + 1;
      local_88 = _DAT_142aa9140;
      lStack_80 = _UNK_142aa9148;
      local_98 = anon_c925c76b833903a453982b6800d1a02b_1_llvm_8881321364459548795;
      lStack_90 = _UNK_142aa9138;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_02 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      local_d8 = *extraout_RAX_02;
      lStack_d0 = extraout_RAX_02[1];
      *extraout_RAX_02 = *extraout_RAX_02 + 1;
      lVar10 = DAT_142c6bf18;
      lVar9 = _DAT_142c6bf10;
      lVar16 = _DAT_142c6bf08;
      puVar8 = anon_b07311dcbeea092d4c88647b99c4ec26_9_llvm_12982147591757793021;
      local_f8 = anon_b07311dcbeea092d4c88647b99c4ec26_9_llvm_12982147591757793021;
      lStack_f0 = _DAT_142c6bf08;
      local_e8 = _DAT_142c6bf10;
      lStack_e0 = DAT_142c6bf18;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_03 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
        std::thread::local::panic_access_error();
      }
      lVar6 = *extraout_RAX_03;
      lVar7 = extraout_RAX_03[1];
      *extraout_RAX_03 = *extraout_RAX_03 + 1;
      in_RCX[0x20] = lVar9;
      in_RCX[0x21] = lVar10;
      in_RCX[0x1e] = (longlong)puVar8;
      in_RCX[0x1f] = lVar16;
      in_RCX[0x1c] = local_78;
      in_RCX[0x1d] = lStack_70;
      in_RCX[0x1a] = local_88;
      in_RCX[0x1b] = lStack_80;
      in_RCX[0x18] = (longlong)local_98;
      in_RCX[0x19] = lStack_90;
      in_RCX[0x16] = local_a8;
      in_RCX[0x17] = lStack_a0;
      in_RCX[0x14] = (longlong)puStack_b8;
      in_RCX[0x15] = (longlong)puStack_b0;
      in_RCX[0x12] = (longlong)local_c8;
      in_RCX[0x13] = (longlong)puStack_c0;
      in_RCX[0x22] = lVar6;
      in_RCX[0x23] = lVar7;
    }
  }
  else if (in_RCX[0x24] == 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>_>_>
              ();
    hashbrown::raw::RawTable::
    remove_entry<tuple_<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::DecoratorId,std::hash::random::RandomState>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::DecoratorId,std::hash::random::RandomState>_>_>
              ();
    if ((local_148 == 3) || (local_138 == (undefined *)0x0)) {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_04 == (longlong *)0x0) goto LAB_140a1d839;
      local_118 = *extraout_RAX_04;
      lStack_110 = extraout_RAX_04[1];
      *extraout_RAX_04 = *extraout_RAX_04 + 1;
      local_138 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      ppuVar11 = (undefined **)&DAT_142c6bf08;
    }
    else {
      ppuVar11 = &local_f8;
    }
    puStack_b0 = ppuVar11[2];
    puStack_c0 = *ppuVar11;
    puStack_b8 = ppuVar11[1];
    local_c8 = local_138;
    local_a8 = local_118;
    lStack_a0 = lStack_110;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_05 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    local_78 = *extraout_RAX_05;
    lStack_70 = extraout_RAX_05[1];
    *extraout_RAX_05 = *extraout_RAX_05 + 1;
    local_88 = _DAT_142aa9140;
    lStack_80 = _UNK_142aa9148;
    local_98 = anon_c925c76b833903a453982b6800d1a02b_1_llvm_8881321364459548795;
    lStack_90 = _UNK_142aa9138;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_06 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    local_d8 = *extraout_RAX_06;
    lStack_d0 = extraout_RAX_06[1];
    *extraout_RAX_06 = *extraout_RAX_06 + 1;
    lVar10 = DAT_142c6bf18;
    lVar9 = _DAT_142c6bf10;
    lVar16 = _DAT_142c6bf08;
    puVar8 = anon_b07311dcbeea092d4c88647b99c4ec26_9_llvm_12982147591757793021;
    local_f8 = anon_b07311dcbeea092d4c88647b99c4ec26_9_llvm_12982147591757793021;
    lStack_f0 = _DAT_142c6bf08;
    local_e8 = _DAT_142c6bf10;
    lStack_e0 = DAT_142c6bf18;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_07 == (longlong *)0x0) {
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    lVar6 = *extraout_RAX_07;
    lVar7 = extraout_RAX_07[1];
    *extraout_RAX_07 = *extraout_RAX_07 + 1;
    in_RCX[0x32] = lVar9;
    in_RCX[0x33] = lVar10;
    in_RCX[0x30] = (longlong)puVar8;
    in_RCX[0x31] = lVar16;
    in_RCX[0x2e] = local_78;
    in_RCX[0x2f] = lStack_70;
    in_RCX[0x2c] = local_88;
    in_RCX[0x2d] = lStack_80;
    in_RCX[0x2a] = (longlong)local_98;
    in_RCX[0x2b] = lStack_90;
    in_RCX[0x28] = local_a8;
    in_RCX[0x29] = lStack_a0;
    in_RCX[0x26] = (longlong)puStack_b8;
    in_RCX[0x27] = (longlong)puStack_b0;
    in_RCX[0x24] = (longlong)local_c8;
    in_RCX[0x25] = (longlong)puStack_c0;
    in_RCX[0x34] = lVar6;
    in_RCX[0x35] = lVar7;
  }
  return;
}

