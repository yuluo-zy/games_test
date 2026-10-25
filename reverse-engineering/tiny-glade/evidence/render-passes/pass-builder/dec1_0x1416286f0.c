// address: 0x1416286f0
// pdb name: bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>
// signature: void __cdecl bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>(void)

void __cdecl
country_core::render::pass_builder::bind_resources::impl$0::
bind_color_library<country_core::render::pass_builder::draw_builder::DrawBuilder>(void)

{
  char *pcVar1;
  undefined1 uVar2;
  undefined4 uVar3;
  undefined1 (*pauVar4) [16];
  ulonglong uVar5;
  uint uVar6;
  undefined1 extraout_AL;
  void *in_RCX;
  void *in_RDX;
  ushort uVar7;
  ulonglong uVar8;
  byte bVar9;
  uint uVar10;
  ulonglong uVar11;
  longlong lVar12;
  uint uVar13;
  longlong lVar14;
  longlong lVar15;
  byte bVar16;
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  longlong local_80;
  longlong local_78;
  undefined8 *local_70;
  ulonglong local_68;
  
  hashbrown::map::HashMap::
  rustc_entry<ref_<str_>,rhapsody::rg::binding::PassResourceSlot,std::hash::random::RandomState,alloc::alloc::Global>
            ();
  if (local_80 == 0) {
    lVar14 = *(longlong *)((longlong)in_RDX + 0xe8);
    lVar12 = *(longlong *)(lVar14 + 0xb0);
    lVar15 = local_78;
  }
  else {
    pass_resource_access::
    read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>();
    pauVar4 = (undefined1 (*) [16])*local_70;
    uVar5 = local_70[1];
    uVar11 = uVar5 & local_68;
    auVar18 = *(undefined1 (*) [16])(*pauVar4 + uVar11);
    uVar7 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
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
    uVar13 = (uint)uVar7;
    if (uVar7 == 0) {
      lVar15 = 0x10;
      do {
        uVar11 = uVar11 + lVar15 & uVar5;
        auVar18 = *(undefined1 (*) [16])(*pauVar4 + uVar11);
        lVar15 = lVar15 + 0x10;
        uVar13 = (uint)(ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
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
      } while (uVar13 == 0);
    }
    uVar10 = 0;
    for (; (uVar13 & 1) == 0; uVar13 = uVar13 >> 1 | 0x80000000) {
      uVar10 = uVar10 + 1;
    }
    uVar11 = uVar10 + uVar11 & uVar5;
    bVar9 = (*pauVar4)[uVar11];
    if (-1 < (char)bVar9) {
      auVar18 = *pauVar4;
      uVar13 = 0;
      for (uVar10 = (uint)(ushort)((ushort)(SUB161(auVar18 >> 7,0) & 1) |
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
                                  (ushort)(byte)(auVar18[0xf] >> 7) << 0xf); (uVar10 & 1) == 0;
          uVar10 = uVar10 >> 1 | 0x80000000) {
        uVar13 = uVar13 + 1;
      }
      uVar11 = (ulonglong)uVar13;
      bVar9 = (*pauVar4)[uVar11];
    }
    bVar16 = (byte)(local_68 >> 0x39);
    (*pauVar4)[uVar11] = bVar16;
    pauVar4[1][uVar11 - 0x10 & uVar5] = bVar16;
    local_70[2] = local_70[2] - (ulonglong)(bVar9 & 1);
    lVar15 = (longlong)pauVar4 + uVar11 * -0x18;
    *(longlong *)((longlong)pauVar4 + uVar11 * -0x18 + -0x18) = local_80;
    *(longlong *)((longlong)pauVar4 + uVar11 * -0x18 + -0x10) = local_78;
    *(undefined1 *)((longlong)pauVar4 + uVar11 * -0x18 + -8) = extraout_AL;
    local_70[3] = local_70[3] + 1;
    lVar14 = *(longlong *)((longlong)in_RDX + 0xe8);
    lVar12 = *(longlong *)(lVar14 + 0xb0);
  }
  if (lVar12 != 0) {
    uVar2 = *(undefined1 *)(lVar15 + -8);
    lVar15 = *(longlong *)(lVar14 + 0x98);
    uVar5 = *(ulonglong *)(lVar14 + 0xa0);
    uVar11 = uVar5 & 0x52ec6a17beb8fef0;
    lVar14 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar15 + uVar11);
      auVar18[0] = -(*pcVar1 == ')');
      auVar18[1] = -(pcVar1[1] == ')');
      auVar18[2] = -(pcVar1[2] == ')');
      auVar18[3] = -(pcVar1[3] == ')');
      auVar18[4] = -(pcVar1[4] == ')');
      auVar18[5] = -(pcVar1[5] == ')');
      auVar18[6] = -(pcVar1[6] == ')');
      auVar18[7] = -(pcVar1[7] == ')');
      auVar18[8] = -(pcVar1[8] == ')');
      auVar18[9] = -(pcVar1[9] == ')');
      auVar18[10] = -(pcVar1[10] == ')');
      auVar18[0xb] = -(pcVar1[0xb] == ')');
      auVar18[0xc] = -(pcVar1[0xc] == ')');
      auVar18[0xd] = -(pcVar1[0xd] == ')');
      auVar18[0xe] = -(pcVar1[0xe] == ')');
      auVar18[0xf] = -(pcVar1[0xf] == ')');
      uVar7 = (ushort)(SUB161(auVar18 >> 7,0) & 1) | (ushort)(SUB161(auVar18 >> 0xf,0) & 1) << 1 |
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
              (ushort)(SUB161(auVar18 >> 0x77,0) & 1) << 0xe | (ushort)(auVar18[0xf] >> 7) << 0xf;
      uVar13 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar10 = 0;
        for (uVar6 = uVar13; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar10 = uVar10 + 1;
        }
        uVar8 = uVar10 + uVar11 & uVar5;
        if (*(longlong *)(lVar15 + -0x10 + uVar8 * -0x10) == 0x52ec6a17beb8fef0) {
          lVar14 = uVar8 * -0x10;
          uVar3 = *(undefined4 *)(lVar15 + -4 + lVar14);
          if (*(int *)(lVar15 + -8 + lVar14) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          lVar15 = *(longlong *)((longlong)in_RDX + 0x20);
          if (lVar15 == *(longlong *)((longlong)in_RDX + 0x10)) {
            alloc::raw_vec::RawVec::
            grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                      ();
          }
          lVar14 = *(longlong *)((longlong)in_RDX + 0x18);
          *(undefined4 *)(lVar14 + lVar15 * 8) = uVar3;
          *(undefined1 *)(lVar14 + 4 + lVar15 * 8) = uVar2;
          *(longlong *)((longlong)in_RDX + 0x20) = lVar15 + 1;
          goto LAB_14162890e;
        }
        uVar7 = (ushort)(uVar13 - 1) & (ushort)uVar13;
        uVar13 = CONCAT22((short)(uVar13 - 1 >> 0x10),uVar7);
      }
      auVar17[0] = -(*pcVar1 == -1);
      auVar17[1] = -(pcVar1[1] == -1);
      auVar17[2] = -(pcVar1[2] == -1);
      auVar17[3] = -(pcVar1[3] == -1);
      auVar17[4] = -(pcVar1[4] == -1);
      auVar17[5] = -(pcVar1[5] == -1);
      auVar17[6] = -(pcVar1[6] == -1);
      auVar17[7] = -(pcVar1[7] == -1);
      auVar17[8] = -(pcVar1[8] == -1);
      auVar17[9] = -(pcVar1[9] == -1);
      auVar17[10] = -(pcVar1[10] == -1);
      auVar17[0xb] = -(pcVar1[0xb] == -1);
      auVar17[0xc] = -(pcVar1[0xc] == -1);
      auVar17[0xd] = -(pcVar1[0xd] == -1);
      auVar17[0xe] = -(pcVar1[0xe] == -1);
      auVar17[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar17 >> 7,0) & 1) != 0 || (SUB161(auVar17 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar17 >> 0x17,0) & 1) != 0) || (SUB161(auVar17 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar17 >> 0x27,0) & 1) != 0) || (SUB161(auVar17 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar17 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar17 >> 0x3f,0) & 1) != 0) || (SUB161(auVar17 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar17 >> 0x4f,0) & 1) != 0) || (SUB161(auVar17 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar17 >> 0x5f,0) & 1) != 0) || (SUB161(auVar17 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar17 >> 0x6f,0) & 1) != 0) || (SUB161(auVar17 >> 0x77,0) & 1) != 0) ||
          auVar17[0xf] < '\0') break;
      lVar12 = uVar11 + lVar14;
      lVar14 = lVar14 + 0x10;
      uVar11 = lVar12 + 0x10U & uVar5;
    }
  }
LAB_14162890e:
  memcpy(in_RCX,in_RDX,0xf8);
  return;
}


