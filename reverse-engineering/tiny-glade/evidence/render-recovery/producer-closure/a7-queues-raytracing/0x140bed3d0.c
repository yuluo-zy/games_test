
void FUN_140bed3d0(int *param_1,longlong param_2)

{
  undefined8 uVar1;
  undefined8 uVar2;
  code *pcVar3;
  int iVar4;
  undefined *puStack_68;
  undefined8 uStack_60;
  int **ppiStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  int *piStack_38;
  code *pcStack_30;
  
  puStack_68 = (undefined *)CONCAT71(puStack_68._1_7_,0x18);
  FUN_1418c1380(0x143634e40,&puStack_68,1);
  if (*param_1 == 1) {
    uVar1 = *(undefined8 *)(param_1 + 4);
    uVar2 = *(undefined8 *)(param_1 + 2);
    iVar4 = (**(code **)(param_2 + 0x2c8))(*(undefined8 *)(param_1 + 0x12));
    if (iVar4 == 0) {
      param_1[0] = 2;
      param_1[1] = 0;
      *(undefined8 *)(param_1 + 2) = uVar2;
      *(undefined8 *)(param_1 + 4) = uVar1;
      return;
    }
  }
  else {
    pcStack_30 = FUN_140bed620;
    puStack_68 = &UNK_142b14d78;
    uStack_60 = 1;
    uStack_48 = 0;
    ppiStack_58 = &piStack_38;
    uStack_50 = 1;
    piStack_38 = param_1;
    iVar4 = FUN_1428d9390(&puStack_68,&UNK_142b14d88);
  }
  puStack_68 = (undefined *)CONCAT44(puStack_68._4_4_,iVar4);
  FUN_1428d9760(&UNK_142b14d20,0x12,&puStack_68,&UNK_142b14b30,&UNK_142b14d38);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

