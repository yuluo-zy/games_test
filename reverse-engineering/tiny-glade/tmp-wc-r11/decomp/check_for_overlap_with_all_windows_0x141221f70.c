// name: check_for_overlap_with_all_windows
// addr: 0x141221f70
// size: 617
// signature: void __cdecl check_for_overlap_with_all_windows(void)


void __cdecl system_wall_constructor::utils::half_timber::check_for_overlap_with_all_windows(void)

{
  undefined1 auVar1 [16];
  ushort uVar2;
  uint uVar3;
  char extraout_AL;
  char extraout_AL_00;
  longlong extraout_RAX;
  uint uVar4;
  uint uVar5;
  undefined1 (*pauVar6) [16];
  undefined1 (*pauVar7) [16];
  longlong lVar8;
  float extraout_XMM0_Da;
  float extraout_XMM0_Da_00;
  float extraout_XMM0_Da_01;
  float extraout_XMM0_Da_02;
  float in_XMM3_Da;
  
  country_core::resources::walls::decorator_storage::DecoratorStorage::get();
  if ((extraout_RAX != 0) && (lVar8 = *(longlong *)(extraout_RAX + 0x48), lVar8 != 0)) {
    pauVar6 = *(undefined1 (**) [16])(extraout_RAX + 0x30);
    pauVar7 = pauVar6 + 1;
    auVar1 = *pauVar6;
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
          auVar1 = *pauVar7;
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
          pauVar6 = pauVar6 + -0x50;
          pauVar7 = pauVar7 + 1;
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
      if (((byte)pauVar6[(ulonglong)uVar3 * -5 + -3][10] < 2) &&
         ((byte)(pauVar6[(ulonglong)uVar3 * -5 + -1][0xe] - 0x38) < 3)) {
        country_core::resources::walls::decorator_subtype::DerivedDecoratorInfo::subtype();
        get_window_aabb();
        ::utils::geometry::aabb::Aabb2::contains();
        if (extraout_AL_00 != '\0') {
          return;
        }
        ::utils::geometry::aabb::Aabb2::center();
        ::utils::geometry::aabb::Aabb2::width();
        if (in_XMM3_Da < extraout_XMM0_Da_00 * 0.5 + extraout_XMM0_Da) {
          ::utils::geometry::aabb::Aabb2::center();
        }
        else {
          ::utils::geometry::aabb::Aabb2::center();
          ::utils::geometry::aabb::Aabb2::width();
          if (extraout_XMM0_Da_02 * 0.5 <= extraout_XMM0_Da_01) goto LAB_141222080;
          ::utils::geometry::aabb::Aabb2::center();
        }
        ::utils::geometry::aabb::Aabb2::center();
        ::utils::geometry::aabb::Aabb2::width();
        ::utils::geometry::aabb::Aabb2::height();
        ::utils::geometry::aabb::Aabb2::from_center_dims();
        ::utils::geometry::aabb::Aabb2::contains();
        if (extraout_AL != '\0') {
          return;
        }
      }
LAB_141222080:
      lVar8 = lVar8 + -1;
    } while (lVar8 != 0);
  }
  return;
}

