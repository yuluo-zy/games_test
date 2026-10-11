
void __cdecl country_core::components::transient_mesh::TransientMesh::new_with_registry(void)

{
  longlong lVar1;
  longlong *plVar2;
  code *pcVar3;
  undefined8 *in_RDX;
  
  plVar2 = (longlong *)*in_RDX;
  LOCK();
  lVar1 = *plVar2;
  *plVar2 = *plVar2 + 1;
  UNLOCK();
  if (*plVar2 != 0 && SCARRY8(lVar1,1) == *plVar2 < 0) {
    return;
  }
                    /* WARNING: Does not return */
  pcVar3 = (code *)invalidInstructionException();
  (*pcVar3)();
}

