
// WARNING: Removing unreachable block (ram,0x00014231d282)

void __cdecl system_critters::wren::update_decorator_bird_spots(void)

{
  byte *pbVar1;
  longlong lVar2;
  char *pcVar3;
  uint *puVar4;
  float fVar5;
  float fVar6;
  byte bVar7;
  byte bVar8;
  char cVar9;
  undefined4 uVar10;
  int iVar11;
  longlong lVar12;
  longlong lVar13;
  longlong lVar14;
  longlong lVar15;
  undefined4 *puVar16;
  uint uVar17;
  uint uVar18;
  int iVar19;
  int iVar20;
  int iVar21;
  int iVar22;
  float fVar23;
  float fVar24;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar29;
  undefined4 extraout_EAX;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong *extraout_RAX_01;
  ulonglong extraout_RAX_02;
  ulonglong extraout_RAX_03;
  longlong *extraout_RAX_04;
  longlong *extraout_RAX_05;
  longlong *extraout_RAX_06;
  longlong *in_RCX;
  longlong *plVar30;
  ulonglong in_RDX;
  ulonglong uVar31;
  ushort uVar32;
  ulonglong uVar33;
  float *pfVar34;
  longlong *in_R8;
  undefined *puVar35;
  ulonglong uVar36;
  longlong *in_R9;
  longlong lVar37;
  int *piVar38;
  uint *puVar39;
  byte bVar40;
  char cVar41;
  char cVar43;
  char cVar44;
  char cVar45;
  undefined1 auVar42 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  float fVar48;
  float fVar49;
  undefined1 unaff_XMM7_Ba;
  undefined1 unaff_XMM7_Bb;
  undefined1 unaff_XMM7_Bc;
  undefined1 unaff_XMM7_Bd;
  undefined1 unaff_XMM7_Be;
  undefined1 unaff_XMM7_Bf;
  undefined1 unaff_XMM7_Bg;
  undefined1 unaff_XMM7_Bh;
  undefined1 unaff_XMM7_Bi;
  undefined1 unaff_XMM7_Bj;
  undefined1 unaff_XMM7_Bk;
  undefined1 unaff_XMM7_Bl;
  undefined1 unaff_XMM7_Bm;
  undefined1 unaff_XMM7_Bn;
  undefined1 unaff_XMM7_Bo;
  undefined1 unaff_XMM7_Bp;
  float fVar50;
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  longlong in_stack_00000028;
  undefined8 local_170;
  longlong local_138;
  longlong local_108;
  longlong local_f0;
  longlong local_e8;
  ulonglong local_e0;
  
  uVar31 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar31 != 0) {
        if (uVar31 < 0x32) {
          if ((char)(&DAT_14304b468)[uVar31] < -0x40) {
LAB_14231c890:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 != 0x32) goto LAB_14231c890;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar31 != 0xfffffffffffffffe)) {
        if (uVar31 + 2 < 0x32) {
          if ("stem_critters::wren::update_decorator_bird_spots::f"[uVar31] < -0x40) {
LAB_14231c8e4:
                    // WARNING: Subroutine does not return
            core::str::slice_error_fail();
          }
        }
        else if (uVar31 + 2 != 0x32) goto LAB_14231c8e4;
      }
    }
    puVar35 = &DAT_14304b58b;
    do {
      bVar40 = puVar35[-1];
      uVar29 = (uint)(char)bVar40;
      if ((char)bVar40 < '\0') {
        bVar7 = puVar35[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar35[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = puVar35 + -4;
            puVar35 = puVar35 + -4;
            uVar29 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar35 = puVar35 + -3;
            uVar29 = bVar8 & 0xf;
          }
          uVar29 = bVar7 & 0x3f | uVar29 << 6;
        }
        else {
          puVar35 = puVar35 + -2;
          uVar29 = bVar7 & 0x1f;
        }
        uVar29 = (uint)(bVar40 & 0x3f) | uVar29 << 6;
        if (uVar29 != 0x5c) goto LAB_14231c9a4;
LAB_14231c9b5:
        if (puVar35 == &UNK_14304b567) break;
        if (puVar35 + -0x14304b567 < (undefined *)0x23) {
          if (-0x41 < (char)puVar35[1]) break;
        }
        else if (puVar35 + -0x14304b567 == (undefined *)0x23) break;
                    // WARNING: Subroutine does not return
        core::str::slice_error_fail();
      }
      puVar35 = puVar35 + -1;
      if (uVar29 == 0x5c) goto LAB_14231c9b5;
LAB_14231c9a4:
      if (uVar29 == 0x2f) goto LAB_14231c9b5;
    } while (puVar35 != &DAT_14304b568);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar30 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14231d370;
    if (*plVar30 != 0) goto LAB_14231d37c;
    *plVar30 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar30 = *plVar30 + 1;
  }
  local_108 = 0;
  lVar12 = *in_RCX;
  lVar13 = in_RCX[1];
  puVar39 = *(uint **)(lVar12 + 0xf8);
  puVar4 = puVar39 + *(longlong *)(lVar12 + 0x100);
  cVar9 = *(char *)(lVar12 + 0x108);
  lVar14 = *in_R9;
  lVar15 = in_R9[1];
  uVar10 = *(undefined4 *)(in_RDX + 0x1c);
  puVar16 = *(undefined4 **)(in_RDX + 0x10);
  local_138 = 8;
  iVar19 = -(uint)(CONCAT13(unaff_XMM7_Bd,
                            CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))) ==
                  CONCAT13(unaff_XMM7_Bd,
                           CONCAT12(unaff_XMM7_Bc,CONCAT11(unaff_XMM7_Bb,unaff_XMM7_Ba))));
  iVar20 = -(uint)(CONCAT13(unaff_XMM7_Bh,
                            CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))) ==
                  CONCAT13(unaff_XMM7_Bh,
                           CONCAT12(unaff_XMM7_Bg,CONCAT11(unaff_XMM7_Bf,unaff_XMM7_Be))));
  iVar21 = -(uint)(CONCAT13(unaff_XMM7_Bl,
                            CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))) ==
                  CONCAT13(unaff_XMM7_Bl,
                           CONCAT12(unaff_XMM7_Bk,CONCAT11(unaff_XMM7_Bj,unaff_XMM7_Bi))));
  iVar22 = -(uint)(CONCAT13(unaff_XMM7_Bp,
                            CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))) ==
                  CONCAT13(unaff_XMM7_Bp,
                           CONCAT12(unaff_XMM7_Bo,CONCAT11(unaff_XMM7_Bn,unaff_XMM7_Bm))));
  local_f0 = 0;
  local_e8 = 0;
  uVar31 = 0;
  local_e0 = 0;
