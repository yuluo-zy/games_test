// ADDRESS: 0x14093bee0
// SIZE: 630
// SIGNATURE: void __cdecl clear_preset_texture(void)

void __cdecl
country_core::asset_libraries::texture_library::AssetTextureLibrary::clear_preset_texture(void)

{
  char *pcVar1;
  int iVar2;
  int iVar3;
  int iVar4;
  uint uVar5;
  uint uVar6;
  ulonglong extraout_RAX;
  longlong extraout_RAX_00;
  longlong in_RCX;
  longlong lVar7;
  ulonglong uVar8;
  char in_R8B;
  longlong lVar9;
  uint uVar10;
  ushort uVar11;
  longlong lVar12;
  byte bVar13;
  char cVar14;
  char cVar16;
  char cVar17;
  char cVar18;
  undefined1 auVar15 [16];
  undefined1 auVar19 [16];
  undefined1 auVar20 [16];
  int local_138;
  ulonglong local_e8;
  
  if (*(longlong *)(in_RCX + 0x68) != 0) {
    core::hash::BuildHasher::
    hash_one<std::hash::random::RandomState,ref_<country_core::systems::save::save_game::SaveFileSection>_>
              ();
    lVar7 = *(longlong *)(in_RCX + 0x50);
    bVar13 = (byte)(extraout_RAX >> 0x39);
    auVar15 = ZEXT216(CONCAT11(bVar13,bVar13));
    auVar15 = pshuflw(auVar15,auVar15,0);
    lVar9 = 0;
    uVar8 = extraout_RAX;
    while( true ) {
      uVar8 = *(ulonglong *)(in_RCX + 0x58) & uVar8;
      pcVar1 = (char *)(lVar7 + uVar8);
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
      uVar11 = (ushort)(SUB161(auVar20 >> 7,0) & 1) | (ushort)(SUB161(auVar20 >> 0xf,0) & 1) << 1 |
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
      uVar10 = (uint)uVar11;
      while (uVar11 != 0) {
        uVar5 = 0;
        for (uVar6 = uVar10; (uVar6 & 1) == 0; uVar6 = uVar6 >> 1 | 0x80000000) {
          uVar5 = uVar5 + 1;
        }
        lVar12 = (uVar5 + uVar8 & *(ulonglong *)(in_RCX + 0x58)) * -0x68;
        if (*(char *)(lVar7 + -0x68 + lVar12) == in_R8B) {
          iVar2 = *(int *)(lVar7 + -0x3c + lVar12);
          iVar3 = *(int *)(lVar7 + -0x38 + lVar12);
          iVar4 = *(int *)(lVar7 + -0x34 + lVar12);
          rhapsody::raw::format::vk_format_info();
          lVar7 = (local_e8 >> 0x20) * (ulonglong)(uint)(iVar3 * iVar2 * iVar4);
          if ((-1 < lVar7) &&
             ((lVar7 == 0 ||
              (std::alloc::__default_lib_allocator::__rust_alloc_zeroed(), extraout_RAX_00 != 0))))
          {
            rhapsody::raw::device::GraphicsDevice::update_texture();
            if (local_138 == 0xb) {
              if (lVar7 != 0) {
                std::alloc::__default_lib_allocator::__rust_dealloc();
              }
              return;
            }
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
                    // WARNING: Subroutine does not return
          alloc::raw_vec::handle_error();
        }
        uVar11 = (ushort)(uVar10 - 1) & (ushort)uVar10;
        uVar10 = CONCAT22((short)(uVar10 - 1 >> 0x10),uVar11);
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
      uVar8 = uVar8 + lVar9 + 0x10;
      lVar9 = lVar9 + 0x10;
    }
  }
                    // WARNING: Subroutine does not return
  core::option::expect_failed();
}

