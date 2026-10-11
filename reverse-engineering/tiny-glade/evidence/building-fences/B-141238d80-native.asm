141238d80 push rbp
141238d81 push r15
141238d83 push r14
141238d85 push r13
141238d87 push r12
141238d89 push rsi
141238d8a push rdi
141238d8b push rbx
141238d8c sub rsp, 0x2a8
141238d93 lea rbp, [rsp + 0x80]
141238d9b movaps xmmword ptr [rbp + 0x210], xmm15
141238da3 movaps xmmword ptr [rbp + 0x200], xmm14
141238dab movaps xmmword ptr [rbp + 0x1f0], xmm13
141238db3 movaps xmmword ptr [rbp + 0x1e0], xmm12
141238dbb movaps xmmword ptr [rbp + 0x1d0], xmm11
141238dc3 movaps xmmword ptr [rbp + 0x1c0], xmm10
141238dcb movaps xmmword ptr [rbp + 0x1b0], xmm9
141238dd3 movaps xmmword ptr [rbp + 0x1a0], xmm8
141238ddb movaps xmmword ptr [rbp + 0x190], xmm7
141238de2 movaps xmmword ptr [rbp + 0x180], xmm6
141238de9 mov qword ptr [rbp + 0x178], 0xfffffffffffffffe
141238df4 mov qword ptr [rbp + 0x40], rcx
141238df8 cmp r9, 2
141238dfc jae 0x141238e05
141238dfe xor eax, eax
141238e00 jmp 0x141239eb1
141238e05 mov rbx, r9
141238e08 mov r14, r8
141238e0b movaps xmm6, xmm1
141238e0e mov rcx, qword ptr [rbp + 0x298]
141238e15 mov rax, rcx
141238e18 shr rax, 0x3f
141238e1c lea rcx, [rcx*2 + 1]
141238e24 mov qword ptr [rbp - 0x18], rax
141238e28 mov qword ptr [rbp - 0x20], rcx
141238e2c lea rcx, [rbp - 0x48]
141238e30 movss xmm3, dword ptr [rip + 0x19e5c98]
141238e38 mov rdx, r8
141238e3b mov r8, r9
141238e3e call 0x141fdf9e0
141238e43 xor edi, edi
141238e45 mov ecx, 0x48
141238e4a mov rax, rbx
141238e4d mul rcx
141238e50 mov rsi, rax
141238e53 jo 0x141238e8f
141238e55 movabs rax, 0x7ffffffffffffffc
141238e5f cmp rsi, rax
141238e62 ja 0x141238e8f
141238e64 test rsi, rsi
141238e67 mov qword ptr [rbp + 0x20], rbx
141238e6b mov qword ptr [rbp - 0x10], r14
141238e6f je 0x141238ea6
141238e71 movzx eax, byte ptr [rip + 0x23fbd91]
141238e78 mov edi, 4
141238e7d mov edx, 4
141238e82 mov rcx, rsi
141238e85 call 0x140613c10
141238e8a test rax, rax
141238e8d jne 0x141238ead
141238e8f lea r8, [rip + 0x19e5eba]
141238e96 mov rcx, rdi
141238e99 mov rdx, rsi
141238e9c call 0x1428d8fe3
141238ea1 jmp 0x14123a020
141238ea6 mov eax, 4
141238eab xor ebx, ebx
141238ead mov rdx, qword ptr [rbp + 0x2b0]
141238eb4 mov qword ptr [rbp + 0xf0], rdx
141238ebb addss xmm6, dword ptr [rip + 0x16eca15]
141238ec3 mov qword ptr [rbp + 0x150], rbx
141238eca mov qword ptr [rbp + 0x158], rax
141238ed1 mov qword ptr [rbp + 0x160], 0
141238edc mov rdx, qword ptr [rbp - 0x40]
141238ee0 mov rax, qword ptr [rbp - 0x38]
141238ee4 lea rax, [rdx + rax*8]
141238ee8 mov qword ptr [rbp + 0x10], rax
141238eec movaps xmmword ptr [rbp], xmm6
141238ef0 movsldup xmm13, xmm6
141238ef5 mov al, 1
141238ef7 mov qword ptr [rbp + 0xf8], 0
141238f02 lea r12, [rbp - 0x20]
141238f06 movss xmm15, dword ptr [rip + 0x16eca61]
141238f0f mov qword ptr [rbp + 0x148], 0
141238f1a mov qword ptr [rbp - 0x28], rdx
141238f1e mov rsi, rdx
141238f21 jmp 0x141238f47
141238f23 nop word ptr cs:[rax + rax]
141238f30 xor eax, eax
141238f32 mov rcx, qword ptr [rbp + 0x90]
141238f39 mov qword ptr [rbp + 0x148], rcx
141238f40 mov rcx, qword ptr [rbp + 0x88]
141238f47 cmp rsi, qword ptr [rbp + 0x10]
141238f4b je 0x14123965b
141238f51 lea rdx, [rsi + 8]
141238f55 test al, 1
141238f57 je 0x141238f70
141238f59 cmp rdx, qword ptr [rbp + 0x10]
141238f5d je 0x14123965b
141238f63 mov rcx, rsi
141238f66 lea rax, [rsi + 0x10]
141238f6a mov rsi, rdx
141238f6d mov rdx, rax
141238f70 mov rax, qword ptr [rsi]
141238f73 cmp rax, -1
141238f77 je 0x141239f13
141238f7d mov qword ptr [rbp + 0x88], rsi
141238f84 mov qword ptr [rbp + 0x80], rdx
141238f8b mov rbx, qword ptr [rcx]
141238f8e lea rdx, [rax + 1]
141238f92 mov rsi, rdx
141238f95 sub rsi, rbx
141238f98 jb 0x141239f24
141238f9e cmp rax, qword ptr [rbp + 0x20]
141238fa2 jae 0x141239f38
141238fa8 mov rax, rsi
141238fab mov ecx, 0xc
141238fb0 mul rcx
141238fb3 jo 0x14123a009
141238fb9 mov r14, rax
141238fbc movabs rax, 0x7ffffffffffffffc
141238fc6 cmp r14, rax
141238fc9 ja 0x14123a009
141238fcf test r14, r14
141238fd2 je 0x141239000
141238fd4 movzx eax, byte ptr [rip + 0x23fbc2e]
141238fdb mov edx, 4
141238fe0 mov rcx, r14
141238fe3 call 0x140613c10
141238fe8 test rax, rax
141238feb je 0x141239ff9
141238ff1 mov r15, rsi
141238ff4 jmp 0x141239008
141238ff6 nop word ptr cs:[rax + rax]
141239000 mov eax, 4
141239005 xor r15d, r15d
141239008 lea rdx, [rbx + rbx*2]
14123900c mov rcx, qword ptr [rbp - 0x10]
141239010 lea rdx, [rcx + rdx*4]
141239014 mov rcx, rax
141239017 mov rbx, rax
14123901a mov r8, r14
14123901d call 0x142923c00
141239022 mov qword ptr [rbp + 0x100], r15
141239029 mov qword ptr [rbp + 0x30], rbx
14123902d mov qword ptr [rbp + 0x108], rbx
141239034 mov qword ptr [rbp + 0x110], rsi
14123903b lea rcx, [rbp + 0xb0]
141239042 lea rdx, [rbp + 0x100]
141239049 xor r8d, r8d
14123904c call 0x1411f9be0
141239051 xor eax, eax
141239053 cmp rax, qword ptr [rbp + 0xb0]
14123905a jo 0x141239fbe
141239060 mov rax, qword ptr [rbp + 0xe0]
141239067 mov qword ptr [rbp + 0x130], rax
14123906e movups xmm0, xmmword ptr [rbp + 0xb0]
141239075 movups xmm1, xmmword ptr [rbp + 0xc0]
14123907c movups xmm2, xmmword ptr [rbp + 0xd0]
141239083 movaps xmmword ptr [rbp + 0x120], xmm2
14123908a movaps xmmword ptr [rbp + 0x110], xmm1
141239091 movaps xmmword ptr [rbp + 0x100], xmm0
141239098 mov rcx, qword ptr [rbp + 0x290]
14123909f call 0x1412402b0
1412390a4 addss xmm0, dword ptr [rip + 0x1721038]
1412390ac divss xmm0, dword ptr [rip + 0x16ec8d4]
1412390b4 xorps xmm1, xmm1
1412390b7 maxss xmm1, xmm0
1412390bb movss xmm3, dword ptr [rip + 0x16ec841]
1412390c3 movaps xmm0, xmm3
1412390c6 minss xmm0, xmm1
1412390ca movaps xmm1, xmm0
1412390cd mulss xmm1, xmm0
1412390d1 addss xmm0, xmm0
1412390d5 movss xmm2, dword ptr [rip + 0x1793f9b]
1412390dd subss xmm2, xmm0
1412390e1 mulss xmm2, xmm1
1412390e5 movaps xmm0, xmm3
1412390e8 subss xmm0, xmm2
1412390ec mulss xmm0, dword ptr [rip + 0x1793f3c]
1412390f4 mulss xmm2, dword ptr [rip + 0x19e59d8]
1412390fc addss xmm2, xmm0
141239100 movss xmm7, dword ptr [rbp + 0x130]
141239108 movaps xmm0, xmm7
14123910b divss xmm0, xmm2
14123910f call 0x142923e60
141239114 divss xmm7, xmm0
141239118 lea rcx, [rbp + 0xb0]
14123911f lea rdx, [rbp + 0x100]
141239126 movaps xmm2, xmm7
141239129 call 0x1411f8ed0
14123912e mov rax, qword ptr [rbp + 0x100]
141239135 test rax, rax
141239138 je 0x141239154
14123913a mov rcx, qword ptr [rbp + 0x108]
141239141 shl rax, 2
141239145 lea rdx, [rax + rax*2]
141239149 mov r8d, 4
14123914f call 0x140613c20
141239154 mov rdx, qword ptr [rbp + 0x118]
14123915b test rdx, rdx
14123915e je 0x141239176
141239160 mov rcx, qword ptr [rbp + 0x120]
141239167 shl rdx, 2
14123916b mov r8d, 4
141239171 call 0x140613c20
141239176 mov rax, qword ptr [rbp + 0x148]
14123917d inc rax
141239180 mov qword ptr [rbp + 0x90], rax
141239187 mov rax, qword ptr [rbp + 0xe0]
14123918e mov qword ptr [rbp + 0x130], rax
141239195 movups xmm0, xmmword ptr [rbp + 0xb0]
14123919c movups xmm1, xmmword ptr [rbp + 0xc0]
1412391a3 movups xmm2, xmmword ptr [rbp + 0xd0]
1412391aa movaps xmmword ptr [rbp + 0x120], xmm2
1412391b1 movaps xmmword ptr [rbp + 0x110], xmm1
1412391b8 movaps xmmword ptr [rbp + 0x100], xmm0
1412391bf mov rbx, qword ptr [rbp + 0x108]
1412391c6 mov rax, qword ptr [rbp + 0x110]
1412391cd lea rax, [rax + rax*2]
1412391d1 lea r13, [rbx + rax*4]
1412391d5 mov al, 1
1412391d7 xor esi, esi
1412391d9 jmp 0x1412392ef
1412391de nop 
1412391e0 inc rsi
1412391e3 mulss xmm7, xmm9
1412391e8 mulss xmm7, dword ptr [rip + 0x16ec6e8]
1412391f0 movaps xmm0, xmm7
1412391f3 mulss xmm0, xmm7
1412391f7 movss xmm2, dword ptr [rip + 0x16ec705]
1412391ff addss xmm0, xmm2
141239203 movaps xmm1, xmm3
141239206 mulss xmm1, xmm3
14123920a addss xmm1, xmm0
14123920e xorps xmm0, xmm0
141239211 sqrtss xmm0, xmm1
141239215 movaps xmm1, xmm2
141239218 divss xmm1, xmm0
14123921c movaps xmm0, xmm1
14123921f mulss xmm0, xmm12
141239224 mov rax, qword ptr [rbp + 0x158]
14123922b lea rcx, [rdi + rdi*8]
14123922f movlps qword ptr [rax + rcx*8], xmm6
141239233 movss dword ptr [rax + rcx*8 + 8], xmm11
14123923a mov rdx, qword ptr [rbp + 0xb0]
141239241 mov qword ptr [rax + rcx*8 + 0x18], rdx
141239246 mov edx, dword ptr [rbp + 0xb8]
14123924c mov dword ptr [rax + rcx*8 + 0x20], edx
141239250 movss dword ptr [rax + rcx*8 + 0x28], xmm1
141239256 unpcklps xmm3, xmm7
141239259 movsldup xmm2, xmm1
14123925d mulps xmm2, xmm3
141239260 movaps xmm7, xmmword ptr [rbp]
141239264 movaps xmm3, xmm7
141239267 mulss xmm3, xmm2
14123926b addss xmm3, xmm11
141239270 movshdup xmm4, xmm2
141239274 movaps xmm5, xmm1
141239277 shufps xmm1, xmm2, 0xd4
14123927b shufps xmm1, xmm2, 0xf2
14123927f mulps xmm1, xmm13
141239283 addps xmm1, xmm6
141239286 movlps qword ptr [rax + rcx*8 + 0xc], xmm1
14123928b movss dword ptr [rax + rcx*8 + 0x14], xmm3
141239291 movss dword ptr [rax + rcx*8 + 0x24], xmm4
141239297 mulss xmm4, xmm14
14123929c subss xmm0, xmm4
1412392a0 unpcklps xmm5, xmm2
1412392a3 movss dword ptr [rax + rcx*8 + 0x2c], xmm2
1412392a9 mulps xmm2, xmm14
1412392ad shufps xmm12, xmm14, 0xd4
1412392b2 shufps xmm12, xmm14, 0xf2
1412392b7 mulps xmm12, xmm5
1412392bb subps xmm2, xmm12
1412392bf movlps qword ptr [rax + rcx*8 + 0x30], xmm2
1412392c4 movss dword ptr [rax + rcx*8 + 0x38], xmm0
1412392ca mov dword ptr [rax + rcx*8 + 0x3c], 0x3e99999a
1412392d2 movss dword ptr [rax + rcx*8 + 0x40], xmm7
1412392d8 mov dword ptr [rax + rcx*8 + 0x44], 0x3e99999a
1412392e0 inc rdi
1412392e3 mov qword ptr [rbp + 0x160], rdi
1412392ea xor eax, eax
1412392ec mov rdi, r14
1412392ef cmp rbx, r13
1412392f2 je 0x141239603
1412392f8 lea rcx, [rbx + 0xc]
1412392fc test al, 1
1412392fe je 0x141239320
141239300 cmp rcx, r13
141239303 je 0x141239600
141239309 mov rdi, rbx
14123930c add rbx, 0x18
141239310 mov r14, rcx
141239313 jmp 0x141239326
141239315 nop word ptr cs:[rax + rax]
141239320 mov r14, rbx
141239323 mov rbx, rcx
141239326 movsd xmm14, qword ptr [rdi]
14123932b movss xmm12, dword ptr [rdi + 8]
141239331 movsd xmm6, qword ptr [r14]
141239336 movss xmm11, dword ptr [r14 + 8]
14123933c mov rax, rsi
14123933f or rax, qword ptr [rbp + 0x148]
141239346 jne 0x14123950d
14123934c mov rcx, r12
14123934f call 0x141625cb0
141239354 movaps xmm7, xmm0
141239357 mov rcx, r12
14123935a call 0x141625ae0
14123935f movaps xmm8, xmm0
141239363 mulss xmm8, dword ptr [rip + 0x16ec59c]
14123936c addss xmm8, xmm15
141239371 movaps xmm0, xmm6
141239374 subss xmm0, xmm14
141239379 movaps xmm1, xmm11
14123937d subss xmm1, xmm12
141239382 movaps xmm2, xmm0
141239385 mulss xmm2, xmm0
141239389 movaps xmm3, xmm1
14123938c mulss xmm3, xmm1
141239390 addss xmm3, xmm2
141239394 xorps xmm2, xmm2
141239397 sqrtss xmm2, xmm3
14123939b movss xmm3, dword ptr [rip + 0x16ec561]
1412393a3 divss xmm3, xmm2
1412393a7 mulss xmm0, xmm3
1412393ab mulss xmm3, xmm1
1412393af movss dword ptr [rbp + 0x168], xmm0
1412393b7 movss dword ptr [rbp + 0x16c], xmm3
1412393bf lea rcx, [rbp + 0xb0]
1412393c6 lea rdx, [rbp + 0x168]
1412393cd call 0x140c9cdc0
1412393d2 movss xmm9, dword ptr [rbp + 0xb0]
1412393db movsd xmm10, qword ptr [rbp + 0xb4]
1412393e4 mov rdi, qword ptr [rbp + 0x160]
1412393eb cmp rdi, qword ptr [rbp + 0x150]
1412393f2 jne 0x141239407
1412393f4 lea rcx, [rbp + 0x150]
1412393fb lea rdx, [rip + 0x19e5a06]
141239402 call 0x14078cc50
141239407 mulss xmm8, xmm8
14123940c mulss xmm7, xmm8
141239411 mulss xmm7, dword ptr [rip + 0x16ec4bf]
141239419 movaps xmm0, xmm7
14123941c mulss xmm0, xmm7
141239420 movss xmm1, dword ptr [rip + 0x16ec4dc]
141239428 addss xmm0, xmm1
14123942c sqrtss xmm0, xmm0
141239430 divss xmm1, xmm0
141239434 movaps xmm0, xmm1
141239437 mulss xmm0, xmm9
14123943c mov rax, qword ptr [rbp + 0x158]
141239443 lea rcx, [rdi + rdi*8]
141239447 movlps qword ptr [rax + rcx*8], xmm14
14123944c movss dword ptr [rax + rcx*8 + 8], xmm12
141239453 mov rdx, qword ptr [rbp + 0xb0]
14123945a mov qword ptr [rax + rcx*8 + 0x18], rdx
14123945f mov edx, dword ptr [rbp + 0xb8]
141239465 mov dword ptr [rax + rcx*8 + 0x20], edx
141239469 movss dword ptr [rax + rcx*8 + 0x28], xmm1
14123946f movsldup xmm3, xmm1
141239473 movq xmm2, xmm7
141239477 shufps xmm2, xmmword ptr [rip + 0x16ec521], 0xe2
14123947f mulps xmm2, xmm3
141239482 movaps xmm7, xmmword ptr [rbp]
141239486 movaps xmm3, xmm7
141239489 mulss xmm3, xmm2
14123948d addss xmm3, xmm12
141239492 movshdup xmm4, xmm2
141239496 movaps xmm5, xmm1
141239499 shufps xmm1, xmm2, 0xd4
14123949d shufps xmm1, xmm2, 0xf2
1412394a1 mulps xmm1, xmm13
1412394a5 addps xmm1, xmm14
1412394a9 movlps qword ptr [rax + rcx*8 + 0xc], xmm1
1412394ae movss dword ptr [rax + rcx*8 + 0x14], xmm3
1412394b4 movss dword ptr [rax + rcx*8 + 0x24], xmm4
1412394ba mulss xmm4, xmm10
1412394bf subss xmm0, xmm4
1412394c3 unpcklps xmm5, xmm2
1412394c6 movss dword ptr [rax + rcx*8 + 0x2c], xmm2
1412394cc mulps xmm2, xmm10
1412394d0 shufps xmm9, xmm10, 0xd4
1412394d5 shufps xmm9, xmm10, 0xf2
1412394da mulps xmm9, xmm5
1412394de subps xmm2, xmm9
1412394e2 movlps qword ptr [rax + rcx*8 + 0x30], xmm2
1412394e7 movss dword ptr [rax + rcx*8 + 0x38], xmm0
1412394ed mov dword ptr [rax + rcx*8 + 0x3c], 0x3e99999a
1412394f5 movss dword ptr [rax + rcx*8 + 0x40], xmm7
1412394fb mov dword ptr [rax + rcx*8 + 0x44], 0x3e99999a
141239503 inc rdi
141239506 mov qword ptr [rbp + 0x160], rdi
14123950d mov r15, qword ptr [rbp + 0x110]
141239514 mov rcx, r12
141239517 call 0x141625cb0
14123951c movaps xmm7, xmm0
14123951f movaps xmm8, xmm1
141239523 mov rcx, r12
141239526 call 0x141625ae0
14123952b movaps xmm9, xmm0
14123952f mulss xmm9, dword ptr [rip + 0x16ec3d0]
141239538 addss xmm9, xmm15
14123953d movaps xmm0, xmm6
141239540 subss xmm0, xmm14
141239545 movaps xmm1, xmm11
141239549 subss xmm1, xmm12
14123954e movaps xmm2, xmm0
141239551 mulss xmm2, xmm0
141239555 movaps xmm3, xmm1
141239558 mulss xmm3, xmm1
14123955c addss xmm3, xmm2
141239560 xorps xmm2, xmm2
141239563 sqrtss xmm2, xmm3
141239567 movss xmm3, dword ptr [rip + 0x16ec395]
14123956f divss xmm3, xmm2
141239573 mulss xmm0, xmm3
141239577 mulss xmm3, xmm1
14123957b movss dword ptr [rbp + 0x168], xmm0
141239583 movss dword ptr [rbp + 0x16c], xmm3
14123958b lea rcx, [rbp + 0xb0]
141239592 lea rdx, [rbp + 0x168]
141239599 call 0x140c9cdc0
14123959e movss xmm12, dword ptr [rbp + 0xb0]
1412395a7 movsd xmm14, qword ptr [rbp + 0xb4]
1412395b0 mov rdi, qword ptr [rbp + 0x160]
1412395b7 cmp rdi, qword ptr [rbp + 0x150]
1412395be jne 0x1412395d3
1412395c0 lea rcx, [rbp + 0x150]
1412395c7 lea rdx, [rip + 0x19e583a]
1412395ce call 0x14078cc50
1412395d3 lea rax, [rsi + 2]
1412395d7 mulss xmm9, xmm9
1412395dc xorps xmm3, xmm3
1412395df cmp rax, r15
1412395e2 je 0x1412391e0
1412395e8 mulss xmm8, xmm9
1412395ed mulss xmm8, dword ptr [rip + 0x16ec2e2]
1412395f6 movaps xmm3, xmm8
1412395fa jmp 0x1412391e0
1412395ff nop 
141239600 mov rdi, r13
141239603 mov rax, qword ptr [rbp + 0x100]
14123960a test rax, rax
14123960d mov rsi, qword ptr [rbp + 0x80]
141239614 je 0x141239630
141239616 mov rcx, qword ptr [rbp + 0x108]
14123961d shl rax, 2
141239621 lea rdx, [rax + rax*2]
141239625 mov r8d, 4
14123962b call 0x140613c20
141239630 mov rdx, qword ptr [rbp + 0x118]
141239637 test rdx, rdx
14123963a je 0x141238f30
141239640 mov rcx, qword ptr [rbp + 0x120]
141239647 shl rdx, 2
14123964b mov r8d, 4
141239651 call 0x140613c20
141239656 jmp 0x141238f30
14123965b mov rdi, qword ptr [rbp + 0x158]
141239662 mov rax, qword ptr [rbp + 0x160]
141239669 lea rax, [rax + rax*8]
14123966d lea r13, [rdi + rax*8]
141239671 dec qword ptr [rbp + 0xf0]
141239678 mov rax, qword ptr [rbp + 0x2b8]
14123967f mov rbx, qword ptr [rax]
141239682 mov rax, qword ptr [rax + 8]
141239686 mov qword ptr [rbp + 0x48], rax
14123968a xor r15d, r15d
14123968d xorps xmm6, xmm6
141239690 movss xmm7, dword ptr [rip + 0x17b2060]
141239698 lea rsi, [rbp + 0xb0]
14123969f lea r12, [rbp + 0x138]
1412396a6 nop word ptr cs:[rax + rax]
1412396b0 cmp rdi, r13
1412396b3 je 0x14123982b
1412396b9 mov r14, r15
1412396bc mov rax, rdi
1412396bf cmp qword ptr [rbp + 0x2a0], 0
1412396c7 setne cl
1412396ca add rdi, 0x48
1412396ce inc r15
1412396d1 test r14, r14
1412396d4 sete dl
1412396d7 test cl, dl
1412396d9 jne 0x1412396b0
1412396db mov rcx, qword ptr [rbp + 0x2a8]
1412396e2 cmp qword ptr [rbp + 0xf0], rcx
1412396e9 jbe 0x1412396fa
1412396eb mov rcx, qword ptr [rbp + 0x160]
1412396f2 dec rcx
1412396f5 cmp r14, rcx
1412396f8 je 0x1412396b0
1412396fa movss xmm1, dword ptr [rax + 0x3c]
1412396ff addss xmm1, xmm6
141239703 movss xmm2, dword ptr [rax + 0x40]
141239708 addss xmm2, dword ptr [rip + 0x16ec274]
141239710 movss xmm0, dword ptr [rax + 0x44]
141239715 addss xmm0, xmm6
141239719 movsldup xmm3, xmm1
14123971d mulss xmm1, dword ptr [rax + 0x20]
141239722 movss xmm4, dword ptr [rax + 0x2c]
141239727 mulss xmm4, xmm2
14123972b movss xmm5, dword ptr [rax + 0x38]
141239730 mulss xmm5, xmm0
141239734 movss xmm8, dword ptr [rax]
141239739 movss xmm9, dword ptr [rax + 8]
14123973f movss xmm10, dword ptr [rax + 4]
141239745 addss xmm10, xmm7
14123974a movss dword ptr [rbp + 0xa0], xmm8
141239753 movss dword ptr [rbp + 0xa4], xmm10
14123975c movss dword ptr [rbp + 0xa8], xmm9
141239765 mov rcx, qword ptr [rbp + 0x40]
141239769 mov qword ptr [rbp + 0x138], rcx
141239770 movsd xmm8, qword ptr [rax + 0x18]
141239776 mulps xmm8, xmm3
14123977a movlps qword ptr [rbp + 0x70], xmm8
14123977f movss dword ptr [rbp + 0x78], xmm1
141239784 movsd xmm1, qword ptr [rax + 0x24]
141239789 movsldup xmm2, xmm2
14123978d mulps xmm2, xmm1
141239790 movlps qword ptr [rbp + 0x168], xmm2
141239797 movss dword ptr [rbp + 0x170], xmm4
14123979f movsd xmm1, qword ptr [rax + 0x30]
1412397a4 movsldup xmm0, xmm0
1412397a8 mulps xmm0, xmm1
1412397ab movlps qword ptr [rbp + 0x100], xmm0
1412397b2 movss dword ptr [rbp + 0x108], xmm5
1412397ba lea rax, [rbp + 0xa0]
1412397c1 mov qword ptr [rsp + 0x20], rax
1412397c6 mov rcx, rsi
1412397c9 lea rdx, [rbp + 0x70]
1412397cd lea r8, [rbp + 0x168]
1412397d4 lea r9, [rbp + 0x100]
1412397db call 0x14089d500
1412397e0 mov rcx, r12
1412397e3 call 0x1400885d0
1412397e8 mov rcx, qword ptr [rbp + 0x298]
1412397ef mov dword ptr [rbp + 0xe0], ecx
1412397f5 mov dword ptr [rbp + 0xe4], r14d
1412397fc mov dword ptr [rbp + 0xe8], eax
141239802 mov dword ptr [rbp + 0xec], 0
14123980c mov rcx, rbx
14123980f mov rdx, rsi
141239812 call 0x14120c430
141239817 test al, al
141239819 je 0x1412396b0
14123981f mov rax, qword ptr [rbp + 0x48]
141239823 mov byte ptr [rax], 1
141239826 jmp 0x1412396b0
14123982b mov rax, qword ptr [rbp + 0x158]
141239832 mov rcx, qword ptr [rbp + 0x160]
141239839 lea rcx, [rcx + rcx*8]
14123983d lea rcx, [rax + rcx*8]
141239841 mov qword ptr [rbp + 0x58], rcx
141239845 mov cl, 1
141239847 lea r14, [rbp - 0x20]
14123984b lea r15, [rbp + 0xb0]
141239852 lea r12, [rbp + 0x100]
141239859 mov qword ptr [rbp + 0x98], 0
141239864 jmp 0x14123987a
141239866 nop word ptr cs:[rax + rax]
141239870 xor ecx, ecx
141239872 mov rax, qword ptr [rbp + 0x50]
141239876 mov rdx, qword ptr [rbp - 0x30]
14123987a cmp rax, qword ptr [rbp + 0x58]
14123987e je 0x141239e62
141239884 lea r8, [rax + 0x48]
141239888 test cl, 1
14123988b je 0x1412398b0
14123988d cmp r8, qword ptr [rbp + 0x58]
141239891 je 0x141239e62
141239897 mov rcx, r8
14123989a lea rdx, [rax + 0x90]
1412398a1 mov qword ptr [rbp + 0x50], rdx
1412398a5 mov rdx, rax
1412398a8 jmp 0x1412398b7
1412398aa nop word ptr [rax + rax]
1412398b0 mov qword ptr [rbp + 0x50], r8
1412398b4 mov rcx, rax
1412398b7 movss xmm0, dword ptr [rdx + 8]
1412398bc movss dword ptr [rbp + 0x80], xmm0
1412398c4 movss xmm0, dword ptr [rdx + 0x14]
1412398c9 movss dword ptr [rbp + 0x88], xmm0
1412398d1 movss xmm0, dword ptr [rcx + 8]
1412398d6 movss dword ptr [rbp + 0x90], xmm0
1412398de movsd xmm0, qword ptr [rdx]
1412398e2 movaps xmmword ptr [rbp + 0x10], xmm0
1412398e6 movsd xmm0, qword ptr [rdx + 0xc]
1412398eb movaps xmmword ptr [rbp + 0x20], xmm0
1412398ef movsd xmm0, qword ptr [rcx]
1412398f3 movaps xmmword ptr [rbp - 0x10], xmm0
1412398f7 movsd xmm0, qword ptr [rcx + 0xc]
1412398fc movaps xmmword ptr [rbp + 0x30], xmm0
141239900 mov qword ptr [rbp - 0x30], rcx
141239904 movss xmm0, dword ptr [rcx + 0x14]
141239909 movss dword ptr [rbp + 0xf0], xmm0
141239911 mov rcx, r14
141239914 call 0x141625ae0
141239919 movss xmm6, dword ptr [rip + 0x16ebfe3]
141239921 subss xmm6, xmm0
141239925 mulss xmm6, dword ptr [rip + 0x16ec057]
14123992d mulss xmm0, dword ptr [rip + 0x16ebfa3]
141239935 movaps xmm7, xmm0
141239938 mov rcx, r14
14123993b call 0x141625ae0
141239940 mov rsi, qword ptr [rbp + 0x98]
141239947 inc rsi
14123994a addss xmm7, xmm6
14123994e movss xmm6, dword ptr [rip + 0x16ebfae]
141239956 movaps xmm1, xmm6
141239959 subss xmm1, xmm0
14123995d mulss xmm1, dword ptr [rip + 0x16ebfa3]
141239965 mulss xmm0, dword ptr [rip + 0x17dd34b]
14123996d addss xmm0, xmm1
141239971 movss dword ptr [rbp + 0x64], xmm0
141239976 movss dword ptr [rbp + 0xf8], xmm7
14123997e subss xmm0, xmm7
141239982 mulss xmm0, dword ptr [rbp]
141239987 movss dword ptr [rbp + 0x6c], xmm0
14123998c mulss xmm0, dword ptr [rip + 0x16fad00]
141239994 call 0x142923ee0
141239999 cvttss2si rax, xmm0
14123999e mov rcx, rax
1412399a1 sar rcx, 0x3f
1412399a5 movaps xmm1, xmm0
1412399a8 subss xmm1, dword ptr [rip + 0x172072c]
1412399b0 cvttss2si rdx, xmm1
1412399b5 and rdx, rcx
1412399b8 or rdx, rax
1412399bb ucomiss xmm0, dword ptr [rip + 0x1726cce]
1412399c2 mov eax, 0
1412399c7 cmovb rdx, rax
1412399cb ucomiss xmm0, dword ptr [rip + 0x172070e]
1412399d2 mov rax, 0xffffffffffffffff
1412399d9 cmova rdx, rax
1412399dd cmp rdx, 4
1412399e1 mov r13d, 4
1412399e7 cmovb r13, rdx
1412399eb test rdx, rdx
1412399ee mov eax, 1
1412399f3 cmove r13, rax
1412399f7 lea rax, [r13 - 1]
1412399fb xorps xmm0, xmm0
1412399fe cvtsi2ss xmm0, rax
141239a03 movss dword ptr [rbp + 0x68], xmm0
141239a08 mov qword ptr [rbp + 0x98], rsi
141239a0f lea rdi, [rsi*4]
141239a17 xor esi, esi
141239a19 jmp 0x141239a2b
141239a1b nop dword ptr [rax + rax]
141239a20 inc rsi
141239a23 movss xmm6, dword ptr [rip + 0x16ebed9]
141239a2b cmp rsi, r13
141239a2e jae 0x141239870
141239a34 movss xmm0, dword ptr [rip + 0x16ebe34]
141239a3c cmp r13, 1
141239a40 je 0x141239a4f
141239a42 xorps xmm0, xmm0
141239a45 cvtsi2ss xmm0, rsi
141239a4a divss xmm0, dword ptr [rbp + 0x68]
141239a4f movaps xmm1, xmm6
141239a52 subss xmm1, xmm0
141239a56 mulss xmm1, dword ptr [rbp + 0xf8]
141239a5e mulss xmm0, dword ptr [rbp + 0x64]
141239a63 addss xmm0, xmm1
141239a67 movaps xmm1, xmm6
141239a6a subss xmm1, xmm0
141239a6e movss xmm2, dword ptr [rbp + 0x80]
141239a76 mulss xmm2, xmm1
141239a7a movss xmm14, dword ptr [rbp + 0x88]
141239a83 mulss xmm14, xmm0
141239a88 addss xmm14, xmm2
141239a8d movss xmm2, dword ptr [rbp + 0x90]
141239a95 mulss xmm2, xmm1
141239a99 movss xmm9, dword ptr [rbp + 0xf0]
141239aa2 mulss xmm9, xmm0
141239aa7 movsldup xmm1, xmm1
141239aab movaps xmm3, xmmword ptr [rbp + 0x10]
141239aaf mulps xmm3, xmm1
141239ab2 movsldup xmm12, xmm0
141239ab7 movaps xmm6, xmmword ptr [rbp + 0x20]
141239abb mulps xmm6, xmm12
141239abf addps xmm6, xmm3
141239ac2 mulps xmm1, xmmword ptr [rbp - 0x10]
141239ac6 mulps xmm12, xmmword ptr [rbp + 0x30]
141239acb addps xmm12, xmm1
141239acf addss xmm9, xmm2
141239ad4 movaps xmm15, xmm9
141239ad8 shufps xmm15, xmm12, 0xd4
141239add shufps xmm15, xmm12, 0xf2
141239ae2 movaps xmm0, xmm14
141239ae6 shufps xmm0, xmm6, 0xd4
141239aea shufps xmm0, xmm6, 0xf2
141239aee subps xmm15, xmm0
141239af2 movaps xmm11, xmm12
141239af6 subss xmm11, xmm6
141239afb movss dword ptr [rbp + 0x138], xmm11
141239b04 movlps qword ptr [rbp + 0x13c], xmm15
141239b0c mov rcx, r14
141239b0f call 0x141625ae0
141239b14 movss xmm1, dword ptr [rip + 0x16ebe04]
141239b1c ucomiss xmm1, xmm0
141239b1f jbe 0x141239c50
141239b25 mov rcx, r14
141239b28 call 0x141625ae0
141239b2d movss xmm1, dword ptr [rip + 0x16ebd3b]
141239b35 ucomiss xmm1, xmm0
141239b38 movaps xmm10, xmm11
141239b3c movss dword ptr [rbp + 0x148], xmm14
141239b45 movaps xmm13, xmm6
141239b49 movaps xmm8, xmm15
141239b4d jbe 0x141239b84
141239b4f movaps xmm10, xmm11
141239b53 movaps xmm0, xmmword ptr [rip + 0x16ebd96]
141239b5a xorps xmm10, xmm0
141239b5e movaps xmm8, xmm15
141239b62 xorps xmm8, xmm0
141239b66 movss dword ptr [rbp + 0x138], xmm10
141239b6f movlps qword ptr [rbp + 0x13c], xmm8
141239b77 movss dword ptr [rbp + 0x148], xmm9
141239b80 movaps xmm13, xmm12
141239b84 mov rcx, r14
141239b87 call 0x141625ae0
141239b8c movss xmm1, dword ptr [rip + 0x16ebd70]
141239b94 subss xmm1, xmm0
141239b98 mulss xmm1, dword ptr [rip + 0x16ebde4]
141239ba0 mulss xmm0, dword ptr [rip + 0x16ebcc8]
141239ba8 addss xmm0, xmm1
141239bac mulss xmm0, dword ptr [rbp + 0x6c]
141239bb1 movaps xmm7, xmm8
141239bb5 subss xmm7, xmm0
141239bb9 movss dword ptr [rbp + 0x13c], xmm7
141239bc1 movshdup xmm0, xmm15
141239bc6 xorps xmm0, xmmword ptr [rip + 0x16ebd23]
141239bcd movss dword ptr [rbp + 0x100], xmm0
141239bd5 movss dword ptr [rbp + 0x104], xmm11
141239bde mov rcx, r15
141239be1 mov rdx, r12
141239be4 call 0x140c9cdc0
141239be9 mov edx, 2
141239bee mov rcx, r14
141239bf1 call 0x141625a50
141239bf6 cmp rax, 2
141239bfa jae 0x141239fa8
141239c00 lea rcx, [rip + 0x19e5161]
141239c07 movss xmm0, dword ptr [rcx + rax*4]
141239c0c mulss xmm0, dword ptr [rip + 0x16f715c]
141239c14 movsldup xmm1, xmm0
141239c18 mulss xmm0, dword ptr [rbp + 0xb0]
141239c20 addss xmm10, xmm0
141239c25 movsd xmm0, qword ptr [rbp + 0xb4]
141239c2d mulps xmm0, xmm1
141239c30 movss xmm8, xmm7
141239c35 addps xmm8, xmm0
141239c39 movaps xmm11, xmm10
141239c3d movaps xmm15, xmm8
141239c41 movss xmm8, dword ptr [rbp + 0x148]
141239c4a jmp 0x141239c58
141239c4c nop dword ptr [rax]
141239c50 movaps xmm8, xmm14
141239c54 movaps xmm13, xmm6
141239c58 movaps xmm0, xmm11
141239c5c mulss xmm0, xmm11
141239c61 movaps xmm1, xmm15
141239c65 mulps xmm1, xmm15
141239c69 addss xmm0, xmm1
141239c6d movshdup xmm1, xmm1
141239c71 addss xmm1, xmm0
141239c75 xorps xmm0, xmm0
141239c78 sqrtss xmm0, xmm1
141239c7c movss xmm7, dword ptr [rip + 0x16ebc80]
141239c84 movaps xmm1, xmm7
141239c87 divss xmm1, xmm0
141239c8b mulss xmm11, xmm1
141239c90 movsldup xmm0, xmm1
141239c94 mulps xmm0, xmm15
141239c98 movss dword ptr [rbp + 0x138], xmm11
141239ca1 movlps qword ptr [rbp + 0x13c], xmm0
141239ca8 ucomiss xmm11, dword ptr [rip + 0x17269e0]
141239cb0 jne 0x141239cce
141239cb2 jp 0x141239cce
141239cb4 ucomiss xmm0, xmm7
141239cb7 jne 0x141239cce
141239cb9 jp 0x141239cce
141239cbb movshdup xmm1, xmm0
141239cbf ucomiss xmm1, dword ptr [rip + 0x17269ca]
141239cc6 jne 0x141239cce
141239cc8 jnp 0x141239f50
141239cce subss xmm14, xmm9
141239cd3 subps xmm6, xmm12
141239cd7 mulps xmm6, xmm6
141239cda movshdup xmm1, xmm6
141239cde addss xmm1, xmm6
141239ce2 mulss xmm14, xmm14
141239ce7 addss xmm14, xmm1
141239cec xorps xmm1, xmm1
141239cef sqrtss xmm1, xmm14
141239cf4 movaps xmm3, xmm0
141239cf7 mulps xmm3, xmmword ptr [rip + 0x16ebca2]
141239cfe movaps xmm2, xmm11
141239d02 mulss xmm2, dword ptr [rip + 0x1726986]
141239d0a movaps xmm5, xmm11
141239d0e subss xmm5, xmm3
141239d12 movaps xmm4, xmm5
141239d15 mulss xmm4, xmm5
141239d19 movlps qword ptr [rbp + 0xa0], xmm13
141239d21 movss dword ptr [rbp + 0xa8], xmm8
141239d2a mov rax, qword ptr [rbp + 0x40]
141239d2e mov qword ptr [rbp - 0x50], rax
141239d32 shufps xmm2, xmm0, 0xd4
141239d36 shufps xmm2, xmm0, 0xf2
141239d3a subps xmm3, xmm2
141239d3d movaps xmm2, xmm3
141239d40 mulps xmm2, xmm3
141239d43 movshdup xmm6, xmm2
141239d47 addss xmm6, xmm2
141239d4b addss xmm6, xmm4
141239d4f xorps xmm4, xmm4
141239d52 sqrtss xmm4, xmm6
141239d56 movaps xmm2, xmm7
141239d59 divss xmm2, xmm4
141239d5d movsldup xmm4, xmm2
141239d61 mulps xmm4, xmm3
141239d64 mulss xmm2, xmm5
141239d68 movshdup xmm5, xmm4
141239d6c movaps xmm3, xmm0
141239d6f mulss xmm3, xmm4
141239d73 mulss xmm5, xmm11
141239d78 subss xmm3, xmm5
141239d7c movaps xmm5, xmm2
141239d7f movaps xmm6, xmm2
141239d82 shufps xmm6, xmm4, 0xd4
141239d86 shufps xmm6, xmm4, 0xe2
141239d8a unpcklps xmm2, xmm4
141239d8d mulps xmm4, xmmword ptr [rip + 0x16f6fcc]
141239d94 mulss xmm5, dword ptr [rip + 0x16ebbe8]
141239d9c movlps qword ptr [rbp + 0x70], xmm4
141239da0 movss dword ptr [rbp + 0x78], xmm5
141239da5 movsldup xmm4, xmm1
141239da9 mulss xmm1, xmm11
141239dae movss dword ptr [rbp + 0x168], xmm1
141239db6 mulps xmm4, xmm0
141239db9 movlps qword ptr [rbp + 0x16c], xmm4
141239dc0 mulss xmm3, dword ptr [rip + 0x17932a0]
141239dc8 shufps xmm11, xmm0, 0xd4
141239dcd shufps xmm11, xmm0, 0xf2
141239dd2 mulps xmm6, xmm11
141239dd6 mulps xmm2, xmm0
141239dd9 subps xmm6, xmm2
141239ddc mulps xmm6, xmmword ptr [rip + 0x185290d]
141239de3 movlps qword ptr [rbp + 0x100], xmm6
141239dea movss dword ptr [rbp + 0x108], xmm3
141239df2 lea rax, [rbp + 0xa0]
141239df9 mov qword ptr [rsp + 0x20], rax
141239dfe mov rcx, r15
141239e01 lea rdx, [rbp + 0x70]
141239e05 lea r8, [rbp + 0x168]
141239e0c mov r9, r12
141239e0f call 0x14089d500
141239e14 lea rcx, [rbp - 0x50]
141239e18 call 0x1400885d0
141239e1d lea ecx, [rsi + rdi]
141239e20 mov rdx, qword ptr [rbp + 0x298]
141239e27 mov dword ptr [rbp + 0xe0], edx
141239e2d mov dword ptr [rbp + 0xe4], ecx
141239e33 mov dword ptr [rbp + 0xe8], eax
141239e39 mov dword ptr [rbp + 0xec], 0
141239e43 mov rcx, rbx
141239e46 mov rdx, r15
141239e49 call 0x14120c430
141239e4e test al, al
141239e50 je 0x141239a20
141239e56 mov rax, qword ptr [rbp + 0x48]
141239e5a mov byte ptr [rax], 1
141239e5d jmp 0x141239a20
141239e62 mov rax, qword ptr [rbp + 0x150]
141239e69 test rax, rax
141239e6c je 0x141239e88
141239e6e mov rcx, qword ptr [rbp + 0x158]
141239e75 shl rax, 3
141239e79 lea rdx, [rax + rax*8]
141239e7d mov r8d, 4
141239e83 call 0x140613c20
141239e88 mov rdx, qword ptr [rbp - 0x48]
141239e8c test rdx, rdx
141239e8f mov rax, qword ptr [rbp + 0x98]
141239e96 je 0x141239eb1
141239e98 shl rdx, 3
141239e9c mov r8d, 8
141239ea2 mov rcx, qword ptr [rbp - 0x28]
141239ea6 mov rsi, rax
141239ea9 call 0x140613c20
141239eae mov rax, rsi
141239eb1 movaps xmm6, xmmword ptr [rbp + 0x180]
141239eb8 movaps xmm7, xmmword ptr [rbp + 0x190]
141239ebf movaps xmm8, xmmword ptr [rbp + 0x1a0]
141239ec7 movaps xmm9, xmmword ptr [rbp + 0x1b0]
141239ecf movaps xmm10, xmmword ptr [rbp + 0x1c0]
141239ed7 movaps xmm11, xmmword ptr [rbp + 0x1d0]
141239edf movaps xmm12, xmmword ptr [rbp + 0x1e0]
141239ee7 movaps xmm13, xmmword ptr [rbp + 0x1f0]
141239eef movaps xmm14, xmmword ptr [rbp + 0x200]
141239ef7 movaps xmm15, xmmword ptr [rbp + 0x210]
141239eff add rsp, 0x2a8
141239f06 pop rbx
141239f07 pop rdi
141239f08 pop rsi
141239f09 pop r12
141239f0b pop r13
141239f0d pop r14
141239f0f pop r15
141239f11 pop rbp
141239f12 ret 
141239f13 lea rcx, [rip + 0x19e4ebe]
141239f1a call 0x1428d98a0
141239f1f jmp 0x14123a020
141239f24 lea r8, [rip + 0x19e4ead]
141239f2b mov rcx, rbx
141239f2e call 0x1428d9850
141239f33 jmp 0x14123a020
141239f38 lea r8, [rip + 0x19e4e99]
141239f3f mov rcx, rdx
141239f42 mov rdx, qword ptr [rbp + 0x20]
141239f46 call 0x1428d9840
141239f4b jmp 0x14123a020
141239f50 lea rax, [rip + 0x19e4e59]
141239f57 mov qword ptr [rbp + 0xb0], rax
141239f5e mov qword ptr [rbp + 0xb8], 1
141239f69 mov qword ptr [rbp + 0xc0], 8
141239f74 xorps xmm0, xmm0
141239f77 movups xmmword ptr [rbp + 0xc8], xmm0
141239f7e lea rax, [rip + 0x19e4e3b]
141239f85 mov qword ptr [rsp + 0x20], rax
141239f8a lea r8, [rip + 0x19e4ddf]
141239f91 lea rdx, [rbp + 0x138]
141239f98 lea r9, [rbp + 0xb0]
141239f9f mov cl, 1
141239fa1 call 0x142914b5a
141239fa6 jmp 0x14123a020
141239fa8 lea r8, [rip + 0x1d91259]
141239faf mov edx, 2
141239fb4 mov rcx, rax
141239fb7 call 0x1428d9518
141239fbc jmp 0x14123a020
141239fbe movups xmm0, xmmword ptr [rbp + 0xb8]
141239fc5 movaps xmmword ptr [rbp + 0x100], xmm0
141239fcc lea rax, [rip + 0x19e4e1d]
141239fd3 mov qword ptr [rsp + 0x20], rax
141239fd8 lea rcx, [rip + 0x19debe9]
141239fdf lea r9, [rip + 0x19debc2]
141239fe6 lea r8, [rbp + 0x100]
141239fed mov edx, 0x2b
141239ff2 call 0x1428d9760
141239ff7 jmp 0x14123a020
141239ff9 mov eax, 4
141239ffe mov qword ptr [rbp + 0xf8], rax
14123a005 mov qword ptr [rbp + 0x30], r14
14123a009 lea r8, [rip + 0x19e4bc8]
14123a010 mov rcx, qword ptr [rbp + 0xf8]
14123a017 mov rdx, qword ptr [rbp + 0x30]
14123a01b call 0x1428d8fe3
14123a020 ud2 
