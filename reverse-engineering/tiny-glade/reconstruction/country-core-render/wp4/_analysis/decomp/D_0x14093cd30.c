// ADDRESS: 0x14093cd30
// SIZE: 240
// SIGNATURE: void __cdecl import_texture_presets_to_rg(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::import_texture_presets_to_rg
          (void)

{
  undefined1 auVar1 [16];
  ushort uVar2;
  uint uVar3;
  uint uVar4;
  uint uVar5;
  longlong in_RCX;
  longlong lVar6;
  longlong lVar7;
  undefined1 (*pauVar8) [16];
  undefined1 (*pauVar9) [16];
  undefined8 local_5c;
  undefined8 uStack_54;
  undefined8 local_4c;
  undefined8 uStack_44;
  undefined4 local_3c;
  
  lVar6 = *(longlong *)(in_RCX + 0x68);
  if (lVar6 != 0) {
    pauVar8 = *(undefined1 (**) [16])(in_RCX + 0x50);
    pauVar9 = pauVar8 + 1;
    auVar1 = *pauVar8;
    uVar4 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
                            (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                            (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                            (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                            (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                            (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                            (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                            (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                            (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                            (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                            (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                            (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                            (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                            (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                            (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                           (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
    do {
      if ((short)uVar4 == 0) {
        do {
          auVar1 = *pauVar9;
          uVar2 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
                  (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 |
                  (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
                  (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 |
                  (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
                  (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 |
                  (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
                  (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 |
                  (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
                  (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
                  (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
                  (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
                  (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
                  (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe |
                  (ushort)(byte)(auVar1[0xf] >> 7) << 0xf;
          pauVar8 = pauVar8 + -0x68;
          pauVar9 = pauVar9 + 1;
        } while (uVar2 == 0xffff);
        uVar5 = ~(uint)uVar2;
        uVar4 = uVar5 & -(uint)uVar2 - 2;
      }
      else {
        uVar5 = uVar4;
        uVar4 = uVar4 - 1 & uVar4;
      }
      uVar3 = 0;
      for (; (uVar5 & 1) == 0; uVar5 = uVar5 >> 1 | 0x80000000) {
        uVar3 = uVar3 + 1;
      }
      lVar7 = (ulonglong)uVar3 * -0x68;
      if (*(char *)((longlong)pauVar8 + lVar7 + -6) != '\a') {
                    // WARNING: Subroutine does not return
        core::panicking::panic();
      }
      rhapsody::rg::import_export::impl$0::import_unowned();
      *(undefined4 *)((longlong)pauVar8 + lVar7 + -8) = local_3c;
      *(undefined8 *)((longlong)pauVar8 + lVar7 + -0x18) = local_4c;
      *(undefined8 *)((longlong)pauVar8 + lVar7 + -0x10) = uStack_44;
      *(undefined8 *)((longlong)pauVar8 + lVar7 + -0x28) = local_5c;
      *(undefined8 *)((longlong)pauVar8 + lVar7 + -0x20) = uStack_54;
      lVar6 = lVar6 + -1;
    } while (lVar6 != 0);
  }
  return;
}

