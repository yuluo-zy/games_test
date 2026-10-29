
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl system_decorator::decorator_visual::stairs_assemble::construct_ladder(void)

{
  longlong lVar1;
  float fVar2;
  float fVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  byte bVar6;
  undefined1 auVar7 [16];
  undefined1 auVar8 [16];
  undefined1 auVar9 [16];
  undefined1 auVar10 [16];
  undefined1 auVar11 [16];
  undefined1 auVar12 [16];
  undefined1 auVar13 [16];
  undefined1 auVar14 [16];
  undefined1 auVar15 [16];
  undefined1 auVar16 [16];
  undefined1 auVar17 [16];
  undefined1 auVar18 [16];
  bool bVar19;
  bool bVar20;
  ulonglong uVar21;
  ulonglong uVar22;
  undefined8 *in_RDX;
  longlong in_R8;
  ulonglong uVar23;
  ulonglong uVar24;
  float *in_R9;
  float fVar25;
  float fVar26;
  float fVar27;
  float fVar28;
  undefined1 auVar29 [16];
  longlong in_stack_00000028;
  byte *in_stack_00000030;
  ulonglong *in_stack_00000040;
  
  if (in_stack_00000028 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (in_R9 + in_stack_00000028 + -1 == (float *)0x0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  if (in_R8 == 0) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  lVar1 = (longlong)in_RDX + in_R8 * 0xc;
  if (lVar1 == 0xc) {
                    // WARNING: Subroutine does not return
    core::option::unwrap_failed();
  }
  fVar26 = *in_R9;
  fVar2 = in_R9[in_stack_00000028 + -1];
  uVar4 = *in_RDX;
  fVar28 = *(float *)(in_RDX + 1);
  uVar5 = *(undefined8 *)(lVar1 + -0xc);
  fVar3 = *(float *)(lVar1 + -4);
  ::utils::swizzlers::impl$0::x0y();
  bVar6 = *in_stack_00000030;
  bVar20 = true;
  do {
    bVar19 = bVar20;
    ::utils::swizzlers::impl$0::x0y();
    country_core::resources::render::Affine3Packed::from_cols();
    uVar23 = *in_stack_00000040;
    uVar21 = uVar23 + 0xa0761d6478bd642f;
    *in_stack_00000040 = uVar21;
    auVar7._8_8_ = 0;
    auVar7._0_8_ = uVar21 ^ 0xe7037ed1a0b428db;
    auVar13._8_8_ = 0;
    auVar13._0_8_ = uVar21;
    if ((SUB164(auVar7 * auVar13,8) ^ SUB164(auVar7 * auVar13,0)) * 200 < 0x60) {
      uVar23 = uVar23 + 0x40ec3ac8f17ac85e;
      do {
        uVar21 = uVar21 + 0xa0761d6478bd642f;
        auVar8._8_8_ = 0;
        auVar8._0_8_ = uVar23 ^ 0xe7037ed1a0b428db;
        auVar14._8_8_ = 0;
        auVar14._0_8_ = uVar23;
        uVar23 = uVar23 + 0xa0761d6478bd642f;
      } while ((SUB164(auVar8 * auVar14,8) ^ SUB164(auVar8 * auVar14,0)) * 200 < 0x60);
      *in_stack_00000040 = uVar21;
    }
    country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
    push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
    if ((bVar6 < 5) && ((0x16U >> (bVar6 & 0x1f) & 1) != 0)) {
      ::utils::swizzlers::impl$1::xvy();
      ::utils::swizzlers::impl$0::x0y();
      country_core::resources::render::Affine3Packed::from_cols();
      uVar23 = *in_stack_00000040;
      uVar21 = uVar23 + 0xa0761d6478bd642f;
      *in_stack_00000040 = uVar21;
      auVar9._8_8_ = 0;
      auVar9._0_8_ = uVar21 ^ 0xe7037ed1a0b428db;
      auVar15._8_8_ = 0;
      auVar15._0_8_ = uVar21;
      if ((SUB164(auVar9 * auVar15,8) ^ SUB164(auVar9 * auVar15,0)) * 200 < 0x60) {
        uVar23 = uVar23 + 0x40ec3ac8f17ac85e;
        do {
          uVar21 = uVar21 + 0xa0761d6478bd642f;
          auVar10._8_8_ = 0;
          auVar10._0_8_ = uVar23 ^ 0xe7037ed1a0b428db;
          auVar16._8_8_ = 0;
          auVar16._0_8_ = uVar23;
          uVar23 = uVar23 + 0xa0761d6478bd642f;
        } while ((SUB164(auVar10 * auVar16,8) ^ SUB164(auVar10 * auVar16,0)) * 200 < 0x60);
        *in_stack_00000040 = uVar21;
      }
      country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
      push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
    }
    bVar20 = false;
  } while (bVar19);
  fVar25 = (float)uVar4 - (float)uVar5;
  fVar27 = (float)((ulonglong)uVar4 >> 0x20) - (float)((ulonglong)uVar5 >> 0x20);
  fVar28 = fVar28 - fVar3;
  fVar28 = ceilf(SQRT(fVar28 * fVar28 + fVar27 * fVar27 + fVar25 * fVar25) / 0.65);
  uVar23 = 0;
  uVar21 = (longlong)(fVar28 - 9.223372e+18) & (longlong)fVar28 >> 0x3f | (longlong)fVar28;
  if (fVar28 < 0.0) {
    uVar21 = uVar23;
  }
  uVar22 = 0xffffffffffffffff;
  if (fVar28 <= 1.8446743e+19) {
    uVar22 = uVar21;
  }
  uVar21 = 3;
  if (3 < uVar22) {
    uVar21 = uVar22;
  }
  auVar29._4_4_ = fVar26 + -0.593;
  auVar29._0_4_ = fVar2 + -0.593;
  auVar29._8_8_ = 0;
  maxps(___xmm_00000000000000003ecccccd3ecccccd,auVar29);
  do {
    if ((longlong)uVar23 < 0) {
      fVar26 = (float)uVar23 / (float)(uVar21 - 1);
      if (fVar26 == 0.0) goto joined_r0x0001407dbf94;
LAB_1407dbf96:
      if ((fVar26 != 1.0) || (NAN(fVar26))) {
        country_core::resources::render::Affine3Packed::from_cols();
        uVar24 = *in_stack_00000040;
        uVar22 = uVar24 + 0xa0761d6478bd642f;
        *in_stack_00000040 = uVar22;
        auVar11._8_8_ = 0;
        auVar11._0_8_ = uVar22 ^ 0xe7037ed1a0b428db;
        auVar17._8_8_ = 0;
        auVar17._0_8_ = uVar22;
        if ((SUB164(auVar11 * auVar17,8) ^ SUB164(auVar11 * auVar17,0)) * 200 < 0x60) {
          uVar24 = uVar24 + 0x40ec3ac8f17ac85e;
          do {
            uVar22 = uVar22 + 0xa0761d6478bd642f;
            auVar12._8_8_ = 0;
            auVar12._0_8_ = uVar24 ^ 0xe7037ed1a0b428db;
            auVar18._8_8_ = 0;
            auVar18._0_8_ = uVar24;
            uVar24 = uVar24 + 0xa0761d6478bd642f;
          } while ((SUB164(auVar12 * auVar18,8) ^ SUB164(auVar12 * auVar18,0)) * 200 < 0x60);
          *in_stack_00000040 = uVar22;
        }
        country_core::render::sparse_instance_buffer::SparseInstanceBufferWriter::
        push<country_core::resources::roofs::roof_buffers::WoodenPlankSsbo>();
      }
    }
    else {
      fVar26 = (float)(longlong)uVar23 / (float)(uVar21 - 1);
      if (fVar26 != 0.0) goto LAB_1407dbf96;
joined_r0x0001407dbf94:
      if (NAN(fVar26)) goto LAB_1407dbf96;
    }
    uVar23 = uVar23 + 1;
    if (uVar23 == uVar21) {
      return;
    }
  } while( true );
}

