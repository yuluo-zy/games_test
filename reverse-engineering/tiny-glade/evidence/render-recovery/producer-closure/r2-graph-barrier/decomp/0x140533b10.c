
void FUN_140533b10(ulonglong *param_1,uint param_2,uint param_3,undefined4 param_4)

{
  int *piVar1;
  longlong lVar2;
  ulonglong uVar3;
  code *pcVar4;
  ulonglong uVar5;
  char cVar6;
  undefined4 uVar7;
  longlong lVar8;
  undefined4 *puVar9;
  undefined4 uVar10;
  char cVar11;
  ulonglong uVar12;
  char *pcVar13;
  ulonglong uVar14;
  char cStack_d5;
  undefined4 uStack_d4;
  uint uStack_d0;
  uint uStack_cc;
  ulonglong uStack_c8;
  uint *puStack_c0;
  code *pcStack_b8;
  char *pcStack_b0;
  undefined *puStack_a8;
  undefined4 *puStack_a0;
  code *pcStack_98;
  char *pcStack_90;
  undefined *puStack_88;
  undefined4 *puStack_80;
  code *pcStack_78;
  undefined *puStack_70;
  undefined8 uStack_68;
  uint **ppuStack_60;
  undefined8 uStack_58;
  undefined8 uStack_50;
  
  cVar11 = (char)param_3;
  uVar14 = *param_1;
  cStack_d5 = cVar11;
  uStack_d4 = param_4;
  uStack_cc = param_2;
  if (SBORROW8(0,uVar14)) {
code_r0x000140533d5f:
    FUN_1428d9310(&UNK_1429f4d60);
  }
  else if ((ulonglong)param_2 < *(ulonglong *)(param_1[8] + 0x40)) {
    lVar2 = *(longlong *)(param_1[8] + 0x38);
    lVar8 = (ulonglong)param_2 * 0x10;
    piVar1 = (int *)(lVar2 + lVar8);
    uVar5 = param_1[9];
    if (*(int *)(lVar2 + lVar8) != (int)uVar5) {
      uVar12 = param_1[2];
      goto code_r0x000140533c3e;
    }
    uVar3 = *(ulonglong *)(piVar1 + 2);
    uVar12 = param_1[2];
    uStack_d0 = param_2;
    uStack_c8 = uVar14;
    if (uVar12 <= uVar3) goto code_r0x000140533d77;
    uVar14 = param_1[1];
    puVar9 = (undefined4 *)(uVar14 + uVar3 * 0xc + 4);
    if ((*(char *)(uVar14 + 8 + uVar3 * 0xc) == cVar11) &&
       (cVar6 = func_0x000140bece60(puVar9,&uStack_d4), cVar6 != '\0')) {
      return;
    }
    pcVar13 = (char *)(uVar14 + uVar3 * 0xc + 8);
    cVar6 = func_0x000140bece60(puVar9,&uStack_d4);
    if (cVar6 == '\0') {
code_r0x000140533c10:
      if ((cVar11 == '7') && ((byte)(*pcVar13 - 0x19U) < 2)) {
code_r0x000140533c26:
        *pcVar13 = '7';
        uStack_d4 = 0;
        *puVar9 = 0;
        uVar14 = uStack_c8;
        param_2 = uStack_d0;
code_r0x000140533c3e:
        *piVar1 = (int)uVar5;
        *(ulonglong *)(piVar1 + 2) = uVar12;
        uVar10 = uStack_d4;
        uVar7 = func_0x000140b25c90(param_3);
        if (uVar12 == uVar14) {
          FUN_140562cf0(param_1,&UNK_1429f4da8);
        }
        uVar14 = param_1[1];
        *(uint *)(uVar14 + uVar12 * 0xc) = param_2;
        *(undefined4 *)(uVar14 + 4 + uVar12 * 0xc) = uVar7;
        *(char *)(uVar14 + 8 + uVar12 * 0xc) = (char)uVar10;
        param_1[2] = uVar12 + 1;
        return;
      }
    }
    else {
      cVar6 = *pcVar13;
      if (cVar6 == '\x17') {
        if (cVar11 == '\x1d') {
code_r0x000140533ca6:
          *pcVar13 = '\x1e';
          return;
        }
      }
      else if (cVar6 == '\x1a') {
        if (cVar11 == '\x02') {
          *pcVar13 = '\x02';
          return;
        }
        if ((param_3 & 0xff) == 0x37) goto code_r0x000140533c26;
      }
      else {
        if (cVar6 != '\x1d') goto code_r0x000140533c10;
        if (cVar11 == '\x17') goto code_r0x000140533ca6;
      }
    }
    puStack_c0 = &uStack_cc;
    pcStack_b8 = FUN_140533a30;
    puStack_a8 = &UNK_140533a00;
    pcStack_98 = FUN_140533a80;
    pcStack_90 = &cStack_d5;
    puStack_88 = &UNK_140533a00;
    pcStack_78 = FUN_140533a80;
    puStack_70 = &UNK_1429f4e50;
    uStack_68 = 6;
    uStack_50 = 0;
    ppuStack_60 = &puStack_c0;
    uStack_58 = 5;
    pcStack_b0 = pcVar13;
    puStack_a0 = puVar9;
    puStack_80 = &uStack_d4;
    FUN_1428d9390(&puStack_70,&UNK_1429f4eb0);
    goto code_r0x000140533d5f;
  }
  FUN_1428d9518();
code_r0x000140533d77:
  FUN_1428d9518();
  pcVar4 = (code *)swi(3);
  (*pcVar4)();
  return;
}

