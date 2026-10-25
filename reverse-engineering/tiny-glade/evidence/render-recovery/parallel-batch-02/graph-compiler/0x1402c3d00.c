
void FUN_1402c3d00(longlong param_1,char *param_2,undefined8 param_3,longlong param_4)

{
  undefined1 uVar1;
  longlong lVar2;
  code *pcVar3;
  char cVar4;
  undefined4 uVar5;
  char *pcVar6;
  undefined1 *puVar7;
  longlong lVar8;
  undefined8 uVar9;
  char *pcVar10;
  char *unaff_RBP;
  undefined1 uVar11;
  undefined *puVar12;
  undefined *unaff_R12;
  undefined1 *puVar13;
  undefined8 in_stack_ffffffffffffff48;
  undefined4 uVar14;
  undefined4 uStack_a8;
  undefined1 uStack_a4;
  undefined4 uStack_a0;
  undefined1 uStack_9c;
  char *pcStack_98;
  undefined8 uStack_90;
  undefined8 auStack_88 [6];
  longlong lStack_58;
  undefined8 uStack_50;
  char cStack_48;
  char cStack_47;
  
  pcVar6 = *(char **)(param_1 + 0x70);
  uStack_90 = param_3;
  if (param_2 < pcVar6) {
    pcVar10 = (char *)(ulonglong)
                      *(uint *)(*(longlong *)(param_1 + 0x68) + 4 + (longlong)param_2 * 8);
    pcVar6 = (char *)func_0x000140be9690(*(undefined4 *)
                                          (*(longlong *)(param_1 + 0x68) + (longlong)param_2 * 8));
    if (pcVar10 < pcVar6) {
code_r0x0001402c41c9:
      FUN_1428d9850(pcVar6);
    }
    else {
      pcStack_98 = *(char **)(param_1 + 0x58);
      if (pcVar10 <= pcStack_98) {
        if (pcVar6 != pcVar10) {
          lVar2 = *(longlong *)(param_1 + 0x50);
          puVar13 = (undefined1 *)(lVar2 + (longlong)pcVar6 * 0x14 + 0x10);
          lStack_58 = param_4;
          do {
            FUN_140be78b0(&uStack_50,param_1,*(undefined4 *)(puVar13 + -0x10));
            unaff_RBP = (char *)(ulonglong)*(uint *)(puVar13 + -0xc);
            if (unaff_RBP == (char *)0xffffffff) {
              if (cStack_47 == '\0') goto code_r0x0001402c3dd1;
              if (cStack_48 == '\0') {
                param_2 = &UNK_1429aefb0;
                lVar8 = FUN_140bd6c80(uStack_50);
                uVar14 = (undefined4)((ulonglong)in_stack_ffffffffffffff48 >> 0x20);
                if (lVar8 != 0) {
                  if (lVar8 == 1) {
                    puVar12 = *(undefined **)(param_4 + 0x648);
                    if (*(short *)(param_2 + 0x14) != *(short *)(puVar12 + 0xa50))
                    goto code_r0x0001402c40e7;
                    if (param_2[0x16] != '\x01') goto code_r0x0001402c4171;
                    if (*(int *)(*(longlong *)(param_2 + 8) + 0xc) == *(int *)(param_2 + 0x10)) {
                      uVar9 = *(undefined8 *)(*(longlong *)(param_2 + 8) + 0x30);
                      uVar1 = *puVar13;
                      uVar11 = 0;
                      goto code_r0x0001402c3f6a;
                    }
                    goto code_r0x0001402c425c;
                  }
                  goto code_r0x0001402c4244;
                }
                puVar12 = *(undefined **)(param_4 + 0x648);
                if (*(short *)(param_2 + 0x14) != *(short *)(puVar12 + 0xa50))
                goto code_r0x0001402c40e7;
                if (param_2[0x16] == '\0') {
                  lVar8 = *(longlong *)(param_2 + 8);
                  if (*(int *)(lVar8 + 0xc) == *(int *)(param_2 + 0x10)) {
                    puVar12 = *(undefined **)(lVar8 + 0x40);
                    uVar1 = *puVar13;
                    uVar5 = func_0x000140bd6630(*(undefined4 *)(lVar8 + 0x18));
                    in_stack_ffffffffffffff48 = CONCAT44(uVar14,uVar5);
                    uVar11 = 0;
                    goto code_r0x0001402c3db7;
                  }
                  FUN_1428d9310(&UNK_1429aefc8);
                  goto code_r0x0001402c4088;
                }
              }
              else {
code_r0x0001402c40b7:
                auStack_88[0] = 0;
                param_2 = &cStack_48;
                puVar12 = &UNK_1429aef90;
                FUN_1428a21b8(0,param_2,&UNK_1429aef90,auStack_88,&UNK_1429aef98);
code_r0x0001402c40e7:
                auStack_88[0] = 0;
                FUN_1428ca816(0,param_2 + 0x14,puVar12 + 0xa50,auStack_88,&UNK_1429f7d18);
code_r0x0001402c4113:
                unaff_R12 = unaff_R12 + 0x16;
                auStack_88[0] = 0;
                FUN_1428ca902(0,unaff_R12,&UNK_1429f7cf0,auStack_88,&UNK_1429f7d30);
code_r0x0001402c4142:
                param_2 = unaff_R12 + 0x16;
                auStack_88[0] = 0;
                FUN_1428ca902(0,param_2,&UNK_1429f7d10,auStack_88,&UNK_1429f7d30);
code_r0x0001402c4171:
                param_2 = param_2 + 0x16;
                auStack_88[0] = 0;
                FUN_1428ca902(0,param_2,&UNK_1429f7d10,auStack_88,&UNK_1429f7d30);
              }
              pcVar10 = param_2 + 0x16;
              auStack_88[0] = 0;
              pcVar6 = (char *)FUN_1428ca902(0,pcVar10,&UNK_1429f7cf0,auStack_88,&UNK_1429f7d30);
              goto code_r0x0001402c41c9;
            }
            unaff_R12 = &UNK_1429aef00;
            lVar8 = FUN_140bd6c80(uStack_50);
            if (lVar8 == 0) {
              if (pcStack_98 <= unaff_RBP) goto code_r0x0001402c41ec;
              uStack_a8 = *(undefined4 *)(lVar2 + 0xc + (longlong)unaff_RBP * 0x14);
              uStack_a4 = *(undefined1 *)(lVar2 + 0x10 + (longlong)unaff_RBP * 0x14);
              uStack_a0 = *(undefined4 *)(puVar13 + -4);
              uStack_9c = *puVar13;
              cVar4 = FUN_140bece70(&uStack_a0,&uStack_a8);
              uVar1 = uStack_9c;
              uVar11 = uStack_a4;
              uVar14 = (undefined4)((ulonglong)in_stack_ffffffffffffff48 >> 0x20);
              if (cVar4 == '\0') {
                puVar12 = *(undefined **)(param_4 + 0x648);
                if (*(short *)(unaff_R12 + 0x14) != *(short *)(puVar12 + 0xa50))
                goto code_r0x0001402c4088;
                if (unaff_R12[0x16] != '\0') goto code_r0x0001402c4113;
                lVar8 = *(longlong *)(unaff_R12 + 8);
                if (*(int *)(lVar8 + 0xc) != *(int *)(unaff_R12 + 0x10)) goto code_r0x0001402c422c;
                puVar12 = *(undefined **)(lVar8 + 0x40);
                uVar5 = func_0x000140bd6630(*(undefined4 *)(lVar8 + 0x18));
                in_stack_ffffffffffffff48 = CONCAT44(uVar14,uVar5);
                unaff_R12 = puVar12;
code_r0x0001402c3db7:
                func_0x000140bd65e0(auStack_88,puVar12,uVar11,uVar1,in_stack_ffffffffffffff48);
                FUN_140bd66a0(uStack_90,auStack_88);
                param_4 = lStack_58;
              }
            }
            else {
              if (lVar8 != 1) goto code_r0x0001402c4200;
              if (pcStack_98 <= unaff_RBP) goto code_r0x0001402c4218;
              uStack_a8 = *(undefined4 *)(lVar2 + 0xc + (longlong)unaff_RBP * 0x14);
              uStack_a4 = *(undefined1 *)(lVar2 + 0x10 + (longlong)unaff_RBP * 0x14);
              uStack_a0 = *(undefined4 *)(puVar13 + -4);
              uStack_9c = *puVar13;
              cVar4 = FUN_140bece70(&uStack_a0,&uStack_a8);
              if (cVar4 == '\0') {
                puVar12 = *(undefined **)(param_4 + 0x648);
                if (*(short *)(unaff_R12 + 0x14) != *(short *)(puVar12 + 0xa50)) {
code_r0x0001402c4088:
                  unaff_R12 = unaff_R12 + 0x14;
                  auStack_88[0] = 0;
                  FUN_1428ca816(0,unaff_R12,puVar12 + 0xa50,auStack_88,&UNK_1429f7d18);
                  goto code_r0x0001402c40b7;
                }
                if (unaff_R12[0x16] != '\x01') goto code_r0x0001402c4142;
                if (*(int *)(*(longlong *)(unaff_R12 + 8) + 0xc) != *(int *)(unaff_R12 + 0x10))
                goto code_r0x0001402c4238;
                uVar9 = *(undefined8 *)(*(longlong *)(unaff_R12 + 8) + 0x30);
                uVar1 = uStack_9c;
                uVar11 = uStack_a4;
code_r0x0001402c3f6a:
                func_0x000140bd6610(auStack_88,uVar9,uVar11,uVar1);
                FUN_140bd6810(uStack_90,auStack_88);
              }
            }
code_r0x0001402c3dd1:
            puVar7 = puVar13 + 4;
            puVar13 = puVar13 + 0x14;
          } while (puVar7 != (undefined1 *)(lVar2 + (longlong)pcVar10 * 0x14));
        }
        return;
      }
    }
    FUN_1428d9840(pcVar10,pcStack_98,&UNK_1429aeee8);
code_r0x0001402c41ec:
    FUN_1428d9518(unaff_RBP,pcStack_98,&UNK_1429aef18);
code_r0x0001402c4200:
    FUN_1428d9430(&UNK_1429aed78,0x13,&UNK_1429aef78);
code_r0x0001402c4218:
    FUN_1428d9518(unaff_RBP,pcStack_98,&UNK_1429aef48);
code_r0x0001402c422c:
    FUN_1428d9310(&UNK_1429aef30);
code_r0x0001402c4238:
    FUN_1428d9310(&UNK_1429aef60);
code_r0x0001402c4244:
    param_2 = (char *)0x13;
    FUN_1428d9430(&UNK_1429aed78,0x13,&UNK_1429aeff8);
code_r0x0001402c425c:
    pcVar6 = (char *)FUN_1428d9310(&UNK_1429aefe0);
  }
  FUN_1428d9518(param_2,pcVar6,&UNK_1429aeed0);
  pcVar3 = (code *)swi(3);
  (*pcVar3)();
  return;
}

