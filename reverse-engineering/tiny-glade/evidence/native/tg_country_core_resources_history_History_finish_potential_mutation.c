
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140a86980(longlong param_1,char param_2)

{
  longlong lVar1;
  longlong *plVar2;
  char cVar3;
  undefined8 uVar4;
  longlong lVar5;
  undefined8 *puVar6;
  undefined *puStack_78;
  undefined8 uStack_70;
  ulonglong uStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  longlong lStack_48;
  undefined8 *puStack_40;
  longlong lStack_38;
  byte bStack_29;
  undefined8 uStack_28;
  
  uStack_28 = 0xfffffffffffffffe;
  bStack_29 = 0;
  if (param_2 != '\0') {
    puVar6 = *(undefined8 **)(param_1 + 0x20);
    lVar1 = *(longlong *)(param_1 + 0x28);
    *(undefined8 *)(param_1 + 0x28) = 0;
    puStack_40 = puVar6;
    lStack_38 = lVar1;
    if (lVar1 != 0) {
      lVar5 = 1;
      do {
        plVar2 = (longlong *)*puVar6;
        LOCK();
        *plVar2 = *plVar2 + -1;
        UNLOCK();
        lStack_48 = lVar5;
        if (*plVar2 == 0) {
          func_0x0001402e5d80(puVar6);
        }
        lVar5 = lStack_48 + 1;
        puVar6 = puVar6 + 2;
      } while (lStack_48 != lVar1);
    }
    puStack_78 = (undefined *)0x0;
    uStack_70 = 0xffffffffffffffff;
    uStack_68 = uStack_68 & 0xffffffffffffff00;
    uVar4 = FUN_14132ade0(&UNK_142ad7918,&puStack_78);
    *(undefined8 *)(param_1 + 0x60) = uVar4;
    if ((bStack_29 & 1) != 0) {
      if ((*_DAT_14362c420 & 0x7fffffffffffffff) != 0) goto LAB_140a86a8a;
      do {
        puStack_78 = &UNK_142a98ad8;
        uStack_70 = 1;
        uStack_68 = 8;
        uStack_60 = 0;
        uStack_58 = 0;
        FUN_1428d9390(&puStack_78,&UNK_142a98b18);
LAB_140a86a8a:
        cVar3 = FUN_1428d86b0();
      } while (cVar3 != '\0');
    }
  }
  return;
}

