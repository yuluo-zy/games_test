
void __cdecl
country_core::systems::collision::world::impl$14::iter_shape_overlaps::
closure_0<country_core::systems::collision::world::SubworldColliderHandle,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_2,system_decorator::decorator_visual::stairs_assemble::calculate_distortion_from_structural_graph::closure_env_3>
          (void)

{
  char *pcVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined8 *puVar7;
  longlong *plVar8;
  uint uVar9;
  uint uVar10;
  char extraout_AL;
  int extraout_EAX;
  int extraout_EAX_00;
  ulonglong *extraout_RAX;
  ulonglong extraout_RAX_00;
  longlong *in_RCX;
  int iVar11;
  ulonglong uVar12;
  longlong lVar13;
  uint uVar14;
  ushort uVar15;
  ulonglong uVar16;
  byte bVar17;
  char cVar18;
  char cVar20;
  char cVar21;
  char cVar22;
  undefined1 auVar19 [16];
  int iVar23;
  int iVar24;
  undefined1 auVar25 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  char local_38;
  
  lVar5 = *in_RCX;
  rapier3d::geometry::collider_set::ColliderSet::get_mut_internal();
  if (extraout_RAX == (ulonglong *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  uVar12 = *extraout_RAX;
  resources::walls::decorator::DecoratorRank::new();
  lVar13 = *(longlong *)(lVar5 + 8);
  uVar16 = *(ulonglong *)(lVar5 + 0x10);
  iVar11 = (int)uVar12;
  impl$62::data();
  uVar12 = uVar12 & 0xffffffff;
  if (((uVar16 <= uVar12) || (lVar13 == 0)) ||
     (*(int *)(lVar13 + 0x20 + uVar12 * 0x28) != extraout_EAX_00)) {
                    // WARNING: Subroutine does not return
    std::panicking::begin_panic<ref_<str_>_>();
  }
  pcVar1 = (char *)(lVar13 + uVar12 * 0x28);
  puVar6 = (undefined8 *)in_RCX[2];
  SubworldColliderHandle::new();
  if (local_38 == '\x10') {
    if (*pcVar1 != '\x10') {
                    // WARNING: Subroutine does not return
      core::panicking::panic();
    }
    puVar7 = (undefined8 *)*puVar6;
    uVar3 = *puVar7;
    uVar4 = *(undefined8 *)(pcVar1 + 8);
    iVar23 = -(uint)((int)uVar4 == (int)uVar3);
    iVar24 = -(uint)((int)((ulonglong)uVar4 >> 0x20) == (int)((ulonglong)uVar3 >> 0x20));
    auVar19._4_4_ = iVar23;
    auVar19._0_4_ = iVar23;
    auVar19._8_4_ = iVar24;
    auVar19._12_4_ = iVar24;
    iVar23 = movmskpd((int)puVar7,auVar19);
    if (iVar23 == 3) {
      return;
    }
  }
  world::impl$3::partial_cmp();
  if ((extraout_AL != -1) && (plVar8 = (longlong *)puVar6[2], plVar8[3] != 0)) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::collision::world::SubworldColliderHandle>_>
              ();
    lVar5 = *plVar8;
    bVar17 = (byte)(extraout_RAX_00 >> 0x39);
    auVar19 = ZEXT216(CONCAT11(bVar17,bVar17));
    auVar19 = pshuflw(auVar19,auVar19,0);
    lVar13 = 0;
    uVar12 = extraout_RAX_00;
    while( true ) {
      uVar12 = plVar8[1] & uVar12;
      pcVar1 = (char *)(lVar5 + uVar12);
      cVar18 = auVar19[0];
      auVar26[0] = -(*pcVar1 == cVar18);
      cVar20 = auVar19[1];
      auVar26[1] = -(pcVar1[1] == cVar20);
      cVar21 = auVar19[2];
      auVar26[2] = -(pcVar1[2] == cVar21);
      cVar22 = auVar19[3];
      auVar26[3] = -(pcVar1[3] == cVar22);
      auVar26[4] = -(pcVar1[4] == cVar18);
      auVar26[5] = -(pcVar1[5] == cVar20);
      auVar26[6] = -(pcVar1[6] == cVar21);
      auVar26[7] = -(pcVar1[7] == cVar22);
      auVar26[8] = -(pcVar1[8] == cVar18);
      auVar26[9] = -(pcVar1[9] == cVar20);
      auVar26[10] = -(pcVar1[10] == cVar21);
      auVar26[0xb] = -(pcVar1[0xb] == cVar22);
      auVar26[0xc] = -(pcVar1[0xc] == cVar18);
      auVar26[0xd] = -(pcVar1[0xd] == cVar20);
      auVar26[0xe] = -(pcVar1[0xe] == cVar21);
      auVar26[0xf] = -(pcVar1[0xf] == cVar22);
      uVar15 = (ushort)(SUB161(auVar26 >> 7,0) & 1) | (ushort)(SUB161(auVar26 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar26 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar26 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar26 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar26 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar26 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar26 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar26 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar26 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar26 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar26 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar26 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar26 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar26 >> 0x77,0) & 1) << 0xe | (ushort)(auVar26[0xf] >> 7) << 0xf;
      uVar14 = (uint)uVar15;
      while (uVar15 != 0) {
        uVar9 = 0;
        for (uVar10 = uVar14; (uVar10 & 1) == 0; uVar10 = uVar10 >> 1 | 0x80000000) {
          uVar9 = uVar9 + 1;
        }
        uVar16 = uVar9 + uVar12 & plVar8[1];
        if ((local_38 == *(char *)(lVar5 + -4 + uVar16 * -0xc)) &&
           (lVar2 = lVar5 + uVar16 * -0xc, uVar3 = *(undefined8 *)(lVar2 + -0xc),
           auVar27._0_4_ = -(uint)((int)uVar3 == extraout_EAX),
           iVar23 = -(uint)((int)((ulonglong)uVar3 >> 0x20) == iVar11),
           auVar27._4_4_ = auVar27._0_4_, auVar27._8_4_ = iVar23, auVar27._12_4_ = iVar23,
           iVar23 = movmskpd((int)lVar2,auVar27), iVar23 == 3)) {
          return;
        }
        uVar15 = (ushort)(uVar14 - 1) & (ushort)uVar14;
        uVar14 = CONCAT22((short)(uVar14 - 1 >> 0x10),uVar15);
      }
      auVar25[0] = -(*pcVar1 == -1);
      auVar25[1] = -(pcVar1[1] == -1);
      auVar25[2] = -(pcVar1[2] == -1);
      auVar25[3] = -(pcVar1[3] == -1);
      auVar25[4] = -(pcVar1[4] == -1);
      auVar25[5] = -(pcVar1[5] == -1);
      auVar25[6] = -(pcVar1[6] == -1);
      auVar25[7] = -(pcVar1[7] == -1);
      auVar25[8] = -(pcVar1[8] == -1);
      auVar25[9] = -(pcVar1[9] == -1);
      auVar25[10] = -(pcVar1[10] == -1);
      auVar25[0xb] = -(pcVar1[0xb] == -1);
      auVar25[0xc] = -(pcVar1[0xc] == -1);
      auVar25[0xd] = -(pcVar1[0xd] == -1);
      auVar25[0xe] = -(pcVar1[0xe] == -1);
      auVar25[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar25 >> 7,0) & 1) != 0 || (SUB161(auVar25 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar25 >> 0x17,0) & 1) != 0) || (SUB161(auVar25 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar25 >> 0x27,0) & 1) != 0) || (SUB161(auVar25 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar25 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar25 >> 0x3f,0) & 1) != 0) || (SUB161(auVar25 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar25 >> 0x4f,0) & 1) != 0) || (SUB161(auVar25 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar25 >> 0x5f,0) & 1) != 0) || (SUB161(auVar25 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar25 >> 0x6f,0) & 1) != 0) || (SUB161(auVar25 >> 0x77,0) & 1) != 0) ||
          auVar25[0xf] < '\0') break;
      uVar12 = uVar12 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
  return;
}