LAB_14231cb3b:
  do {
    if (cVar9 == '\0') {
      if (local_e0 == uVar31) {
        do {
          if (puVar39 == puVar4) goto LAB_14231d25c;
          uVar29 = *puVar39;
          puVar39 = puVar39 + 1;
          uVar31 = *(ulonglong *)(*(longlong *)(lVar13 + 0x100) + 0x58 + (ulonglong)uVar29 * 0xa0);
        } while (uVar31 == 0);
        lVar37 = *(longlong *)(lVar13 + 0x100) + (ulonglong)uVar29 * 0xa0;
        uVar36 = (ulonglong)*(uint *)(lVar37 + 0x94);
        lVar2 = *(longlong *)(lVar13 + 0x1a8) + uVar36 * 0x48;
        uVar36 = *(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x40 + uVar36 * 0x48);
        if ((*(ulonglong *)(lVar12 + 0x110) < uVar36) &&
           (uVar33 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
           uVar33 != 0)) {
          local_f0 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar33 * 0x30);
          uVar33 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar36 <= uVar33) goto LAB_14231cd57;
LAB_14231cd14:
          uVar36 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar33 * 8);
          if (uVar36 == 0) goto LAB_14231cd57;
          local_e8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar36 * 0x30);
        }
        else {
          local_f0 = 0;
          uVar33 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar33 < uVar36) goto LAB_14231cd14;
