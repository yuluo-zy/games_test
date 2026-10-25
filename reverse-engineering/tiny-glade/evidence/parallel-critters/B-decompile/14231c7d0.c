
/* WARNING: Removing unreachable block (ram,0x00014231d282) */

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
  float fVar29;
  char extraout_AL;
  char extraout_AL_00;
  uint uVar30;
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
  longlong *plVar31;
  ulonglong in_RDX;
  ulonglong uVar32;
  ushort uVar33;
  ulonglong uVar34;
  float *pfVar35;
  longlong *in_R8;
  undefined *puVar36;
  ulonglong uVar37;
  longlong *in_R9;
  longlong lVar38;
  int *piVar39;
  uint *puVar40;
  byte bVar41;
  char cVar42;
  char cVar44;
  char cVar45;
  char cVar46;
  undefined1 auVar43 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  float fVar49;
  float fVar50;
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
  float fVar51;
  float fVar52;
  float fVar53;
  float fVar54;
  float fVar55;
  longlong in_stack_00000028;
  undefined8 local_170;
  longlong local_138;
  longlong local_108;
  longlong local_f0;
  longlong local_e8;
  ulonglong local_e0;
  
  uVar32 = in_RDX;
  puffin::are_scopes_on();
  if (extraout_AL != '\0') {
    core::str::impl$0::rfind<ref_<str_>_>();
    if (extraout_RAX == 1) {
      if (uVar32 != 0) {
        if (uVar32 < 0x32) {
          if ((char)(&DAT_14304b468)[uVar32] < -0x40) {
LAB_14231c890:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar32 != 0x32) goto LAB_14231c890;
      }
      core::str::impl$0::rfind<ref_<str_>_>();
      if ((extraout_RAX_00 == 1) && (uVar32 != 0xfffffffffffffffe)) {
        if (uVar32 + 2 < 0x32) {
          if ("stem_critters::wren::update_decorator_bird_spots::f"[uVar32] < -0x40) {
LAB_14231c8e4:
                    /* WARNING: Subroutine does not return */
            core::str::slice_error_fail();
          }
        }
        else if (uVar32 + 2 != 0x32) goto LAB_14231c8e4;
      }
    }
    puVar36 = &DAT_14304b58b;
    do {
      bVar41 = puVar36[-1];
      uVar30 = (uint)(char)bVar41;
      if ((char)bVar41 < '\0') {
        bVar7 = puVar36[-2];
        if ((char)bVar7 < -0x40) {
          bVar8 = puVar36[-3];
          if ((char)bVar8 < -0x40) {
            pbVar1 = puVar36 + -4;
            puVar36 = puVar36 + -4;
            uVar30 = bVar8 & 0x3f | (*pbVar1 & 7) << 6;
          }
          else {
            puVar36 = puVar36 + -3;
            uVar30 = bVar8 & 0xf;
          }
          uVar30 = bVar7 & 0x3f | uVar30 << 6;
        }
        else {
          puVar36 = puVar36 + -2;
          uVar30 = bVar7 & 0x1f;
        }
        uVar30 = (uint)(bVar41 & 0x3f) | uVar30 << 6;
        if (uVar30 != 0x5c) goto LAB_14231c9a4;
LAB_14231c9b5:
        if (puVar36 == &UNK_14304b567) break;
        if (puVar36 + -0x14304b567 < (undefined *)0x23) {
          if (-0x41 < (char)puVar36[1]) break;
        }
        else if (puVar36 + -0x14304b567 == (undefined *)0x23) break;
                    /* WARNING: Subroutine does not return */
        core::str::slice_error_fail();
      }
      puVar36 = puVar36 + -1;
      if (uVar30 == 0x5c) goto LAB_14231c9b5;
LAB_14231c9a4:
      if (uVar30 == 0x2f) goto LAB_14231c9b5;
    } while (puVar36 != &DAT_14304b568);
    puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL();
    if (*extraout_RAX_01 == 1) {
      plVar31 = extraout_RAX_01 + 1;
    }
    else if ((*extraout_RAX_01 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar31 = extraout_RAX_05, extraout_RAX_05 == (longlong *)0x0))
    goto LAB_14231d370;
    if (*plVar31 != 0) goto LAB_14231d37c;
    *plVar31 = -1;
    puffin::ThreadProfiler::begin_scope();
    *plVar31 = *plVar31 + 1;
  }
  fVar29 = __real_3f800000;
  local_108 = 0;
  lVar12 = *in_RCX;
  lVar13 = in_RCX[1];
  puVar40 = *(uint **)(lVar12 + 0xf8);
  puVar4 = puVar40 + *(longlong *)(lVar12 + 0x100);
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
  uVar32 = 0;
  local_e0 = 0;
LAB_14231cb3b:
  do {
    if (cVar9 == '\0') {
      if (local_e0 == uVar32) {
        do {
          if (puVar40 == puVar4) goto LAB_14231d25c;
          uVar30 = *puVar40;
          puVar40 = puVar40 + 1;
          uVar32 = *(ulonglong *)(*(longlong *)(lVar13 + 0x100) + 0x58 + (ulonglong)uVar30 * 0xa0);
        } while (uVar32 == 0);
        lVar38 = *(longlong *)(lVar13 + 0x100) + (ulonglong)uVar30 * 0xa0;
        uVar37 = (ulonglong)*(uint *)(lVar38 + 0x94);
        lVar2 = *(longlong *)(lVar13 + 0x1a8) + uVar37 * 0x48;
        uVar37 = *(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x40 + uVar37 * 0x48);
        if ((*(ulonglong *)(lVar12 + 0x110) < uVar37) &&
           (uVar34 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
           uVar34 != 0)) {
          local_f0 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar34 * 0x30);
          uVar34 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar37 <= uVar34) goto LAB_14231cd57;
LAB_14231cd14:
          uVar37 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar34 * 8);
          if (uVar37 == 0) goto LAB_14231cd57;
          local_e8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar37 * 0x30);
        }
        else {
          local_f0 = 0;
          uVar34 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar34 < uVar37) goto LAB_14231cd14;
