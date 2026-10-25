// address: 0x140906400
// pdb name: bind_external_storage_buffer
// signature: void __cdecl bind_external_storage_buffer(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::bind_external_storage_buffer(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  int iVar3;
  longlong lVar4;
  undefined1 (*pauVar5) [16];
  uint uVar6;
  ulonglong extraout_RAX;
  ulonglong extraout_RAX_00;
  void *in_RCX;
  uint uVar7;
  uint uVar8;
  void *in_RDX;
  ulonglong uVar9;
  ushort uVar10;
  ulonglong uVar11;
  longlong lVar12;
  ulonglong in_R8;
  undefined8 in_R9;
  longlong lVar13;
  byte bVar14;
  char cVar15;
  byte bVar16;
  char cVar18;
  char cVar19;
  char cVar20;
  undefined1 auVar17 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  undefined1 auVar23 [16];
  undefined1 auVar24 [16];
  undefined8 local_b8;
  undefined8 uStack_b0;
  undefined8 local_a8;
  
  lVar12 = *(longlong *)((longlong)in_RDX + 0xe8);
  if (*(longlong *)(lVar12 + 0xb0) != 0) {
    lVar4 = *(longlong *)(lVar12 + 0x98);
    bVar14 = (byte)(in_R8 >> 0x39);
    auVar17 = ZEXT216(CONCAT11(bVar14,bVar14));
    auVar17 = pshuflw(auVar17,auVar17,0);
    lVar13 = 0;
    uVar9 = in_R8;
    while( true ) {
      uVar9 = *(ulonglong *)(lVar12 + 0xa0) & uVar9;
      pcVar1 = (char *)(lVar4 + uVar9);
      cVar15 = auVar17[0];
      auVar23[0] = -(*pcVar1 == cVar15);
      cVar18 = auVar17[1];
      auVar23[1] = -(pcVar1[1] == cVar18);
      cVar19 = auVar17[2];
      auVar23[2] = -(pcVar1[2] == cVar19);
      cVar20 = auVar17[3];
      auVar23[3] = -(pcVar1[3] == cVar20);
      auVar23[4] = -(pcVar1[4] == cVar15);
      auVar23[5] = -(pcVar1[5] == cVar18);
      auVar23[6] = -(pcVar1[6] == cVar19);
      auVar23[7] = -(pcVar1[7] == cVar20);
      auVar23[8] = -(pcVar1[8] == cVar15);
      auVar23[9] = -(pcVar1[9] == cVar18);
      auVar23[10] = -(pcVar1[10] == cVar19);
      auVar23[0xb] = -(pcVar1[0xb] == cVar20);
      auVar23[0xc] = -(pcVar1[0xc] == cVar15);
      auVar23[0xd] = -(pcVar1[0xd] == cVar18);
      auVar23[0xe] = -(pcVar1[0xe] == cVar19);
      auVar23[0xf] = -(pcVar1[0xf] == cVar20);
      uVar10 = (ushort)(SUB161(auVar23 >> 7,0) & 1) | (ushort)(SUB161(auVar23 >> 0xf,0) & 1) << 1 |
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
      uVar7 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar8 = 0;
        for (uVar6 = uVar7; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar8 = uVar8 + 1;
        }
        uVar11 = uVar8 + uVar9 & *(ulonglong *)(lVar12 + 0xa0);
        if (*(ulonglong *)(lVar4 + -0x10 + uVar11 * -0x10) == in_R8) {
          lVar12 = uVar11 * -0x10;
          iVar3 = *(int *)(lVar4 + -4 + lVar12);
          if (*(int *)(lVar4 + -8 + lVar12) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          local_b8 = CONCAT44(local_b8._4_4_,iVar3);
          core::hash::BuildHasher::
          hash_one<std::hash::random::RandomState,ref_<shader_pipeline::reflection::BindingIndex>_>
                    ();
          bVar14 = (byte)(extraout_RAX >> 0x38);
          bVar16 = bVar14 >> 1;
          auVar17 = ZEXT216(CONCAT11(bVar16,bVar16));
          auVar17 = pshuflw(auVar17,auVar17,0);
          lVar12 = 0;
          uVar9 = extraout_RAX;
          goto LAB_14090655a;
        }
        uVar10 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar10);
      }
      auVar21[0] = -(*pcVar1 == -1);
      auVar21[1] = -(pcVar1[1] == -1);
      auVar21[2] = -(pcVar1[2] == -1);
      auVar21[3] = -(pcVar1[3] == -1);
      auVar21[4] = -(pcVar1[4] == -1);
      auVar21[5] = -(pcVar1[5] == -1);
      auVar21[6] = -(pcVar1[6] == -1);
      auVar21[7] = -(pcVar1[7] == -1);
      auVar21[8] = -(pcVar1[8] == -1);
      auVar21[9] = -(pcVar1[9] == -1);
      auVar21[10] = -(pcVar1[10] == -1);
      auVar21[0xb] = -(pcVar1[0xb] == -1);
      auVar21[0xc] = -(pcVar1[0xc] == -1);
      auVar21[0xd] = -(pcVar1[0xd] == -1);
      auVar21[0xe] = -(pcVar1[0xe] == -1);
      auVar21[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar21 >> 7,0) & 1) != 0 || (SUB161(auVar21 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar21 >> 0x17,0) & 1) != 0) || (SUB161(auVar21 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar21 >> 0x27,0) & 1) != 0) || (SUB161(auVar21 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar21 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar21 >> 0x3f,0) & 1) != 0) || (SUB161(auVar21 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar21 >> 0x4f,0) & 1) != 0) || (SUB161(auVar21 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar21 >> 0x5f,0) & 1) != 0) || (SUB161(auVar21 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar21 >> 0x6f,0) & 1) != 0) || (SUB161(auVar21 >> 0x77,0) & 1) != 0) ||
          auVar21[0xf] < '\0') break;
      uVar9 = uVar9 + lVar13 + 0x10;
      lVar13 = lVar13 + 0x10;
    }
  }
