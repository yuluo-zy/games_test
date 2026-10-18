void __cdecl spawn_prefabs(void)


void __cdecl country_core::systems::prefab::spawn_prefabs(void)

{
  byte *pbVar1;
  longlong *plVar2;
  char *pcVar3;
  undefined1 auVar4 [16];
  byte bVar5;
  byte bVar6;
  undefined4 uVar7;
  undefined4 *puVar8;
  undefined **ppuVar9;
  code *pcVar10;
  uint uVar11;
  int iVar12;
  int iVar13;
  int iVar14;
  int iVar15;
  char extraout_AL;
  uint uVar16;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong uVar17;
  undefined8 *puVar18;
  ulonglong extraout_RAX_02;
  longlong lVar19;
  longlong extraout_RAX_03;
  undefined1 (*extraout_RAX_04) [16];
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined8 *in_RCX;
  longlong *plVar20;
  ulonglong *in_RDX;
  ulonglong uVar21;
  ulonglong uVar22;
  undefined8 *puVar23;
  undefined8 *puVar24;
  longlong *in_R8;
  undefined *puVar25;
  ulonglong uVar26;
  ushort uVar27;
  int iVar28;
  uint uVar29;
  ulonglong *puVar30;
  byte bVar31;
  char cVar32;
  char cVar34;
  char cVar35;
  char cVar36;
  undefined1 auVar33 [16];
  undefined1 auVar37 [16];
  undefined1 auVar38 [16];
  undefined1 unaff_XMM6_Ba;
  undefined1 unaff_XMM6_Bb;
  undefined1 unaff_XMM6_Bc;
  undefined1 unaff_XMM6_Bd;
  undefined1 unaff_XMM6_Be;
  undefined1 unaff_XMM6_Bf;
  undefined1 unaff_XMM6_Bg;
  undefined1 unaff_XMM6_Bh;
  undefined1 unaff_XMM6_Bi;
  undefined1 unaff_XMM6_Bj;
  undefined1 unaff_XMM6_Bk;
  undefined1 unaff_XMM6_Bl;
  undefined1 unaff_XMM6_Bm;
  undefined1 unaff_XMM6_Bn;
  undefined1 unaff_XMM6_Bo;
  undefined1 unaff_XMM6_Bp;
  longlong *in_stack_00000028;
  undefined8 *in_stack_00000038;
  undefined8 *in_stack_00000040;
  undefined8 *in_stack_00000048;
  longlong *in_stack_00000050;
  undefined1 local_240 [8];
  ulonglong local_238;
  ulonglong *local_230;
  char *pcStack_228;
  undefined8 local_220;
  undefined8 local_218;
  undefined4 local_210;
  longlong *local_208;
  code *local_200;
  undefined8 local_1f8;
  undefined8 local_1f0;
  undefined8 local_1e8;
  longlong local_1e0;
  undefined4 *local_1d8;
  ulonglong *local_1d0;
  undefined8 *local_1c8;
  longlong *local_1c0;
  undefined8 local_1b0;
  ulonglong local_1a8;
  longlong *local_1a0;
  code *local_198;
  undefined8 local_190;
  undefined8 local_188;
  undefined8 *local_180;
  undefined **local_178;
  char *pcStack_170;
  undefined1 local_168 [16];
  undefined1 local_158 [16];
  longlong local_148;
  char *pcStack_140;
  longlong local_138;
  undefined8 local_130;
  undefined **local_128;
  undefined8 local_120;
  longlong **pplStack_118;
  undefined8 local_110;
  undefined8 uStack_108;
  undefined8 local_100;
  undefined8 uStack_f8;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_e0;
  undefined8 local_d8;
  code *pcStack_d0;
  undefined1 *local_c8;
  code *local_c0;
  undefined4 local_b4;
  longlong local_b0;
  char *pcStack_a8;
  longlong local_a0;
  longlong *local_98;
  longlong *local_90;
  undefined8 *local_88;
  undefined8 *local_80;
  longlong *local_70;
  char local_61;
  undefined8 local_60;
  
  local_60 = 0xfffffffffffffffe;
  puVar30 = in_RDX;
  puffin::are_scopes_on();
  local_61 = extraout_AL;
  if (extraout_AL != '\0') {
    puVar30 = (ulonglong *)&DAT_142a93f80;
    uVar21 = 0x2c;
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar21 != 0) {
        if (uVar21 < 0x2c) {
          if ((char)(&DAT_142a93f80)[uVar21] < -0x40) {
LAB_1408406a0:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar21 != 0x2c) goto LAB_1408406a0;
      }
      puVar30 = (ulonglong *)&DAT_142a93f80;
      core::str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (uVar21 != 0xfffffffffffffffe) {
          if (uVar21 + 2 < 0x2c) {
            if ((char)(&UNK_142a93f82)[uVar21] < -0x40) {
LAB_1408406f7:
                    // WARNING: Subroutine does not return
              core::str::slice_error_fail();
            }
          }
          else if (uVar21 + 2 != 0x2c) goto LAB_1408406f7;
        }
        puVar30 = (ulonglong *)(&UNK_142a93f82 + uVar21);
      }
    }
    puVar25 = &DAT_142a94181;
    do {
      bVar31 = puVar25[-1];
      uVar16 = (uint)(char)bVar31;
      if ((char)bVar31 < '\0') {
        bVar5 = puVar25[-2];
        if ((char)bVar5 < -0x40) {
          bVar6 = puVar25[-3];
          if ((char)bVar6 < -0x40) {
            pbVar1 = puVar25 + -4;
            puVar25 = puVar25 + -4;
            uVar16 = bVar6 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar25 = puVar25 + -3;
            uVar16 = bVar6 & 0xf;
          }
          uVar16 = bVar5 & 0x3f | uVar16 << 6;
        }
        else {
          puVar25 = puVar25 + -2;
          uVar16 = bVar5 & 0x1f;
        }
        uVar16 = (uint)(bVar31 & 0x3f) | uVar16 << 6;
        if (uVar16 != 0x5c) goto LAB_1408407b4;
LAB_1408407c6:
        if (puVar25 == &UNK_142a94157) break;
        if (puVar25 + -0x142a94157 < (undefined *)0x29) {
          if (-0x41 < (char)puVar25[1]) break;
        }
        else if (puVar25 + -0x142a94157 == (undefined *)0x29) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar25 = puVar25 + -1;
      if (uVar16 == 0x5c) goto LAB_1408407c6;
LAB_1408407b4:
      if (uVar16 == 0x2f) goto LAB_1408407c6;
    } while (puVar25 != &DAT_142a94158);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar20 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar20 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_140841246:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar20 != 0) {
LAB_140841252:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar20 = -1;
    local_70 = plVar20;
    puffin::ThreadProfiler::begin_scope();
    *local_70 = *local_70 + 1;
  }
  uVar7 = *(undefined4 *)((longlong)in_stack_00000050 + 0x1c);
  puVar8 = (undefined4 *)in_stack_00000050[2];
  *puVar8 = uVar7;
  plVar20 = (longlong *)*in_stack_00000050;
  local_70 = in_R8;
  if (plVar20[3] != 0) {
    uVar21 = plVar20[1];
    if (uVar21 != 0) {
      memset((void *)*plVar20,(int)CONCAT71((int7)((ulonglong)puVar30 >> 8),0xff),uVar21 + 0x11);
    }
    plVar20[3] = 0;
    uVar17 = (uVar21 + 1 & 0xfffffffffffffff8) - (uVar21 + 1 >> 3);
    if (uVar21 < 8) {
      uVar17 = uVar21;
    }
    plVar20[2] = uVar17;
  }
  *puVar8 = uVar7;
  local_178 = (undefined **)*in_stack_00000048;
  lVar19 = in_stack_00000048[1];
  uVar21 = (ulonglong)*local_178;
  uVar26 = 0;
  uVar17 = uVar21 - *(ulonglong *)(lVar19 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar19 + 0x18)) {
    uVar17 = uVar26;
  }
  uVar22 = uVar21 - *(ulonglong *)(lVar19 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar19 + 0x38)) {
    uVar22 = uVar26;
  }
  pcStack_170 = (char *)(uVar17 * 0x10 + *(longlong *)(lVar19 + 8));
  uVar21 = *(ulonglong *)(lVar19 + 0x10) - uVar17;
  if (*(ulonglong *)(lVar19 + 0x10) < uVar17) {
    pcStack_170 = (char *)0x8;
    uVar21 = uVar26;
  }
  local_168._8_8_ = uVar22 * 0x10 + *(longlong *)(lVar19 + 0x28);
  uVar17 = *(ulonglong *)(lVar19 + 0x30) - uVar22;
  if (*(ulonglong *)(lVar19 + 0x30) < uVar22) {
    local_168._8_8_ = 8;
    uVar17 = uVar26;
  }
  *local_178 = (undefined *)(*(longlong *)(lVar19 + 0x40) - (uVar17 + uVar21));
  local_168._0_8_ = pcStack_170 + uVar21 * 0x10;
  local_158._8_8_ = uVar17 + uVar21;
  local_158._0_8_ = uVar17 * 0x10 + local_168._8_8_;
  local_98 = plVar20;
  hashbrown::map::impl$82::
  extend<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure_env_0>,hashbrown::set::impl_10::extend::closure_env_0<country_core::resources::walls::WallId,std::hash::random::RandomState,alloc::alloc::Global,core::iter::adapters::map::Map<bevy_ecs::event::iterators::EventIterator<country_core::systems::wall::wall_state::publish_public_wall_state::OnWallChanged>,system_wall_constructor::construct_elevation_supports::detect_overhang_support_segments::closure_env_0>_>_>_>
            ();
  local_1d0 = (ulonglong *)*in_stack_00000038;
  lVar19 = in_stack_00000038[1];
  uVar21 = *local_1d0;
  uVar26 = 0;
  uVar17 = uVar21 - *(ulonglong *)(lVar19 + 0x18);
  if (uVar21 < *(ulonglong *)(lVar19 + 0x18)) {
    uVar17 = uVar26;
  }
  uVar22 = uVar21 - *(ulonglong *)(lVar19 + 0x38);
  if (uVar21 < *(ulonglong *)(lVar19 + 0x38)) {
    uVar22 = uVar26;
  }
  puVar23 = (undefined8 *)(uVar17 * 0x50 + *(longlong *)(lVar19 + 8));
  uVar21 = *(ulonglong *)(lVar19 + 0x10) - uVar17;
  if (*(ulonglong *)(lVar19 + 0x10) < uVar17) {
    puVar23 = (undefined8 *)&DAT_00000008;
    uVar21 = uVar26;
  }
  puVar18 = (undefined8 *)(uVar22 * 0x50 + *(longlong *)(lVar19 + 0x28));
  uVar17 = *(ulonglong *)(lVar19 + 0x30) - uVar22;
  if (*(ulonglong *)(lVar19 + 0x30) < uVar22) {
    puVar18 = (undefined8 *)&DAT_00000008;
    uVar17 = uVar26;
  }
  *local_1d0 = (*(longlong *)(lVar19 + 0x40) - uVar17) - uVar21;
  local_1c8 = puVar23 + uVar21 * 10;
  local_180 = puVar18 + uVar17 * 10;
  local_1b0 = *in_RCX;
  local_1a8 = *in_RDX;
  local_1a0 = (longlong *)*in_stack_00000040;
  local_198 = (code *)in_stack_00000040[1];
  local_190 = in_stack_00000040[3];
  local_188 = in_stack_00000040[2];
  local_b4 = *(undefined4 *)((longlong)in_stack_00000028 + 0x1c);
  puVar30 = (ulonglong *)*in_stack_00000028;
  local_1d8 = (undefined4 *)in_stack_00000028[2];
  local_1e0 = *local_70;
  iVar12 = -(uint)(CONCAT13(unaff_XMM6_Bd,
                            CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))) ==
                  CONCAT13(unaff_XMM6_Bd,
                           CONCAT12(unaff_XMM6_Bc,CONCAT11(unaff_XMM6_Bb,unaff_XMM6_Ba))));
  iVar13 = -(uint)(CONCAT13(unaff_XMM6_Bh,
                            CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))) ==
                  CONCAT13(unaff_XMM6_Bh,
                           CONCAT12(unaff_XMM6_Bg,CONCAT11(unaff_XMM6_Bf,unaff_XMM6_Be))));
  iVar14 = -(uint)(CONCAT13(unaff_XMM6_Bl,
                            CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))) ==
                  CONCAT13(unaff_XMM6_Bl,
                           CONCAT12(unaff_XMM6_Bk,CONCAT11(unaff_XMM6_Bj,unaff_XMM6_Bi))));
  iVar15 = -(uint)(CONCAT13(unaff_XMM6_Bp,
                            CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))) ==
                  CONCAT13(unaff_XMM6_Bp,
                           CONCAT12(unaff_XMM6_Bo,CONCAT11(unaff_XMM6_Bn,unaff_XMM6_Bm))));
  local_1c0 = local_98 + 4;
