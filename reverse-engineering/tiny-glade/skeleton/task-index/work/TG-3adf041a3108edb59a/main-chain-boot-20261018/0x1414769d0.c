
void __cdecl
core::iter::adapters::map::impl$2::
fold<country_core::systems::history::replay_wall::WallReplayEditCmd,core::slice::iter::Iter<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>_>,country_core::systems::history::replay_wall::impl_0::new::closure_0::closure_env_0,tuple_<>,core::iter::traits::iterator::Iterator::for_each::call::closure_env_0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::vec::impl_20::extend_trusted::closure_env_0<country_core::systems::history::replay_wall::WallReplayEditCmd,alloc::alloc::Global,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::sync::Arc<enum2_<country_core::resources::history::wall_compound_edit::WallHistoryEdit>,alloc::alloc::Global>_>,country_core::systems::history::replay_wall::impl_0::new::closure_0::closure_env_0>_>_>_>
          (void)

{
  longlong lVar1;
  undefined1 uVar2;
  undefined8 *puVar3;
  undefined8 *puVar4;
  longlong *plVar5;
  undefined1 *puVar6;
  char *pcVar7;
  code *pcVar8;
  undefined8 *in_RCX;
  undefined8 *in_RDX;
  ulonglong uVar9;
  longlong *plVar10;
  undefined1 *puVar11;
  longlong lVar12;
  undefined4 uVar13;
  float in_XMM0_Da;
  
  puVar3 = (undefined8 *)*in_RCX;
  puVar4 = (undefined8 *)in_RCX[1];
  plVar5 = (longlong *)*in_RDX;
  lVar12 = in_RDX[1];
  if (puVar3 == puVar4) {
LAB_141476b18:
    *plVar5 = lVar12;
    return;
  }
  puVar6 = (undefined1 *)in_RCX[2];
  pcVar7 = (char *)in_RCX[3];
  puVar11 = (undefined1 *)(in_RDX[2] + lVar12 * 0x18 + 0x15);
  uVar9 = 0;
  plVar10 = (longlong *)*puVar3;
  uVar13 = 0;
  switch(plVar10[2]) {
  case -0x8000000000000000:
  case -0x7fffffffffffffff:
  case -0x7ffffffffffffffd:
  case -0x7ffffffffffffff9:
  case -0x7ffffffffffffff8:
  case -0x7ffffffffffffff7:
  case -0x7ffffffffffffff6:
  case -0x7ffffffffffffff5:
  case -0x7ffffffffffffff1:
    break;
  case -0x7ffffffffffffffe:
  case -0x7ffffffffffffffc:
  case -0x7ffffffffffffffb:
  case -0x7ffffffffffffffa:
  case -0x7ffffffffffffff4:
  case -0x7ffffffffffffff3:
    goto code_r0x000141476ad2;
  case -0x7ffffffffffffff2:
code_r0x000141476b2d:
                    // WARNING: Subroutine does not return
    country_core::resources::history::wall_compound_edit::panic_deprecated_operation();
  default:
    goto LAB_141476ae2;
  }
  do {
    puVar11 = puVar11 + 0x18;
    plVar10 = (longlong *)puVar3[uVar9];
    uVar13 = 0;
    switch(plVar10[2]) {
    case -0x7ffffffffffffffe:
    case -0x7ffffffffffffffc:
    case -0x7ffffffffffffffb:
    case -0x7ffffffffffffffa:
    case -0x7ffffffffffffff4:
    case -0x7ffffffffffffff3:
code_r0x000141476ad2:
      uVar13 = 1;
      in_XMM0_Da = 0.3;
    case -0x8000000000000000:
    case -0x7fffffffffffffff:
    case -0x7ffffffffffffffd:
    case -0x7ffffffffffffff9:
    case -0x7ffffffffffffff8:
    case -0x7ffffffffffffff7:
    case -0x7ffffffffffffff6:
    case -0x7ffffffffffffff5:
    case -0x7ffffffffffffff1:
      LOCK();
      lVar1 = *plVar10;
      *plVar10 = *plVar10 + 1;
      UNLOCK();
      if (*plVar10 == 0 || SCARRY8(lVar1,1) != *plVar10 < 0) {
                    // WARNING: Does not return
        pcVar8 = (code *)invalidInstructionException();
        (*pcVar8)();
      }
      uVar2 = *puVar6;
      if (*pcVar7 != '\0') {
        uVar13 = 0;
      }
      *(undefined4 *)(puVar11 + -0x15) = uVar13;
      *(float *)(puVar11 + -0x11) = in_XMM0_Da;
      *(longlong **)(puVar11 + -0xd) = plVar10;
      *(undefined4 *)(puVar11 + -5) = 0;
      puVar11[-1] = uVar2;
      *puVar11 = 0;
      uVar9 = uVar9 + 1;
      if ((ulonglong)((longlong)puVar4 - (longlong)puVar3) >> 3 == uVar9) {
        lVar12 = lVar12 + uVar9;
        goto LAB_141476b18;
      }
      break;
    case -0x7ffffffffffffff2:
      goto code_r0x000141476b2d;
    default:
LAB_141476ae2:
      in_XMM0_Da = ABS(*(float *)(plVar10 + 0x16) - *(float *)(plVar10 + 8)) * 0.01;
      if (in_XMM0_Da <= 0.4) {
        in_XMM0_Da = 0.4;
      }
      uVar13 = 1;
      break;
    }
  } while( true );
}

