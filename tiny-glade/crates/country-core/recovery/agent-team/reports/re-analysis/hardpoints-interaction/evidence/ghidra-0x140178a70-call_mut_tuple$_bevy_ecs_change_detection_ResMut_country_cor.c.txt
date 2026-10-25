
void __cdecl
core::ops::function::impls::impl$3::
call_mut<tuple_<bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>_>,void_(*)(bevy_ecs::change_detection::ResMut<country_core::resources::stairs_walls::StairsPillarProposals>,bevy_ecs::change_detection::Res<country_core::systems::ui::signifiers::signifier_create::UiSignifierStream>,bevy_ecs::change_detection::ResMut<country_core::systems::collision::world::RaycastWorld>,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::stairs_collider::StairColliderRegistry>,bevy_ecs::change_detection::ResMut<country_core::resources::rt::RtWorld>,bevy_ecs::event::writer::EventWriter<country_core::resources::rt::UpdateRtWorldEvent>)>
          (void)

{
  byte *pbVar1;
  char *pcVar2;
  float fVar3;
  byte bVar4;
  undefined8 *puVar5;
  undefined4 *puVar6;
  undefined4 *puVar7;
  undefined8 uVar8;
  uint uVar9;
  int iVar10;
  int iVar11;
  int iVar12;
  int iVar13;
  undefined8 uVar14;
  undefined8 uVar15;
  undefined1 auVar16 [16];
  undefined4 *puVar17;
  char extraout_AL;
  uint uVar18;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  byte *extraout_RAX_02;
  longlong extraout_RAX_03;
  ulonglong extraout_RAX_04;
  ulonglong extraout_RAX_05;
  longlong extraout_RAX_06;
  longlong *extraout_RAX_07;
  undefined1 (*extraout_RAX_08) [16];
  longlong *extraout_RAX_09;
  uint uVar19;
  undefined1 (*pauVar20) [16];
  longlong lVar21;
  longlong *plVar22;
  undefined8 *in_RDX;
  undefined4 *puVar23;
  undefined1 (*pauVar24) [16];
  byte bVar25;
  ulonglong uVar26;
  undefined1 (*pauVar27) [16];
  ulonglong uVar28;
  longlong lVar29;
  undefined **ppuVar30;
  longlong lVar31;
  undefined1 (*in_R9) [16];
  undefined *puVar32;
  ushort uVar33;
  int iVar34;
  longlong lVar35;
  byte bVar36;
  char cVar37;
  char cVar41;
  char cVar42;
  float fVar38;
  float fVar39;
  char cVar43;
  float fVar44;
  undefined1 auVar40 [16];
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  int iVar50;
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 unaff_XMM9_Ba;
  undefined1 unaff_XMM9_Bb;
  char cVar51;
  undefined1 unaff_XMM9_Bc;
  char cVar52;
  undefined1 unaff_XMM9_Bd;
  char cVar53;
  undefined1 unaff_XMM9_Be;
  undefined1 unaff_XMM9_Bf;
  char cVar54;
  undefined1 unaff_XMM9_Bg;
  char cVar55;
  undefined1 unaff_XMM9_Bh;
  char cVar56;
  undefined1 unaff_XMM9_Bi;
  undefined1 unaff_XMM9_Bj;
  char cVar57;
  undefined1 unaff_XMM9_Bk;
  char cVar58;
  undefined1 unaff_XMM9_Bl;
  char cVar59;
  undefined1 unaff_XMM9_Bm;
  undefined1 unaff_XMM9_Bn;
  char cVar60;
  undefined1 unaff_XMM9_Bo;
  char cVar61;
  undefined1 unaff_XMM9_Bp;
  char cVar62;
  longlong in_stack_00000030;
  longlong *in_stack_00000038;
  longlong local_340;
  longlong local_338;
  longlong local_330;
  longlong local_318;
  undefined8 local_258;
  undefined1 (*pauStack_250) [16];
  undefined1 (*local_248) [16];
  undefined1 (*local_240) [16];
  undefined1 (*local_238) [16];
  undefined *local_230;
  undefined8 uStack_228;
  undefined1 local_220 [16];
  undefined8 *local_210;
  longlong local_1d0;
  longlong local_1c8;
  undefined4 local_1c0;
  undefined4 uStack_1bc;
  longlong local_1b8;
  undefined4 *local_1b0;
  undefined4 local_1a8;
  undefined4 local_1a4;
  int local_1a0;
  int iStack_19c;
  ulonglong local_198;
  ulonglong local_190;
  longlong local_188;
  longlong local_180;
  undefined1 (*local_178) [16];
  ulonglong uStack_170;
  longlong local_168;
  longlong lStack_160;
  longlong local_158;
  undefined4 *local_150;
  undefined1 (*local_140) [16];
  undefined1 (*local_138) [16];
  undefined4 local_12c;
  longlong local_128;
  undefined1 (*local_120) [16];
  longlong local_118;
  longlong local_110;
  undefined2 local_104;
  undefined1 local_102;
  undefined1 (*local_100) [16];
  char local_f3;
  undefined1 local_f2;
  undefined1 local_f1;
  undefined8 local_f0;
  
  local_f0 = 0xfffffffffffffffe;
  puVar5 = (undefined8 *)*in_RDX;
  local_1b0 = (undefined4 *)in_RDX[2];
  local_12c = *(undefined4 *)((longlong)in_RDX + 0x1c);
  puVar6 = *(undefined4 **)(in_stack_00000030 + 0x10);
  local_1a8 = *(undefined4 *)(in_stack_00000030 + 0x1c);
  local_180 = *in_stack_00000038;
  puVar7 = (undefined4 *)in_stack_00000038[2];
  local_1a4 = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
  puVar23 = puVar6;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    str::impl$0::rfind<ref_<str_>_>();
    puVar17 = (undefined4 *)&DAT_142960ed0;
    if (extraout_RAX == 1) {
      if (puVar23 != (undefined4 *)0x0) {
        if (&DAT_0000007a < puVar23) {
          if (puVar23 != (undefined4 *)0x7b) goto LAB_1401789c8;
        }
        else if (*(char *)(puVar23 + 0x50a583b4) < -0x40) {
LAB_1401789c8:
                    // WARNING: Subroutine does not return
          str::slice_error_fail();
        }
      }
      str::impl$0::rfind<ref_<str_>_>();
      if (extraout_RAX_00 == 1) {
        if (puVar23 != (undefined4 *)0xfffffffffffffffe) {
          if ((undefined1 *)((longlong)puVar23 + 2) < (undefined1 *)0x7b) {
            if (*(char *)((longlong)puVar23 + 0x142960ed2) < -0x40) {
LAB_140178a1c:
                    // WARNING: Subroutine does not return
              str::slice_error_fail();
            }
          }
          else if ((undefined1 *)((longlong)puVar23 + 2) != (undefined1 *)0x7b) goto LAB_140178a1c;
        }
        puVar17 = (undefined4 *)((longlong)puVar23 + 0x142960ed2);
      }
    }
    puVar23 = puVar17;
    ppuVar30 = &PTR_DAT_142960b90;
    in_R9 = (undefined1 (*) [16])&DAT_142960b34;
    do {
      bVar25 = *(byte *)((longlong)ppuVar30 + -1);
      uVar18 = (uint)(char)bVar25;
      if ((char)bVar25 < '\0') {
        bVar36 = *(byte *)((longlong)ppuVar30 + -2);
        if ((char)bVar36 < -0x40) {
          bVar4 = *(byte *)((longlong)ppuVar30 + -3);
          if ((char)bVar4 < -0x40) {
            pbVar1 = (byte *)((longlong)ppuVar30 + -4);
            ppuVar30 = (undefined **)((longlong)ppuVar30 + -4);
            uVar18 = bVar4 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            ppuVar30 = (undefined **)((longlong)ppuVar30 + -3);
            uVar18 = bVar4 & 0xf;
          }
          uVar18 = bVar36 & 0x3f | uVar18 << 6;
        }
        else {
          ppuVar30 = (undefined **)((longlong)ppuVar30 + -2);
          uVar18 = bVar36 & 0x1f;
        }
        uVar18 = (uint)(bVar25 & 0x3f) | uVar18 << 6;
        if (uVar18 != 0x5c) goto LAB_140178ae4;
LAB_140178af9:
        if (ppuVar30 != (undefined **)&UNK_142960b33) {
          if ((undefined *)((longlong)ppuVar30 + -0x142960b33) < (undefined *)0x5c) {
            if (*(char *)((longlong)ppuVar30 + 1) < -0x40) {
LAB_140178b15:
                    // WARNING: Subroutine does not return
              str::slice_error_fail();
            }
          }
          else if ((undefined *)((longlong)ppuVar30 + -0x142960b33) != (undefined *)0x5c)
          goto LAB_140178b15;
        }
        in_R9 = (undefined1 (*) [16])((longlong)ppuVar30 + 1);
        break;
      }
      ppuVar30 = (undefined **)((longlong)ppuVar30 + -1);
      if (uVar18 == 0x5c) goto LAB_140178af9;
LAB_140178ae4:
      if (uVar18 == 0x2f) goto LAB_140178af9;
    } while (ppuVar30 != (undefined **)&DAT_142960b34);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      pauVar20 = (undefined1 (*) [16])(extraout_RAX_01 + 1);
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), pauVar20 = extraout_RAX_08, extraout_RAX_08 == (undefined1 (*) [16])0x0))
    {
LAB_140179aa6:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*(longlong *)*pauVar20 != 0) {
LAB_140179ab2:
                    // WARNING: Subroutine does not return
      cell::panic_already_borrowed();
    }
    *(undefined8 *)*pauVar20 = 0xffffffffffffffff;
    local_100 = pauVar20;
    puffin::ThreadProfiler::begin_scope();
    *(longlong *)*local_100 = *(longlong *)*local_100 + 1;
  }
  local_f3 = extraout_AL;
  std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL();
  if ((*extraout_RAX_02 & 1) == 0) {
    std::sys::random::hashmap_random_keys();
    extraout_RAX_02[0] = 1;
    extraout_RAX_02[1] = 0;
    extraout_RAX_02[2] = 0;
    extraout_RAX_02[3] = 0;
    extraout_RAX_02[4] = 0;
    extraout_RAX_02[5] = 0;
    extraout_RAX_02[6] = 0;
    extraout_RAX_02[7] = 0;
    *(longlong *)(extraout_RAX_02 + 8) = extraout_RAX_03;
    *(undefined4 **)(extraout_RAX_02 + 0x10) = puVar23;
    local_158 = extraout_RAX_03;
  }
  else {
    local_158 = *(longlong *)(extraout_RAX_02 + 8);
    puVar23 = *(undefined4 **)(extraout_RAX_02 + 0x10);
  }
  *(longlong *)(extraout_RAX_02 + 8) = local_158 + 1;
  local_168 = 0;
  lStack_160 = 0;
  local_178 = (undefined1 (*) [16])&DAT_142960d80;
  uStack_170 = 0;
  *local_1b0 = local_12c;
  local_258 = (undefined1 (*) [16])*puVar5;
  local_248 = (undefined1 (*) [16])(*local_258 + puVar5[1] + 1);
  auVar40 = *local_258;
  local_238 = (undefined1 (*) [16])puVar5[3];
  local_230 = (undefined *)*puVar5;
  uStack_228 = puVar5[1];
  local_220 = *(undefined1 (*) [16])(puVar5 + 2);
  *puVar5 = &anon_5d53dcb640c21e2af8f73dc429315ef4_0_llvm_7340642063134707846;
  puVar5[1] = 0;
  puVar5[2] = 0;
  puVar5[3] = 0;
  pauStack_250 = local_258 + 1;
  local_240 = (undefined1 (*) [16])
              CONCAT62(local_240._2_6_,
                       ~((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                         (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                        (ushort)(byte)(auVar40[0xf] >> 7) << 0xf));
  local_f2 = 1;
  local_210 = puVar5;
  local_150 = puVar23;
  ::alloc::vec::spec_from_iter::impl$0::
  from_iter<tuple_<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,alloc::vec::Vec<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>_>,std::collections::hash::map::Drain<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,alloc::vec::Vec<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>_>_>
            ();
  local_138 = (undefined1 (*) [16])(local_338 + local_330 * 0x20);
  uVar28 = local_338 + 0x30;
  iVar10 = -(uint)(CONCAT13(unaff_XMM9_Bd,
                            CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))) ==
                  CONCAT13(unaff_XMM9_Bd,
                           CONCAT12(unaff_XMM9_Bc,CONCAT11(unaff_XMM9_Bb,unaff_XMM9_Ba))));
  cVar51 = (char)((uint)iVar10 >> 8);
  cVar52 = (char)((uint)iVar10 >> 0x10);
  cVar53 = (char)((uint)iVar10 >> 0x18);
  iVar11 = -(uint)(CONCAT13(unaff_XMM9_Bh,
                            CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))) ==
                  CONCAT13(unaff_XMM9_Bh,
                           CONCAT12(unaff_XMM9_Bg,CONCAT11(unaff_XMM9_Bf,unaff_XMM9_Be))));
  cVar54 = (char)((uint)iVar11 >> 8);
  cVar55 = (char)((uint)iVar11 >> 0x10);
  cVar56 = (char)((uint)iVar11 >> 0x18);
  iVar12 = -(uint)(CONCAT13(unaff_XMM9_Bl,
                            CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))) ==
                  CONCAT13(unaff_XMM9_Bl,
                           CONCAT12(unaff_XMM9_Bk,CONCAT11(unaff_XMM9_Bj,unaff_XMM9_Bi))));
  cVar57 = (char)((uint)iVar12 >> 8);
  cVar58 = (char)((uint)iVar12 >> 0x10);
  cVar59 = (char)((uint)iVar12 >> 0x18);
  iVar13 = -(uint)(CONCAT13(unaff_XMM9_Bp,
                            CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))) ==
                  CONCAT13(unaff_XMM9_Bp,
                           CONCAT12(unaff_XMM9_Bo,CONCAT11(unaff_XMM9_Bn,unaff_XMM9_Bm))));
  cVar60 = (char)((uint)iVar13 >> 8);
  cVar61 = (char)((uint)iVar13 >> 0x10);
  cVar62 = (char)((uint)iVar13 >> 0x18);
  uVar26 = local_330 * 0x20;
