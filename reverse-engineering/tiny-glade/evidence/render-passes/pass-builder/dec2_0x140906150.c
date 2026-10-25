// address: 0x140906150
// pdb name: read_rg_storage_buffer
// signature: void __cdecl read_rg_storage_buffer(void)

void __cdecl
country_core::render::pass_builder::draw_builder::DrawBuilder::read_rg_storage_buffer(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  void *in_RCX;
  void *in_RDX;
  ushort uVar7;
  ulonglong uVar8;
  longlong lVar9;
  ulonglong in_R8;
  longlong lVar10;
  uint uVar11;
  ulonglong uVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined8 local_98;
  undefined8 uStack_90;
  undefined8 local_88;
  undefined8 uStack_80;
  undefined8 local_78;
  undefined8 uStack_70;
  undefined8 local_68;
  undefined8 uStack_60;
  undefined4 local_58;
  
  lVar9 = *(longlong *)((longlong)in_RDX + 0xe8);
  if (*(longlong *)(lVar9 + 0xb0) != 0) {
    lVar4 = *(longlong *)(lVar9 + 0x98);
    bVar13 = (byte)(in_R8 >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    lVar10 = 0;
    uVar12 = in_R8;
    while( true ) {
      uVar12 = *(ulonglong *)(lVar9 + 0xa0) & uVar12;
      pcVar1 = (char *)(lVar4 + uVar12);
      cVar14 = auVar15[0];
      auVar20[0] = -(*pcVar1 == cVar14);
      cVar16 = auVar15[1];
      auVar20[1] = -(pcVar1[1] == cVar16);
      cVar17 = auVar15[2];
      auVar20[2] = -(pcVar1[2] == cVar17);
      cVar18 = auVar15[3];
      auVar20[3] = -(pcVar1[3] == cVar18);
      auVar20[4] = -(pcVar1[4] == cVar14);
      auVar20[5] = -(pcVar1[5] == cVar16);
      auVar20[6] = -(pcVar1[6] == cVar17);
      auVar20[7] = -(pcVar1[7] == cVar18);
      auVar20[8] = -(pcVar1[8] == cVar14);
      auVar20[9] = -(pcVar1[9] == cVar16);
      auVar20[10] = -(pcVar1[10] == cVar17);
      auVar20[0xb] = -(pcVar1[0xb] == cVar18);
      auVar20[0xc] = -(pcVar1[0xc] == cVar14);
      auVar20[0xd] = -(pcVar1[0xd] == cVar16);
      auVar20[0xe] = -(pcVar1[0xe] == cVar17);
      auVar20[0xf] = -(pcVar1[0xf] == cVar18);
      uVar7 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
              (ushort)(SUB161(auVar20 >> 0x17,0) & 1) << 2 |
              (ushort)(SUB161(auVar20 >> 0x1f,0) & 1) << 3 |
              (ushort)(SUB161(auVar20 >> 0x27,0) & 1) << 4 |
              (ushort)(SUB161(auVar20 >> 0x2f,0) & 1) << 5 |
              (ushort)(SUB161(auVar20 >> 0x37,0) & 1) << 6 |
              (ushort)(SUB161(auVar20 >> 0x3f,0) & 1) << 7 |
              (ushort)(SUB161(auVar20 >> 0x47,0) & 1) << 8 |
              (ushort)(SUB161(auVar20 >> 0x4f,0) & 1) << 9 |
              (ushort)(SUB161(auVar20 >> 0x57,0) & 1) << 10 |
              (ushort)(SUB161(auVar20 >> 0x5f,0) & 1) << 0xb |
              (ushort)(SUB161(auVar20 >> 0x67,0) & 1) << 0xc |
              (ushort)(SUB161(auVar20 >> 0x6f,0) & 1) << 0xd |
              (ushort)(SUB161(auVar20 >> 0x77,0) & 1) << 0xe | (ushort)(auVar20[0xf] >> 7) << 0xf;
      uVar11 = (uint)uVar7;
      while (uVar7 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar11; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar8 = uVar5 + uVar12 & *(ulonglong *)(lVar9 + 0xa0);
        if (*(ulonglong *)(lVar4 + -0x10 + uVar8 * -0x10) == in_R8) {
          lVar9 = uVar8 * -0x10;
          uVar3 = *(undefined4 *)(lVar4 + -4 + lVar9);
          if (*(int *)(lVar4 + -8 + lVar9) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          rhapsody::rg::pass_builder::PassBuilder::
          add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
          rhapsody::rg::pass_api::impl$5::bind();
          lVar9 = *(longlong *)((longlong)in_RDX + 0xe0);
          uVar12 = *(ulonglong *)(lVar9 + 0x28);
          if (0xff < uVar12) {
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          if (uVar12 == *(ulonglong *)(lVar9 + 0x18)) {
            alloc::raw_vec::RawVec::
            grow_one<enum2_<rhapsody::rg::pass_api::RenderPassBinding>,alloc::alloc::Global>();
          }
          lVar4 = *(longlong *)(lVar9 + 0x20);
          lVar10 = uVar12 * 0x44;
          *(undefined4 *)(lVar4 + 0x40 + lVar10) = local_58;
          puVar2 = (undefined8 *)(lVar4 + 0x30 + lVar10);
          *puVar2 = local_68;
          puVar2[1] = uStack_60;
          puVar2 = (undefined8 *)(lVar4 + 0x20 + lVar10);
          *puVar2 = local_78;
          puVar2[1] = uStack_70;
          puVar2 = (undefined8 *)(lVar4 + 0x10 + lVar10);
          *puVar2 = local_88;
          puVar2[1] = uStack_80;
          *(undefined8 *)(lVar4 + lVar10) = local_98;
          ((undefined8 *)(lVar4 + lVar10))[1] = uStack_90;
          *(ulonglong *)(lVar9 + 0x28) = uVar12 + 1;
          lVar9 = *(longlong *)((longlong)in_RDX + 0x20);
          if (lVar9 == *(longlong *)((longlong)in_RDX + 0x10)) {
            alloc::raw_vec::RawVec::
            grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                      ();
          }
          lVar4 = *(longlong *)((longlong)in_RDX + 0x18);
          *(undefined4 *)(lVar4 + lVar9 * 8) = uVar3;
          *(char *)(lVar4 + 4 + lVar9 * 8) = (char)uVar12;
          *(longlong *)((longlong)in_RDX + 0x20) = lVar9 + 1;
          goto LAB_140906340;
        }
        uVar7 = (ushort)(uVar11 - 1) & (ushort)uVar11;
        uVar11 = CONCAT22((short)(uVar11 - 1 >> 0x10),uVar7);
      }
      auVar19[0] = -(*pcVar1 == -1);
      auVar19[1] = -(pcVar1[1] == -1);
      auVar19[2] = -(pcVar1[2] == -1);
      auVar19[3] = -(pcVar1[3] == -1);
      auVar19[4] = -(pcVar1[4] == -1);
      auVar19[5] = -(pcVar1[5] == -1);
      auVar19[6] = -(pcVar1[6] == -1);
      auVar19[7] = -(pcVar1[7] == -1);
      auVar19[8] = -(pcVar1[8] == -1);
      auVar19[9] = -(pcVar1[9] == -1);
      auVar19[10] = -(pcVar1[10] == -1);
      auVar19[0xb] = -(pcVar1[0xb] == -1);
      auVar19[0xc] = -(pcVar1[0xc] == -1);
      auVar19[0xd] = -(pcVar1[0xd] == -1);
      auVar19[0xe] = -(pcVar1[0xe] == -1);
      auVar19[0xf] = -(pcVar1[0xf] == -1);
      if ((((((((((((((((SUB161(auVar19 >> 7,0) & 1) != 0 || (SUB161(auVar19 >> 0xf,0) & 1) != 0) ||
                      (SUB161(auVar19 >> 0x17,0) & 1) != 0) || (SUB161(auVar19 >> 0x1f,0) & 1) != 0)
                    || (SUB161(auVar19 >> 0x27,0) & 1) != 0) || (SUB161(auVar19 >> 0x2f,0) & 1) != 0
                   ) || (SUB161(auVar19 >> 0x37,0) & 1) != 0) ||
                 (SUB161(auVar19 >> 0x3f,0) & 1) != 0) || (SUB161(auVar19 >> 0x47,0) & 1) != 0) ||
               (SUB161(auVar19 >> 0x4f,0) & 1) != 0) || (SUB161(auVar19 >> 0x57,0) & 1) != 0) ||
             (SUB161(auVar19 >> 0x5f,0) & 1) != 0) || (SUB161(auVar19 >> 0x67,0) & 1) != 0) ||
           (SUB161(auVar19 >> 0x6f,0) & 1) != 0) || (SUB161(auVar19 >> 0x77,0) & 1) != 0) ||
          auVar19[0xf] < '\0') break;
      uVar12 = uVar12 + lVar10 + 0x10;
      lVar10 = lVar10 + 0x10;
    }
  }
LAB_140906340:
  memcpy(in_RCX,in_RDX,0xf8);
  return;
}


