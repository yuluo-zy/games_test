0x140f9e130 push rbp
0x140f9e131 push r14
0x140f9e133 push rsi
0x140f9e134 push rdi
0x140f9e135 push rbx
0x140f9e136 sub rsp, 0x80
0x140f9e13d lea rbp, [rsp + 0x80]
0x140f9e145 mov qword ptr [rbp - 8], 0xfffffffffffffffe
0x140f9e14d mov rdi, qword ptr [rcx]
0x140f9e150 cmp qword ptr [rdi + 0x1b8], 0
0x140f9e158 je 0x140f9e225
0x140f9e15e mov ebx, edx
0x140f9e160 mov rsi, rcx
0x140f9e163 lea rdx, [rcx + 8]
0x140f9e167 lea rcx, [rdi + 0x1c0]
0x140f9e16e call 0x1405b0d70
0x140f9e173 mov rcx, qword ptr [rdi + 0x1a0]
0x140f9e17a mov rdx, qword ptr [rdi + 0x1a8]
0x140f9e181 mov r8, rdx
0x140f9e184 and r8, rax
0x140f9e187 shr rax, 0x39
0x140f9e18b movd xmm0, eax
0x140f9e18f punpcklbw xmm0, xmm0
0x140f9e193 pshuflw xmm0, xmm0, 0
0x140f9e198 pshufd xmm0, xmm0, 0
0x140f9e19d lea rax, [rcx - 0xc0]
0x140f9e1a4 mov r9, qword ptr [rsi + 8]
0x140f9e1a8 xor r10d, r10d
0x140f9e1ab pcmpeqd xmm1, xmm1
0x140f9e1af movdqu xmm2, xmmword ptr [rcx + r8]
0x140f9e1b5 movdqa xmm3, xmm2
0x140f9e1b9 pcmpeqb xmm3, xmm0
0x140f9e1bd pmovmskb r11d, xmm3
0x140f9e1c2 test r11d, r11d
0x140f9e1c5 je 0x140f9e1f0
0x140f9e1c7 tzcnt esi, r11d
0x140f9e1cc add rsi, r8
0x140f9e1cf and rsi, rdx
0x140f9e1d2 neg rsi
0x140f9e1d5 lea r14, [rsi + rsi*2]
0x140f9e1d9 shl r14, 6
0x140f9e1dd cmp r9, qword ptr [rax + r14]
0x140f9e1e1 je 0x140f9e20e
0x140f9e1e3 lea esi, [r11 - 1]
0x140f9e1e7 and si, r11w
0x140f9e1eb mov r11d, esi
0x140f9e1ee jne 0x140f9e1c7
0x140f9e1f0 pcmpeqb xmm2, xmm1
0x140f9e1f4 pmovmskb r11d, xmm2
0x140f9e1f9 test r11d, r11d
0x140f9e1fc jne 0x140f9e225
0x140f9e1fe add r8, r10
0x140f9e201 add r8, 0x10
0x140f9e205 add r10, 0x10
0x140f9e209 and r8, rdx
0x140f9e20c jmp 0x140f9e1af
0x140f9e20e lea rsi, [rcx + r14]
0x140f9e212 mov rcx, qword ptr [rcx + r14 - 0x38]
0x140f9e217 test rcx, rcx
0x140f9e21a je 0x140f9e23d
0x140f9e21c mov rax, qword ptr [rsi - 0x30]
0x140f9e220 jmp 0x140f9e310
0x140f9e225 lea rcx, [rip + 0x1c10e14] ; 0x142baf040 6e6f6465206578697374730000000000 f32=6.7422192063862815e+22 str=b'node exists'
0x140f9e22c lea r8, [rip + 0x1c10e1d] ; 0x142baf050 a0efba42010000002600000000000000 f32=93.468017578125 str=b'\xa0\xef\xbaB\x01'
0x140f9e233 mov edx, 0xb
0x140f9e238 call 0x1428d9330
0x140f9e23d test bl, bl
0x140f9e23f jne 0x140f9e350
0x140f9e245 mov rax, qword ptr [rdi + 0x260]
0x140f9e24c lock inc qword ptr [rax]
0x140f9e250 jle 0x140f9e3ab
0x140f9e256 mov qword ptr [rbp - 0x58], rax
0x140f9e25a mov qword ptr [rbp - 0x50], 0
0x140f9e262 mov qword ptr [rbp - 0x48], 1
0x140f9e26a mov qword ptr [rbp - 0x40], 0
0x140f9e272 mov dword ptr [rbp - 0x38], 0
0x140f9e279 mov byte ptr [rbp - 0x34], 0
0x140f9e27d mov dword ptr [rbp - 0x10], 0xffffffff
0x140f9e284 mov byte ptr [rbp - 0xc], 0
0x140f9e288 mov qword ptr [rbp - 0x30], 0
0x140f9e290 mov qword ptr [rbp - 0x28], 4
0x140f9e298 pxor xmm0, xmm0
0x140f9e29c movdqu xmmword ptr [rbp - 0x20], xmm0
0x140f9e2a1 lock dec qword ptr [rax]
0x140f9e2a5 jne 0x140f9e2b0
0x140f9e2a7 lea rcx, [rbp - 0x58]
0x140f9e2ab call 0x140b8e120
0x140f9e2b0 movzx eax, byte ptr [rip + 0x2696952] ; 0x143634c09 5a1903301400006a140000745a190370 f32=4.769354911005053e-10 str=b'Z\x19\x030\x14'
0x140f9e2b7 mov ecx, 0x48
0x140f9e2bc mov edx, 8
0x140f9e2c1 call 0x140613c10
0x140f9e2c6 test rax, rax
0x140f9e2c9 je 0x140f9e39c
0x140f9e2cf mov rcx, rax
0x140f9e2d2 mov rax, qword ptr [rbp - 0x10]
0x140f9e2d6 mov qword ptr [rcx + 0x40], rax
0x140f9e2da movdqu xmm0, xmmword ptr [rbp - 0x50]
0x140f9e2df movdqu xmm1, xmmword ptr [rbp - 0x40]
0x140f9e2e4 movdqu xmm2, xmmword ptr [rbp - 0x30]
0x140f9e2e9 movdqu xmm3, xmmword ptr [rbp - 0x20]
0x140f9e2ee movdqu xmmword ptr [rcx + 0x30], xmm3
0x140f9e2f3 movdqu xmmword ptr [rcx + 0x20], xmm2
0x140f9e2f8 movdqu xmmword ptr [rcx + 0x10], xmm1
0x140f9e2fd movdqu xmmword ptr [rcx], xmm0
0x140f9e301 mov qword ptr [rsi - 0x38], rcx
0x140f9e305 lea rax, [rip + 0x1c11324] ; 0x142baf630 90195f40010000004800000000000000 f32=3.4859352111816406 str=b'\x90\x19_@\x01'
0x140f9e30c mov qword ptr [rsi - 0x30], rax
0x140f9e310 call qword ptr [rax + 0x28]
0x140f9e313 mov rsi, rax
0x140f9e316 mov rcx, rax
0x140f9e319 call qword ptr [rdx + 0x18]
0x140f9e31c test rsi, rsi
0x140f9e31f je 0x140f9e384
0x140f9e321 movabs r8, 0x12ddabfdd3347ef1
0x140f9e32b cmp rax, r8
0x140f9e32e jne 0x140f9e384
0x140f9e330 movabs rcx, 0x1f6d60c8858b8ff
0x140f9e33a cmp rdx, rcx
0x140f9e33d jne 0x140f9e384
0x140f9e33f mov rax, rsi
0x140f9e342 add rsp, 0x80
0x140f9e349 pop rbx
0x140f9e34a pop rdi
0x140f9e34b pop rsi
0x140f9e34c pop r14
0x140f9e34e pop rbp
0x140f9e34f ret 
0x140f9e350 lea rax, [rip + 0x1c10e21] ; 0x142baf178 28f1ba42010000004b00000000000000 f32=93.47100830078125 str=b'(\xf1\xbaB\x01'
0x140f9e357 mov qword ptr [rbp - 0x50], rax
0x140f9e35b mov qword ptr [rbp - 0x48], 1
0x140f9e363 mov qword ptr [rbp - 0x40], 8
0x140f9e36b pxor xmm0, xmm0
0x140f9e36f movdqu xmmword ptr [rbp - 0x38], xmm0
0x140f9e374 lea rdx, [rip + 0x1c10e0d] ; 0x142baf188 a0efba42010000002600000000000000 f32=93.468017578125 str=b'\xa0\xef\xbaB\x01'
0x140f9e37b lea rcx, [rbp - 0x50]
0x140f9e37f call 0x1428d9390
0x140f9e384 lea rcx, [rip + 0x1c10cdd] ; 0x142baf068 646f776e63617374a0efba4201000000 f32=1.9144365107124436e+28 str=b'downcast\xa0\xef\xbaB\x01'
0x140f9e38b lea r8, [rip + 0x1c10cde] ; 0x142baf070 a0efba42010000002600000000000000 f32=93.468017578125 str=b'\xa0\xef\xbaB\x01'
0x140f9e392 mov edx, 8
0x140f9e397 call 0x1428d9330
0x140f9e39c mov ecx, 8
0x140f9e3a1 mov edx, 0x48
0x140f9e3a6 call 0x1428d9000
0x140f9e3ab ud2 
0x140f9e3ad nop dword ptr [rax]
0x140f9e3b0 mov qword ptr [rsp + 0x10], rdx
0x140f9e3b5 push rbp
0x140f9e3b6 push r14
0x140f9e3b8 push rsi
0x140f9e3b9 push rdi
0x140f9e3ba push rbx
0x140f9e3bb sub rsp, 0x20
0x140f9e3bf lea rbp, [rdx + 0x80]
0x140f9e3c6 lea rcx, [rbp - 0x50]
0x140f9e3ca call 0x1405f1990
0x140f9e3cf nop 
0x140f9e3d0 add rsp, 0x20
0x140f9e3d4 pop rbx
0x140f9e3d5 pop rdi
0x140f9e3d6 pop rsi
0x140f9e3d7 pop r14
0x140f9e3d9 pop rbp
0x140f9e3da ret 