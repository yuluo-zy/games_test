1415014a0: push rsi
1415014a1: push rdi
1415014a2: push rbx
1415014a3: sub rsp, 0x1b0
1415014aa: mov rsi, rdx
1415014ad: mov rdi, rcx
1415014b0: call 0x1413efa40
1415014b5: mov ebx, 1
1415014ba: lock xadd dword ptr [rsi + 0x5b0], ebx
1415014c2: xor eax, eax
1415014c4: cmp rax, qword ptr [rdi + 0x160]
1415014cb: jo 0x141501606
1415014d1: lea rdx, [rdi + 0x138]
1415014d8: mov dword ptr [rsp + 0x20], ebx
1415014dc: lea rcx, [rsp + 0xf0]
1415014e4: mov r8, rdi
1415014e7: mov r9, rsi
1415014ea: call 0x141507230
1415014ef: movups xmm0, xmmword ptr [rsp + 0xf0]
1415014f7: movups xmm1, xmmword ptr [rsp + 0x100]
1415014ff: movups xmm2, xmmword ptr [rsp + 0x110]
141501507: movups xmm3, xmmword ptr [rsp + 0x120]
14150150f: movaps xmmword ptr [rsp + 0x40], xmm1
141501514: movaps xmmword ptr [rsp + 0x30], xmm0
141501519: lea rdx, [rsp + 0x50]
14150151e: movaps xmmword ptr [rsp + 0x50], xmm2
141501523: movaps xmmword ptr [rsp + 0x60], xmm3
141501528: lea r8, [rsp + 0x70]
14150152d: movups xmm0, xmmword ptr [rsp + 0x130]
141501535: movaps xmmword ptr [rsp + 0x70], xmm0
14150153a: movups xmm0, xmmword ptr [rsp + 0x140]
141501542: movaps xmmword ptr [rsp + 0x80], xmm0
14150154a: lea r9, [rsp + 0x90]
141501552: movups xmm0, xmmword ptr [rsp + 0x150]
14150155a: movaps xmmword ptr [rsp + 0x90], xmm0
141501562: movups xmm0, xmmword ptr [rsp + 0x160]
14150156a: movaps xmmword ptr [rsp + 0xa0], xmm0
141501572: lea rax, [rsp + 0xb0]
14150157a: movups xmm0, xmmword ptr [rsp + 0x180]
141501582: movaps xmmword ptr [rsp + 0xc0], xmm0
14150158a: movups xmm0, xmmword ptr [rsp + 0x170]
141501592: movaps xmmword ptr [rsp + 0xb0], xmm0
14150159a: lea rcx, [rsp + 0xd0]
1415015a2: movups xmm0, xmmword ptr [rsp + 0x190]
1415015aa: movups xmm1, xmmword ptr [rsp + 0x1a0]
1415015b2: movaps xmmword ptr [rsp + 0xe0], xmm1
1415015ba: movaps xmmword ptr [rsp + 0xd0], xmm0
1415015c2: mov qword ptr [rsp + 0x28], rcx
1415015c7: mov qword ptr [rsp + 0x20], rax
1415015cc: lea rcx, [rsp + 0x30]
1415015d1: call 0x140a21620
1415015d6: mov dword ptr [rdi + 0x130], ebx
1415015dc: xor eax, eax
1415015de: cmp rax, qword ptr [rdi + 0x160]
1415015e5: jo 0x14150161e
1415015e7: lea rcx, [rdi + 0x160]
1415015ee: mov rdx, rdi
1415015f1: mov r8, rsi
1415015f4: call 0x140d14560
1415015f9: xor eax, eax
1415015fb: add rsp, 0x1b0
141501602: pop rbx
141501603: pop rdi
141501604: pop rsi
141501605: ret 
141501606: lea rcx, [rip + 0x176a334]
14150160d: lea r8, [rip + 0x176a3f4]
141501614: mov edx, 0x59
141501619: call 0x1428d9330
14150161e: lea rcx, [rip + 0x176a31c]
141501625: lea r8, [rip + 0x176a3f4]
14150162c: mov edx, 0x59
141501631: call 0x1428d9330
141501636: int3 
