140b6a8ff: 48c7453003000000 mov qword ptr [rbp + 0x30], 3
140b6a907: c7453832000000 mov dword ptr [rbp + 0x38], 0x32
140b6a90e: 488d0d237af501 lea rcx, [rip + 0x1f57a23]
140b6a915: 48894d20 mov qword ptr [rbp + 0x20], rcx
140b6a919: 48c7452814000000 mov qword ptr [rbp + 0x28], 0x14
140b6a921: 488d4da8 lea rcx, [rbp - 0x58]
140b6a925: 4c8d4520 lea r8, [rbp + 0x20]
140b6a929: 4889c2 mov rdx, rax
140b6a92c: e8efe8eeff call 0x140a59220
140b6a931: 31c0 xor eax, eax
140b6a933: 483b45a8 cmp rax, qword ptr [rbp - 0x58]
140b6a937: 0f80d6000000 jo 0x140b6aa13
140b6a93d: 488d4d00 lea rcx, [rbp]
140b6a941: 488b45d8 mov rax, qword ptr [rbp - 0x28]
140b6a945: 48894550 mov qword ptr [rbp + 0x50], rax
140b6a949: 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
140b6a94d: 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
140b6a951: 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
140b6a955: 0f295540 movaps xmmword ptr [rbp + 0x40], xmm2
140b6a959: 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
140b6a95d: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
140b6a961: 4c8d05307af501 lea r8, [rip + 0x1f57a30]
140b6a968: 488d5520 lea rdx, [rbp + 0x20]
140b6a96c: e83f457700 call 0x1412deeb0
