// address: 0x14096e540
// pdb name: bind_external_raw
// signature: void __cdecl bind_external_raw(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::bind_external_raw(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  void *in_RCX;
  void *in_RDX;
  ushort uVar14;
  ulonglong uVar15;
  longlong lVar16;
  ulonglong in_R8;
  undefined8 *in_R9;
  longlong lVar17;
  uint uVar18;
  ulonglong uVar19;
  byte bVar20;
  char cVar21;
  char cVar23;
  char cVar24;
  char cVar25;
  undefined1 auVar22 [16];
  undefined1 auVar26 [16];
  undefined1 auVar27 [16];
  undefined8 local_a8;
  
  lVar16 = *(longlong *)((longlong)in_RDX + 0x138);
  if (*(longlong *)(lVar16 + 0xb0) != 0) {
    lVar3 = *(longlong *)(lVar16 + 0x98);
    bVar20 = (byte)(in_R8 >> 0x39);
    auVar22 = ZEXT216(CONCAT11(bVar20,bVar20));
    auVar22 = pshuflw(auVar22,auVar22,0);
    lVar17 = 0;
    uVar19 = in_R8;
    while( true ) {
      uVar19 = *(ulonglong *)(lVar16 + 0xa0) & uVar19;
      pcVar1 = (char *)(lVar3 + uVar19);
      cVar21 = auVar22[0];
      auVar27[0] = -(*pcVar1 == cVar21);
      cVar23 = auVar22[1];
      auVar27[1] = -(pcVar1[1] == cVar23);
      cVar24 = auVar22[2];
      auVar27[2] = -(pcVar1[2] == cVar24);
      cVar25 = auVar22[3];
      auVar27[3] = -(pcVar1[3] == cVar25);
      auVar27[4] = -(pcVar1[4] == cVar21);
      auVar27[5] = -(pcVar1[5] == cVar23);
      auVar27[6] = -(pcVar1[6] == cVar24);
      auVar27[7] = -(pcVar1[7] == cVar25);
      auVar27[8] = -(pcVar1[8] == cVar21);
      auVar27[9] = -(pcVar1[9] == cVar23);
      auVar27[10] = -(pcVar1[10] == cVar24);
      auVar27[0xb] = -(pcVar1[0xb] == cVar25);
      auVar27[0xc] = -(pcVar1[0xc] == cVar21);
      auVar27[0xd] = -(pcVar1[0xd] == cVar23);
      auVar27[0xe] = -(pcVar1[0xe] == cVar24);
      auVar27[0xf] = -(pcVar1[0xf] == cVar25);
      uVar14 = (ushort)(SUB161(auVar27 >> 7,0) & 1) | (ushort)(SUB161(auVar27 >> 0xf,0) & 1) << 1 |
               (ushort)(SUB161(auVar27 >> 0x17,0) & 1) << 2 |
               (ushort)(SUB161(auVar27 >> 0x1f,0) & 1) << 3 |
               (ushort)(SUB161(auVar27 >> 0x27,0) & 1) << 4 |
               (ushort)(SUB161(auVar27 >> 0x2f,0) & 1) << 5 |
               (ushort)(SUB161(auVar27 >> 0x37,0) & 1) << 6 |
               (ushort)(SUB161(auVar27 >> 0x3f,0) & 1) << 7 |
               (ushort)(SUB161(auVar27 >> 0x47,0) & 1) << 8 |
               (ushort)(SUB161(auVar27 >> 0x4f,0) & 1) << 9 |
               (ushort)(SUB161(auVar27 >> 0x57,0) & 1) << 10 |
               (ushort)(SUB161(auVar27 >> 0x5f,0) & 1) << 0xb |
               (ushort)(SUB161(auVar27 >> 0x67,0) & 1) << 0xc |
               (ushort)(SUB161(auVar27 >> 0x6f,0) & 1) << 0xd |
               (ushort)(SUB161(auVar27 >> 0x77,0) & 1) << 0xe | (ushort)(auVar27[0xf] >> 7) << 0xf;
      uVar18 = (uint)uVar14;
      while (uVar14 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar18; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar15 = uVar4 + uVar19 & *(ulonglong *)(lVar16 + 0xa0);
        if (*(ulonglong *)(lVar3 + -0x10 + uVar15 * -0x10) == in_R8) {
          lVar16 = uVar15 * -0x10;
          if (*(int *)(lVar3 + -8 + lVar16) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          local_a8 = CONCAT44(local_a8._4_4_,*(undefined4 *)(lVar3 + -4 + lVar16));
          uVar6 = *in_R9;
          uVar7 = in_R9[1];
          uVar8 = in_R9[2];
          uVar9 = in_R9[3];
          uVar10 = in_R9[4];
          uVar11 = in_R9[5];
          uVar12 = in_R9[6];
          uVar13 = in_R9[7];
          lVar16 = *(longlong *)((longlong)in_RDX + 0x40);
          if (lVar16 == *(longlong *)((longlong)in_RDX + 0x30)) {
            alloc::raw_vec::RawVec::
            grow_one<indexmap::Bucket<tuple_<country_core::systems::collision::world::SubworldColliderHandle,country_core::systems::collision::world::SubworldColliderHandle>,country_core::systems::collision::intersect_meshes::MeshIntersectionArtifacts>,alloc::alloc::Global>
                      ();
          }
          lVar3 = *(longlong *)((longlong)in_RDX + 0x38);
          *(undefined8 *)(lVar3 + 0x40 + lVar16 * 0x48) = uVar13;
          puVar2 = (undefined8 *)(lVar3 + 0x30 + lVar16 * 0x48);
          *puVar2 = uVar11;
          puVar2[1] = uVar12;
          puVar2 = (undefined8 *)(lVar3 + 0x20 + lVar16 * 0x48);
          *puVar2 = uVar9;
          puVar2[1] = uVar10;
          puVar2 = (undefined8 *)(lVar3 + 0x10 + lVar16 * 0x48);
          *puVar2 = uVar7;
          puVar2[1] = uVar8;
          puVar2 = (undefined8 *)(lVar3 + lVar16 * 0x48);
          *puVar2 = local_a8;
          puVar2[1] = uVar6;
          *(longlong *)((longlong)in_RDX + 0x40) = lVar16 + 1;
          goto LAB_14096e735;
        }
        uVar14 = (ushort)(uVar18 - 1) & (ushort)uVar18;
        uVar18 = CONCAT22((short)(uVar18 - 1 >> 0x10),uVar14);
      }
      auVar26[0] = -(*pcVar1 == -1);
      auVar26[1] = -(pcVar1[1] == -1);
      auVar26[2] = -(pcVar1[2] == -1);
      auVar26[3] = -(pcVar1[3] == -1);
      auVar26[4] = -(pcVar1[4] == -1);
      auVar26[5] = -(pcVar1[5] == -1);
      auVar26[6] = -(pcVar1[6] == -1);
      auVar26[7] = -(pcVar1[7] == -1);
      auVar26[8] = -(pcVar1[8] == -1);
      auVar26[9] = -(pcVar1[9] == -1);
      auVar26[10] = -(pcVar1[10] == -1);
      auVar26[0xb] = -(pcVar1[0xb] == -1);
      auVar26[0xc] = -(pcVar1[0xc] == -1);
      auVar26[0xd] = -(pcVar1[0xd] == -1);
      auVar26[0xe] = -(pcVar1[0xe] == -1);
      auVar26[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar26 >> 7,0) & 1) != 0 || (SUB161(auVar26 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar26 >> 0x17,0) & 1) != 0) || (SUB161(auVar26 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar26 >> 0x27,0) & 1) != 0) || (SUB161(auVar26 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar26 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar26 >> 0x3f,0) & 1) != 0) || (SUB161(auVar26 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar26 >> 0x4f,0) & 1) != 0) || (SUB161(auVar26 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar26 >> 0x5f,0) & 1) != 0) || (SUB161(auVar26 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar26 >> 0x6f,0) & 1) != 0) || (SUB161(auVar26 >> 0x77,0) & 1) != 0) ||
          auVar26[0xf] < '\0') break;
      uVar19 = uVar19 + lVar17 + 0x10;
      lVar17 = lVar17 + 0x10;
    }
  }
  uVar6 = *in_R9;
  lVar16 = in_R9[1];
  rhapsody::raw::managed::handle::impl$6::drop();
  local_a8._0_4_ = (uint)uVar6;
  if ((((uint)local_a8 & 0xfffffffe) == 2) && (lVar16 != 0)) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
LAB_14096e735:
  memcpy(in_RCX,in_RDX,400);
  return;
}