LAB_14231cd57:
          local_e8 = 0;
        }
        local_138 = *(longlong *)(lVar38 + 0x50);
        local_e0 = 0;
      }
      uVar37 = (ulonglong)*(uint *)(local_138 + 8 + local_e0 * 0x10);
    }
    else {
      if (local_e0 == uVar32) {
        do {
          if (puVar40 == puVar4) goto LAB_14231d25c;
          uVar30 = *puVar40;
          puVar40 = puVar40 + 1;
          uVar32 = *(ulonglong *)(*(longlong *)(lVar13 + 0x1a8) + 0x10 + (ulonglong)uVar30 * 0x48);
        } while (uVar32 == 0);
        lVar2 = *(longlong *)(lVar13 + 0x1a8) + (ulonglong)uVar30 * 0x48;
        uVar37 = *(ulonglong *)(lVar2 + 0x40);
        if ((*(ulonglong *)(lVar12 + 0x110) < uVar37) &&
           (uVar34 = *(ulonglong *)
                      (*(longlong *)(lVar2 + 0x38) + *(ulonglong *)(lVar12 + 0x110) * 8),
           uVar34 != 0)) {
          local_f0 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar34 * 0x30);
          uVar34 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar37 <= uVar34) goto LAB_14231cd3e;
LAB_14231cccc:
          uVar37 = *(ulonglong *)(*(longlong *)(lVar2 + 0x38) + uVar34 * 8);
          if (uVar37 == 0) goto LAB_14231cd3e;
          local_e8 = *(longlong *)(*(longlong *)(lVar2 + 0x18) + 0x10 + ~uVar37 * 0x30);
        }
        else {
          local_f0 = 0;
          uVar34 = *(ulonglong *)(lVar12 + 0x118);
          if (uVar34 < uVar37) goto LAB_14231cccc;
LAB_14231cd3e:
          local_e8 = 0;
        }
        local_e0 = 0;
      }
      uVar37 = local_e0 & 0xffffffff;
    }
    piVar39 = (int *)(uVar37 * 0x10 + local_f0);
    pfVar35 = (float *)(uVar37 * 0x30 + local_e8);
    iVar11 = *piVar39;
    if (local_108 == 0) {
      alloc::raw_vec::RawVec::
      grow_one<country_core::render::sparse_instance_buffer::ChunkId,alloc::alloc::Global>();
    }
    local_e0 = local_e0 + 1;
    *(int *)(local_108 * 4 + 4) = iVar11;
    local_108 = local_108 + 1;
    fVar23 = pfVar35[7];
    fVar24 = pfVar35[8];
    fVar25 = pfVar35[9];
    fVar26 = *pfVar35;
    fVar27 = pfVar35[1];
    fVar28 = pfVar35[3];
    fVar53 = pfVar35[2];
    fVar55 = fVar53 + fVar53;
    fVar50 = fVar27 + fVar27;
    fVar51 = fVar26 * (fVar26 + fVar26);
    fVar49 = (fVar26 + fVar26) * fVar28;
    fVar54 = (float)piVar39[1];
    fVar5 = (float)piVar39[2];
    fVar6 = (float)piVar39[3];
    fVar52 = fVar6 * fVar25 * (fVar50 * fVar28 + fVar55 * fVar26) +
             fVar5 * fVar24 * (fVar50 * fVar26 - fVar28 * fVar55) +
             fVar54 * fVar23 * (fVar29 - (fVar27 * fVar50 + fVar53 * fVar55)) + pfVar35[4];
    fVar53 = fVar6 * fVar25 * (fVar55 * fVar27 - fVar49) +
             fVar5 * fVar24 * (fVar29 - (fVar53 * fVar55 + fVar51)) +
             fVar54 * fVar23 * (fVar50 * fVar26 + fVar28 * fVar55) +
             (float)*(undefined8 *)(pfVar35 + 5);
    fVar54 = fVar6 * fVar25 * (fVar29 - (fVar51 + fVar27 * fVar50)) +
             fVar5 * fVar24 * (fVar49 + fVar55 * fVar27) +
             fVar54 * fVar23 * (fVar55 * fVar26 - fVar50 * fVar28) +
             (float)((ulonglong)*(undefined8 *)(pfVar35 + 5) >> 0x20);
    if (in_R8[3] != 0) {
      core::hash::BuildHasher::
      hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
      lVar2 = *in_R8;
      bVar41 = (byte)(extraout_RAX_02 >> 0x39);
      auVar43 = ZEXT216(CONCAT11(bVar41,bVar41));
      auVar43 = pshuflw(auVar43,auVar43,0);
      lVar38 = 0;
      uVar37 = extraout_RAX_02;
      do {
        uVar37 = in_R8[1] & uVar37;
        pcVar3 = (char *)(lVar2 + uVar37);
        cVar42 = auVar43[0];
        auVar48[0] = -(*pcVar3 == cVar42);
        cVar44 = auVar43[1];
        auVar48[1] = -(pcVar3[1] == cVar44);
        cVar45 = auVar43[2];
        auVar48[2] = -(pcVar3[2] == cVar45);
        cVar46 = auVar43[3];
        auVar48[3] = -(pcVar3[3] == cVar46);
        auVar48[4] = -(pcVar3[4] == cVar42);
        auVar48[5] = -(pcVar3[5] == cVar44);
        auVar48[6] = -(pcVar3[6] == cVar45);
        auVar48[7] = -(pcVar3[7] == cVar46);
        auVar48[8] = -(pcVar3[8] == cVar42);
        auVar48[9] = -(pcVar3[9] == cVar44);
        auVar48[10] = -(pcVar3[10] == cVar45);
        auVar48[0xb] = -(pcVar3[0xb] == cVar46);
        auVar48[0xc] = -(pcVar3[0xc] == cVar42);
        auVar48[0xd] = -(pcVar3[0xd] == cVar44);
        auVar48[0xe] = -(pcVar3[0xe] == cVar45);
        auVar48[0xf] = -(pcVar3[0xf] == cVar46);
        uVar33 = (ushort)(SUB161(auVar48 >> 7,0) & 1) | (ushort)(SUB161(auVar48 >> 0xf,0) & 1) << 1
                 | (ushort)(SUB161(auVar48 >> 0x17,0) & 1) << 2 |
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
                 (ushort)(SUB161(auVar48 >> 0x77,0) & 1) << 0xe | (ushort)(auVar48[0xf] >> 7) << 0xf
        ;
        uVar30 = (uint)uVar33;
        if (uVar33 != 0) {
          do {
            uVar17 = 0;
            for (uVar18 = uVar30; (uVar18 & 1) == 0; uVar18 = uVar18 >> 1 | 0x80000000) {
              uVar17 = uVar17 + 1;
            }
            uVar34 = uVar17 + uVar37 & in_R8[1];
            if (*piVar39 == *(int *)(lVar2 + -0x10 + uVar34 * -0x10)) {
              uVar37 = *(ulonglong *)(lVar2 + -8 + uVar34 * -0x10);
              uVar34 = uVar37 & 0xffffffff;
              if ((uVar34 < *(ulonglong *)(lVar15 + 0x10)) &&
                 (*(int *)(*(longlong *)(lVar15 + 8) + uVar34 * 0x14) == (int)(uVar37 >> 0x20))) {
                lVar2 = *(longlong *)(lVar15 + 8) + uVar34 * 0x14;
                uVar37 = (ulonglong)*(uint *)(lVar2 + 4);
                if ((uVar37 != 0xffffffff) &&
                   ((uVar37 < *(ulonglong *)(lVar14 + 0x38) &&
                    ((*(ulonglong *)
                       (*(longlong *)(lVar14 + 0x28) + (ulonglong)(*(uint *)(lVar2 + 4) >> 6) * 8)
                      >> (uVar37 & 0x3f) & 1) != 0)))) {
                  if (*(ulonglong *)(lVar14 + 0x110) <
                      *(ulonglong *)
                       (*(longlong *)(lVar15 + 0x1a8) + 0x40 +
                       (ulonglong)*(uint *)(lVar2 + 0xc) * 0x48)) {
                    lVar38 = *(longlong *)(lVar15 + 0x1a8) +
                             (ulonglong)*(uint *)(lVar2 + 0xc) * 0x48;
                    uVar37 = *(ulonglong *)
                              (*(longlong *)(lVar38 + 0x38) + *(ulonglong *)(lVar14 + 0x110) * 8);
                    if (uVar37 == 0) goto LAB_14231d172;
                    lVar38 = *(longlong *)(*(longlong *)(lVar38 + 0x18) + 0x10 + ~uVar37 * 0x30);
                  }
                  else {
LAB_14231d172:
                    lVar38 = 0;
                  }
                  if (*(longlong *)(lVar38 + 8 + (ulonglong)*(uint *)(lVar2 + 0x10) * 0x18) == 0) {
                    /* WARNING: Subroutine does not return */
                    core::panicking::panic_bounds_check();
                  }
                  lVar2 = *(longlong *)(lVar38 + (ulonglong)*(uint *)(lVar2 + 0x10) * 0x18);
                  if ((*(float *)(lVar2 + 8) == fVar52) &&
                     (!NAN(*(float *)(lVar2 + 8)) && !NAN(fVar52))) {
                    if ((*(float *)(lVar2 + 0xc) == fVar53) &&
                       (!NAN(*(float *)(lVar2 + 0xc)) && !NAN(fVar53))) {
                      if ((*(float *)(lVar2 + 0x10) == fVar54) &&
                         (!NAN(*(float *)(lVar2 + 0x10)) && !NAN(fVar54))) goto LAB_14231cb3b;
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
              uVar37 = 0;
              if (local_170._4_4_ != 0) {
                uVar37 = local_170;
              }
              if ((uVar37 >> 0x20 != 0) &&
                 (bevy_ecs::entity::Entities::contains(), extraout_AL_00 != '\0')) {
                local_170 = *(ulonglong *)(in_stack_00000028 + 8);
                bevy_ecs::system::commands::EntityCommands::despawn();
              }
              goto LAB_14231cb3b;
            }
            uVar33 = (ushort)(uVar30 - 1) & (ushort)uVar30;
            uVar30 = CONCAT22((short)(uVar30 - 1 >> 0x10),uVar33);
          } while (uVar33 != 0);
        }
        auVar47[0] = -(*pcVar3 == (char)iVar19);
        auVar47[1] = -(pcVar3[1] == (char)((uint)iVar19 >> 8));
        auVar47[2] = -(pcVar3[2] == (char)((uint)iVar19 >> 0x10));
        auVar47[3] = -(pcVar3[3] == (char)((uint)iVar19 >> 0x18));
        auVar47[4] = -(pcVar3[4] == (char)iVar20);
        auVar47[5] = -(pcVar3[5] == (char)((uint)iVar20 >> 8));
        auVar47[6] = -(pcVar3[6] == (char)((uint)iVar20 >> 0x10));
        auVar47[7] = -(pcVar3[7] == (char)((uint)iVar20 >> 0x18));
        auVar47[8] = -(pcVar3[8] == (char)iVar21);
        auVar47[9] = -(pcVar3[9] == (char)((uint)iVar21 >> 8));
        auVar47[10] = -(pcVar3[10] == (char)((uint)iVar21 >> 0x10));
        auVar47[0xb] = -(pcVar3[0xb] == (char)((uint)iVar21 >> 0x18));
        auVar47[0xc] = -(pcVar3[0xc] == (char)iVar22);
        auVar47[0xd] = -(pcVar3[0xd] == (char)((uint)iVar22 >> 8));
        auVar47[0xe] = -(pcVar3[0xe] == (char)((uint)iVar22 >> 0x10));
        auVar47[0xf] = -(pcVar3[0xf] == (char)((uint)iVar22 >> 0x18));
        if ((((((((((((((((SUB161(auVar47 >> 7,0) & 1) != 0 || (SUB161(auVar47 >> 0xf,0) & 1) != 0)
                        || (SUB161(auVar47 >> 0x17,0) & 1) != 0) ||
                       (SUB161(auVar47 >> 0x1f,0) & 1) != 0) || (SUB161(auVar47 >> 0x27,0) & 1) != 0
                      ) || (SUB161(auVar47 >> 0x2f,0) & 1) != 0) ||
                    (SUB161(auVar47 >> 0x37,0) & 1) != 0) || (SUB161(auVar47 >> 0x3f,0) & 1) != 0)
                  || (SUB161(auVar47 >> 0x47,0) & 1) != 0) || (SUB161(auVar47 >> 0x4f,0) & 1) != 0)
                || (SUB161(auVar47 >> 0x57,0) & 1) != 0) || (SUB161(auVar47 >> 0x5f,0) & 1) != 0) ||
              (SUB161(auVar47 >> 0x67,0) & 1) != 0) || (SUB161(auVar47 >> 0x6f,0) & 1) != 0) ||
            (SUB161(auVar47 >> 0x77,0) & 1) != 0) || auVar47[0xf] < '\0') break;
        uVar37 = uVar37 + lVar38 + 0x10;
        lVar38 = lVar38 + 0x10;
      } while( true );
    }
    *puVar16 = uVar10;
    country_core::resources::critters::BirdSpotManager::gen_id();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_03 == 0) {
                    /* WARNING: Subroutine does not return */
      alloc::alloc::handle_alloc_error();
    }
    *(undefined4 *)(extraout_RAX_03 + 4) = 0;
    *(ulonglong *)(extraout_RAX_03 + 8) = CONCAT44(fVar53,fVar52);
    *(float *)(extraout_RAX_03 + 0x10) = fVar54;
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
      plVar31 = extraout_RAX_04 + 1;
    }
    else if ((*extraout_RAX_04 != 0) ||
            (std::sys::thread_local::native::lazy::Storage::
             initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple_<>,core::cell::RefCell<puffin::ThreadProfiler>_(*)()>
                       (), plVar31 = extraout_RAX_06, extraout_RAX_06 == (longlong *)0x0)) {
LAB_14231d370:
                    /* WARNING: Subroutine does not return */
      std::thread::local::panic_access_error();
    }
    if (*plVar31 != 0) {
LAB_14231d37c:
                    /* WARNING: Subroutine does not return */
      core::cell::panic_already_borrowed();
    }
    *plVar31 = -1;
    puffin::ThreadProfiler::end_scope();
    *plVar31 = *plVar31 + 1;
  }
  return;
}

