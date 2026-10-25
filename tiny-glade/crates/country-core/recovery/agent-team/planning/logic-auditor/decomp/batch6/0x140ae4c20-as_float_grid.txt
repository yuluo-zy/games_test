
void __cdecl country_core::systems::paths::draw_path::PathRastersResources::as_float_grid(void)

{
  undefined8 *puVar1;
  longlong lVar2;
  longlong lVar3;
  undefined8 uVar4;
  undefined8 uVar5;
  undefined8 uVar6;
  undefined8 uVar7;
  undefined8 uVar8;
  undefined8 uVar9;
  undefined8 uVar10;
  undefined8 *in_RCX;
  undefined8 *puVar11;
  longlong *in_RDX;
  undefined8 *puVar12;
  undefined8 *puVar13;
  
  lVar2 = *in_RDX;
  lVar3 = in_RDX[4];
  puVar1 = (undefined8 *)(lVar2 + 0x20);
  puVar11 = (undefined8 *)(lVar2 + 0x28);
  if (*(char *)(lVar2 + 0x298) == '\0') {
    puVar11 = (undefined8 *)(lVar2 + 0x10);
    puVar1 = (undefined8 *)(lVar2 + 8);
  }
  uVar4 = *puVar11;
  puVar11 = (undefined8 *)(lVar3 + 0x20);
  puVar12 = (undefined8 *)(lVar3 + 0x28);
  if (*(char *)(lVar3 + 0x298) == '\0') {
    puVar12 = (undefined8 *)(lVar3 + 0x10);
    puVar11 = (undefined8 *)(lVar3 + 8);
  }
  uVar5 = *puVar12;
  uVar6 = *puVar11;
  lVar2 = in_RDX[8];
  puVar11 = (undefined8 *)(lVar2 + 0x20);
  puVar12 = (undefined8 *)(lVar2 + 0x28);
  if (*(char *)(lVar2 + 0x298) == '\0') {
    puVar12 = (undefined8 *)(lVar2 + 0x10);
    puVar11 = (undefined8 *)(lVar2 + 8);
  }
  uVar7 = *puVar12;
  lVar2 = in_RDX[0xc];
  puVar12 = (undefined8 *)(lVar2 + 0x20);
  puVar13 = (undefined8 *)(lVar2 + 0x28);
  if (*(char *)(lVar2 + 0x298) == '\0') {
    puVar13 = (undefined8 *)(lVar2 + 0x10);
    puVar12 = (undefined8 *)(lVar2 + 8);
  }
  uVar8 = *puVar11;
  uVar9 = *puVar13;
  uVar10 = *puVar12;
  *in_RCX = *puVar1;
  in_RCX[1] = uVar4;
  in_RCX[2] = uVar6;
  in_RCX[3] = uVar5;
  in_RCX[4] = uVar8;
  in_RCX[5] = uVar7;
  in_RCX[6] = uVar10;
  in_RCX[7] = uVar9;
  return;
}

