
void derive_stair_node_params__14080c640(undefined8 param_1,longlong param_2,undefined8 param_3,undefined8 param_4)
{
  undefined8 uVar1;
  longlong lVar2;
  
  uVar1 = graph__1408fe8b0(param_4);
  lVar2 = get__14125de50(uVar1);
  if (lVar2 != 0) {
    lVar2 = 0;
    if ((byte)(*(byte *)(param_2 + 0x22) - 2) < 3) {
      lVar2 = (ulonglong)*(byte *)(param_2 + 0x22) - 1;
    }
                    /* WARNING: Could not emulate address calculation at 0x00014080c6e0 */
                    /* WARNING: Treating indirect jump as call */
    (*(code *)(&UNK_142a8e448 + *(int *)(&UNK_142a8e448 + lVar2 * 4)))
              (&UNK_142a8e448 + *(int *)(&UNK_142a8e448 + lVar2 * 4));
    return;
  }
  lVar2 = 0;
  if ((byte)(*(byte *)(param_2 + 0x22) - 2) < 3) {
    lVar2 = (ulonglong)*(byte *)(param_2 + 0x22) - 1;
  }
                    /* WARNING: Could not emulate address calculation at 0x00014080c6bb */
                    /* WARNING: Treating indirect jump as call */
  (*(code *)(&UNK_142a8e448 + *(int *)(&UNK_142a8e448 + lVar2 * 4)))
            (&UNK_142a8e448 + *(int *)(&UNK_142a8e448 + lVar2 * 4));
  return;
}
