// ADDR 0x14081c060
// SIG void __cdecl decorator_on_circle_edited(void)
// ORIG system::decorator_on_circle_edited


void __cdecl system_decorator::decorator_on_freehand_wall_edited::decorator_on_circle_edited(void)

{
  undefined4 uVar1;
  ulonglong *puVar2;
  ulonglong uVar3;
  undefined4 *puVar4;
  longlong extraout_RAX;
  longlong extraout_RAX_00;
  longlong lVar5;
  longlong extraout_RAX_01;
  longlong extraout_RAX_02;
  undefined8 *in_RDX;
  longlong lVar6;
  longlong lVar7;
  longlong lVar8;
  longlong lVar9;
  longlong in_R8;
  ulonglong uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  undefined8 local_b4;
  undefined4 local_ac;
  longlong local_58;
  
  puVar2 = (ulonglong *)*in_RDX;
  lVar7 = in_RDX[1];
  uVar3 = *puVar2;
  uVar12 = 0;
  uVar11 = uVar3 - *(ulonglong *)(lVar7 + 0x18);
  if (uVar3 < *(ulonglong *)(lVar7 + 0x18)) {
    uVar11 = uVar12;
  }
  uVar10 = uVar3 - *(ulonglong *)(lVar7 + 0x38);
  if (uVar3 < *(ulonglong *)(lVar7 + 0x38)) {
    uVar10 = uVar12;
  }
  lVar6 = uVar11 * 0x10 + *(longlong *)(lVar7 + 8);
  uVar3 = *(ulonglong *)(lVar7 + 0x10) - uVar11;
  if (*(ulonglong *)(lVar7 + 0x10) < uVar11) {
    lVar6 = 8;
    uVar3 = uVar12;
  }
  lVar8 = uVar10 * 0x10 + *(longlong *)(lVar7 + 0x28);
  uVar11 = *(ulonglong *)(lVar7 + 0x30) - uVar10;
  if (*(ulonglong *)(lVar7 + 0x30) < uVar10) {
    lVar8 = 8;
    uVar11 = uVar12;
  }
  *puVar2 = (*(longlong *)(lVar7 + 0x40) - uVar11) - uVar3;
  lVar9 = uVar3 * 0x10 + lVar6;
  lVar7 = uVar11 * 0x10 + lVar8;
  uVar1 = *(undefined4 *)(in_R8 + 0x1c);
  puVar4 = *(undefined4 **)(in_R8 + 0x10);
  do {
    if ((lVar6 == 0) || (lVar6 == lVar9)) {
      if ((lVar8 == 0) || (lVar8 == lVar7)) {
        return;
      }
      lVar8 = lVar8 + 0x10;
      lVar6 = 0;
    }
    else {
      lVar6 = lVar6 + 0x10;
    }
    *puVar2 = *puVar2 + 1;
    *puVar4 = uVar1;
    country_core::resources::walls::decorator_storage::DecoratorStorage::wall_decorators_mut();
    if (extraout_RAX != 0) {
      while (country_core::resources::walls::decorator_storage::impl$3::next(), local_58 != 0) {
        minidump_unwind::instruction_seems_valid_by_symbols::async_fn$0::impl$0::get_instruction();
        lVar5 = (ulonglong)*(byte *)(extraout_RAX_00 + 0x22) - 1;
        if (2 < (byte)(*(byte *)(extraout_RAX_00 + 0x22) - 2)) {
          lVar5 = 0;
        }
        if (lVar5 == 0) {
          country_core::resources::walls::public_walls::PublicWalls::get();
          if (extraout_RAX_01 == 0) {
                    // WARNING: Subroutine does not return
            core::option::unwrap_failed();
          }
          ::utils::wall_space::WallSpaceCoord::to_world_space();
          country_core::resources::walls::decorator_storage::DecoratorMaybeMut::get_mut();
          country_core::resources::walls::decorator::DecoratorInfo::dst_as_wall_mut();
          *(undefined8 *)(extraout_RAX_02 + 0x14) = local_b4;
          *(undefined4 *)(extraout_RAX_02 + 0x1c) = local_ac;
        }
        else if (lVar5 != 1) {
                    // WARNING: Subroutine does not return
          core::panicking::panic();
        }
      }
    }
  } while( true );
}


