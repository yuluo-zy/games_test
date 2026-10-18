0x142374e10 push r15
0x142374e12 push r14
0x142374e14 push rsi
0x142374e15 push rdi
0x142374e16 push rbx
0x142374e17 sub rsp, 0x90
0x142374e1e movaps xmmword ptr [rsp + 0x80], xmm7
0x142374e26 movaps xmmword ptr [rsp + 0x70], xmm6
0x142374e2b mov esi, r8d
0x142374e2e mov rdi, rcx
0x142374e31 movzx ecx, byte ptr [rcx + 0x90]
0x142374e38 and cl, byte ptr [rdi + 0x91]
0x142374e3e mov r8d, 0x2fc
0x142374e44 mov eax, 0x310
0x142374e49 cmovne rax, r8
0x142374e4d mov r8d, esi
0x142374e50 cmp qword ptr [rdi + 0x10], r8
0x142374e54 jbe 0x142374f2b
0x142374e5a mov ebx, edx
0x142374e5c mov rdx, qword ptr [rdi + 8]
0x142374e60 test rdx, rdx
0x142374e63 je 0x142374f2b
0x142374e69 imul r8, r8, 0x330
0x142374e70 cmp dword ptr [rdx + r8 + 0x328], ebx
0x142374e78 jne 0x142374f2b
0x142374e7e movaps xmm6, xmm3
0x142374e81 movss xmm0, dword ptr [rsp + 0xe0]
0x142374e8a add rdx, r8
0x142374e8d movsd xmm1, qword ptr [rdx + rax + 8]
0x142374e93 unpcklps xmm6, xmm0
0x142374e96 addps xmm6, xmm1
0x142374e99 movlps qword ptr [rdx + rax + 8], xmm6
0x142374e9e lea rcx, [rsp + 0x30]
0x142374ea3 mov rdx, rdi
0x142374ea6 mov r8d, ebx
0x142374ea9 mov r9d, esi
0x142374eac call 0x140fe7d60
0x142374eb1 cmp dword ptr [rsp + 0x30], 4
0x142374eb6 jne 0x142374f50
0x142374ebc mov rcx, rdi
0x142374ebf mov edx, ebx
0x142374ec1 mov r8d, esi
0x142374ec4 call 0x140fe7300
0x142374ec9 test rax, rax
0x142374ecc je 0x142374f0f
0x142374ece mov r14, rax
0x142374ed1 movshdup xmm7, xmm6
0x142374ed5 xor r9d, r9d
0x142374ed8 nop dword ptr [rax + rax]
0x142374ee0 lea r15, [r9 + 1]
0x142374ee4 mov rcx, rdi
0x142374ee7 mov edx, ebx
0x142374ee9 mov r8d, esi
0x142374eec call 0x140fe73a0
0x142374ef1 mov r8d, edx
0x142374ef4 movss dword ptr [rsp + 0x20], xmm7
0x142374efa mov rcx, rdi
0x142374efd mov edx, eax
0x142374eff movaps xmm3, xmm6
0x142374f02 call 0x142374e10
0x142374f07 mov r9, r15
0x142374f0a cmp r14, r15
0x142374f0d jne 0x142374ee0
0x142374f0f movaps xmm6, xmmword ptr [rsp + 0x70]
0x142374f14 movaps xmm7, xmmword ptr [rsp + 0x80]
0x142374f1c add rsp, 0x90
0x142374f23 pop rbx
0x142374f24 pop rdi
0x142374f25 pop rsi
0x142374f26 pop r14
0x142374f28 pop r15
0x142374f2a ret 
0x142374f2b lea rax, [rip + 0xcddc76]
0x142374f32 lea r8, [rip + 0xcddc57]
0x142374f39 test cl, cl
0x142374f3b cmovne r8, rax
0x142374f3f lea rcx, [rip + 0xdd4d8d]
0x142374f46 mov edx, 0x18
0x142374f4b call 0x1428d8725
0x142374f50 movups xmm0, xmmword ptr [rsp + 0x30]
0x142374f55 movups xmm1, xmmword ptr [rsp + 0x40]
0x142374f5a movaps xmmword ptr [rsp + 0x60], xmm1
0x142374f5f movaps xmmword ptr [rsp + 0x50], xmm0
0x142374f64 lea rax, [rip + 0xcddc9d]
0x142374f6b mov qword ptr [rsp + 0x20], rax
0x142374f70 lea rcx, [rip + 0xcdda79]
0x142374f77 lea r9, [rip + 0xcdda52]
0x142374f7e lea r8, [rsp + 0x50]
0x142374f83 mov edx, 0x2b
0x142374f88 call 0x1428d9760
0x142374f8d int3 