LAB_14231cd57:
          local_e8 = 0;
        }
        local_138 = *(longlong *)(lVar37 + 0x50);
        local_e0 = 0;
      }
      uVar36 = (ulonglong)*(uint *)(local_138 + 8 + local_e0 * 0x10);
    }
    else {
      if (local_e0 == uVar31) {
        do {
          if (puVar39 == puVar4) goto LAB_14231d25c;
          uVar29 = *puVar39;
          puVar39 = puVar39 + 1;
          uVar31 = *(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x10 + (ulonglong)uVar29 * 0x48);
        } while (uVar31 == 0);
        lVar2 = *(longlong *)(lVar13 + 0x1a8) + (ulonglong)uVar29 * 0x48;
        uVar36 = *(ulonglong *)(lVar2 + 0x40);
        if ((*(ulonglong *)(lVar12 + 0x110) < uVar36) &&
           (uVar33 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
           uVar33 != 0)) {
          local_f0 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar33 * 0x30);
          uVar33 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar36 <= uVar33) goto LAB_14231cd3e;
LAB_14231cccc:
          uVar36 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar33 * 8);
          if (uVar36 == 0) goto LAB_14231cd3e;
          local_e8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar36 * 0x30);
        }
        else {
          local_f0 = 0;
          uVar33 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar33 < uVar36) goto LAB_14231cccc;
