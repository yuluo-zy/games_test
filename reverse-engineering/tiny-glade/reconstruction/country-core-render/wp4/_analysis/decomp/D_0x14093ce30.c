// ADDRESS: 0x14093ce30
// SIZE: 241
// SIGNATURE: void __cdecl export_preset_textures_from_rg(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::export_preset_textures_from_rg
          (void)

{
  undefined1 auVar1 [16];
  char cVar2;
  ushort uVar3;
  uint uVar4;
  uint uVar5;
  uint uVar6;
  longlong in_RCX;
  longlong lVar7;
  undefined1 (*pauVar8) [16];
  undefined1 (*pauVar9) [16];
  
  lVar7 = *(longlong *)(in_RCX + 0x68);
  if (lVar7 != 0) {
    pauVar8 = *(undefined1 (**) [16])(in_RCX + 0x50);
    pauVar9 = pauVar8 + 1;
    auVar1 = *pauVar8;
    uVar5 = ~(uint)(ushort)((ushort)(SUB161(auVar1 >> 7,0) & 1) |
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
      if ((short)uVar5 == 0) {
        do {
          auVar1 = *pauVar9;
          uVar3 = (ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
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
        } while (uVar3 == 0xffff);
        uVar6 = ~(uint)uVar3;
        uVar5 = uVar6 & -(uint)uVar3 - 2;
      }
      else {
        uVar6 = uVar5;
        uVar5 = uVar5 - 1 & uVar5;
      }
      uVar4 = 0;
      for (; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
        uVar4 = uVar4 + 1;
      }
      cVar2 = *(char *)((longlong)pauVar8 + (ulonglong)uVar4 * -0x68 + -6);
      *(undefined1 *)((longlong)pauVar8 + (ulonglong)uVar4 * -0x68 + -6) = 7;
      if (cVar2 == '\a') {
                    // WARNING: Subroutine does not return
        core::option::unwrap_failed();
      }
      rhapsody::rg::import_export::impl$0::export();
      lVar7 = lVar7 + -1;
    } while (lVar7 != 0);
  }
  return;
}