LAB_140840ace:
  do {
    if ((puVar23 == (undefined8 *)0x0) || (puVar23 == local_1c8)) {
      if ((puVar18 == (undefined8 *)0x0) || (puVar18 == local_180)) {
        if (local_61 != '\0') {
          puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
          if (*extraout_RAX_05 == 1) {
            plVar20 = extraout_RAX_05 + 1;
          }
          else if ((*extraout_RAX_05 != 0) ||
                  (std::sys::thread_local::native::lazy::Storage::
                   initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                             (), plVar20 = extraout_RAX_07, extraout_RAX_07 == (longlong *)0x0))
          goto LAB_140841246;
          if (*plVar20 != 0) goto LAB_140841252;
          *plVar20 = -1;
          local_70 = plVar20;
          puffin::ThreadProfiler::end_scope();
          *local_70 = *local_70 + 1;
        }
        return;
      }
      local_88 = puVar18 + 10;
      local_80 = (undefined8 *)0x0;
      puVar24 = puVar18;
    }
    else {
      local_80 = puVar23 + 10;
      puVar24 = puVar23;
      local_88 = puVar18;
    }
    *local_1d0 = *local_1d0 + 1;
    if (local_98[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
      bVar31 = (byte)(extraout_RAX_02 >> 0x39);
      auVar33 = ZEXT216(CONCAT11(bVar31,bVar31));
      auVar33 = pshuflw(auVar33,auVar33,0);
      lVar19 = 0;
      uVar21 = extraout_RAX_02;
      while( true ) {
        uVar21 = local_98[1] & uVar21;
        auVar4 = *(undefined1 (*) [16])(*local_98 + uVar21);
        cVar32 = auVar33[0];
        auVar38[0] = -(auVar4[0] == cVar32);
        cVar34 = auVar33[1];
        auVar38[1] = -(auVar4[1] == cVar34);
        cVar35 = auVar33[2];
        auVar38[2] = -(auVar4[2] == cVar35);
        cVar36 = auVar33[3];
        auVar38[3] = -(auVar4[3] == cVar36);
        auVar38[4] = -(auVar4[4] == cVar32);
        auVar38[5] = -(auVar4[5] == cVar34);
        auVar38[6] = -(auVar4[6] == cVar35);
        auVar38[7] = -(auVar4[7] == cVar36);
        auVar38[8] = -(auVar4[8] == cVar32);
        auVar38[9] = -(auVar4[9] == cVar34);
        auVar38[10] = -(auVar4[10] == cVar35);
        auVar38[0xb] = -(auVar4[0xb] == cVar36);
        auVar38[0xc] = -(auVar4[0xc] == cVar32);
        auVar38[0xd] = -(auVar4[0xd] == cVar34);
        auVar38[0xe] = -(auVar4[0xe] == cVar35);
        auVar38[0xf] = -(auVar4[0xf] == cVar36);
        uVar27 = (ushort)(SUB161(auVar38 >> 7,0) & 1) | (ushort)(SUB161(auVar38 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar38 >> 0x17,0) & 1) << 2 |
                 (ushort)(SUB161(auVar38 >> 0x1f,0) & 1) << 3 |
                 (ushort)(SUB161(auVar38 >> 0x27,0) & 1) << 4 |
                 (ushort)(SUB161(auVar38 >> 0x2f,0) & 1) << 5 |
                 (ushort)(SUB161(auVar38 >> 0x37,0) & 1) << 6 |
                 (ushort)(SUB161(auVar38 >> 0x3f,0) & 1) << 7 |
                 (ushort)(SUB161(auVar38 >> 0x47,0) & 1) << 8 |
                 (ushort)(SUB161(auVar38 >> 0x4f,0) & 1) << 9 |
                 (ushort)(SUB161(auVar38 >> 0x57,0) & 1) << 10 |
                 (ushort)(SUB161(auVar38 >> 0x5f,0) & 1) << 0xb |
                 (ushort)(SUB161(auVar38 >> 0x67,0) & 1) << 0xc |
                 (ushort)(SUB161(auVar38 >> 0x6f,0) & 1) << 0xd |
                 (ushort)(SUB161(auVar38 >> 0x77,0) & 1) << 0xe | (ushort)(auVar38[0xf] >> 7) << 0xf
        ;
        uVar16 = (uint)uVar27;
        if (uVar27 != 0) {
          do {
            uVar11 = 0;
            for (uVar29 = uVar16; (uVar29 & 1) == 0; uVar29 = uVar29 >> 1 | 0x80000000) {
              uVar11 = uVar11 + 1;
            }
            if (puVar24[2] == *(longlong *)(*local_98 + (uVar11 + uVar21 & local_98[1]) * -8 + -8))
            {
              puVar18 = local_88;
              puVar23 = local_80;
              if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
                local_200 = bevy_ecs::entity::impl$10::fmt;
                local_208 = puVar24 + 2;
                toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
                local_158 = *extraout_RAX_04;
                local_148 = 2;
                pcStack_140 = "country_core::systems::prefab";
                local_138 = 0x1d;
                local_128 = &PTR_s_Issued_creation_of_a_prefab_on_t_142a941e8;
                local_120 = 2;
                pplStack_118 = &local_208;
                local_110 = 1;
                uStack_108 = 0;
                local_178 = (undefined **)0x0;
                pcStack_170 = "country_core::systems::prefab";
                local_168 = ZEXT816(0x1d);
                local_130 = (longlong *)CONCAT44(*(undefined4 *)extraout_RAX_04[1],1);
                ::log::__private_api::impl$2::log();
                puVar18 = local_88;
                puVar23 = local_80;
              }
              goto LAB_140840ace;
            }
            uVar27 = (ushort)(uVar16 - 1) & (ushort)uVar16;
            uVar16 = CONCAT22((short)(uVar16 - 1 >> 0x10),uVar27);
          } while (uVar27 != 0);
        }
        auVar37[0] = -(auVar4[0] == (char)iVar12);
        auVar37[1] = -(auVar4[1] == (char)((uint)iVar12 >> 8));
        auVar37[2] = -(auVar4[2] == (char)((uint)iVar12 >> 0x10));
        auVar37[3] = -(auVar4[3] == (char)((uint)iVar12 >> 0x18));
        auVar37[4] = -(auVar4[4] == (char)iVar13);
        auVar37[5] = -(auVar4[5] == (char)((uint)iVar13 >> 8));
        auVar37[6] = -(auVar4[6] == (char)((uint)iVar13 >> 0x10));
        auVar37[7] = -(auVar4[7] == (char)((uint)iVar13 >> 0x18));
        auVar37[8] = -(auVar4[8] == (char)iVar14);
        auVar37[9] = -(auVar4[9] == (char)((uint)iVar14 >> 8));
        auVar37[10] = -(auVar4[10] == (char)((uint)iVar14 >> 0x10));
        auVar37[0xb] = -(auVar4[0xb] == (char)((uint)iVar14 >> 0x18));
        auVar37[0xc] = -(auVar4[0xc] == (char)iVar15);
        auVar37[0xd] = -(auVar4[0xd] == (char)((uint)iVar15 >> 8));
        auVar37[0xe] = -(auVar4[0xe] == (char)((uint)iVar15 >> 0x10));
        auVar37[0xf] = -(auVar4[0xf] == (char)((uint)iVar15 >> 0x18));
        if ((((((((((((((((SUB161(auVar37 >> 7,0) & 1) != 0 || (SUB161(auVar37 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar37 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar37 >> 0x1f,0) & 1) != 0) || (SUB161(auVar37 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar37 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar37 >> 0x37,0) & 1) != 0) || (SUB161(auVar37 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar37 >> 0x47,0) & 1) != 0) || (SUB161(auVar37 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar37 >> 0x57,0) & 1) != 0) || (SUB161(auVar37 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar37 >> 0x67,0) & 1) != 0) || (SUB161(auVar37 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar37 >> 0x77,0) & 1) != 0) || auVar37[0xf] < '\0') break;
        uVar21 = uVar21 + lVar19 + 0x10;
        lVar19 = lVar19 + 0x10;
      }
    }
    resources::prefab::PrefabLibrary::get<ref_<asset::prefab::PrefabName>_>();
    asset::prefab::PrefabName::as_name_ref();
    uVar21 = local_1a8;
    create_prefab_gpu_instances();
    local_f0 = 0;
    uStack_e8 = 8;
    local_e0 = 0;
    local_b0 = 0;
    pcStack_a8 = (char *)0x8;
    local_a0 = 0;
    local_1e8 = puVar24[2];
    local_208 = local_1a0;
    local_200 = local_198;
    local_1f8 = local_188;
    local_1f0 = local_190;
    plVar20 = *(longlong **)(extraout_RAX_03 + 8);
    local_90 = plVar20 + *(longlong *)(extraout_RAX_03 + 0x10) * 0x13;
    while (local_70 = plVar20, plVar20 != local_90) {
      asset::prefab::PrefabName::as_name_ref();
      plVar20 = local_70;
      local_238 = uVar21;
      if (!SBORROW8(0,local_70[3])) {
        if ((local_70[5] != 4) || (*(int *)local_70[4] != 0x65657254)) {
          local_d8 = local_70 + 3;
          pcStack_d0 = alloc::string::impl$24::fmt;
          local_c8 = local_240;
          local_c0 = core::fmt::impl$74::fmt<asset::prefab::PrefabNameRef>;
          local_178 = &PTR_s_Unknown_audio_trigger_kind_in_pr_142a94280;
          pcStack_170 = (char *)0x2;
          auVar33._8_8_ = 0;
          auVar33._0_8_ = local_158._8_8_;
          local_158 = auVar33 << 0x40;
          local_168._8_8_ = 2;
          local_168._0_8_ = &local_d8;
                    // WARNING: Subroutine does not return
          core::panicking::panic_fmt();
        }
        plVar2 = local_70 + 6;
        uVar7 = *(undefined4 *)((longlong)puVar24 + 0x24);
        local_d8 = (longlong *)CONCAT44(uVar7,uVar7);
        pcStack_d0 = (code *)CONCAT44(pcStack_d0._4_4_,uVar7);
        ::utils::geometry::aabb::Aabb3::scaled();
        local_210 = *(undefined4 *)(puVar24 + 4);
        local_218 = puVar24[3];
        ::utils::geometry::aabb::Aabb3::translated();
        local_168._0_8_ = plVar20[8];
        local_178 = (undefined **)*plVar2;
        pcStack_170 = (char *)plVar20[7];
        *local_1d8 = local_b4;
        iVar28 = *(int *)((longlong)puVar30 + 0x1c) + 1;
        if (iVar28 == -1) {
                    // WARNING: Subroutine does not return
          std::panicking::begin_panic<ref_<str_>_>();
        }
        uVar21 = puVar30[2];
        uVar16 = (uint)puVar30[3];
        if (uVar16 < uVar21) {
          uVar21 = puVar30[1];
          lVar19 = (ulonglong)uVar16 * 0x1c;
          uVar29 = *(uint *)(uVar21 + 0x18 + lVar19) | 1;
          *(undefined4 *)(puVar30 + 3) = *(undefined4 *)(uVar21 + lVar19);
          *(undefined1 **)(uVar21 + 0x10 + lVar19) = local_c8;
          *(undefined8 *)(uVar21 + lVar19) = local_d8;
          ((undefined8 *)(uVar21 + lVar19))[1] = pcStack_d0;
          *(uint *)(uVar21 + 0x18 + lVar19) = uVar29;
          *(int *)((longlong)puVar30 + 0x1c) = iVar28;
          uVar11 = uVar29;
          if (local_a0 == local_b0) {
LAB_140840f06:
            uVar29 = uVar11;
            lVar19 = local_a0;
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,enum2_<country_core::resources::stairs::stairs_assembly_params::AlignKind>_>,alloc::alloc::Global>
                      ();
            local_a0 = lVar19;
          }
        }
        else {
          if (uVar21 == *puVar30) {
            alloc::raw_vec::RawVec::
            grow_one<system_wall_constructor::place_pillars::PillarPlacementProposal,alloc::alloc::Global>
                      ();
          }
          uVar17 = puVar30[1];
          lVar19 = uVar21 * 0x1c;
          *(undefined1 **)(uVar17 + 0x10 + lVar19) = local_c8;
          *(undefined8 *)(uVar17 + lVar19) = local_d8;
          ((undefined8 *)(uVar17 + lVar19))[1] = pcStack_d0;
          *(undefined4 *)(uVar17 + 0x18 + lVar19) = 1;
          puVar30[2] = uVar21 + 1;
          uVar16 = (uint)uVar21;
          *(uint *)(puVar30 + 3) = uVar16 + 1;
          uVar29 = 1;
          *(int *)((longlong)puVar30 + 0x1c) = iVar28;
          uVar11 = 1;
          if (local_a0 == local_b0) goto LAB_140840f06;
        }
        *(undefined8 *)(pcStack_a8 + local_a0 * 0x28 + 0x10) = local_168._0_8_;
        *(undefined ***)(pcStack_a8 + local_a0 * 0x28) = local_178;
        *(char **)(pcStack_a8 + local_a0 * 0x28 + 8) = pcStack_170;
        pcVar3 = pcStack_a8 + local_a0 * 0x28 + 0x18;
        pcVar3[0] = '\0';
        pcVar3[1] = '\0';
        pcVar3[2] = '\0';
        pcVar3[3] = '\0';
        pcVar3[4] = '\0';
        pcVar3[5] = '\0';
        pcVar3[6] = '\0';
        pcVar3[7] = '\0';
        *(uint *)(pcStack_a8 + local_a0 * 0x28 + 0x20) = uVar29;
        *(uint *)(pcStack_a8 + local_a0 * 0x28 + 0x24) = uVar16;
        local_a0 = local_a0 + 1;
      }
      plVar20 = local_70 + 0x13;
      uVar21 = (ulonglong)*(byte *)(local_1e0 + 0x58);
      create_prefab_rt();
    }
    plVar20 = (longlong *)*puVar24;
    ppuVar9 = (undefined **)puVar24[1];
    LOCK();
    lVar19 = *plVar20;
    *plVar20 = *plVar20 + 1;
    UNLOCK();
    if (*plVar20 == 0 || SCARRY8(lVar19,1) != *plVar20 < 0) {
                    // WARNING: Does not return
      pcVar10 = (code *)invalidInstructionException();
      (*pcVar10)();
    }
    local_120 = puVar24[3];
    pplStack_118 = (longlong **)puVar24[4];
    local_110 = puVar24[5];
    uStack_108 = puVar24[6];
    local_100 = puVar24[7];
    uStack_f8 = puVar24[8];
    local_138 = local_a0;
    local_168._8_8_ = local_f0;
    local_168._0_8_ = local_220;
    local_158._8_8_ = local_e0;
    local_158._0_8_ = uStack_e8;
    local_148 = local_b0;
    pcStack_140 = pcStack_a8;
    local_178 = (undefined **)local_230;
    pcStack_170 = pcStack_228;
    local_130 = plVar20;
    local_128 = ppuVar9;
    bevy_ecs::system::commands::EntityCommands::
    insert<tuple_<country_core::resources::prefab::PrefabInstance,country_core::systems::prefab::PrefabInstanceData>_>
              ();
    puVar18 = local_88;
    puVar23 = local_80;
  } while( true );
}

