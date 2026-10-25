// 0x1407adea0
// callees: [{"name": "as_inner_mut", "address": "14067dd80"}, {"name": "decorator_info", "address": "140a1c5f0"}, {"name": "into_drag_state", "address": "14092df10"}, {"name": "is_active", "address": "14092e680"}, {"name": "lookup_storage", "address": "140a1c3e0"}, {"name": "unwrap_failed", "address": "1428d9310"}]

void __cdecl system_decorator::ui_systems::ui_change_stair_height::ui_change_stair_height(void)

{
  ulonglong *puVar1;
  char extraout_AL;
  byte extraout_AL_00;
  longlong extraout_RAX;
  longlong *in_RDX;
  ulonglong uVar2;
  longlong lVar3;
  longlong *in_stack_00000030;
  
  puVar1 = (ulonglong *)*in_RDX;
  uVar2 = *puVar1 - 2;
  lVar3 = (puVar1[1] - 1) + (ulonglong)(1 < *puVar1);
  if (((lVar3 == 0 && (ulonglong)(4 < uVar2) <= (ulonglong)-lVar3) && (uVar2 == 2)) &&
     (*(char *)((longlong)puVar1 + 0x5c) == '?')) {
    toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut();
    country_core::systems::ui::resources::ui_state::UiWidgetContext::is_active();
    if (extraout_AL != '\0') {
      country_core::systems::ui::resources::slider3d::impl$1::into_drag_state();
      if (extraout_AL_00 < 2) {
        *(undefined4 *)in_stack_00000030[2] = *(undefined4 *)((longlong)in_stack_00000030 + 0x1c);
        *(undefined1 *)(*in_stack_00000030 + 0x48) = 1;
      }
      country_core::resources::walls::decorator_storage::DecoratorStorage::lookup_storage();
      country_core::resources::walls::decorator_storage::DecoratorStorage::decorator_info();
      if (extraout_RAX != 0) {
        lVar3 = 0;
        if ((byte)(*(byte *)(extraout_RAX + 0x22) - 2) < 3) {
          lVar3 = (ulonglong)*(byte *)(extraout_RAX + 0x22) - 1;
        }
                    // WARNING: Could not emulate address calculation at 0x0001407adfd2
                    // WARNING: Treating indirect jump as call
        (*(code *)((longlong)&DAT_142a832b0 + (longlong)(int)(&DAT_142a832b0)[lVar3]))
                  ((code *)((longlong)&DAT_142a832b0 + (longlong)(int)(&DAT_142a832b0)[lVar3]));
        return;
      }
                    // WARNING: Subroutine does not return
      core::option::unwrap_failed();
    }
  }
  return;
}

