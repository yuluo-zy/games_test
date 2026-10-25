0x141405b00 55 push rbp
0x141405b01 56 push rsi
0x141405b02 4881ecd8000000 sub rsp, 0xd8
0x141405b09 488dac2480000000 lea rbp, [rsp + 0x80]
0x141405b11 48c74550feffffff mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
0x141405b19 4889ce mov rsi, rcx
0x141405b1c 48895530 mov qword ptr [rbp + 0x30], rdx
0x141405b20 4c894538 mov qword ptr [rbp + 0x38], r8
0x141405b24 48c7454000000000 mov qword ptr [rbp + 0x40], 0
0x141405b2c 48c7451800000000 mov qword ptr [rbp + 0x18], 0
0x141405b34 48c7452001000000 mov qword ptr [rbp + 0x20], 1
0x141405b3c 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x141405b44 c6454880 mov byte ptr [rbp + 0x48], 0x80
0x141405b48 488d4db0 lea rcx, [rbp - 0x50]
0x141405b4c 488d5518 lea rdx, [rbp + 0x18]
0x141405b50 e85bb70100 call 0x1414212b0
0x141405b55 31c0 xor eax, eax
0x141405b57 483b45b0 cmp rax, qword ptr [rbp - 0x50]
0x141405b5b 7117 jno 0x141405b74
0x141405b5d 488b45b8 mov rax, qword ptr [rbp - 0x48]
0x141405b61 48894608 mov qword ptr [rsi + 8], rax
0x141405b65 48b80000000000000080 movabs rax, 0x8000000000000000
0x141405b6f 488906 mov qword ptr [rsi], rax
0x141405b72 eb79 jmp 0x141405bed
0x141405b74 0f1045b0 movups xmm0, xmmword ptr [rbp - 0x50]
0x141405b78 0f104dc0 movups xmm1, xmmword ptr [rbp - 0x40]
0x141405b7c 0f1055d0 movups xmm2, xmmword ptr [rbp - 0x30]
0x141405b80 0f295500 movaps xmmword ptr [rbp], xmm2
0x141405b84 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x141405b88 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x141405b8c 488b4538 mov rax, qword ptr [rbp + 0x38]
0x141405b90 488b4d40 mov rcx, qword ptr [rbp + 0x40]
0x141405b94 4839c1 cmp rcx, rax
0x141405b97 733d jae 0x141405bd6
0x141405b99 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x141405b9d 48f7d8 neg rax
0x141405ba0 48ffc1 inc rcx
0x141405ba3 49b80026000001000000 movabs r8, 0x100002600
0x141405bad 0f1f00 nop dword ptr [rax]
0x141405bb0 440fb64c0aff movzx r9d, byte ptr [rdx + rcx - 1]
0x141405bb6 4983f920 cmp r9, 0x20
0x141405bba 7756 ja 0x141405c12
0x141405bbc 4d0fa3c8 bt r8, r9
0x141405bc0 7350 jae 0x141405c12
0x141405bc2 48894d40 mov qword ptr [rbp + 0x40], rcx
0x141405bc6 4c8d0c08 lea r9, [rax + rcx]
0x141405bca 49ffc1 inc r9
0x141405bcd 48ffc1 inc rcx
0x141405bd0 4983f901 cmp r9, 1
0x141405bd4 75da jne 0x141405bb0
0x141405bd6 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
0x141405bda 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
0x141405bde 0f285500 movaps xmm2, xmmword ptr [rbp]
0x141405be2 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x141405be6 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x141405bea 0f1106 movups xmmword ptr [rsi], xmm0
0x141405bed 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x141405bf1 4885d2 test rdx, rdx
0x141405bf4 740f je 0x141405c05
0x141405bf6 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x141405bfa 41b801000000 mov r8d, 1
0x141405c00 e81be020ff call 0x140613c20
0x141405c05 4889f0 mov rax, rsi
0x141405c08 4881c4d8000000 add rsp, 0xd8
0x141405c0f 5e pop rsi
0x141405c10 5d pop rbp
0x141405c11 c3 ret 
0x141405c12 48c745b013000000 mov qword ptr [rbp - 0x50], 0x13
0x141405c1a 488d4d18 lea rcx, [rbp + 0x18]
0x141405c1e 488d55b0 lea rdx, [rbp - 0x50]
0x141405c22 e8f9ac4901 call 0x1428a0920
0x141405c27 48894608 mov qword ptr [rsi + 8], rax
0x141405c2b 48b80000000000000080 movabs rax, 0x8000000000000000
0x141405c35 488906 mov qword ptr [rsi], rax
0x141405c38 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x141405c3c 4885c0 test rax, rax
0x141405c3f 7417 je 0x141405c58
0x141405c41 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x141405c45 48c1e002 shl rax, 2
0x141405c49 488d14c0 lea rdx, [rax + rax*8]
0x141405c4d 41b804000000 mov r8d, 4
0x141405c53 e8c8df20ff call 0x140613c20
0x141405c58 488b55f8 mov rdx, qword ptr [rbp - 8]
0x141405c5c 4885d2 test rdx, rdx
0x141405c5f 748c je 0x141405bed
0x141405c61 488b4d00 mov rcx, qword ptr [rbp]
0x141405c65 48c1e204 shl rdx, 4
0x141405c69 41b804000000 mov r8d, 4
0x141405c6f e8acdf20ff call 0x140613c20
0x141405c74 e974ffffff jmp 0x141405bed
0x141405c79 0f1f8000000000 nop dword ptr [rax]
0x141405c80 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141405c85 55 push rbp
0x141405c86 56 push rsi
0x141405c87 4883ec38 sub rsp, 0x38
0x141405c8b 488daa80000000 lea rbp, [rdx + 0x80]
0x141405c92 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x141405c96 4885d2 test rdx, rdx
0x141405c99 740f je 0x141405caa
0x141405c9b 488b4d20 mov rcx, qword ptr [rbp + 0x20]
0x141405c9f 41b801000000 mov r8d, 1
0x141405ca5 e876df20ff call 0x140613c20
0x141405caa 90 nop 
0x141405cab 4883c438 add rsp, 0x38
0x141405caf 5e pop rsi
0x141405cb0 5d pop rbp
0x141405cb1 c3 ret 
0x141405cb2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x141405cc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x141405cc5 55 push rbp
0x141405cc6 56 push rsi
0x141405cc7 4883ec38 sub rsp, 0x38
0x141405ccb 488daa80000000 lea rbp, [rdx + 0x80]
0x141405cd2 488d4de0 lea rcx, [rbp - 0x20]
0x141405cd6 e805ae48ff call 0x140890ae0
0x141405cdb 90 nop 
0x141405cdc 4883c438 add rsp, 0x38
0x141405ce0 5e pop rsi
0x141405ce1 5d pop rbp
0x141405ce2 c3 ret 
