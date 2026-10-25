0x140fe19a0 push rbp
0x140fe19a1 push r15
0x140fe19a3 push r14
0x140fe19a5 push r13
0x140fe19a7 push r12
0x140fe19a9 push rsi
0x140fe19aa push rdi
0x140fe19ab push rbx
0x140fe19ac sub rsp, 0x128
0x140fe19b3 lea rbp, [rsp + 0x80]
0x140fe19bb movaps xmmword ptr [rbp + 0x90], xmm15
0x140fe19c3 movaps xmmword ptr [rbp + 0x80], xmm14
0x140fe19cb movaps xmmword ptr [rbp + 0x70], xmm13
0x140fe19d0 movaps xmmword ptr [rbp + 0x60], xmm12
0x140fe19d5 movaps xmmword ptr [rbp + 0x50], xmm11
0x140fe19da movaps xmmword ptr [rbp + 0x40], xmm10
0x140fe19df movaps xmmword ptr [rbp + 0x30], xmm9
0x140fe19e4 movaps xmmword ptr [rbp + 0x20], xmm8
0x140fe19e9 movaps xmmword ptr [rbp + 0x10], xmm7
0x140fe19ed movaps xmmword ptr [rbp], xmm6
0x140fe19f1 mov qword ptr [rbp - 8], 0xfffffffffffffffe
0x140fe19f9 mov r13, qword ptr [rbp + 0x110]
0x140fe1a00 mov r15, r13
0x140fe1a03 shl r15, 4
0x140fe1a07 xor r12d, r12d
0x140fe1a0a mov rax, r13
0x140fe1a0d shr rax, 0x3c
0x140fe1a11 jne 0x140fe1f8e
0x140fe1a17 movabs rax, 0x7ffffffffffffffc
0x140fe1a21 cmp r15, rax
0x140fe1a24 ja 0x140fe1f8e
0x140fe1a2a mov rdi, r9
0x140fe1a2d mov r14, r8
0x140fe1a30 mov rbx, rdx
0x140fe1a33 mov rsi, rcx
0x140fe1a36 test r15, r15
0x140fe1a39 je 0x140fe1a63
0x140fe1a3b movzx eax, byte ptr [rip + 0x26531c7]
0x140fe1a42 mov r12d, 4
0x140fe1a48 mov edx, 4
0x140fe1a4d mov rcx, r15
0x140fe1a50 call 0x140613c10
0x140fe1a55 test rax, rax
0x140fe1a58 je 0x140fe1f8e
0x140fe1a5e mov rcx, r13
0x140fe1a61 jmp 0x140fe1a6a
0x140fe1a63 mov eax, 4
0x140fe1a68 xor ecx, ecx
0x140fe1a6a mov qword ptr [rbp - 0x30], rcx
0x140fe1a6e mov qword ptr [rbp - 0x28], rax
0x140fe1a72 mov qword ptr [rbp - 0x20], 0
0x140fe1a7a movss xmm13, dword ptr [r14]
0x140fe1a7f movss xmm11, dword ptr [rbx + 4]
0x140fe1a85 cmp byte ptr [rbx], 0
0x140fe1a88 jne 0x140fe1c19
0x140fe1a8e movss xmm15, dword ptr [r14 + 4]
0x140fe1a94 movss xmm9, dword ptr [rbx + 0xc]
0x140fe1a9a cmp byte ptr [rbx + 8], 0
0x140fe1a9e jne 0x140fe1c34
0x140fe1aa4 movss xmm0, dword ptr [rbx + 0x14]
0x140fe1aa9 cmp byte ptr [rbx + 0x10], 0
0x140fe1aad jne 0x140fe1c48
0x140fe1ab3 movss xmm1, dword ptr [rbx + 0x1c]
0x140fe1ab8 cmp byte ptr [rbx + 0x18], 0
0x140fe1abc je 0x140fe1ac3
0x140fe1abe mulss xmm1, xmm15
0x140fe1ac3 addss xmm1, xmm9
0x140fe1ac8 movaps xmm10, xmm15
0x140fe1acc subss xmm10, xmm1
0x140fe1ad1 movss xmm14, dword ptr [rbx + 0x24]
0x140fe1ad7 cmp byte ptr [rbx + 0x20], 0
0x140fe1adb je 0x140fe1ae2
0x140fe1add mulss xmm14, xmm10
0x140fe1ae2 xor eax, eax
0x140fe1ae4 mov rcx, r13
0x140fe1ae7 sub rcx, 1
0x140fe1aeb cmovae rax, rcx
0x140fe1aef cvtsi2ss xmm3, rax
0x140fe1af4 mulss xmm3, xmm14
0x140fe1af9 subss xmm10, xmm3
0x140fe1afe xorps xmm2, xmm2
0x140fe1b01 xorps xmm4, xmm4
0x140fe1b04 xorps xmm12, xmm12
0x140fe1b08 test r13, r13
0x140fe1b0b je 0x140fe1ba7
0x140fe1b11 lea rax, [r13*2]
0x140fe1b19 add rax, r13
0x140fe1b1c lea rax, [rdi + rax*8]
0x140fe1b20 mov rcx, rdi
0x140fe1b23 jmp 0x140fe1b7e
0x140fe1b25 nop word ptr cs:[rax + rax]
0x140fe1b30 addss xmm12, dword ptr [rdx + 0x38]
0x140fe1b36 movaps xmm7, xmm5
0x140fe1b39 cmpunordss xmm7, xmm5
0x140fe1b3e movaps xmm8, xmm7
0x140fe1b42 andps xmm8, xmm6
0x140fe1b46 maxss xmm6, xmm5
0x140fe1b4a andnps xmm7, xmm6
0x140fe1b4d orps xmm7, xmm8
0x140fe1b51 movss xmm5, dword ptr [rcx + 0x14]
0x140fe1b56 movaps xmm6, xmm5
0x140fe1b59 maxss xmm6, xmm7
0x140fe1b5d cmpunordss xmm7, xmm7
0x140fe1b62 movaps xmm8, xmm7
0x140fe1b66 andnps xmm8, xmm6
0x140fe1b6a andps xmm7, xmm5
0x140fe1b6d orps xmm7, xmm8
0x140fe1b71 addss xmm4, xmm7
0x140fe1b75 add rcx, 0x18
0x140fe1b79 cmp rcx, rax
0x140fe1b7c je 0x140fe1ba7
0x140fe1b7e mov rdx, qword ptr [rcx + 8]
0x140fe1b82 movss xmm5, dword ptr [rdx + 0xc]
0x140fe1b87 cmp byte ptr [rdx + 8], 0
0x140fe1b8b je 0x140fe1b92
0x140fe1b8d mulss xmm5, xmm10
0x140fe1b92 mov rdx, qword ptr [rcx]
0x140fe1b95 movss xmm6, dword ptr [rdx + 0x34]
0x140fe1b9a cmp byte ptr [rdx + 0x30], 0
0x140fe1b9e je 0x140fe1b30
0x140fe1ba0 mulss xmm6, xmm10
0x140fe1ba5 jmp 0x140fe1b30
0x140fe1ba7 addss xmm3, xmm4
0x140fe1bab subss xmm15, xmm3
0x140fe1bb0 subss xmm15, xmm1
0x140fe1bb5 movss xmm1, dword ptr [rip + 0x1943d47]
0x140fe1bbd movaps xmm3, xmm1
0x140fe1bc0 subss xmm3, xmm12
0x140fe1bc5 maxss xmm2, xmm3
0x140fe1bc9 minss xmm1, xmm2
0x140fe1bcd mulss xmm1, xmm15
0x140fe1bd2 movzx eax, byte ptr [rbx + 0x46]
0x140fe1bd6 lea rcx, [rip + 0x1be0683]
0x140fe1bdd movsxd rax, dword ptr [rcx + rax*4]
0x140fe1be1 add rax, rcx
0x140fe1be4 mov qword ptr [rbp - 0x38], rsi
0x140fe1be8 jmp rax
0x140fe1bea cmp r13, 1
0x140fe1bee jbe 0x140fe1c61
0x140fe1bf0 lea rax, [r13 - 1]
0x140fe1bf4 xorps xmm2, xmm2
0x140fe1bf7 cvtsi2ss xmm2, rax
0x140fe1bfc divss xmm1, xmm2
0x140fe1c00 addss xmm14, xmm1
0x140fe1c05 movss dword ptr [rbp - 0xc], xmm14
0x140fe1c0b test r13, r13
0x140fe1c0e jne 0x140fe1cc5
0x140fe1c14 jmp 0x140fe1f31
0x140fe1c19 mulss xmm11, xmm13
0x140fe1c1e movss xmm15, dword ptr [r14 + 4]
0x140fe1c24 movss xmm9, dword ptr [rbx + 0xc]
0x140fe1c2a cmp byte ptr [rbx + 8], 0
0x140fe1c2e je 0x140fe1aa4
0x140fe1c34 mulss xmm9, xmm15
0x140fe1c39 movss xmm0, dword ptr [rbx + 0x14]
0x140fe1c3e cmp byte ptr [rbx + 0x10], 0
0x140fe1c42 je 0x140fe1ab3
0x140fe1c48 mulss xmm0, xmm13
0x140fe1c4d movss xmm1, dword ptr [rbx + 0x1c]
0x140fe1c52 cmp byte ptr [rbx + 0x18], 0
0x140fe1c56 jne 0x140fe1abe
0x140fe1c5c jmp 0x140fe1ac3
0x140fe1c61 movss dword ptr [rbp - 0xc], xmm14
0x140fe1c67 mulss xmm1, dword ptr [rip + 0x1943c01]
0x140fe1c6f addss xmm9, xmm1
0x140fe1c74 test r13, r13
0x140fe1c77 jne 0x140fe1cc5
0x140fe1c79 jmp 0x140fe1f31
0x140fe1c7e mov rax, r13
0x140fe1c81 inc rax
0x140fe1c84 js 0x140fe1c90
0x140fe1c86 xorps xmm2, xmm2
0x140fe1c89 cvtsi2ss xmm2, rax
0x140fe1c8e jmp 0x140fe1ca8
0x140fe1c90 mov rcx, rax
0x140fe1c93 shr rcx, 1
0x140fe1c96 and eax, 1
0x140fe1c99 or rax, rcx
0x140fe1c9c xorps xmm2, xmm2
0x140fe1c9f cvtsi2ss xmm2, rax
0x140fe1ca4 addss xmm2, xmm2
0x140fe1ca8 divss xmm1, xmm2
0x140fe1cac addss xmm14, xmm1
0x140fe1cb1 movss dword ptr [rbp - 0xc], xmm14
0x140fe1cb7 addss xmm9, xmm1
0x140fe1cbc test r13, r13
0x140fe1cbf je 0x140fe1f31
0x140fe1cc5 addss xmm0, xmm11
0x140fe1cca subss xmm13, xmm0
0x140fe1ccf maxss xmm12, dword ptr [rip + 0x1a99dc8]
0x140fe1cd8 xorps xmm0, xmm0
0x140fe1cdb maxss xmm15, xmm0
0x140fe1ce0 movzx eax, byte ptr [rbx + 0x45]
0x140fe1ce4 movsd xmm0, qword ptr [r14 + 8]
0x140fe1cea movaps xmmword ptr [rbp - 0x60], xmm0
0x140fe1cee movsd xmm0, qword ptr [rbx + 0x3c]
0x140fe1cf3 movaps xmmword ptr [rbp - 0x50], xmm0
0x140fe1cf7 shl r13, 3
0x140fe1cfb lea r15, [r13*2]
0x140fe1d03 add r15, r13
0x140fe1d06 mov r12d, 8
0x140fe1d0c xor r13d, r13d
0x140fe1d0f mov eax, eax
0x140fe1d11 lea rcx, [rip + 0x1be055c]
0x140fe1d18 movsxd rsi, dword ptr [rcx + rax*4]
0x140fe1d1c add rsi, rcx
0x140fe1d1f lea r14, [rbp - 0x30]
0x140fe1d23 xor ebx, ebx
0x140fe1d25 movss dword ptr [rbp - 0x14], xmm13
0x140fe1d2b movss dword ptr [rbp - 0x10], xmm11
0x140fe1d31 movaps xmm14, xmm15
0x140fe1d35 jmp 0x140fe1dd7
0x140fe1d3a nop word ptr [rax + rax]
0x140fe1d40 movaps xmm0, xmm15
0x140fe1d44 cmpunordss xmm0, xmm15
0x140fe1d4a movaps xmm1, xmm0
0x140fe1d4d andps xmm1, xmm11
0x140fe1d51 maxss xmm11, xmm15
0x140fe1d56 andnps xmm0, xmm11
0x140fe1d5a orps xmm0, xmm1
0x140fe1d5d movaps xmm1, xmm13
0x140fe1d61 maxss xmm1, xmm0
0x140fe1d65 cmpunordss xmm0, xmm0
0x140fe1d6a movaps xmm2, xmm0
0x140fe1d6d andnps xmm2, xmm1
0x140fe1d70 andps xmm0, xmm13
0x140fe1d74 orps xmm0, xmm2
0x140fe1d77 divss xmm8, xmm12
0x140fe1d7c mulss xmm8, xmm14
0x140fe1d81 addss xmm8, xmm0
0x140fe1d86 addss xmm7, dword ptr [rbp - 0x10]
0x140fe1d8b unpcklps xmm7, xmm9
0x140fe1d8f addps xmm7, xmmword ptr [rbp - 0x60]
0x140fe1d93 addps xmm7, xmmword ptr [rbp - 0x50]
0x140fe1d97 mov rax, qword ptr [rbp - 0x28]
0x140fe1d9b movss dword ptr [rax + r12 - 8], xmm6
0x140fe1da2 movss dword ptr [rax + r12 - 4], xmm8
0x140fe1da9 movlps qword ptr [rax + r12], xmm7
0x140fe1dae inc rbx
0x140fe1db1 mov qword ptr [rbp - 0x20], rbx
0x140fe1db5 addss xmm9, dword ptr [rbp - 0xc]
0x140fe1dbb addss xmm9, xmm8
0x140fe1dc0 add r12, 0x10
0x140fe1dc4 add r13, 0x18
0x140fe1dc8 cmp r15, r13
0x140fe1dcb movss xmm13, dword ptr [rbp - 0x14]
0x140fe1dd1 je 0x140fe1f31
0x140fe1dd7 mov rax, qword ptr [rdi + r13 + 8]
0x140fe1ddc movss xmm0, dword ptr [rax + 4]
0x140fe1de1 cmp byte ptr [rax], 0
0x140fe1de4 jne 0x140fe1e80
0x140fe1dea movss xmm15, dword ptr [rax + 0xc]
0x140fe1df0 cmp byte ptr [rax + 8], 0
0x140fe1df4 jne 0x140fe1e95
0x140fe1dfa mov rax, qword ptr [rdi + r13]
0x140fe1dfe movss xmm1, dword ptr [rax + 0x2c]
0x140fe1e03 cmp byte ptr [rax + 0x28], 0
0x140fe1e07 jne 0x140fe1ead
0x140fe1e0d movss xmm11, dword ptr [rax + 0x34]
0x140fe1e13 cmp byte ptr [rax + 0x30], 0
0x140fe1e17 je 0x140fe1e1e
0x140fe1e19 mulss xmm11, xmm10
0x140fe1e1e movaps xmm6, xmm0
0x140fe1e21 cmpunordss xmm6, xmm0
0x140fe1e26 movaps xmm2, xmm6
0x140fe1e29 andps xmm2, xmm1
0x140fe1e2c maxss xmm1, xmm0
0x140fe1e30 andnps xmm6, xmm1
0x140fe1e33 orps xmm6, xmm2
0x140fe1e36 movss xmm0, dword ptr [rdi + r13 + 0x10]
0x140fe1e3d movaps xmm1, xmm0
0x140fe1e40 maxss xmm1, xmm6
0x140fe1e44 cmpunordss xmm6, xmm6
0x140fe1e49 movaps xmm2, xmm6
0x140fe1e4c andnps xmm2, xmm1
0x140fe1e4f andps xmm6, xmm0
0x140fe1e52 orps xmm6, xmm2
0x140fe1e55 xorps xmm7, xmm7
0x140fe1e58 addss xmm6, xmm7
0x140fe1e5c jmp rsi
0x140fe1e5e movaps xmm6, xmm13
0x140fe1e62 movss xmm13, dword ptr [rdi + r13 + 0x14]
0x140fe1e69 movss xmm8, dword ptr [rax + 0x38]
0x140fe1e6f cmp rbx, qword ptr [rbp - 0x30]
0x140fe1e73 jne 0x140fe1d40
0x140fe1e79 jmp 0x140fe1ef7
0x140fe1e7b nop dword ptr [rax + rax]
0x140fe1e80 mulss xmm0, xmm13
0x140fe1e85 movss xmm15, dword ptr [rax + 0xc]
0x140fe1e8b cmp byte ptr [rax + 8], 0
0x140fe1e8f je 0x140fe1dfa
0x140fe1e95 mulss xmm15, xmm10
0x140fe1e9a mov rax, qword ptr [rdi + r13]
0x140fe1e9e movss xmm1, dword ptr [rax + 0x2c]
0x140fe1ea3 cmp byte ptr [rax + 0x28], 0
0x140fe1ea7 je 0x140fe1e0d
0x140fe1ead mulss xmm1, xmm13
0x140fe1eb2 movss xmm11, dword ptr [rax + 0x34]
0x140fe1eb8 cmp byte ptr [rax + 0x30], 0
0x140fe1ebc jne 0x140fe1e19
0x140fe1ec2 jmp 0x140fe1e1e
0x140fe1ec7 nop word ptr [rax + rax]
0x140fe1ed0 movaps xmm7, xmm13
0x140fe1ed4 subss xmm7, xmm6
0x140fe1ed8 mulss xmm7, dword ptr [rip + 0x1943990]
0x140fe1ee0 movss xmm13, dword ptr [rdi + r13 + 0x14]
0x140fe1ee7 movss xmm8, dword ptr [rax + 0x38]
0x140fe1eed cmp rbx, qword ptr [rbp - 0x30]
0x140fe1ef1 jne 0x140fe1d40
0x140fe1ef7 mov rcx, r14
0x140fe1efa lea rdx, [rip + 0x1be002f]
0x140fe1f01 call 0x14068d3d0
0x140fe1f06 jmp 0x140fe1d40
0x140fe1f0b nop dword ptr [rax + rax]
0x140fe1f10 movaps xmm7, xmm13
0x140fe1f14 subss xmm7, xmm6
0x140fe1f18 movss xmm13, dword ptr [rdi + r13 + 0x14]
0x140fe1f1f movss xmm8, dword ptr [rax + 0x38]
0x140fe1f25 cmp rbx, qword ptr [rbp - 0x30]
0x140fe1f29 jne 0x140fe1d40
0x140fe1f2f jmp 0x140fe1ef7
0x140fe1f31 mov rcx, qword ptr [rbp - 0x20]
0x140fe1f35 mov rax, qword ptr [rbp - 0x38]
0x140fe1f39 mov qword ptr [rax + 0x10], rcx
0x140fe1f3d movups xmm0, xmmword ptr [rbp - 0x30]
0x140fe1f41 movups xmmword ptr [rax], xmm0
0x140fe1f44 movaps xmm6, xmmword ptr [rbp]
0x140fe1f48 movaps xmm7, xmmword ptr [rbp + 0x10]
0x140fe1f4c movaps xmm8, xmmword ptr [rbp + 0x20]
0x140fe1f51 movaps xmm9, xmmword ptr [rbp + 0x30]
0x140fe1f56 movaps xmm10, xmmword ptr [rbp + 0x40]
0x140fe1f5b movaps xmm11, xmmword ptr [rbp + 0x50]
0x140fe1f60 movaps xmm12, xmmword ptr [rbp + 0x60]
0x140fe1f65 movaps xmm13, xmmword ptr [rbp + 0x70]
0x140fe1f6a movaps xmm14, xmmword ptr [rbp + 0x80]
0x140fe1f72 movaps xmm15, xmmword ptr [rbp + 0x90]
0x140fe1f7a add rsp, 0x128
0x140fe1f81 pop rbx
0x140fe1f82 pop rdi
0x140fe1f83 pop rsi
0x140fe1f84 pop r12
0x140fe1f86 pop r13
0x140fe1f88 pop r14
0x140fe1f8a pop r15
0x140fe1f8c pop rbp
0x140fe1f8d ret 
0x140fe1f8e lea r8, [rip + 0x1bdff83]
0x140fe1f95 mov rcx, r12
0x140fe1f98 mov rdx, r15
0x140fe1f9b call 0x1428d8fe3
0x140fe1fa0 int3 
0x140fe1fa1 nop word ptr cs:[rax + rax]
0x140fe1fb0 mov qword ptr [rsp + 0x10], rdx
0x140fe1fb5 push rbp
0x140fe1fb6 push r15
0x140fe1fb8 push r14
0x140fe1fba push r13
0x140fe1fbc push r12
0x140fe1fbe push rsi
0x140fe1fbf push rdi
0x140fe1fc0 push rbx
0x140fe1fc1 sub rsp, 0xc8
0x140fe1fc8 lea rbp, [rdx + 0x80]
0x140fe1fcf movaps xmmword ptr [rsp + 0x20], xmm15
0x140fe1fd5 movaps xmmword ptr [rsp + 0x30], xmm14
0x140fe1fdb movaps xmmword ptr [rsp + 0x40], xmm13
0x140fe1fe1 movaps xmmword ptr [rsp + 0x50], xmm12
0x140fe1fe7 movaps xmmword ptr [rsp + 0x60], xmm11
0x140fe1fed movaps xmmword ptr [rsp + 0x70], xmm10
0x140fe1ff3 movaps xmmword ptr [rsp + 0x80], xmm9
0x140fe1ffc movaps xmmword ptr [rsp + 0x90], xmm8
0x140fe2005 movaps xmmword ptr [rsp + 0xa0], xmm7
0x140fe200d movaps xmmword ptr [rsp + 0xb0], xmm6
0x140fe2015 mov rdx, qword ptr [rbp - 0x30]
0x140fe2019 test rdx, rdx
0x140fe201c je 0x140fe2031
0x140fe201e mov rcx, qword ptr [rbp - 0x28]
0x140fe2022 shl rdx, 4
0x140fe2026 mov r8d, 4
0x140fe202c call 0x140613c20
0x140fe2031 movaps xmm6, xmmword ptr [rsp + 0xb0]
0x140fe2039 movaps xmm7, xmmword ptr [rsp + 0xa0]
0x140fe2041 movaps xmm8, xmmword ptr [rsp + 0x90]
0x140fe204a movaps xmm9, xmmword ptr [rsp + 0x80]
0x140fe2053 movaps xmm10, xmmword ptr [rsp + 0x70]
0x140fe2059 movaps xmm11, xmmword ptr [rsp + 0x60]
0x140fe205f movaps xmm12, xmmword ptr [rsp + 0x50]
0x140fe2065 movaps xmm13, xmmword ptr [rsp + 0x40]
0x140fe206b movaps xmm14, xmmword ptr [rsp + 0x30]
0x140fe2071 movaps xmm15, xmmword ptr [rsp + 0x20]
0x140fe2077 add rsp, 0xc8
0x140fe207e pop rbx
0x140fe207f pop rdi
0x140fe2080 pop rsi
0x140fe2081 pop r12
0x140fe2083 pop r13
0x140fe2085 pop r14
0x140fe2087 pop r15
0x140fe2089 pop rbp
0x140fe208a ret 
