141231af0 push r14
141231af2 push rsi
141231af3 push rdi
141231af4 push rbp
141231af5 push rbx
141231af6 sub rsp, 0x1b0
141231afd movaps xmmword ptr [rsp + 0x1a0], xmm14
141231b06 movaps xmmword ptr [rsp + 0x190], xmm13
141231b0f movaps xmmword ptr [rsp + 0x180], xmm12
141231b18 movaps xmmword ptr [rsp + 0x170], xmm11
141231b21 movaps xmmword ptr [rsp + 0x160], xmm10
141231b2a movaps xmmword ptr [rsp + 0x150], xmm9
141231b33 movaps xmmword ptr [rsp + 0x140], xmm8
141231b3c movaps xmmword ptr [rsp + 0x130], xmm7
141231b44 movaps xmmword ptr [rsp + 0x120], xmm6
141231b4c movaps xmm6, xmm3
141231b4f mov rsi, rcx
141231b52 movss xmm1, dword ptr [rdx + 8]
141231b57 movss xmm4, dword ptr [r8 + 8]
141231b5d movaps xmm2, xmm1
141231b60 addss xmm2, xmm4
141231b64 mulss xmm2, dword ptr [rip + 0x16f3d04]
141231b6c movss xmm0, dword ptr [rsp + 0x200]
141231b75 movsd xmm5, qword ptr [rdx]
141231b79 movsd xmm3, qword ptr [r8]
141231b7e movaps xmm7, xmm5
141231b81 addps xmm7, xmm3
141231b84 mulps xmm7, xmmword ptr [rip + 0x16f3d55]
141231b8b movlps qword ptr [rsp + 0x20], xmm7
141231b90 movss dword ptr [rsp + 0x28], xmm2
141231b96 subss xmm4, xmm1
141231b9a subps xmm3, xmm5
141231b9d movaps xmm1, xmm3
141231ba0 mulps xmm1, xmm3
141231ba3 movshdup xmm2, xmm1
141231ba7 addss xmm2, xmm1
141231bab movaps xmm1, xmm4
141231bae mulss xmm1, xmm4
141231bb2 addss xmm1, xmm2
141231bb6 xorps xmm5, xmm5
141231bb9 sqrtss xmm5, xmm1
141231bbd movss xmm1, dword ptr [rip + 0x16f3d3f]
141231bc5 movaps xmm9, xmm1
141231bc9 divss xmm9, xmm5
141231bce movshdup xmm12, xmm3
141231bd3 mulss xmm12, xmm9
141231bd8 xorps xmm2, xmm2
141231bdb xorps xmm7, xmm7
141231bde movss xmm7, xmm5
141231be2 shufps xmm7, xmm7, 0x40
141231be6 movaps xmm5, xmm12
141231bea movlhps xmm5, xmm3
141231bed shufps xmm5, xmm3, 0xe2
141231bf1 movss xmm2, xmm9
141231bf6 mulps xmm2, xmm5
141231bf9 movaps xmm5, xmm12
141231bfd movlhps xmm5, xmm2
141231c00 movaps xmm3, xmm5
141231c03 shufps xmm3, xmm2, 0xe2
141231c07 mulss xmm9, xmm4
141231c0c xorps xmm11, xmm11
141231c10 movaps xmm4, xmm9
141231c14 mulss xmm4, xmm11
141231c19 movaps xmm8, xmm9
141231c1d shufps xmm8, xmm2, 0xd4
141231c22 shufps xmm8, xmm2, 0xf2
141231c27 movaps xmm10, xmm2
141231c2b subps xmm10, xmm8
141231c2f movaps xmm8, xmm10
141231c33 mulps xmm8, xmm10
141231c37 movaps xmm13, xmm9
141231c3b shufps xmm13, xmm2, 0xf4
141231c40 shufps xmm5, xmm13, 0x82
141231c45 shufps xmm5, xmm5, 0x24
141231c49 mulps xmm5, xmm7
141231c4c movaps xmm13, xmm5
141231c50 shufps xmm13, xmm5, 0xff
141231c55 movaps xmm7, xmm4
141231c58 subss xmm7, xmm13
141231c5d movaps xmm13, xmm7
141231c61 mulss xmm13, xmm7
141231c66 movshdup xmm14, xmm8
141231c6b addss xmm14, xmm13
141231c70 addss xmm14, xmm8
141231c75 xorps xmm8, xmm8
141231c79 sqrtss xmm8, xmm14
141231c7e movaps xmm13, xmm1
141231c82 divss xmm13, xmm8
141231c87 mulss xmm7, xmm13
141231c8c movsldup xmm8, xmm13
141231c91 mulps xmm8, xmm10
141231c95 movaps xmm10, xmm9
141231c99 mulss xmm10, xmm7
141231c9e mulss xmm12, xmm8
141231ca3 subss xmm10, xmm12
141231ca8 mulps xmm3, xmm8
141231cac unpcklps xmm9, xmm7
141231cb0 movaps xmm12, xmm2
141231cb4 shufps xmm12, xmm8, 0x10
141231cb9 shufps xmm12, xmm8, 0xe2
141231cbe mulps xmm12, xmm9
141231cc2 subps xmm3, xmm12
141231cc6 movshdup xmm9, xmm8
141231ccb shufps xmm10, xmm3, 0x40
141231cd0 shufps xmm10, xmm10, 0x78
141231cd5 mulps xmm10, xmmword ptr [rip + 0x17abd13]
141231cdd mulss xmm9, xmm11
141231ce2 movss xmm12, dword ptr [rip + 0x18814c5]
141231ceb movshdup xmm11, xmm3
141231cf0 mulss xmm3, xmm12
141231cf5 mulss xmm11, xmm12
141231cfa shufps xmm11, xmm2, 0xd4
141231cff shufps xmm2, xmm5, 0xf5
141231d03 addps xmm5, xmm10
141231d07 shufps xmm11, xmm10, 0x42
141231d0c movhlps xmm10, xmm10
141231d10 addss xmm10, xmm4
141231d15 shufps xmm7, xmm8, 0x40
141231d1a mulss xmm8, dword ptr [rip + 0x179b30d]
141231d23 addss xmm8, xmm10
141231d28 shufps xmm7, xmm7, 0x78
141231d2c mulps xmm7, xmmword ptr [rip + 0x19ec6dd]
141231d33 shufps xmm9, xmm7, 0x40
141231d38 shufps xmm9, xmm9, 0x78
141231d3d addps xmm9, xmm5
141231d41 movlhps xmm4, xmm3
141231d44 shufps xmm4, xmm2, 0x22
141231d48 addps xmm11, xmm4
141231d4c addps xmm11, xmm7
141231d50 movaps xmmword ptr [rsp + 0x30], xmm9
141231d56 movaps xmmword ptr [rsp + 0x40], xmm11
141231d5c movss dword ptr [rsp + 0x50], xmm8
141231d63 addss xmm1, xmm0
141231d67 movaps xmm7, xmm0
141231d6a cmpltss xmm7, xmm6
141231d6f andps xmm1, xmm7
141231d72 andnps xmm7, xmm0
141231d75 orps xmm7, xmm1
141231d78 lea rdi, [rsp + 0x60]
141231d7d lea rdx, [rsp + 0x30]
141231d82 lea r8, [rsp + 0x20]
141231d87 mov rcx, rdi
141231d8a call 0x14089d4d0
141231d8f mov rax, qword ptr [rsi]
141231d92 mov r14, qword ptr [rsi + 8]
141231d96 mov ebp, dword ptr [r14]
141231d99 add ebp, dword ptr [rax]
141231d9b mov rax, qword ptr [rsi + 0x10]
141231d9f mov rcx, qword ptr [rax]
141231da2 call 0x140858790
141231da7 mov ebx, eax
141231da9 mov rax, qword ptr [rsi + 0x18]
141231dad movss xmm8, dword ptr [rax + 0x28c]
141231db6 lea rcx, [rsp + 0xc0]
141231dbe call 0x1408587c0
141231dc3 movups xmm0, xmmword ptr [rsp + 0xf0]
141231dcb movups xmmword ptr [rsp + 0x90], xmm0
141231dd3 mov rax, qword ptr [rsp + 0x100]
141231ddb mov qword ptr [rsp + 0xa0], rax
141231de3 mov eax, dword ptr [rsp + 0x108]
141231dea mov dword ptr [rsp + 0xa8], eax
141231df1 mov dword ptr [rsp + 0xac], ebp
141231df8 mov dword ptr [rsp + 0xb0], ebx
141231dff unpcklps xmm6, xmm7
141231e02 movsldup xmm0, xmm8
141231e07 mulps xmm0, xmm6
141231e0a movlps qword ptr [rsp + 0xb4], xmm0
141231e12 mov eax, dword ptr [rsp + 0x11c]
141231e19 mov dword ptr [rsp + 0xbc], eax
141231e20 mov rcx, qword ptr [rsi + 0x20]
141231e24 mov rdx, rdi
141231e27 call 0x14120c120
141231e2c test al, al
141231e2e je 0x141231e37
141231e30 mov rax, qword ptr [rsi + 0x28]
141231e34 mov byte ptr [rax], 1
141231e37 inc qword ptr [r14]
141231e3a movaps xmm6, xmmword ptr [rsp + 0x120]
141231e42 movaps xmm7, xmmword ptr [rsp + 0x130]
141231e4a movaps xmm8, xmmword ptr [rsp + 0x140]
141231e53 movaps xmm9, xmmword ptr [rsp + 0x150]
141231e5c movaps xmm10, xmmword ptr [rsp + 0x160]
141231e65 movaps xmm11, xmmword ptr [rsp + 0x170]
141231e6e movaps xmm12, xmmword ptr [rsp + 0x180]
141231e77 movaps xmm13, xmmword ptr [rsp + 0x190]
141231e80 movaps xmm14, xmmword ptr [rsp + 0x1a0]
141231e89 add rsp, 0x1b0
141231e90 pop rbx
141231e91 pop rbp
141231e92 pop rdi
141231e93 pop rsi
141231e94 pop r14
141231e96 ret 
