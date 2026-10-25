1414ba880: push rsi
1414ba881: push rdi
1414ba882: push rbx
1414ba883: sub rsp, 0x1b0
1414ba88a: mov rdi, rdx
1414ba88d: mov rsi, rcx
1414ba890: call 0x1413efa40
1414ba895: mov ebx, 1
1414ba89a: lock xadd dword ptr [rdi + 0x5b0], ebx
1414ba8a2: xor eax, eax
1414ba8a4: cmp rax, qword ptr [rsi + 0x160]
1414ba8ab: jo 0x1414ba9c9
1414ba8b1: lea rdx, [rsi + 0x138]
1414ba8b8: mov dword ptr [rsp + 0x20], ebx
1414ba8bc: lea rcx, [rsp + 0xf0]
1414ba8c4: mov r8, rsi
1414ba8c7: mov r9, rdi
1414ba8ca: call 0x141507230
1414ba8cf: movups xmm0, xmmword ptr [rsp + 0xf0]
1414ba8d7: movups xmm1, xmmword ptr [rsp + 0x100]
1414ba8df: movups xmm2, xmmword ptr [rsp + 0x110]
1414ba8e7: movups xmm3, xmmword ptr [rsp + 0x120]
1414ba8ef: movaps xmmword ptr [rsp + 0x40], xmm1
1414ba8f4: movaps xmmword ptr [rsp + 0x30], xmm0
1414ba8f9: lea rdx, [rsp + 0x50]
1414ba8fe: movaps xmmword ptr [rsp + 0x50], xmm2
1414ba903: movaps xmmword ptr [rsp + 0x60], xmm3
1414ba908: lea r8, [rsp + 0x70]
1414ba90d: movups xmm0, xmmword ptr [rsp + 0x130]
1414ba915: movaps xmmword ptr [rsp + 0x70], xmm0
1414ba91a: movups xmm0, xmmword ptr [rsp + 0x140]
1414ba922: movaps xmmword ptr [rsp + 0x80], xmm0
1414ba92a: lea r9, [rsp + 0x90]
1414ba932: movups xmm0, xmmword ptr [rsp + 0x150]
1414ba93a: movaps xmmword ptr [rsp + 0x90], xmm0
1414ba942: movups xmm0, xmmword ptr [rsp + 0x160]
1414ba94a: movaps xmmword ptr [rsp + 0xa0], xmm0
1414ba952: lea rax, [rsp + 0xb0]
1414ba95a: movups xmm0, xmmword ptr [rsp + 0x180]
1414ba962: movaps xmmword ptr [rsp + 0xc0], xmm0
1414ba96a: movups xmm0, xmmword ptr [rsp + 0x170]
1414ba972: movaps xmmword ptr [rsp + 0xb0], xmm0
1414ba97a: lea rcx, [rsp + 0xd0]
1414ba982: movups xmm0, xmmword ptr [rsp + 0x190]
1414ba98a: movups xmm1, xmmword ptr [rsp + 0x1a0]
1414ba992: movaps xmmword ptr [rsp + 0xe0], xmm1
1414ba99a: movaps xmmword ptr [rsp + 0xd0], xmm0
1414ba9a2: mov qword ptr [rsp + 0x28], rcx
1414ba9a7: mov qword ptr [rsp + 0x20], rax
1414ba9ac: lea rcx, [rsp + 0x30]
1414ba9b1: call 0x140a21620
1414ba9b6: mov dword ptr [rsi + 0x130], ebx
1414ba9bc: xor eax, eax
1414ba9be: add rsp, 0x1b0
1414ba9c5: pop rbx
1414ba9c6: pop rdi
1414ba9c7: pop rsi
1414ba9c8: ret 
1414ba9c9: lea rcx, [rip + 0x17b0f71]
1414ba9d0: lea r8, [rip + 0x17b1031]
1414ba9d7: mov edx, 0x59
1414ba9dc: call 0x1428d9330
1414ba9e1: int3 
