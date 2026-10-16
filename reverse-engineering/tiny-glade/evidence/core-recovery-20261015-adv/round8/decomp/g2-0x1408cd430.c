// FUN_1408cd430 0x1408cd430

undefined8 *
FUN_1408cd430(undefined8 *param_1,undefined8 param_2,undefined8 param_3,undefined8 param_4,
             undefined4 param_5,uint param_6)

{
  byte abStack_7c [4];
  undefined4 uStack_78;
  undefined8 uStack_74;
  undefined4 uStack_6c;
  uint uStack_68;
  undefined4 uStack_64;
  undefined8 uStack_60;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 uStack_40;
  undefined4 uStack_38;
  int iStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  
  if ((param_6 & 0x20) == 0) {
    FUN_1408cdc60(param_1,param_2,param_3,0);
  }
  else {
    FUN_1408cdc60(&uStack_68,param_2,param_3,0);
    if (iStack_34 == 2) {
      FUN_1409e9a20(&uStack_68,param_4,param_3,param_5);
      if ((uStack_68 & 1) == 0) {
        *(undefined4 *)((longlong)param_1 + 0x34) = 2;
      }
      else {
        *(undefined4 *)(param_1 + 4) = uStack_58;
        param_1[3] = uStack_60;
        *(undefined1 *)param_1 = 0xb;
        *(undefined8 *)((longlong)param_1 + 0x24) = 0x3f80000000000000;
        *(undefined4 *)((longlong)param_1 + 0x2c) = 0;
        *(undefined4 *)(param_1 + 6) = uStack_64;
        *(undefined4 *)((longlong)param_1 + 0x34) = 0;
      }
    }
    else {
      FUN_1409e9a20(abStack_7c,param_4,param_3,uStack_38);
      if ((abStack_7c[0] & 1) == 0) {
        param_1[4] = uStack_48;
        param_1[5] = uStack_40;
        param_1[2] = CONCAT44(uStack_54,uStack_58);
        param_1[3] = uStack_50;
        *param_1 = CONCAT44(uStack_64,uStack_68);
        param_1[1] = uStack_60;
        *(ulonglong *)((longlong)param_1 + 0x34) = CONCAT44(uStack_30,iStack_34);
        *(undefined4 *)((longlong)param_1 + 0x3c) = uStack_2c;
        *(undefined4 *)(param_1 + 6) = uStack_38;
      }
      else {
        *(undefined4 *)(param_1 + 4) = uStack_6c;
        param_1[3] = uStack_74;
        *(undefined1 *)param_1 = 0xb;
        *(undefined8 *)((longlong)param_1 + 0x24) = 0x3f80000000000000;
        *(undefined4 *)((longlong)param_1 + 0x2c) = 0;
        *(undefined4 *)((longlong)param_1 + 0x34) = 0;
        *(undefined4 *)(param_1 + 6) = uStack_78;
      }
    }
  }
  return param_1;
}

