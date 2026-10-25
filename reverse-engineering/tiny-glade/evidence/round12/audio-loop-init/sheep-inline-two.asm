1414d9b58: 48c7453003000000 mov qword ptr [rbp + 0x30], 3
1414d9b60: c7453832000000 mov dword ptr [rbp + 0x38], 0x32
1414d9b67: 488d05ca875e01 lea rax, [rip + 0x15e87ca]
1414d9b6e: 48894520 mov qword ptr [rbp + 0x20], rax
1414d9b72: 48c7452814000000 mov qword ptr [rbp + 0x28], 0x14
1414d9b7a: 488d4da8 lea rcx, [rbp - 0x58]
1414d9b7e: 4c8d4520 lea r8, [rbp + 0x20]
1414d9b82: e899f657ff call 0x140a59220
1414d9b87: 31c0 xor eax, eax
1414d9b89: 483b45a8 cmp rax, qword ptr [rbp - 0x58]
1414d9b8d: 0f80d7000000 jo 0x1414d9c6a
1414d9b93: 488d4d00 lea rcx, [rbp]
1414d9b97: 488b45d8 mov rax, qword ptr [rbp - 0x28]
1414d9b9b: 48894550 mov qword ptr [rbp + 0x50], rax
1414d9b9f: 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
1414d9ba3: 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
1414d9ba7: 0f1055c8 movups xmm2, xmmword ptr [rbp - 0x38]
1414d9bab: 0f295540 movaps xmmword ptr [rbp + 0x40], xmm2
1414d9baf: 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
1414d9bb3: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
1414d9bb7: 4c8d05da875e01 lea r8, [rip + 0x15e87da]
1414d9bbe: 488d5520 lea rdx, [rbp + 0x20]
1414d9bc2: e8e952e0ff call 0x1412deeb0
