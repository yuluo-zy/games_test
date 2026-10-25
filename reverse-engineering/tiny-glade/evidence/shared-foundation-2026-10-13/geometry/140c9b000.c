utils::geometry::rectangle::Rectangle2d::get_closest_u_from_pos

/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140c9b000(undefined8 *param_1,longlong param_2)

{
  code *pcVar1;
  longlong lVar2;
  longlong lVar3;
  float fVar4;
  float fVar5;
  undefined1 auStack_bc [4];
  undefined8 uStack_b8;
  undefined8 uStack_b0;
  undefined8 uStack_a8;
  undefined8 uStack_a0;
  undefined8 uStack_98;
  undefined1 *puStack_88;
  code *pcStack_80;
  
  lVar2 = FUN_140c9a9d0();
  fVar4 = (float)param_1[2];
  fVar5 = (float)((ulonglong)param_1[2] >> 0x20);
  if (0.0 < fVar4 * fVar5) {
    lVar3 = 0;
    if (param_2 != 4) {
      lVar3 = param_2;
    }
    uStack_b8 = (undefined *)CONCAT44(fVar5 * _DAT_142aa5d60,fVar4 * _UNK_142aa5d64);
    uStack_b0 = CONCAT44(fVar5 * _UNK_142aa5d68,fVar4 * _UNK_142aa5d64);
    uStack_a8 = (undefined1 **)CONCAT44(fVar5 * _UNK_142aa5d68,fVar4 * _UNK_142aa5d6c);
    uStack_a0 = CONCAT44(fVar5 * _DAT_142aa5d60,fVar4 * _UNK_142aa5d6c);
                    /* WARNING: Could not recover jumptable at 0x000140c9b0f9. Too many branches */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142b2eb00 + *(int *)(&UNK_142b2eb00 + lVar2 * 4)))(lVar3,0,*param_1,param_1[1]);
    return;
  }
  FUN_1428d9430(&UNK_142b2e66c,0x23,&UNK_142b2e6b8);
  puStack_88 = auStack_bc;
  pcStack_80 = FUN_140d8c910;
  uStack_b8 = &UNK_142e717b8;
  uStack_b0 = 1;
  uStack_98 = 0;
  uStack_a8 = &puStack_88;
  uStack_a0 = 1;
  FUN_1428d9390(&uStack_b8,&UNK_142b2e8f8);
  FUN_1428d9430(&UNK_142e717c8,0x1d,&UNK_142b2e8f8);
  pcVar1 = (code *)swi(3);
  (*pcVar1)();
  return;
}

