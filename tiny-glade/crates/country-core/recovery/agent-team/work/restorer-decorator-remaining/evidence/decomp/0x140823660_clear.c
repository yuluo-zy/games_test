// 0x140823660 clear
// sig: void __cdecl clear(void)
// callees: [{"name": "__rust_dealloc", "address": "140613c20"}, {"name": "clone", "address": "1408335a0"}, {"name": "clone_from", "address": "140a1b760"}, {"name": "clone_from<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>", "address": "141263210"}, {"name": "clone_from<tuple$<country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform>,alloc::alloc::Global>", "address": "141262b00"}, {"name": "clone_from<tuple$<tuple$<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple$<>_>,alloc::alloc::Global>", "address": "141262fb0"}, {"name": "default", "address": "140824680"}, {"name": "drop_in_place<system_decorator::backup::DecoratorBackup>", "address": "1405f7140"}, {"name": "spec_extend<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2$<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>_>,alloc::alloc::Global>", "address": "14125e710"}]

void __cdecl system_decorator::backup::DecoratorBackup::clear(void)

{
  longlong lVar1;
  undefined8 uVar2;
  longlong in_RCX;
  undefined8 uVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong local_2d8;
  ulonglong local_2d0;
  undefined8 local_f0;
  undefined8 uStack_e8;
  undefined8 local_c0;
  undefined8 uStack_b8;
  undefined8 local_90;
  undefined8 uStack_88;
  undefined1 local_60;
  undefined1 local_5f;
  undefined1 local_5e;
  undefined8 local_58;
  undefined8 uStack_50;
  undefined8 local_48;
  undefined8 uStack_40;
  
  backup::impl$3::default();
  country_core::resources::walls::decorator_storage::impl$12::clone_from();
  hashbrown::raw::impl$13::
  clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global>_>,alloc::alloc::Global>
            ();
  *(undefined8 *)(in_RCX + 0x1f0) = local_f0;
  *(undefined8 *)(in_RCX + 0x1f8) = uStack_e8;
  hashbrown::raw::impl$13::
  clone_from<tuple_<country_core::resources::walls::decorator::DecoratorId,utils::components::transform::Transform>,alloc::alloc::Global>
            ();
  *(undefined8 *)(in_RCX + 0x220) = local_c0;
  *(undefined8 *)(in_RCX + 0x228) = uStack_b8;
  hashbrown::raw::impl$13::
  clone_from<tuple_<tuple_<country_core::resources::walls::decorator::DecoratorId,country_core::resources::walls::decorator::DecoratorId>,tuple_<>_>,alloc::alloc::Global>
            ();
  *(undefined8 *)(in_RCX + 0x250) = local_90;
  *(undefined8 *)(in_RCX + 600) = uStack_88;
  country_core::resources::walls::decorator::impl$12::clone();
  if (*(longlong *)(in_RCX + 0x268) != 0) {
    std::alloc::__default_lib_allocator::__rust_dealloc();
  }
  *(undefined8 *)(in_RCX + 0x270) = local_48;
  *(undefined8 *)(in_RCX + 0x278) = uStack_40;
  *(undefined8 *)(in_RCX + 0x260) = local_58;
  *(undefined8 *)(in_RCX + 0x268) = uStack_50;
  uVar5 = *(ulonglong *)(in_RCX + 0x10);
  if (local_2d0 <= *(ulonglong *)(in_RCX + 0x10)) {
    *(ulonglong *)(in_RCX + 0x10) = local_2d0;
    uVar5 = local_2d0;
  }
  if (uVar5 != 0) {
    lVar1 = *(longlong *)(in_RCX + 8);
    lVar4 = 8;
    do {
      uVar2 = *(undefined8 *)(local_2d8 + -8 + lVar4);
      uVar3 = 3;
      switch(uVar2) {
      case 3:
        break;
      case 4:
        uVar3 = 4;
        break;
      case 5:
        uVar3 = 5;
        break;
      default:
        local_48 = *(undefined8 *)(local_2d8 + 0x10 + lVar4);
        local_58 = *(undefined8 *)(local_2d8 + lVar4);
        uStack_50 = ((undefined8 *)(local_2d8 + lVar4))[1];
        uVar3 = uVar2;
      }
      *(undefined8 *)(lVar1 + -8 + lVar4) = uVar3;
      *(undefined8 *)(lVar1 + lVar4) = local_58;
      ((undefined8 *)(lVar1 + lVar4))[1] = uStack_50;
      *(undefined8 *)(lVar1 + 0x10 + lVar4) = local_48;
      lVar4 = lVar4 + 0x20;
      uVar5 = uVar5 - 1;
    } while (uVar5 != 0);
  }
  alloc::vec::spec_extend::impl$3::
  spec_extend<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>,core::slice::iter::Iter<enum2_<country_core::resources::history::decorator_edit::DecoratorHistoryEdit>_>,alloc::alloc::Global>
            ();
  *(undefined1 *)(in_RCX + 0x282) = local_5e;
  *(undefined1 *)(in_RCX + 0x280) = local_60;
  *(undefined1 *)(in_RCX + 0x281) = local_5f;
  core::ptr::drop_in_place<system_decorator::backup::DecoratorBackup>();
  return;
}

