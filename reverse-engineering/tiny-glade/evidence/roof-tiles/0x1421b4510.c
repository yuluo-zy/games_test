
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1421b4510(void)

{
  undefined8 *puVar1;
  code *pcVar2;
  undefined8 uVar3;
  char **ppcVar4;
  char cVar5;
  undefined4 *puVar6;
  char ***pppcStack_c8;
  undefined8 uStack_c0;
  longlong lStack_b8;
  undefined8 uStack_b0;
  undefined4 uStack_a8;
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined8 uStack_98;
  undefined *puStack_90;
  undefined8 uStack_88;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined *puStack_78;
  undefined8 uStack_70;
  char ***pppcStack_68;
  undefined8 uStack_60;
  undefined8 uStack_58;
  char **ppcStack_40;
  char ***pppcStack_38;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  longlong lStack_28;
  char cStack_19;
  undefined8 uStack_18;
  
  uStack_18 = 0xfffffffffffffffe;
  cVar5 = FUN_141a3c070();
  if (cVar5 == '\0') {
    return;
  }
  if (_DAT_14363a8b8 != 3) {
    pppcStack_38 = (char ***)CONCAT71(pppcStack_38._1_7_,1);
    pppcStack_c8 = (char ***)&pppcStack_38;
    FUN_1428d8820(&DAT_14363a8b8,0,&pppcStack_c8,&UNK_142ff83a8,&UNK_142ff8620);
  }
  FUN_140542890(&pppcStack_c8,_DAT_14363a8b0);
  if ((int)pppcStack_c8 == 1) {
    pppcStack_38 = uStack_c0;
    uStack_30 = CONCAT31(uStack_30._1_3_,(char)lStack_b8);
    FUN_1428d9760(&UNK_142ff8638,0x13,&pppcStack_38,&UNK_142ff8440,&UNK_142ff8620);
  }
  else {
    ppcStack_40 = (char **)uStack_c0;
    cStack_19 = (char)lStack_b8;
    uStack_c0 = (char ***)func_0x000140613c10(0xf,1);
    if (uStack_c0 != (char ***)0x0) {
      *(undefined8 *)((longlong)uStack_c0 + 7) = 0x2928202168746170;
      *uStack_c0 = (char **)0x705f656c75646f6d;
      pppcStack_c8 = (char ***)0xf;
      lStack_b8 = 0xf;
      FUN_1428dacd0(&pppcStack_c8,0xf,0x17,1,1);
      uVar3 = _UNK_142ff8653;
      *(undefined8 *)((longlong)uStack_c0 + lStack_b8) = _DAT_142ff864b;
      ((undefined8 *)((longlong)uStack_c0 + lStack_b8))[1] = uVar3;
      *(undefined8 *)((longlong)uStack_c0 + lStack_b8 + 0xf) = 0x6e726157203a3a20;
      lStack_28 = lStack_b8 + 0x17;
      pppcStack_38 = pppcStack_c8;
      uStack_30 = (undefined4)uStack_c0;
      uStack_2c = uStack_c0._4_4_;
      if ((ulonglong)((longlong)pppcStack_c8 - lStack_28) < 0x1a) {
        FUN_1428dacd0(&pppcStack_38,lStack_28,0x1a,1,1);
      }
      uVar3 = _UNK_142ff8674;
      puVar1 = (undefined8 *)(CONCAT44(uStack_2c,uStack_30) + 10 + lStack_28);
      *puVar1 = CONCAT26(_UNK_142ff8672,_DAT_142ff866c);
      puVar1[1] = uVar3;
      uVar3 = CONCAT62(_DAT_142ff866c,_UNK_142ff866a);
      puVar1 = (undefined8 *)(CONCAT44(uStack_2c,uStack_30) + lStack_28);
      *puVar1 = _DAT_142ff8662;
      puVar1[1] = uVar3;
      lStack_b8 = lStack_28 + 0x1a;
      pppcStack_c8 = pppcStack_38;
      uStack_c0 = (char ***)CONCAT44(uStack_2c,uStack_30);
      lStack_28 = lStack_b8;
      cVar5 = FUN_141a0bc00(ppcStack_40 + 1,&pppcStack_c8);
      if ((cVar5 == '\0') && (1 < *_DAT_14362c3a8)) {
        puVar6 = (undefined4 *)func_0x00014067dd80(&UNK_142ff8620);
        uStack_a8 = *puVar6;
        uStack_a4 = puVar6[1];
        uStack_a0 = puVar6[2];
        uStack_9c = puVar6[3];
        uStack_7c = puVar6[4];
        uStack_98 = 2;
        puStack_90 = &UNK_142ff8690;
        uStack_88 = 0x28;
        puStack_78 = &UNK_142ff8680;
        uStack_70 = 1;
        pppcStack_68 = (char ***)&pppcStack_38;
        pppcStack_c8 = (char ***)0x0;
        uStack_60 = 0;
        uStack_58 = 0;
        uStack_c0 = (char ***)&UNK_142ff8690;
        lStack_b8 = 0x28;
        uStack_b0 = 0;
        uStack_80 = 1;
        func_0x000140d4a330(pppcStack_68,&pppcStack_c8);
      }
      ppcVar4 = ppcStack_40;
      if (((cStack_19 == '\0') && ((*_DAT_14362c420 & 0x7fffffffffffffff) != 0)) &&
         (cVar5 = FUN_1428d86b0(), cVar5 == '\0')) {
        *(char *)((longlong)ppcVar4 + 1) = '\x01';
      }
      LOCK();
      cVar5 = *(char *)ppcVar4;
      *(char *)ppcVar4 = '\0';
      UNLOCK();
      if (cVar5 != '\x02') {
        return;
      }
      FUN_1428d8810(ppcVar4);
      return;
    }
    FUN_1428d8fe3(1,0xf,&UNK_142ff84b0);
  }
                    /* WARNING: Does not return */
  pcVar2 = (code *)invalidInstructionException();
  (*pcVar2)();
}

