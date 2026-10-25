1412bc6f0: push rbp
1412bc6f1: push rsi
1412bc6f2: sub rsp, 0xd8
1412bc6f9: lea rbp, [rsp + 0x80]
1412bc701: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bc709: mov rsi, rcx
1412bc70c: lea rcx, [rbp - 0x58]
1412bc710: xor r8d, r8d
1412bc713: call 0x141611000
1412bc718: movzx eax, byte ptr [rip + 0x23784ea]
1412bc71f: mov ecx, 0x28
1412bc724: mov edx, 8
1412bc729: call 0x140613c10
1412bc72e: test rax, rax
1412bc731: je 0x1412bc7e7
1412bc737: mov rcx, qword ptr [rbp - 0x38]
1412bc73b: mov qword ptr [rax + 0x20], rcx
1412bc73f: movups xmm0, xmmword ptr [rbp - 0x58]
1412bc743: movups xmm1, xmmword ptr [rbp - 0x48]
1412bc747: movups xmmword ptr [rax + 0x10], xmm1
1412bc74b: movups xmmword ptr [rax], xmm0
1412bc74e: lea rcx, [rip + 0x17e084f]
1412bc755: mov qword ptr [rbp], rcx
1412bc759: mov qword ptr [rbp + 8], 0x46
1412bc761: mov qword ptr [rbp + 0x10], rax
1412bc765: lea rax, [rip + 0x196fe84]
1412bc76c: mov qword ptr [rbp + 0x18], rax
1412bc770: mov qword ptr [rbp + 0x30], 0
1412bc778: mov byte ptr [rbp + 0x46], 5
1412bc77c: mov qword ptr [rbp + 0x20], rcx
1412bc780: mov qword ptr [rbp + 0x28], 0x46
1412bc788: movzx eax, byte ptr [rip + 0x237847a]
1412bc78f: mov ecx, 0x40
1412bc794: mov edx, 8
1412bc799: call 0x140613c10
1412bc79e: test rax, rax
1412bc7a1: je 0x1412bc7f8
1412bc7a3: movups xmm0, xmmword ptr [rbp + 0x10]
1412bc7a7: movups xmm1, xmmword ptr [rbp + 0x20]
1412bc7ab: movups xmm2, xmmword ptr [rbp + 0x30]
1412bc7af: movups xmm3, xmmword ptr [rbp + 0x40]
1412bc7b3: movups xmmword ptr [rax + 0x30], xmm3
1412bc7b7: movups xmmword ptr [rax + 0x20], xmm2
1412bc7bb: movups xmmword ptr [rax + 0x10], xmm1
1412bc7bf: movups xmmword ptr [rax], xmm0
1412bc7c2: movabs rdx, 0xe62e51063c1df811
1412bc7cc: mov rcx, rsi
1412bc7cf: mov r8, rax
1412bc7d2: call 0x140969bf0
1412bc7d7: cmp rax, 1
1412bc7db: je 0x1412bc809
1412bc7dd: add rsp, 0xd8
1412bc7e4: pop rsi
1412bc7e5: pop rbp
1412bc7e6: ret 
1412bc7e7: mov ecx, 8
1412bc7ec: mov edx, 0x28
1412bc7f1: call 0x1428d9000
1412bc7f6: jmp 0x1412bc807
1412bc7f8: mov ecx, 8
1412bc7fd: mov edx, 0x40
1412bc802: call 0x1428d9000
1412bc807: ud2 
1412bc809: mov rax, rdx
1412bc80c: add rdx, 0x10
1412bc810: mov rsi, rdx
1412bc813: cmp qword ptr [rax + 0x18], 0x46
1412bc818: jne 0x1412bc896
1412bc81a: mov rdx, qword ptr [rsi]
1412bc81d: lea rcx, [rip + 0x17e0780]
1412bc824: mov r8d, 0x46
1412bc82a: call 0x142923bf0
1412bc82f: test eax, eax
1412bc831: jne 0x1412bc896
1412bc833: lea rax, [rip + 0x1971826]
1412bc83a: mov qword ptr [rbp - 0x30], rax
1412bc83e: lea rax, [rip - 0xd88fa5]
1412bc845: mov qword ptr [rbp - 0x28], rax
1412bc849: mov rax, rbp
1412bc84c: mov qword ptr [rbp - 0x20], rax
1412bc850: lea rax, [rip - 0x11b29c7]
1412bc857: mov qword ptr [rbp - 0x18], rax
1412bc85b: lea rax, [rip + 0x1970dc6]
1412bc862: mov qword ptr [rbp + 0x10], rax
1412bc866: mov qword ptr [rbp + 0x18], 3
1412bc86e: mov qword ptr [rbp + 0x30], 0
1412bc876: lea rax, [rbp - 0x30]
1412bc87a: mov qword ptr [rbp + 0x20], rax
1412bc87e: mov qword ptr [rbp + 0x28], 2
1412bc886: lea rdx, [rip + 0x1970dcb]
1412bc88d: lea rcx, [rbp + 0x10]
1412bc891: call 0x1428d9390
1412bc896: lea rax, [rip + 0x19717c3]
1412bc89d: mov qword ptr [rbp + 0x10], rax
1412bc8a1: lea rax, [rip - 0xd89008]
1412bc8a8: mov qword ptr [rbp + 0x18], rax
1412bc8ac: mov qword ptr [rbp + 0x20], rsi
1412bc8b0: lea rax, [rip - 0x11b2a27]
1412bc8b7: mov qword ptr [rbp + 0x28], rax
1412bc8bb: mov rcx, rbp
1412bc8be: mov qword ptr [rbp + 0x30], rcx
1412bc8c2: mov qword ptr [rbp + 0x38], rax
1412bc8c6: lea rax, [rip + 0x1970c9b]
1412bc8cd: mov qword ptr [rbp - 0x30], rax
1412bc8d1: mov qword ptr [rbp - 0x28], 3
1412bc8d9: mov qword ptr [rbp - 0x10], 0
1412bc8e1: lea rax, [rbp + 0x10]
1412bc8e5: mov qword ptr [rbp - 0x20], rax
1412bc8e9: mov qword ptr [rbp - 0x18], 3
1412bc8f1: lea rdx, [rip + 0x1970cd8]
1412bc8f8: lea rcx, [rbp - 0x30]
1412bc8fc: call 0x1428d9390
1412bc901: int3 
1412bc902: nop word ptr cs:[rax + rax]
1412bc910: mov qword ptr [rsp + 0x10], rdx
1412bc915: push rbp
1412bc916: push rsi
1412bc917: sub rsp, 0x28
1412bc91b: lea rbp, [rdx + 0x80]
1412bc922: lea rcx, [rbp - 0x58]
1412bc926: call 0x140be24b0
1412bc92b: nop 
1412bc92c: add rsp, 0x28
1412bc930: pop rsi
1412bc931: pop rbp
1412bc932: ret 
1412bc933: nop word ptr cs:[rax + rax]
1412bc940: mov qword ptr [rsp + 0x10], rdx
1412bc945: push rbp
1412bc946: push rsi
1412bc947: sub rsp, 0x28
1412bc94b: lea rbp, [rdx + 0x80]
1412bc952: lea rcx, [rbp + 0x10]
1412bc956: call 0x1400c04b0
1412bc95b: nop 
1412bc95c: add rsp, 0x28
1412bc960: pop rsi
1412bc961: pop rbp
1412bc962: ret 
