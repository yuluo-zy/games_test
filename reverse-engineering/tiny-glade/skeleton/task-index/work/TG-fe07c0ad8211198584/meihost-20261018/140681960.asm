0x140681960 push rbp
0x140681961 push r15
0x140681963 push r14
0x140681965 push r13
0x140681967 push r12
0x140681969 push rsi
0x14068196a push rdi
0x14068196b push rbx
0x14068196c sub rsp, 0x98
0x140681973 lea rbp, [rsp + 0x80]
0x14068197b mov qword ptr [rbp + 0x10], 0xfffffffffffffffe
0x140681983 mov rdi, r8
0x140681986 mov rbx, rdx
0x140681989 mov qword ptr [rbp - 0x38], rcx
0x14068198d mov rax, qword ptr [r8 + 0x18]
0x140681991 movzx esi, byte ptr [rax]
0x140681994 mov rcx, rdx
0x140681997 call 0x140b92590
0x14068199c lea r14, [rbp - 0x30]
0x1406819a0 mov rcx, r14
0x1406819a3 call 0x140a82b20
0x1406819a8 mov r8d, esi
0x1406819ab and r8d, 1
0x1406819af shl r8d, 5
0x1406819b3 add r8d, 0xffffff20
0x1406819ba lea r15, [rbp - 0x58]
0x1406819be mov rcx, r15
0x1406819c1 mov rdx, r14
0x1406819c4 call 0x140b920b0
0x1406819c9 movss xmm2, dword ptr [rip + 0x22b2bd7] ; 0x1429345a8 0000204100000000000000000000803f f32=10.0 str=b''
0x1406819d1 mov rcx, r14
0x1406819d4 mov rdx, r15
0x1406819d7 call 0x140b92160
0x1406819dc mov r8, qword ptr [rdi + 0x10]
0x1406819e0 lea r9, [rip + 0x23957b1] ; 0x142a17198 90b1fa4001000000c800000000000000 f32=7.834175109863281 str=b'\x90\xb1\xfa@\x01'
0x1406819e7 mov rcx, rbx
0x1406819ea mov rdx, r14
0x1406819ed mov qword ptr [rbp - 0x40], r8
0x1406819f1 call 0x140b92340
0x1406819f6 mov r14, qword ptr [rbp - 0x28]
0x1406819fa mov r15, qword ptr [rbp - 0x20]
0x1406819fe movabs r12, 0x7ffffffffffffffe
0x140681a08 movabs r13, 0x8000000000000002
0x140681a12 test r15, r15
0x140681a15 je 0x140681a5d
0x140681a17 lea rdi, [r14 + 8]
0x140681a1b jmp 0x140681a29
0x140681a1d nop dword ptr [rax]
0x140681a20 add rdi, 0x20
0x140681a24 dec r15
0x140681a27 je 0x140681a5d
0x140681a29 mov rdx, qword ptr [rdi - 8]
0x140681a2d lea rcx, [rdx + r12]
0x140681a31 cmp rcx, 3
0x140681a35 setb al
0x140681a38 cmp rcx, 1
0x140681a3c setne cl
0x140681a3f cmp rdx, r13
0x140681a42 jl 0x140681a20
0x140681a44 and al, cl
0x140681a46 jne 0x140681a20
0x140681a48 test rdx, rdx
0x140681a4b je 0x140681a20
0x140681a4d mov rcx, qword ptr [rdi]
0x140681a50 mov r8d, 1
0x140681a56 call 0x140613c20
0x140681a5b jmp 0x140681a20
0x140681a5d mov rdx, qword ptr [rbp - 0x30]
0x140681a61 test rdx, rdx
0x140681a64 je 0x140681a78
0x140681a66 shl rdx, 5
0x140681a6a mov r8d, 8
0x140681a70 mov rcx, r14
0x140681a73 call 0x140613c20
0x140681a78 lea r14, [rbx + 0x18]
0x140681a7c mov rcx, r14
0x140681a7f call 0x140b92590
0x140681a84 lea r15, [rbp - 0x30]
0x140681a88 mov rcx, r15
0x140681a8b call 0x140a82b20
0x140681a90 mov r8, qword ptr [rbx + 0x30]
0x140681a94 mov r9, qword ptr [rbx + 0x38]
0x140681a98 mov byte ptr [rsp + 0x20], 1
0x140681a9d lea rcx, [rbp - 0x10]
0x140681aa1 mov rdx, r15
0x140681aa4 call 0x140fad6a0
0x140681aa9 test sil, 1
0x140681aad jne 0x140681b39
0x140681ab3 movss xmm0, dword ptr [rbx + 0x40]
0x140681ab8 movss xmm1, dword ptr [rip + 0x22a3e44] ; 0x142925904 0000803f9a99193f00000000feffffff f32=1.0 str=b''
0x140681ac0 ucomiss xmm1, xmm0
0x140681ac3 jbe 0x140681b39
0x140681ac5 mulss xmm0, dword ptr [rip + 0x231ed07] ; 0x1429a07d4 000080430000000000000000cdcc4c3e f32=256.0 str=b''
0x140681acd xorps xmm1, xmm1
0x140681ad0 maxss xmm1, xmm0
0x140681ad4 movss xmm0, dword ptr [rip + 0x22b9194] ; 0x14293ac70 00007f43000000000000000000000000 f32=255.0 str=b''
0x140681adc minss xmm0, xmm1
0x140681ae0 cvttss2si rax, xmm0
0x140681ae5 xorps xmm1, xmm1
0x140681ae8 xor ecx, ecx
0x140681aea ucomiss xmm0, xmm1
0x140681aed cmovae ecx, eax
0x140681af0 mov rax, qword ptr [rbp]
0x140681af4 mov qword ptr [rbp - 0x20], rax
0x140681af8 ucomiss xmm0, dword ptr [rip + 0x231ecd1] ; 0x1429a07d0 ffff7f4f000080430000000000000000 f32=4294967040.0 str=b'\xff\xff\x7fO'
0x140681aff mov r8d, 0xffffffff
0x140681b05 cmovbe r8d, ecx
0x140681b09 movups xmm0, xmmword ptr [rbp - 0x10]
0x140681b0d movaps xmmword ptr [rbp - 0x30], xmm0
0x140681b11 or r8d, 0xffffff00
0x140681b18 mov byte ptr [rbp + 0xf], 0
0x140681b1c lea rcx, [rbp - 0x58]
0x140681b20 lea rdx, [rbp - 0x30]
0x140681b24 call 0x140b920b0
0x140681b29 movups xmm0, xmmword ptr [rbp - 0x58]
0x140681b2d movaps xmmword ptr [rbp - 0x10], xmm0
0x140681b31 mov rax, qword ptr [rbp - 0x48]
0x140681b35 mov qword ptr [rbp], rax
0x140681b39 mov byte ptr [rbp + 0xf], 1
0x140681b3d lea r9, [rip + 0x2395654] ; 0x142a17198 90b1fa4001000000c800000000000000 f32=7.834175109863281 str=b'\x90\xb1\xfa@\x01'
0x140681b44 lea rdx, [rbp - 0x10]
0x140681b48 mov rcx, r14
0x140681b4b mov r8, qword ptr [rbp - 0x40]
0x140681b4f call 0x140b92340
0x140681b54 xorps xmm0, xmm0
0x140681b57 mov r14, qword ptr [rbp - 0x38]
0x140681b5b movups xmmword ptr [r14], xmm0
0x140681b5f mov rdi, qword ptr [rbp - 8]
0x140681b63 mov rsi, qword ptr [rbp]
0x140681b67 test rsi, rsi
0x140681b6a je 0x140681bbd
0x140681b6c lea rbx, [rdi + 8]
0x140681b70 jmp 0x140681b89
0x140681b72 nop word ptr cs:[rax + rax]
0x140681b80 add rbx, 0x20
0x140681b84 dec rsi
0x140681b87 je 0x140681bbd
0x140681b89 mov rdx, qword ptr [rbx - 8]
0x140681b8d lea rcx, [rdx + r12]
0x140681b91 cmp rcx, 3
0x140681b95 setb al
0x140681b98 cmp rcx, 1
0x140681b9c setne cl
0x140681b9f cmp rdx, r13
0x140681ba2 jl 0x140681b80
0x140681ba4 and al, cl
0x140681ba6 jne 0x140681b80
0x140681ba8 test rdx, rdx
0x140681bab je 0x140681b80
0x140681bad mov rcx, qword ptr [rbx]
0x140681bb0 mov r8d, 1
0x140681bb6 call 0x140613c20
0x140681bbb jmp 0x140681b80
0x140681bbd mov rdx, qword ptr [rbp - 0x10]
0x140681bc1 test rdx, rdx
0x140681bc4 je 0x140681bd8
0x140681bc6 shl rdx, 5
0x140681bca mov r8d, 8
0x140681bd0 mov rcx, rdi
0x140681bd3 call 0x140613c20
0x140681bd8 mov rax, r14
0x140681bdb add rsp, 0x98
0x140681be2 pop rbx
0x140681be3 pop rdi
0x140681be4 pop rsi
0x140681be5 pop r12
0x140681be7 pop r13
0x140681be9 pop r14
0x140681beb pop r15
0x140681bed pop rbp
0x140681bee ret 
0x140681bef nop 
0x140681bf0 mov qword ptr [rsp + 0x10], rdx
0x140681bf5 push rbp
0x140681bf6 push r15
0x140681bf8 push r14
0x140681bfa push r13
0x140681bfc push r12
0x140681bfe push rsi
0x140681bff push rdi
0x140681c00 push rbx
0x140681c01 sub rsp, 0x28
0x140681c05 lea rbp, [rdx + 0x80]
0x140681c0c cmp byte ptr [rbp + 0xf], 0
0x140681c10 jne 0x140681c23
0x140681c12 add rsp, 0x28
0x140681c16 pop rbx
0x140681c17 pop rdi
0x140681c18 pop rsi
0x140681c19 pop r12
0x140681c1b pop r13
0x140681c1d pop r14
0x140681c1f pop r15
0x140681c21 pop rbp
0x140681c22 ret 
0x140681c23 lea rcx, [rbp - 0x10]
0x140681c27 call 0x1405f11f0
0x140681c2c jmp 0x140681c12
0x140681c2e nop 
0x140681c30 mov qword ptr [rsp + 0x10], rdx
0x140681c35 push rbp
0x140681c36 push r15
0x140681c38 push r14
0x140681c3a push r13
0x140681c3c push r12
0x140681c3e push rsi
0x140681c3f push rdi
0x140681c40 push rbx
0x140681c41 sub rsp, 0x28
0x140681c45 lea rbp, [rdx + 0x80]
0x140681c4c lea rcx, [rbp - 0x30]
0x140681c50 call 0x1405f11f0
0x140681c55 nop 
0x140681c56 add rsp, 0x28
0x140681c5a pop rbx
0x140681c5b pop rdi
0x140681c5c pop rsi
0x140681c5d pop r12
0x140681c5f pop r13
0x140681c61 pop r14
0x140681c63 pop r15
0x140681c65 pop rbp
0x140681c66 ret 