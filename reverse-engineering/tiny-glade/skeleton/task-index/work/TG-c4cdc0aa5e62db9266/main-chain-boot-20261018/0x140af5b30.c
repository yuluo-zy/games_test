
// WARNING: Control flow encountered bad instruction data
// WARNING: Instruction at (ram,0x000140af7d6a) overlaps instruction at (ram,0x000140af7d67)
// 
// WARNING: Removing unreachable block (ram,0x000140af64eb)
// WARNING: Globals starting with '_' overlap smaller symbols at the same address

void __cdecl country_core::systems::history::history_process::history_process(void)

{
  size_t _Size;
  byte *pbVar1;
  undefined1 auVar2 [16];
  byte bVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined4 uVar7;
  int iVar8;
  ulonglong *puVar9;
  undefined4 *puVar10;
  undefined4 *puVar11;
  undefined **ppuVar12;
  undefined4 *puVar13;
  undefined8 uVar14;
  undefined1 (*pauVar15) [16];
  longlong *plVar16;
  undefined8 uVar17;
  undefined8 uVar18;
  undefined8 uVar19;
  undefined8 uVar20;
  undefined8 uVar21;
  undefined8 uVar22;
  longlong lVar23;
  longlong lVar24;
  longlong lVar25;
  uint *puVar26;
  uint uVar27;
  uint *puVar28;
  ulonglong uVar29;
  code *UNRECOVERED_JUMPTABLE;
  undefined **ppuVar30;
  undefined **ppuVar31;
  ulonglong uVar32;
  undefined1 *extraout_RAX;
  undefined1 *_Dst;
  undefined8 extraout_RAX_00;
  longlong *plVar33;
  longlong *in_RCX;
  longlong lVar34;
  undefined8 *in_RDX;
  undefined **ppuVar35;
  ulonglong *puVar36;
  undefined1 (*pauVar37) [16];
  longlong *plVar38;
  char cVar39;
  longlong lVar40;
  longlong *plVar41;
  undefined8 *puVar42;
  longlong *in_R8;
  ulonglong *puVar43;
  longlong lVar44;
  undefined8 *in_R9;
  ulonglong uVar45;
  ulonglong *unaff_R12;
  uint *puVar46;
  ulonglong uVar47;
  uint *_Src;
  char cVar48;
  ulonglong uVar49;
  uint *puVar50;
  ulonglong *puVar51;
  uint *puVar52;
  bool bVar53;
  undefined8 *in_stack_00000030;
  longlong *in_stack_00000038;
  undefined8 *in_stack_00000040;
  longlong *in_stack_00000048;
  
  puVar36 = (ulonglong *)*in_R9;
  lVar34 = in_R9[1];
  uVar29 = *puVar36;
  uVar47 = 0;
  uVar32 = uVar29 - *(ulonglong *)(lVar34 + 0x18);
  if (uVar29 < *(ulonglong *)(lVar34 + 0x18)) {
    uVar32 = uVar47;
  }
  uVar45 = uVar29 - *(ulonglong *)(lVar34 + 0x38);
  if (uVar29 < *(ulonglong *)(lVar34 + 0x38)) {
    uVar45 = uVar47;
  }
  puVar46 = (uint *)(uVar32 * 0x10 + *(longlong *)(lVar34 + 8));
  uVar29 = *(ulonglong *)(lVar34 + 0x10);
  uVar49 = uVar29 - uVar32;
  if (uVar29 < uVar32) {
    uVar49 = uVar47;
  }
  if (uVar29 < uVar32) {
    puVar46 = (uint *)&DAT_00000008;
  }
  puVar28 = (uint *)(uVar45 * 0x10 + *(longlong *)(lVar34 + 0x28));
  uVar29 = *(ulonglong *)(lVar34 + 0x30) - uVar45;
  if (*(ulonglong *)(lVar34 + 0x30) < uVar45) {
    puVar28 = (uint *)&DAT_00000008;
    uVar29 = uVar47;
  }
  *puVar36 = (*(longlong *)(lVar34 + 0x40) - uVar29) - uVar49;
  puVar50 = puVar46 + uVar49 * 4;
  puVar52 = puVar28 + uVar29 * 4;
  puVar9 = (ulonglong *)*in_RDX;
  puVar10 = (undefined4 *)in_RDX[2];
  uVar4 = *(undefined4 *)((longlong)in_RDX + 0x1c);
  uVar5 = *(undefined4 *)((longlong)in_stack_00000040 + 0x1c);
  puVar42 = (undefined8 *)*in_stack_00000040;
  puVar11 = (undefined4 *)in_stack_00000040[2];
  uVar6 = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
  puVar51 = (ulonglong *)*in_stack_00000030;
  ppuVar12 = (undefined **)in_stack_00000030[2];
  lVar34 = *in_RCX;
  uVar7 = *(undefined4 *)((longlong)in_R8 + 0x1c);
  lVar44 = *in_R8;
  puVar13 = (undefined4 *)in_R8[2];
  ppuVar35 = ppuVar12;
  do {
    if ((puVar46 == (uint *)0x0) || (puVar46 == puVar50)) {
      if ((puVar28 == (uint *)0x0) || (puVar28 == puVar52)) {
        if (*(longlong *)(lVar44 + 0x18) != 0) {
          return;
        }
        if (*(longlong *)(*in_stack_00000048 + 0x10) != 0) {
          return;
        }
        *puVar10 = uVar4;
        if (puVar9[3] == 0) {
          return;
        }
        uVar29 = puVar9[2];
        uVar32 = 0;
        if (*puVar9 <= uVar29 + 1) {
          uVar32 = *puVar9;
        }
        puVar9[2] = (uVar29 - uVar32) + 1;
        puVar9[3] = puVar9[3] - 1;
        cVar39 = (char)*(undefined2 *)(puVar9[1] + 1 + uVar29 * 4);
        if (cVar39 == '\x04' || *(char *)(puVar9[1] + uVar29 * 4) == '\x02') {
          return;
        }
        *(undefined4 *)in_stack_00000038[2] = *(undefined4 *)((longlong)in_stack_00000038 + 0x1c);
        lVar44 = *in_stack_00000038;
        lVar40 = *(longlong *)(lVar44 + 0x30);
        uVar14 = *(undefined8 *)(lVar44 + 0x40);
        if (lVar40 == *(longlong *)(lVar44 + 0x20)) {
          alloc::raw_vec::RawVec::
          grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
                    ();
        }
        *(undefined8 *)(*(longlong *)(lVar44 + 0x28) + lVar40 * 8) = uVar14;
        *(longlong *)(lVar44 + 0x30) = lVar40 + 1;
        *(longlong *)(lVar44 + 0x40) = *(longlong *)(lVar44 + 0x40) + 1;
        if (cVar39 == '\0') {
          *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
          iVar8 = *(int *)(lVar34 + 0x68);
          if ((longlong)iVar8 < 0) {
            return;
          }
          if (iVar8 <= *(int *)(lVar34 + 0x6c)) {
            return;
          }
          if (*(ulonglong *)(lVar34 + 0x10) <= (ulonglong)(longlong)iVar8) {
            return;
          }
          *(int *)(lVar34 + 0x68) = iVar8 + -1;
        }
        else {
          if (cVar39 != '\x01') {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          *(undefined4 *)in_RCX[2] = *(undefined4 *)((longlong)in_RCX + 0x1c);
          if (*(int *)(lVar34 + 0x68) < -1) {
                    // WARNING: Subroutine does not return
            core::panicking::panic();
          }
          uVar27 = *(int *)(lVar34 + 0x68) + 1;
          if ((int)uVar27 < 0) {
            return;
          }
          if (*(ulonglong *)(lVar34 + 0x10) <= (ulonglong)uVar27) {
            return;
          }
          *(uint *)(lVar34 + 0x68) = uVar27;
        }
        replay_edit();
        return;
      }
      puVar46 = (uint *)0x0;
      puVar26 = puVar28 + 4;
      _Src = puVar28;
    }
    else {
      puVar26 = puVar28;
      _Src = puVar46;
      puVar46 = puVar46 + 4;
    }
    puVar28 = puVar26;
    *puVar36 = *puVar36 + 1;
    if (!SBORROW8(0,*(longlong *)(lVar34 + 0x30))) {
      if (1 < *(ulonglong *)__imp__ZN3log20MAX_LOG_LEVEL_FILTER17h0a1e497ddb29da86E) {
        toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
        ::log::__private_api::impl$2::log();
      }
                    // WARNING: Read-only address (ram,0x000142ae5420) is written
      return;
    }
    uVar29 = (ulonglong)*(int *)(&DAT_142ae5420 + (ulonglong)*(byte *)((longlong)_Src + 9) * 4);
    UNRECOVERED_JUMPTABLE = (code *)(&DAT_142ae5420 + uVar29);
    cVar39 = (char)UNRECOVERED_JUMPTABLE;
    uVar27 = (uint)UNRECOVERED_JUMPTABLE;
                    // WARNING (jumptable): Sanity check requires truncation of jumptable
                    // WARNING: Could not find normalized switch variable to match jumptable
    switch(*(byte *)((longlong)_Src + 9)) {
    case 0:
      if (puVar9[3] != 0) {
        ppuVar30 = (undefined **)(puVar9[3] + puVar9[2] + -1);
        ppuVar35 = (undefined **)0x0;
        if ((undefined **)*puVar9 <= ppuVar30) {
          ppuVar35 = (undefined **)*puVar9;
        }
        if (*(char *)(puVar9[1] + 1 + ((longlong)ppuVar30 - (longlong)ppuVar35) * 4) == '\x01')
        goto code_r0x000140af5df8;
      }
      break;
    case 1:
      if (puVar9[3] != 0) {
        ppuVar30 = (undefined **)(puVar9[3] + puVar9[2] + -1);
        ppuVar35 = (undefined **)0x0;
        if ((undefined **)*puVar9 <= ppuVar30) {
          ppuVar35 = (undefined **)*puVar9;
        }
        if (*(char *)(puVar9[1] + 1 + ((longlong)ppuVar30 - (longlong)ppuVar35) * 4) == '\0')
        goto code_r0x000140af5df8;
      }
      break;
    case 2:
      goto code_r0x000140af5df8;
    case 3:
      *puVar13 = uVar7;
      resources::history::replay_cmd_buffer::ReplayBuffer::make_animations_instant();
      goto code_r0x000140af5df8;
    case 4:
      *(uint *)UNRECOVERED_JUMPTABLE = *(int *)UNRECOVERED_JUMPTABLE + uVar27;
      (&UNK_142ae5421)[uVar29] = (&UNK_142ae5421)[uVar29] + ' ';
      puVar46 = (uint *)((longlong)puVar42 + 99);
      *puVar46 = *puVar46 >> 0xc | *puVar46 << 0x14;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    default:
      *(char *)((longlong)in_stack_00000030 + 0x66) =
           *(char *)((longlong)in_stack_00000030 + 0x66) +
           (char)((ulonglong)UNRECOVERED_JUMPTABLE >> 8);
      goto LAB_140af6660;
    case 8:
      goto LAB_140af6660;
    case 9:
      *(char *)(uVar29 + 0xe1225a5e) = *(char *)(uVar29 + 0xe1225a5e) + cVar39;
      goto LAB_140af6660;
    case 10:
      if ((int)in_stack_00000040 == 8) goto code_r0x000140af6490;
      goto LAB_140af6660;
    case 0xb:
      pbVar1 = (byte *)((longlong)puVar42 + 0x142ae5469);
      *pbVar1 = *pbVar1 | 0x10;
      if (*pbVar1 == 0) {
code_r0x000140af6490:
        puVar43 = (ulonglong *)(in_R8 + 1);
        puVar36 = puVar51;
        if (puVar9 == puVar43) goto LAB_140af6610;
        goto LAB_140af649d;
      }
      goto LAB_140af6660;
    case 0xc:
                    // WARNING (jumptable): Read-only address (ram,0x000142ae5420) is written
                    // WARNING: Read-only address (ram,0x000142ae5420) is written
                    // WARNING: Read-only address (ram,0x000142ae5421) is written
      _DAT_142ae5420 = CONCAT31(_UNK_142ae5421,(-0xe - cVar39) - (0xfffffffebd51abdf < uVar29));
      *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + cVar39);
      goto LAB_140af6467;
    case 0xe:
      *(uint *)UNRECOVERED_JUMPTABLE = *(int *)UNRECOVERED_JUMPTABLE + uVar27;
      *(char *)(uVar29 + 0xcbf6585e) = *(char *)(uVar29 + 0xcbf6585e) + cVar39;
      UNRECOVERED_JUMPTABLE = (code *)swi(1);
      (*UNRECOVERED_JUMPTABLE)();
      return;
    case 0xf:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x10:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x11:
      LOCK();
      *(undefined4 *)(uVar29 + 0x2855ca838) = 0x42ae5420;
      UNLOCK();
      goto code_r0x000140af7d86;
    case 0x12:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x13:
      (&UNK_142ae5459)[uVar29] = (&UNK_142ae5459)[uVar29] | 0x20;
      *(int *)((longlong)puVar28 + -0x1a) = *(int *)((longlong)puVar28 + -0x1a) << 1;
      *(undefined8 *)(puVar28 + 0x14) = *(undefined8 *)(puVar28 + 0x10);
      *(undefined8 *)(puVar28 + 0x16) = *(undefined8 *)(puVar28 + 0xe);
      if (*(longlong *)(puVar28 + 0x12) != 0) {
        unaff_R12 = *(ulonglong **)(puVar28 + 0x14);
        puVar51 = unaff_R12 + *(longlong *)(puVar28 + 0x12);
        puVar46 = (uint *)&DAT_142ae545c;
        do {
          uVar29 = *unaff_R12;
          uVar32 = *(ulonglong *)(_Src + 4);
          if (uVar32 <= uVar29) {
                    // WARNING: Subroutine does not return
            alloc::vec::impl$1::remove::assert_failed();
          }
          lVar34 = *(longlong *)(_Src + 2);
          lVar44 = uVar29 * 0x30;
          uVar14 = *(undefined8 *)(lVar34 + lVar44);
          uVar17 = ((undefined8 *)(lVar34 + lVar44))[1];
          puVar42 = (undefined8 *)(lVar34 + 0x10 + lVar44);
          uVar18 = *puVar42;
          uVar19 = puVar42[1];
          puVar42 = (undefined8 *)(lVar34 + 0x20 + lVar44);
          uVar20 = puVar42[1];
          *(undefined8 *)(puVar28 + 4) = *puVar42;
          *(undefined8 *)(puVar28 + 6) = uVar20;
          *(undefined8 *)puVar28 = uVar18;
          *(undefined8 *)(puVar28 + 2) = uVar19;
          *(undefined8 *)(puVar28 + -4) = uVar14;
          *(undefined8 *)(puVar28 + -2) = uVar17;
          memmove((void *)(lVar34 + lVar44),(void *)(lVar34 + lVar44 + 0x30),
                  (~uVar29 + uVar32) * 0x30);
          *(ulonglong *)(_Src + 4) = uVar32 - 1;
          if ((ulonglong)(byte)puVar28[-4] < 10) {
            UNRECOVERED_JUMPTABLE =
                 (code *)((longlong)(int)puVar46[(byte)puVar28[-4]] + (longlong)puVar46);
code_r0x000140af7d86:
                    // WARNING: Could not recover jumptable at 0x000140af7d86. Too many branches
                    // WARNING: Treating indirect jump as call
            (*UNRECOVERED_JUMPTABLE)();
            return;
          }
          plVar33 = *(longlong **)puVar28;
          LOCK();
          *plVar33 = *plVar33 + -1;
          UNLOCK();
          if (*plVar33 == 0) {
            alloc::sync::Arc::
            drop_slow<std::sync::poison::mutex::Mutex<winit::dpi::PhysicalSize<u32>_>,alloc::alloc::Global>
                      ();
          }
LAB_140af7d00:
          unaff_R12 = unaff_R12 + 1;
        } while (unaff_R12 != puVar51);
      }
      if (*(longlong *)(puVar28 + 0x16) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    case 0x14:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x15:
      UNRECOVERED_JUMPTABLE = (code *)swi(1);
      (*UNRECOVERED_JUMPTABLE)(&UNK_142ae5474);
      return;
    case 0x16:
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    case 0x17:
      plVar33 = (longlong *)(ulonglong)*puVar28;
      LOCK();
      *plVar33 = *plVar33 + -1;
      UNLOCK();
      if (*plVar33 != 0) goto LAB_140af7d00;
      alloc::sync::Arc::
      drop_slow<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,alloc::alloc::Global>
                ();
      goto LAB_140af7d00;
    case 0x18:
      return;
    case 0x19:
    case 0x1a:
    case 0x1b:
                    // WARNING (jumptable): Read-only address (ram,0x000142ae5420) is written
                    // WARNING: Read-only address (ram,0x000142ae5420) is written
      *UNRECOVERED_JUMPTABLE = (code)((char)*UNRECOVERED_JUMPTABLE + '\x01');
                    // WARNING (jumptable): Read-only address (ram,0x000142ae5420) is written
                    // WARNING: Read-only address (ram,0x000142ae5420) is written
      _DAT_142ae5420 = uVar27 | 0xfe0108f2;
                    // WARNING: Bad instruction - Truncating control flow here
      halt_baddata();
    }
    *puVar10 = uVar4;
    uVar27 = _Src[2];
    ppuVar30 = (undefined **)*puVar9;
    ppuVar31 = (undefined **)puVar9[3];
    if (ppuVar31 == ppuVar30) {
      alloc::collections::vec_deque::VecDeque::
      grow<country_core::systems::history::history_process::HistoryCmd,alloc::alloc::Global>();
      ppuVar30 = (undefined **)*puVar9;
      ppuVar31 = (undefined **)puVar9[3];
    }
    ppuVar35 = (undefined **)0x0;
    if (ppuVar30 <= (undefined **)((longlong)ppuVar31 + puVar9[2])) {
      ppuVar35 = ppuVar30;
    }
    *(uint *)(puVar9[1] + ((longlong)((longlong)ppuVar31 + puVar9[2]) - (longlong)ppuVar35) * 4) =
         uVar27;
    puVar9[3] = puVar9[3] + 1;
code_r0x000140af5e5d:
    bVar3 = *(byte *)((longlong)_Src + 0xb);
    unaff_R12 = (ulonglong *)(ulonglong)bVar3;
    if (bVar3 != 3) {
      *(undefined4 *)ppuVar12 = uVar6;
      uVar29 = puVar51[6];
      in_stack_00000030 = (undefined8 *)puVar51[8];
      if (uVar29 == puVar51[4]) {
        ppuVar35 = &anon_60360aed504c4a0f85b3886feb406ce0_1_llvm_17420070914341975340;
        alloc::raw_vec::RawVec::
        grow_one<bevy_ecs::event::base::EventInstance<country_core::resources::window::WindowFocusEvent>,alloc::alloc::Global>
                  ();
      }
      uVar32 = puVar51[5];
      *(undefined8 **)(uVar32 + uVar29 * 0x10) = in_stack_00000030;
      *(byte *)(uVar32 + 8 + uVar29 * 0x10) = bVar3;
      puVar51[6] = uVar29 + 1;
      puVar51[8] = puVar51[8] + 1;
    }
  } while( true );
LAB_140af6467:
  UNRECOVERED_JUMPTABLE = *(code **)(puVar28 + 0x46);
  ppuVar35 = *(undefined ***)(puVar28 + 0x48);
  puVar43 = (ulonglong *)0x0;
LAB_140af649d:
  if (ppuVar35 <= *(undefined ***)((longlong)_Src + (longlong)puVar43)) {
    *(undefined1 *)((longlong)puVar28 + 0x17f) = 1;
    *(undefined1 *)((longlong)puVar28 + 0x17e) = 1;
                    // WARNING: Subroutine does not return
    core::panicking::panic_bounds_check();
  }
  lVar34 = (longlong)*(undefined ***)((longlong)_Src + (longlong)puVar43) * 0x30;
  if ((ulonglong)(byte)UNRECOVERED_JUMPTABLE[lVar34] < 0xb) {
                    // WARNING: Could not recover jumptable at 0x000140af64cc. Too many branches
                    // WARNING: Treating indirect jump as call
    (*(code *)((longlong)(int)puVar46[(byte)UNRECOVERED_JUMPTABLE[lVar34]] + (longlong)puVar46))
              (UNRECOVERED_JUMPTABLE + lVar34);
    return;
  }
  goto LAB_140af6660;
code_r0x000140af5df8:
  *puVar11 = uVar5;
  lVar40 = puVar42[6];
  in_stack_00000030 = (undefined8 *)puVar42[8];
  if (lVar40 == puVar42[4]) {
    ppuVar35 = &anon_60360aed504c4a0f85b3886feb406ce0_1_llvm_17420070914341975340;
    alloc::raw_vec::RawVec::
    grow_one<bevy_ecs::event::base::EventInstance<tiny_glade::app_config::WriteSettingsFileCmd>,alloc::alloc::Global>
              ();
  }
  *(undefined8 **)(puVar42[5] + lVar40 * 8) = in_stack_00000030;
  puVar42[6] = lVar40 + 1;
  puVar42[8] = puVar42[8] + 1;
  *puVar10 = uVar4;
  if (puVar9[3] != 0) {
    puVar9[3] = 0;
  }
  puVar9[2] = 0;
  goto code_r0x000140af5e5d;
LAB_140af6660:
  _Size = (longlong)puVar51 * 8;
  if (((ulonglong)puVar51 >> 0x3d != 0) || (0x7ffffffffffffff8 < _Size)) {
LAB_140af6e5f:
    *(undefined1 *)((longlong)puVar28 + 0x17f) = 1;
    *(undefined1 *)((longlong)puVar28 + 0x17e) = 1;
                    // WARNING: Subroutine does not return
    alloc::raw_vec::handle_error();
  }
  if (_Size == 0) {
    _Dst = &DAT_00000008;
    puVar28[0x5a] = 0;
    puVar28[0x5b] = 0;
  }
  else {
    std::alloc::__default_lib_allocator::__rust_alloc();
    if (extraout_RAX == (undefined1 *)0x0) goto LAB_140af6e5f;
    *(ulonglong **)(puVar28 + 0x5a) = puVar51;
    _Dst = extraout_RAX;
  }
  *(undefined1 **)(puVar28 + 0x58) = _Dst;
  memcpy(_Dst,_Src,_Size);
  lVar34 = *(longlong *)(puVar28 + 0x3e);
  if (lVar34 == *(longlong *)(puVar28 + 0x3a)) {
    alloc::raw_vec::RawVec::
    grow_one<enum2_<slab::Entry<enum2_<futures_util::lock::mutex::Waiter>_>_>,alloc::alloc::Global>
              ();
  }
  lVar44 = *(longlong *)(puVar28 + 0x3c);
  *(undefined8 *)(lVar44 + lVar34 * 0x18) = *(undefined8 *)(puVar28 + 0x5a);
  *(undefined8 *)(lVar44 + 8 + lVar34 * 0x18) = *(undefined8 *)(puVar28 + 0x58);
  *(ulonglong **)(lVar44 + 0x10 + lVar34 * 0x18) = puVar51;
  *(longlong *)(puVar28 + 0x3e) = lVar34 + 1;
  puVar51 = *(ulonglong **)(puVar28 + 0x4e);
  puVar36 = puVar51 + *(longlong *)(puVar28 + 0x50) + -1;
  if (*(longlong *)(puVar28 + 0x50) == 0) {
    puVar36 = (ulonglong *)0x0;
  }
  uVar29 = *puVar36;
  uVar47 = uVar29 + 1;
  uVar32 = *(ulonglong *)(puVar28 + 0x48);
  if (uVar47 < uVar32) {
    puVar28[0x50] = 0;
    puVar28[0x51] = 0;
    if (*(longlong *)(puVar28 + 0x4c) == 0) {
      *(undefined1 *)((longlong)puVar28 + 0x17f) = 1;
      *(undefined1 *)((longlong)puVar28 + 0x17e) = 1;
      alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
      puVar51 = *(ulonglong **)(puVar28 + 0x4e);
    }
    *puVar51 = uVar47;
    puVar28[0x50] = 1;
    puVar28[0x51] = 0;
    in_stack_00000030 = *(undefined8 **)(puVar28 + 0x46);
    puVar42 = in_stack_00000030 + *(longlong *)(puVar28 + 0x48) * 6;
    uVar29 = uVar29 + 2;
    puVar51 = (ulonglong *)0x1;
    unaff_R12 = (ulonglong *)&DAT_00000008;
    lVar34 = 0;
    while (bVar53 = unaff_R12 != (ulonglong *)0x0,
          unaff_R12 = (ulonglong *)((longlong)unaff_R12 + -1), bVar53) {
      if (uVar29 == 0) {
        if (in_stack_00000030 == puVar42) break;
        *(longlong *)(puVar28 + 0x5a) = lVar34;
        _Src = *(uint **)(puVar28 + 0x4e);
      }
      else {
        uVar32 = ((ulonglong)((longlong)puVar42 - (longlong)in_stack_00000030) >> 4) *
                 -0x5555555555555555;
        if (uVar32 < uVar29 || uVar32 - uVar29 == 0) break;
        in_stack_00000030 = in_stack_00000030 + uVar29 * 6;
        *(ulonglong *)(puVar28 + 0x5a) = lVar34 + uVar29;
        _Src = *(uint **)(puVar28 + 0x4e);
      }
      puVar36 = (ulonglong *)0x0;
      if (puVar51 != (ulonglong *)0x0) goto LAB_140af6467;
LAB_140af6610:
      if (puVar36 == *(ulonglong **)(puVar28 + 0x4c)) {
        *(undefined1 *)((longlong)puVar28 + 0x17f) = 1;
        *(undefined1 *)((longlong)puVar28 + 0x17e) = 1;
        alloc::raw_vec::RawVec::grow_one<usize,alloc::alloc::Global>();
        _Src = *(uint **)(puVar28 + 0x4e);
      }
      in_stack_00000030 = in_stack_00000030 + 6;
      lVar34 = *(longlong *)(puVar28 + 0x5a);
      *(longlong *)(_Src + (longlong)puVar36 * 2) = lVar34;
      lVar34 = lVar34 + 1;
      puVar51 = (ulonglong *)((longlong)puVar36 + 1);
      *(ulonglong **)(puVar28 + 0x50) = puVar51;
      uVar29 = 0;
    }
    _Src = *(uint **)(puVar28 + 0x4e);
    goto LAB_140af6660;
  }
  puVar28[0x48] = 0;
  puVar28[0x49] = 0;
  *(longlong *)(puVar28 + -8) = *(longlong *)(puVar28 + 0x46);
  *(ulonglong *)(puVar28 + -6) = uVar32 * 0x30 + *(longlong *)(puVar28 + 0x46);
  *(uint **)(puVar28 + -4) = puVar28 + 0x44;
  *(ulonglong *)(puVar28 + -2) = uVar32;
  puVar28[0] = 0;
  puVar28[1] = 0;
  *(undefined1 *)((longlong)puVar28 + 0x17f) = 1;
  *(undefined1 *)((longlong)puVar28 + 0x17e) = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<enum2_<core::option::Option<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>_>,core::iter::adapters::map::Map<alloc::vec::drain::Drain<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>,alloc::alloc::Global>,enum2_<core::option::Option<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>_>_(*)(enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>)>_>
            ();
  lVar34 = *(longlong *)(puVar28 + 0x52);
  pauVar15 = *(undefined1 (**) [16])(lVar34 + 0x10);
  lVar44 = *(longlong *)(lVar34 + 0x18);
  auVar2 = *pauVar15;
  *(ushort *)(puVar28 + 0x16) =
       ~((ushort)(SUB161(auVar2 >> 7,0) & 1) | (ushort)(SUB161(auVar2 >> 0xf,0) & 1) << 1 |
         (ushort)(SUB161(auVar2 >> 0x17,0) & 1) << 2 | (ushort)(SUB161(auVar2 >> 0x1f,0) & 1) << 3 |
         (ushort)(SUB161(auVar2 >> 0x27,0) & 1) << 4 | (ushort)(SUB161(auVar2 >> 0x2f,0) & 1) << 5 |
         (ushort)(SUB161(auVar2 >> 0x37,0) & 1) << 6 | (ushort)(SUB161(auVar2 >> 0x3f,0) & 1) << 7 |
         (ushort)(SUB161(auVar2 >> 0x47,0) & 1) << 8 | (ushort)(SUB161(auVar2 >> 0x4f,0) & 1) << 9 |
         (ushort)(SUB161(auVar2 >> 0x57,0) & 1) << 10 |
         (ushort)(SUB161(auVar2 >> 0x5f,0) & 1) << 0xb |
         (ushort)(SUB161(auVar2 >> 0x67,0) & 1) << 0xc |
         (ushort)(SUB161(auVar2 >> 0x6f,0) & 1) << 0xd |
         (ushort)(SUB161(auVar2 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar2[0xf] >> 7) << 0xf);
  *(undefined1 (**) [16])(puVar28 + 0x10) = pauVar15;
  pauVar37 = pauVar15 + 1;
  *(undefined1 (**) [16])(puVar28 + 0x12) = pauVar37;
  *(undefined1 **)(puVar28 + 0x14) = *pauVar15 + lVar44 + 1;
  *(undefined8 *)(puVar28 + 0x18) = *(undefined8 *)(lVar34 + 0x28);
  puVar28[-8] = 0;
  puVar28[-7] = 0;
  puVar28[4] = 0;
  puVar28[5] = 0;
  *(undefined1 *)(puVar28 + 0x1a) = *(undefined1 *)((longlong)puVar28 + 0x17b);
  *(undefined1 *)((longlong)puVar28 + 0x17a) = 1;
  *(undefined1 *)((longlong)puVar28 + 0x179) = 1;
  alloc::sync::impl$74::
  to_arc_slice<country_core::resources::walls::decorator::DecoratorId,core::iter::adapters::flatten::FlatMap<std::collections::hash::map::Values<enum2_<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,country_core::resources::history::decorator_edit::DecoratorStorageHistoryDiff>,core::iter::adapters::filter::Filter<core::iter::adapters::copied::Copied<std::collections::hash::map::Keys<country_core::resources::walls::decorator::DecoratorId,country_core::resources::history::decorator_edit::DecoratorHistoryState>_>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_0::closure_env_0>,country_core::resources::history::decorator_edit::impl_7::cull_allow_list::closure_env_0>_>
            ();
  *(undefined8 *)(puVar28 + 0x22) = extraout_RAX_00;
  *(undefined1 (**) [16])(puVar28 + 0x24) = pauVar37;
  *(longlong *)(puVar28 + -8) = *(longlong *)(puVar28 + 0x3c);
  *(longlong *)(puVar28 + -6) = *(longlong *)(puVar28 + 0x3c) + *(longlong *)(puVar28 + 0x3e) * 0x18
  ;
  *(uint **)(puVar28 + -4) = puVar28 + -0x16;
  *(longlong *)(puVar28 + -2) = (longlong)puVar28 + 0x17b;
  *(uint **)puVar28 = puVar28 + 0x22;
  *(undefined1 *)((longlong)puVar28 + 0x175) = 1;
  *(undefined1 *)(puVar28 + 0x5d) = 1;
  alloc::vec::spec_from_iter::impl$0::
  from_iter<country_core::resources::history::replay_cmd_buffer::ReplayBucket,core::iter::adapters::map::Map<core::slice::iter::Iter<alloc::vec::Vec<usize,alloc::alloc::Global>_>,country_core::systems::history::history_process::schedule_replay_commands::closure_env_2>_>
            ();
  plVar33 = *(longlong **)(puVar28 + 0x52);
  if (plVar33[5] == 0) {
    lVar34 = *(longlong *)(puVar28 + 0x56);
    lVar44 = *(longlong *)(lVar34 + 0x40);
    if ((lVar44 != 0) || (*(longlong *)(lVar34 + 0x58) != 0)) {
      lVar40 = *(longlong *)(puVar28 + 0x38) * 0x40 + *(longlong *)(puVar28 + 0x36);
      cVar39 = '\x01';
      if (lVar40 != 0x40 && *(longlong *)(puVar28 + 0x38) != 0) goto LAB_140af6a6c;
      puVar28[0x2a] = 0;
      puVar28[0x2b] = 0;
      puVar28[0x2c] = 8;
      puVar28[0x2d] = 0;
      puVar28[0x2e] = 0;
      puVar28[0x2f] = 0;
      puVar28[0x1c] = 0;
      puVar28[0x1d] = 0;
      *(longlong **)(puVar28 + 0x26) = plVar33;
      LOCK();
      *plVar33 = *plVar33 + -1;
      UNLOCK();
      if (*plVar33 == 0) {
        *(undefined1 *)((longlong)puVar28 + 0x173) = 1;
        alloc::sync::Arc::
        drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                  ();
      }
      puVar28[0x28] = 0;
      puVar28[0x29] = 0;
      plVar33 = *(longlong **)(puVar28 + 0x56);
      lVar34 = plVar33[8];
      goto joined_r0x000140af696d;
    }
    cVar48 = '\x01';
    cVar39 = '\x01';
  }
  else {
    lVar40 = *(longlong *)(puVar28 + 0x38) * 0x40 + *(longlong *)(puVar28 + 0x36);
    if (lVar40 == 0x40 || *(longlong *)(puVar28 + 0x38) == 0) {
      puVar28[0x2a] = 0;
      puVar28[0x2b] = 0;
      puVar28[0x2c] = 8;
      puVar28[0x2d] = 0;
      puVar28[0x2e] = 0;
      puVar28[0x2f] = 0;
      puVar28[0x1c] = 0;
      puVar28[0x1d] = 0;
      *(longlong **)(puVar28 + 0x28) = plVar33;
      plVar33 = *(longlong **)(puVar28 + 0x56);
      lVar34 = plVar33[8];
joined_r0x000140af696d:
      if ((lVar34 == 0) &&
         (lVar34 = plVar33[0xb], *(longlong **)(puVar28 + 0x26) = plVar33, lVar34 == 0)) {
        LOCK();
        *plVar33 = *plVar33 + -1;
        UNLOCK();
        if (*plVar33 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                    ();
        }
        plVar33 = (longlong *)0x0;
      }
      *(undefined8 *)(puVar28 + -8) = *(undefined8 *)(puVar28 + 0x2a);
      *(undefined8 *)(puVar28 + -6) = *(undefined8 *)(puVar28 + 0x2c);
      *(undefined8 *)(puVar28 + -4) = *(undefined8 *)(puVar28 + 0x2e);
      *(undefined1 *)(puVar28 + 6) = *(undefined1 *)((longlong)puVar28 + 0x17b);
      *(undefined8 *)(puVar28 + -2) = *(undefined8 *)(puVar28 + 0x1c);
      *(undefined8 *)puVar28 = *(undefined8 *)(puVar28 + 0x1e);
      *(undefined8 *)(puVar28 + 2) = *(undefined8 *)(puVar28 + 0x28);
      *(longlong **)(puVar28 + 4) = plVar33;
      lVar34 = *(longlong *)(puVar28 + 0x38);
      if (lVar34 == *(longlong *)(puVar28 + 0x34)) {
        alloc::raw_vec::RawVec::
        grow_one<country_core::resources::window::AvailableMonitor,alloc::alloc::Global>();
      }
      lVar44 = *(longlong *)(puVar28 + 0x36);
      lVar40 = lVar34 * 0x40;
      uVar14 = *(undefined8 *)(puVar28 + -8);
      uVar17 = *(undefined8 *)(puVar28 + -6);
      uVar18 = *(undefined8 *)(puVar28 + -4);
      uVar19 = *(undefined8 *)(puVar28 + -2);
      uVar20 = *(undefined8 *)puVar28;
      uVar21 = *(undefined8 *)(puVar28 + 2);
      uVar22 = *(undefined8 *)(puVar28 + 6);
      puVar42 = (undefined8 *)(lVar44 + 0x30 + lVar40);
      *puVar42 = *(undefined8 *)(puVar28 + 4);
      puVar42[1] = uVar22;
      puVar42 = (undefined8 *)(lVar44 + 0x20 + lVar40);
      *puVar42 = uVar20;
      puVar42[1] = uVar21;
      puVar42 = (undefined8 *)(lVar44 + 0x10 + lVar40);
      *puVar42 = uVar18;
      puVar42[1] = uVar19;
      *(undefined8 *)(lVar44 + lVar40) = uVar14;
      ((undefined8 *)(lVar44 + lVar40))[1] = uVar17;
      *(longlong *)(puVar28 + 0x38) = lVar34 + 1;
      cVar48 = '\0';
      cVar39 = '\0';
    }
    else {
      plVar41 = *(longlong **)(lVar40 + -0x18);
      *(longlong **)(lVar40 + -0x18) = plVar33;
      *(longlong **)(puVar28 + -8) = plVar41;
      if (plVar41 != (longlong *)0x0) {
        LOCK();
        *plVar41 = *plVar41 + -1;
        UNLOCK();
        if (*plVar41 == 0) {
          *(undefined1 *)((longlong)puVar28 + 0x17d) = 1;
          *(undefined1 *)(puVar28 + 0x5f) = 0;
          alloc::sync::Arc::
          drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                    ();
        }
      }
      lVar34 = *(longlong *)(puVar28 + 0x56);
      lVar44 = *(longlong *)(lVar34 + 0x40);
      cVar39 = '\0';
LAB_140af6a6c:
      if ((lVar44 == 0) && (*(longlong *)(lVar34 + 0x58) == 0)) {
        cVar48 = '\x01';
      }
      else {
        plVar33 = *(longlong **)(lVar40 + -0x10);
        *(longlong *)(lVar40 + -0x10) = lVar34;
        *(longlong **)(puVar28 + -8) = plVar33;
        if (plVar33 != (longlong *)0x0) {
          LOCK();
          *plVar33 = *plVar33 + -1;
          UNLOCK();
          if (*plVar33 == 0) {
            *(undefined1 *)((longlong)puVar28 + 0x17d) = 0;
            *(char *)(puVar28 + 0x5f) = cVar39;
            alloc::sync::Arc::
            drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                      ();
          }
        }
        cVar48 = '\0';
      }
    }
  }
  *(undefined8 *)(puVar28 + 0x42) = *(undefined8 *)(puVar28 + 0x34);
  *(undefined8 *)(puVar28 + 0x40) = *(undefined8 *)(puVar28 + 0x36);
  lVar34 = *(longlong *)(puVar28 + 0x38);
  plVar33 = *(longlong **)(puVar28 + 0x22);
  LOCK();
  *plVar33 = *plVar33 + -1;
  UNLOCK();
  if (*plVar33 == 0) {
    *(char *)((longlong)puVar28 + 0x17a) = cVar48;
    *(char *)((longlong)puVar28 + 0x179) = cVar39;
    alloc::sync::Arc::
    drop_slow<slice2_<country_core::resources::walls::decorator::DecoratorId>,alloc::alloc::Global>
              ();
  }
  *(char *)((longlong)puVar28 + 0x17f) = cVar48;
  *(char *)((longlong)puVar28 + 0x17e) = cVar39;
  core::ptr::
  drop_in_place<alloc::vec::Vec<enum2_<core::option::Option<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>_>,alloc::alloc::Global>_>
            ();
  if (*(longlong *)(puVar28 + 0x4c) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  lVar44 = *(longlong *)(puVar28 + 0x3e);
  if (lVar44 != 0) {
    lVar40 = *(longlong *)(puVar28 + 0x3c) + 8;
    do {
      if (*(longlong *)(lVar40 + -8) != 0) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      lVar40 = lVar40 + 0x18;
      lVar44 = lVar44 + -1;
    } while (lVar44 != 0);
  }
  if (*(longlong *)(puVar28 + 0x3a) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  if (cVar48 != '\0') {
    plVar33 = *(longlong **)(puVar28 + 0x56);
    LOCK();
    *plVar33 = *plVar33 + -1;
    UNLOCK();
    if (*plVar33 == 0) {
      *(char *)(puVar28 + 0x5e) = cVar39;
      alloc::sync::Arc::
      drop_slow<country_core::resources::stairs::stairs_state::StairsHistoryDiff,alloc::alloc::Global>
                ();
    }
  }
  if (cVar39 != '\0') {
    plVar33 = *(longlong **)(puVar28 + 0x52);
    LOCK();
    *plVar33 = *plVar33 + -1;
    UNLOCK();
    if (*plVar33 == 0) {
      alloc::sync::Arc::
      drop_slow<country_core::resources::history::decorator_edit::DecoratorHistoryDiff,alloc::alloc::Global>
                ();
    }
  }
  *(undefined8 *)(puVar28 + 0x58) = *(undefined8 *)(puVar28 + 0x48);
  puVar28[0x5a] = 0;
  puVar28[0x5b] = 0;
  *(undefined8 *)(puVar28 + 0x32) = *(undefined8 *)(puVar28 + 0x46);
  while (*(longlong *)(puVar28 + 0x58) != *(longlong *)(puVar28 + 0x5a)) {
    *(longlong *)(puVar28 + 0x5a) = *(longlong *)(puVar28 + 0x5a) + 1;
    core::ptr::
    drop_in_place<enum2_<country_core::resources::history::replay_cmd_buffer::ReplayCmd>_>();
  }
  if (*(longlong *)(puVar28 + 0x44) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  plVar33 = *(longlong **)(puVar28 + 0x40);
  plVar41 = plVar33 + lVar34 * 8;
  *(longlong **)(puVar28 + 0x2a) = plVar33;
  *(longlong **)(puVar28 + 0x2c) = plVar33;
  *(undefined8 *)(puVar28 + 0x2e) = *(undefined8 *)(puVar28 + 0x42);
  *(longlong **)(puVar28 + 0x30) = plVar41;
  if (lVar34 != 0) {
    plVar33 = *(longlong **)(puVar28 + 0x54);
    uVar4 = *(undefined4 *)((longlong)plVar33 + 0x1c);
    puVar36 = (ulonglong *)*plVar33;
    puVar10 = (undefined4 *)plVar33[2];
    plVar16 = *(longlong **)(puVar28 + 0x40);
    do {
      plVar38 = plVar16 + 8;
      lVar34 = *plVar16;
      plVar33 = plVar38;
      if (SBORROW8(0,lVar34)) break;
      *(longlong **)(puVar28 + 0x54) = plVar38;
      *(longlong *)(puVar28 + 6) = plVar16[7];
      lVar44 = plVar16[1];
      lVar40 = plVar16[2];
      lVar23 = plVar16[3];
      lVar24 = plVar16[4];
      lVar25 = plVar16[6];
      *(longlong *)(puVar28 + 2) = plVar16[5];
      *(longlong *)(puVar28 + 4) = lVar25;
      *(longlong *)(puVar28 + -2) = lVar23;
      *(longlong *)puVar28 = lVar24;
      *(longlong *)(puVar28 + -6) = lVar44;
      *(longlong *)(puVar28 + -4) = lVar40;
      *puVar10 = uVar4;
      *(longlong *)(puVar28 + -8) = lVar34;
      puVar36[4] = puVar36[4] + 1;
      uVar29 = *puVar36;
      uVar32 = puVar36[3];
      if (uVar32 == uVar29) {
        alloc::collections::vec_deque::VecDeque::
        grow<country_core::resources::history::replay_cmd_buffer::ReplayBucket,alloc::alloc::Global>
                  ();
        uVar29 = *puVar36;
        uVar32 = puVar36[3];
      }
      if (uVar32 + puVar36[2] < uVar29) {
        uVar29 = 0;
      }
      uVar47 = puVar36[1];
      lVar34 = ((uVar32 + puVar36[2]) - uVar29) * 0x40;
      uVar14 = *(undefined8 *)(puVar28 + -8);
      uVar17 = *(undefined8 *)(puVar28 + -6);
      uVar18 = *(undefined8 *)(puVar28 + -4);
      uVar19 = *(undefined8 *)(puVar28 + -2);
      uVar20 = *(undefined8 *)puVar28;
      uVar21 = *(undefined8 *)(puVar28 + 2);
      uVar22 = *(undefined8 *)(puVar28 + 6);
      puVar42 = (undefined8 *)(uVar47 + 0x30 + lVar34);
      *puVar42 = *(undefined8 *)(puVar28 + 4);
      puVar42[1] = uVar22;
      puVar42 = (undefined8 *)(uVar47 + 0x20 + lVar34);
      *puVar42 = uVar20;
      puVar42[1] = uVar21;
      puVar42 = (undefined8 *)(uVar47 + 0x10 + lVar34);
      *puVar42 = uVar18;
      puVar42[1] = uVar19;
      *(undefined8 *)(uVar47 + lVar34) = uVar14;
      ((undefined8 *)(uVar47 + lVar34))[1] = uVar17;
      puVar36[3] = puVar36[3] + 1;
      plVar16 = *(longlong **)(puVar28 + 0x54);
      plVar33 = plVar41;
    } while (plVar38 != plVar41);
    *(longlong **)(puVar28 + 0x2c) = plVar33;
  }
  *(ulonglong *)(puVar28 + 0x58) = (ulonglong)((longlong)plVar41 - (longlong)plVar33) >> 6;
  puVar28[0x5a] = 0;
  puVar28[0x5b] = 0;
  *(longlong **)(puVar28 + 0x54) = plVar33;
  while (*(longlong *)(puVar28 + 0x58) != *(longlong *)(puVar28 + 0x5a)) {
    *(longlong *)(puVar28 + 0x5a) = *(longlong *)(puVar28 + 0x5a) + 1;
    core::ptr::drop_in_place<country_core::resources::history::replay_cmd_buffer::ReplayBucket>();
  }
  if (*(longlong *)(puVar28 + 0x42) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  return;
}

