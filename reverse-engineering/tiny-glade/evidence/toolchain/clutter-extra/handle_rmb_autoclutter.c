
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void __cdecl country_core::systems::ui_clutter::rmb_autoclutter::handle_rmb_autoclutter(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  uint uVar7;
  uint uVar8;
  int iVar9;
  int iVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined1 extraout_AL;
  ulonglong extraout_RAX;
  undefined8 *extraout_RAX_00;
  float *in_RCX;
  longlong in_RDX;
  longlong lVar13;
  ushort uVar14;
  longlong in_R8;
  longlong *in_R9;
  ulonglong uVar15;
  longlong lVar16;
  uint uVar17;
  ulonglong uVar18;
  float fVar19;
  float fVar20;
  float extraout_XMM0_Da;
  float fVar21;
  float fVar22;
  byte bVar23;
  char cVar24;
  char cVar29;
  float fVar25;
  char cVar27;
  char cVar28;
  undefined1 auVar26 [16];
  undefined1 in_XMM2_Bi;
  undefined1 in_XMM2_Bj;
  undefined1 in_XMM2_Bk;
  undefined1 in_XMM2_Bl;
  undefined1 in_XMM2_Bm;
  undefined1 in_XMM2_Bn;
  undefined1 in_XMM2_Bo;
  undefined1 in_XMM2_Bp;
  float fVar30;
  undefined1 auVar31 [16];
  undefined1 auVar32 [16];
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined1 uStack_80;
  undefined7 uStack_7f;
  undefined1 uStack_78;
  undefined8 uStack_77;
  
  fVar19 = *in_RCX;
  fVar20 = in_RCX[1];
  fVar22 = in_RCX[2];
  fVar21 = in_RCX[3];
  if (*(int *)(in_RDX + 0x18) != 0) {
LAB_140976f99:
    fVar30 = fVar22 * fVar22 + fVar20 * fVar20 + fVar19 * fVar19;
    fVar25 = _UNK_1429a9088 * fVar22 +
             _UNK_1429a9084 * fVar20 + ___xmm_00000000bf8000008000000080000000 * fVar19;
    fVar25 = fVar25 + fVar25;
    fVar19 = atan2f((float)((uint)((fVar21 + fVar21) *
                                   (fVar20 * ___xmm_0000000080000000bf800000bf800000 -
                                   fVar22 * ___xmm_00000000800000008000000080000000) +
                                  fVar25 * fVar19 +
                                  (fVar21 * fVar21 - fVar30) *
                                  ___xmm_00000000bf8000000000000080000000) ^
                           ___xmm_80000000800000008000000080000000),
                    (float)((uint)((fVar21 + fVar21) *
                                   (fVar19 * _UNK_142a923f8 - fVar20 * _UNK_1429cd108) +
                                  fVar25 * fVar22 + (fVar21 * fVar21 - fVar30) * _UNK_142a923e8) ^
                           ___xmm_80000000800000008000000080000000));
    uVar4 = *in_stack_00000030;
    hashbrown::map::HashMap::
    insert<country_core::resources::walls::WallId,tuple_<>,std::hash::random::RandomState,alloc::alloc::Global>
              ();
    resources::clutter_meshes::ClutterMeshTy::from();
    uVar11 = *(undefined8 *)(in_RDX + 0x18);
    uVar12 = *(undefined8 *)(in_RDX + 0x20);
    uVar5 = *(undefined8 *)(in_RDX + 0x28);
    *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
    lVar13 = *in_stack_00000038;
    lVar3 = *(longlong *)(lVar13 + 0x30);
    uVar6 = *(undefined8 *)(lVar13 + 0x40);
    if (lVar3 == *(longlong *)(lVar13 + 0x20)) {
      alloc::raw_vec::RawVec::
      grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                ();
    }
    lVar16 = *(longlong *)(lVar13 + 0x28);
    puVar2 = (undefined8 *)(lVar16 + 0x19 + lVar3 * 0x48);
    *puVar2 = CONCAT17(uStack_78,uStack_7f);
    puVar2[1] = uStack_77;
    puVar2 = (undefined8 *)(lVar16 + 0x10 + lVar3 * 0x48);
    *puVar2 = uVar5;
    puVar2[1] = CONCAT71(uStack_7f,uStack_80);
    puVar2 = (undefined8 *)(lVar16 + lVar3 * 0x48);
    *puVar2 = uVar11;
    puVar2[1] = uVar12;
    *(undefined1 *)(lVar16 + 0x29 + lVar3 * 0x48) = 3;
    *(float *)(lVar16 + 0x2c + lVar3 * 0x48) = fVar19;
    *(undefined8 *)(lVar16 + 0x30 + lVar3 * 0x48) = *(undefined8 *)(in_RCX + 4);
    *(float *)(lVar16 + 0x38 + lVar3 * 0x48) = in_RCX[6];
    *(undefined1 *)(lVar16 + 0x3c + lVar3 * 0x48) = extraout_AL;
    *(undefined8 *)(lVar16 + 0x40 + lVar3 * 0x48) = uVar6;
    *(longlong *)(lVar13 + 0x30) = lVar3 + 1;
    *(longlong *)(lVar13 + 0x40) = *(longlong *)(lVar13 + 0x40) + 1;
    resources::history::History::start_input_recording();
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX_00 != (undefined8 *)0x0) {
      *extraout_RAX_00 = 1;
      extraout_RAX_00[1] = 1;
      extraout_RAX_00[2] = uVar4;
      resources::history::History::add();
      return;
    }
                    /* WARNING: Subroutine does not return */
    alloc::alloc::handle_alloc_error();
  }
  lVar13 = *(longlong *)(in_RDX + 0x28);
  if (*(longlong *)(in_R8 + 0x48) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::ui::GlobalId>_>();
    lVar3 = *(longlong *)(in_R8 + 0x30);
    bVar23 = (byte)(extraout_RAX >> 0x39);
    auVar26 = ZEXT216(CONCAT11(bVar23,bVar23));
    auVar26 = pshuflw(auVar26,auVar26,0);
    lVar16 = 0;
    iVar9 = -(uint)(CONCAT13(in_XMM2_Bl,CONCAT12(in_XMM2_Bk,CONCAT11(in_XMM2_Bj,in_XMM2_Bi))) ==
                   CONCAT13(in_XMM2_Bl,CONCAT12(in_XMM2_Bk,CONCAT11(in_XMM2_Bj,in_XMM2_Bi))));
    iVar10 = -(uint)(CONCAT13(in_XMM2_Bp,CONCAT12(in_XMM2_Bo,CONCAT11(in_XMM2_Bn,in_XMM2_Bm))) ==
                    CONCAT13(in_XMM2_Bp,CONCAT12(in_XMM2_Bo,CONCAT11(in_XMM2_Bn,in_XMM2_Bm))));
    uVar15 = extraout_RAX;
    do {
      uVar15 = *(ulonglong *)(in_R8 + 0x38) & uVar15;
      pcVar1 = (char *)(lVar3 + uVar15);
      cVar24 = auVar26[0];
      auVar32[0] = -(*pcVar1 == cVar24);
      cVar27 = auVar26[1];
      auVar32[1] = -(pcVar1[1] == cVar27);
      cVar28 = auVar26[2];
      auVar32[2] = -(pcVar1[2] == cVar28);
      cVar29 = auVar26[3];
      auVar32[3] = -(pcVar1[3] == cVar29);
      auVar32[4] = -(pcVar1[4] == cVar24);
      auVar32[5] = -(pcVar1[5] == cVar27);
      auVar32[6] = -(pcVar1[6] == cVar28);
      auVar32[7] = -(pcVar1[7] == cVar29);
      auVar32[8] = -(pcVar1[8] == cVar24);
      auVar32[9] = -(pcVar1[9] == cVar27);
      auVar32[10] = -(pcVar1[10] == cVar28);
      auVar32[0xb] = -(pcVar1[0xb] == cVar29);
      auVar32[0xc] = -(pcVar1[0xc] == cVar24);
      auVar32[0xd] = -(pcVar1[0xd] == cVar27);
      auVar32[0xe] = -(pcVar1[0xe] == cVar28);
      auVar32[0xf] = -(pcVar1[0xf] == cVar29);
      uVar14 = (ushort)(SUB161(auVar32 >> 7,0) & 1) | (ushort)(SUB161(auVar32 >> 0xf,0) & 1) << 1 |
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
               (ushort)(SUB161(auVar32 >> 0x77,0) & 1) << 0xe | (ushort)(auVar32[0xf] >> 7) << 0xf;
      uVar17 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar7 = 0;
        for (uVar8 = uVar17; (uVar8 & 1) == 0; uVar8 = uVar8 >> 1 | 0x80000000) {
          uVar7 = uVar7 + 1;
        }
        uVar18 = uVar7 + uVar15 & *(ulonglong *)(in_R8 + 0x38);
        if (*(longlong *)(lVar3 + -0x10 + uVar18 * -0x10) == lVar13) {
          uVar15 = *(ulonglong *)(lVar3 + -8 + uVar18 * -0x10);
          if (uVar15 >> 0x20 == 0) goto LAB_140976f23;
          lVar13 = in_R9[1];
          uVar18 = uVar15 & 0xffffffff;
          if ((*(ulonglong *)(lVar13 + 0x10) <= uVar18) ||
             (*(int *)(*(longlong *)(lVar13 + 8) + uVar18 * 0x14) != (int)(uVar15 >> 0x20))) {
LAB_1409771b7:
                    /* WARNING: Subroutine does not return */
            core::result::unwrap_failed();
          }
          lVar3 = *(longlong *)(lVar13 + 8) + uVar18 * 0x14;
          uVar15 = (ulonglong)*(uint *)(lVar3 + 4);
          if ((uVar15 == 0xffffffff) ||
             ((lVar16 = *in_R9, *(ulonglong *)(lVar16 + 0x38) <= uVar15 ||
              ((*(ulonglong *)
                 (*(longlong *)(lVar16 + 0x28) + (ulonglong)(*(uint *)(lVar3 + 4) >> 6) * 8) >>
                (uVar15 & 0x3f) & 1) == 0)))) goto LAB_1409771b7;
          if (*(ulonglong *)(lVar16 + 0x110) <
              *(ulonglong *)
               (*(longlong *)(lVar13 + 0x1a8) + 0x40 + (ulonglong)*(uint *)(lVar3 + 0xc) * 0x48)) {
            lVar13 = *(longlong *)(lVar13 + 0x1a8) + (ulonglong)*(uint *)(lVar3 + 0xc) * 0x48;
            uVar15 = *(ulonglong *)
                      (*(longlong *)(lVar13 + 0x38) + *(ulonglong *)(lVar16 + 0x110) * 8);
            if (uVar15 == 0) goto LAB_140976e5d;
            lVar13 = *(longlong *)(*(longlong *)(lVar13 + 0x18) + 0x10 + ~uVar15 * 0x30);
          }
          else {
LAB_140976e5d:
            lVar13 = 0;
          }
          lVar16 = (ulonglong)*(uint *)(lVar3 + 0x10) * 0x58;
          lVar3 = lVar13 + lVar16;
          fVar30 = fVar22 * fVar22 + fVar20 * fVar20 + fVar19 * fVar19;
          fVar25 = _UNK_1429a9088 * fVar22 +
                   _UNK_1429a9084 * fVar20 + ___xmm_00000000bf8000008000000080000000 * fVar19;
          fVar25 = fVar25 + fVar25;
          fVar19 = atan2f((float)((uint)((fVar21 + fVar21) *
                                         (fVar20 * ___xmm_0000000080000000bf800000bf800000 -
                                         fVar22 * ___xmm_00000000800000008000000080000000) +
                                        fVar25 * fVar19 +
                                        (fVar21 * fVar21 - fVar30) *
                                        ___xmm_00000000bf8000000000000080000000) ^
                                 ___xmm_80000000800000008000000080000000),
                          (float)((uint)((fVar21 + fVar21) *
                                         (fVar19 * _UNK_142a923f8 - fVar20 * _UNK_1429cd108) +
                                        fVar25 * fVar22 +
                                        (fVar21 * fVar21 - fVar30) * _UNK_142a923e8) ^
                                 ___xmm_80000000800000008000000080000000));
          if ((*(byte *)(lVar13 + 8 + lVar16) & 1) == 0) {
            ::utils::circular_f32::CircularF32::as_f32();
            fVar20 = __real_bfc90fdb - extraout_XMM0_Da;
          }
          else {
            fVar20 = atan2f((float)(*(uint *)(lVar3 + 0x10) ^
                                   ___xmm_80000000800000008000000080000000),*(float *)(lVar3 + 0xc))
            ;
          }
          fVar19 = fmodf(fVar19 - fVar20,__real_40c90fdb);
          fVar19 = fVar19 * __real_3f000000;
          fVar20 = sinf(fVar19);
          fVar21 = cosf(fVar19);
          fVar19 = 0.0;
          fVar22 = 0.0;
          goto LAB_140976f99;
        }
        uVar14 = (ushort)(uVar17 - 1) & (ushort)uVar17;
        uVar17 = CONCAT22((short)(uVar17 - 1 >> 0x10),uVar14);
      }
      auVar31[0] = -(*pcVar1 == -1);
      auVar31[1] = -(pcVar1[1] == -1);
      auVar31[2] = -(pcVar1[2] == -1);
      auVar31[3] = -(pcVar1[3] == -1);
      auVar31[4] = -(pcVar1[4] == -1);
      auVar31[5] = -(pcVar1[5] == -1);
      auVar31[6] = -(pcVar1[6] == -1);
      auVar31[7] = -(pcVar1[7] == -1);
      auVar31[8] = -(pcVar1[8] == (char)iVar9);
      auVar31[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
      auVar31[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
      auVar31[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
      auVar31[0xc] = -(pcVar1[0xc] == (char)iVar10);
      auVar31[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
      auVar31[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
      auVar31[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar31 >> 7,0) & 1) != 0 || (SUB161(auVar31 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar31 >> 0x17,0) & 1) != 0) || (SUB161(auVar31 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar31 >> 0x27,0) & 1) != 0) || (SUB161(auVar31 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar31 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar31 >> 0x3f,0) & 1) != 0) || (SUB161(auVar31 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar31 >> 0x4f,0) & 1) != 0) || (SUB161(auVar31 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar31 >> 0x5f,0) & 1) != 0) || (SUB161(auVar31 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar31 >> 0x6f,0) & 1) != 0) || (SUB161(auVar31 >> 0x77,0) & 1) != 0) ||
          auVar31[0xf] < '\0') break;
      uVar15 = uVar15 + lVar16 + 0x10;
      lVar16 = lVar16 + 0x10;
    } while( true );
  }
LAB_140976f23:
                    /* WARNING: Subroutine does not return */
  core::option::unwrap_failed();
}

