
void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::
archive_current_decorators_in_storages<ref_<str_>_>(void)

{
  undefined1 auVar1 [16];
  longlong *plVar2;
  code *pcVar3;
  uint uVar4;
  uint uVar5;
  int iVar6;
  uint uVar7;
  uint uVar8;
  void *extraout_RAX;
  longlong *extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong *extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  longlong *extraout_RAX_08;
  longlong *extraout_RAX_09;
  ulonglong extraout_RAX_10;
  longlong *extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  undefined8 *extraout_RAX_14;
  ulonglong extraout_RAX_15;
  undefined8 *extraout_RAX_16;
  ulonglong extraout_RAX_17;
  longlong *in_RCX;
  longlong *plVar9;
  longlong *in_RDX;
  byte bVar10;
  void *_Dst;
  undefined1 (*pauVar11) [16];
  longlong in_R8;
  ulonglong uVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  undefined1 (*pauVar16) [16];
  ushort uVar17;
  longlong *plVar18;
  ulonglong uVar19;
  byte bVar20;
  char cVar21;
  char cVar23;
  char cVar24;
  char cVar25;
  undefined1 auVar22 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined1 auVar28 [16];
  undefined1 auVar29 [16];
  undefined1 auVar30 [16];
  undefined1 auVar31 [16];
  undefined1 unaff_XMM6_Ba;
  char cVar32;
  undefined1 unaff_XMM6_Bb;
  char cVar33;
  undefined1 unaff_XMM6_Bc;
  char cVar34;
  undefined1 unaff_XMM6_Bd;
  char cVar35;
  undefined1 unaff_XMM6_Be;
  char cVar36;
  undefined1 unaff_XMM6_Bf;
  char cVar37;
  undefined1 unaff_XMM6_Bg;
  char cVar38;
  undefined1 unaff_XMM6_Bh;
  char cVar39;
  undefined1 unaff_XMM6_Bi;
  char cVar40;
  undefined1 unaff_XMM6_Bj;
  char cVar41;
  undefined1 unaff_XMM6_Bk;
  char cVar42;
  undefined1 unaff_XMM6_Bl;
  char cVar43;
  undefined1 unaff_XMM6_Bm;
  char cVar44;
  undefined1 unaff_XMM6_Bn;
  char cVar45;
  undefined1 unaff_XMM6_Bo;
  char cVar46;
  undefined1 unaff_XMM6_Bp;
  char cVar47;
  undefined8 *in_stack_00000028;
  void *in_stack_00000030;
  size_t in_stack_00000038;
  undefined1 local_268 [16];
  undefined1 local_258 [16];
  longlong local_248;
  longlong lStack_240;
  longlong *local_238;
  code *pcStack_230;
  undefined8 local_228;
  undefined8 uStack_220;
  longlong local_218;
  longlong lStack_210;
  undefined *local_208;
  undefined8 uStack_200;
  undefined8 local_1f8;
  undefined8 uStack_1f0;
  longlong local_1e8;
  longlong lStack_1e0;
  longlong local_1d8;
  undefined8 uStack_1d0;
  undefined8 local_1c8;
  undefined8 uStack_1c0;
  undefined8 local_1b8;
  char local_1b0;
  undefined2 local_1af;
  undefined1 local_1ad;
  undefined8 local_1a8;
  char local_1a0;
  undefined2 local_19f;
  undefined1 local_19d;
  size_t local_198;
  void *local_190;
  longlong *local_180;
  longlong *local_178;
  longlong *local_170;
  undefined1 local_168 [16];
  undefined8 local_158;
  undefined8 uStack_150;
  int local_148;
  undefined4 uStack_144;
  longlong lStack_140;
  longlong *local_138;
  code *pcStack_130;
  undefined8 local_128;
  undefined8 uStack_120;
  longlong local_118;
  longlong lStack_110;
  undefined *local_108;
  longlong lStack_100;
  undefined8 local_f8;
  undefined8 uStack_f0;
  longlong local_e8;
  longlong lStack_e0;
  longlong *local_d8;
  code *pcStack_d0;
  undefined8 local_c8;
  undefined8 uStack_c0;
  longlong local_b8;
  longlong lStack_b0;
  undefined8 local_a0;
  longlong local_98;
  int local_90;
  int local_8c;
  longlong *local_88;
  undefined8 local_80;
  
  local_80 = 0xfffffffffffffffe;
  if (-1 < (longlong)in_stack_00000038) {
    local_88 = in_RCX;
    if (in_stack_00000038 == 0) {
      _Dst = (void *)0x1;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX;
      if (extraout_RAX == (void *)0x0) goto LAB_1412b5ead;
    }
    memcpy(_Dst,in_stack_00000030,in_stack_00000038);
    local_198 = in_stack_00000038;
    plVar18 = in_RDX + in_R8 * 2;
    local_a0 = *in_stack_00000028;
    plVar2 = (longlong *)in_stack_00000028[4];
    local_180 = local_88 + 0x2a;
    local_178 = local_88 + 0x18;
    local_170 = local_88 + 4;
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                             CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                   CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
    cVar32 = (char)iVar6;
    cVar33 = (char)((uint)iVar6 >> 8);
    cVar34 = (char)((uint)iVar6 >> 0x10);
    cVar35 = (char)((uint)iVar6 >> 0x18);
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                             CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                   CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
    cVar36 = (char)iVar6;
    cVar37 = (char)((uint)iVar6 >> 8);
    cVar38 = (char)((uint)iVar6 >> 0x10);
    cVar39 = (char)((uint)iVar6 >> 0x18);
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                             CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                   CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
    cVar40 = (char)iVar6;
    cVar41 = (char)((uint)iVar6 >> 8);
    cVar42 = (char)((uint)iVar6 >> 0x10);
    cVar43 = (char)((uint)iVar6 >> 0x18);
    iVar6 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                             CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                   CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
    cVar44 = (char)iVar6;
    cVar45 = (char)((uint)iVar6 >> 8);
    cVar46 = (char)((uint)iVar6 >> 0x10);
    cVar47 = (char)((uint)iVar6 >> 0x18);
    local_190 = _Dst;
LAB_1412b52d0:
    while( true ) {
      if (in_RDX == plVar18) {
        if (local_198 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        return;
      }
      lVar14 = *in_RDX;
      lVar15 = in_RDX[1];
      decorator_storage::DecoratorStorage::get();
      in_RDX = in_RDX + 2;
      if (extraout_RAX_03 != 0) break;
      alloc::string::impl$6::clone();
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_07 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b8 = *extraout_RAX_07;
      lStack_b0 = extraout_RAX_07[1];
      *extraout_RAX_07 = *extraout_RAX_07 + 1;
      local_c8 = 0;
      uStack_c0 = 0;
      local_d8 = (longlong *)&anon_bf4ed97acacab9c6e71765005891de1b_1_llvm_1767052282543269069;
      pcStack_d0 = (code *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_08 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_248 = *extraout_RAX_08;
      lStack_240 = extraout_RAX_08[1];
      *extraout_RAX_08 = *extraout_RAX_08 + 1;
      local_268._8_8_ = 0;
      local_268._0_8_ = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
      local_258 = ZEXT816(0);
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_09 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_e8 = *extraout_RAX_09;
      lStack_e0 = extraout_RAX_09[1];
      *extraout_RAX_09 = *extraout_RAX_09 + 1;
      local_118 = local_b8;
      lStack_110 = lStack_b0;
      local_128 = local_c8;
      uStack_120 = uStack_c0;
      local_138 = local_d8;
      pcStack_130 = pcStack_d0;
      local_168 = local_268;
      local_158 = local_258._0_8_;
      uStack_150 = local_258._8_8_;
      local_148 = (int)local_248;
      uStack_144 = (undefined4)((ulonglong)local_248 >> 0x20);
      lStack_140 = lStack_240;
      local_f8 = 0;
      uStack_f0 = 0;
      local_108 = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
      lStack_100 = 0;
      local_d8 = &local_1d8;
      pcStack_d0 = alloc::string::impl$23::fmt;
      local_268._8_8_ = 2;
      local_268._0_8_ = &anon_41f28c8b0f8809a1d72a2d55317e3f61_6_llvm_146935197314726508;
      local_258._8_8_ = 1;
      local_258._0_8_ = &local_d8;
      local_248 = 0;
      alloc::fmt::format::format_inner();
      archive_storage_decorators<alloc::string::String>();
      if ((pcStack_130 != (code *)0x0) && ((longlong)pcStack_130 * 0x51 != -0x61)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((local_168._8_8_ != 0) &&
         (local_168._8_8_ + (local_168._8_8_ * 4 + 0x13U & 0xfffffffffffffff0) != -0x11)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if ((lStack_100 != 0) && (lStack_100 + (lStack_100 * 4 + 0x13U & 0xfffffffffffffff0) != -0x11)
         ) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      if (local_1d8 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    }
    local_98 = extraout_RAX_03;
    if (lVar14 == 0) goto LAB_1412b56a0;
    if ((int)lVar14 == 1) {
      if (*local_178 != 0) goto LAB_1412b52d0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_04 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b8 = *extraout_RAX_04;
      lStack_b0 = extraout_RAX_04[1];
      *extraout_RAX_04 = *extraout_RAX_04 + 1;
      local_c8 = 0;
      uStack_c0 = 0;
      local_d8 = (longlong *)&anon_bf4ed97acacab9c6e71765005891de1b_1_llvm_1767052282543269069;
      pcStack_d0 = (code *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_05 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_248 = *extraout_RAX_05;
      lStack_240 = extraout_RAX_05[1];
      *extraout_RAX_05 = *extraout_RAX_05 + 1;
      local_268._8_8_ = 0;
      local_268._0_8_ = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
      local_258 = ZEXT816(0);
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_06 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_e8 = *extraout_RAX_06;
      lStack_e0 = extraout_RAX_06[1];
      *extraout_RAX_06 = *extraout_RAX_06 + 1;
      local_118 = local_b8;
      lStack_110 = lStack_b0;
      local_128 = local_c8;
      uStack_120 = uStack_c0;
      local_138 = local_d8;
      pcStack_130 = pcStack_d0;
      local_168 = local_268;
      local_158 = local_258._0_8_;
      uStack_150 = local_258._8_8_;
      local_148 = (int)local_248;
      uStack_144 = (undefined4)((ulonglong)local_248 >> 0x20);
      lStack_140 = lStack_240;
      local_f8 = 0;
      uStack_f0 = 0;
      local_108 = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
      lStack_100 = 0;
      decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
      memcpy(local_268,local_168,0x90);
      plVar9 = local_178;
    }
    else {
      if (*local_180 != 0) goto LAB_1412b52d0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_11 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_b8 = *extraout_RAX_11;
      lStack_b0 = extraout_RAX_11[1];
      *extraout_RAX_11 = *extraout_RAX_11 + 1;
      local_c8 = 0;
      uStack_c0 = 0;
      local_d8 = (longlong *)&anon_bf4ed97acacab9c6e71765005891de1b_1_llvm_1767052282543269069;
      pcStack_d0 = (code *)0x0;
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_12 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_248 = *extraout_RAX_12;
      lStack_240 = extraout_RAX_12[1];
      *extraout_RAX_12 = *extraout_RAX_12 + 1;
      local_268._8_8_ = 0;
      local_268._0_8_ = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
      local_258 = ZEXT816(0);
      core::ops::function::FnOnce::
      call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
                ();
      if (extraout_RAX_13 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
        std::thread::local::panic_access_error();
      }
      local_e8 = *extraout_RAX_13;
      lStack_e0 = extraout_RAX_13[1];
      *extraout_RAX_13 = *extraout_RAX_13 + 1;
      local_118 = local_b8;
      lStack_110 = lStack_b0;
      local_128 = local_c8;
      uStack_120 = uStack_c0;
      local_138 = local_d8;
      pcStack_130 = pcStack_d0;
      local_168 = local_268;
      local_158 = local_258._0_8_;
      uStack_150 = local_258._8_8_;
      local_148 = (int)local_248;
      uStack_144 = (undefined4)((ulonglong)local_248 >> 0x20);
      lStack_140 = lStack_240;
      local_f8 = 0;
      uStack_f0 = 0;
      local_108 = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
      lStack_100 = 0;
      decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
      memcpy(local_268,local_168,0x90);
      plVar9 = local_180;
    }
    memcpy(plVar9,local_268,0x90);
    goto LAB_1412b582f;
  }
LAB_1412b5ead:
                    // WARNING: Subroutine does not return
  alloc::raw_vec::handle_error();
LAB_1412b56a0:
  local_168._0_8_ = lVar15;
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
  bVar10 = (byte)(extraout_RAX_10 >> 0x38);
  bVar20 = bVar10 >> 1;
  auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
  auVar22 = pshuflw(auVar22,auVar22,0);
  lVar14 = 0;
  uVar12 = extraout_RAX_10;
  while( true ) {
    uVar12 = local_88[1] & uVar12;
    auVar1 = *(undefined1 (*) [16])(*local_88 + uVar12);
    cVar21 = auVar22[0];
    auVar29[0] = -(auVar1[0] == cVar21);
    cVar23 = auVar22[1];
    auVar29[1] = -(auVar1[1] == cVar23);
    cVar24 = auVar22[2];
    auVar29[2] = -(auVar1[2] == cVar24);
    cVar25 = auVar22[3];
    auVar29[3] = -(auVar1[3] == cVar25);
    auVar29[4] = -(auVar1[4] == cVar21);
    auVar29[5] = -(auVar1[5] == cVar23);
    auVar29[6] = -(auVar1[6] == cVar24);
    auVar29[7] = -(auVar1[7] == cVar25);
    auVar29[8] = -(auVar1[8] == cVar21);
    auVar29[9] = -(auVar1[9] == cVar23);
    auVar29[10] = -(auVar1[10] == cVar24);
    auVar29[0xb] = -(auVar1[0xb] == cVar25);
    auVar29[0xc] = -(auVar1[0xc] == cVar21);
    auVar29[0xd] = -(auVar1[0xd] == cVar23);
    auVar29[0xe] = -(auVar1[0xe] == cVar24);
    auVar29[0xf] = -(auVar1[0xf] == cVar25);
    uVar17 = (ushort)(SUB161(auVar29 >> 7,0) & 1) | (ushort)(SUB161(auVar29 >> 0xf,0) & 1) << 1 |
             (ushort)(SUB161(auVar29 >> 0x17,0) & 1) << 2 |
             (ushort)(SUB161(auVar29 >> 0x1f,0) & 1) << 3 |
             (ushort)(SUB161(auVar29 >> 0x27,0) & 1) << 4 |
             (ushort)(SUB161(auVar29 >> 0x2f,0) & 1) << 5 |
             (ushort)(SUB161(auVar29 >> 0x37,0) & 1) << 6 |
             (ushort)(SUB161(auVar29 >> 0x3f,0) & 1) << 7 |
             (ushort)(SUB161(auVar29 >> 0x47,0) & 1) << 8 |
             (ushort)(SUB161(auVar29 >> 0x4f,0) & 1) << 9 |
             (ushort)(SUB161(auVar29 >> 0x57,0) & 1) << 10 |
             (ushort)(SUB161(auVar29 >> 0x5f,0) & 1) << 0xb |
             (ushort)(SUB161(auVar29 >> 0x67,0) & 1) << 0xc |
             (ushort)(SUB161(auVar29 >> 0x6f,0) & 1) << 0xd |
             (ushort)(SUB161(auVar29 >> 0x77,0) & 1) << 0xe | (ushort)(auVar29[0xf] >> 7) << 0xf;
    uVar7 = (uint)uVar17;
    while (uVar17 != 0) {
      uVar8 = 0;
      for (uVar4 = uVar7; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar8 = uVar8 + 1;
      }
      if (*(longlong *)(*local_88 + -0x98 + (uVar8 + uVar12 & local_88[1]) * -0x98) == lVar15)
      goto LAB_1412b52d0;
      uVar17 = (ushort)(uVar7 - 1) & (ushort)uVar7;
      uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar17);
    }
    auVar26[0] = -(auVar1[0] == cVar32);
    auVar26[1] = -(auVar1[1] == cVar33);
    auVar26[2] = -(auVar1[2] == cVar34);
    auVar26[3] = -(auVar1[3] == cVar35);
    auVar26[4] = -(auVar1[4] == cVar36);
    auVar26[5] = -(auVar1[5] == cVar37);
    auVar26[6] = -(auVar1[6] == cVar38);
    auVar26[7] = -(auVar1[7] == cVar39);
    auVar26[8] = -(auVar1[8] == cVar40);
    auVar26[9] = -(auVar1[9] == cVar41);
    auVar26[10] = -(auVar1[10] == cVar42);
    auVar26[0xb] = -(auVar1[0xb] == cVar43);
    auVar26[0xc] = -(auVar1[0xc] == cVar44);
    auVar26[0xd] = -(auVar1[0xd] == cVar45);
    auVar26[0xe] = -(auVar1[0xe] == cVar46);
    auVar26[0xf] = -(auVar1[0xf] == cVar47);
    if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar26 >> 0x37,0) & 1) != 0) || (SUB161(auVar26 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar26 >> 0x47,0) & 1) != 0) || (SUB161(auVar26 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar26 >> 0x57,0) & 1) != 0) || (SUB161(auVar26 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar26 >> 0x67,0) & 1) != 0) || (SUB161(auVar26 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar26 >> 0x77,0) & 1) != 0) || auVar26[0xf] < '\0') break;
    uVar12 = uVar12 + lVar14 + 0x10;
    lVar14 = lVar14 + 0x10;
  }
  if (local_88[2] == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators,std::hash::random::RandomState>_>
              ();
    lVar15 = local_168._0_8_;
  }
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_00 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_b8 = *extraout_RAX_00;
  lStack_b0 = extraout_RAX_00[1];
  *extraout_RAX_00 = *extraout_RAX_00 + 1;
  local_c8 = 0;
  uStack_c0 = 0;
  local_d8 = (longlong *)&anon_bf4ed97acacab9c6e71765005891de1b_1_llvm_1767052282543269069;
  pcStack_d0 = (code *)0x0;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_01 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  lVar14 = *extraout_RAX_01;
  lStack_140 = extraout_RAX_01[1];
  *extraout_RAX_01 = *extraout_RAX_01 + 1;
  local_148 = (int)lVar14;
  uStack_144 = (undefined4)((ulonglong)lVar14 >> 0x20);
  local_168._8_8_ = 0;
  local_168._0_8_ = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
  local_158 = 0;
  uStack_150 = 0;
  core::ops::function::FnOnce::
  call_once<std::hash::random::impl_0::new::KEYS::constant_0::closure_env_1,tuple_<enum2_<core::option::Option<ref_mut_<enum2_<core::option::Option<core::cell::Cell<tuple_<u64,u64>_>_>_>_>_>_>_>_>
            ();
  if (extraout_RAX_02 == (longlong *)0x0) {
                    // WARNING: Subroutine does not return
    std::thread::local::panic_access_error();
  }
  local_1e8 = *extraout_RAX_02;
  lStack_1e0 = extraout_RAX_02[1];
  *extraout_RAX_02 = *extraout_RAX_02 + 1;
  local_218 = local_b8;
  lStack_210 = lStack_b0;
  local_228 = local_c8;
  uStack_220 = uStack_c0;
  local_238 = local_d8;
  pcStack_230 = pcStack_d0;
  local_258._8_8_ = uStack_150;
  local_258._0_8_ = local_158;
  local_248 = CONCAT44(uStack_144,local_148);
  local_268 = local_168;
  lStack_240 = lStack_140;
  local_1f8 = 0;
  uStack_1f0 = 0;
  local_208 = &anon_335873a3c4aec110b80141a54edc665a_1_llvm_11115642958688891036;
  uStack_200 = 0;
  decorator_storage::Decorators::tracked_clone_from<tuple_<>_>();
  memcpy(local_168,local_268,0x90);
  plVar9 = local_88;
  pauVar11 = (undefined1 (*) [16])*local_88;
  uVar12 = local_88[1];
  uVar19 = extraout_RAX_10 & uVar12;
  auVar22 = *(undefined1 (*) [16])(*pauVar11 + uVar19);
  uVar17 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1 |
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
           (ushort)(SUB161(auVar22 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar22[0xf] >> 7) << 0xf
  ;
  uVar7 = (uint)uVar17;
  if (uVar17 == 0) {
    lVar14 = 0x10;
    do {
      uVar19 = uVar19 + lVar14 & uVar12;
      auVar22 = *(undefined1 (*) [16])(*pauVar11 + uVar19);
      lVar14 = lVar14 + 0x10;
      uVar7 = (uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
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
    } while (uVar7 == 0);
  }
  uVar8 = 0;
  for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
    uVar8 = uVar8 + 1;
  }
  uVar19 = uVar8 + uVar19 & uVar12;
  bVar20 = (*pauVar11)[uVar19];
  if (-1 < (char)bVar20) {
    auVar22 = *pauVar11;
    uVar7 = 0;
    for (uVar8 = (uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
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
                               (ushort)(byte)(auVar22[0xf] >> 7) << 0xf); (uVar8 & 1) == 0;
        uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar7 = uVar7 + 1;
    }
    uVar19 = (ulonglong)uVar7;
    bVar20 = (*pauVar11)[uVar19];
  }
  bVar10 = bVar10 >> 1;
  (*pauVar11)[uVar19] = bVar10;
  pauVar11[1][uVar19 - 0x10 & uVar12] = bVar10;
  local_88[2] = local_88[2] - (ulonglong)(bVar20 & 1);
  *(longlong *)((longlong)pauVar11 + uVar19 * -0x98 + -0x98) = lVar15;
  memcpy((void *)((longlong)pauVar11 + uVar19 * -0x98 + -0x90),local_168,0x90);
  plVar9[3] = plVar9[3] + 1;
LAB_1412b582f:
  pauVar16 = *(undefined1 (**) [16])(local_98 + 0x30);
  lVar14 = *(longlong *)(local_98 + 0x48);
  auVar22 = *pauVar16;
  uVar7 = ~(uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
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
  pauVar11 = pauVar16 + 1;
LAB_1412b59f2:
  if (lVar14 != 0) {
    if ((short)uVar7 == 0) {
      do {
        auVar22 = *pauVar11;
        uVar17 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
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
        pauVar16 = pauVar16 + -0x50;
        pauVar11 = pauVar11 + 1;
      } while (uVar17 == 0xffff);
      uVar8 = ~(uint)uVar17;
      uVar7 = uVar8 & -(uint)uVar17 - 2;
    }
    else {
      uVar8 = uVar7;
      uVar7 = uVar7 - 1 & uVar7;
    }
    lVar14 = lVar14 + -1;
    uVar4 = 0;
    for (; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar4 = uVar4 + 1;
    }
    local_8c = *(int *)pauVar16[(ulonglong)uVar4 * -5 + -5];
    if (plVar2[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar15 = *plVar2;
      bVar10 = (byte)(extraout_RAX_15 >> 0x39);
      auVar22 = ZEXT216(CONCAT11(bVar10,bVar10));
      auVar22 = pshuflw(auVar22,auVar22,0);
      lVar13 = 0;
      uVar12 = extraout_RAX_15;
      while( true ) {
        uVar12 = plVar2[1] & uVar12;
        auVar1 = *(undefined1 (*) [16])(lVar15 + uVar12);
        cVar21 = auVar22[0];
        auVar30[0] = -(auVar1[0] == cVar21);
        cVar23 = auVar22[1];
        auVar30[1] = -(auVar1[1] == cVar23);
        cVar24 = auVar22[2];
        auVar30[2] = -(auVar1[2] == cVar24);
        cVar25 = auVar22[3];
        auVar30[3] = -(auVar1[3] == cVar25);
        auVar30[4] = -(auVar1[4] == cVar21);
        auVar30[5] = -(auVar1[5] == cVar23);
        auVar30[6] = -(auVar1[6] == cVar24);
        auVar30[7] = -(auVar1[7] == cVar25);
        auVar30[8] = -(auVar1[8] == cVar21);
        auVar30[9] = -(auVar1[9] == cVar23);
        auVar30[10] = -(auVar1[10] == cVar24);
        auVar30[0xb] = -(auVar1[0xb] == cVar25);
        auVar30[0xc] = -(auVar1[0xc] == cVar21);
        auVar30[0xd] = -(auVar1[0xd] == cVar23);
        auVar30[0xe] = -(auVar1[0xe] == cVar24);
        auVar30[0xf] = -(auVar1[0xf] == cVar25);
        uVar17 = (ushort)(SUB161(auVar30 >> 7,0) & 1) | (ushort)(SUB161(auVar30 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar30 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar30 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar30 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar30 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar30 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar30 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar30 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar30 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar30 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar30 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar30 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar30 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar30 >> 0x77,0) & 1) << 0xe | (ushort)(auVar30[0xf] >> 7) << 0xf
        ;
        uVar8 = (uint)uVar17;
        while (uVar17 != 0) {
          uVar4 = 0;
          for (uVar5 = uVar8; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          uVar19 = uVar4 + uVar12 & plVar2[1];
          if (local_8c == *(int *)(lVar15 + -0x18 + uVar19 * -0x18)) {
            cVar21 = *(char *)(lVar15 + -4 + uVar19 * -0x18);
            if (cVar21 == '\x11') goto LAB_1412b59f2;
            lVar15 = lVar15 + uVar19 * -0x18;
            local_1a8 = *(undefined8 *)(lVar15 + -0xc);
            local_19d = *(undefined1 *)(lVar15 + -1);
            local_19f = *(undefined2 *)(lVar15 + -3);
            local_1a0 = cVar21;
            systems::collision::world::RaycastWorld::get_collider();
            if (local_148 == 0) {
              local_168._8_8_ = 1;
              local_168._0_8_ = &DAT_142c2bd28;
              local_158 = 8;
              uStack_150 = 0;
              local_148 = 0;
              uStack_144 = 0;
                    // WARNING: Subroutine does not return
              core::panicking::panic_fmt();
            }
            local_1c8 = local_158;
            uStack_1c0 = uStack_150;
            local_1d8 = local_168._0_8_;
            uStack_1d0 = local_168._8_8_;
            systems::collision::world::RaycastWorld::get_collider_mesh();
            pcVar3 = (code *)extraout_RAX_14[3];
            LOCK();
            lVar15 = *(longlong *)pcVar3;
            *(longlong *)pcVar3 = *(longlong *)pcVar3 + 1;
            UNLOCK();
            if (*(longlong *)pcVar3 == 0 || SCARRY8(lVar15,1) != *(longlong *)pcVar3 < 0)
            goto LAB_1412b5ea8;
            local_138 = (longlong *)extraout_RAX_14[2];
            lStack_140 = extraout_RAX_14[1];
            local_148 = (int)*extraout_RAX_14;
            uStack_144 = (undefined4)((ulonglong)*extraout_RAX_14 >> 0x20);
            local_158 = local_1c8;
            uStack_150 = uStack_1c0;
            local_168._8_8_ = uStack_1d0;
            local_168._0_8_ = local_1d8;
            pcStack_130 = pcVar3;
            hashbrown::map::HashMap::
            insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                      ();
            if ((char)local_248 != '\x11') {
              LOCK();
              *(longlong *)pcStack_230 = *(longlong *)pcStack_230 + -1;
              UNLOCK();
              if (*(longlong *)pcStack_230 == 0) {
                alloc::sync::Arc::
                drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                          ();
              }
            }
            goto LAB_1412b59f2;
          }
          uVar17 = (ushort)(uVar8 - 1) & (ushort)uVar8;
          uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar17);
        }
        auVar27[0] = -(auVar1[0] == cVar32);
        auVar27[1] = -(auVar1[1] == cVar33);
        auVar27[2] = -(auVar1[2] == cVar34);
        auVar27[3] = -(auVar1[3] == cVar35);
        auVar27[4] = -(auVar1[4] == cVar36);
        auVar27[5] = -(auVar1[5] == cVar37);
        auVar27[6] = -(auVar1[6] == cVar38);
        auVar27[7] = -(auVar1[7] == cVar39);
        auVar27[8] = -(auVar1[8] == cVar40);
        auVar27[9] = -(auVar1[9] == cVar41);
        auVar27[10] = -(auVar1[10] == cVar42);
        auVar27[0xb] = -(auVar1[0xb] == cVar43);
        auVar27[0xc] = -(auVar1[0xc] == cVar44);
        auVar27[0xd] = -(auVar1[0xd] == cVar45);
        auVar27[0xe] = -(auVar1[0xe] == cVar46);
        auVar27[0xf] = -(auVar1[0xf] == cVar47);
        if ((((((((((((((((SUB161(auVar27 >> 7,0) & 1) != 0 || (SUB161(auVar27 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar27 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar27 >> 0x1f,0) & 1) != 0) || (SUB161(auVar27 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar27 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar27 >> 0x37,0) & 1) != 0) || (SUB161(auVar27 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar27 >> 0x47,0) & 1) != 0) || (SUB161(auVar27 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar27 >> 0x57,0) & 1) != 0) || (SUB161(auVar27 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar27 >> 0x67,0) & 1) != 0) || (SUB161(auVar27 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar27 >> 0x77,0) & 1) != 0) || auVar27[0xf] < '\0') break;
        uVar12 = uVar12 + lVar13 + 0x10;
        lVar13 = lVar13 + 0x10;
      }
    }
    goto LAB_1412b59f2;
  }
  pauVar16 = *(undefined1 (**) [16])(local_98 + 0x30);
  lVar14 = *(longlong *)(local_98 + 0x48);
  auVar22 = *pauVar16;
  uVar7 = ~(uint)(ushort)((ushort)(SUB161(auVar22 >> 7,0) & 1) |
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
  pauVar11 = pauVar16 + 1;
LAB_1412b5c72:
  if (lVar14 != 0) {
    if ((short)uVar7 == 0) {
      do {
        auVar22 = *pauVar11;
        uVar17 = (ushort)(SUB161(auVar22 >> 7,0) & 1) | (ushort)(SUB161(auVar22 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar22 >> 0x17,0) & 1) << 2 |
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
        pauVar16 = pauVar16 + -0x50;
        pauVar11 = pauVar11 + 1;
      } while (uVar17 == 0xffff);
      uVar8 = ~(uint)uVar17;
      uVar7 = uVar8 & -(uint)uVar17 - 2;
    }
    else {
      uVar8 = uVar7;
      uVar7 = uVar7 - 1 & uVar7;
    }
    lVar14 = lVar14 + -1;
    uVar4 = 0;
    for (; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar4 = uVar4 + 1;
    }
    local_90 = *(int *)pauVar16[(ulonglong)uVar4 * -5 + -5];
    if (plVar2[9] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar15 = plVar2[6];
      bVar10 = (byte)(extraout_RAX_17 >> 0x39);
      auVar22 = ZEXT216(CONCAT11(bVar10,bVar10));
      auVar22 = pshuflw(auVar22,auVar22,0);
      lVar13 = 0;
      uVar12 = extraout_RAX_17;
      do {
        uVar12 = plVar2[7] & uVar12;
        auVar1 = *(undefined1 (*) [16])(lVar15 + uVar12);
        cVar21 = auVar22[0];
        auVar31[0] = -(auVar1[0] == cVar21);
        cVar23 = auVar22[1];
        auVar31[1] = -(auVar1[1] == cVar23);
        cVar24 = auVar22[2];
        auVar31[2] = -(auVar1[2] == cVar24);
        cVar25 = auVar22[3];
        auVar31[3] = -(auVar1[3] == cVar25);
        auVar31[4] = -(auVar1[4] == cVar21);
        auVar31[5] = -(auVar1[5] == cVar23);
        auVar31[6] = -(auVar1[6] == cVar24);
        auVar31[7] = -(auVar1[7] == cVar25);
        auVar31[8] = -(auVar1[8] == cVar21);
        auVar31[9] = -(auVar1[9] == cVar23);
        auVar31[10] = -(auVar1[10] == cVar24);
        auVar31[0xb] = -(auVar1[0xb] == cVar25);
        auVar31[0xc] = -(auVar1[0xc] == cVar21);
        auVar31[0xd] = -(auVar1[0xd] == cVar23);
        auVar31[0xe] = -(auVar1[0xe] == cVar24);
        auVar31[0xf] = -(auVar1[0xf] == cVar25);
        uVar17 = (ushort)(SUB161(auVar31 >> 7,0) & 1) | (ushort)(SUB161(auVar31 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar31 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar31 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar31 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar31 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar31 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar31 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar31 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar31 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar31 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar31 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar31 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar31 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar31 >> 0x77,0) & 1) << 0xe | (ushort)(auVar31[0xf] >> 7) << 0xf
        ;
        uVar8 = (uint)uVar17;
        while (uVar17 != 0) {
          uVar4 = 0;
          for (uVar5 = uVar8; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
            uVar4 = uVar4 + 1;
          }
          uVar19 = uVar4 + uVar12 & plVar2[7];
          if (local_90 == *(int *)(lVar15 + -0x18 + uVar19 * -0x18)) {
            cVar21 = *(char *)(lVar15 + -4 + uVar19 * -0x18);
            if (cVar21 != '\x11') {
              lVar15 = lVar15 + uVar19 * -0x18;
              local_1b8 = *(undefined8 *)(lVar15 + -0xc);
              local_1ad = *(undefined1 *)(lVar15 + -1);
              local_1af = *(undefined2 *)(lVar15 + -3);
              local_1b0 = cVar21;
              systems::collision::world::RaycastWorld::get_collider();
              if (local_148 == 0) {
                local_168._8_8_ = 1;
                local_168._0_8_ = &DAT_142c2bd28;
                local_158 = 8;
                uStack_150 = 0;
                local_148 = 0;
                uStack_144 = 0;
                    // WARNING: Subroutine does not return
                core::panicking::panic_fmt();
              }
              local_c8 = local_158;
              uStack_c0 = uStack_150;
              local_d8 = (longlong *)local_168._0_8_;
              pcStack_d0 = (code *)local_168._8_8_;
              systems::collision::world::RaycastWorld::get_collider_mesh();
              pcVar3 = (code *)extraout_RAX_16[3];
              LOCK();
              lVar15 = *(longlong *)pcVar3;
              *(longlong *)pcVar3 = *(longlong *)pcVar3 + 1;
              UNLOCK();
              if (*(longlong *)pcVar3 == 0 || SCARRY8(lVar15,1) != *(longlong *)pcVar3 < 0) {
LAB_1412b5ea8:
                    // WARNING: Does not return
                pcVar3 = (code *)invalidInstructionException();
                (*pcVar3)();
              }
              local_138 = (longlong *)extraout_RAX_16[2];
              lStack_140 = extraout_RAX_16[1];
              local_148 = (int)*extraout_RAX_16;
              uStack_144 = (undefined4)((ulonglong)*extraout_RAX_16 >> 0x20);
              local_158 = local_c8;
              uStack_150 = uStack_c0;
              local_168._8_8_ = pcStack_d0;
              local_168._0_8_ = local_d8;
              pcStack_130 = pcVar3;
              hashbrown::map::HashMap::
              insert<country_core::resources::walls::decorator::DecoratorId,tuple_<country_core::systems::collision::world::OffsetColliderTransform,country_core::systems::collision::world::RaycastMesh>,std::hash::random::RandomState,alloc::alloc::Global>
                        ();
              if ((char)local_248 != '\x11') {
                LOCK();
                *(longlong *)pcStack_230 = *(longlong *)pcStack_230 + -1;
                UNLOCK();
                if (*(longlong *)pcStack_230 == 0) {
                  alloc::sync::Arc::
                  drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                            ();
                }
              }
            }
            goto LAB_1412b5c72;
          }
          uVar17 = (ushort)(uVar8 - 1) & (ushort)uVar8;
          uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar17);
        }
        auVar28[0] = -(auVar1[0] == cVar32);
        auVar28[1] = -(auVar1[1] == cVar33);
        auVar28[2] = -(auVar1[2] == cVar34);
        auVar28[3] = -(auVar1[3] == cVar35);
        auVar28[4] = -(auVar1[4] == cVar36);
        auVar28[5] = -(auVar1[5] == cVar37);
        auVar28[6] = -(auVar1[6] == cVar38);
        auVar28[7] = -(auVar1[7] == cVar39);
        auVar28[8] = -(auVar1[8] == cVar40);
        auVar28[9] = -(auVar1[9] == cVar41);
        auVar28[10] = -(auVar1[10] == cVar42);
        auVar28[0xb] = -(auVar1[0xb] == cVar43);
        auVar28[0xc] = -(auVar1[0xc] == cVar44);
        auVar28[0xd] = -(auVar1[0xd] == cVar45);
        auVar28[0xe] = -(auVar1[0xe] == cVar46);
        auVar28[0xf] = -(auVar1[0xf] == cVar47);
        if ((((((((((((((((SUB161(auVar28 >> 7,0) & 1) != 0 || (SUB161(auVar28 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar28 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar28 >> 0x1f,0) & 1) != 0) || (SUB161(auVar28 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar28 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar28 >> 0x37,0) & 1) != 0) || (SUB161(auVar28 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar28 >> 0x47,0) & 1) != 0) || (SUB161(auVar28 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar28 >> 0x57,0) & 1) != 0) || (SUB161(auVar28 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar28 >> 0x67,0) & 1) != 0) || (SUB161(auVar28 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar28 >> 0x77,0) & 1) != 0) || auVar28[0xf] < '\0') break;
        uVar12 = uVar12 + lVar13 + 0x10;
        lVar13 = lVar13 + 0x10;
      } while( true );
    }
    goto LAB_1412b5c72;
  }
  goto LAB_1412b52d0;
}

