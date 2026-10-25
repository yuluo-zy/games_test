// addr 0x1425e57e0 sig void __cdecl sort8_stable<ref$<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::slice::impl$0::sort_by::closure_env$0<ref$<system_onboarding::hint_orchestrator::DisplayHintRequest>,system_onboarding::hint_orchestrator::hint_orchestrator::closure_env$0>_>(void) size 898

void __cdecl
core::slice::sort::shared::smallsort::
sort8_stable<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::slice::impl_0::sort_by::closure_env_0<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,system_onboarding::hint_orchestrator::hint_orchestrator::closure_env_0>_>
          (void)

{
  byte bVar1;
  byte bVar2;
  byte bVar3;
  byte bVar4;
  ulonglong uVar5;
  ulonglong uVar6;
  uint uVar7;
  longlong *plVar8;
  longlong lVar9;
  longlong *in_RCX;
  ulonglong uVar10;
  longlong *in_RDX;
  ulonglong uVar11;
  longlong lVar12;
  uint uVar13;
  longlong lVar14;
  longlong *in_R8;
  ulonglong uVar15;
  ulonglong uVar16;
  longlong lVar17;
  bool bVar18;
  bool bVar19;
  bool bVar20;
  
  bVar18 = *(byte *)(in_RCX[1] + 0x49) < *(byte *)(*in_RCX + 0x49);
  uVar15 = (ulonglong)!bVar18;
  bVar19 = *(byte *)(in_RCX[3] + 0x49) < *(byte *)(in_RCX[2] + 0x49);
  uVar7 = bVar19 + 2;
  uVar13 = (uint)bVar19;
  uVar10 = (ulonglong)(uVar13 ^ 3);
  bVar1 = *(byte *)(in_RCX[uVar7] + 0x49);
  bVar2 = *(byte *)(in_RCX[bVar18] + 0x49);
  uVar11 = (ulonglong)uVar7;
  if (bVar1 < bVar2) {
    uVar11 = uVar15;
  }
  uVar16 = uVar15;
  if (*(byte *)(in_RCX[uVar10] + 0x49) < *(byte *)(in_RCX[uVar15] + 0x49)) {
    uVar16 = (ulonglong)uVar7;
    uVar11 = (ulonglong)uVar13 ^ 3;
    uVar10 = uVar15;
  }
  if (bVar1 < bVar2) {
    uVar16 = (ulonglong)bVar18;
  }
  lVar12 = in_RCX[bVar18];
  if (bVar1 < bVar2) {
    lVar12 = in_RCX[uVar7];
  }
  bVar18 = *(byte *)(in_RCX[uVar11] + 0x49) < *(byte *)(in_RCX[uVar16] + 0x49);
  *in_R8 = lVar12;
  plVar8 = in_RCX + uVar16;
  if (bVar18) {
    plVar8 = in_RCX + uVar11;
  }
  in_R8[1] = *plVar8;
  plVar8 = in_RCX + uVar11;
  if (bVar18) {
    plVar8 = in_RCX + uVar16;
  }
  in_R8[2] = *plVar8;
  lVar9 = in_RCX[uVar10];
  in_R8[3] = lVar9;
  bVar18 = *(byte *)(in_RCX[5] + 0x49) < *(byte *)(in_RCX[4] + 0x49);
  uVar15 = (ulonglong)!bVar18;
  bVar19 = *(byte *)(in_RCX[7] + 0x49) < *(byte *)(in_RCX[6] + 0x49);
  uVar7 = bVar19 + 2;
  uVar13 = (uint)bVar19;
  uVar10 = (ulonglong)(uVar13 ^ 3);
  bVar1 = *(byte *)(in_RCX[(ulonglong)uVar7 + 4] + 0x49);
  bVar2 = *(byte *)(in_RCX[(ulonglong)bVar18 + 4] + 0x49);
  uVar11 = (ulonglong)uVar7;
  if (bVar1 < bVar2) {
    uVar11 = uVar15;
  }
  uVar16 = uVar15;
  if (*(byte *)(in_RCX[uVar10 + 4] + 0x49) < *(byte *)(in_RCX[uVar15 + 4] + 0x49)) {
    uVar16 = (ulonglong)uVar7;
    uVar11 = (ulonglong)uVar13 ^ 3;
    uVar10 = uVar15;
  }
  if (bVar1 < bVar2) {
    uVar16 = (ulonglong)bVar18;
  }
  lVar14 = in_RCX[(ulonglong)bVar18 + 4];
  if (bVar1 < bVar2) {
    lVar14 = in_RCX[(ulonglong)uVar7 + 4];
  }
  bVar18 = *(byte *)(in_RCX[uVar11 + 4] + 0x49) < *(byte *)(in_RCX[uVar16 + 4] + 0x49);
  in_R8[4] = lVar14;
  plVar8 = in_RCX + uVar16 + 4;
  if (bVar18) {
    plVar8 = in_RCX + uVar11 + 4;
  }
  in_R8[5] = *plVar8;
  plVar8 = in_RCX + uVar11 + 4;
  if (bVar18) {
    plVar8 = in_RCX + uVar16 + 4;
  }
  in_R8[6] = *plVar8;
  lVar17 = in_RCX[uVar10 + 4];
  in_R8[7] = lVar17;
  bVar18 = *(byte *)(lVar14 + 0x49) < *(byte *)(lVar12 + 0x49);
  uVar10 = (ulonglong)!bVar18;
  if (!bVar18) {
    lVar14 = lVar12;
  }
  *in_RDX = lVar14;
  bVar1 = *(byte *)(lVar17 + 0x49);
  bVar2 = *(byte *)(lVar9 + 0x49);
  lVar12 = (ulonglong)(bVar1 < bVar2) - 1;
  if (bVar1 < bVar2) {
    lVar17 = lVar9;
  }
  in_RDX[7] = lVar17;
  uVar11 = (ulonglong)(bVar1 < bVar2);
  bVar19 = *(byte *)(in_R8[(ulonglong)bVar18 + 4] + 0x49) < *(byte *)(in_R8[uVar10] + 0x49);
  uVar16 = (ulonglong)!bVar19;
  uVar15 = (ulonglong)bVar19;
  lVar9 = in_R8[uVar10];
  if (bVar19) {
    lVar9 = in_R8[(ulonglong)bVar18 + 4];
  }
  in_RDX[1] = lVar9;
  bVar3 = *(byte *)(in_R8[(ulonglong)(bVar1 < bVar2) + 6] + 0x49);
  bVar4 = *(byte *)(in_R8[3 - uVar11] + 0x49);
  lVar14 = (ulonglong)(bVar3 < bVar4) - 1;
  lVar9 = in_R8[(ulonglong)(bVar1 < bVar2) + 6];
  if (bVar3 < bVar4) {
    lVar9 = in_R8[3 - uVar11];
  }
  in_RDX[6] = lVar9;
  uVar5 = (ulonglong)(bVar3 < bVar4);
  bVar19 = *(byte *)(in_R8[(ulonglong)bVar18 + uVar15 + 4] + 0x49) <
           *(byte *)(in_R8[uVar10 + uVar16] + 0x49);
  lVar9 = in_R8[uVar10 + uVar16];
  if (bVar19) {
    lVar9 = in_R8[(ulonglong)bVar18 + uVar15 + 4];
  }
  in_RDX[2] = lVar9;
  bVar1 = *(byte *)(in_R8[lVar12 + (ulonglong)(bVar3 < bVar4) + 6] + 0x49);
  bVar2 = *(byte *)(in_R8[(3 - uVar5) - uVar11] + 0x49);
  lVar9 = in_R8[lVar12 + (ulonglong)(bVar3 < bVar4) + 6];
  if (bVar1 < bVar2) {
    lVar9 = in_R8[(3 - uVar5) - uVar11];
  }
  in_RDX[5] = lVar9;
  uVar6 = (ulonglong)(bVar1 < bVar2);
  bVar20 = *(byte *)(in_R8[(ulonglong)bVar18 + uVar15 + (ulonglong)bVar19 + 4] + 0x49) <
           *(byte *)(in_R8[uVar10 + uVar16 + !bVar19] + 0x49);
  lVar9 = in_R8[uVar10 + uVar16 + !bVar19];
  if (bVar20) {
    lVar9 = in_R8[(ulonglong)bVar18 + uVar15 + (ulonglong)bVar19 + 4];
  }
  in_RDX[3] = lVar9;
  bVar3 = *(byte *)(in_R8[lVar12 + lVar14 + (ulonglong)(bVar1 < bVar2) + 6] + 0x49);
  bVar4 = *(byte *)(in_R8[((3 - uVar6) - uVar5) - uVar11] + 0x49);
  lVar9 = in_R8[lVar12 + lVar14 + (ulonglong)(bVar1 < bVar2) + 6];
  if (bVar3 < bVar4) {
    lVar9 = in_R8[((3 - uVar6) - uVar5) - uVar11];
  }
  in_RDX[4] = lVar9;
  if ((in_R8 + uVar10 + uVar16 + (ulonglong)!bVar19 + (ulonglong)!bVar20 ==
       in_R8 + ((((4 - (ulonglong)(bVar3 < bVar4)) - uVar6) - uVar5) - uVar11)) &&
     (in_R8 + (ulonglong)bVar18 + uVar15 + (ulonglong)bVar19 + (ulonglong)bVar20 + 4 ==
      in_R8 + lVar12 + lVar14 + (ulonglong)(bVar1 < bVar2) + (ulonglong)(bVar3 < bVar4) + 6)) {
    return;
  }
                    // WARNING: Subroutine does not return
  panic_on_ord_violation();
}


// CALLEES: [{"address": "1428d97f0", "name": "panic_on_ord_violation"}]