LAB_14231cd3e:
          local_e8 = 0;
        }
        local_e0 = 0;
      }
      uVar36 = local_e0 & 0xffffffff;
    }
    piVar38 = (int *)(uVar36 * 0x10 + local_f0);
    pfVar34 = (float *)(uVar36 * 0x30 + local_e8);
    iVar11 = *piVar38;
    if (local_108 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
    }
    local_e0 = local_e0 + 1;
    *(int *)(local_108 * 4 + 4) = iVar11;
    local_108 = local_108 + 1;
    fVar23 = pfVar34[7];
    fVar24 = pfVar34[8];
    fVar25 = pfVar34[9];
    fVar26 = *pfVar34;
    fVar27 = pfVar34[1];
    fVar28 = pfVar34[3];
    fVar52 = pfVar34[2];
    fVar54 = fVar52 + fVar52;
    fVar49 = fVar27 + fVar27;
    fVar50 = fVar26 * (fVar26 + fVar26);
    fVar48 = (fVar26 + fVar26) * fVar28;
    fVar53 = (float)piVar38[1];
    fVar5 = (float)piVar38[2];
    fVar6 = (float)piVar38[3];
    fVar51 = fVar6 * fVar25 * (fVar49 * fVar28 + fVar54 * fVar26) +
             fVar5 * fVar24 * (fVar49 * fVar26 - fVar28 * fVar54) +
             fVar53 * fVar23 * (1.0 - (fVar27 * fVar49 + fVar52 * fVar54)) + pfVar34[4];
    fVar52 = fVar6 * fVar25 * (fVar54 * fVar27 - fVar48) +
             fVar5 * fVar24 * (1.0 - (fVar52 * fVar54 + fVar50)) +
             fVar53 * fVar23 * (fVar49 * fVar26 + fVar28 * fVar54) +
             (float)*(undefined8 *)(pfVar34 + 5);
    fVar53 = fVar6 * fVar25 * (1.0 - (fVar50 + fVar27 * fVar49)) +
             fVar5 * fVar24 * (fVar48 + fVar54 * fVar27) +
             fVar53 * fVar23 * (fVar54 * fVar26 - fVar49 * fVar28) +
             (float)((ulonglong)*(undefined8 *)(pfVar34 + 5) >> 0x20);
    if (in_R8[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar2 = *in_R8;
      bVar40 = (byte)(extraout_RAX_02 >> 0x39);
      auVar42 = ZEXT216(CONCAT11(bVar40,bVar40));
      auVar42 = pshuflw(auVar42,auVar42,0);
      lVar37 = 0;
      uVar36 = extraout_RAX_02;
      do {
        uVar36 = in_R8[1] & uVar36;
        pcVar3 = (char *)(lVar2 + uVar36);
        cVar41 = auVar42[0];
        auVar47[0] = -(*pcVar3 == cVar41);
        cVar43 = auVar42[1];
        auVar47[1] = -(pcVar3[1] == cVar43);
        cVar44 = auVar42[2];
        auVar47[2] = -(pcVar3[2] == cVar44);
        cVar45 = auVar42[3];
        auVar47[3] = -(pcVar3[3] == cVar45);
        auVar47[4] = -(pcVar3[4] == cVar41);
        auVar47[5] = -(pcVar3[5] == cVar43);
        auVar47[6] = -(pcVar3[6] == cVar44);
        auVar47[7] = -(pcVar3[7] == cVar45);
        auVar47[8] = -(pcVar3[8] == cVar41);
        auVar47[9] = -(pcVar3[9] == cVar43);
        auVar47[10] = -(pcVar3[10] == cVar44);
        auVar47[0xb] = -(pcVar3[0xb] == cVar45);
        auVar47[0xc] = -(pcVar3[0xc] == cVar41);
        auVar47[0xd] = -(pcVar3[0xd] == cVar43);
        auVar47[0xe] = -(pcVar3[0xe] == cVar44);
        auVar47[0xf] = -(pcVar3[0xf] == cVar45);
        uVar32 = (ushort)(SUB161(auVar47 >> 7,0) & 1) | (ushort)(SUB161(auVar47 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar47 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar47 >> 0x77,0) & 1) << 0xe | (ushort)(auVar47[0xf] >> 7) << 0xf
        ;
        uVar29 = (uint)uVar32;
        if (uVar32 != 0) {
          do {
            uVar17 = 0;
            for (uVar18 = uVar29; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
              uVar17 = uVar17 + 1;
            }
            uVar33 = uVar17 + uVar36 & in_R8[1];
            if (*piVar38 == *(int *)(lVar2 + -0x10 + uVar33 * -0x10)) {
              uVar36 = *(ulonglong *)(lVar2 + -8 + uVar33 * -0x10);
              uVar33 = uVar36 & 0xffffffff;
              if ((uVar33 < *(ulonglong *)(lVar15 + 0x10)) &&
                 (*(int *)(*(longlong *)(lVar15 + 8) + uVar33 * 0x14) == (int)(uVar36 >> 0x20))) {
                lVar2 = *(longlong *)(lVar15 + 8) + uVar33 * 0x14;
                uVar36 = (ulonglong)*(uint *)(lVar2 + 4);
                if ((uVar36 != 0xffffffff) &&
                   ((uVar36 < *(ulonglong *)(lVar14 + 0x38) &&
                    ((*(ulonglong *)
                       (*(longlong *)(lVar14 + 0x28) + (ulonglong)(*(uint *)(lVar2 + 4) >> 6) * 8)
                      >> (uVar36 & 0x3f) & 1) != 0)))) {
                  if (*(ulonglong *)(lVar14 + 0x110) <
                      *(ulonglong *)
                       (*(longlong *)(lVar15 + 0x1a8) + 0x40 +
                       (ulonglong)*(uint *)(lVar2 + 0xc) * 0x48)) {
                    lVar37 = *(longlong *)(lVar15 + 0x1a8) +
                             (ulonglong)*(uint *)(lVar2 + 0xc) * 0x48;
                    uVar36 = *(ulonglong *)
                              (*(longlong *)(lVar37 + 0x38) + *(ulonglong *)(lVar14 + 0x110) * 8);
                    if (uVar36 == 0) goto LAB_14231d172;
                    lVar37 = *(longlong *)(*(longlong *)(lVar37 + 0x18) + 0x10 + ~uVar36 * 0x30);
                  }
                  else {
LAB_14231d172:
                    lVar37 = 0;
                  }
                  if (*(longlong *)(lVar37 + 8 + (ulonglong)*(uint *)(lVar2 + 0x10) * 0x18) == 0) {
                    // WARNING: Subroutine does not return
                    core::panicking::panic_bounds_check();
                  }
                  lVar2 = *(longlong *)(lVar37 + (ulonglong)*(uint *)(lVar2 + 0x10) * 0x18);
                  if ((*(float *)(lVar2 + 8) == fVar51) &&
                     (!NAN(*(float *)(lVar2 + 8)) && !NAN(fVar51))) {
                    if ((*(float *)(lVar2 + 0xc) == fVar52) &&
                       (!NAN(*(float *)(lVar2 + 0xc)) && !NAN(fVar52))) {
                      if ((*(float *)(lVar2 + 0x10) == fVar53) &&
                         (!NAN(*(float *)(lVar2 + 0x10)) && !NAN(fVar53))) goto LAB_14231cb3b;
                    }
                  }
                }
              }
              core::hash::BuildHasher::
              hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                        ();
              hashbrown::raw::RawTable::
              remove_entry<tuple_<country_core::resources::walls::decorator::DecoratorId,bevy_ecs::entity::Entity>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId,bevy_ecs::entity::Entity>_>
                        ();
              uVar36 = 0;
              if (local_170._4_4_ != 0) {
                uVar36 = local_170;
              }
              if ((uVar36 >> 0x20 != 0) &&
                 (bevy_ecs::entity::Entities::contains(), extraout_AL_00 != '\0')) {
                local_170 = *(ulonglong *)(in_stack_00000028 + 8);
                bevy_ecs::system::commands::EntityCommands::despawn();
              }
              goto LAB_14231cb3b;
            }
            uVar32 = (ushort)(uVar29 - 1) & (ushort)uVar29;
            uVar29 = CONCAT22((short)(uVar29 - 1 >> 0x10),uVar32);
          } while (uVar32 != 0);
        }
        auVar46[0] = -(*pcVar3 == (char)iVar19);
        auVar46[1] = -(pcVar3[1] == (char)((uint)iVar19 >> 8));
        auVar46[2] = -(pcVar3[2] == (char)((uint)iVar19 >> 0x10));
        auVar46[3] = -(pcVar3[3] == (char)((uint)iVar19 >> 0x18));
        auVar46[4] = -(pcVar3[4] == (char)iVar20);
        auVar46[5] = -(pcVar3[5] == (char)((uint)iVar20 >> 8));
        auVar46[6] = -(pcVar3[6] == (char)((uint)iVar20 >> 0x10));
        auVar46[7] = -(pcVar3[7] == (char)((uint)iVar20 >> 0x18));
        auVar46[8] = -(pcVar3[8] == (char)iVar21);
        auVar46[9] = -(pcVar3[9] == (char)((uint)iVar21 >> 8));
        auVar46[10] = -(pcVar3[10] == (char)((uint)iVar21 >> 0x10));
        auVar46[0xb] = -(pcVar3[0xb] == (char)((uint)iVar21 >> 0x18));
        auVar46[0xc] = -(pcVar3[0xc] == (char)iVar22);
        auVar46[0xd] = -(pcVar3[0xd] == (char)((uint)iVar22 >> 8));
        auVar46[0xe] = -(pcVar3[0xe] == (char)((uint)iVar22 >> 0x10));
        auVar46[0xf] = -(pcVar3[0xf] == (char)((uint)iVar22 >> 0x18));
        if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 || (SUB161(auVar46 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x1f,0) & 1) != 0) || (SUB161(auVar46 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x37,0) & 1) != 0) || (SUB161(auVar46 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x47,0) & 1) != 0) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
        uVar36 = uVar36 + lVar37 + 0x10;
        lVar37 = lVar37 + 0x10;
      } while( true );
    }
    *puVar16 = uVar10;
    country_core::resources::critters::BirdSpotManager::gen_id();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_03 == 0) {
                    // WARNING: Subroutine does not return
      alloc::alloc::handle_alloc_error();
    }
    *(undefined4 *)(extraout_RAX_03 + 4) = 0;
    *(ulonglong *)(extraout_RAX_03 + 8) = CONCAT44(fVar52,fVar51);
    *(float *)(extraout_RAX_03 + 0x10) = fVar53;
    *(undefined4 *)(extraout_RAX_03 + 0x14) = extraout_EAX;
    country_core::resources::critters::BirdSpotManager::create_spots();
    bevy_ecs::system::commands::Commands::spawn_empty();
    bevy_ecs::system::commands::EntityCommands::
    queue<bevy_ecs::system::commands::entity_command::insert::closure_env_0<country_core::resources::critters::BirdSpots>,tuple_<>,enum2_<core::result::Result<tuple_<>,enum2_<bevy_ecs::world::error::EntityMutableFetchError>_>_>_>
              ();
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::decorator::DecoratorId,bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    local_170 = extraout_RAX_03;
  } while( true );
LAB_14231d25c:
  hashbrown::map::HashMap::
  retain<country_core::resources::walls::decorator::DecoratorId,bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global,system_critters::wren::update_decorator_bird_spots::closure_env_0>
            ();
  if (extraout_AL != '\0') {
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_04 == 1) {
      plVar30 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar30 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_14231d370:
                    // WARNING: Subroutine does not return
      std::thread::local::panic_access_error();
    }
    if (*plVar30 != 0) {
LAB_14231d37c:
                    // WARNING: Subroutine does not return
      core::cell::panic_already_borrowed();
    }
    *plVar30 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar30 = *plVar30 + 1;
  }
  return;
}

