// 0x1405896e0 bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass> size=384

void __cdecl
country_core::render::pass_builder::bind_resources::impl$0::
bind_color_library<country_core::render::pass_builder::compute_pass_builder::ComputePass>(void)

{
  char *pcVar1;
  undefined4 uVar2;
  longlong lVar3;
  ulonglong uVar4;
  uint uVar5;
  uint uVar6;
  undefined1 extraout_AL;
  void *in_RCX;
  void *in_RDX;
  ushort uVar7;
  ulonglong uVar8;
  ulonglong uVar9;
  longlong lVar10;
  longlong lVar11;
  uint uVar12;
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  
  pass_resource_access::
  read_access_buffer<country_core::asset_libraries::color_library::ColorLibrarySsbo>();
  lVar11 = *(longlong *)((longlong)in_RDX + 0x138);
  if (*(longlong *)(lVar11 + 0xb0) != 0) {
    lVar3 = *(longlong *)(lVar11 + 0x98);
    uVar4 = *(ulonglong *)(lVar11 + 0xa0);
    uVar9 = uVar4 & 0x52ec6a17beb8fef0;
    lVar11 = 0;
    while( true ) {
      pcVar1 = (char *)(lVar3 + uVar9);
      auVar14[0] = -(*pcVar1 == ')');
      auVar14[1] = -(pcVar1[1] == ')');
      auVar14[2] = -(pcVar1[2] == ')');
      auVar14[3] = -(pcVar1[3] == ')');
      auVar14[4] = -(pcVar1[4] == ')');
      auVar14[5] = -(pcVar1[5] == ')');
      auVar14[6] = -(pcVar1[6] == ')');
      auVar14[7] = -(pcVar1[7] == ')');
      auVar14[8] = -(pcVar1[8] == ')');
      auVar14[9] = -(pcVar1[9] == ')');
      auVar14[10] = -(pcVar1[10] == ')');
      auVar14[0xb] = -(pcVar1[0xb] == ')');
      auVar14[0xc] = -(pcVar1[0xc] == ')');
      auVar14[0xd] = -(pcVar1[0xd] == ')');
      auVar14[0xe] = -(pcVar1[0xe] == ')');
      auVar14[0xf] = -(pcVar1[0xf] == ')');
      uVar7 = (ushort)(SUB161(auVar14 >> 7,0) & 1) | (ushort)(SUB161(auVar14 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar14 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar14 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar14 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar14 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar14 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar14 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar14 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar14 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar14 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar14 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar14 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar14 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar14 >> 0x77,0) & 1) << 0xe | (ushort)(auVar14[0xf] >> 7) << 0xf;
      uVar12 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar12; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar8 = uVar5 + uVar9 & uVar4;
        if (*(longlong *)(lVar3 + -0x10 + uVar8 * -0x10) == 0x52ec6a17beb8fef0) {
          lVar11 = uVar8 * -0x10;
          uVar2 = *(undefined4 *)(lVar3 + -4 + lVar11);
          if (*(int *)(lVar3 + -8 + lVar11) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          lVar11 = *(longlong *)((longlong)in_RDX + 0x28);
          if (lVar11 == *(longlong *)((longlong)in_RDX + 0x18)) {
            alloc::raw_vec::RawVec::
            grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                      ();
          }
          lVar3 = *(longlong *)((longlong)in_RDX + 0x20);
          *(undefined4 *)(lVar3 + lVar11 * 8) = uVar2;
          *(undefined1 *)(lVar3 + 4 + lVar11 * 8) = extraout_AL;
          *(longlong *)((longlong)in_RDX + 0x28) = lVar11 + 1;
          goto LAB_14058981a;
        }
        uVar7 = (ushort)(uVar12 - 1) & (ushort)uVar12;
        uVar12 = CONCAT22((short)(uVar12 - 1 >> 0x10),uVar7);
      }
      auVar13[0] = -(*pcVar1 == -1);
      auVar13[1] = -(pcVar1[1] == -1);
      auVar13[2] = -(pcVar1[2] == -1);
      auVar13[3] = -(pcVar1[3] == -1);
      auVar13[4] = -(pcVar1[4] == -1);
      auVar13[5] = -(pcVar1[5] == -1);
      auVar13[6] = -(pcVar1[6] == -1);
      auVar13[7] = -(pcVar1[7] == -1);
      auVar13[8] = -(pcVar1[8] == -1);
      auVar13[9] = -(pcVar1[9] == -1);
      auVar13[10] = -(pcVar1[10] == -1);
      auVar13[0xb] = -(pcVar1[0xb] == -1);
      auVar13[0xc] = -(pcVar1[0xc] == -1);
      auVar13[0xd] = -(pcVar1[0xd] == -1);
      auVar13[0xe] = -(pcVar1[0xe] == -1);
      auVar13[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar13 >> 7,0) & 1) != 0 || (SUB161(auVar13 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar13 >> 0x17,0) & 1) != 0) || (SUB161(auVar13 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar13 >> 0x27,0) & 1) != 0) || (SUB161(auVar13 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar13 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar13 >> 0x3f,0) & 1) != 0) || (SUB161(auVar13 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar13 >> 0x4f,0) & 1) != 0) || (SUB161(auVar13 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar13 >> 0x5f,0) & 1) != 0) || (SUB161(auVar13 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar13 >> 0x6f,0) & 1) != 0) || (SUB161(auVar13 >> 0x77,0) & 1) != 0) ||
          auVar13[0xf] < '\0') break;
      lVar10 = uVar9 + lVar11;
      lVar11 = lVar11 + 0x10;
      uVar9 = lVar10 + 0x10U & uVar4;
    }
  }
LAB_14058981a:
  memcpy(in_RCX,in_RDX,400);
  return;
}

