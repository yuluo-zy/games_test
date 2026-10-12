
/* WARNING: Possible PIC construction at 0x0001418c13d3: Changing call to branch */
/* WARNING: Removing unreachable block (ram,0x0001418c13d8) */

void FUN_1418c1380(longlong *param_1,undefined8 param_2,longlong param_3)

{
  ulonglong *puVar1;
  ulonglong uVar2;
  code *pcVar3;
  ulonglong uVar4;
  longlong lVar5;
  ulonglong uVar6;
  
  LOCK();
  puVar1 = (ulonglong *)(param_1 + 2);
  uVar4 = *puVar1;
  *puVar1 = *puVar1 + param_3;
  UNLOCK();
  uVar2 = param_1[1];
  if (uVar2 == 0) goto code_r0x0001418c1422;
  if ((uVar4 | uVar2) >> 0x20 == 0) {
    uVar4 = (uVar4 & 0xffffffff) % (uVar2 & 0xffffffff);
    uVar6 = (uVar4 + param_3) - uVar2;
    if (uVar4 + param_3 < uVar2 || uVar6 == 0) goto code_r0x0001418c13f3;
  }
  else {
    uVar4 = uVar4 % uVar2;
    uVar6 = (uVar4 + param_3) - uVar2;
    if (uVar4 + param_3 < uVar2 || uVar6 == 0) {
code_r0x0001418c13f3:
      lVar5 = uVar4 + *param_1;
      goto code_r0x000142923c00;
    }
  }
  if (uVar2 < uVar6) {
    FUN_1428d9430(&UNK_142d06168,0x2a,&UNK_142d06198);
code_r0x0001418c1422:
    FUN_1428d9a20(&UNK_142d06150);
    pcVar3 = (code *)swi(3);
    (*pcVar3)();
    return;
  }
  param_3 = uVar2 - uVar4;
  lVar5 = uVar4 + *param_1;
code_r0x000142923c00:
                    /* WARNING: Could not recover jumptable at 0x000142923c00. Too many branches */
                    /* WARNING: Treating indirect jump as call */
  memcpy(lVar5,param_2,param_3);
  return;
}

