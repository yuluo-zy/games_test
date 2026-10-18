141612870: push rbp
141612871: push r15
141612873: push r14
141612875: push r12
141612877: push rsi
141612878: push rdi
141612879: push rbx
14161287a: sub rsp, 0x100
141612881: lea rbp, [rsp + 0x80]
141612889: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141612891: mov rbx, r8
141612894: mov rdi, rdx
141612897: mov rsi, rcx
14161289a: test r8, r8
14161289d: je 0x141612932
1416128a3: mov r14, r9
1416128a6: mov rax, r9
1416128a9: and rax, 3
1416128ad: mov qword ptr [rbp - 0x40], rax
1416128b1: jne 0x1416129be
1416128b7: mov rax, rbx
1416128ba: and rax, 3
1416128be: mov qword ptr [rbp - 0x40], rax
1416128c2: jne 0x1416129cf
1416128c8: cmp r14, 0x1b4
1416128cf: jb 0x141612935
1416128d1: lea rax, [rip + 0x148a9eb]
1416128d8: mov qword ptr [rbp - 0x40], rax
1416128dc: mov qword ptr [rbp - 0x38], 0x3b
1416128e4: lea rax, [rbp - 0x40]
1416128e8: mov qword ptr [rbp + 0x68], rax
1416128ec: lea rax, [rip - 0x1508a63]
1416128f3: mov qword ptr [rbp + 0x70], rax
1416128f7: lea rax, [rip + 0x166ceca]
1416128fe: mov qword ptr [rbp + 0x10], rax
141612902: mov qword ptr [rbp + 0x18], 1
14161290a: mov qword ptr [rbp + 0x30], 0
141612912: lea rax, [rbp + 0x68]
141612916: mov qword ptr [rbp + 0x20], rax
14161291a: mov qword ptr [rbp + 0x28], 1
141612922: lea rdx, [rip + 0x166ceaf]
141612929: lea rcx, [rbp + 0x10]
14161292d: call 0x1428d9390
141612932: xor r14d, r14d
141612935: lea r15, [rbp + 0x10]
141612939: mov edx, 0x1b0
14161293e: mov rcx, r15
141612941: mov r8d, 0x122
141612947: mov r9b, 1
14161294a: call 0x140bd4de0
14161294f: lea r12, [rbp + 0x68]
141612953: mov r8d, 4
141612959: mov rcx, r12
14161295c: mov rdx, r15
14161295f: call 0x140bd4f20
141612964: mov qword ptr [rsp + 0x30], r14
141612969: mov qword ptr [rsp + 0x28], rbx
14161296e: mov qword ptr [rsp + 0x20], r12
141612973: lea r8, [rip + 0x166d5bb]
14161297a: lea rcx, [rbp - 0x40]
14161297e: mov r9d, 0x16
141612984: mov rdx, rdi
141612987: call 0x1416a2dc0
14161298c: cmp dword ptr [rbp - 0x40], 0xb
141612990: jne 0x1416129f9
141612992: mov rax, qword ptr [rbp - 0x18]
141612996: mov qword ptr [rsi + 0x20], rax
14161299a: movups xmm0, xmmword ptr [rbp - 0x38]
14161299e: movups xmm1, xmmword ptr [rbp - 0x28]
1416129a2: movups xmmword ptr [rsi + 0x10], xmm1
1416129a6: movups xmmword ptr [rsi], xmm0
1416129a9: mov rax, rsi
1416129ac: add rsp, 0x100
1416129b3: pop rbx
1416129b4: pop rdi
1416129b5: pop rsi
1416129b6: pop r12
1416129b8: pop r14
1416129ba: pop r15
1416129bc: pop rbp
1416129bd: ret 
1416129be: mov qword ptr [rbp + 0x10], 0
1416129c6: lea rax, [rip + 0x16b757b]
1416129cd: jmp 0x1416129de
1416129cf: mov qword ptr [rbp + 0x10], 0
1416129d7: lea rax, [rip + 0x16b7582]
1416129de: mov qword ptr [rsp + 0x20], rax
1416129e3: lea r8, [rip + 0x16b7556]
1416129ea: lea rdx, [rbp - 0x40]
1416129ee: lea r9, [rbp + 0x10]
1416129f2: xor ecx, ecx
1416129f4: call 0x1428d9596
1416129f9: movups xmm0, xmmword ptr [rbp]
1416129fd: movaps xmmword ptr [rbp + 0x50], xmm0
141612a01: movups xmm0, xmmword ptr [rbp - 0x40]
141612a05: movups xmm1, xmmword ptr [rbp - 0x30]
141612a09: movups xmm2, xmmword ptr [rbp - 0x20]
141612a0d: movups xmm3, xmmword ptr [rbp - 0x10]
141612a11: movaps xmmword ptr [rbp + 0x40], xmm3
141612a15: movaps xmmword ptr [rbp + 0x30], xmm2
141612a19: movaps xmmword ptr [rbp + 0x20], xmm1
141612a1d: movaps xmmword ptr [rbp + 0x10], xmm0
141612a21: lea rax, [rip + 0x166d780]
141612a28: mov qword ptr [rsp + 0x20], rax
141612a2d: lea rcx, [rip + 0x166c0b4]
141612a34: lea r9, [rip + 0x166c08d]
141612a3b: lea r8, [rbp + 0x10]
141612a3f: mov edx, 0x2b
141612a44: call 0x1428d9760
141612a49: ud2 
141612a4b: nop dword ptr [rax + rax]
141612a50: mov qword ptr [rsp + 0x10], rdx
141612a55: push rbp
141612a56: push r15
141612a58: push r14
141612a5a: push r12
141612a5c: push rsi
141612a5d: push rdi
141612a5e: push rbx
141612a5f: sub rsp, 0x40
141612a63: lea rbp, [rdx + 0x80]
141612a6a: lea rcx, [rbp + 0x10]
141612a6e: call 0x1407cea70
141612a73: nop 
141612a74: add rsp, 0x40
141612a78: pop rbx
141612a79: pop rdi
141612a7a: pop rsi
141612a7b: pop r12
141612a7d: pop r14
141612a7f: pop r15
141612a81: pop rbp
141612a82: ret 
