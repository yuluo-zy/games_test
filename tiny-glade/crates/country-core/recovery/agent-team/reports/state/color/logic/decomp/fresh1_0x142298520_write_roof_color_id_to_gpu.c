// name=write_roof_color_id_to_gpu
// sig=void __cdecl write_roof_color_id_to_gpu(void)
// size=999
// address=0x142298520

void __cdecl system_color::write_roof_color_id_to_gpu::write_roof_color_id_to_gpu(void)

{
  longlong lVar1;
  longlong lVar2;
  ulonglong uVar3;
  ulonglong *puVar4;
  int iVar5;
  undefined8 *extraout_RAX;
  longlong lVar6;
  longlong extraout_RAX_00;
  ulonglong uVar7;
  longlong lVar8;
  longlong lVar9;
  ulonglong *puVar10;
  ulonglong *puVar11;
  undefined8 *in_R8;
  longlong lVar12;
  undefined8 *in_R9;
  ulonglong uVar13;
  longlong lVar14;
  ulonglong uVar15;
  longlong lVar16;
  longlong lVar17;
  longlong local_e0;
  
  frozen::FrozenMap::
  get<country_core::asset_libraries::ssbo_library::SsboId,alloc::boxed::Box<country_core::asset_libraries::ssbo_library::BufferState,alloc::alloc::Global>,core::hash::BuildHasherDefault<nohash_hasher::NoHashHasher<country_core::asset_libraries::ssbo_library::SsboId>_>,country_core::asset_libraries::ssbo_library::SsboId>
            ();
  if (extraout_RAX != (undefined8 *)0x0) {
    if ((extraout_RAX[3] != 0x37) ||
       (iVar5 = memcmp(&anon_a49387b4b2f61cd3615a99b166e92ad9_3_llvm_6175655424687704485,
                       (void *)extraout_RAX[2],0x37), iVar5 != 0)) {
                    // WARNING: Subroutine does not return
      core::panicking::assert_failed<ref_<str_>,ref_<str_>_>();
    }
    lVar6 = (**(code **)(extraout_RAX[1] + 0x18))(*extraout_RAX);
    if (lVar6 != 0) {
      puVar10 = (ulonglong *)*in_R9;
      lVar2 = in_R9[1];
      uVar3 = *puVar10;
      uVar15 = 0;
      uVar7 = uVar3 - *(ulonglong *)(lVar2 + 0x18);
      if (uVar3 < *(ulonglong *)(lVar2 + 0x18)) {
        uVar7 = uVar15;
      }
      uVar13 = uVar3 - *(ulonglong *)(lVar2 + 0x38);
      if (uVar3 < *(ulonglong *)(lVar2 + 0x38)) {
        uVar13 = uVar15;
      }
      lVar9 = uVar7 * 0x40 + *(longlong *)(lVar2 + 8);
      uVar3 = *(ulonglong *)(lVar2 + 0x10) - uVar7;
      if (*(ulonglong *)(lVar2 + 0x10) < uVar7) {
        lVar9 = 8;
        uVar3 = uVar15;
      }
      lVar8 = uVar13 * 0x40 + *(longlong *)(lVar2 + 0x28);
      uVar7 = *(ulonglong *)(lVar2 + 0x30) - uVar13;
      if (*(ulonglong *)(lVar2 + 0x30) < uVar13) {
        lVar8 = 8;
        uVar7 = uVar15;
      }
      *puVar10 = (*(longlong *)(lVar2 + 0x40) - uVar7) - uVar3;
      lVar16 = uVar3 * 0x40 + lVar9;
      lVar17 = uVar7 * 0x40 + lVar8;
      puVar4 = (ulonglong *)*in_R8;
      lVar2 = in_R8[1];
      uVar3 = *puVar4;
      uVar7 = uVar3 - *(ulonglong *)(lVar2 + 0x18);
      if (uVar3 < *(ulonglong *)(lVar2 + 0x18)) {
        uVar7 = uVar15;
      }
      uVar13 = uVar3 - *(ulonglong *)(lVar2 + 0x38);
      if (uVar3 < *(ulonglong *)(lVar2 + 0x38)) {
        uVar13 = uVar15;
      }
      lVar14 = uVar7 * 0x18 + *(longlong *)(lVar2 + 8);
      uVar3 = *(ulonglong *)(lVar2 + 0x10) - uVar7;
      if (*(ulonglong *)(lVar2 + 0x10) < uVar7) {
        lVar14 = 8;
        uVar3 = uVar15;
      }
      lVar12 = uVar13 * 0x18 + *(longlong *)(lVar2 + 0x28);
      uVar7 = *(ulonglong *)(lVar2 + 0x30) - uVar13;
      if (*(ulonglong *)(lVar2 + 0x30) < uVar13) {
        lVar12 = 8;
        uVar7 = uVar15;
      }
      *puVar4 = (*(longlong *)(lVar2 + 0x40) - uVar7) - uVar3;
      lVar2 = lVar14 + uVar3 * 0x18;
      lVar1 = lVar12 + uVar7 * 0x18;
      if (puVar10 == (ulonglong *)0x0) goto LAB_142298713;
      do {
        puVar11 = puVar10;
        if ((lVar9 == 0) || (lVar9 == lVar16)) {
          if (lVar8 == 0) {
            lVar9 = 0;
            goto LAB_142298713;
          }
          lVar9 = 0;
          if (lVar8 == lVar17) goto LAB_142298713;
          lVar8 = lVar8 + 0x40;
          lVar9 = 0;
        }
        else {
          lVar9 = lVar9 + 0x40;
        }
        while( true ) {
          *puVar11 = *puVar11 + 1;
          country_core::render::impl$0::deref();
          minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction()
          ;
          if ((*(ulonglong *)(lVar6 + 0x18) < extraout_RAX_00 * 4 + 4U) ||
             (rhapsody::raw::device::GraphicsDevice::update_buffer_bytes(), local_e0 != 0xb)) {
                    // WARNING: Subroutine does not return
            core::result::unwrap_failed();
          }
          if (puVar10 != (ulonglong *)0x0) break;
LAB_142298713:
          puVar11 = puVar4;
          if ((lVar14 == 0) || (lVar14 == lVar2)) {
            if ((lVar12 == 0) || (lVar12 == lVar1)) {
              return;
            }
            lVar12 = lVar12 + 0x18;
            puVar10 = (ulonglong *)0x0;
            lVar14 = 0;
          }
          else {
            lVar14 = lVar14 + 0x18;
            puVar10 = (ulonglong *)0x0;
          }
        }
      } while( true );
    }
  }
                    // WARNING: Subroutine does not return
  core::option::unwrap_failed();
}


