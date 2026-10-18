
void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::
archive_storage_decorators<ref_<str_>_>(void)

{
  char *pcVar1;
  longlong *plVar2;
  undefined8 uVar3;
  undefined4 uVar4;
  longlong lVar5;
  longlong *plVar6;
  code *pcVar7;
  uint uVar8;
  uint uVar9;
  undefined *puVar10;
  undefined8 uVar11;
  longlong *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  ulonglong *extraout_RAX_09;
  ulonglong extraout_RAX_10;
  ulonglong *extraout_RAX_11;
  ulonglong extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *in_RCX;
  longlong in_RDX;
  longlong *plVar12;
  undefined1 (*pauVar13) [16];
  byte bVar14;
  undefined1 (*pauVar15) [16];
  longlong lVar16;
  undefined *in_R8;
  ulonglong uVar17;
  longlong in_R9;
  longlong lVar18;
  uint uVar19;
  ushort uVar20;
  ulonglong uVar21;
  uint uVar22;
  byte bVar23;
  char cVar24;
  char cVar26;
  char cVar27;
  char cVar28;
  undefined1 auVar25 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined1 auVar33 [16];
  undefined1 auVar34 [16];
  longlong *in_stack_00000028;
  undefined *local_1e8;
  undefined8 uStack_1e0;
  undefined8 local_1d8;
  undefined8 uStack_1d0;
  longlong local_1c8;
  longlong lStack_1c0;
  undefined *local_1b8;
  longlong *plStack_1b0;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  longlong local_198;
  longlong lStack_190;
  undefined *local_188;
  undefined8 uStack_180;
  undefined8 local_178;
  undefined8 uStack_170;
  longlong local_168;
  longlong lStack_160;
  undefined *local_158;
  undefined8 uStack_150;
  undefined8 local_148;
  undefined8 uStack_140;
  longlong local_138;
  longlong lStack_130;
  undefined *local_128;
  longlong *plStack_120;
  undefined8 local_118;
  undefined8 uStack_110;
  longlong local_108;
  longlong lStack_100;
  undefined *local_f8;
  undefined8 uStack_f0;
  undefined8 local_e8;
  undefined8 uStack_e0;
  longlong local_d8;
  longlong lStack_d0;
  longlong local_c0;
  undefined *local_b8;
  longlong *plStack_b0;
  undefined8 local_a8;
  undefined8 uStack_a0;
  longlong local_98;
  longlong lStack_90;
  longlong local_88;
  longlong *local_80;
  int local_78;
  int local_74;
  undefined8 local_70;
  
  local_70 = 0xfffffffffffffffe;
  local_80 = in_RCX;
  if (in_RDX == 0) {
    local_158 = in_R8;
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    bVar14 = (byte)(extraout_RAX_02 >> 0x38);
    bVar23 = bVar14 >> 1;
    auVar25 = ZEXT216(CONCAT11(bVar23,bVar23));
    auVar25 = pshuflw(auVar25,auVar25,0);
    lVar16 = 0;
    uVar17 = extraout_RAX_02;
    while( true ) {
      uVar17 = in_RCX[1] & uVar17;
      pcVar1 = (char *)(*in_RCX + uVar17);
      cVar24 = auVar25[0];
      auVar34[0] = -(*pcVar1 == cVar24);
      cVar26 = auVar25[1];
      auVar34[1] = -(pcVar1[1] == cVar26);
      cVar27 = auVar25[2];
      auVar34[2] = -(pcVar1[2] == cVar27);
      cVar28 = auVar25[3];
      auVar34[3] = -(pcVar1[3] == cVar28);
      auVar34[4] = -(pcVar1[4] == cVar24);
      auVar34[5] = -(pcVar1[5] == cVar26);
      auVar34[6] = -(pcVar1[6] == cVar27);
      auVar34[7] = -(pcVar1[7] == cVar28);
      auVar34[8] = -(pcVar1[8] == cVar24);
      auVar34[9] = -(pcVar1[9] == cVar26);
      auVar34[10] = -(pcVar1[10] == cVar27);
      auVar34[0xb] = -(pcVar1[0xb] == cVar28);
      auVar34[0xc] = -(pcVar1[0xc] == cVar24);
      auVar34[0xd] = -(pcVar1[0xd] == cVar26);
      auVar34[0xe] = -(pcVar1[0xe] == cVar27);
      auVar34[0xf] = -(pcVar1[0xf] == cVar28);
      uVar20 = (ushort)(SUB161(auVar34 >> 7,0) & 1) | (ushort)(SUB161(auVar34 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar34 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar34 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar34 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar34 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar34 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar34 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar34 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar34 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar34 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar34 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar34 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar34 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar34 >> 0x77,0) & 1) << 0xe | (ushort)(auVar34[0xf] >> 7) << 0xf;
      uVar19 = (uint)uVar20;
      while (uVar20 != 0) {
        uVar22 = 0;
        for (uVar8 = uVar19; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar22 = uVar22 + 1;
        }
        if (*(undefined **)(*in_RCX + -0x98 + (uVar22 + uVar17 & in_RCX[1]) * -0x98) == in_R8) {
          return;
        }
        uVar20 = (ushort)(uVar19 - 1) & (ushort)uVar19;
        uVar19 = CONCAT22((short)(uVar19 - 1 >> 0x10),uVar20);
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
                    || (SUB161(auVar31 >> 0x27,0) & 1) != 0) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar31 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar31 >> 0x3f,0) & 1) != 0) || (SUB161(auVar31 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar31 >> 0x4f,0) & 1) != 0) || (SUB161(auVar31 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar31 >> 0x5f,0) & 1) != 0) || (SUB161(auVar31 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar31 >> 0x6f,0) & 1) != 0) || (SUB161(auVar31 >> 0x77,0) & 1) != 0) ||
          auVar31[0xf] < '\0') break;
      uVar17 = uVar17 + lVar16 + 0x10;
      lVar16 = lVar16 + 0x10;
    }
    if (in_RCX[2] == 0) {
      hashbrown::raw::RawTable::
      reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,std::hash::random::RandomState>_>
                ();
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      plVar12 = extraout_RAX_13;
      in_R8 = local_158;
    }
    else {
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      plVar12 = extraout_RAX_06;
    }
    if (plVar12 == (longlong *)0x0) goto LAB_1413e5dd5;
    local_98 = *plVar12;
    lStack_90 = plVar12[1];
    *plVar12 = *plVar12 + 1;
    local_a8 = 0;
    uStack_a0 = 0;
    local_b8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
    plStack_b0 = (longlong *)0x0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_07 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_138 = *extraout_RAX_07;
    lStack_130 = extraout_RAX_07[1];
    *extraout_RAX_07 = *extraout_RAX_07 + 1;
    local_158 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    uStack_150 = 0;
    local_148 = 0;
    uStack_140 = 0;
    core::ops::function::FnOnce::
    call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
              ();
    if (extraout_RAX_08 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    local_168 = *extraout_RAX_08;
    lStack_160 = extraout_RAX_08[1];
    *extraout_RAX_08 = *extraout_RAX_08 + 1;
    local_198 = local_98;
    lStack_190 = lStack_90;
    local_1a8 = local_a8;
    uStack_1a0 = uStack_a0;
    local_1b8 = local_b8;
    plStack_1b0 = plStack_b0;
    local_1e8 = local_158;
    uStack_1e0 = uStack_150;
    local_1d8 = local_148;
    uStack_1d0 = uStack_140;
    local_1c8 = local_138;
    lStack_1c0 = lStack_130;
    local_188 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
    uStack_180 = 0;
    local_178 = 0;
    uStack_170 = 0;
    decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
    memcpy(&local_158,&local_1e8,0x90);
    pauVar13 = (undefined1 (*) [16])*in_RCX;
    uVar17 = in_RCX[1];
    uVar21 = extraout_RAX_02 & uVar17;
    auVar25 = *(undefined1 (*) [16])(*pauVar13 + uVar21);
    uVar20 = (ushort)(SUB161(auVar25 >> 7,0) & 1) | (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
             (ushort)(byte)(auVar25[0xf] >> 7) << 0xf;
    uVar19 = (uint)uVar20;
    if (uVar20 == 0) {
      lVar16 = 0x10;
      do {
        uVar21 = uVar21 + lVar16 & uVar17;
        auVar25 = *(undefined1 (*) [16])(*pauVar13 + uVar21);
        lVar16 = lVar16 + 0x10;
        uVar19 = (uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
                                (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar25[0xf] >> 7) << 0xf);
      } while (uVar19 == 0);
    }
    uVar22 = 0;
    for (; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
      uVar22 = uVar22 + 1;
    }
    uVar21 = uVar22 + uVar21 & uVar17;
    bVar23 = (*pauVar13)[uVar21];
    if (-1 < (char)bVar23) {
      auVar25 = *pauVar13;
      uVar19 = 0;
      for (uVar22 = (uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                                   (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
                                   (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                                  (ushort)(byte)(auVar25[0xf] >> 7) << 0xf); (uVar22 & 1) == 0;
          uVar22 = uVar22 >> 1 | 0x80000000) {
        uVar19 = uVar19 + 1;
      }
      uVar21 = (ulonglong)uVar19;
      bVar23 = (*pauVar13)[uVar21];
    }
    bVar14 = bVar14 >> 1;
    (*pauVar13)[uVar21] = bVar14;
    pauVar13[1][uVar21 - 0x10 & uVar17] = bVar14;
    in_RCX[2] = in_RCX[2] - (ulonglong)(bVar23 & 1);
    *(undefined **)((longlong)pauVar13 + uVar21 * -0x98 + -0x98) = in_R8;
    memcpy((void *)((longlong)pauVar13 + uVar21 * -0x98 + -0x90),&local_158,0x90);
    in_RCX[3] = in_RCX[3] + 1;
    lVar16 = *(longlong *)(in_R9 + 0x48);
  }
  else {
    if (in_RDX == 1) {
      if (in_RCX[0x18] != 0) {
        return;
      }
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX == (longlong *)0x0) {
LAB_1413e5dd5:
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_98 = *extraout_RAX;
      lStack_90 = extraout_RAX[1];
      *extraout_RAX = *extraout_RAX + 1;
      local_a8 = 0;
      uStack_a0 = 0;
      local_b8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
      plStack_b0 = (longlong *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      plVar12 = local_80;
      if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_1c8 = *extraout_RAX_00;
      lStack_1c0 = extraout_RAX_00[1];
      *extraout_RAX_00 = *extraout_RAX_00 + 1;
      local_1e8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_1e0 = 0;
      local_1d8 = 0;
      uStack_1d0 = 0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_d8 = *extraout_RAX_01;
      lStack_d0 = extraout_RAX_01[1];
      *extraout_RAX_01 = *extraout_RAX_01 + 1;
      local_108 = local_98;
      lStack_100 = lStack_90;
      local_118 = local_a8;
      uStack_110 = uStack_a0;
      local_128 = local_b8;
      plStack_120 = plStack_b0;
      local_158 = local_1e8;
      uStack_150 = uStack_1e0;
      local_148 = local_1d8;
      uStack_140 = uStack_1d0;
      local_138 = local_1c8;
      lStack_130 = lStack_1c0;
      local_f8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_f0 = 0;
      local_e8 = 0;
      uStack_e0 = 0;
      decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
      plVar12 = plVar12 + 0x18;
    }
    else {
      if (in_RCX[0x2a] != 0) {
        return;
      }
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_03 == (longlong *)0x0) goto LAB_1413e5dd5;
      local_98 = *extraout_RAX_03;
      lStack_90 = extraout_RAX_03[1];
      *extraout_RAX_03 = *extraout_RAX_03 + 1;
      local_a8 = 0;
      uStack_a0 = 0;
      local_b8 = &anon_c925c76b833903a453982b6800d1a02b_0_llvm_8881321364459548795;
      plStack_b0 = (longlong *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      plVar12 = local_80;
      if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_1c8 = *extraout_RAX_04;
      lStack_1c0 = extraout_RAX_04[1];
      *extraout_RAX_04 = *extraout_RAX_04 + 1;
      local_1e8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_1e0 = 0;
      local_1d8 = 0;
      uStack_1d0 = 0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_05 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_d8 = *extraout_RAX_05;
      lStack_d0 = extraout_RAX_05[1];
      *extraout_RAX_05 = *extraout_RAX_05 + 1;
      local_108 = local_98;
      lStack_100 = lStack_90;
      local_118 = local_a8;
      uStack_110 = uStack_a0;
      local_128 = local_b8;
      plStack_120 = plStack_b0;
      local_158 = local_1e8;
      uStack_150 = uStack_1e0;
      local_148 = local_1d8;
      uStack_140 = uStack_1d0;
      local_138 = local_1c8;
      lStack_130 = lStack_1c0;
      local_f8 = &anon_b07311dcbeea092d4c88647b99c4ec26_8_llvm_12982147591757793021;
      uStack_f0 = 0;
      local_e8 = 0;
      uStack_e0 = 0;
      decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
      plVar12 = plVar12 + 0x2a;
    }
    memcpy(&local_1e8,&local_158,0x90);
    memcpy(plVar12,&local_1e8,0x90);
    lVar16 = *(longlong *)(in_R9 + 0x48);
  }
  if (lVar16 != 0) {
    local_88 = *in_stack_00000028;
    plVar12 = (longlong *)in_stack_00000028[4];
    pauVar15 = *(undefined1 (**) [16])(in_R9 + 0x30);
    pauVar13 = pauVar15 + 1;
    auVar25 = *pauVar15;
    uVar19 = ~(uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
                             (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar25[0xf] >> 7) << 0xf);
    local_c0 = local_88 + 0x20;
    do {
      if ((short)uVar19 == 0) {
        do {
          auVar25 = *pauVar13;
          uVar20 = (ushort)(SUB161(auVar25 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
                   (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar25[0xf] >> 7) << 0xf;
          pauVar15 = pauVar15 + -0x50;
          pauVar13 = pauVar13 + 1;
        } while (uVar20 == 0xffff);
        uVar22 = ~(uint)uVar20;
        uVar19 = uVar22 & -(uint)uVar20 - 2;
      }
      else {
        uVar22 = uVar19;
        uVar19 = uVar19 - 1 & uVar19;
      }
      uVar8 = 0;
      for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      local_74 = *(int *)pauVar15[(ulonglong)uVar8 * -5 + -5];
      if (plVar12[3] != 0) {
        core::hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
        lVar5 = *plVar12;
        bVar14 = (byte)(extraout_RAX_10 >> 0x39);
        auVar25 = ZEXT216(CONCAT11(bVar14,bVar14));
        auVar25 = pshuflw(auVar25,auVar25,0);
        lVar18 = 0;
        uVar17 = extraout_RAX_10;
        while( true ) {
          uVar17 = plVar12[1] & uVar17;
          pcVar1 = (char *)(lVar5 + uVar17);
          cVar24 = auVar25[0];
          auVar32[0] = -(*pcVar1 == cVar24);
          cVar26 = auVar25[1];
          auVar32[1] = -(pcVar1[1] == cVar26);
          cVar27 = auVar25[2];
          auVar32[2] = -(pcVar1[2] == cVar27);
          cVar28 = auVar25[3];
          auVar32[3] = -(pcVar1[3] == cVar28);
          auVar32[4] = -(pcVar1[4] == cVar24);
          auVar32[5] = -(pcVar1[5] == cVar26);
          auVar32[6] = -(pcVar1[6] == cVar27);
          auVar32[7] = -(pcVar1[7] == cVar28);
          auVar32[8] = -(pcVar1[8] == cVar24);
          auVar32[9] = -(pcVar1[9] == cVar26);
          auVar32[10] = -(pcVar1[10] == cVar27);
          auVar32[0xb] = -(pcVar1[0xb] == cVar28);
          auVar32[0xc] = -(pcVar1[0xc] == cVar24);
          auVar32[0xd] = -(pcVar1[0xd] == cVar26);
          auVar32[0xe] = -(pcVar1[0xe] == cVar27);
          auVar32[0xf] = -(pcVar1[0xf] == cVar28);
          uVar20 = (ushort)(SUB161(auVar32 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
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
                   (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar32[0xf] >> 7) << 0xf;
          uVar22 = (uint)uVar20;
          while (uVar20 != 0) {
            uVar8 = 0;
            for (uVar9 = uVar22; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
              uVar8 = uVar8 + 1;
            }
            uVar21 = uVar8 + uVar17 & plVar12[1];
            if (local_74 == *(int *)(lVar5 + -0x18 + uVar21 * -0x18)) {
              if (*(char *)(lVar5 + -4 + uVar21 * -0x18) == '\x11') goto LAB_1413e5ae0;
              rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
              if (extraout_RAX_09 == (ulonglong *)0x0) goto LAB_1413e5e0f;
              uVar17 = *extraout_RAX_09 & 0xffffffff;
              if (((*(ulonglong *)(local_88 + 0x10) <= uVar17) ||
                  (lVar5 = *(longlong *)(local_88 + 8), lVar5 == 0)) ||
                 (*(uint *)(lVar5 + 0x20 + uVar17 * 0x28) != ((uint)(*extraout_RAX_09 >> 0x20) | 1))
                 ) goto LAB_1413e5df7;
              uVar3 = *(undefined8 *)((longlong)extraout_RAX_09 + 0x44);
              uVar4 = *(undefined4 *)((longlong)extraout_RAX_09 + 0x4c);
              puVar10 = *(undefined **)((longlong)extraout_RAX_09 + 0x34);
              uVar11 = *(undefined8 *)((longlong)extraout_RAX_09 + 0x3c);
              plVar2 = (longlong *)(lVar5 + uVar17 * 0x28);
              plVar6 = (longlong *)plVar2[3];
              LOCK();
              lVar5 = *plVar6;
              *plVar6 = *plVar6 + 1;
              UNLOCK();
              if (*plVar6 == 0 || SCARRY8(lVar5,1) != *plVar6 < 0) goto LAB_1413e5e79;
              local_128 = (undefined *)plVar2[2];
              local_138 = *plVar2;
              lStack_130 = plVar2[1];
              uStack_140 = CONCAT44(uStack_140._4_4_,uVar4);
              local_158 = puVar10;
              uStack_150 = uVar11;
              local_148 = uVar3;
              plStack_120 = plVar6;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              if ((char)local_1c8 != '\x11') {
                LOCK();
                *plStack_1b0 = *plStack_1b0 + -1;
                UNLOCK();
                if (*plStack_1b0 == 0) {
                  alloc::sync::Arc::
                  drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                            ();
                }
              }
              goto LAB_1413e5ae0;
            }
            uVar20 = (ushort)(uVar22 - 1) & (ushort)uVar22;
            uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar20);
          }
          auVar29[0] = -(*pcVar1 == -1);
          auVar29[1] = -(pcVar1[1] == -1);
          auVar29[2] = -(pcVar1[2] == -1);
          auVar29[3] = -(pcVar1[3] == -1);
          auVar29[4] = -(pcVar1[4] == -1);
          auVar29[5] = -(pcVar1[5] == -1);
          auVar29[6] = -(pcVar1[6] == -1);
          auVar29[7] = -(pcVar1[7] == -1);
          auVar29[8] = -(pcVar1[8] == -1);
          auVar29[9] = -(pcVar1[9] == -1);
          auVar29[10] = -(pcVar1[10] == -1);
          auVar29[0xb] = -(pcVar1[0xb] == -1);
          auVar29[0xc] = -(pcVar1[0xc] == -1);
          auVar29[0xd] = -(pcVar1[0xd] == -1);
          auVar29[0xe] = -(pcVar1[0xe] == -1);
          auVar29[0xf] = -(pcVar1[0xf] == -1);
          if ((((((((((((((((SUB161(auVar29 >> 7,0) & 1) != 0 || (SUB161(auVar29 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar29 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar29 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar29 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar29 >> 0x2f,0) & 1) != 0) || (SUB161(auVar29 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar29 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar29 >> 0x47,0) & 1) != 0) || (SUB161(auVar29 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar29 >> 0x57,0) & 1) != 0) || (SUB161(auVar29 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar29 >> 0x67,0) & 1) != 0) || (SUB161(auVar29 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar29 >> 0x77,0) & 1) != 0) || auVar29[0xf] < '\0') break;
          uVar17 = uVar17 + lVar18 + 0x10;
          lVar18 = lVar18 + 0x10;
        }
      }
LAB_1413e5ae0:
      lVar16 = lVar16 + -1;
    } while (lVar16 != 0);
    lVar16 = *(longlong *)(in_R9 + 0x48);
    if (lVar16 != 0) {
      local_80 = local_80 + 0x12;
      pauVar15 = *(undefined1 (**) [16])(in_R9 + 0x30);
      pauVar13 = pauVar15 + 1;
      auVar25 = *pauVar15;
      uVar19 = ~(uint)(ushort)((ushort)(SUB161(auVar25 >> 7,0) & 1) |
                               (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
                               (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                              (ushort)(byte)(auVar25[0xf] >> 7) << 0xf);
      do {
        if ((short)uVar19 == 0) {
          do {
            auVar25 = *pauVar13;
            uVar20 = (ushort)(SUB161(auVar25 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar25 >> 0xf,0) & 1) << 1 |
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
                     (ushort)(SUB161(auVar25 >> 0x77,0) & 1) << 0xe |
                     (ushort)(byte)(auVar25[0xf] >> 7) << 0xf;
            pauVar15 = pauVar15 + -0x50;
            pauVar13 = pauVar13 + 1;
          } while (uVar20 == 0xffff);
          uVar22 = ~(uint)uVar20;
          uVar19 = uVar22 & -(uint)uVar20 - 2;
        }
        else {
          uVar22 = uVar19;
          uVar19 = uVar19 - 1 & uVar19;
        }
        uVar8 = 0;
        for (; (uVar22 & 1) == 0; uVar22 = uVar22 >> 1 | 0x80000000) {
          uVar8 = uVar8 + 1;
        }
        local_78 = *(int *)pauVar15[(ulonglong)uVar8 * -5 + -5];
        if (plVar12[9] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                    ();
          lVar5 = plVar12[6];
          bVar14 = (byte)(extraout_RAX_12 >> 0x39);
          auVar25 = ZEXT216(CONCAT11(bVar14,bVar14));
          auVar25 = pshuflw(auVar25,auVar25,0);
          lVar18 = 0;
          uVar17 = extraout_RAX_12;
          do {
            uVar17 = plVar12[7] & uVar17;
            pcVar1 = (char *)(lVar5 + uVar17);
            cVar24 = auVar25[0];
            auVar33[0] = -(*pcVar1 == cVar24);
            cVar26 = auVar25[1];
            auVar33[1] = -(pcVar1[1] == cVar26);
            cVar27 = auVar25[2];
            auVar33[2] = -(pcVar1[2] == cVar27);
            cVar28 = auVar25[3];
            auVar33[3] = -(pcVar1[3] == cVar28);
            auVar33[4] = -(pcVar1[4] == cVar24);
            auVar33[5] = -(pcVar1[5] == cVar26);
            auVar33[6] = -(pcVar1[6] == cVar27);
            auVar33[7] = -(pcVar1[7] == cVar28);
            auVar33[8] = -(pcVar1[8] == cVar24);
            auVar33[9] = -(pcVar1[9] == cVar26);
            auVar33[10] = -(pcVar1[10] == cVar27);
            auVar33[0xb] = -(pcVar1[0xb] == cVar28);
            auVar33[0xc] = -(pcVar1[0xc] == cVar24);
            auVar33[0xd] = -(pcVar1[0xd] == cVar26);
            auVar33[0xe] = -(pcVar1[0xe] == cVar27);
            auVar33[0xf] = -(pcVar1[0xf] == cVar28);
            uVar20 = (ushort)(SUB161(auVar33 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar33 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar33 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar33 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar33 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar33 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar33 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar33 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar33 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar33 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar33 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar33 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar33 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar33 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar33 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar33[0xf] >> 7) << 0xf;
            uVar22 = (uint)uVar20;
            while (uVar20 != 0) {
              uVar8 = 0;
              for (uVar9 = uVar22; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
                uVar8 = uVar8 + 1;
              }
              uVar21 = uVar8 + uVar17 & plVar12[7];
              if (local_78 == *(int *)(lVar5 + -0x18 + uVar21 * -0x18)) {
                if (*(char *)(lVar5 + -4 + uVar21 * -0x18) != '\x11') {
                  rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
                  if (extraout_RAX_11 == (ulonglong *)0x0) {
LAB_1413e5e0f:
                    // WARNING: Subroutine does not return
                    core::option::expect_failed();
                  }
                  uVar17 = *extraout_RAX_11 & 0xffffffff;
                  if (((*(ulonglong *)(local_88 + 0x10) <= uVar17) ||
                      (lVar5 = *(longlong *)(local_88 + 8), lVar5 == 0)) ||
                     (*(uint *)(lVar5 + 0x20 + uVar17 * 0x28) !=
                      ((uint)(*extraout_RAX_11 >> 0x20) | 1))) {
LAB_1413e5df7:
                    // WARNING: Subroutine does not return
                    std::panicking::begin_panic<ref_<str_>_>();
                  }
                  uVar3 = *(undefined8 *)((longlong)extraout_RAX_11 + 0x44);
                  uVar4 = *(undefined4 *)((longlong)extraout_RAX_11 + 0x4c);
                  puVar10 = *(undefined **)((longlong)extraout_RAX_11 + 0x34);
                  uVar11 = *(undefined8 *)((longlong)extraout_RAX_11 + 0x3c);
                  plVar2 = (longlong *)(lVar5 + uVar17 * 0x28);
                  plVar6 = (longlong *)plVar2[3];
                  LOCK();
                  lVar5 = *plVar6;
                  *plVar6 = *plVar6 + 1;
                  UNLOCK();
                  if (*plVar6 == 0 || SCARRY8(lVar5,1) != *plVar6 < 0) {
LAB_1413e5e79:
                    // WARNING: Does not return
                    pcVar7 = (code *)invalidInstructionException();
                    (*pcVar7)();
                  }
                  local_128 = (undefined *)plVar2[2];
                  local_138 = *plVar2;
                  lStack_130 = plVar2[1];
                  uStack_140 = CONCAT44(uStack_140._4_4_,uVar4);
                  local_158 = puVar10;
                  uStack_150 = uVar11;
                  local_148 = uVar3;
                  plStack_120 = plVar6;
                  hashbrown::map::HashMap::
                  insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                            ();
                  if ((char)local_1c8 != '\x11') {
                    LOCK();
                    *plStack_1b0 = *plStack_1b0 + -1;
                    UNLOCK();
                    if (*plStack_1b0 == 0) {
                      alloc::sync::Arc::
                      drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                                ();
                    }
                  }
                }
                goto LAB_1413e5d60;
              }
              uVar20 = (ushort)(uVar22 - 1) & (ushort)uVar22;
              uVar22 = CONCAT22((short)(uVar22 - 1 >> 0x10),uVar20);
            }
            auVar30[0] = -(*pcVar1 == -1);
            auVar30[1] = -(pcVar1[1] == -1);
            auVar30[2] = -(pcVar1[2] == -1);
            auVar30[3] = -(pcVar1[3] == -1);
            auVar30[4] = -(pcVar1[4] == -1);
            auVar30[5] = -(pcVar1[5] == -1);
            auVar30[6] = -(pcVar1[6] == -1);
            auVar30[7] = -(pcVar1[7] == -1);
            auVar30[8] = -(pcVar1[8] == -1);
            auVar30[9] = -(pcVar1[9] == -1);
            auVar30[10] = -(pcVar1[10] == -1);
            auVar30[0xb] = -(pcVar1[0xb] == -1);
            auVar30[0xc] = -(pcVar1[0xc] == -1);
            auVar30[0xd] = -(pcVar1[0xd] == -1);
            auVar30[0xe] = -(pcVar1[0xe] == -1);
            auVar30[0xf] = -(pcVar1[0xf] == -1);
            if ((((((((((((((((SUB161(auVar30 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar30 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar30 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar30 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar30 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar30 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar30 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar30 >> 0x3f,0) & 1) != 0) || (SUB161(auVar30 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar30 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar30 >> 0x57,0) & 1) != 0) || (SUB161(auVar30 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar30 >> 0x67,0) & 1) != 0) || (SUB161(auVar30 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar30 >> 0x77,0) & 1) != 0) || auVar30[0xf] < '\0') break;
            uVar17 = uVar17 + lVar18 + 0x10;
            lVar18 = lVar18 + 0x10;
          } while( true );
        }
LAB_1413e5d60:
        lVar16 = lVar16 + -1;
      } while (lVar16 != 0);
    }
  }
  return;
}

