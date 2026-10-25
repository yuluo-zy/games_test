// ADDR 0x14081c790
// SIG void __cdecl move_decorators_following_anchors(void)
// ORIG system::move_decorators_following_anchors


void __cdecl
system_decorator::move_decorators_following_anchors::move_decorators_following_anchors(void)

{
  byte *pbVar1;
  byte bVar2;
  byte bVar3;
  undefined4 uVar4;
  ulonglong *puVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined1 (*pauVar8) [16];
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  char extraout_AL;
  uint uVar14;
  uint extraout_EAX;
  uint extraout_EAX_00;
  undefined4 extraout_EAX_01;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  longlong lVar15;
  ulonglong uVar16;
  longlong extraout_RAX_02;
  longlong extraout_RAX_03;
  longlong extraout_RAX_04;
  longlong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong extraout_RAX_07;
  ulonglong extraout_RAX_08;
  undefined4 *extraout_RAX_09;
  longlong extraout_RAX_10;
  longlong extraout_RAX_11;
  longlong *extraout_RAX_12;
  longlong *extraout_RAX_13;
  longlong *extraout_RAX_14;
  uint uVar17;
  undefined8 *in_RCX;
  longlong *plVar18;
  undefined8 *in_RDX;
  undefined8 *puVar19;
  ulonglong uVar20;
  undefined1 *puVar21;
  undefined1 *puVar22;
  longlong lVar23;
  ushort uVar24;
  longlong *in_R8;
  char *pcVar25;
  ulonglong uVar26;
  ulonglong uVar27;
  undefined8 *in_R9;
  longlong lVar28;
  longlong lVar29;
  undefined1 (*pauVar30) [16];
  undefined1 (*pauVar31) [16];
  byte bVar32;
  char cVar33;
  char cVar38;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  undefined4 extraout_XMM0_Da_01;
  undefined4 extraout_XMM0_Da_02;
  float fVar34;
  float extraout_XMM0_Da_03;
  char cVar36;
  char cVar37;
  undefined1 auVar35 [16];
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  float fVar41;
  float fVar42;
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  undefined1 unaff_XMM8_Bc;
  undefined1 unaff_XMM8_Bd;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  undefined1 unaff_XMM8_Bg;
  undefined1 unaff_XMM8_Bh;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  undefined1 unaff_XMM8_Bk;
  undefined1 unaff_XMM8_Bl;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  undefined1 unaff_XMM8_Bo;
  undefined1 unaff_XMM8_Bp;
  undefined8 *in_stack_00000028;
  longlong in_stack_00000030;
  longlong in_stack_00000038;
  longlong *in_stack_00000040;
  longlong local_1e8;
  longlong *local_188;
  code *pcStack_180;
  longlong local_168 [2];
  undefined8 local_158;
  undefined1 (*local_150) [16];
  longlong **local_148;
  undefined8 local_140;
  undefined8 uStack_138;
  longlong local_130;
  longlong *local_128;
  ulonglong local_120;
  undefined8 local_118;
  ulonglong *local_110;
  undefined1 local_102 [17];
  char local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar19 = in_RDX;
  local_110 = (ulonglong *)in_R8;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (puVar19 != (undefined8 *)0x0) {
        if (puVar19 < (undefined8 *)0x56) {
          if (*(char *)(puVar19 + 0x285520bf) < -0x40) {
LAB_14081c864:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (puVar19 != (undefined8 *)0x56) goto LAB_14081c864;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (puVar19 != (undefined8 *)0xfffffffffffffffe)) {
        if ((longlong)puVar19 + 2U < 0x56) {
          if (*(char *)((longlong)puVar19 + 0x142a905fa) < -0x40) {
LAB_14081c8b1:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if ((longlong)puVar19 + 2U != 0x56) goto LAB_14081c8b1;
      }
    }
    pcVar25 = "internal error: entered unreachable code";
    do {
      bVar32 = pcVar25[-1];
      uVar14 = (uint)(char)bVar32;
      if ((char)bVar32 < '\0') {
        bVar2 = pcVar25[-2];
        if ((char)bVar2 < -0x40) {
          bVar3 = pcVar25[-3];
          if ((char)bVar3 < -0x40) {
            pbVar1 = (byte *)(pcVar25 + -4);
            pcVar25 = pcVar25 + -4;
            uVar14 = bVar3 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            pcVar25 = pcVar25 + -3;
            uVar14 = bVar3 & 0xf;
          }
          uVar14 = bVar2 & 0x3f | uVar14 << 6;
        }
        else {
          pcVar25 = pcVar25 + -2;
          uVar14 = bVar2 & 0x1f;
        }
        uVar14 = (uint)(bVar32 & 0x3f) | uVar14 << 6;
        if (uVar14 != 0x5c) goto LAB_14081c974;
LAB_14081c985:
        if (pcVar25 == "") break;
        if (pcVar25 + -0x142a90737 < (char *)0x41) {
          if (-0x41 < pcVar25[1]) break;
        }
        else if (pcVar25 + -0x142a90737 == (char *)0x41) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      pcVar25 = pcVar25 + -1;
      if (uVar14 == 0x5c) goto LAB_14081c985;
LAB_14081c974:
      if (uVar14 == 0x2f) goto LAB_14081c985;
    } while (pcVar25 !=
             "crates/systems/decorator/src/move_decorators_following_anchors.rsinternal error: entered unreachable code"
            );
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar18 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_13, extraout_RAX_13 == (longlong *)0x0))
    goto LAB_14081d5bc;
    if (*plVar18 != 0) goto LAB_14081d602;
    *plVar18 = -1;
    local_102._2_8_ = plVar18;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)local_102._2_8_ = *(longlong *)local_102._2_8_ + 1;
  }
  puVar5 = (ulonglong *)*in_RDX;
  lVar23 = in_RDX[1];
  uVar27 = *puVar5;
  uVar20 = 0;
  uVar16 = uVar27 - *(ulonglong *)(lVar23 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x18)) {
    uVar16 = uVar20;
  }
  uVar26 = uVar27 - *(ulonglong *)(lVar23 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x38)) {
    uVar26 = uVar20;
  }
  lVar15 = uVar16 * 0x10 + *(longlong *)(lVar23 + 8);
  uVar27 = *(ulonglong *)(lVar23 + 0x10) - uVar16;
  if (*(ulonglong *)(lVar23 + 0x10) < uVar16) {
    lVar15 = 8;
    uVar27 = uVar20;
  }
  lVar29 = uVar26 * 0x10 + *(longlong *)(lVar23 + 0x28);
  uVar16 = *(ulonglong *)(lVar23 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar23 + 0x30) < uVar26) {
    lVar29 = 8;
    uVar16 = uVar20;
  }
  *puVar5 = (*(longlong *)(lVar23 + 0x40) - uVar16) - uVar27;
  lVar23 = uVar27 * 0x10 + lVar15;
  lVar28 = uVar16 * 0x10 + lVar29;
  local_120 = CONCAT44(local_120._4_4_,*(undefined4 *)((longlong)in_stack_00000040 + 0x1c));
  local_102._2_8_ = *in_stack_00000040;
  local_118 = (undefined4 *)in_stack_00000040[2];
  local_f1 = extraout_AL;
  do {
    if ((lVar15 == 0) || (lVar15 == lVar23)) {
      if ((lVar29 == 0) || (lVar29 == lVar28)) break;
      lVar29 = lVar29 + 0x10;
      lVar15 = 0;
    }
    else {
      lVar15 = lVar15 + 0x10;
    }
    *puVar5 = *puVar5 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::on_wall_removed();
  } while( true );
  country_core::resources::walls::decorator_subtype::impl$8::from();
  puVar5 = (ulonglong *)*local_110;
  lVar23 = local_110[1];
  uVar27 = *puVar5;
  uVar20 = 0;
  uVar16 = uVar27 - *(ulonglong *)(lVar23 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x18)) {
    uVar16 = uVar20;
  }
  uVar26 = uVar27 - *(ulonglong *)(lVar23 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x38)) {
    uVar26 = uVar20;
  }
  lVar15 = uVar16 * 0x10 + *(longlong *)(lVar23 + 8);
  uVar27 = *(ulonglong *)(lVar23 + 0x10) - uVar16;
  if (*(ulonglong *)(lVar23 + 0x10) < uVar16) {
    lVar15 = 8;
    uVar27 = uVar20;
  }
  lVar29 = uVar26 * 0x10 + *(longlong *)(lVar23 + 0x28);
  uVar16 = *(ulonglong *)(lVar23 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar23 + 0x30) < uVar26) {
    lVar29 = 8;
    uVar16 = uVar20;
  }
  *puVar5 = (*(longlong *)(lVar23 + 0x40) - uVar16) - uVar27;
  lVar28 = uVar27 * 0x10 + lVar15;
  lVar23 = uVar16 * 0x10 + lVar29;
  local_128 = (longlong *)CONCAT44(local_128._4_4_,*(undefined4 *)(in_stack_00000030 + 0x1c));
  puVar6 = *(undefined4 **)(in_stack_00000030 + 0x10);
  uVar4 = *(undefined4 *)(in_stack_00000038 + 0x1c);
  puVar7 = *(undefined4 **)(in_stack_00000038 + 0x10);
  local_110 = puVar5;
  do {
    if ((lVar15 == 0) || (lVar15 == lVar28)) {
      if ((lVar29 == 0) || (lVar29 == lVar23)) break;
      lVar29 = lVar29 + 0x10;
      lVar15 = 0;
    }
    else {
      lVar15 = lVar15 + 0x10;
    }
    *local_110 = *local_110 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX_02 != 0) {
      country_core::resources::walls::decorator_storage::Decorators::mark_all_modified();
    }
    *puVar6 = local_128._0_4_;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    *puVar7 = uVar4;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
  } while( true );
  puVar5 = (ulonglong *)*in_R9;
  lVar23 = in_R9[1];
  uVar27 = *puVar5;
  uVar20 = 0;
  uVar16 = uVar27 - *(ulonglong *)(lVar23 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x18)) {
    uVar16 = uVar20;
  }
  uVar26 = uVar27 - *(ulonglong *)(lVar23 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x38)) {
    uVar26 = uVar20;
  }
  lVar15 = uVar16 * 0x20 + *(longlong *)(lVar23 + 8);
  uVar27 = *(ulonglong *)(lVar23 + 0x10) - uVar16;
  if (*(ulonglong *)(lVar23 + 0x10) < uVar16) {
    lVar15 = 8;
    uVar27 = uVar20;
  }
  lVar29 = uVar26 * 0x20 + *(longlong *)(lVar23 + 0x28);
  uVar16 = *(ulonglong *)(lVar23 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar23 + 0x30) < uVar26) {
    lVar29 = 8;
    uVar16 = uVar20;
  }
  *puVar5 = (*(longlong *)(lVar23 + 0x40) - uVar16) - uVar27;
  local_110 = (ulonglong *)(uVar27 * 0x20 + lVar15);
  local_128 = (longlong *)(uVar16 * 0x20 + lVar29);
  do {
    if ((lVar15 == 0) || ((ulonglong *)lVar15 == local_110)) {
      if ((lVar29 == 0) || ((longlong *)lVar29 == local_128)) break;
      lVar29 = lVar29 + 0x20;
      lVar15 = 0;
    }
    else {
      lVar15 = lVar15 + 0x20;
    }
    *puVar5 = *puVar5 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX_03 != 0) {
      local_158 = *(undefined ***)(extraout_RAX_03 + 0x30);
      local_148 = (longlong **)
                  (*(undefined1 (*) [16])local_158 + *(longlong *)(extraout_RAX_03 + 0x38) + 1);
      auVar35 = *(undefined1 (*) [16])local_158;
      local_140 = CONCAT62(local_140._2_6_,
                           ~((ushort)(SUB161(auVar35 >> 7,0) & 1) |
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
                            (ushort)(byte)(auVar35[0xf] >> 7) << 0xf));
      local_150 = (undefined1 (*) [16])((longlong)local_158 + 0x10);
      uStack_138 = *(ulonglong *)(extraout_RAX_03 + 0x48);
      local_130 = extraout_RAX_03 + 0x60;
      while (country_core::resources::walls::decorator_storage::impl$3::next(), local_1e8 != 0) {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (*(char *)(extraout_RAX_04 + 0x22) == '\x02') {
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
        }
      }
    }
  } while( true );
  puVar5 = (ulonglong *)*in_stack_00000028;
  lVar23 = in_stack_00000028[1];
  uVar27 = *puVar5;
  uVar20 = 0;
  uVar16 = uVar27 - *(ulonglong *)(lVar23 + 0x18);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x18)) {
    uVar16 = uVar20;
  }
  uVar26 = uVar27 - *(ulonglong *)(lVar23 + 0x38);
  if (uVar27 < *(ulonglong *)(lVar23 + 0x38)) {
    uVar26 = uVar20;
  }
  lVar15 = uVar16 * 0x30 + *(longlong *)(lVar23 + 8);
  uVar27 = *(ulonglong *)(lVar23 + 0x10) - uVar16;
  if (*(ulonglong *)(lVar23 + 0x10) < uVar16) {
    lVar15 = 8;
    uVar27 = uVar20;
  }
  lVar29 = uVar26 * 0x30 + *(longlong *)(lVar23 + 0x28);
  uVar16 = *(ulonglong *)(lVar23 + 0x30) - uVar26;
  if (*(ulonglong *)(lVar23 + 0x30) < uVar26) {
    lVar29 = 8;
    uVar16 = uVar20;
  }
  *puVar5 = (*(longlong *)(lVar23 + 0x40) - uVar16) - uVar27;
  local_110 = (ulonglong *)(uVar27 * 0x30 + lVar15);
  local_128 = (longlong *)(uVar16 * 0x30 + lVar29);
  do {
    if ((lVar15 == 0) || ((ulonglong *)lVar15 == local_110)) {
      if ((lVar29 == 0) || ((longlong *)lVar29 == local_128)) goto LAB_14081cfdf;
      lVar29 = lVar29 + 0x30;
      lVar15 = 0;
    }
    else {
      lVar15 = lVar15 + 0x30;
    }
    *puVar5 = *puVar5 + 1;
    *local_118 = (undefined4)local_120;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX_05 != 0) {
      local_158 = *(undefined ***)(extraout_RAX_05 + 0x30);
      local_148 = (longlong **)
                  (*(undefined1 (*) [16])local_158 + *(longlong *)(extraout_RAX_05 + 0x38) + 1);
      auVar35 = *(undefined1 (*) [16])local_158;
      local_140 = CONCAT62(local_140._2_6_,
                           ~((ushort)(SUB161(auVar35 >> 7,0) & 1) |
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
                            (ushort)(byte)(auVar35[0xf] >> 7) << 0xf));
      local_150 = (undefined1 (*) [16])((longlong)local_158 + 0x10);
      uStack_138 = *(ulonglong *)(extraout_RAX_05 + 0x48);
      local_130 = extraout_RAX_05 + 0x60;
      while (country_core::resources::walls::decorator_storage::impl$3::next(), local_1e8 != 0) {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        if (*(char *)(extraout_RAX_06 + 0x22) == '\x02') {
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
        }
      }
    }
  } while( true );
