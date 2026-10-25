
void FUN_140a27b10(undefined8 param_1,longlong param_2,undefined8 param_3,undefined8 param_4,
                  undefined8 param_5,undefined8 param_6,int param_7,undefined8 *param_8,char param_9
                  ,undefined4 param_10)

{
  code *pcVar1;
  int iVar2;
  longlong lVar3;
  undefined8 *puVar4;
  ulonglong uVar5;
  undefined4 uVar6;
  undefined1 auStack_838 [400];
  undefined1 auStack_6a8 [400];
  undefined1 auStack_518 [400];
  undefined *puStack_388;
  undefined8 uStack_380;
  undefined1 auStack_278 [38];
  char cStack_252;
  longlong lStack_250;
  undefined8 uStack_1f8;
  undefined4 uStack_1f0;
  undefined4 uStack_1ec;
  undefined8 uStack_1e8;
  uint uStack_1e0;
  char cStack_1dc;
  undefined1 auStack_60 [8];
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined8 uStack_50;
  undefined8 *puStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  uVar5 = 0x8226cc786e679355;
  lVar3 = FUN_1400dd6f0(param_2 + 0x38,0x8226cc786e679355,0x65cfda6a55a841ac);
  if (((lVar3 != 0) && (uVar5 < *(ulonglong *)(param_2 + 0x218))) &&
     (uVar5 = *(ulonglong *)(*(longlong *)(param_2 + 0x210) + uVar5 * 8), uVar5 != 0)) {
    lVar3 = ~uVar5 * 0x60;
    if (*(longlong *)(*(longlong *)(param_2 + 0x1e0) + 0x20 + lVar3) != 0) {
      puVar4 = (undefined8 *)
               FUN_140969720(*(undefined8 *)(*(longlong *)(param_2 + 0x1e0) + lVar3 + 0x10),
                             &UNK_142c2e360);
      if (puVar4 == (undefined8 *)0x0) goto code_r0x000140a27fa7;
      puStack_388 = &UNK_142a9de39;
      uStack_380 = 0x3d;
      if (puVar4[3] == 0x3d) {
        iVar2 = func_0x000142923bf0(&UNK_142a9de39,puVar4[2],0x3d);
        if (iVar2 == 0) {
          lVar3 = (**(code **)(puVar4[1] + 0x18))(*puVar4);
          if (lVar3 != 0) {
            FUN_1412e4250(auStack_60,lVar3);
            uStack_39 = 1;
            FUN_141602080(&uStack_1f8,&UNK_142aca980,0x12,param_10,param_2,param_1);
            uVar6 = (undefined4)((ulonglong)param_2 >> 0x20);
            uStack_39 = 1;
            FUN_141608a20(&puStack_388,&uStack_1f8,0x686c38b1c905f34d,1);
            uStack_39 = 1;
            FUN_141608260(&uStack_1f8,&puStack_388,0xd005d9dfc249ec9b,1);
            uStack_39 = 1;
            FUN_14096e000(&puStack_388,&uStack_1f8,0x77fada2ea41d423b,param_4,CONCAT44(uVar6,1));
            uStack_39 = 1;
            FUN_14096e000(&uStack_1f8,&puStack_388,0x7ab593be137ba575,param_3,1);
            uStack_39 = 1;
            FUN_14096e2a0(&puStack_388,&uStack_1f8,0x2a75aeb2d5bdccbd,param_5);
            uStack_39 = 1;
            FUN_14096e2a0(auStack_6a8,&puStack_388,0x668a7272bfe497f1,param_6);
            if (param_7 == 0) {
              uStack_39 = 1;
              FUN_14160df90(&uStack_1f8,auStack_6a8,0x35ab36bc43c7b8ed);
              uStack_39 = 1;
              FUN_141608e00(auStack_518,&uStack_1f8,0x3783a9903b7f67c5,1);
            }
            else {
              uStack_39 = 1;
              FUN_14160cd70(&uStack_1f8,auStack_6a8,0x35ab36bc43c7b8ed);
              uStack_39 = 1;
              FUN_1416091e0(auStack_518,&uStack_1f8,0x3783a9903b7f67c5,1);
            }
            if (param_8 != (undefined8 *)0x0) {
              func_0x000142923c00(&uStack_1f8,auStack_518,400);
              puStack_48 = param_8 + 1;
              puStack_388 = (undefined *)CONCAT44(puStack_388._4_4_,2);
              FUN_1408ee240(&puStack_48,&uStack_1f8,0x62b6812f89a8a569,0x19,&puStack_388);
              func_0x000142923c00(&puStack_388,&uStack_1f8,400);
              if (cStack_252 != '\b') {
                FUN_14096b760(auStack_278,lStack_250 + 0xe8,0xe8d23613960341d3,*param_8);
              }
              func_0x000142923c00(auStack_518,&puStack_388,400);
            }
            func_0x000142923c00(&uStack_1f8,auStack_518,400);
            uStack_39 = 1;
            FUN_14160c460(auStack_838,&uStack_1f8,0x6d9256d4ac7887ad);
            uStack_1e8 = uStack_50;
            uStack_1f0 = uStack_58;
            uStack_1ec = uStack_54;
            cStack_1dc = param_9;
            uStack_1e0 = (uint)(byte)(param_9 * '\x04');
            uStack_39 = 0;
            FUN_141609940(auStack_838,&uStack_1f8);
            return;
          }
          goto code_r0x000140a27fa7;
        }
      }
      uStack_1f8 = 0;
      FUN_1428cab41(0,&puStack_388,puVar4 + 2,&uStack_1f8,&UNK_142c2e180);
    }
  }
  FUN_1428d9310(&UNK_142aca950);
code_r0x000140a27fa7:
  FUN_1428d9310(&UNK_142aca968);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

