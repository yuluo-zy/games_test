1407661e0 push r15
1407661e2 push r14
1407661e4 push r13
1407661e6 push r12
1407661e8 push rsi
1407661e9 push rdi
1407661ea push rbp
1407661eb push rbx
1407661ec sub rsp, 0x1b8
1407661f3 movaps xmmword ptr [rsp + 0x1a0], xmm12
1407661fc movaps xmmword ptr [rsp + 0x190], xmm11
140766205 movaps xmmword ptr [rsp + 0x180], xmm10
14076620e movaps xmmword ptr [rsp + 0x170], xmm9
140766217 movaps xmmword ptr [rsp + 0x160], xmm8
140766220 movaps xmmword ptr [rsp + 0x150], xmm7
140766228 movdqa xmmword ptr [rsp + 0x140], xmm6
140766231 mov qword ptr [rsp + 0x40], rdx
140766236 mov qword ptr [rsp + 0x38], rcx
14076623b mov rdi, qword ptr [r8]
14076623e mov rdx, qword ptr [r8 + 8]
140766242 mov r11, qword ptr [rdi]
140766245 xor r8d, r8d
140766248 mov rcx, r11
14076624b sub rcx, qword ptr [rdx + 0x18]
14076624f cmovb rcx, r8
140766253 sub r11, qword ptr [rdx + 0x38]
140766257 cmovb r11, r8
14076625b mov rax, rcx
14076625e shl rax, 4
140766262 add rax, qword ptr [rdx + 8]
140766266 mov rbp, qword ptr [rdx + 0x10]
14076626a mov rsi, qword ptr [rdx + 0x30]
14076626e sub rbp, rcx
140766271 cmovb rbp, r8
140766275 mov r10d, 8
14076627b cmovb rax, r10
14076627f mov rcx, r11
140766282 shl rcx, 4
140766286 add rcx, qword ptr [rdx + 0x28]
14076628a sub rsi, r11
14076628d cmovb rsi, r8
140766291 cmovb rcx, r10
140766295 mov rdx, qword ptr [rdx + 0x40]
140766299 sub rdx, rsi
14076629c sub rdx, rbp
14076629f mov qword ptr [rdi], rdx
1407662a2 shl rbp, 4
1407662a6 add rbp, rax
1407662a9 shl rsi, 4
1407662ad add rsi, rcx
1407662b0 mov qword ptr [rsp + 0x28], rsi
1407662b5 mov qword ptr [rsp + 0x48], r9
1407662ba lea rdx, [r9 + 0x20]
1407662be mov qword ptr [rsp + 0x50], rdx
1407662c3 lea r12, [rsp + 0x100]
1407662cb lea r13, [rsp + 0x80]
1407662d3 jmp 0x1407662e6
1407662d5 nop word ptr cs:[rax + rax]
1407662e0 mov rcx, rbx
1407662e3 mov rax, rsi
1407662e6 test rax, rax
1407662e9 je 0x140766300
1407662eb cmp rax, rbp
1407662ee je 0x140766300
1407662f0 lea rsi, [rax + 0x10]
1407662f4 mov rbx, rcx
1407662f7 jmp 0x14076631d
1407662f9 nop dword ptr [rax]
140766300 test rcx, rcx
140766303 je 0x1407663df
140766309 cmp rcx, qword ptr [rsp + 0x28]
14076630e je 0x1407663df
140766314 lea rbx, [rcx + 0x10]
140766318 xor esi, esi
14076631a mov rax, rcx
14076631d inc qword ptr [rdi]
140766320 mov rax, qword ptr [rax + 8]
140766324 mov qword ptr [rsp + 0x100], rax
14076632c mov rcx, qword ptr [rsp + 0x50]
140766331 mov rdx, r12
140766334 call 0x1405b0d70
140766339 mov rcx, r13
14076633c mov rdx, qword ptr [rsp + 0x48]
140766341 mov r8, rax
140766344 mov r9, r12
140766347 call 0x140b76f90
14076634c mov eax, dword ptr [rsp + 0x88]
140766353 mov ecx, dword ptr [rsp + 0x8c]
14076635a mov rdx, rcx
14076635d shl rdx, 0x20
140766361 or rdx, rax
140766364 test ecx, ecx
140766366 mov r14d, 0
14076636c cmovne r14, rdx
140766370 mov rax, r14
140766373 shr rax, 0x20
140766377 je 0x1407662e0
14076637d mov rax, qword ptr [rsp + 0x220]
140766385 mov r15, qword ptr [rax]
140766388 mov rcx, r15
14076638b mov rdx, r14
14076638e call 0x140ceafa0
140766393 test al, al
140766395 je 0x1407662e0
14076639b mov rax, qword ptr [rsp + 0x220]
1407663a3 movups xmm0, xmmword ptr [rax + 8]
1407663a7 mov rax, qword ptr [rax + 0x18]
1407663ab mov qword ptr [rsp + 0x80], r15
1407663b3 movups xmmword ptr [rsp + 0x88], xmm0
1407663bb mov qword ptr [rsp + 0x98], rax
1407663c3 mov qword ptr [rsp + 0xa0], r14
1407663cb mov rcx, r13
1407663ce lea rdx, [rip + 0x23179fb]
1407663d5 call 0x140d14770
1407663da jmp 0x1407662e0
1407663df mov rax, qword ptr [rsp + 0x40]
1407663e4 mov r11, qword ptr [rax]
1407663e7 mov rcx, qword ptr [rax + 8]
1407663eb mov r8, qword ptr [r11]
1407663ee xor r10d, r10d
1407663f1 mov rdx, r8
1407663f4 sub rdx, qword ptr [rcx + 0x18]
1407663f8 cmovb rdx, r10
1407663fc sub r8, qword ptr [rcx + 0x38]
140766400 cmovb r8, r10
140766404 mov rax, rdx
140766407 shl rax, 4
14076640b add rax, qword ptr [rcx + 8]
14076640f mov r12, qword ptr [rcx + 0x10]
140766413 mov r14, qword ptr [rcx + 0x30]
140766417 sub r12, rdx
14076641a cmovb r12, r10
14076641e mov r9d, 8
140766424 cmovb rax, r9
140766428 mov rdx, r8
14076642b shl rdx, 4
14076642f add rdx, qword ptr [rcx + 0x28]
140766433 sub r14, r8
140766436 cmovb r14, r10
14076643a cmovb rdx, r9
14076643e mov rcx, qword ptr [rcx + 0x40]
140766442 sub rcx, r14
140766445 sub rcx, r12
140766448 mov qword ptr [rsp + 0x40], r11
14076644d mov qword ptr [r11], rcx
140766450 shl r12, 4
140766454 add r12, rax
140766457 shl r14, 4
14076645b add r14, rdx
14076645e mov rcx, qword ptr [rsp + 0x38]
140766463 mov rbp, qword ptr [rcx]
140766466 lea rcx, [rbp + 0x20]
14076646a mov qword ptr [rsp + 0x38], rcx
14076646f pcmpeqd xmm6, xmm6
140766473 xorps xmm7, xmm7
140766476 movss xmm8, dword ptr [rip + 0x21bf485]
14076647f movss xmm9, dword ptr [rip + 0x2266bf0]
140766488 movss xmm10, dword ptr [rip + 0x21bf4f7]
140766491 movss xmm11, dword ptr [rip + 0x223a336]
14076649a test rax, rax
14076649d je 0x1407664b0
14076649f cmp rax, r12
1407664a2 je 0x1407664b0
1407664a4 lea rbx, [rax + 0x10]
1407664a8 mov qword ptr [rsp + 0x28], rdx
1407664ad jmp 0x1407664d0
1407664af nop 
1407664b0 test rdx, rdx
1407664b3 je 0x1407668ef
1407664b9 cmp rdx, r14
1407664bc je 0x1407668ef
1407664c2 lea rax, [rdx + 0x10]
1407664c6 mov qword ptr [rsp + 0x28], rax
1407664cb xor ebx, ebx
1407664cd mov rax, rdx
1407664d0 mov rcx, qword ptr [rsp + 0x40]
1407664d5 inc qword ptr [rcx]
1407664d8 mov rax, qword ptr [rax + 8]
1407664dc mov qword ptr [rsp + 0x30], rax
1407664e1 mov rcx, qword ptr [rsp + 0x50]
1407664e6 lea rsi, [rsp + 0x30]
1407664eb mov rdx, rsi
1407664ee call 0x1405b0d70
1407664f3 lea rcx, [rsp + 0x80]
1407664fb mov rdx, qword ptr [rsp + 0x48]
140766500 mov r8, rax
140766503 mov r9, rsi
140766506 call 0x140b76f90
14076650b mov eax, dword ptr [rsp + 0x88]
140766512 mov ecx, dword ptr [rsp + 0x8c]
140766519 mov rdx, rcx
14076651c shl rdx, 0x20
140766520 or rdx, rax
140766523 test ecx, ecx
140766525 mov r15d, 0
14076652b cmovne r15, rdx
14076652f mov rax, r15
140766532 shr rax, 0x20
140766536 je 0x140766596
140766538 mov rax, qword ptr [rsp + 0x220]
140766540 mov r13, qword ptr [rax]
140766543 mov rcx, r13
140766546 mov rdx, r15
140766549 call 0x140ceafa0
14076654e test al, al
140766550 je 0x140766596
140766552 mov rax, qword ptr [rsp + 0x220]
14076655a movups xmm0, xmmword ptr [rax + 8]
14076655e mov rax, qword ptr [rax + 0x18]
140766562 mov qword ptr [rsp + 0x80], r13
14076656a movups xmmword ptr [rsp + 0x88], xmm0
140766572 mov qword ptr [rsp + 0x98], rax
14076657a mov qword ptr [rsp + 0xa0], r15
140766582 lea rcx, [rsp + 0x80]
14076658a lea rdx, [rip + 0x231780f]
140766591 call 0x140d14770
140766596 cmp qword ptr [rbp + 0x18], 0
14076659b je 0x1407668e3
1407665a1 mov rcx, qword ptr [rsp + 0x38]
1407665a6 mov rdx, rsi
1407665a9 call 0x1405b0d70
1407665ae mov rcx, qword ptr [rbp]
1407665b2 mov rdx, qword ptr [rbp + 8]
1407665b6 mov r8, rdx
1407665b9 and r8, rax
1407665bc shr rax, 0x39
1407665c0 movd xmm0, eax
1407665c4 punpcklbw xmm0, xmm0
1407665c8 pshuflw xmm0, xmm0, 0
1407665cd pshufd xmm0, xmm0, 0
1407665d2 lea rax, [rcx - 0x2a8]
1407665d9 mov r9, qword ptr [rsp + 0x30]
1407665de xor r10d, r10d
1407665e1 movdqu xmm1, xmmword ptr [rcx + r8]
1407665e7 movdqa xmm2, xmm1
1407665eb pcmpeqb xmm2, xmm0
1407665ef pmovmskb r11d, xmm2
1407665f4 test r11d, r11d
1407665f7 je 0x140766630
1407665f9 tzcnt esi, r11d
1407665fe add rsi, r8
140766601 and rsi, rdx
140766604 neg rsi
140766607 imul rdi, rsi, 0x2a8
14076660e cmp r9, qword ptr [rax + rdi]
140766612 je 0x140766660
140766614 lea esi, [r11 - 1]
140766618 and si, r11w
14076661c mov r11d, esi
14076661f jne 0x1407665f9
140766621 nop word ptr cs:[rax + rax]
140766630 pcmpeqb xmm1, xmm6
140766634 pmovmskb r11d, xmm1
140766639 test r11d, r11d
14076663c jne 0x1407668e3
140766642 add r8, r10
140766645 add r8, 0x10
140766649 add r10, 0x10
14076664d and r8, rdx
140766650 jmp 0x1407665e1
140766652 nop word ptr cs:[rax + rax]
140766660 movss xmm0, dword ptr [rcx + rdi - 0x20]
140766666 ucomiss xmm0, xmm7
140766669 mov rax, rbx
14076666c mov r8, qword ptr [rsp + 0x28]
140766671 mov rdx, r8
140766674 jbe 0x14076649a
14076667a lea rsi, [rcx + rdi]
14076667e cmp dword ptr [rsi - 0x48], 1
140766682 mov rax, rbx
140766685 mov rdx, r8
140766688 ja 0x14076649a
14076668e add rcx, rdi
140766691 add rcx, -0x2a0
140766698 call 0x140b2a5d0
14076669d movups xmm0, xmmword ptr [rax]
1407666a0 movups xmm1, xmmword ptr [rax + 0xc]
1407666a4 movaps xmmword ptr [rsp + 0x60], xmm0
1407666a9 movups xmmword ptr [rsp + 0x6c], xmm1
1407666ae xor ecx, ecx
1407666b0 test byte ptr [rsp + 0x60], 1
1407666b5 jne 0x1407666f8
1407666b7 movss xmm0, dword ptr [rsp + 0x6c]
1407666bd maxss xmm0, xmm8
1407666c2 mulss xmm0, xmm9
1407666c7 call 0x142923f50
1407666cc movaps xmm1, xmm8
1407666d0 maxss xmm1, xmm0
1407666d4 movaps xmm0, xmm10
1407666d8 minss xmm0, xmm1
1407666dc cvttss2si rcx, xmm0
1407666e1 ucomiss xmm0, xmm7
1407666e4 mov eax, 0
1407666e9 cmovb ecx, eax
1407666ec ucomiss xmm0, xmm11
1407666f0 mov eax, 0xffffffff
1407666f5 cmova ecx, eax
1407666f8 call 0x140833250
1407666fd lea rcx, [rsp + 0x130]
140766705 mov edx, eax
140766707 call 0x14082bd60
14076670c lea rcx, [rsp + 0x60]
140766711 call 0x1411ff2a0
140766716 movss dword ptr [rsp + 0x58], xmm0
14076671c movss dword ptr [rsp + 0x5c], xmm1
140766722 movss xmm2, dword ptr [rsi - 0x20]
140766727 lea rsi, [rsp + 0x90]
14076672f mov rcx, rsi
140766732 lea rdx, [rsp + 0x58]
140766737 call 0x140c9cde0
14076673c lea rdi, [rsp + 0x100]
140766744 mov rcx, rdi
140766747 call 0x140c9d180
14076674c movaps xmm12, xmmword ptr [rsp + 0x100]
140766755 lea rcx, [rsp + 0x11c]
14076675d mov eax, dword ptr [rcx + 8]
140766760 mov dword ptr [rsi + 0x14], eax
140766763 mov rax, qword ptr [rcx]
140766766 mov qword ptr [rsi + 0xc], rax
14076676a call 0x14003bb80
14076676f mov rax, qword ptr [rsp + 0x30]
140766774 movups xmm0, xmmword ptr [rsp + 0x6c]
140766779 movups xmmword ptr [rsi + 0x54], xmm0
14076677d movaps xmm0, xmmword ptr [rsp + 0x60]
140766782 movups xmmword ptr [rsi + 0x48], xmm0
140766786 movaps xmmword ptr [rsp + 0x80], xmm12
14076678f movaps xmm0, xmmword ptr [rsp + 0x130]
140766797 movaps xmmword ptr [rsi + 0x20], xmm0
14076679b mov byte ptr [rsp + 0xc0], 9
1407667a3 mov qword ptr [rsp + 0xd0], rax
1407667ab mov rcx, rdi
1407667ae mov rdx, qword ptr [rsp + 0x220]
1407667b6 call 0x140d146a0
1407667bb mov r15, qword ptr [rsp + 0x108]
1407667c3 mov r8, qword ptr [rsp + 0x120]
1407667cb test r15, r15
1407667ce jne 0x1407667d8
1407667d0 mov r15, qword ptr [rsp + 0x110]
1407667d8 mov rax, qword ptr [r15]
1407667db mov r13, qword ptr [r15 + 0x10]
1407667df sub rax, r13
1407667e2 cmp rax, 0xb7
1407667e8 jbe 0x1407668b8
1407667ee mov rax, qword ptr [r15 + 8]
1407667f2 lea rcx, [rip + 0xaa87f7]
1407667f9 mov qword ptr [rax + r13], rcx
1407667fd movaps xmm0, xmmword ptr [rsp + 0x80]
140766805 movdqa xmm1, xmmword ptr [rsp + 0x90]
14076680e movdqa xmm2, xmmword ptr [rsp + 0xa0]
140766817 movaps xmm3, xmmword ptr [rsp + 0xb0]
14076681f movups xmmword ptr [rax + r13 + 8], xmm0
140766825 movdqu xmmword ptr [rax + r13 + 0x18], xmm1
14076682c movdqu xmmword ptr [rax + r13 + 0x28], xmm2
140766833 movups xmmword ptr [rax + r13 + 0x38], xmm3
140766839 movaps xmm0, xmmword ptr [rsp + 0xc0]
140766841 movups xmmword ptr [rax + r13 + 0x48], xmm0
140766847 movaps xmm0, xmmword ptr [rsp + 0xd0]
14076684f movups xmmword ptr [rax + r13 + 0x58], xmm0
140766855 movaps xmm0, xmmword ptr [rsp + 0xe0]
14076685d movups xmmword ptr [rax + r13 + 0x68], xmm0
140766863 movaps xmm0, xmmword ptr [rsp + 0xf0]
14076686b movups xmmword ptr [rax + r13 + 0x78], xmm0
140766871 mov byte ptr [rax + r13 + 0x88], 0
14076687a mov qword ptr [rax + r13 + 0x98], r8
140766882 lea rcx, [rip + 0xaa9737]
140766889 mov qword ptr [rax + r13 + 0xa8], rcx
140766891 add r13, 0xb8
140766898 mov qword ptr [r15 + 0x10], r13
14076689c mov rdx, qword ptr [rsp + 0x30]
1407668a1 mov rcx, qword ptr [rsp + 0x48]
1407668a6 call 0x140eec970
1407668ab mov rax, rbx
1407668ae mov rdx, qword ptr [rsp + 0x28]
1407668b3 jmp 0x14076649a
1407668b8 mov qword ptr [rsp + 0x20], 1
1407668c1 mov rsi, r8
1407668c4 mov r8d, 0xb8
1407668ca mov r9d, 1
1407668d0 mov rcx, r15
1407668d3 mov rdx, r13
1407668d6 call 0x1428a7de0
1407668db mov r8, rsi
1407668de jmp 0x1407667ee
1407668e3 lea rcx, [rip + 0x23174ce]
1407668ea call 0x1428d9310
1407668ef movaps xmm6, xmmword ptr [rsp + 0x140]
1407668f7 movaps xmm7, xmmword ptr [rsp + 0x150]
1407668ff movaps xmm8, xmmword ptr [rsp + 0x160]
140766908 movaps xmm9, xmmword ptr [rsp + 0x170]
140766911 movaps xmm10, xmmword ptr [rsp + 0x180]
14076691a movaps xmm11, xmmword ptr [rsp + 0x190]
140766923 movaps xmm12, xmmword ptr [rsp + 0x1a0]
14076692c add rsp, 0x1b8
140766933 pop rbx
140766934 pop rbp
140766935 pop rdi
140766936 pop rsi
140766937 pop r12
140766939 pop r13
14076693b pop r14
14076693d pop r15
14076693f ret 