LAB_1409066d9:
  memcpy(in_RCX,in_RDX,0xf8);
  return;
LAB_14090655a:
  uVar9 = *(ulonglong *)((longlong)in_RDX + 0x78) & uVar9;
  pcVar1 = (char *)(*(longlong *)((longlong)in_RDX + 0x70) + uVar9);
  cVar15 = auVar17[0];
  auVar24[0] = -(*pcVar1 == cVar15);
  cVar18 = auVar17[1];
  auVar24[1] = -(pcVar1[1] == cVar18);
  cVar19 = auVar17[2];
  auVar24[2] = -(pcVar1[2] == cVar19);
  cVar20 = auVar17[3];
  auVar24[3] = -(pcVar1[3] == cVar20);
  auVar24[4] = -(pcVar1[4] == cVar15);
  auVar24[5] = -(pcVar1[5] == cVar18);
  auVar24[6] = -(pcVar1[6] == cVar19);
  auVar24[7] = -(pcVar1[7] == cVar20);
  auVar24[8] = -(pcVar1[8] == cVar15);
  auVar24[9] = -(pcVar1[9] == cVar18);
  auVar24[10] = -(pcVar1[10] == cVar19);
  auVar24[0xb] = -(pcVar1[0xb] == cVar20);
  auVar24[0xc] = -(pcVar1[0xc] == cVar15);
  auVar24[0xd] = -(pcVar1[0xd] == cVar18);
  auVar24[0xe] = -(pcVar1[0xe] == cVar19);
  auVar24[0xf] = -(pcVar1[0xf] == cVar20);
  uVar10 = (ushort)(SUB161(auVar24 >> 7,0) & 1) | (ushort)(SUB161(auVar24 >> 0xf,0) & 1) << 1 |
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
  uVar7 = (uint)uVar10;
  while (uVar10 != 0) {
    uVar8 = 0;
    for (uVar6 = uVar7; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
      uVar8 = uVar8 + 1;
    }
    if (*(int *)(*(longlong *)((longlong)in_RDX + 0x70) + -0x48 +
                (uVar8 + uVar9 & *(ulonglong *)((longlong)in_RDX + 0x78)) * -0x48) == iVar3)
    goto LAB_1409066d9;
    uVar10 = (ushort)(uVar7 - 1) & (ushort)uVar7;
    uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar10);
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
                  (SUB161(auVar22 >> 0x17,0) & 1) != 0) || (SUB161(auVar22 >> 0x1f,0) & 1) != 0) ||
                (SUB161(auVar22 >> 0x27,0) & 1) != 0) || (SUB161(auVar22 >> 0x2f,0) & 1) != 0) ||
              (SUB161(auVar22 >> 0x37,0) & 1) != 0) || (SUB161(auVar22 >> 0x3f,0) & 1) != 0) ||
            (SUB161(auVar22 >> 0x47,0) & 1) != 0) || (SUB161(auVar22 >> 0x4f,0) & 1) != 0) ||
          (SUB161(auVar22 >> 0x57,0) & 1) != 0) || (SUB161(auVar22 >> 0x5f,0) & 1) != 0) ||
        (SUB161(auVar22 >> 0x67,0) & 1) != 0) || (SUB161(auVar22 >> 0x6f,0) & 1) != 0) ||
      (SUB161(auVar22 >> 0x77,0) & 1) != 0) || auVar22[0xf] < '\0') goto LAB_1409065eb;
  uVar9 = uVar9 + lVar12 + 0x10;
  lVar12 = lVar12 + 0x10;
  goto LAB_14090655a;
