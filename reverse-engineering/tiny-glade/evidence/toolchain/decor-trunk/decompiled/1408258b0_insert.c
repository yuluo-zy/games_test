// ADDR 0x1408258b0
// SIG void __cdecl insert(void)
// ORIG system::insert


void __cdecl system_decorator::blueprint::DecoratorBlueprints::insert(void)

{
  char *pcVar1;
  undefined1 (*pauVar2) [16];
  longlong *plVar3;
  uint uVar4;
  ulonglong extraout_RAX;
  void *_Dst;
  undefined8 *extraout_RAX_00;
  longlong *in_RCX;
  undefined4 in_EDX;
  undefined8 *puVar5;
  uint uVar6;
  uint uVar7;
  undefined8 in_R8;
  ulonglong uVar8;
  ushort uVar9;
  undefined8 in_R9;
  longlong lVar10;
  longlong lVar11;
  ulonglong uVar12;
  byte bVar13;
  undefined1 (*pauVar14) [16];
  bool bVar15;
  byte bVar16;
  char cVar17;
  char cVar19;
  char cVar20;
  char cVar21;
  undefined1 auVar18 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 in_stack_00000028;
  undefined8 local_1a8;
  undefined8 uStack_1a0;
  undefined8 local_198;
  undefined4 uStack_190;
  undefined4 uStack_18c;
  undefined4 uStack_188;
  undefined8 uStack_184;
  undefined8 local_158;
  undefined4 local_148;
  undefined4 local_118;
  undefined8 local_e8;
  undefined8 local_e0;
  undefined1 local_d8 [16];
  undefined8 local_c8;
  undefined1 local_c0 [16];
  undefined8 local_b0;
  undefined1 local_a8 [16];
  undefined8 local_98;
  undefined1 local_90 [16];
  undefined1 local_80 [16];
  undefined2 local_70;
  void *local_60;
  undefined8 local_58;
  undefined8 *local_50;
  undefined8 local_48;
  
  local_48 = 0xfffffffffffffffe;
  local_50 = (undefined8 *)CONCAT44(local_50._4_4_,in_EDX);
  local_1a8 = CONCAT44(local_1a8._4_4_,in_EDX);
  core::hash::BuildHasher::
  hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>();
  lVar10 = *in_RCX;
  bVar13 = (byte)(extraout_RAX >> 0x38);
  bVar16 = bVar13 >> 1;
  auVar18 = ZEXT216(CONCAT11(bVar16,bVar16));
  auVar18 = pshuflw(auVar18,auVar18,0);
  lVar11 = 0;
  uVar8 = extraout_RAX;
  do {
    uVar8 = in_RCX[1] & uVar8;
    pcVar1 = (char *)(lVar10 + uVar8);
    cVar17 = auVar18[0];
    auVar23[0] = -(*pcVar1 == cVar17);
    cVar19 = auVar18[1];
    auVar23[1] = -(pcVar1[1] == cVar19);
    cVar20 = auVar18[2];
    auVar23[2] = -(pcVar1[2] == cVar20);
    cVar21 = auVar18[3];
    auVar23[3] = -(pcVar1[3] == cVar21);
    auVar23[4] = -(pcVar1[4] == cVar17);
    auVar23[5] = -(pcVar1[5] == cVar19);
    auVar23[6] = -(pcVar1[6] == cVar20);
    auVar23[7] = -(pcVar1[7] == cVar21);
    auVar23[8] = -(pcVar1[8] == cVar17);
    auVar23[9] = -(pcVar1[9] == cVar19);
    auVar23[10] = -(pcVar1[10] == cVar20);
    auVar23[0xb] = -(pcVar1[0xb] == cVar21);
    auVar23[0xc] = -(pcVar1[0xc] == cVar17);
    auVar23[0xd] = -(pcVar1[0xd] == cVar19);
    auVar23[0xe] = -(pcVar1[0xe] == cVar20);
    auVar23[0xf] = -(pcVar1[0xf] == cVar21);
    uVar9 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
            (ushort)(SUB161(auVar23 >> 0x17,0) & 1) << 2 |
            (ushort)(SUB161(auVar23 >> 0x1f,0) & 1) << 3 |
            (ushort)(SUB161(auVar23 >> 0x27,0) & 1) << 4 |
            (ushort)(SUB161(auVar23 >> 0x2f,0) & 1) << 5 |
            (ushort)(SUB161(auVar23 >> 0x37,0) & 1) << 6 |
            (ushort)(SUB161(auVar23 >> 0x3f,0) & 1) << 7 |
            (ushort)(SUB161(auVar23 >> 0x47,0) & 1) << 8 |
            (ushort)(SUB161(auVar23 >> 0x4f,0) & 1) << 9 |
            (ushort)(SUB161(auVar23 >> 0x57,0) & 1) << 10 |
            (ushort)(SUB161(auVar23 >> 0x5f,0) & 1) << 0xb |
            (ushort)(SUB161(auVar23 >> 0x67,0) & 1) << 0xc |
            (ushort)(SUB161(auVar23 >> 0x6f,0) & 1) << 0xd |
            (ushort)(SUB161(auVar23 >> 0x77,0) & 1) << 0xe | (ushort)(auVar23[0xf] >> 7) << 0xf;
    uVar6 = (uint)uVar9;
    while (uVar9 != 0) {
      uVar7 = 0;
      for (uVar4 = uVar6; (uVar4 & 1) == 0; uVar4 = uVar4 >> 1 | 0x80000000) {
        uVar7 = uVar7 + 1;
      }
      uVar12 = uVar7 + uVar8 & in_RCX[1];
      if (*(int *)(lVar10 + -0x10 + uVar12 * -0x10) == (int)local_50) {
        lVar11 = uVar12 * -0x10;
        pauVar14 = (undefined1 (*) [16])(lVar10 + lVar11);
        puVar5 = (undefined8 *)(lVar10 + lVar11 + -8);
        plVar3 = *(longlong **)(lVar10 + -8 + lVar11);
        LOCK();
        bVar15 = plVar3[1] == 1;
        if (bVar15) {
          plVar3[1] = -1;
        }
        UNLOCK();
        local_58 = in_R8;
        if ((bVar15) && (plVar3[1] = 1, *plVar3 == 1)) {
          local_50 = (undefined8 *)*puVar5;
          local_50[10] = 1;
          if (*(int *)(local_50 + 0xc) == 0) {
            plVar3 = (longlong *)local_50[0xe];
            LOCK();
            *plVar3 = *plVar3 + -1;
            UNLOCK();
            if (*plVar3 == 0) {
              alloc::sync::Arc::
              drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                        ();
            }
          }
          *(undefined4 *)(local_50 + 0xc) = 2;
          *(undefined8 *)((longlong)local_50 + 100) = local_1a8;
          *(undefined8 *)((longlong)local_50 + 0x6c) = uStack_1a0;
          *(undefined8 *)((longlong)local_50 + 0x74) = local_198;
          *(ulonglong *)((longlong)local_50 + 0x7c) = CONCAT44(uStack_18c,uStack_190);
          local_50[0x10] = CONCAT44(uStack_188,uStack_18c);
          local_50[0x11] = uStack_184;
          if (*(int *)(local_50 + 0x12) == 0) {
            plVar3 = (longlong *)local_50[0x14];
            LOCK();
            *plVar3 = *plVar3 + -1;
            UNLOCK();
            if (*plVar3 == 0) {
              alloc::sync::Arc::
              drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>
                        ();
            }
          }
          puVar5 = local_50;
          *(undefined4 *)(local_50 + 0x12) = 2;
          *(undefined8 *)((longlong)local_50 + 0x94) = local_1a8;
          *(undefined8 *)((longlong)local_50 + 0x9c) = uStack_1a0;
          *(undefined8 *)((longlong)local_50 + 0xa4) = local_198;
          *(ulonglong *)((longlong)local_50 + 0xac) = CONCAT44(uStack_18c,uStack_190);
          local_50[0x16] = CONCAT44(uStack_188,uStack_18c);
          local_50[0x17] = uStack_184;
          *(undefined1 (*) [16])(local_50 + 2) = (undefined1  [16])0x0;
          if (local_50[0x24] != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          puVar5[0x24] = 0;
          if (puVar5[0x25] != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          puVar5[0x25] = 0;
          if (puVar5[0x26] != 0) {
            std::alloc::__default_lib_allocator::__rust_dealloc();
          }
          puVar5[0x26] = 0;
          puVar5[0x1a] = 0;
          puVar5[0x1d] = 0;
          puVar5[0x20] = 0;
          puVar5[0x23] = 0;
          *(undefined2 *)(puVar5 + 0x27) = 0;
          in_R8 = local_58;
        }
        else {
          local_d8 = (undefined1  [16])0x0;
          local_c0 = (undefined1  [16])0x0;
          local_90 = (undefined1  [16])0x0;
          local_80 = (undefined1  [16])0x0;
          local_70 = 0;
          local_1a8 = 1;
          uStack_1a0 = 1;
          local_198 = 0;
          uStack_190 = 0;
          uStack_18c = 0;
          local_158 = 1;
          local_148 = 2;
          local_118 = 2;
          local_e8 = 0;
          local_e0 = 0x10;
          local_c8 = 4;
          local_b0 = 4;
          local_a8 = (undefined1  [16])0x0;
          local_98 = 0x10;
          std::alloc::__default_lib_allocator::__rust_alloc();
          if (_Dst == (void *)0x0) {
                    // WARNING: Subroutine does not return
            alloc::alloc::handle_alloc_error();
          }
          local_60 = _Dst;
          memcpy(_Dst,&local_1a8,0x140);
          plVar3 = (longlong *)*puVar5;
          LOCK();
          *plVar3 = *plVar3 + -1;
          UNLOCK();
          local_50 = puVar5;
          if (*plVar3 == 0) {
            alloc::sync::Arc::
            drop_slow<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>();
          }
          *local_50 = local_60;
          in_R8 = local_58;
        }
        goto LAB_140825dc1;
      }
      uVar9 = (ushort)(uVar6 - 1) & (ushort)uVar6;
      uVar6 = CONCAT22((short)(uVar6 - 1 >> 0x10),uVar9);
    }
    auVar22[0] = -(*pcVar1 == -1);
    auVar22[1] = -(pcVar1[1] == -1);
    auVar22[2] = -(pcVar1[2] == -1);
    auVar22[3] = -(pcVar1[3] == -1);
    auVar22[4] = -(pcVar1[4] == -1);
    auVar22[5] = -(pcVar1[5] == -1);
    auVar22[6] = -(pcVar1[6] == -1);
    auVar22[7] = -(pcVar1[7] == -1);
    auVar22[8] = -(pcVar1[8] == -1);
    auVar22[9] = -(pcVar1[9] == -1);
    auVar22[10] = -(pcVar1[10] == -1);
    auVar22[0xb] = -(pcVar1[0xb] == -1);
    auVar22[0xc] = -(pcVar1[0xc] == -1);
    auVar22[0xd] = -(pcVar1[0xd] == -1);
    auVar22[0xe] = -(pcVar1[0xe] == -1);
    auVar22[0xf] = -(pcVar1[0xf] == -1);
    if ((((((((((((((((SUB161(auVar22 >> 7,0) & 1) != 0 || (SUB161(auVar22 >> 0xf,0) & 1) != 0) ||
                    (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0)
                  || (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0)
                || (SUB161(auVar22 >> 0x37,0) & 1) != 0) || (SUB161(auVar22 >> 0x3f,0) & 1) != 0) ||
              (SUB161(auVar22 >> 0x47,0) & 1) != 0) || (SUB161(auVar22 >> 0x4f,0) & 1) != 0) ||
            (SUB161(auVar22 >> 0x57,0) & 1) != 0) || (SUB161(auVar22 >> 0x5f,0) & 1) != 0) ||
          (SUB161(auVar22 >> 0x67,0) & 1) != 0) || (SUB161(auVar22 >> 0x6f,0) & 1) != 0) ||
        (SUB161(auVar22 >> 0x77,0) & 1) != 0) || auVar22[0xf] < '\0') {
      if (in_RCX[2] == 0) {
        hashbrown::raw::RawTable::
        reserve_rehash<tuple_<country_core::resources::walls::decorator::DecoratorId,glam::f32::vec3::Vec3>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<country_core::resources::walls::decorator::DecoratorId,glam::f32::vec3::Vec3,std::hash::random::RandomState>_>
                  ();
      }
      std::alloc::__default_lib_allocator::__rust_alloc();
      if (extraout_RAX_00 == (undefined8 *)0x0) {
                    // WARNING: Subroutine does not return
        alloc::alloc::handle_alloc_error();
      }
      *extraout_RAX_00 = 1;
      extraout_RAX_00[1] = 1;
      *(undefined1 (*) [16])(extraout_RAX_00 + 2) = (undefined1  [16])0x0;
      extraout_RAX_00[10] = 1;
      *(undefined4 *)(extraout_RAX_00 + 0xc) = 2;
      *(undefined4 *)(extraout_RAX_00 + 0x12) = 2;
      extraout_RAX_00[0x18] = 0;
      extraout_RAX_00[0x19] = 0x10;
      *(undefined1 (*) [16])(extraout_RAX_00 + 0x1a) = (undefined1  [16])0x0;
      extraout_RAX_00[0x1c] = 4;
      *(undefined1 (*) [16])(extraout_RAX_00 + 0x1d) = (undefined1  [16])0x0;
      extraout_RAX_00[0x1f] = 4;
      *(undefined1 (*) [16])(extraout_RAX_00 + 0x20) = (undefined1  [16])0x0;
      extraout_RAX_00[0x22] = 0x10;
      *(undefined1 (*) [16])(extraout_RAX_00 + 0x23) = (undefined1  [16])0x0;
      *(undefined1 (*) [16])(extraout_RAX_00 + 0x25) = (undefined1  [16])0x0;
      *(undefined2 *)(extraout_RAX_00 + 0x27) = 0;
      pauVar2 = (undefined1 (*) [16])*in_RCX;
      uVar8 = in_RCX[1];
      uVar12 = extraout_RAX & uVar8;
      auVar18 = *(undefined1 (*) [16])(*pauVar2 + uVar12);
      uVar9 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe |
              (ushort)(byte)(auVar18[0xf] >> 7) << 0xf;
      uVar6 = (uint)uVar9;
      if (uVar9 == 0) {
        lVar10 = 0x10;
        do {
          uVar12 = uVar12 + lVar10 & uVar8;
          auVar18 = *(undefined1 (*) [16])(*pauVar2 + uVar12);
          lVar10 = lVar10 + 0x10;
          uVar6 = (uint)(ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
                                 (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
                                 (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
                                 (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
                                 (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
                                 (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
                                 (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
                                 (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
                                 (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
                                 (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
                                 (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
                                 (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
                                 (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
                                 (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
                                 (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe |
                                (ushort)(byte)(auVar18[0xf] >> 7) << 0xf);
        } while (uVar6 == 0);
      }
      uVar7 = 0;
      for (; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar7 = uVar7 + 1;
      }
      uVar12 = uVar7 + uVar12 & uVar8;
      bVar16 = (*pauVar2)[uVar12];
      if (-1 < (char)bVar16) {
        auVar18 = *pauVar2;
        uVar6 = 0;
        for (uVar7 = (uint)(ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
                                    (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
                                    (ushort)(SUB161(auVar18 >> 0x17,0) & 1) << 2 |
                                    (ushort)(SUB161(auVar18 >> 0x1f,0) & 1) << 3 |
                                    (ushort)(SUB161(auVar18 >> 0x27,0) & 1) << 4 |
                                    (ushort)(SUB161(auVar18 >> 0x2f,0) & 1) << 5 |
                                    (ushort)(SUB161(auVar18 >> 0x37,0) & 1) << 6 |
                                    (ushort)(SUB161(auVar18 >> 0x3f,0) & 1) << 7 |
                                    (ushort)(SUB161(auVar18 >> 0x47,0) & 1) << 8 |
                                    (ushort)(SUB161(auVar18 >> 0x4f,0) & 1) << 9 |
                                    (ushort)(SUB161(auVar18 >> 0x57,0) & 1) << 10 |
                                    (ushort)(SUB161(auVar18 >> 0x5f,0) & 1) << 0xb |
                                    (ushort)(SUB161(auVar18 >> 0x67,0) & 1) << 0xc |
                                    (ushort)(SUB161(auVar18 >> 0x6f,0) & 1) << 0xd |
                                    (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe |
                                   (ushort)(byte)(auVar18[0xf] >> 7) << 0xf); (uVar7 & 1) == 0;
            uVar7 = uVar7 >> 1 | 0x80000000) {
          uVar6 = uVar6 + 1;
        }
        uVar12 = (ulonglong)uVar6;
        bVar16 = (*pauVar2)[uVar12];
      }
      bVar13 = bVar13 >> 1;
      (*pauVar2)[uVar12] = bVar13;
      pauVar2[1][uVar12 - 0x10 & uVar8] = bVar13;
      in_RCX[2] = in_RCX[2] - (ulonglong)(bVar16 & 1);
      pauVar14 = pauVar2 + -uVar12;
      *(int *)pauVar2[-1 - uVar12] = (int)local_50;
      *(undefined8 **)(pauVar14[-1] + 8) = extraout_RAX_00;
      in_RCX[3] = in_RCX[3] + 1;
LAB_140825dc1:
      plVar3 = *(longlong **)(pauVar14[-1] + 8);
      LOCK();
      bVar15 = plVar3[1] == 1;
      if (bVar15) {
        plVar3[1] = -1;
      }
      UNLOCK();
      if ((bVar15) && (plVar3[1] = 1, *plVar3 == 1)) {
        lVar10 = *(longlong *)(pauVar14[-1] + 8);
        *(undefined8 *)(lVar10 + 0x50) = in_R8;
        *(undefined8 *)(lVar10 + 0x58) = in_R9;
        *(undefined1 *)(lVar10 + 0x138) = in_stack_00000028;
        return;
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
    uVar8 = uVar8 + lVar11 + 0x10;
    lVar11 = lVar11 + 0x10;
  } while( true );
}


