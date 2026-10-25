// address 0x141572760
// name copy<country_core::render::nani::instance_data::VelocityOnlyInstanceData>
// signature void __cdecl copy<country_core::render::nani::instance_data::VelocityOnlyInstanceData>(void)

void __cdecl
country_core::render::nani::gpu_world::impl$14::
copy<country_core::render::nani::instance_data::VelocityOnlyInstanceData>(void)

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
  undefined8 uVar11;
  undefined8 uVar12;
  undefined8 uVar13;
  undefined8 uVar14;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong in_RCX;
  ulonglong in_RDX;
  ulonglong in_R8;
  longlong lVar17;
  ulonglong uVar18;
  longlong lVar19;
  bool bVar20;
  
  if (*(ulonglong *)(in_RCX + 0x30) <= in_R8) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  if (*(ulonglong *)(in_RCX + 0x30) <= in_RDX) {
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar3 = *(longlong *)(in_RCX + 0x28);
  lVar17 = in_R8 * 0x60;
  lVar19 = in_RDX * 0x60;
  uVar4 = *(undefined8 *)(lVar3 + lVar17);
  uVar5 = ((undefined8 *)(lVar3 + lVar17))[1];
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar17);
  uVar6 = *puVar1;
  uVar7 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar17);
  uVar8 = *puVar1;
  uVar9 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x30 + lVar17);
  uVar10 = *puVar1;
  uVar11 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x40 + lVar17);
  uVar12 = *puVar1;
  uVar13 = puVar1[1];
  puVar1 = (undefined8 *)(lVar3 + 0x50 + lVar17);
  uVar14 = puVar1[1];
  puVar2 = (undefined8 *)(lVar3 + 0x50 + lVar19);
  *puVar2 = *puVar1;
  puVar2[1] = uVar14;
  puVar1 = (undefined8 *)(lVar3 + 0x40 + lVar19);
  *puVar1 = uVar12;
  puVar1[1] = uVar13;
  puVar1 = (undefined8 *)(lVar3 + 0x30 + lVar19);
  *puVar1 = uVar10;
  puVar1[1] = uVar11;
  puVar1 = (undefined8 *)(lVar3 + 0x20 + lVar19);
  *puVar1 = uVar8;
  puVar1[1] = uVar9;
  puVar1 = (undefined8 *)(lVar3 + 0x10 + lVar19);
  *puVar1 = uVar6;
  puVar1[1] = uVar7;
  *(undefined8 *)(lVar3 + lVar19) = uVar4;
  ((undefined8 *)(lVar3 + lVar19))[1] = uVar5;
  uVar15 = *(ulonglong *)(in_RCX + 0x18);
  do {
    uVar18 = in_RDX;
    if (uVar15 <= in_RDX) {
      uVar18 = uVar15;
    }
    LOCK();
    uVar16 = *(ulonglong *)(in_RCX + 0x18);
    bVar20 = uVar15 == uVar16;
    if (bVar20) {
      *(ulonglong *)(in_RCX + 0x18) = uVar18;
      uVar16 = uVar15;
    }
    UNLOCK();
    uVar15 = uVar16;
  } while (!bVar20);
  uVar15 = *(ulonglong *)(in_RCX + 0x20);
  do {
    uVar18 = in_RDX;
    if (in_RDX < uVar15) {
      uVar18 = uVar15;
    }
    LOCK();
    uVar16 = *(ulonglong *)(in_RCX + 0x20);
    bVar20 = uVar15 == uVar16;
    if (bVar20) {
      *(ulonglong *)(in_RCX + 0x20) = uVar18;
      uVar16 = uVar15;
    }
    UNLOCK();
    uVar15 = uVar16;
  } while (!bVar20);
  return;
}

