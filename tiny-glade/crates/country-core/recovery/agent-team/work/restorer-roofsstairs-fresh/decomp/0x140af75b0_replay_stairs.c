
// WARNING: Removing unreachable block (ram,0x000140af7e9f)

void __cdecl country_core::systems::history::replay_stairs::replay_stairs(void)

{
  longlong lVar1;
  char *pcVar2;
  undefined4 *puVar3;
  float fVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  int iVar7;
  ulonglong *puVar8;
  char *pcVar9;
  undefined4 *puVar10;
  longlong *plVar11;
  longlong lVar12;
  undefined4 *puVar13;
  uint uVar14;
  uint uVar15;
  int iVar16;
  int iVar17;
  int iVar18;
  int iVar19;
  longlong *plVar20;
  undefined1 auVar21 [16];
  ushort uVar22;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  ulonglong extraout_RAX_01;
  ulonglong extraout_RAX_02;
  longlong *in_RCX;
  longlong lVar23;
  longlong *in_RDX;
  longlong lVar24;
  undefined8 *puVar25;
  longlong in_R8;
  ulonglong uVar26;
  undefined1 (*pauVar27) [16];
  longlong lVar28;
  longlong *in_R9;
  longlong lVar29;
  ulonglong uVar30;
  uint uVar31;
  longlong lVar32;
  ulonglong uVar33;
  undefined8 uVar34;
  char cVar35;
  longlong lVar36;
  byte bVar37;
  char cVar38;
  char cVar41;
  char cVar42;
  char cVar43;
  undefined1 auVar39 [16];
  undefined1 auVar40 [16];
  float fVar44;
  undefined1 auVar45 [16];
  undefined1 auVar46 [16];
  undefined1 auVar47 [16];
  undefined1 auVar48 [16];
  undefined1 auVar49 [16];
  undefined1 auVar50 [16];
  undefined1 unaff_XMM8_Ba;
  undefined1 unaff_XMM8_Bb;
  char cVar51;
  undefined1 unaff_XMM8_Bc;
  char cVar52;
  undefined1 unaff_XMM8_Bd;
  char cVar53;
  undefined1 unaff_XMM8_Be;
  undefined1 unaff_XMM8_Bf;
  char cVar54;
  undefined1 unaff_XMM8_Bg;
  char cVar55;
  undefined1 unaff_XMM8_Bh;
  char cVar56;
  undefined1 unaff_XMM8_Bi;
  undefined1 unaff_XMM8_Bj;
  char cVar57;
  undefined1 unaff_XMM8_Bk;
  char cVar58;
  undefined1 unaff_XMM8_Bl;
  char cVar59;
  undefined1 unaff_XMM8_Bm;
  undefined1 unaff_XMM8_Bn;
  char cVar60;
  undefined1 unaff_XMM8_Bo;
  char cVar61;
  undefined1 unaff_XMM8_Bp;
  char cVar62;
  undefined4 uVar63;
  undefined4 uVar64;
  longlong *in_stack_00000028;
  undefined1 local_118;
  undefined7 uStack_117;
  undefined7 uStack_110;
  undefined4 uStack_109;
  undefined4 uStack_105;
  undefined4 uStack_101;
  ulonglong local_c0;
  ulonglong *local_b8;
  undefined8 local_a0;
  undefined4 local_98;
  
  puVar8 = (ulonglong *)*in_RCX;
  fVar4 = *(float *)(puVar8 + 5);
  *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
  if (puVar8[3] == 0) {
    return;
  }
  uVar26 = puVar8[1];
  uVar33 = *puVar8;
  if (puVar8[2] < *puVar8) {
    uVar33 = 0;
  }
  lVar32 = (puVar8[2] - uVar33) * 0x40;
  lVar1 = uVar26 + lVar32;
  local_c0 = 0;
  pcVar9 = *(char **)(uVar26 + 8 + lVar32);
  lVar32 = *(longlong *)(uVar26 + 0x10 + lVar32);
  lVar28 = *in_RDX;
  uVar5 = *(undefined4 *)(in_R8 + 0x1c);
  puVar10 = *(undefined4 **)(in_R8 + 0x10);
  plVar11 = (longlong *)*in_stack_00000028;
  uVar6 = *(undefined4 *)((longlong)in_R9 + 0x1c);
  lVar12 = *in_R9;
  puVar13 = (undefined4 *)in_R9[2];
  iVar16 = -(uint)(CONCAT13(unaff_XMM8_Bd,
                            CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))) ==
                  CONCAT13(unaff_XMM8_Bd,
                           CONCAT12(unaff_XMM8_Bc,CONCAT11(unaff_XMM8_Bb,unaff_XMM8_Ba))));
  cVar51 = (char)((uint)iVar16 >> 8);
  cVar52 = (char)((uint)iVar16 >> 0x10);
  cVar53 = (char)((uint)iVar16 >> 0x18);
  iVar17 = -(uint)(CONCAT13(unaff_XMM8_Bh,
                            CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))) ==
                  CONCAT13(unaff_XMM8_Bh,
                           CONCAT12(unaff_XMM8_Bg,CONCAT11(unaff_XMM8_Bf,unaff_XMM8_Be))));
  cVar54 = (char)((uint)iVar17 >> 8);
  cVar55 = (char)((uint)iVar17 >> 0x10);
  cVar56 = (char)((uint)iVar17 >> 0x18);
  iVar18 = -(uint)(CONCAT13(unaff_XMM8_Bl,
                            CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))) ==
                  CONCAT13(unaff_XMM8_Bl,
                           CONCAT12(unaff_XMM8_Bk,CONCAT11(unaff_XMM8_Bj,unaff_XMM8_Bi))));
  cVar57 = (char)((uint)iVar18 >> 8);
  cVar58 = (char)((uint)iVar18 >> 0x10);
  cVar59 = (char)((uint)iVar18 >> 0x18);
  iVar19 = -(uint)(CONCAT13(unaff_XMM8_Bp,
                            CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))) ==
                  CONCAT13(unaff_XMM8_Bp,
                           CONCAT12(unaff_XMM8_Bo,CONCAT11(unaff_XMM8_Bn,unaff_XMM8_Bm))));
  cVar60 = (char)((uint)iVar19 >> 8);
  cVar61 = (char)((uint)iVar19 >> 0x10);
  cVar62 = (char)((uint)iVar19 >> 0x18);
  lVar36 = 0;
  puVar8 = (ulonglong *)pcVar9;
