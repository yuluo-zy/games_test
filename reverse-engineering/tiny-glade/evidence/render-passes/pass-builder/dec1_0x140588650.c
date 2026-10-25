// address: 0x140588650
// pdb name: read_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>
// signature: void __cdecl read_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::impl$3::
read_storage_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>(void)

{
  char *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  uint uVar4;
  uint uVar5;
  undefined1 extraout_AL;
  void *in_RCX;
  void *in_RDX;
  ulonglong in_R8;
  longlong lVar6;
  uint uVar7;
  ushort uVar8;
  ulonglong uVar9;
  longlong lVar10;
  ulonglong uVar11;
  byte bVar12;
  char cVar13;
  char cVar15;
  char cVar16;
  char cVar17;
  undefined1 auVar14 [16];
  undefined1 auVar18 [16];
  undefined1 auVar19 [16];
  
  pass_resource_access::
  read_access_buffer<country_core::resources::world_raster_defs::Cobblestone2RasterStagingSsbo>();
  lVar10 = *(longlong *)((longlong)in_RDX + 0x138);
  if (*(longlong *)(lVar10 + 0xb0) != 0) {
    lVar3 = *(longlong *)(lVar10 + 0x98);
    bVar12 = (byte)(in_R8 >> 0x39);
    auVar14 = ZEXT216(CONCAT11(bVar12,bVar12));
    auVar14 = pshuflw(auVar14,auVar14,0);
    lVar6 = 0;
    uVar11 = in_R8;
    while( true ) {
      uVar11 = *(ulonglong *)(lVar10 + 0xa0) & uVar11;
      pcVar1 = (char *)(lVar3 + uVar11);
      cVar13 = auVar14[0];
      auVar19[0] = -(*pcVar1 == cVar13);
      cVar15 = auVar14[1];
      auVar19[1] = -(pcVar1[1] == cVar15);
      cVar16 = auVar14[2];
      auVar19[2] = -(pcVar1[2] == cVar16);
      cVar17 = auVar14[3];
      auVar19[3] = -(pcVar1[3] == cVar17);
      auVar19[4] = -(pcVar1[4] == cVar13);
      auVar19[5] = -(pcVar1[5] == cVar15);
      auVar19[6] = -(pcVar1[6] == cVar16);
      auVar19[7] = -(pcVar1[7] == cVar17);
      auVar19[8] = -(pcVar1[8] == cVar13);
      auVar19[9] = -(pcVar1[9] == cVar15);
      auVar19[10] = -(pcVar1[10] == cVar16);
      auVar19[0xb] = -(pcVar1[0xb] == cVar17);
      auVar19[0xc] = -(pcVar1[0xc] == cVar13);
      auVar19[0xd] = -(pcVar1[0xd] == cVar15);
      auVar19[0xe] = -(pcVar1[0xe] == cVar16);
      auVar19[0xf] = -(pcVar1[0xf] == cVar17);
      uVar8 = (ushort)(SUB161(auVar19 >> 7,0) & 1) | (ushort)(SUB161(auVar19 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar19 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar19 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar19 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar19 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar19 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar19 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar19 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar19 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar19 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar19 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar19 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar19 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar19 >> 0x77,0) & 1) << 0xe | (ushort)(auVar19[0xf] >> 7) << 0xf;
      uVar7 = (uint)uVar8;
      while (uVar8 != 0) {
        uVar4 = 0;
        for (uVar5 = uVar7; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
          uVar4 = uVar4 + 1;
        }
        uVar9 = uVar4 + uVar11 & *(ulonglong *)(lVar10 + 0xa0);
        if (*(ulonglong *)(lVar3 + -0x10 + uVar9 * -0x10) == in_R8) {
          lVar10 = uVar9 * -0x10;
          uVar2 = *(undefined4 *)(lVar3 + -4 + lVar10);
          if (*(int *)(lVar3 + -8 + lVar10) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          lVar10 = *(longlong *)((longlong)in_RDX + 0x28);
          if (lVar10 == *(longlong *)((longlong)in_RDX + 0x18)) {
            alloc::raw_vec::RawVec::
            grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                      ();
          }
          lVar3 = *(longlong *)((longlong)in_RDX + 0x20);
          *(undefined4 *)(lVar3 + lVar10 * 8) = uVar2;
          *(undefined1 *)(lVar3 + 4 + lVar10 * 8) = extraout_AL;
          *(longlong *)((longlong)in_RDX + 0x28) = lVar10 + 1;
          goto LAB_14058879c;
        }
        uVar8 = (ushort)(uVar7 - 1) & (ushort)uVar7;
        uVar7 = CONCAT22((short)(uVar7 - 1 >> 0x10),uVar8);
      }
      auVar18[0] = -(*pcVar1 == -1);
      auVar18[1] = -(pcVar1[1] == -1);
      auVar18[2] = -(pcVar1[2] == -1);
      auVar18[3] = -(pcVar1[3] == -1);
      auVar18[4] = -(pcVar1[4] == -1);
      auVar18[5] = -(pcVar1[5] == -1);
      auVar18[6] = -(pcVar1[6] == -1);
      auVar18[7] = -(pcVar1[7] == -1);
      auVar18[8] = -(pcVar1[8] == -1);
      auVar18[9] = -(pcVar1[9] == -1);
      auVar18[10] = -(pcVar1[10] == -1);
      auVar18[0xb] = -(pcVar1[0xb] == -1);
      auVar18[0xc] = -(pcVar1[0xc] == -1);
      auVar18[0xd] = -(pcVar1[0xd] == -1);
      auVar18[0xe] = -(pcVar1[0xe] == -1);
      auVar18[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar18 >> 7,0) & 1) != 0 || (SUB161(auVar18 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar18 >> 0x17,0) & 1) != 0) || (SUB161(auVar18 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar18 >> 0x27,0) & 1) != 0) || (SUB161(auVar18 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar18 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar18 >> 0x3f,0) & 1) != 0) || (SUB161(auVar18 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar18 >> 0x4f,0) & 1) != 0) || (SUB161(auVar18 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar18 >> 0x5f,0) & 1) != 0) || (SUB161(auVar18 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar18 >> 0x6f,0) & 1) != 0) || (SUB161(auVar18 >> 0x77,0) & 1) != 0) ||
          auVar18[0xf] < '\0') break;
      uVar11 = uVar11 + lVar6 + 0x10;
      lVar6 = lVar6 + 0x10;
    }
  }
LAB_14058879c:
  memcpy(in_RCX,in_RDX,400);
  return;
}


