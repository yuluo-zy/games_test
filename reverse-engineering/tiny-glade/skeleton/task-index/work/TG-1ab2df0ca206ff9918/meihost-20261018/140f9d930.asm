0x140f9d930 push rbp
0x140f9d931 push r14
0x140f9d933 push rsi
0x140f9d934 push rdi
0x140f9d935 push rbx
0x140f9d936 sub rsp, 0x290
0x140f9d93d lea rbp, [rsp + 0x80]
0x140f9d945 mov qword ptr [rbp + 0x208], 0xfffffffffffffffe
0x140f9d950 mov rdi, qword ptr [rcx]
0x140f9d953 cmp qword ptr [rdi + 0x1b8], 0
0x140f9d95b je 0x140f9da35
0x140f9d961 mov ebx, edx
0x140f9d963 mov rsi, rcx
0x140f9d966 lea rdx, [rcx + 8]
0x140f9d96a lea rcx, [rdi + 0x1c0]
0x140f9d971 call 0x1405b0d70
0x140f9d976 mov rcx, qword ptr [rdi + 0x1a0]
0x140f9d97d mov rdx, qword ptr [rdi + 0x1a8]
0x140f9d984 mov r8, rdx
0x140f9d987 and r8, rax
0x140f9d98a shr rax, 0x39
0x140f9d98e movd xmm0, eax
0x140f9d992 punpcklbw xmm0, xmm0
0x140f9d996 pshuflw xmm0, xmm0, 0
0x140f9d99b pshufd xmm0, xmm0, 0
0x140f9d9a0 lea rax, [rcx - 0xc0]
0x140f9d9a7 mov r9, qword ptr [rsi + 8]
0x140f9d9ab xor r10d, r10d
0x140f9d9ae pcmpeqd xmm1, xmm1
0x140f9d9b2 movdqu xmm2, xmmword ptr [rcx + r8]
0x140f9d9b8 movdqa xmm3, xmm2
0x140f9d9bc pcmpeqb xmm3, xmm0
0x140f9d9c0 pmovmskb r11d, xmm3
0x140f9d9c5 test r11d, r11d
0x140f9d9c8 je 0x140f9da00
0x140f9d9ca tzcnt esi, r11d
0x140f9d9cf add rsi, r8
0x140f9d9d2 and rsi, rdx
0x140f9d9d5 neg rsi
0x140f9d9d8 lea rsi, [rsi + rsi*2]
0x140f9d9dc shl rsi, 6
0x140f9d9e0 cmp r9, qword ptr [rax + rsi]
0x140f9d9e4 je 0x140f9da1e
0x140f9d9e6 lea esi, [r11 - 1]
0x140f9d9ea and si, r11w
0x140f9d9ee mov r11d, esi
0x140f9d9f1 jne 0x140f9d9ca
0x140f9d9f3 nop word ptr cs:[rax + rax]
0x140f9da00 pcmpeqb xmm2, xmm1
0x140f9da04 pmovmskb r11d, xmm2
0x140f9da09 test r11d, r11d
0x140f9da0c jne 0x140f9da35
0x140f9da0e add r8, r10
0x140f9da11 add r8, 0x10
0x140f9da15 add r10, 0x10
0x140f9da19 and r8, rdx
0x140f9da1c jmp 0x140f9d9b2
0x140f9da1e lea r14, [rcx + rsi]
0x140f9da22 mov rcx, qword ptr [rcx + rsi - 0x38]
0x140f9da27 test rcx, rcx
0x140f9da2a je 0x140f9da4d
0x140f9da2c mov rax, qword ptr [r14 - 0x30]
0x140f9da30 jmp 0x140f9daca
0x140f9da35 lea rcx, [rip + 0x1c11604] ; 0x142baf040 6e6f6465206578697374730000000000 f32=6.7422192063862815e+22
0x140f9da3c lea r8, [rip + 0x1c1160d] ; 0x142baf050 a0efba42010000002600000000000000 f32=93.468017578125
0x140f9da43 mov edx, 0xb
0x140f9da48 call 0x1428d9330
0x140f9da4d test bl, bl
0x140f9da4f jne 0x140f9db0a
0x140f9da55 mov r9, qword ptr [rdi + 0x260]
0x140f9da5c lock inc qword ptr [r9]
0x140f9da60 jle 0x140f9db65
0x140f9da66 lea rcx, [rbp - 0x58]
0x140f9da6a movd xmm1, dword ptr [rip + 0x1a793ce] ; 0x142a16e40 00004042000000000000000000000000 f32=48.0
0x140f9da72 xor r8d, r8d
0x140f9da75 call 0x140fac1f0
0x140f9da7a mov dword ptr [rbp + 0x200], 0xffffffff
0x140f9da84 movzx eax, byte ptr [rip + 0x269717e] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10
0x140f9da8b mov ecx, 0x260
0x140f9da90 mov edx, 8
0x140f9da95 call 0x140613c10
0x140f9da9a test rax, rax
0x140f9da9d je 0x140f9db56
0x140f9daa3 lea rdx, [rbp - 0x58]
0x140f9daa7 mov r8d, 0x260
0x140f9daad mov rsi, rax
0x140f9dab0 mov rcx, rax
0x140f9dab3 call 0x142923c00
0x140f9dab8 mov rcx, rsi
0x140f9dabb mov qword ptr [r14 - 0x38], rsi
0x140f9dabf lea rax, [rip + 0x1c115f2] ; 0x142baf0b8 b06efa40010000006002000000000000 f32=7.826011657714844
0x140f9dac6 mov qword ptr [r14 - 0x30], rax
0x140f9daca call qword ptr [rax + 0x28]
0x140f9dacd mov rsi, rax
0x140f9dad0 mov rcx, rax
0x140f9dad3 call qword ptr [rdx + 0x18]
0x140f9dad6 test rsi, rsi
0x140f9dad9 je 0x140f9db3e
0x140f9dadb movabs r8, 0xdc603d951b9380b2
0x140f9dae5 cmp rax, r8
0x140f9dae8 jne 0x140f9db3e
0x140f9daea movabs rcx, 0xce69dbf0aee13da2
0x140f9daf4 cmp rdx, rcx
0x140f9daf7 jne 0x140f9db3e
0x140f9daf9 mov rax, rsi
0x140f9dafc add rsp, 0x290
0x140f9db03 pop rbx
0x140f9db04 pop rdi
0x140f9db05 pop rsi
0x140f9db06 pop r14
0x140f9db08 pop rbp
0x140f9db09 ret 
0x140f9db0a lea rax, [rip + 0x1c11667] ; 0x142baf178 28f1ba42010000004b00000000000000 f32=93.47100830078125
0x140f9db11 mov qword ptr [rbp - 0x58], rax
0x140f9db15 mov qword ptr [rbp - 0x50], 1
0x140f9db1d mov qword ptr [rbp - 0x48], 8
0x140f9db25 pxor xmm0, xmm0
0x140f9db29 movdqu xmmword ptr [rbp - 0x40], xmm0
0x140f9db2e lea rdx, [rip + 0x1c11653] ; 0x142baf188 a0efba42010000002600000000000000 f32=93.468017578125
0x140f9db35 lea rcx, [rbp - 0x58]
0x140f9db39 call 0x1428d9390
0x140f9db3e lea rcx, [rip + 0x1c11523] ; 0x142baf068 646f776e63617374a0efba4201000000 f32=1.9144365107124436e+28
0x140f9db45 lea r8, [rip + 0x1c11524] ; 0x142baf070 a0efba42010000002600000000000000 f32=93.468017578125
0x140f9db4c mov edx, 8
0x140f9db51 call 0x1428d9330
0x140f9db56 mov ecx, 8
0x140f9db5b mov edx, 0x260
0x140f9db60 call 0x1428d9000
0x140f9db65 ud2 
0x140f9db67 nop word ptr [rax + rax]
0x140f9db70 mov qword ptr [rsp + 0x10], rdx
0x140f9db75 push rbp
0x140f9db76 push r14
0x140f9db78 push rsi
0x140f9db79 push rdi
0x140f9db7a push rbx
0x140f9db7b sub rsp, 0x20
0x140f9db7f lea rbp, [rdx + 0x80]
0x140f9db86 lea rcx, [rbp - 0x58]
0x140f9db8a call 0x140fa47e0
0x140f9db8f nop 
0x140f9db90 add rsp, 0x20
0x140f9db94 pop rbx
0x140f9db95 pop rdi
0x140f9db96 pop rsi
0x140f9db97 pop r14
0x140f9db99 pop rbp
0x140f9db9a ret 