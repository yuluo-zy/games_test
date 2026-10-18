0x141591cb0 55 push rbp
0x141591cb1 4156 push r14
0x141591cb3 56 push rsi
0x141591cb4 57 push rdi
0x141591cb5 53 push rbx
0x141591cb6 4881ecc0000000 sub rsp, 0xc0
0x141591cbd 488dac2480000000 lea rbp, [rsp + 0x80]
0x141591cc5 48c74538feffffff mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
0x141591ccd 488b9d98000000 mov rbx, qword ptr [rbp + 0x98]
0x141591cd4 48c745f800000000 mov qword ptr [rbp - 8], 0
0x141591cdc 48c7450800000000 mov qword ptr [rbp + 8], 0
0x141591ce4 4885db test rbx, rbx
0x141591ce7 0f8869010000 js 0x141591e56
0x141591ced 4889ce mov rsi, rcx
0x141591cf0 7427 je 0x141591d19
0x141591cf2 0fb605102f0a02 movzx eax, byte ptr [rip + 0x20a2f10]
0x141591cf9 bf01000000 mov edi, 1
0x141591cfe ba01000000 mov edx, 1
0x141591d03 4889d9 mov rcx, rbx
0x141591d06 e8051f08ff call 0x140613c10
0x141591d0b 4885c0 test rax, rax
0x141591d0e 0f8444010000 je 0x141591e58
0x141591d14 4989c6 mov r14, rax
0x141591d17 eb06 jmp 0x141591d1f
0x141591d19 41be01000000 mov r14d, 1
0x141591d1f 488bbda0000000 mov rdi, qword ptr [rbp + 0xa0]
0x141591d26 488b9590000000 mov rdx, qword ptr [rbp + 0x90]
0x141591d2d 4c89f1 mov rcx, r14
0x141591d30 4989d8 mov r8, rbx
0x141591d33 e8c81e3901 call 0x142923c00
0x141591d38 48895dc0 mov qword ptr [rbp - 0x40], rbx
0x141591d3c 4c897530 mov qword ptr [rbp + 0x30], r14
0x141591d40 4c8975c8 mov qword ptr [rbp - 0x38], r14
0x141591d44 48895dd0 mov qword ptr [rbp - 0x30], rbx
0x141591d48 488d4d10 lea rcx, [rbp + 0x10]
0x141591d4c 4889fa mov rdx, rdi
0x141591d4f e8ec5b1500 call 0x1416e7940
0x141591d54 807d1006 cmp byte ptr [rbp + 0x10], 6
0x141591d58 7534 jne 0x141591d8e
0x141591d5a 488b4518 mov rax, qword ptr [rbp + 0x18]
0x141591d5e 48894608 mov qword ptr [rsi + 8], rax
0x141591d62 c60606 mov byte ptr [rsi], 6
0x141591d65 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
0x141591d6c 4885d2 test rdx, rdx
0x141591d6f 740f je 0x141591d80
0x141591d71 41b801000000 mov r8d, 1
0x141591d77 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x141591d7b e8a01e08ff call 0x140613c20
0x141591d80 488d4df8 lea rcx, [rbp - 8]
0x141591d84 e8b77130ff call 0x140898f40
0x141591d89 e9b7000000 jmp 0x141591e45
0x141591d8e 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x141591d92 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x141591d96 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x141591d9a 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x141591d9e 488d4dd8 lea rcx, [rbp - 0x28]
0x141591da2 488d55f8 lea rdx, [rbp - 8]
0x141591da6 4c8d45c0 lea r8, [rbp - 0x40]
0x141591daa 4c8d4da0 lea r9, [rbp - 0x60]
0x141591dae e87d9ceaff call 0x14143ba30
0x141591db3 0fb645d8 movzx eax, byte ptr [rbp - 0x28]
0x141591db7 4883f806 cmp rax, 6
0x141591dbb 7754 ja 0x141591e11
0x141591dbd 488d0d58666e01 lea rcx, [rip + 0x16e6658]
0x141591dc4 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x141591dc8 4801c8 add rax, rcx
0x141591dcb ffe0 jmp rax
0x141591dcd 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x141591dd1 4885d2 test rdx, rdx
0x141591dd4 7444 je 0x141591e1a
0x141591dd6 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x141591dda 41b801000000 mov r8d, 1
0x141591de0 eb28 jmp 0x141591e0a
0x141591de2 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x141591de6 488b55f0 mov rdx, qword ptr [rbp - 0x10]
0x141591dea 48894d30 mov qword ptr [rbp + 0x30], rcx
0x141591dee e87d6f2fff call 0x140888d70
0x141591df3 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x141591df7 4885d2 test rdx, rdx
0x141591dfa 741e je 0x141591e1a
0x141591dfc 48c1e205 shl rdx, 5
0x141591e00 41b808000000 mov r8d, 8
0x141591e06 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x141591e0a e8111e08ff call 0x140613c20
0x141591e0f eb09 jmp 0x141591e1a
0x141591e11 488d4de0 lea rcx, [rbp - 0x20]
0x141591e15 e8267130ff call 0x140898f40
0x141591e1a 488b4508 mov rax, qword ptr [rbp + 8]
0x141591e1e 48894527 mov qword ptr [rbp + 0x27], rax
0x141591e22 0f1045f8 movups xmm0, xmmword ptr [rbp - 8]
0x141591e26 0f114517 movups xmmword ptr [rbp + 0x17], xmm0
0x141591e2a c60605 mov byte ptr [rsi], 5
0x141591e2d 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x141591e31 488b451f mov rax, qword ptr [rbp + 0x1f]
0x141591e35 488b4d27 mov rcx, qword ptr [rbp + 0x27]
0x141591e39 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x141591e3d 48894610 mov qword ptr [rsi + 0x10], rax
0x141591e41 48894e18 mov qword ptr [rsi + 0x18], rcx
0x141591e45 4889f0 mov rax, rsi
0x141591e48 4881c4c0000000 add rsp, 0xc0
0x141591e4f 5b pop rbx
0x141591e50 5f pop rdi
0x141591e51 5e pop rsi
0x141591e52 415e pop r14
0x141591e54 5d pop rbp
0x141591e55 c3 ret 
0x141591e56 31ff xor edi, edi
0x141591e58 4c8d0579636e01 lea r8, [rip + 0x16e6379]
0x141591e5f 4889f9 mov rcx, rdi
0x141591e62 4889da mov rdx, rbx
0x141591e65 e879713401 call 0x1428d8fe3
0x141591e6a 0f0b ud2 
0x141591e6c 0f1f4000 nop dword ptr [rax]
0x141591e70 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141591e75 55 push rbp
0x141591e76 4156 push r14
0x141591e78 56 push rsi
0x141591e79 57 push rdi
0x141591e7a 53 push rbx
0x141591e7b 4883ec20 sub rsp, 0x20
0x141591e7f 488daa80000000 lea rbp, [rdx + 0x80]
0x141591e86 4883bd9800000000 cmp qword ptr [rbp + 0x98], 0
0x141591e8e 7416 je 0x141591ea6
0x141591e90 41b801000000 mov r8d, 1
0x141591e96 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x141591e9a 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
0x141591ea1 e87a1d08ff call 0x140613c20
0x141591ea6 90 nop 
0x141591ea7 4883c420 add rsp, 0x20
0x141591eab 5b pop rbx
0x141591eac 5f pop rdi
0x141591ead 5e pop rsi
0x141591eae 415e pop r14
0x141591eb0 5d pop rbp
0x141591eb1 c3 ret 
0x141591eb2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141591ec0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141591ec5 55 push rbp
0x141591ec6 4156 push r14
0x141591ec8 56 push rsi
0x141591ec9 57 push rdi
0x141591eca 53 push rbx
0x141591ecb 4883ec20 sub rsp, 0x20
0x141591ecf 488daa80000000 lea rbp, [rdx + 0x80]
0x141591ed6 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x141591eda 4885d2 test rdx, rdx
0x141591edd 7413 je 0x141591ef2
0x141591edf 48c1e205 shl rdx, 5
0x141591ee3 41b808000000 mov r8d, 8
0x141591ee9 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x141591eed e82e1d08ff call 0x140613c20
0x141591ef2 90 nop 
0x141591ef3 4883c420 add rsp, 0x20
0x141591ef7 5b pop rbx
0x141591ef8 5f pop rdi
0x141591ef9 5e pop rsi
0x141591efa 415e pop r14
0x141591efc 5d pop rbp
0x141591efd c3 ret 
0x141591efe 6690 nop 
0x141591f00 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141591f05 55 push rbp
0x141591f06 4156 push r14
0x141591f08 56 push rsi
0x141591f09 57 push rdi
0x141591f0a 53 push rbx
0x141591f0b 4883ec20 sub rsp, 0x20
0x141591f0f 488daa80000000 lea rbp, [rdx + 0x80]
0x141591f16 488d4df8 lea rcx, [rbp - 8]
0x141591f1a e8217030ff call 0x140898f40
0x141591f1f 90 nop 
0x141591f20 4883c420 add rsp, 0x20
0x141591f24 5b pop rbx
0x141591f25 5f pop rdi
0x141591f26 5e pop rsi
0x141591f27 415e pop r14
0x141591f29 5d pop rbp
0x141591f2a c3 ret 