LAB_140af7722:
  do {
    local_b8 = puVar8;
    lVar24 = lVar36;
    if (local_b8 == (ulonglong *)(pcVar9 + lVar32 * 0x30)) break;
    puVar8 = (ulonglong *)((longlong)local_b8 + 0x30);
    lVar36 = lVar24 + 1;
    if ((char)*local_b8 == '\n') {
      if ((*(byte *)((longlong)local_b8 + 8) & 1) == 0) {
        if (local_c0 == 0) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(longlong *)(local_c0 * 8 + 8) = lVar24;
        lVar24 = *(longlong *)((longlong)local_b8 + 0x10) + 0x10;
        if ((*(byte *)(*(longlong *)((longlong)local_b8 + 0x10) + 0x10) & 1) == 0)
        goto LAB_140af77bd;
LAB_140af782f:
        resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info();
        if (extraout_RAX_00 == 0) {
          puVar25 = (undefined8 *)0x0;
        }
        else {
          puVar25 = (undefined8 *)(extraout_RAX_00 + 0x30);
          if (0xf9 < (*(byte *)(extraout_RAX_00 + 0x41) | 0xf8)) {
            puVar25 = (undefined8 *)0x0;
          }
        }
        cVar35 = *(char *)((longlong)local_b8 + 0x1c);
        *puVar10 = uVar5;
        if (cVar35 == '\0') {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                    ();
          hashbrown::raw::RawTable::
          remove_entry<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>_>
                    ();
        }
        else {
          hashbrown::map::HashMap::
          insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
        }
        cVar35 = '\x12' - *(char *)((longlong)local_b8 + 0x1c);
        iVar7 = *(int *)(lVar24 + 4);
        if (plVar11[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                    ();
          lVar24 = *plVar11;
          bVar37 = (byte)(extraout_RAX_02 >> 0x39);
          auVar39 = ZEXT216(CONCAT11(bVar37,bVar37));
          auVar39 = pshuflw(auVar39,auVar39,0);
          lVar29 = 0;
          uVar26 = extraout_RAX_02;
          while( true ) {
            uVar26 = plVar11[1] & uVar26;
            pcVar2 = (char *)(lVar24 + uVar26);
            cVar38 = auVar39[0];
            auVar48[0] = -(*pcVar2 == cVar38);
            cVar41 = auVar39[1];
            auVar48[1] = -(pcVar2[1] == cVar41);
            cVar42 = auVar39[2];
            auVar48[2] = -(pcVar2[2] == cVar42);
            cVar43 = auVar39[3];
            auVar48[3] = -(pcVar2[3] == cVar43);
            auVar48[4] = -(pcVar2[4] == cVar38);
            auVar48[5] = -(pcVar2[5] == cVar41);
            auVar48[6] = -(pcVar2[6] == cVar42);
            auVar48[7] = -(pcVar2[7] == cVar43);
            auVar48[8] = -(pcVar2[8] == cVar38);
            auVar48[9] = -(pcVar2[9] == cVar41);
            auVar48[10] = -(pcVar2[10] == cVar42);
            auVar48[0xb] = -(pcVar2[0xb] == cVar43);
            auVar48[0xc] = -(pcVar2[0xc] == cVar38);
            auVar48[0xd] = -(pcVar2[0xd] == cVar41);
            auVar48[0xe] = -(pcVar2[0xe] == cVar42);
            auVar48[0xf] = -(pcVar2[0xf] == cVar43);
            uVar22 = (ushort)(SUB161(auVar48 >> 7,0) & 1) |
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
            uVar31 = (uint)uVar22;
            while (uVar22 != 0) {
              uVar14 = 0;
              for (uVar15 = uVar31; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar14 = uVar14 + 1;
              }
              uVar33 = uVar14 + uVar26 & plVar11[1];
              if (*(int *)(lVar24 + -0x40 + uVar33 * -0x40) == iVar7) {
                lVar29 = uVar33 * -0x40;
                local_a0 = *(undefined8 *)(lVar24 + -0x20 + lVar29);
                local_98 = *(undefined4 *)(lVar24 + -0x18 + lVar29);
                goto joined_r0x000140af7b8d;
              }
              uVar22 = (ushort)(uVar31 - 1) & (ushort)uVar31;
              uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar22);
            }
            auVar46[0] = -(*pcVar2 == (char)iVar16);
            auVar46[1] = -(pcVar2[1] == cVar51);
            auVar46[2] = -(pcVar2[2] == cVar52);
            auVar46[3] = -(pcVar2[3] == cVar53);
            auVar46[4] = -(pcVar2[4] == (char)iVar17);
            auVar46[5] = -(pcVar2[5] == cVar54);
            auVar46[6] = -(pcVar2[6] == cVar55);
            auVar46[7] = -(pcVar2[7] == cVar56);
            auVar46[8] = -(pcVar2[8] == (char)iVar18);
            auVar46[9] = -(pcVar2[9] == cVar57);
            auVar46[10] = -(pcVar2[10] == cVar58);
            auVar46[0xb] = -(pcVar2[0xb] == cVar59);
            auVar46[0xc] = -(pcVar2[0xc] == (char)iVar19);
            auVar46[0xd] = -(pcVar2[0xd] == cVar60);
            auVar46[0xe] = -(pcVar2[0xe] == cVar61);
            auVar46[0xf] = -(pcVar2[0xf] == cVar62);
            if ((((((((((((((((SUB161(auVar46 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar46 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar46 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar46 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar46 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar46 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar46 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar46 >> 0x3f,0) & 1) != 0) || (SUB161(auVar46 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar46 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar46 >> 0x57,0) & 1) != 0) || (SUB161(auVar46 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar46 >> 0x67,0) & 1) != 0) || (SUB161(auVar46 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar46 >> 0x77,0) & 1) != 0) || auVar46[0xf] < '\0') break;
            uVar26 = uVar26 + lVar29 + 0x10;
            lVar29 = lVar29 + 0x10;
          }
        }
        local_a0 = 0;
        local_98 = 0;
joined_r0x000140af7b8d:
        if (puVar25 == (undefined8 *)0x0) {
          uVar63 = 0;
          uVar64 = 0;
        }
        else {
          uVar63 = (undefined4)*puVar25;
          uVar64 = (undefined4)((ulonglong)*puVar25 >> 0x20);
        }
        uStack_105 = (undefined4)local_a0;
        uStack_101 = (undefined4)((ulonglong)local_a0 >> 0x20);
        *puVar13 = uVar6;
        lVar24 = *(longlong *)(lVar12 + 0x30);
        uVar34 = *(undefined8 *)(lVar12 + 0x40);
        if (lVar24 == *(longlong *)(lVar12 + 0x20)) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
      }
      else {
        fVar44 = *(float *)((longlong)local_b8 + 0xc) / fVar4;
        if (fVar44 <= 0.05) {
          fVar44 = 0.05;
        }
        if (*(float *)((longlong)local_b8 + 0x18) <= fVar44) {
          *(float *)((longlong)local_b8 + 0x18) =
               *(float *)((longlong)local_b8 + 0x18) + *(float *)(lVar28 + 0x28);
          goto LAB_140af7722;
        }
        if (local_c0 == 0) {
          alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        }
        *(longlong *)(local_c0 * 8 + 8) = lVar24;
        *(float *)((longlong)local_b8 + 0x18) =
             *(float *)(lVar28 + 0x28) + *(float *)((longlong)local_b8 + 0x18);
        lVar24 = *(longlong *)((longlong)local_b8 + 0x10) + 0x10;
        if ((*(byte *)(*(longlong *)((longlong)local_b8 + 0x10) + 0x10) & 1) != 0)
        goto LAB_140af782f;
LAB_140af77bd:
        resources::walls::decorator_storage::DecoratorStorage::lookup_decorator_info();
        if (extraout_RAX == 0) {
          puVar25 = (undefined8 *)0x0;
        }
        else {
          puVar25 = (undefined8 *)(extraout_RAX + 0x30);
          if (0xf9 < (*(byte *)(extraout_RAX + 0x41) | 0xf8)) {
            puVar25 = (undefined8 *)0x0;
          }
        }
        cVar35 = *(char *)((longlong)local_b8 + 0x1c);
        *puVar10 = uVar5;
        if (cVar35 == '\0') {
          hashbrown::map::HashMap::
          insert<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
                    ();
        }
        else {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<country_core::resources::stairs::stairs_state::StairGraphEdge>_>
                    ();
          hashbrown::raw::RawTable::
          remove_entry<tuple_<country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>,alloc::alloc::Global,hashbrown::map::equivalent_key::closure_env_0<country_core::resources::stairs::stairs_state::StairGraphEdge,country_core::resources::stairs::stairs_state::StairGraphEdge,tuple_<>_>_>
                    ();
        }
        cVar35 = *(char *)((longlong)local_b8 + 0x1c);
        iVar7 = *(int *)(lVar24 + 4);
        if (plVar11[3] != 0) {
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                    ();
          lVar24 = *plVar11;
          bVar37 = (byte)(extraout_RAX_01 >> 0x39);
          auVar39 = ZEXT216(CONCAT11(bVar37,bVar37));
          auVar39 = pshuflw(auVar39,auVar39,0);
          lVar29 = 0;
          uVar26 = extraout_RAX_01;
          while( true ) {
            uVar26 = plVar11[1] & uVar26;
            pcVar2 = (char *)(lVar24 + uVar26);
            cVar38 = auVar39[0];
            auVar47[0] = -(*pcVar2 == cVar38);
            cVar41 = auVar39[1];
            auVar47[1] = -(pcVar2[1] == cVar41);
            cVar42 = auVar39[2];
            auVar47[2] = -(pcVar2[2] == cVar42);
            cVar43 = auVar39[3];
            auVar47[3] = -(pcVar2[3] == cVar43);
            auVar47[4] = -(pcVar2[4] == cVar38);
            auVar47[5] = -(pcVar2[5] == cVar41);
            auVar47[6] = -(pcVar2[6] == cVar42);
            auVar47[7] = -(pcVar2[7] == cVar43);
            auVar47[8] = -(pcVar2[8] == cVar38);
            auVar47[9] = -(pcVar2[9] == cVar41);
            auVar47[10] = -(pcVar2[10] == cVar42);
            auVar47[0xb] = -(pcVar2[0xb] == cVar43);
            auVar47[0xc] = -(pcVar2[0xc] == cVar38);
            auVar47[0xd] = -(pcVar2[0xd] == cVar41);
            auVar47[0xe] = -(pcVar2[0xe] == cVar42);
            auVar47[0xf] = -(pcVar2[0xf] == cVar43);
            uVar22 = (ushort)(SUB161(auVar47 >> 7,0) & 1) |
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
            uVar31 = (uint)uVar22;
            while (uVar22 != 0) {
              uVar14 = 0;
              for (uVar15 = uVar31; (uVar15 & 1) == 0; uVar15 = uVar15 >> 1 | 0x80000000) {
                uVar14 = uVar14 + 1;
              }
              uVar33 = uVar14 + uVar26 & plVar11[1];
              if (*(int *)(lVar24 + -0x40 + uVar33 * -0x40) == iVar7) {
                lVar29 = uVar33 * -0x40;
                local_a0 = *(undefined8 *)(lVar24 + -0x20 + lVar29);
                local_98 = *(undefined4 *)(lVar24 + -0x18 + lVar29);
                goto joined_r0x000140af79e5;
              }
              uVar22 = (ushort)(uVar31 - 1) & (ushort)uVar31;
              uVar31 = CONCAT22((short)(uVar31 - 1 >> 0x10),uVar22);
            }
            auVar45[0] = -(*pcVar2 == (char)iVar16);
            auVar45[1] = -(pcVar2[1] == cVar51);
            auVar45[2] = -(pcVar2[2] == cVar52);
            auVar45[3] = -(pcVar2[3] == cVar53);
            auVar45[4] = -(pcVar2[4] == (char)iVar17);
            auVar45[5] = -(pcVar2[5] == cVar54);
            auVar45[6] = -(pcVar2[6] == cVar55);
            auVar45[7] = -(pcVar2[7] == cVar56);
            auVar45[8] = -(pcVar2[8] == (char)iVar18);
            auVar45[9] = -(pcVar2[9] == cVar57);
            auVar45[10] = -(pcVar2[10] == cVar58);
            auVar45[0xb] = -(pcVar2[0xb] == cVar59);
            auVar45[0xc] = -(pcVar2[0xc] == (char)iVar19);
            auVar45[0xd] = -(pcVar2[0xd] == cVar60);
            auVar45[0xe] = -(pcVar2[0xe] == cVar61);
            auVar45[0xf] = -(pcVar2[0xf] == cVar62);
            if ((((((((((((((((SUB161(auVar45 >> 7,0) & 1) != 0 ||
                             (SUB161(auVar45 >> 0xf,0) & 1) != 0) ||
                            (SUB161(auVar45 >> 0x17,0) & 1) != 0) ||
                           (SUB161(auVar45 >> 0x1f,0) & 1) != 0) ||
                          (SUB161(auVar45 >> 0x27,0) & 1) != 0) ||
                         (SUB161(auVar45 >> 0x2f,0) & 1) != 0) ||
                        (SUB161(auVar45 >> 0x37,0) & 1) != 0) ||
                       (SUB161(auVar45 >> 0x3f,0) & 1) != 0) || (SUB161(auVar45 >> 0x47,0) & 1) != 0
                      ) || (SUB161(auVar45 >> 0x4f,0) & 1) != 0) ||
                    (SUB161(auVar45 >> 0x57,0) & 1) != 0) || (SUB161(auVar45 >> 0x5f,0) & 1) != 0)
                  || (SUB161(auVar45 >> 0x67,0) & 1) != 0) || (SUB161(auVar45 >> 0x6f,0) & 1) != 0)
                || (SUB161(auVar45 >> 0x77,0) & 1) != 0) || auVar45[0xf] < '\0') break;
            uVar26 = uVar26 + lVar29 + 0x10;
            lVar29 = lVar29 + 0x10;
          }
        }
        local_a0 = 0;
        local_98 = 0;
joined_r0x000140af79e5:
        if (puVar25 == (undefined8 *)0x0) {
          uVar63 = 0;
          uVar64 = 0;
        }
        else {
          uVar63 = (undefined4)*puVar25;
          uVar64 = (undefined4)((ulonglong)*puVar25 >> 0x20);
        }
        uStack_105 = (undefined4)local_a0;
        uStack_101 = (undefined4)((ulonglong)local_a0 >> 0x20);
        *puVar13 = uVar6;
        lVar24 = *(longlong *)(lVar12 + 0x30);
        uVar34 = *(undefined8 *)(lVar12 + 0x40);
        if (lVar24 == *(longlong *)(lVar12 + 0x20)) {
          alloc::raw_vec::RawVec::grow_one<enum2_<iced_core::event::Event>,alloc::alloc::Global>();
        }
        cVar35 = cVar35 + '\x11';
      }
      local_c0 = local_c0 + 1;
      lVar29 = *(longlong *)(lVar12 + 0x28);
      lVar23 = lVar24 * 0x38;
      *(undefined8 *)(lVar29 + lVar23) = uVar34;
      *(char *)(lVar29 + 8 + lVar23) = cVar35;
      auVar21._1_7_ = uStack_117;
      auVar21[0] = local_118;
      auVar21._8_7_ = uStack_110;
      auVar21[0xf] = (undefined1)uStack_109;
      *(undefined1 (*) [16])(lVar29 + 9 + lVar23) = auVar21;
      puVar3 = (undefined4 *)(lVar29 + 0x18 + lVar23);
      *puVar3 = uStack_109;
      puVar3[1] = uStack_105;
      puVar3[2] = uStack_101;
      puVar3[3] = local_98;
      *(ulonglong *)(lVar29 + 0x28 + lVar23) = CONCAT44(uVar63,uVar64);
      *(undefined1 *)(lVar29 + 0x30 + lVar23) = 1;
      *(longlong *)(lVar12 + 0x30) = lVar24 + 1;
      *(longlong *)(lVar12 + 0x40) = *(longlong *)(lVar12 + 0x40) + 1;
    }
  } while( true );
  if (1 < local_c0) {
    uVar26 = local_c0 >> 1;
    if (local_c0 < 8) {
      uVar33 = 0;
    }
    else {
      uVar33 = uVar26 & 0xfffffffffffffffc;
      pauVar27 = (undefined1 (*) [16])(local_c0 * 8 + -8);
      uVar30 = 0;
      do {
        auVar39 = *(undefined1 (*) [16])(uVar30 * 8 + 8);
        auVar48 = *(undefined1 (*) [16])(uVar30 * 8 + 0x18);
        auVar40 = pauVar27[-1];
        auVar49 = *pauVar27;
        auVar50._0_8_ = auVar49._8_8_;
        auVar50._8_4_ = auVar49._0_4_;
        auVar50._12_4_ = auVar49._4_4_;
        auVar49._0_8_ = auVar40._8_8_;
        auVar49._8_4_ = auVar40._0_4_;
        auVar49._12_4_ = auVar40._4_4_;
        *(undefined1 (*) [16])(uVar30 * 8 + 8) = auVar50;
        *(undefined1 (*) [16])(uVar30 * 8 + 0x18) = auVar49;
        auVar40._0_8_ = auVar39._8_8_;
        auVar40._8_4_ = auVar39._0_4_;
        auVar40._12_4_ = auVar39._4_4_;
        *pauVar27 = auVar40;
        auVar39._0_8_ = auVar48._8_8_;
        auVar39._8_4_ = auVar48._0_4_;
        auVar39._12_4_ = auVar48._4_4_;
        pauVar27[-1] = auVar39;
        uVar30 = uVar30 + 4;
        pauVar27 = pauVar27 + -2;
      } while (uVar33 != uVar30);
      if (uVar26 == uVar33) goto joined_r0x000140af7cd1;
    }
    puVar25 = (undefined8 *)(uVar33 * 8);
    lVar32 = -uVar33;
    do {
      puVar25 = puVar25 + 1;
      uVar34 = *puVar25;
      *puVar25 = *(undefined8 *)(local_c0 * 8 + lVar32 * 8);
      *(undefined8 *)(local_c0 * 8 + lVar32 * 8) = uVar34;
      lVar32 = lVar32 + -1;
    } while (-lVar32 != uVar26);
  }
joined_r0x000140af7cd1:
  if (local_c0 != 0) {
    local_b8 = (ulonglong *)&DAT_00000008;
    do {
      uVar26 = *local_b8;
      uVar33 = *(ulonglong *)(lVar1 + 0x10);
      if (uVar33 <= uVar26) {
                    // WARNING: Subroutine does not return
        alloc::vec::impl$1::remove::assert_failed();
      }
      lVar32 = *(longlong *)(lVar1 + 8);
      lVar28 = uVar26 * 0x30;
      uVar30 = *(ulonglong *)(lVar32 + lVar28);
      plVar11 = (longlong *)((ulonglong *)(lVar32 + lVar28))[1];
      plVar20 = *(longlong **)(lVar32 + 0x10 + lVar28);
      memmove((void *)(lVar32 + lVar28),(void *)(lVar32 + lVar28 + 0x30),(~uVar26 + uVar33) * 0x30);
      *(ulonglong *)(lVar1 + 0x10) = uVar33 - 1;
      switch(uVar30 & 0xff) {
      case 0:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 1:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 2:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::path_edit::RasterEdit,alloc::alloc::Global>();
        }
        break;
      case 3:
        LOCK();
        *plVar11 = *plVar11 + -1;
        UNLOCK();
        if (*plVar11 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::terrain_edit::TerrainEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 4:
        core::ptr::drop_in_place<country_core::systems::history::replay_wall::WallReplayCmd>();
        break;
      case 5:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::roof_edit::RoofEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 6:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 7:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 8:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<enum2_<country_core::resources::history::tree_edit::TreeEdit>,alloc::alloc::Global>
                    ();
        }
        break;
      case 9:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex,tiny_glade::crash_pickup::crash_payload::CrashMiscInfo>,alloc::alloc::Global>
                    ();
        }
        break;
      default:
        LOCK();
        *plVar20 = *plVar20 + -1;
        UNLOCK();
        if (*plVar20 == 0) {
          alloc::sync::Arc::
          drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                    ();
        }
      }
      local_b8 = local_b8 + 1;
    } while (local_b8 != (ulonglong *)(local_c0 * 8 + 8));
  }
  return;
}

