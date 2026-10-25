1408bb650: push rsi
1408bb651: push rdi
1408bb652: sub rsp, 0x1d8
1408bb659: mov rsi, rcx
1408bb65c: xor edi, edi
1408bb65e: lea rcx, [rsp + 0x28]
1408bb663: mov r8d, 0x1b0
1408bb669: xor edx, edx
1408bb66b: call 0x142923c20
1408bb670: lea rax, [rip + 0x21e5fa1]
1408bb677: xor ecx, ecx
1408bb679: jmp 0x1408bb6b8
1408bb67b: mov dl, 5
1408bb67d: nop dword ptr [rax]
1408bb680: shl dil, 2
1408bb684: movzx r8d, dil
1408bb688: mov dword ptr [rsp + r8*4 + 0x28], ecx
1408bb68d: lea r9d, [rcx + 0x2000]
1408bb694: mov dword ptr [rsp + r8*4 + 0x2c], r9d
1408bb699: lea r9d, [rcx + 0x4000]
1408bb6a0: mov dword ptr [rsp + r8*4 + 0x30], r9d
1408bb6a5: add ecx, 0x6000
1408bb6ab: mov dword ptr [rsp + r8*4 + 0x34], ecx
1408bb6b0: add ecx, 0x2000
1408bb6b6: mov edi, edx
1408bb6b8: movzx edx, dil
1408bb6bc: movsxd rdx, dword ptr [rax + rdx*4]
1408bb6c0: add rdx, rax
1408bb6c3: jmp rdx
1408bb6c5: mov dl, 1
1408bb6c7: jmp 0x1408bb680
1408bb6c9: shl dil, 2
1408bb6cd: movzx edx, dil
1408bb6d1: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb6d5: lea r8d, [rcx + 0x1000]
1408bb6dc: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb6e1: lea r8d, [rcx + 0x2000]
1408bb6e8: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb6ed: add ecx, 0x3000
1408bb6f3: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb6f7: mov dil, 0x11
1408bb6fa: add ecx, 0x1000
1408bb700: jmp 0x1408bb6b8
1408bb702: shl dil, 2
1408bb706: movzx edx, dil
1408bb70a: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb70e: lea r8d, [rcx + 0x200]
1408bb715: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb71a: lea r8d, [rcx + 0x400]
1408bb721: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb726: add ecx, 0x600
1408bb72c: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb730: mov dil, 0xf
1408bb733: add ecx, 0x200
1408bb739: jmp 0x1408bb6b8
1408bb73e: shl dil, 2
1408bb742: movzx edx, dil
1408bb746: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb74a: lea r8d, [rcx + 0xcf80]
1408bb751: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb756: lea r8d, [rcx + 0x19f00]
1408bb75d: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb762: add ecx, 0x26e80
1408bb768: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb76c: mov dil, 0xc
1408bb76f: add ecx, 0xcf80
1408bb775: jmp 0x1408bb6b8
1408bb77a: shl dil, 2
1408bb77e: movzx edx, dil
1408bb782: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb786: lea r8d, [rcx + 0x2580]
1408bb78d: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb792: lea r8d, [rcx + 0x4b00]
1408bb799: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb79e: add ecx, 0x7080
1408bb7a4: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb7a8: mov dil, 0xd
1408bb7ab: add ecx, 0x2580
1408bb7b1: jmp 0x1408bb6b8
1408bb7b6: shl dil, 2
1408bb7ba: movzx edx, dil
1408bb7be: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb7c2: lea r8d, [rcx + 0xa000]
1408bb7c9: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb7ce: lea r8d, [rcx + 0x14000]
1408bb7d5: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb7da: add ecx, 0x1e000
1408bb7e0: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb7e4: mov dil, 0xa
1408bb7e7: add ecx, 0xa000
1408bb7ed: jmp 0x1408bb6b8
1408bb7f2: shl dil, 2
1408bb7f6: movzx edx, dil
1408bb7fa: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb7fe: lea r8d, [rcx + 0x100]
1408bb805: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb80a: lea r8d, [rcx + 0x200]
1408bb811: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb816: add ecx, 0x300
1408bb81c: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb820: mov dil, 0x1a
1408bb823: add ecx, 0x100
1408bb829: jmp 0x1408bb6b8
1408bb82e: shl dil, 2
1408bb832: movzx edx, dil
1408bb836: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb83a: lea r8d, [rcx + 0x14000]
1408bb841: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb846: lea r8d, [rcx + 0x28000]
1408bb84d: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb852: add ecx, 0x3c000
1408bb858: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb85c: mov dil, 0x19
1408bb85f: add ecx, 0x14000
1408bb865: jmp 0x1408bb6b8
1408bb86a: shl dil, 2
1408bb86e: movzx edx, dil
1408bb872: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb876: lea r8d, [rcx + 0x1000]
1408bb87d: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb882: lea r8d, [rcx + 0x2000]
1408bb889: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb88e: add ecx, 0x3000
1408bb894: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb898: mov dil, 3
1408bb89b: add ecx, 0x1000
1408bb8a1: jmp 0x1408bb6b8
1408bb8a6: mov dl, 0x10
1408bb8a8: jmp 0x1408bb680
1408bb8ad: shl dil, 2
1408bb8b1: movzx edx, dil
1408bb8b5: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb8b9: lea r8d, [rcx + 0x8000]
1408bb8c0: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb8c5: lea r8d, [rcx + 0x10000]
1408bb8cc: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb8d1: add ecx, 0x18000
1408bb8d7: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb8db: mov dil, 4
1408bb8de: add ecx, 0x8000
1408bb8e4: jmp 0x1408bb6b8
1408bb8e9: mov dl, 8
1408bb8eb: jmp 0x1408bb680
1408bb8f0: mov dl, 2
1408bb8f2: jmp 0x1408bb680
1408bb8f7: shl dil, 2
1408bb8fb: movzx edx, dil
1408bb8ff: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb903: lea r8d, [rcx + 0x400]
1408bb90a: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb90f: lea r8d, [rcx + 0x800]
1408bb916: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb91b: add ecx, 0xc00
1408bb921: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb925: mov dil, 0x12
1408bb928: add ecx, 0x400
1408bb92e: jmp 0x1408bb6b8
1408bb933: shl dil, 2
1408bb937: movzx edx, dil
1408bb93b: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb93f: lea r8d, [rcx + 0x800]
1408bb946: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb94b: lea r8d, [rcx + 0x1000]
1408bb952: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb957: add ecx, 0x1800
1408bb95d: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb961: mov dil, 0x15
1408bb964: add ecx, 0x800
1408bb96a: jmp 0x1408bb6b8
1408bb96f: mov dl, 6
1408bb971: jmp 0x1408bb680
1408bb976: shl dil, 2
1408bb97a: movzx edx, dil
1408bb97e: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb982: lea r8d, [rcx + 0x400]
1408bb989: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb98e: lea r8d, [rcx + 0x800]
1408bb995: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb99a: add ecx, 0xc00
1408bb9a0: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb9a4: mov dil, 0x16
1408bb9a7: add ecx, 0x400
1408bb9ad: jmp 0x1408bb6b8
1408bb9b2: shl dil, 2
1408bb9b6: movzx edx, dil
1408bb9ba: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bb9be: lea r8d, [rcx + 0x20000]
1408bb9c5: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bb9ca: lea r8d, [rcx + 0x40000]
1408bb9d1: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bb9d6: add ecx, 0x60000
1408bb9dc: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bb9e0: mov dil, 0xe
1408bb9e3: add ecx, 0x20000
1408bb9e9: jmp 0x1408bb6b8
1408bb9ee: mov dl, 7
1408bb9f0: jmp 0x1408bb680
1408bb9f5: shl dil, 2
1408bb9f9: movzx edx, dil
1408bb9fd: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bba01: lea r8d, [rcx + 0x8000]
1408bba08: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bba0d: lea r8d, [rcx + 0x10000]
1408bba14: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bba19: add ecx, 0x18000
1408bba1f: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bba23: mov dil, 0x13
1408bba26: add ecx, 0x8000
1408bba2c: jmp 0x1408bb6b8
1408bba31: shl dil, 2
1408bba35: movzx edx, dil
1408bba39: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bba3d: lea r8d, [rcx + 0x400]
1408bba44: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bba49: lea r8d, [rcx + 0x800]
1408bba50: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bba55: add ecx, 0xc00
1408bba5b: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bba5f: mov dil, 0xb
1408bba62: add ecx, 0x400
1408bba68: jmp 0x1408bb6b8
1408bba6d: shl dil, 2
1408bba71: movzx edx, dil
1408bba75: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bba79: lea r8d, [rcx + 0x40000]
1408bba80: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bba85: lea r8d, [rcx + 0x80000]
1408bba8c: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bba91: add ecx, 0xc0000
1408bba97: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bba9b: mov dil, 9
1408bba9e: add ecx, 0x40000
1408bbaa4: jmp 0x1408bb6b8
1408bbaa9: shl dil, 2
1408bbaad: movzx edx, dil
1408bbab1: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bbab5: lea r8d, [rcx + 0x800]
1408bbabc: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bbac1: lea r8d, [rcx + 0x1000]
1408bbac8: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bbacd: add ecx, 0x1800
1408bbad3: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bbad7: mov dil, 0x17
1408bbada: add ecx, 0x800
1408bbae0: jmp 0x1408bb6b8
1408bbae5: shl dil, 2
1408bbae9: movzx edx, dil
1408bbaed: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bbaf1: lea r8d, [rcx + 0x10000]
1408bbaf8: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bbafd: lea r8d, [rcx + 0x20000]
1408bbb04: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bbb09: add ecx, 0x30000
1408bbb0f: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bbb13: mov dil, 0x14
1408bbb16: add ecx, 0x10000
1408bbb1c: jmp 0x1408bb6b8
1408bbb21: shl dil, 2
1408bbb25: movzx edx, dil
1408bbb29: mov dword ptr [rsp + rdx*4 + 0x28], ecx
1408bbb2d: lea r8d, [rcx + 0x20000]
1408bbb34: mov dword ptr [rsp + rdx*4 + 0x2c], r8d
1408bbb39: lea r8d, [rcx + 0x40000]
1408bbb40: mov dword ptr [rsp + rdx*4 + 0x30], r8d
1408bbb45: add ecx, 0x60000
1408bbb4b: mov dword ptr [rsp + rdx*4 + 0x34], ecx
1408bbb4f: mov dil, 0x18
1408bbb52: add ecx, 0x20000
1408bbb58: jmp 0x1408bb6b8
1408bbb5d: mov dword ptr [rsp + 0x1c8], ecx
1408bbb64: lea edx, [rcx + 0x8000]
1408bbb6a: mov dword ptr [rsp + 0x1cc], edx
1408bbb71: lea edx, [rcx + 0x10000]
1408bbb77: mov dword ptr [rsp + 0x1d0], edx
1408bbb7e: add ecx, 0x18000
1408bbb84: mov dword ptr [rsp + 0x1d4], ecx
1408bbb8b: mov dil, 0x1b
1408bbb8e: add ecx, 0x8000
1408bbb94: jmp 0x1408bb6b8
1408bbb99: lea rcx, [rsi + 4]
1408bbb9d: lea rdx, [rsp + 0x28]
1408bbba2: mov r8d, 0x1b0
1408bbba8: call 0x142923c00
1408bbbad: mov dword ptr [rsi], 1
1408bbbb3: mov rax, rsi
1408bbbb6: add rsp, 0x1d8
1408bbbbd: pop rdi
1408bbbbe: pop rsi
1408bbbbf: ret 
