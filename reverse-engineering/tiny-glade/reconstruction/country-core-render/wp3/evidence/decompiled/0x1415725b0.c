// address 0x1415725b0
// name copy<country_core::render::nani::instance_data::PlantInstanceData>
// signature void __cdecl copy<country_core::render::nani::instance_data::PlantInstanceData>(void)

void __cdecl
country_core::render::nani::gpu_world::impl$14::
copy<country_core::render::nani::instance_data::PlantInstanceData>(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  ulonglong uVar9;
  ulonglong uVar10;
  longlong in_RCX;
  ulonglong in_RDX;
  ulonglong in_R8;
  longlong lVar11;
  ulonglong uVar12;
  longlong lVar13;
  bool bVar14;
  
  if (*(ulonglong *)(in_RCX + 0x30) <= in_R8) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (*(ulonglong *)(in_RCX + 0x30) <= in_RDX) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar3 = *(longlong *)(in_RCX + 0x28);
  lVar11 = in_R8 * 0x30;
  lVar13 = in_RDX * 0x30;
  uVar4 = *(undefined8 *)(lVar3 + lVar11);
  uVar5 = ((undefined8 *)(lVar3 + lVar11))[1];
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar11);
  uVar6 = *puVar1;
  uVar7 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar11);
  uVar8 = puVar1[1];
  puVar2 = (undefined8 *)(lVar3 + 0x20 + lVar13);
  *puVar2 = *puVar1;
  puVar2[1] = uVar8;
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar13);
  *puVar1 = uVar6;
  puVar1[1] = uVar7;
  *(undefined8 *)(lVar3 + lVar13) = uVar4;
  ((undefined8 *)(lVar3 + lVar13))[1] = uVar5;
  uVar9 = *(ulonglong *)(in_RCX + 0x18);
  do {
    uVar12 = in_RDX;
    if (uVar9 <= in_RDX) {
      uVar12 = uVar9;
    }
    LOCK();
    uVar10 = *(ulonglong *)(in_RCX + 0x18);
    bVar14 = uVar9 == uVar10;
    if (bVar14) {
      *(ulonglong *)(in_RCX + 0x18) = uVar12;
      uVar10 = uVar9;
    }
    UNLOCK();
    uVar9 = uVar10;
  } while (!bVar14);
  uVar9 = *(ulonglong *)(in_RCX + 0x20);
  do {
    uVar12 = in_RDX;
    if (in_RDX < uVar9) {
      uVar12 = uVar9;
    }
    LOCK();
    uVar10 = *(ulonglong *)(in_RCX + 0x20);
    bVar14 = uVar9 == uVar10;
    if (bVar14) {
      *(ulonglong *)(in_RCX + 0x20) = uVar12;
      uVar10 = uVar9;
    }
    UNLOCK();
    uVar9 = uVar10;
  } while (!bVar14);
  return;
}

