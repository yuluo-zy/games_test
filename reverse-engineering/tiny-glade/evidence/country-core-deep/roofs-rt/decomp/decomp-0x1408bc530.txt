
byte FUN_1408bc530(longlong *param_1)

{
  uint *puVar1;
  int *piVar2;
  uint uVar3;
  int iVar4;
  char cVar5;
  byte bVar6;
  uint uVar7;
  longlong lVar8;
  longlong lVar9;
  code *pcVar10;
  longlong lStack_40;
  uint *puStack_38;
  undefined8 uStack_30;
  
  uStack_30 = 0xfffffffffffffffe;
  lVar8 = *param_1;
  puVar1 = (uint *)(lVar8 + 0x10);
  uVar7 = *(uint *)(lVar8 + 0x10);
  if (uVar7 < 0x3ffffffe) {
    LOCK();
    uVar3 = *puVar1;
    if (uVar7 == uVar3) {
      *puVar1 = uVar7 + 1;
    }
    UNLOCK();
    if (uVar7 == uVar3) {
      cVar5 = *(char *)(lVar8 + 0x18);
      goto joined_r0x0001408bc5c0;
    }
  }
  FUN_1428d8a30(puVar1);
  cVar5 = *(char *)(lVar8 + 0x18);
joined_r0x0001408bc5c0:
  if (cVar5 != '\0') {
    lStack_40 = lVar8 + 0x20;
    puStack_38 = puVar1;
    FUN_1428d9760(&UNK_142aa03b8,0x2b,&lStack_40,&UNK_142aa03e8,&UNK_142aa09c0);
                    /* WARNING: Does not return */
    pcVar10 = (code *)invalidInstructionException();
    (*pcVar10)();
  }
  cVar5 = *(char *)(lVar8 + 0xa0);
  lVar9 = *(longlong *)(lVar8 + 0x20);
  bVar6 = *(byte *)(lVar8 + 0xa8);
  LOCK();
  piVar2 = (int *)(lVar8 + 0x10);
  iVar4 = *piVar2;
  *piVar2 = *piVar2 + -1;
  UNLOCK();
  if (SBORROW4(0,iVar4 - 1U & 0xbfffffff)) {
    FUN_1428d8d40(puVar1);
  }
  return (bVar6 ^ 1) & (SBORROW8(0,lVar9) && cVar5 == '\x02');
}

