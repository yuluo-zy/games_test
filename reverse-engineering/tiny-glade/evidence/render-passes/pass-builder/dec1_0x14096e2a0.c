// address: 0x14096e2a0
// pdb name: bind_external_storage_buffer
// signature: void __cdecl bind_external_storage_buffer(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_storage_buffer
          (void)

{
  char *pcVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int iVar10;
  ulonglong extraout_RAX;
  void *in_RCX;
  void *in_RDX;
  ushort uVar11;
  ulonglong uVar12;
  longlong lVar13;
  ulonglong in_R8;
  undefined8 in_R9;
  longlong lVar14;
  uint uVar15;
  ulonglong uVar16;
  byte bVar17;
  char cVar18;
  char cVar20;
  char cVar21;
  char cVar22;
  undefined1 auVar19 [16];
  undefined1 in_XMM1_Ba;
  undefined1 in_XMM1_Bb;
  undefined1 in_XMM1_Bc;
  undefined1 in_XMM1_Bd;
  undefined1 in_XMM1_Be;
  undefined1 in_XMM1_Bf;
  undefined1 in_XMM1_Bg;
  undefined1 in_XMM1_Bh;
  undefined1 in_XMM1_Bi;
  undefined1 in_XMM1_Bj;
  undefined1 in_XMM1_Bk;
  undefined1 in_XMM1_Bl;
  undefined1 in_XMM1_Bm;
  undefined1 in_XMM1_Bn;
  undefined1 in_XMM1_Bo;
  undefined1 in_XMM1_Bp;
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined8 local_b0;
  undefined8 uStack_a8;
  undefined8 local_a0;
  undefined8 local_98;
  undefined4 uStack_8c;
  uint7 uStack_57;
  
  lVar13 = *(longlong *)((longlong)in_RDX + 0x138);
  if (*(longlong *)(lVar13 + 0xb0) != 0) {
    lVar4 = *(longlong *)(lVar13 + 0x98);
    bVar17 = (byte)(in_R8 >> 0x39);
    auVar19 = ZEXT216(CONCAT11(bVar17,bVar17));
    auVar19 = pshuflw(auVar19,auVar19,0);
    lVar14 = 0;
    iVar7 = -(uint)(CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))) ==
                   CONCAT13(in_XMM1_Bd,CONCAT12(in_XMM1_Bc,CONCAT11(in_XMM1_Bb,in_XMM1_Ba))));
    iVar8 = -(uint)(CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))) ==
                   CONCAT13(in_XMM1_Bh,CONCAT12(in_XMM1_Bg,CONCAT11(in_XMM1_Bf,in_XMM1_Be))));
    iVar9 = -(uint)(CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))) ==
                   CONCAT13(in_XMM1_Bl,CONCAT12(in_XMM1_Bk,CONCAT11(in_XMM1_Bj,in_XMM1_Bi))));
    iVar10 = -(uint)(CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))) ==
                    CONCAT13(in_XMM1_Bp,CONCAT12(in_XMM1_Bo,CONCAT11(in_XMM1_Bn,in_XMM1_Bm))));
    uVar16 = in_R8;
    while( true ) {
      uVar16 = *(ulonglong *)(lVar13 + 0xa0) & uVar16;
      pcVar1 = (char *)(lVar4 + uVar16);
      cVar18 = auVar19[0];
      auVar24[0] = -(*pcVar1 == cVar18);
      cVar20 = auVar19[1];
      auVar24[1] = -(pcVar1[1] == cVar20);
      cVar21 = auVar19[2];
      auVar24[2] = -(pcVar1[2] == cVar21);
      cVar22 = auVar19[3];
      auVar24[3] = -(pcVar1[3] == cVar22);
      auVar24[4] = -(pcVar1[4] == cVar18);
      auVar24[5] = -(pcVar1[5] == cVar20);
      auVar24[6] = -(pcVar1[6] == cVar21);
      auVar24[7] = -(pcVar1[7] == cVar22);
      auVar24[8] = -(pcVar1[8] == cVar18);
      auVar24[9] = -(pcVar1[9] == cVar20);
      auVar24[10] = -(pcVar1[10] == cVar21);
      auVar24[0xb] = -(pcVar1[0xb] == cVar22);
      auVar24[0xc] = -(pcVar1[0xc] == cVar18);
      auVar24[0xd] = -(pcVar1[0xd] == cVar20);
      auVar24[0xe] = -(pcVar1[0xe] == cVar21);
      auVar24[0xf] = -(pcVar1[0xf] == cVar22);
      uVar11 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar24 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar24 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar24 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar24 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar24 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar24 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar24 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar24 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar24 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar24 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar24 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar24 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar24 >> 0x77,0) & 1) << 0xe | (ushort)(auVar24[0xf] >> 7) << 0xf;
      uVar15 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar15; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar12 = uVar5 + uVar16 & *(ulonglong *)(lVar13 + 0xa0);
        if (*(ulonglong *)(lVar4 + -0x10 + uVar12 * -0x10) == in_R8) {
          lVar13 = uVar12 * -0x10;
          uVar3 = *(undefined4 *)(lVar4 + -4 + lVar13);
          if (*(int *)(lVar4 + -8 + lVar13) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          rhapsody::raw::device::GraphicsDevice::get_vk_buffer();
          if ((extraout_RAX & 1) == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          rhapsody::raw::managed::handle::impl$5::clone();
          local_98 = CONCAT44(local_98._4_4_,uVar3);
          lVar13 = *(longlong *)((longlong)in_RDX + 0x40);
          if (lVar13 == *(longlong *)((longlong)in_RDX + 0x30)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                      ();
          }
          lVar4 = *(longlong *)((longlong)in_RDX + 0x38);
          *(ulonglong *)(lVar4 + 0x40 + lVar13 * 0x48) = (ulonglong)uStack_57 << 8;
          puVar2 = (undefined8 *)(lVar4 + 0x30 + lVar13 * 0x48);
          *puVar2 = uStack_a8;
          puVar2[1] = local_a0;
          puVar2 = (undefined8 *)(lVar4 + 0x20 + lVar13 * 0x48);
          *puVar2 = 0xffffffffffffffff;
          puVar2[1] = local_b0;
          puVar2 = (undefined8 *)(lVar4 + 0x10 + lVar13 * 0x48);
          *puVar2 = in_R9;
          puVar2[1] = 0;
          puVar2 = (undefined8 *)(lVar4 + lVar13 * 0x48);
          *puVar2 = local_98;
          puVar2[1] = CONCAT44(uStack_8c,5);
          *(longlong *)((longlong)in_RDX + 0x40) = lVar13 + 1;
          goto LAB_14096e45b;
        }
        uVar11 = (ushort)(uVar15 - 1) & (ushort)uVar15;
        uVar15 = CONCAT22((short)(uVar15 - 1 >> 0x10),uVar11);
      }
      auVar23[0] = -(*pcVar1 == (char)iVar7);
      auVar23[1] = -(pcVar1[1] == (char)((uint)iVar7 >> 8));
      auVar23[2] = -(pcVar1[2] == (char)((uint)iVar7 >> 0x10));
      auVar23[3] = -(pcVar1[3] == (char)((uint)iVar7 >> 0x18));
      auVar23[4] = -(pcVar1[4] == (char)iVar8);
      auVar23[5] = -(pcVar1[5] == (char)((uint)iVar8 >> 8));
      auVar23[6] = -(pcVar1[6] == (char)((uint)iVar8 >> 0x10));
      auVar23[7] = -(pcVar1[7] == (char)((uint)iVar8 >> 0x18));
      auVar23[8] = -(pcVar1[8] == (char)iVar9);
      auVar23[9] = -(pcVar1[9] == (char)((uint)iVar9 >> 8));
      auVar23[10] = -(pcVar1[10] == (char)((uint)iVar9 >> 0x10));
      auVar23[0xb] = -(pcVar1[0xb] == (char)((uint)iVar9 >> 0x18));
      auVar23[0xc] = -(pcVar1[0xc] == (char)iVar10);
      auVar23[0xd] = -(pcVar1[0xd] == (char)((uint)iVar10 >> 8));
      auVar23[0xe] = -(pcVar1[0xe] == (char)((uint)iVar10 >> 0x10));
      auVar23[0xf] = -(pcVar1[0xf] == (char)((uint)iVar10 >> 0x18));
      if ((((((((((((((((SUB161(auVar23 >> 7,0) & 1) != 0 || (SUB161(auVar23 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar23 >> 0x17,0) & 1) != 0) || (SUB161(auVar23 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar23 >> 0x27,0) & 1) != 0) || (SUB161(auVar23 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar23 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar23 >> 0x3f,0) & 1) != 0) || (SUB161(auVar23 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar23 >> 0x4f,0) & 1) != 0) || (SUB161(auVar23 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar23 >> 0x5f,0) & 1) != 0) || (SUB161(auVar23 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar23 >> 0x6f,0) & 1) != 0) || (SUB161(auVar23 >> 0x77,0) & 1) != 0) ||
          auVar23[0xf] < '\0') break;
      uVar16 = uVar16 + lVar14 + 0x10;
      lVar14 = lVar14 + 0x10;
    }
  }
LAB_14096e45b:
  memcpy(in_RCX,in_RDX,400);
  return;
}


