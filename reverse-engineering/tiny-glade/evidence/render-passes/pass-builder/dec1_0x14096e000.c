// address: 0x14096e000
// pdb name: write_rg_storage_buffer_concurrent
// signature: void __cdecl write_rg_storage_buffer_concurrent(void)

void __cdecl
country_core::render::pass_builder::compute_pass_builder::ComputePass::
write_rg_storage_buffer_concurrent(void)

{
  char *pcVar1;
  undefined8 *puVar2;
  undefined4 uVar3;
  longlong lVar4;
  uint uVar5;
  uint uVar6;
  void *in_RCX;
  longlong *in_RDX;
  ulonglong in_R8;
  longlong lVar7;
  uint uVar8;
  ulonglong uVar9;
  ushort uVar10;
  ulonglong uVar11;
  longlong lVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  undefined8 local_a0;
  undefined8 uStack_98;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined8 local_80;
  undefined8 uStack_78;
  undefined8 local_70;
  undefined8 uStack_68;
  undefined4 local_60;
  
  lVar12 = in_RDX[0x27];
  if (*(longlong *)(lVar12 + 0xb0) != 0) {
    lVar4 = *(longlong *)(lVar12 + 0x98);
    bVar13 = (byte)(in_R8 >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    lVar7 = 0;
    uVar9 = in_R8;
    while( true ) {
      uVar9 = *(ulonglong *)(lVar12 + 0xa0) & uVar9;
      pcVar1 = (char *)(lVar4 + uVar9);
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
      uVar10 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
      uVar8 = (uint)uVar10;
      while (uVar10 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar8; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        uVar11 = uVar5 + uVar9 & *(ulonglong *)(lVar12 + 0xa0);
        if (*(ulonglong *)(lVar4 + -0x10 + uVar11 * -0x10) == in_R8) {
          lVar12 = uVar11 * -0x10;
          uVar3 = *(undefined4 *)(lVar4 + -4 + lVar12);
          if (*(int *)(lVar4 + -8 + lVar12) != 0) {
                    // WARNING: Subroutine does not return
            core::panicking::
            assert_failed<shader_pipeline::reflection::SetIndex,shader_pipeline::reflection::SetIndex>
                      ();
          }
          bevy_app::plugin::Plugin::finish<country_core::systems::main_camera::MainCameraPlugin>();
          rhapsody::rg::pass_builder::PassBuilder::
          add_resource_access<country_core::render::pass_builder::pass_resources::PassResources>();
          uVar9 = in_RDX[2];
          if (0xff < uVar9) {
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          rhapsody::rg::pass_api::impl$5::bind();
          lVar12 = in_RDX[2];
          if (lVar12 == *in_RDX) {
            alloc::raw_vec::RawVec::
            grow_one<enum2_<rhapsody::rg::pass_api::RenderPassBinding>,alloc::alloc::Global>();
          }
          lVar4 = in_RDX[1];
          lVar7 = lVar12 * 0x44;
          *(undefined4 *)(lVar4 + 0x40 + lVar7) = local_60;
          puVar2 = (undefined8 *)(lVar4 + 0x30 + lVar7);
          *puVar2 = local_70;
          puVar2[1] = uStack_68;
          puVar2 = (undefined8 *)(lVar4 + 0x20 + lVar7);
          *puVar2 = local_80;
          puVar2[1] = uStack_78;
          puVar2 = (undefined8 *)(lVar4 + 0x10 + lVar7);
          *puVar2 = local_90;
          puVar2[1] = uStack_88;
          *(undefined8 *)(lVar4 + lVar7) = local_a0;
          ((undefined8 *)(lVar4 + lVar7))[1] = uStack_98;
          in_RDX[2] = lVar12 + 1;
          lVar12 = in_RDX[5];
          if (lVar12 == in_RDX[3]) {
            alloc::raw_vec::RawVec::
            grow_one<core::ops::range::Range<rhapsody::rg::resource_info::ResourceAccessIndex>,alloc::alloc::Global>
                      ();
          }
          lVar4 = in_RDX[4];
          *(undefined4 *)(lVar4 + lVar12 * 8) = uVar3;
          *(char *)(lVar4 + 4 + lVar12 * 8) = (char)uVar9;
          in_RDX[5] = lVar12 + 1;
          goto LAB_14096e1e5;
        }
        uVar10 = (ushort)(uVar8 - 1) & (ushort)uVar8;
        uVar8 = CONCAT22((short)(uVar8 - 1 >> 0x10),uVar10);
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
      uVar9 = uVar9 + lVar7 + 0x10;
      lVar7 = lVar7 + 0x10;
    }
  }
LAB_14096e1e5:
  memcpy(in_RCX,in_RDX,400);
  return;
}


