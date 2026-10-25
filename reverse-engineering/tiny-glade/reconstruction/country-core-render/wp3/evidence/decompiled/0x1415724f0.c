// address 0x1415724f0
// name copy<country_core::render::nani::instance_data::LanternEmissiveInstanceData>
// signature void __cdecl copy<country_core::render::nani::instance_data::LanternEmissiveInstanceData>(void)

void __cdecl
country_core::render::nani::gpu_world::impl$14::
copy<country_core::render::nani::instance_data::LanternEmissiveInstanceData>(void)

{
  undefined8 *puVar1;
  undefined8 *puVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  ulonglong uVar11;
  ulonglong uVar12;
  longlong in_RCX;
  ulonglong in_RDX;
  ulonglong in_R8;
  longlong lVar13;
  ulonglong uVar14;
  longlong lVar15;
  bool bVar16;
  
  if (*(ulonglong *)(in_RCX + 0x30) <= in_R8) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (*(ulonglong *)(in_RCX + 0x30) <= in_RDX) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar3 = *(longlong *)(in_RCX + 0x28);
  lVar13 = in_R8 * 0x40;
  lVar15 = in_RDX * 0x40;
  uVar4 = *(undefined8 *)(lVar3 + lVar13);
  uVar5 = ((undefined8 *)(lVar3 + lVar13))[1];
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar13);
  uVar6 = *puVar1;
  uVar7 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar13);
  uVar8 = *puVar1;
  uVar9 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x30 + lVar13);
  uVar10 = puVar1[1];
  puVar2 = (undefined8 *)(lVar3 + 0x30 + lVar15);
  *puVar2 = *puVar1;
  puVar2[1] = uVar10;
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar15);
  *puVar1 = uVar8;
  puVar1[1] = uVar9;
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar15);
  *puVar1 = uVar6;
  puVar1[1] = uVar7;
  *(undefined8 *)(lVar3 + lVar15) = uVar4;
  ((undefined8 *)(lVar3 + lVar15))[1] = uVar5;
  uVar11 = *(ulonglong *)(in_RCX + 0x18);
  do {
    uVar14 = in_RDX;
    if (uVar11 <= in_RDX) {
      uVar14 = uVar11;
    }
    LOCK();
    uVar12 = *(ulonglong *)(in_RCX + 0x18);
    bVar16 = uVar11 == uVar12;
    if (bVar16) {
      *(ulonglong *)(in_RCX + 0x18) = uVar14;
      uVar12 = uVar11;
    }
    UNLOCK();
    uVar11 = uVar12;
  } while (!bVar16);
  uVar11 = *(ulonglong *)(in_RCX + 0x20);
  do {
    uVar14 = in_RDX;
    if (in_RDX < uVar11) {
      uVar14 = uVar11;
    }
    LOCK();
    uVar12 = *(ulonglong *)(in_RCX + 0x20);
    bVar16 = uVar11 == uVar12;
    if (bVar16) {
      *(ulonglong *)(in_RCX + 0x20) = uVar14;
      uVar12 = uVar11;
    }
    UNLOCK();
    uVar11 = uVar12;
  } while (!bVar16);
  return;
}