LAB_140178d7d:
  uVar26 = uVar26 - 0x20;
  if ((undefined1 (*) [16])local_338 != local_138) {
    lVar31 = local_338 + 0x20;
    lVar29 = *(longlong *)(local_338 + 8);
    if (!SBORROW8(0,lVar29)) {
      pauVar20 = *(undefined1 (**) [16])(local_338 + 0x10);
      local_128 = 0;
      local_120 = (undefined1 (*) [16])&DAT_00000004;
      local_118 = 0;
      in_R9 = (undefined1 (*) [16])&local_258;
      local_198 = uVar26;
      local_190 = uVar28;
      local_188 = lVar31;
      local_110 = lVar29;
      local_100 = pauVar20;
      if (*(longlong *)(local_338 + 0x18) != 0) {
        pauVar27 = (undefined1 (*) [16])
                   (*(longlong *)(local_338 + 0x18) * 0x2c + (longlong)pauVar20);
        do {
          lVar31 = local_118;
          cVar37 = pauVar20[2][8];
          if (cVar37 == '\x02') break;
          auVar40 = *pauVar20;
          fVar39 = auVar40._8_4_;
          fVar3 = auVar40._12_4_;
          uVar8 = *(undefined8 *)pauVar20[2];
          uVar14 = *(undefined8 *)pauVar20[1];
          uVar15 = *(undefined8 *)(pauVar20[1] + 8);
          local_104 = *(undefined2 *)(pauVar20[2] + 9);
          local_102 = pauVar20[2][0xb];
          if (local_118 == 0) {
            if (local_128 == 0) {
              ::alloc::raw_vec::RawVec::
              grow_one<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>
                        ();
            }
            *local_120 = auVar40;
            *(undefined8 *)local_120[1] = uVar14;
            *(undefined8 *)(local_120[1] + 8) = uVar15;
            *(undefined8 *)local_120[2] = uVar8;
            local_120[2][8] = cVar37;
            *(undefined2 *)(local_120[2] + 9) = local_104;
            local_120[2][0xb] = local_102;
            local_118 = 1;
          }
          else {
            lVar35 = local_118 * 0x2c;
            lVar29 = 0;
            do {
              lVar21 = lVar29;
              if (lVar35 - lVar21 == 0) {
                if (local_118 == local_128) {
                  ::alloc::raw_vec::RawVec::
                  grow_one<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>
                            ();
                }
                *(undefined1 (*) [16])((longlong)local_120 + lVar35) = auVar40;
                puVar5 = (undefined8 *)((longlong)local_120 + lVar35 + 0x10);
                *puVar5 = uVar14;
                puVar5[1] = uVar15;
                *(undefined8 *)((longlong)local_120 + lVar35 + 0x20) = uVar8;
                *(char *)((longlong)local_120 + lVar35 + 0x28) = cVar37;
                *(undefined2 *)((longlong)local_120 + lVar35 + 0x29) = local_104;
                *(undefined1 *)((longlong)local_120 + lVar35 + 0x2b) = local_102;
                local_118 = lVar31 + 1;
                goto LAB_140178e66;
              }
              fVar38 = (float)*(undefined8 *)(*local_120 + lVar21) - auVar40._0_4_;
              fVar44 = (float)((ulonglong)*(undefined8 *)(*local_120 + lVar21) >> 0x20) -
                       auVar40._4_4_;
              lVar29 = lVar21 + 0x2c;
            } while (0.01 <= SQRT(fVar44 * fVar44 + fVar38 * fVar38));
            fVar38 = *(float *)(*local_120 + lVar21 + 8);
            fVar44 = *(float *)(*local_120 + lVar21 + 0xc);
            auVar40._8_4_ = fVar39;
            auVar40._0_8_ = auVar40._8_8_;
            auVar40._12_4_ = fVar3;
            if (fVar39 <= fVar38) {
              auVar40._0_4_ = fVar38;
            }
            *(uint *)(*local_120 + lVar21 + 8) =
                 -(uint)NAN(fVar38) & (uint)fVar39 | ~-(uint)NAN(fVar38) & auVar40._0_4_;
            fVar39 = fVar3;
            if (fVar44 <= fVar3) {
              fVar39 = fVar44;
            }
            *(uint *)(*local_120 + lVar21 + 0xc) =
                 -(uint)NAN(fVar44) & (uint)fVar3 | ~-(uint)NAN(fVar44) & (uint)fVar39;
          }
LAB_140178e66:
          pauVar20 = (undefined1 (*) [16])(pauVar20[2] + 0xc);
        } while (pauVar20 != pauVar27);
      }
      if (local_110 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      *local_1b0 = local_12c;
      local_f1 = 1;
      ::alloc::vec::impl$11::
      clone<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>();
      local_f1 = 1;
      hashbrown::map::HashMap::
      insert<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,alloc::vec::Vec<country_core::resources::stairs_walls::StairsPillarProposal,alloc::alloc::Global>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if ((!SBORROW8(0,local_318)) && (local_318 != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      local_110 = local_128;
      pauVar27 = (undefined1 (*) [16])(local_118 * 0x2c + (longlong)local_120);
      local_140 = local_120;
      pauVar20 = local_120;
LAB_140179334:
      do {
        if ((pauVar20 == pauVar27) || (pauVar20[2][8] == '\x02')) goto LAB_1401795f0;
        fVar39 = *(float *)(*pauVar20 + 8);
        fVar3 = *(float *)(*pauVar20 + 0xc);
        fVar38 = *(float *)(pauVar20[1] + 8);
        iVar50 = *(int *)(pauVar20[2] + 4);
        iVar34 = *(int *)pauVar20[2];
        local_258 = *(undefined1 (**) [16])pauVar20[2];
        local_1a0 = iVar34;
        iStack_19c = iVar50;
        hash::BuildHasher::
        hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                  ();
        pauVar20 = (undefined1 (*) [16])(pauVar20[2] + 0xc);
        pauVar24 = (undefined1 (*) [16])(uStack_170 & extraout_RAX_04);
        bVar25 = (byte)(extraout_RAX_04 >> 0x38);
        bVar36 = bVar25 >> 1;
        auVar40 = ZEXT216(CONCAT11(bVar36,bVar36));
        auVar40 = pshuflw(auVar40,auVar40,0);
        lVar31 = 0;
        in_R9 = pauVar24;
        while( true ) {
          auVar48 = *(undefined1 (*) [16])(*in_R9 + (longlong)*local_178);
          cVar37 = auVar40[0];
          auVar47[0] = -(auVar48[0] == cVar37);
          cVar41 = auVar40[1];
          auVar47[1] = -(auVar48[1] == cVar41);
          cVar42 = auVar40[2];
          auVar47[2] = -(auVar48[2] == cVar42);
          cVar43 = auVar40[3];
          auVar47[3] = -(auVar48[3] == cVar43);
          auVar47[4] = -(auVar48[4] == cVar37);
          auVar47[5] = -(auVar48[5] == cVar41);
          auVar47[6] = -(auVar48[6] == cVar42);
          auVar47[7] = -(auVar48[7] == cVar43);
          auVar47[8] = -(auVar48[8] == cVar37);
          auVar47[9] = -(auVar48[9] == cVar41);
          auVar47[10] = -(auVar48[10] == cVar42);
          auVar47[0xb] = -(auVar48[0xb] == cVar43);
          auVar47[0xc] = -(auVar48[0xc] == cVar37);
          auVar47[0xd] = -(auVar48[0xd] == cVar41);
          auVar47[0xe] = -(auVar48[0xe] == cVar42);
          auVar47[0xf] = -(auVar48[0xf] == cVar43);
          uVar33 = (ushort)(SUB161(auVar47 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
                   (ushort)(SUB161(auVar47 >> 0x1f,0) & 1) << 3 |
                   (ushort)(SUB161(auVar47 >> 0x27,0) & 1) << 4 |
                   (ushort)(SUB161(auVar47 >> 0x2f,0) & 1) << 5 |
                   (ushort)(SUB161(auVar47 >> 0x37,0) & 1) << 6 |
                   (ushort)(SUB161(auVar47 >> 0x3f,0) & 1) << 7 |
                   (ushort)(SUB161(auVar47 >> 0x47,0) & 1) << 8 |
                   (ushort)(SUB161(auVar47 >> 0x4f,0) & 1) << 9 |
                   (ushort)(SUB161(auVar47 >> 0x57,0) & 1) << 10 |
                   (ushort)(SUB161(auVar47 >> 0x5f,0) & 1) << 0xb |
                   (ushort)(SUB161(auVar47 >> 0x67,0) & 1) << 0xc |
                   (ushort)(SUB161(auVar47 >> 0x6f,0) & 1) << 0xd |
                   (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe |
                   (ushort)(auVar47[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar33;
          while (uVar33 != 0) {
            uVar19 = 0;
            for (uVar9 = uVar18; (uVar9 & 1) == 0; uVar9 = uVar9 >> 1 | 0x80000000) {
              uVar19 = uVar19 + 1;
            }
            uVar28 = (ulonglong)(*in_R9 + uVar19) & uStack_170;
            if ((*(int *)local_178[uVar28 * -2 + -2] == iVar34) &&
               (*(int *)(local_178[uVar28 * -2 + -2] + 4) == iVar50)) goto joined_r0x000140179461;
            uVar33 = (ushort)(uVar18 - 1) & (ushort)uVar18;
            uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar33);
          }
          auVar45[0] = -(auVar48[0] == (char)iVar10);
          auVar45[1] = -(auVar48[1] == cVar51);
          auVar45[2] = -(auVar48[2] == cVar52);
          auVar45[3] = -(auVar48[3] == cVar53);
          auVar45[4] = -(auVar48[4] == (char)iVar11);
          auVar45[5] = -(auVar48[5] == cVar54);
          auVar45[6] = -(auVar48[6] == cVar55);
          auVar45[7] = -(auVar48[7] == cVar56);
          auVar45[8] = -(auVar48[8] == (char)iVar12);
          auVar45[9] = -(auVar48[9] == cVar57);
          auVar45[10] = -(auVar48[10] == cVar58);
          auVar45[0xb] = -(auVar48[0xb] == cVar59);
          auVar45[0xc] = -(auVar48[0xc] == (char)iVar13);
          auVar45[0xd] = -(auVar48[0xd] == cVar60);
          auVar45[0xe] = -(auVar48[0xe] == cVar61);
          auVar45[0xf] = -(auVar48[0xf] == cVar62);
          if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 || (SUB161(auVar45 >> 0xf,0) & 1) != 0
                           ) || (SUB161(auVar45 >> 0x17,0) & 1) != 0) ||
                         (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                        (SUB161(auVar45 >> 0x27,0) & 1) != 0) ||
                       (SUB161(auVar45 >> 0x2f,0) & 1) != 0) || (SUB161(auVar45 >> 0x37,0) & 1) != 0
                      ) || (SUB161(auVar45 >> 0x3f,0) & 1) != 0) ||
                    (SUB161(auVar45 >> 0x47,0) & 1) != 0) || (SUB161(auVar45 >> 0x4f,0) & 1) != 0)
                  || (SUB161(auVar45 >> 0x57,0) & 1) != 0) || (SUB161(auVar45 >> 0x5f,0) & 1) != 0)
                || (SUB161(auVar45 >> 0x67,0) & 1) != 0) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0) ||
              (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') break;
          puVar32 = in_R9[1] + lVar31;
          lVar31 = lVar31 + 0x10;
          in_R9 = (undefined1 (*) [16])((ulonglong)puVar32 & uStack_170);
        }
        if (local_168 == 0) {
          hashbrown::raw::RawTable::
          reserve_rehash<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,alloc::vec::Vec<glam::f32::vec3::Vec3,alloc::alloc::Global>,std::hash::random::RandomState>_>
                    ();
          pauVar24 = (undefined1 (*) [16])(extraout_RAX_04 & uStack_170);
          auVar40 = *(undefined1 (*) [16])(*pauVar24 + (longlong)*local_178);
          uVar18 = (uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                                  (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                                  (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                                  (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                                 (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
          iVar50 = local_258._4_4_;
          iVar34 = (int)local_258;
          if (uVar18 == 0) goto LAB_1401795ac;
        }
        else {
          auVar40 = *(undefined1 (*) [16])(*pauVar24 + (longlong)*local_178);
          uVar33 = (ushort)(SUB161(auVar40 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                   (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
          uVar18 = (uint)uVar33;
          if (uVar33 == 0) {
LAB_1401795ac:
            lVar31 = 0x10;
            do {
              pauVar24 = (undefined1 (*) [16])((ulonglong)(*pauVar24 + lVar31) & uStack_170);
              auVar40 = *(undefined1 (*) [16])(*pauVar24 + (longlong)*local_178);
              lVar31 = lVar31 + 0x10;
              uVar18 = (uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                                      (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                                      (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                                      (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                                     (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
            } while (uVar18 == 0);
          }
        }
        uVar19 = 0;
        for (; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
          uVar19 = uVar19 + 1;
        }
        uVar28 = (ulonglong)(*pauVar24 + uVar19) & uStack_170;
        bVar36 = (*local_178)[uVar28];
        if (-1 < (char)bVar36) {
          auVar40 = *local_178;
          uVar18 = 0;
          for (uVar19 = (uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                                       (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                                       (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                                       (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                                      (ushort)(byte)(auVar40[0xf] >> 7) << 0xf); (uVar19 & 1) == 0;
              uVar19 = uVar19 >> 1 | 0x80000000) {
            uVar18 = uVar18 + 1;
          }
          uVar28 = (ulonglong)uVar18;
          bVar36 = (*local_178)[uVar28];
        }
        in_R9 = (undefined1 (*) [16])(uVar28 - 0x10 & uStack_170);
        bVar25 = bVar25 >> 1;
        (*local_178)[uVar28] = bVar25;
        (*in_R9)[(longlong)local_178[1]] = bVar25;
        local_168 = local_168 - (ulonglong)(bVar36 & 1);
        *(int *)local_178[uVar28 * -2 + -2] = iVar34;
        *(int *)(local_178[uVar28 * -2 + -2] + 4) = iVar50;
        *(undefined8 *)(local_178[uVar28 * -2 + -2] + 8) = 0;
        *(undefined8 *)local_178[uVar28 * -2 + -1] = 4;
        *(undefined8 *)(local_178[uVar28 * -2 + -1] + 8) = 0;
        lStack_160 = lStack_160 + 1;
joined_r0x000140179461:
        if (lStack_160 != 0) {
          hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                    ();
          bVar25 = (byte)(extraout_RAX_05 >> 0x39);
          auVar40 = ZEXT216(CONCAT11(bVar25,bVar25));
          auVar40 = pshuflw(auVar40,auVar40,0);
          lVar31 = 0;
          uVar28 = extraout_RAX_05;
          while( true ) {
            uVar28 = uStack_170 & uVar28;
            pcVar2 = *local_178 + uVar28;
            cVar37 = auVar40[0];
            auVar48[0] = -(*pcVar2 == cVar37);
            cVar41 = auVar40[1];
            auVar48[1] = -(pcVar2[1] == cVar41);
            cVar42 = auVar40[2];
            auVar48[2] = -(pcVar2[2] == cVar42);
            cVar43 = auVar40[3];
            auVar48[3] = -(pcVar2[3] == cVar43);
            auVar48[4] = -(pcVar2[4] == cVar37);
            auVar48[5] = -(pcVar2[5] == cVar41);
            auVar48[6] = -(pcVar2[6] == cVar42);
            auVar48[7] = -(pcVar2[7] == cVar43);
            auVar48[8] = -(pcVar2[8] == cVar37);
            auVar48[9] = -(pcVar2[9] == cVar41);
            auVar48[10] = -(pcVar2[10] == cVar42);
            auVar48[0xb] = -(pcVar2[0xb] == cVar43);
            auVar48[0xc] = -(pcVar2[0xc] == cVar37);
            auVar48[0xd] = -(pcVar2[0xd] == cVar41);
            auVar48[0xe] = -(pcVar2[0xe] == cVar42);
            auVar48[0xf] = -(pcVar2[0xf] == cVar43);
            uVar33 = (ushort)(SUB161(auVar48 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar48 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar48 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar48 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar48 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar48 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar48 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar48 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar48 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar48 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar48 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar48 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar48[0xf] >> 7) << 0xf;
            in_R9 = (undefined1 (*) [16])(ulonglong)uVar33;
            while (uVar33 != 0) {
              uVar18 = 0;
              for (uVar19 = (uint)in_R9; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
                uVar18 = uVar18 + 1;
              }
              uVar26 = uVar18 + uVar28 & uStack_170;
              auVar49._0_4_ = -(uint)((int)*(undefined8 *)local_178[uVar26 * -2 + -2] == local_1a0);
              iVar50 = -(uint)((int)((ulonglong)*(undefined8 *)local_178[uVar26 * -2 + -2] >> 0x20)
                              == iStack_19c);
              auVar49._4_4_ = auVar49._0_4_;
              auVar49._8_4_ = iVar50;
              auVar49._12_4_ = iVar50;
              iVar50 = movmskpd((int)uVar26 * 0x20,auVar49);
              if (iVar50 == 3) {
                ::utils::swizzlers::impl$1::xvy();
                local_258 = (undefined1 (*) [16])CONCAT44((fVar39 - fVar3) + 0.3,fVar38 + -0.5);
                pauStack_250 = (undefined1 (*) [16])CONCAT44(pauStack_250._4_4_,0x3f0f5c29);
                ::utils::geometry::rectangle::Rectangle3d::new();
                ::utils::swizzlers::impl$1::xvy();
                ::utils::xz_rotation_bad();
                local_318 = (longlong)local_258;
                country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::
                box_trigger_volume();
                local_100 = (undefined1 (*) [16])CONCAT44(uStack_1bc,local_1c0);
                local_1d0 = (longlong)local_100 + local_1b8 * 0x24;
                local_258 = (undefined1 (*) [16])0x0;
                local_220._8_8_ = 0;
                ::alloc::vec::spec_extend::impl$1::
                spec_extend<glam::f32::vec3::Vec3,core::iter::adapters::flatten::FlatMap<core::slice::iter::Iter<utils::geometry::intersect::Triangle>,array_<glam::f32::vec3::Vec3,3>,system_wall_constructor::construct_playermade_stairs::pillars_process_proposals::process_playermade_stair_pillars_proposals::closure_env_0<true>_>,alloc::alloc::Global>
                          ();
                if (local_1c8 != 0) {
                  std::alloc::__default_lib_allocator::__rust_dealloc();
                }
                goto LAB_140179334;
              }
              iVar50 = (uint)in_R9 - 1;
              uVar33 = (ushort)iVar50 & (ushort)in_R9;
              in_R9 = (undefined1 (*) [16])(ulonglong)CONCAT22((short)((uint)iVar50 >> 0x10),uVar33)
              ;
            }
            auVar46[0] = -(*pcVar2 == (char)iVar10);
            auVar46[1] = -(pcVar2[1] == cVar51);
            auVar46[2] = -(pcVar2[2] == cVar52);
            auVar46[3] = -(pcVar2[3] == cVar53);
            auVar46[4] = -(pcVar2[4] == (char)iVar11);
            auVar46[5] = -(pcVar2[5] == cVar54);
            auVar46[6] = -(pcVar2[6] == cVar55);
            auVar46[7] = -(pcVar2[7] == cVar56);
            auVar46[8] = -(pcVar2[8] == (char)iVar12);
            auVar46[9] = -(pcVar2[9] == cVar57);
            auVar46[10] = -(pcVar2[10] == cVar58);
            auVar46[0xb] = -(pcVar2[0xb] == cVar59);
            auVar46[0xc] = -(pcVar2[0xc] == (char)iVar13);
            auVar46[0xd] = -(pcVar2[0xd] == cVar60);
            auVar46[0xe] = -(pcVar2[0xe] == cVar61);
            auVar46[0xf] = -(pcVar2[0xf] == cVar62);
            uVar33 = (ushort)(SUB161(auVar46 >> 7,0) & 1) |
                     (ushort)(SUB161(auVar46 >> 0xf,0) & 1) << 1 |
                     (ushort)(SUB161(auVar46 >> 0x17,0) & 1) << 2 |
                     (ushort)(SUB161(auVar46 >> 0x1f,0) & 1) << 3 |
                     (ushort)(SUB161(auVar46 >> 0x27,0) & 1) << 4 |
                     (ushort)(SUB161(auVar46 >> 0x2f,0) & 1) << 5 |
                     (ushort)(SUB161(auVar46 >> 0x37,0) & 1) << 6 |
                     (ushort)(SUB161(auVar46 >> 0x3f,0) & 1) << 7 |
                     (ushort)(SUB161(auVar46 >> 0x47,0) & 1) << 8 |
                     (ushort)(SUB161(auVar46 >> 0x4f,0) & 1) << 9 |
                     (ushort)(SUB161(auVar46 >> 0x57,0) & 1) << 10 |
                     (ushort)(SUB161(auVar46 >> 0x5f,0) & 1) << 0xb |
                     (ushort)(SUB161(auVar46 >> 0x67,0) & 1) << 0xc |
                     (ushort)(SUB161(auVar46 >> 0x6f,0) & 1) << 0xd |
                     (ushort)(SUB161(auVar46 >> 0x77,0) & 1) << 0xe |
                     (ushort)(auVar46[0xf] >> 7) << 0xf;
            in_R9 = (undefined1 (*) [16])(ulonglong)uVar33;
            if (uVar33 != 0) break;
            uVar28 = uVar28 + lVar31 + 0x10;
            lVar31 = lVar31 + 0x10;
          }
        }
      } while( true );
    }
    if (local_138 != (undefined1 (*) [16])lVar31) {
      uVar26 = uVar26 >> 5;
      do {
        if (*(longlong *)(uVar28 - 8) != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        uVar28 = uVar28 + 0x20;
        uVar26 = uVar26 - 1;
      } while (uVar26 != 0);
    }
  }
  lVar31 = 0;
  if (local_340 != 0) {
    lVar31 = local_340 << 5;
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  auVar40 = *local_178;
  if (uStack_170 == 0) {
    local_258 = (undefined1 (*) [16])0x0;
  }
  else {
    lVar31 = uStack_170 * 0x21 + 0x31;
    in_R9 = local_178 + uStack_170 * -2 + -2;
    local_258 = (undefined1 (*) [16])&DAT_00000010;
  }
  pauVar20 = local_178 + 1;
  uVar18 = ~(uint)(ushort)((ushort)(SUB161(auVar40 >> 7,0) & 1) |
                           (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                           (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                           (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                          (ushort)(byte)(auVar40[0xf] >> 7) << 0xf);
  local_190 = uStack_170;
  local_230 = *local_178 + uStack_170 + 1;
  local_240 = local_178;
  uStack_228 = CONCAT62(uStack_228._2_6_,(short)uVar18);
  local_220._0_8_ = lStack_160;
  pauStack_250 = (undefined1 (*) [16])lVar31;
  local_248 = in_R9;
  local_238 = pauVar20;
  local_188 = lVar31;
  local_140 = local_258;
  local_138 = in_R9;
  if (lStack_160 != 0) {
    local_198 = local_180 + 0x20;
    pauVar27 = local_178;
    lVar31 = lStack_160;
    do {
      if ((short)uVar18 == 0) {
        do {
          auVar40 = *pauVar20;
          uVar33 = (ushort)(SUB161(auVar40 >> 7,0) & 1) |
                   (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                   (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                   (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                   (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
          pauVar27 = pauVar27 + -0x20;
          pauVar20 = pauVar20 + 1;
        } while (uVar33 == 0xffff);
        uVar19 = ~(uint)uVar33;
        uVar18 = uVar19 & -(uint)uVar33 - 2;
        local_240 = pauVar27;
        local_238 = pauVar20;
      }
      else {
        uVar19 = uVar18;
        uVar18 = uVar18 - 1 & uVar18;
      }
      uStack_228 = CONCAT62(uStack_228._2_6_,(short)uVar18);
      lVar31 = lVar31 + -1;
      local_220._0_8_ = lVar31;
      iVar10 = 0;
      for (; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
        iVar10 = iVar10 + 1;
      }
      uVar19 = iVar10 << 5;
      lVar29 = *(longlong *)((longlong)pauVar27 + (-0x18 - (ulonglong)uVar19));
      if (SBORROW8(0,lVar29)) {
        if (lVar31 != 0) {
          do {
            if ((short)uVar18 == 0) {
              do {
                auVar40 = *pauVar20;
                uVar33 = (ushort)(SUB161(auVar40 >> 7,0) & 1) |
                         (ushort)(SUB161(auVar40 >> 0xf,0) & 1) << 1 |
                         (ushort)(SUB161(auVar40 >> 0x17,0) & 1) << 2 |
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
                         (ushort)(SUB161(auVar40 >> 0x77,0) & 1) << 0xe |
                         (ushort)(byte)(auVar40[0xf] >> 7) << 0xf;
                pauVar27 = pauVar27 + -0x20;
                pauVar20 = pauVar20 + 1;
              } while (uVar33 == 0xffff);
              uVar19 = ~(uint)uVar33;
              uVar18 = uVar19 & -(uint)uVar33 - 2;
            }
            else {
              uVar19 = uVar18;
              uVar18 = uVar18 - 1 & uVar18;
            }
            iVar10 = 0;
            for (; (uVar19 & 1) == 0; uVar19 = uVar19 >> 1 | 0x80000000) {
              iVar10 = iVar10 + 1;
            }
            if (*(longlong *)((longlong)pauVar27 + (-0x18 - (ulonglong)(uint)(iVar10 << 5))) != 0) {
              std::alloc::__default_lib_allocator::__rust_dealloc();
            }
            lVar31 = lVar31 + -1;
          } while (lVar31 != 0);
          uStack_228 = CONCAT62(uStack_228._2_6_,(short)uVar18);
          auVar16._8_8_ = 0;
          auVar16._0_8_ = local_220._8_8_;
          local_220 = auVar16 << 0x40;
        }
        break;
      }
      local_100 = *(undefined1 (**) [16])((longlong)pauVar27 + (-0x10 - (ulonglong)uVar19));
      uVar28 = *(ulonglong *)((longlong)pauVar27 + (-8 - (ulonglong)uVar19));
      *puVar6 = local_1a8;
      local_110 = lVar29;
      country_core::resources::stairs::stairs_assembly_params::impl$2::from();
      country_core::resources::rt::RtWorld::
      get_mut<enum2_<country_core::resources::rt::RtMeshHandle>_>();
      lVar29 = *(longlong *)(extraout_RAX_06 + 0x50);
      if ((ulonglong)(*(longlong *)(extraout_RAX_06 + 0x40) - lVar29) < uVar28) {
        ::alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>();
        lVar29 = *(longlong *)(extraout_RAX_06 + 0x50);
      }
      memcpy((void *)(lVar29 * 0xc + *(longlong *)(extraout_RAX_06 + 0x48)),local_100,uVar28 * 0xc);
      *(ulonglong *)(extraout_RAX_06 + 0x50) = lVar29 + uVar28;
      *puVar7 = local_1a4;
      lVar29 = *(longlong *)(local_180 + 0x30);
      uVar8 = *(undefined8 *)(local_180 + 0x40);
      if (lVar29 == *(longlong *)(local_180 + 0x20)) {
        ::alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                  ();
      }
      *(undefined8 *)(*(longlong *)(local_180 + 0x28) + lVar29 * 8) = uVar8;
      *(longlong *)(local_180 + 0x30) = lVar29 + 1;
      *(longlong *)(local_180 + 0x40) = *(longlong *)(local_180 + 0x40) + 1;
      if (local_110 != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
    } while (lVar31 != 0);
  }
  if (local_188 != 0 && local_190 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (local_f3 == '\0') {
    return;
  }
  puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
  if (*extraout_RAX_07 == 1) {
    plVar22 = extraout_RAX_07 + 1;
  }
  else if ((*extraout_RAX_07 != 0) ||
          (std::sys::thread_local::native::lazy::Storage::
           initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                     (), plVar22 = extraout_RAX_09, extraout_RAX_09 == (longlong *)0x0))
  goto LAB_140179aa6;
  if (*plVar22 == 0) {
    *plVar22 = -1;
    local_100 = (undefined1 (*) [16])plVar22;
    puffin::ThreadProfiler::end_scope();
    *(longlong *)local_100 = *(longlong *)local_100 + 1;
    return;
  }
  goto LAB_140179ab2;
LAB_1401795f0:
  if (local_110 != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  uVar28 = local_190 + 0x20;
  local_338 = local_188;
  uVar26 = local_198;
  goto LAB_140178d7d;
}

