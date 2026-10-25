// all_storages  size=105
// void __cdecl all_storages(void)

void __cdecl
country_core::resources::walls::decorator_archivist::DecoratorArchivist::all_storages(void)

{
  undefined1 auVar1 [16];
  undefined1 (*pauVar2) [16];
  longlong lVar3;
  longlong lVar4;
  longlong lVar5;
  longlong *in_RCX;
  undefined8 *in_RDX;
  
  pauVar2 = (undefined1 (*) [16])*in_RDX;
  lVar3 = in_RDX[1];
  auVar1 = *pauVar2;
  lVar4 = in_RDX[3];
  lVar5 = in_RDX[0x2a];
  in_RCX[2] = (ulonglong)(in_RDX[0x18] == 0) * 2 + 1;
  in_RCX[4] = (longlong)pauVar2;
  in_RCX[5] = (longlong)(pauVar2 + 1);
  in_RCX[6] = (longlong)(*pauVar2 + lVar3 + 1);
  *(ushort *)(in_RCX + 7) =
       ~((ushort)(SUB161(auVar1 >> 7,0) & 1) | (ushort)(SUB161(auVar1 >> 0xf,0) & 1) << 1 |
         (ushort)(SUB161(auVar1 >> 0x17,0) & 1) << 2 | (ushort)(SUB161(auVar1 >> 0x1f,0) & 1) << 3 |
         (ushort)(SUB161(auVar1 >> 0x27,0) & 1) << 4 | (ushort)(SUB161(auVar1 >> 0x2f,0) & 1) << 5 |
         (ushort)(SUB161(auVar1 >> 0x37,0) & 1) << 6 | (ushort)(SUB161(auVar1 >> 0x3f,0) & 1) << 7 |
         (ushort)(SUB161(auVar1 >> 0x47,0) & 1) << 8 | (ushort)(SUB161(auVar1 >> 0x4f,0) & 1) << 9 |
         (ushort)(SUB161(auVar1 >> 0x57,0) & 1) << 10 |
         (ushort)(SUB161(auVar1 >> 0x5f,0) & 1) << 0xb |
         (ushort)(SUB161(auVar1 >> 0x67,0) & 1) << 0xc |
         (ushort)(SUB161(auVar1 >> 0x6f,0) & 1) << 0xd |
         (ushort)(SUB161(auVar1 >> 0x77,0) & 1) << 0xe | (ushort)(byte)(auVar1[0xf] >> 7) << 0xf);
  in_RCX[8] = lVar4;
  *in_RCX = (ulonglong)(lVar5 == 0) + 2;
  return;
}



// CALLEES: []