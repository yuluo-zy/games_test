
void __cdecl
country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary::
insert_collision_and_interaction_as_same(void)

{
  longlong lVar1;
  void *_Src;
  longlong *plVar2;
  undefined1 auVar3 [16];
  code *pcVar4;
  longlong lVar5;
  longlong lVar6;
  longlong *plVar7;
  ulonglong _Size;
  undefined1 *extraout_RAX;
  longlong in_R9;
  undefined1 *_Dst;
  int local_98;
  
  auVar3._8_8_ = 0;
  auVar3._0_8_ = *(ulonglong *)(in_R9 + 0x10);
  _Size = SUB168(auVar3 * ZEXT816(0x24),0);
  if ((SUB168(auVar3 * ZEXT816(0x24),8) == 0) && (_Size < 0x7ffffffffffffffd)) {
    _Src = *(void **)(in_R9 + 8);
    if (_Size == 0) {
      _Dst = &DAT_00000004;
    }
    else {
      std::alloc::__default_lib_allocator::__rust_alloc();
      _Dst = extraout_RAX;
      if (extraout_RAX == (undefined1 *)0x0) goto LAB_140b261d8;
    }
    memcpy(_Dst,_Src,_Size);
    lVar5 = *(longlong *)(in_R9 + 0x20);
    lVar6 = *(longlong *)(in_R9 + 0x28);
    plVar7 = *(longlong **)(in_R9 + 0x38);
    plVar2 = *(longlong **)(in_R9 + 0x18);
    LOCK();
    lVar1 = *plVar2;
    *plVar2 = *plVar2 + 1;
    UNLOCK();
    if (*plVar2 != 0 && SCARRY8(lVar1,1) == *plVar2 < 0) {
      hashbrown::map::HashMap::
      insert<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (!SBORROW8(0,lVar5)) {
        if (lVar5 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        LOCK();
        *plVar7 = *plVar7 + -1;
        UNLOCK();
        if (*plVar7 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      hashbrown::map::HashMap::
      insert<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if (!SBORROW8(0,lVar5)) {
        if (lVar5 != 0) {
          std::alloc::__default_lib_allocator::__rust_dealloc();
        }
        LOCK();
        *plVar7 = *plVar7 + -1;
        UNLOCK();
        if (*plVar7 == 0) {
          alloc::sync::Arc::
          drop_slow<country_core::systems::collision::world::RaycastMeshData,alloc::alloc::Global>()
          ;
        }
      }
      hashbrown::map::HashMap::
      insert<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2_<country_core::resources::walls::decorator_outline::DecoratorOutline>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      local_98 = (int)lVar5;
      if (((local_98 == 1) || (local_98 == 0)) && (lVar6 != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      hashbrown::map::HashMap::
      insert<enum2_<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2_<country_core::resources::walls::decorator_outline::DecoratorCollisionOutline>,std::hash::random::RandomState,alloc::alloc::Global>
                ();
      if ((-0x7ffffffffffffffe < lVar5) && (lVar5 != 0)) {
        std::alloc::__default_lib_allocator::__rust_dealloc();
      }
      return;
    }
                    /* WARNING: Does not return */
    pcVar4 = (code *)invalidInstructionException();
    (*pcVar4)();
  }
LAB_140b261d8:
                    /* WARNING: Subroutine does not return */
  alloc::raw_vec::handle_error();
}

