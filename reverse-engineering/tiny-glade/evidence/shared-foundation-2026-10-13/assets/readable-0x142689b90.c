
/* WARNING: Type propagation algorithm not settling */
void asset__inheritable__impl$1__inherit_from(ulonglong *param_1,ulonglong *param_2)
{
  int iVar1;
  code *pcVar2;
  ulonglong uVar3;
  undefined1 unaff_BL;
  uint unaff_EDI;
  ulonglong uVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  undefined *puVar7;
  undefined8 *******pppppppuStack_78;
  undefined8 *******pppppppuStack_70;
  undefined *puStack_68;
  undefined8 *******pppppppuStack_60;
  undefined8 *******pppppppuStack_58;
  undefined8 uStack_50;
  undefined8 uStack_48;
  undefined8 *******pppppppuStack_38;
  undefined *puStack_30;
  uVar6 = *param_1;
  uVar3 = uVar6 + 0x7ffffffffffffffe;
  if (1 < uVar3) {
    uVar3 = 2;
  }
  if (uVar3 == 0) {
    uVar5 = *param_2 + 0x7ffffffffffffffe;
    uVar3 = 2;
    if (uVar5 < 2) {
      uVar3 = uVar5;
    }
    if (uVar3 == 0) {
      puVar7 = (undefined *)0x8000000000000002;
    }
    else if (uVar3 == 2) {
      unaff_EDI = (uint)param_2[3];
      if (1 < unaff_EDI) {
        pppppppuStack_70 =
             (undefined8 *******)
             CONCAT44(pppppppuStack_70._4_4_,*(undefined4 *)((longlong)param_2 + 0x24));
        pppppppuStack_78 = *(undefined8 ********)((longlong)param_2 + 0x1c);
      }
      unaff_BL = (undefined1)param_2[5];
      uVar5 = *param_2 ^ 0x8000000000000000;
      uVar3 = 2;
      if (uVar5 < 2) {
        uVar3 = uVar5;
      }
      puVar7 = (undefined *)0x8000000000000000;
      if (uVar3 != 0) {
        if (uVar3 == 2) {
          FUN_140d7bd90(&puStack_68);
          puVar7 = puStack_68;
        }
        else {
          puVar7 = (undefined *)0x8000000000000001;
        }
      }
      puStack_30 = (undefined *)CONCAT44(puStack_30._4_4_,pppppppuStack_70._0_4_);
      pppppppuStack_38 = pppppppuStack_78;
      pppppppuStack_78 = pppppppuStack_60;
      pppppppuStack_70 = pppppppuStack_58;
    }
    else {
      puVar7 = (undefined *)0x8000000000000003;
    }
    if (((uVar6 >> 1 != 0x4000000000000001) && (-0x7fffffffffffffff < (longlong)uVar6)) &&
       (uVar6 != 0)) {
      func_0x000140613c20(param_1[1],uVar6,1);
    }
    *param_1 = (ulonglong)puVar7;
    param_1[1] = (ulonglong)pppppppuStack_78;
    param_1[2] = (ulonglong)pppppppuStack_70;
    *(uint *)(param_1 + 3) = unaff_EDI;
    *(undefined8 ********)((longlong)param_1 + 0x1c) = pppppppuStack_38;
    *(undefined4 *)((longlong)param_1 + 0x24) = puStack_30._0_4_;
    *(undefined1 *)(param_1 + 5) = unaff_BL;
    return;
  }
  if (uVar3 == 2) {
    uVar3 = *param_2;
    uVar4 = uVar3 + 0x7ffffffffffffffe;
    uVar5 = 2;
    if (uVar4 < 2) {
      uVar5 = uVar4;
    }
    if (uVar5 == 2) {
      if ((char)param_1[5] == '\0') {
        *(char *)(param_1 + 5) = (char)param_2[5];
        iVar1 = (int)param_1[3];
      }
      else {
        iVar1 = (int)param_1[3];
      }
      if (iVar1 == 0) {
        if (1 < (uint)param_2[3]) {
          pppppppuStack_60 =
               (undefined8 *******)
               CONCAT44(pppppppuStack_60._4_4_,*(undefined4 *)((longlong)param_2 + 0x24));
          puStack_68 = *(undefined **)((longlong)param_2 + 0x1c);
        }
        *(uint *)(param_1 + 3) = (uint)param_2[3];
        *(undefined **)((longlong)param_1 + 0x1c) = puStack_68;
        *(undefined4 *)((longlong)param_1 + 0x24) = pppppppuStack_60._0_4_;
      }
      if (!SBORROW8(0,uVar6)) {
        return;
      }
      uVar3 = uVar3 ^ 0x8000000000000000;
      uVar6 = 2;
      if (uVar3 < 2) {
        uVar6 = uVar3;
      }
      if (uVar6 == 2) {
        FUN_140d7bd90(&puStack_68);
      }
      else if (uVar6 == 1) {
        puStack_68 = (undefined *)0x8000000000000001;
      }
      else {
        puStack_68 = (undefined *)0x8000000000000000;
      }
      param_1[2] = (ulonglong)pppppppuStack_58;
      *param_1 = (ulonglong)puStack_68;
      param_1[1] = (ulonglong)pppppppuStack_60;
      return;
    }
    if (uVar5 != 1) goto LAB_142689e8b;
    pppppppuStack_78 = (undefined8 *******)&UNK_1431511e1;
    pppppppuStack_70 = (undefined8 *******)0x29;
    pppppppuStack_38 = &pppppppuStack_78;
    puStack_30 = &UNK_140109e90;
    puStack_68 = &UNK_143150340;
    pppppppuStack_60 = (undefined8 *******)0x2;
    uStack_48 = 0;
    pppppppuStack_58 = &pppppppuStack_38;
    uStack_50 = 1;
    FUN_1428d9390(&puStack_68,&UNK_143150420);
  }
  FUN_1428d9430(&UNK_1431503c0,0x28,&UNK_143150408);
LAB_142689e8b:
  puStack_68 = &UNK_1431502f8;
  pppppppuStack_60 = (undefined8 *******)0x1;
  pppppppuStack_58 = (undefined8 *******)0x8;
  uStack_50 = 0;
  uStack_48 = 0;
  FUN_1428d9390(&puStack_68,&UNK_143150420);
  pcVar2 = (code *)swi(3);
  (*pcVar2)();
  return;
}