code_r0x00014081d17b:
  uVar24 = (ushort)(uVar14 - 1) & (ushort)uVar14;
  uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar24);
  if (uVar24 == 0) goto LAB_14081d190;
  goto LAB_14081d160;
LAB_14081cfdf:
  *local_118 = (undefined4)local_120;
  country_core::resources::walls::decorator_storage::DecoratorStorage::iter_walls_mut();
  local_110 = (ulonglong *)*in_RCX;
  local_128 = (longlong *)(local_110 + 4);
  iVar10 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                            CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                  CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  iVar11 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  iVar12 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  uVar14 = (uint)(ushort)local_140;
  pauVar31 = (undefined1 (*) [16])local_158;
  pauVar30 = local_150;
  puVar6 = local_118;
  local_120 = uStack_138;
  while (local_120 != 0) {
    if ((short)uVar14 == 0) {
      do {
        auVar35 = *pauVar30;
        uVar24 = (ushort)(SUB161(auVar35 >> 7,0) & 1) | (ushort)(SUB161(auVar35 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar35 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(byte)(auVar35[0xf] >> 7) << 0xf;
        pauVar31 = pauVar31 + -0x98;
        pauVar30 = pauVar30 + 1;
      } while (uVar24 == 0xffff);
      uVar14 = ~(uint)uVar24;
      uVar17 = uVar14 & -(uint)uVar24 - 2;
    }
    else {
      if (pauVar31 == (undefined1 (*) [16])0x0) break;
      uVar17 = uVar14 - 1 & uVar14;
    }
    local_118 = (undefined4 *)CONCAT44((int)((ulonglong)puVar6 >> 0x20),uVar17);
    uVar17 = 0;
    for (; (uVar14 & 1) == 0; uVar14 = uVar14 >> 1 | 0x80000000) {
      uVar17 = uVar17 + 1;
    }
    local_168[0] = *(longlong *)((longlong)pauVar31 + (ulonglong)uVar17 * -0x98 + -0x98);
    country_core::resources::walls::decorator_storage::Decorators::iter_modified_mut();
    country_core::resources::walls::decorator_storage::impl$4::len();
    puVar5 = local_110;
    local_120 = local_120 - 1;
    if (extraout_RAX_07 != 0) {
      if (local_110[3] == 0) {
LAB_14081d531:
        local_188 = local_168;
        pcStack_180 = country_core::resources::walls::impl$7::fmt;
        local_158 = &PTR_s_The_list_of_decorators_contains_a_142a90808;
        local_150 = (undefined1 (*) [16])0x1;
        uStack_138 = 0;
        local_140 = 1;
        local_148 = &local_188;
                    // WARNING: Subroutine does not return
        core::panicking::panic_fmt();
      }
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      lVar23 = *puVar5;
      puVar21 = (undefined1 *)puVar5[1];
      bVar32 = (byte)(extraout_RAX_08 >> 0x39);
      auVar35 = ZEXT216(CONCAT11(bVar32,bVar32));
      auVar35 = pshuflw(auVar35,auVar35,0);
      lVar15 = 0;
      uVar27 = extraout_RAX_08;
      do {
        uVar27 = (ulonglong)puVar21 & uVar27;
        pcVar25 = (char *)(lVar23 + uVar27);
        cVar33 = auVar35[0];
        auVar40[0] = -(*pcVar25 == cVar33);
        cVar36 = auVar35[1];
        auVar40[1] = -(pcVar25[1] == cVar36);
        cVar37 = auVar35[2];
        auVar40[2] = -(pcVar25[2] == cVar37);
        cVar38 = auVar35[3];
        auVar40[3] = -(pcVar25[3] == cVar38);
        auVar40[4] = -(pcVar25[4] == cVar33);
        auVar40[5] = -(pcVar25[5] == cVar36);
        auVar40[6] = -(pcVar25[6] == cVar37);
        auVar40[7] = -(pcVar25[7] == cVar38);
        auVar40[8] = -(pcVar25[8] == cVar33);
        auVar40[9] = -(pcVar25[9] == cVar36);
        auVar40[10] = -(pcVar25[10] == cVar37);
        auVar40[0xb] = -(pcVar25[0xb] == cVar38);
        auVar40[0xc] = -(pcVar25[0xc] == cVar33);
        auVar40[0xd] = -(pcVar25[0xd] == cVar36);
        auVar40[0xe] = -(pcVar25[0xe] == cVar37);
        auVar40[0xf] = -(pcVar25[0xf] == cVar38);
        uVar24 = (ushort)(SUB161(auVar40 >> 7,0) & 1) | (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar40 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar40 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar40 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar40 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar40 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar40 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar40 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar40 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar40 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar40 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar40 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe | (ushort)(auVar40[0xf] >> 7) << 0xf
        ;
        uVar14 = (uint)uVar24;
        if (uVar24 != 0) {
LAB_14081d160:
          uVar17 = 0;
          for (uVar9 = uVar14; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
            uVar17 = uVar17 + 1;
          }
          lVar29 = (uVar17 + uVar27 & (ulonglong)puVar21) * -0x2a8;
          if (local_168[0] != *(longlong *)(lVar23 + -0x2a8 + lVar29)) goto code_r0x00014081d17b;
          lVar15 = lVar23 + lVar29;
          local_102._2_8_ = lVar23 + lVar29 + -0x2a0;
          while (country_core::resources::walls::decorator_storage::impl$5::next(),
                puVar21 != (undefined1 *)0x0) {
            puVar22 = (undefined1 *)(ulonglong)extraout_EAX;
            lVar23 = (ulonglong)(byte)puVar21[0x22] - 1;
            if (2 < (byte)(puVar21[0x22] - 2)) {
              lVar23 = 0;
            }
            if (lVar23 == 0) {
              country_core::resources::walls::decorator::DecoratorDst::as_wall_mut();
              country_core::resources::walls::public_wall_state::PublicWallState::
              get_wall_attachment_height();
              ::utils::curve::Curve::
              get_approx_coord_from_pos<glam::f32::vec2::Vec2,utils::curve::CurveCoordSemantic2D>();
              puVar21 = *(undefined1 **)(lVar15 + -0x278);
              if (puVar21 <= (undefined1 *)(ulonglong)extraout_EAX_00) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              if (puVar21 <= (undefined1 *)(ulonglong)extraout_EAX_00 + 1) {
                    // WARNING: Subroutine does not return
                core::panicking::panic_bounds_check();
              }
              if ((extraout_XMM0_Da_00 < 0.0) || (1.0 < extraout_XMM0_Da_00)) {
                    // WARNING: Subroutine does not return
                core::panicking::panic();
              }
              ::utils::lerp();
              fVar41 = (float)extraout_RAX_09[4];
              country_core::resources::walls::public_wall_state::PublicWallState::wall_space();
              local_158 = (undefined **)CONCAT44(extraout_XMM0_Da_01,extraout_EAX_01);
              ::utils::wall_space::impl$1::project_clamp();
              *extraout_RAX_09 = extraout_XMM0_Da_02;
              extraout_RAX_09[1] = extraout_XMM0_Da + fVar41;
              if ((*(char *)((longlong)extraout_RAX_09 + 0x21) == '\x01') &&
                 (country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::
                  try_get_subtype(), puVar21 = puVar22, extraout_RAX_10 != 0)) {
                pauVar8 = *(undefined1 (**) [16])(lVar15 + -600);
                if (pauVar8 == (undefined1 (*) [16])0x1) {
                    // WARNING: Subroutine does not return
                  core::option::unwrap_failed();
                }
                if (pauVar8 == (undefined1 (*) [16])0x0) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic();
                }
                local_158 = *(undefined ***)(lVar15 + -0x260);
                local_148 = (longlong **)
                            core::ops::function::FnOnce::
                            call_once<utils::curve::curve_slice::impl_5::remap_2d::closure_env_0<slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemanticThresholded3D,glam::f32::vec3::Vec3_(*)(usize,ref_<glam::f32::vec3::Vec3>)>,tuple_<usize,ref_<glam::f32::vec3::Vec3>_>_>
                ;
                local_140 = 0;
                uStack_138 = 0;
                local_150 = pauVar8;
                ::utils::curve::curve_slice::CurveSlice::
                extend_right_until<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>),utils::curve::curve_slice::impl_6::extend_right::closure_env_0<glam::f32::vec3::Vec3,slice2_<glam::f32::vec3::Vec3>,utils::curve::CurveCoordSemantic3D,glam::f32::vec2::Vec2,glam::f32::vec2::Vec2_(*)(usize,ref_<glam::f32::vec3::Vec3>)>_>
                          ();
                uVar27 = uStack_138 & 0xffffffff;
                if (*(ulonglong *)(lVar15 + -600) <= uVar27) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                if (*(ulonglong *)(lVar15 + -600) <= uVar27 + 1) {
                    // WARNING: Subroutine does not return
                  core::panicking::panic_bounds_check();
                }
                fVar42 = uStack_138._4_4_;
                fVar41 = *(float *)(*(longlong *)(lVar15 + -0x260) + 4 + uVar27 * 0xc);
                fVar34 = *(float *)(*(longlong *)(lVar15 + -0x260) + 4 + (uVar27 + 1) * 0xc);
                country_core::resources::walls::decorator::DecoratorRank::new();
                country_core::resources::walls::decorator_subtype::DecoratorLibKey::new();
                puVar21 = local_102;
                std::collections::hash::map::HashMap::
                get<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>_>
                          ();
                if (extraout_RAX_11 == 0) {
                  local_188 = (longlong *)local_102;
                  pcStack_180 = country_core::resources::walls::decorator_subtype::impl$25::fmt;
                  local_158 = &anon_aaa2ffd9936f24d9e01572d66e2542e3_34_llvm_7518301806615881319;
                  local_150 = (undefined1 (*) [16])0x1;
                  uStack_138 = 0;
                  local_140 = 1;
                  local_148 = &local_188;
                    // WARNING: Subroutine does not return
                  core::panicking::panic_fmt();
                }
                fVar41 = fVar42 * (fVar34 - fVar41) + fVar41;
                if (fVar41 <= 0.0) {
                  fVar41 = 0.0;
                }
                fVar34 = (*(float *)(extraout_RAX_11 + 0x3c) - *(float *)(extraout_RAX_11 + 0x38)) *
                         0.5;
                fVar42 = fVar41 + fVar34 + 0.05;
                fVar41 = fVar34;
                if (fVar34 <= fVar42) {
                  fVar41 = fVar42;
                }
                fVar41 = (float)(-(uint)NAN(fVar42) & (uint)fVar34 |
                                ~-(uint)NAN(fVar42) & (uint)fVar41);
                extraout_RAX_09[6] = fVar41;
                country_core::resources::walls::public_wall_state::PublicWallState::max_y();
                extraout_RAX_09[4] = fVar41 - extraout_XMM0_Da_03;
                extraout_RAX_09[1] = fVar41;
              }
            }
            else if (lVar23 != 1) {
                    // WARNING: Subroutine does not return
              core::panicking::panic();
            }
          }
          break;
        }
LAB_14081d190:
        auVar39[0] = -(*pcVar25 == (char)iVar10);
        auVar39[1] = -(pcVar25[1] == (char)((uint)iVar10 >> 8));
        auVar39[2] = -(pcVar25[2] == (char)((uint)iVar10 >> 0x10));
        auVar39[3] = -(pcVar25[3] == (char)((uint)iVar10 >> 0x18));
        auVar39[4] = -(pcVar25[4] == (char)iVar11);
        auVar39[5] = -(pcVar25[5] == (char)((uint)iVar11 >> 8));
        auVar39[6] = -(pcVar25[6] == (char)((uint)iVar11 >> 0x10));
        auVar39[7] = -(pcVar25[7] == (char)((uint)iVar11 >> 0x18));
        auVar39[8] = -(pcVar25[8] == (char)iVar12);
        auVar39[9] = -(pcVar25[9] == (char)((uint)iVar12 >> 8));
        auVar39[10] = -(pcVar25[10] == (char)((uint)iVar12 >> 0x10));
        auVar39[0xb] = -(pcVar25[0xb] == (char)((uint)iVar12 >> 0x18));
        auVar39[0xc] = -(pcVar25[0xc] == (char)iVar13);
        auVar39[0xd] = -(pcVar25[0xd] == (char)((uint)iVar13 >> 8));
        auVar39[0xe] = -(pcVar25[0xe] == (char)((uint)iVar13 >> 0x10));
        auVar39[0xf] = -(pcVar25[0xf] == (char)((uint)iVar13 >> 0x18));
        if ((((((((((((((((SUB161(auVar39 >> 7,0) & 1) != 0 || (SUB161(auVar39 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar39 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar39 >> 0x1f,0) & 1) != 0) || (SUB161(auVar39 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar39 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar39 >> 0x37,0) & 1) != 0) || (SUB161(auVar39 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar39 >> 0x47,0) & 1) != 0) || (SUB161(auVar39 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar39 >> 0x57,0) & 1) != 0) || (SUB161(auVar39 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar39 >> 0x67,0) & 1) != 0) || (SUB161(auVar39 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar39 >> 0x77,0) & 1) != 0) || auVar39[0xf] < '\0') goto LAB_14081d531;
        uVar27 = uVar27 + lVar15 + 0x10;
        lVar15 = lVar15 + 0x10;
      } while( true );
    }
    uVar14 = (uint)local_118;
    puVar6 = local_118;
  }
  if (local_f1 != '\0') {
    local_118 = puVar6;
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_12 == 1) {
      plVar18 = extraout_RAX_12 + 1;
    }
    else if ((*extraout_RAX_12 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar18 = extraout_RAX_14, extraout_RAX_14 == (longlong *)0x0)) {
LAB_14081d5bc:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar18 != 0) {
LAB_14081d602:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar18 = -1;
    local_102._2_8_ = plVar18;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_102._2_8_ = *(longlong *)local_102._2_8_ + 1;
  }
  return;
}