LAB_1409065eb:
  if (*(longlong *)((longlong)in_RDX + 0x80) == 0) {
    hashbrown::raw::RawTable::
    reserve_rehash<tuple_<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2_<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>_>,alloc::alloc::Global,hashbrown::map::make_hasher::closure_env_0<shader_pipeline::reflection::BindingIndex,rhapsody::raw::managed::handle::GpuOwned<enum2_<rhapsody::raw::binding::DescriptorSetBinding>,rhapsody::raw::managed::handle::RawManagedHandle>,std::hash::random::RandomState>_>
              ();
  }
  rhapsody::raw::device::GraphicsDevice::get_vk_buffer();
  if ((extraout_RAX_00 & 1) == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  rhapsody::raw::managed::handle::impl$5::clone();
  pauVar5 = *(undefined1 (**) [16])((longlong)in_RDX + 0x70);
  uVar9 = *(ulonglong *)((longlong)in_RDX + 0x78);
  uVar11 = extraout_RAX & uVar9;
  auVar17 = *(undefined1 (*) [16])(*pauVar5 + uVar11);
  uVar10 = (ushort)(SUB161(auVar17 >> 7,0) & 1) | (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
           (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
           (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
           (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
           (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
           (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
           (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
           (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
           (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
           (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
           (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
           (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
           (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
           (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar17[0xf] >> 7) << 0xf
  ;
  uVar7 = (uint)uVar10;
  if (uVar10 == 0) {
    lVar12 = 0x10;
    do {
      uVar11 = uVar11 + lVar12 & uVar9;
      auVar17 = *(undefined1 (*) [16])(*pauVar5 + uVar11);
      lVar12 = lVar12 + 0x10;
      uVar7 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
                             (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
                             (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
                             (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
                             (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
                             (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
                             (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
                             (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
                             (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
                             (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
                             (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
                             (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
                             (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
                             (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
                             (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
                            (ushort)(byte)(auVar17[0xf] >> 7) << 0xf);
    } while (uVar7 == 0);
  }
  uVar8 = 0;
  for (; (uVar7 & 1) == 0; uVar7 = uVar7 >> 1 | 0x80000000) {
    uVar8 = uVar8 + 1;
  }
  uVar11 = uVar8 + uVar11 & uVar9;
  bVar16 = (*pauVar5)[uVar11];
  if (-1 < (char)bVar16) {
    auVar17 = *pauVar5;
    uVar7 = 0;
    for (uVar8 = (uint)(ushort)((ushort)(SUB161(auVar17 >> 7,0) & 1) |
                                (ushort)(SUB161(auVar17 >> 0xf,0) & 1) << 1 |
                                (ushort)(SUB161(auVar17 >> 0x17,0) & 1) << 2 |
                                (ushort)(SUB161(auVar17 >> 0x1f,0) & 1) << 3 |
                                (ushort)(SUB161(auVar17 >> 0x27,0) & 1) << 4 |
                                (ushort)(SUB161(auVar17 >> 0x2f,0) & 1) << 5 |
                                (ushort)(SUB161(auVar17 >> 0x37,0) & 1) << 6 |
                                (ushort)(SUB161(auVar17 >> 0x3f,0) & 1) << 7 |
                                (ushort)(SUB161(auVar17 >> 0x47,0) & 1) << 8 |
                                (ushort)(SUB161(auVar17 >> 0x4f,0) & 1) << 9 |
                                (ushort)(SUB161(auVar17 >> 0x57,0) & 1) << 10 |
                                (ushort)(SUB161(auVar17 >> 0x5f,0) & 1) << 0xb |
                                (ushort)(SUB161(auVar17 >> 0x67,0) & 1) << 0xc |
                                (ushort)(SUB161(auVar17 >> 0x6f,0) & 1) << 0xd |
                                (ushort)(SUB161(auVar17 >> 0x77,0) & 1) << 0xe |
                               (ushort)(byte)(auVar17[0xf] >> 7) << 0xf); (uVar8 & 1) == 0;
        uVar8 = uVar8 >> 1 | 0x80000000) {
      uVar7 = uVar7 + 1;
    }
    uVar11 = (ulonglong)uVar7;
    bVar16 = (*pauVar5)[uVar11];
  }
  bVar14 = bVar14 >> 1;
  (*pauVar5)[uVar11] = bVar14;
  pauVar5[1][uVar11 - 0x10 & uVar9] = bVar14;
  *(longlong *)((longlong)in_RDX + 0x80) =
       *(longlong *)((longlong)in_RDX + 0x80) - (ulonglong)(bVar16 & 1);
  *(int *)((longlong)pauVar5 + uVar11 * -0x48 + -0x48) = iVar3;
  *(undefined4 *)((longlong)pauVar5 + uVar11 * -0x48 + -0x40) = 5;
  *(undefined8 *)((longlong)pauVar5 + uVar11 * -0x48 + -0x38) = in_R9;
  *(undefined8 *)((longlong)pauVar5 + uVar11 * -0x48 + -0x30) = 0;
  *(undefined8 *)((longlong)pauVar5 + uVar11 * -0x48 + -0x28) = 0xffffffffffffffff;
  puVar2 = (undefined8 *)((longlong)pauVar5 + uVar11 * -0x48 + -0x20);
  *puVar2 = local_b8;
  puVar2[1] = uStack_b0;
  *(undefined8 *)((longlong)pauVar5 + uVar11 * -0x48 + -0x10) = local_a8;
  *(undefined1 *)((longlong)pauVar5 + uVar11 * -0x48 + -8) = 0;
  *(longlong *)((longlong)in_RDX + 0x88) = *(longlong *)((longlong)in_RDX + 0x88) + 1;
  goto LAB_1409066d9;
}


