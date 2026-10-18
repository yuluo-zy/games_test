// addr 0x1425e5b70 sig void __cdecl insertion_sort_shift_left<ref$<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::slice::impl$0::sort_by::closure_env$0<ref$<system_onboarding::hint_orchestrator::DisplayHintRequest>,system_onboarding::hint_orchestrator::hint_orchestrator::closure_env$0>_>(void) size 116

void __cdecl
core::slice::sort::shared::smallsort::
insertion_sort_shift_left<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,alloc::slice::impl_0::sort_by::closure_env_0<ref_<system_onboarding::hint_orchestrator::DisplayHintRequest>,system_onboarding::hint_orchestrator::hint_orchestrator::closure_env_0>_>
          (void)

{
  longlong lVar1;
  code *pcVar2;
  longlong *in_RCX;
  ulonglong in_RDX;
  longlong *plVar3;
  ulonglong in_R8;
  longlong lVar4;
  longlong *plVar5;
  longlong lVar6;
  longlong lVar7;
  
  if (in_RDX <= in_R8 - 1) {
                    // WARNING: Does not return
    pcVar2 = (code *)invalidInstructionException();
    (*pcVar2)();
  }
  if (in_R8 != in_RDX) {
    plVar3 = in_RCX + in_R8;
    lVar4 = in_R8 << 3;
    do {
      lVar7 = plVar3[-1];
      lVar1 = *plVar3;
      lVar6 = lVar4;
      if (*(byte *)(lVar1 + 0x49) < *(byte *)(lVar7 + 0x49)) {
        do {
          *(longlong *)((longlong)in_RCX + lVar6) = lVar7;
          plVar5 = in_RCX;
          if (lVar6 == 8) goto LAB_1425e5b93;
          lVar7 = *(longlong *)((longlong)in_RCX + lVar6 + -0x10);
          lVar6 = lVar6 + -8;
        } while (*(byte *)(lVar1 + 0x49) < *(byte *)(lVar7 + 0x49));
        plVar5 = (longlong *)(lVar6 + (longlong)in_RCX);
LAB_1425e5b93:
        *plVar5 = lVar1;
      }
      plVar3 = plVar3 + 1;
      lVar4 = lVar4 + 8;
    } while (plVar3 != in_RCX + in_RDX);
  }
  return;
}

