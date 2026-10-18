0x1406884c0 push rbp
0x1406884c1 push r15
0x1406884c3 push r14
0x1406884c5 push r13
0x1406884c7 push r12
0x1406884c9 push rsi
0x1406884ca push rdi
0x1406884cb push rbx
0x1406884cc sub rsp, 0x238
0x1406884d3 lea rbp, [rsp + 0x80]
0x1406884db movaps xmmword ptr [rbp + 0x1a0], xmm15
0x1406884e3 movaps xmmword ptr [rbp + 0x190], xmm14
0x1406884eb movaps xmmword ptr [rbp + 0x180], xmm13
0x1406884f3 movaps xmmword ptr [rbp + 0x170], xmm12
0x1406884fb movdqa xmmword ptr [rbp + 0x160], xmm11
0x140688504 movaps xmmword ptr [rbp + 0x150], xmm10
0x14068850c movaps xmmword ptr [rbp + 0x140], xmm9
0x140688514 movdqa xmmword ptr [rbp + 0x130], xmm8
0x14068851d movaps xmmword ptr [rbp + 0x120], xmm7
0x140688524 movaps xmmword ptr [rbp + 0x110], xmm6
0x14068852b mov qword ptr [rbp + 0x108], 0xfffffffffffffffe
0x140688536 mov r14, r9
0x140688539 mov r12, r8
0x14068853c mov rdi, rdx
0x14068853f mov rsi, rcx
0x140688542 call 0x140c16a00
0x140688547 test al, al
0x140688549 je 0x1406885aa
0x14068854b lea r15, [rip + 0x238f3fe]
0x140688552 mov ebx, 0x23
0x140688557 mov edx, 0x23
0x14068855c mov rcx, r15
0x14068855f call 0x140682140
0x140688564 cmp rax, 1
0x140688568 jne 0x14068861f
0x14068856e mov r9, rdx
0x140688571 test rdx, rdx
0x140688574 je 0x1406885b3
0x140688576 cmp r9, 0x23
0x14068857a jae 0x1406885b1
0x14068857c lea rax, [rip + 0x238f3cd]
0x140688583 cmp byte ptr [r9 + rax], 0xbf
0x140688588 jg 0x1406885b3
0x14068858a lea rax, [rip + 0x238f48f]
0x140688591 mov qword ptr [rsp + 0x20], rax
0x140688596 lea rcx, [rip + 0x238f3b3]
0x14068859d mov edx, 0x23
0x1406885a2 xor r8d, r8d
0x1406885a5 call 0x1428d9940
0x1406885aa xor eax, eax
0x1406885ac jmp 0x140688784
0x1406885b1 jne 0x14068858a
0x1406885b3 lea r15, [rip + 0x238f396]
0x1406885ba mov rcx, r15
0x1406885bd mov rdx, r9
0x1406885c0 call 0x140682140
0x1406885c5 cmp rax, 1
0x1406885c9 jne 0x14068861f
0x1406885cb mov r8, rdx
0x1406885ce add r8, 2
0x1406885d2 je 0x14068860d
0x1406885d4 cmp r8, 0x23
0x1406885d8 jae 0x14068860b
0x1406885da lea rax, [rip + 0x238f36f]
0x1406885e1 cmp byte ptr [r8 + rax], 0xbf
0x1406885e6 jg 0x14068860d
0x1406885e8 lea rax, [rip + 0x238f449]
0x1406885ef mov qword ptr [rsp + 0x20], rax
0x1406885f4 lea rcx, [rip + 0x238f355]
0x1406885fb mov edx, 0x23
0x140688600 mov r9d, 0x23
0x140688606 call 0x1428d9940
0x14068860b jne 0x1406885e8
0x14068860d mov ebx, 0x21
0x140688612 sub rbx, rdx
0x140688615 lea r15, [rip + 0x238f334]
0x14068861c add r15, r8
0x14068861f lea r8, [rip + 0x238f5f8]
0x140688626 lea r13, [rip + 0x238f5cb]
0x14068862d mov qword ptr [rbp + 0x90], rbx
0x140688634 nop word ptr cs:[rax + rax]
0x140688640 movsx eax, byte ptr [r8 - 1]
0x140688645 test eax, eax
0x140688647 js 0x140688660
0x140688649 dec r8
0x14068864c cmp eax, 0x5c
0x14068864f jne 0x1406886b4
0x140688651 jmp 0x1406886c9
0x140688653 nop word ptr cs:[rax + rax]
0x140688660 movzx ecx, byte ptr [r8 - 2]
0x140688665 cmp cl, 0xc0
0x140688668 jge 0x14068868d
0x14068866a movzx edx, byte ptr [r8 - 3]
0x14068866f cmp dl, 0xc0
0x140688672 jge 0x140688696
0x140688674 movzx r9d, byte ptr [r8 - 4]
0x140688679 add r8, -4
0x14068867d and r9d, 7
0x140688681 shl r9d, 6
0x140688685 and edx, 0x3f
0x140688688 or edx, r9d
0x14068868b jmp 0x14068869d
0x14068868d add r8, -2
0x140688691 and ecx, 0x1f
0x140688694 jmp 0x1406886a5
0x140688696 add r8, -3
0x14068869a and edx, 0xf
0x14068869d shl edx, 6
0x1406886a0 and ecx, 0x3f
0x1406886a3 or ecx, edx
0x1406886a5 shl ecx, 6
0x1406886a8 and al, 0x3f
0x1406886aa movzx eax, al
0x1406886ad or eax, ecx
0x1406886af cmp eax, 0x5c
0x1406886b2 je 0x1406886c9
0x1406886b4 cmp eax, 0x2f
0x1406886b7 je 0x1406886c9
0x1406886b9 cmp r8, r13
0x1406886bc jne 0x140688640
0x1406886be mov rbx, r15
0x1406886c1 mov r15d, 0x26
0x1406886c7 jmp 0x140688719
0x1406886c9 lea r13, [rip + 0x238f528]
0x1406886d0 sub r8, r13
0x1406886d3 inc r8
0x1406886d6 je 0x14068870a
0x1406886d8 cmp r8, 0x26
0x1406886dc jae 0x140688708
0x1406886de cmp byte ptr [r8 + r13], 0xbf
0x1406886e3 jg 0x14068870a
0x1406886e5 lea rax, [rip + 0x238f314]
0x1406886ec mov qword ptr [rsp + 0x20], rax
0x1406886f1 lea rcx, [rip + 0x238f500]
0x1406886f8 mov edx, 0x26
0x1406886fd mov r9d, 0x26
0x140688703 call 0x1428d9940
0x140688708 jne 0x1406886e5
0x14068870a mov rbx, r15
0x14068870d mov r15d, 0x26
0x140688713 sub r15, r8
0x140688716 add r13, r8
0x140688719 call 0x140c18130
0x14068871e mov rcx, rax
0x140688721 mov rax, qword ptr [rax]
0x140688724 cmp rax, 1
0x140688728 jne 0x140688e90
0x14068872e add rcx, 8
0x140688732 cmp qword ptr [rcx], 0
0x140688736 jne 0x140689afb
0x14068873c mov qword ptr [rcx], 0xffffffffffffffff
0x140688743 mov qword ptr [rbp + 0x28], rcx
0x140688747 add rcx, 8
0x14068874b mov qword ptr [rsp + 0x20], r15
0x140688750 mov qword ptr [rsp + 0x30], 0
0x140688759 mov qword ptr [rsp + 0x28], 1
0x140688762 mov rdx, rbx
0x140688765 mov r8, qword ptr [rbp + 0x90]
0x14068876c mov r9, r13
0x14068876f call 0x140c16d40
0x140688774 mov rcx, qword ptr [rbp + 0x28]
0x140688778 inc qword ptr [rcx]
0x14068877b mov qword ptr [rbp - 0x30], rax
0x14068877f mov eax, 1
0x140688784 mov rcx, qword ptr [rbp + 0x228]
0x14068878b mov qword ptr [rbp - 0x38], rax
0x14068878f mov eax, dword ptr [rsi + 0x1c]
0x140688792 mov rdx, qword ptr [rsi + 0x10]
0x140688796 mov dword ptr [rdx], eax
0x140688798 mov rax, qword ptr [rsi]
0x14068879b mov qword ptr [rbp + 0xf8], rax
0x1406887a2 mov rcx, qword ptr [rcx]
0x1406887a5 call 0x140949c10
0x1406887aa movaps xmm7, xmm0
0x1406887ad movaps xmm6, xmm1
0x1406887b0 mov rax, qword ptr [rbp + 0x220]
0x1406887b7 mov ecx, dword ptr [rdi + 0x1c]
0x1406887ba mov rdx, qword ptr [rdi + 0x10]
0x1406887be mov dword ptr [rdx], ecx
0x1406887c0 mov rcx, qword ptr [rdi]
0x1406887c3 mov qword ptr [rbp + 0x90], rcx
0x1406887ca mov ecx, dword ptr [r14 + 0x1c]
0x1406887ce mov rdx, qword ptr [r14 + 0x10]
0x1406887d2 mov dword ptr [rdx], ecx
0x1406887d4 mov rcx, qword ptr [r14]
0x1406887d7 mov qword ptr [rbp + 0xe8], rcx
0x1406887de mov rcx, qword ptr [rax]
0x1406887e1 call 0x140987ca0
0x1406887e6 mov qword ptr [rbp + 0xf0], rax
0x1406887ed mov rax, qword ptr [rbp + 0x240]
0x1406887f4 mov ecx, dword ptr [r12 + 0x1c]
0x1406887f9 mov rdx, qword ptr [r12 + 0x10]
0x1406887fe mov dword ptr [rdx], ecx
0x140688800 mov rcx, qword ptr [r12]
0x140688804 mov qword ptr [rbp + 0x30], rcx
0x140688808 mov rcx, qword ptr [rax]
0x14068880b call 0x140958f30
0x140688810 mov byte ptr [rbp + 0x40], al
0x140688813 lea rcx, [rbp + 0x40]
0x140688817 call 0x1408c5680
0x14068881c mov rcx, qword ptr [rbp + 0x238]
0x140688823 mov rdx, qword ptr [rcx]
0x140688826 mov rcx, rdx
0x140688829 mov qword ptr [rbp + 0xd0], rdx
0x140688830 test rdx, rdx
0x140688833 je 0x14068883f
0x140688835 mov rcx, qword ptr [rbp + 0x230]
0x14068883c mov rcx, qword ptr [rcx]
0x14068883f xor edx, edx
0x140688841 test al, al
0x140688843 cmovne rdx, rcx
0x140688847 mov qword ptr [rbp + 0xc8], rdx
0x14068884e divss xmm6, dword ptr [rip + 0x22d2dda]
0x140688856 divss xmm7, xmm6
0x14068885a mov rcx, qword ptr [rbp + 0xf8]
0x140688861 lea rax, [rcx + 0x1a0]
0x140688868 mov qword ptr [rbp - 0x20], rax
0x14068886c mov r12, qword ptr [rcx + 0x1a0]
0x140688873 mov rbx, qword ptr [rcx + 0x1b8]
0x14068887a movdqa xmm0, xmmword ptr [r12]
0x140688880 pmovmskb eax, xmm0
0x140688884 not eax
0x140688886 lea r15, [r12 + 0x10]
0x14068888b lea r13, [rcx + 0x30]
0x14068888f add rcx, 0x268
0x140688896 mov qword ptr [rbp + 0x28], rcx
0x14068889a lea r14, [rbp + 0x84]
0x1406888a1 jmp 0x1406888bc
0x1406888a3 nop word ptr cs:[rax + rax]
0x1406888b0 mov dword ptr [rbp + 0xa0], 2
0x1406888ba mov eax, esi
0x1406888bc test rbx, rbx
0x1406888bf je 0x1406889c6
0x1406888c5 test ax, ax
0x1406888c8 je 0x1406888e0
0x1406888ca lea esi, [rax - 1]
0x1406888cd and esi, eax
0x1406888cf jmp 0x140688908
0x1406888d1 nop word ptr cs:[rax + rax]
0x1406888e0 movdqa xmm0, xmmword ptr [r15]
0x1406888e5 pmovmskb eax, xmm0
0x1406888e9 add r12, -0xc00
0x1406888f0 add r15, 0x10
0x1406888f4 cmp eax, 0xffff
0x1406888f9 je 0x1406888e0
0x1406888fb mov ecx, 0xfffffffe
0x140688900 sub ecx, eax
0x140688902 not eax
0x140688904 mov esi, eax
0x140688906 and esi, ecx
0x140688908 dec rbx
0x14068890b tzcnt eax, eax
0x14068890f neg rax
0x140688912 lea rdi, [rax + rax*2]
0x140688916 shl rdi, 6
0x14068891a mov rdx, qword ptr [r12 + rdi - 0x38]
0x14068891f test rdx, rdx
0x140688922 je 0x1406888b0
0x140688924 add rdi, r12
0x140688927 lea rax, [rdi - 2]
0x14068892b mov r9, qword ptr [rdi - 0x30]
0x14068892f mov r8, qword ptr [rbp + 0x28]
0x140688933 mov ecx, dword ptr [r8 + 8]
0x140688937 mov dword ptr [r14 + 8], ecx
0x14068893b mov rcx, qword ptr [r8]
0x14068893e mov qword ptr [r14], rcx
0x140688941 mov rcx, qword ptr [rbp + 0x90]
0x140688948 mov qword ptr [rbp + 0x40], rcx
0x14068894c mov rcx, qword ptr [rbp + 0xf0]
0x140688953 mov qword ptr [rbp + 0x48], rcx
0x140688957 mov rcx, qword ptr [rbp + 0xe8]
0x14068895e mov qword ptr [rbp + 0x50], rcx
0x140688962 movss dword ptr [rbp + 0x68], xmm7
0x140688967 mov dword ptr [rbp + 0x6c], 0x447a0000
0x14068896e movss dword ptr [rbp + 0x80], xmm6
0x140688976 mov qword ptr [rbp + 0x58], rax
0x14068897a mov qword ptr [rbp + 0x60], r13
0x14068897e mov rax, qword ptr [rbp + 0xc8]
0x140688985 mov qword ptr [rbp + 0x70], rax
0x140688989 mov rax, qword ptr [rbp + 0xd0]
0x140688990 mov qword ptr [rbp + 0x78], rax
0x140688994 lea rcx, [rbp + 0xa0]
0x14068899b lea r8, [rbp + 0x40]
0x14068899f call qword ptr [r9 + 0x60]
0x1406889a3 cmp dword ptr [rbp + 0xa0], 2
0x1406889aa mov eax, esi
0x1406889ac je 0x1406888bc
0x1406889b2 movdqu xmm0, xmmword ptr [rbp + 0xa0]
0x1406889ba movdqu xmmword ptr [rdi - 0x58], xmm0
0x1406889bf mov eax, esi
0x1406889c1 jmp 0x1406888bc
0x1406889c6 mov qword ptr [rbp - 0x18], 0
0x1406889ce mov qword ptr [rbp - 0x10], 8
0x1406889d6 mov qword ptr [rbp - 8], 0
0x1406889de lea rbx, [rbp + 8]
0x1406889e2 mov rcx, rbx
0x1406889e5 call 0x140d4a3b0
0x1406889ea mov qword ptr [rbp + 0xa0], 8
0x1406889f5 mov qword ptr [rbp + 0xa8], rbx
0x1406889fc pxor xmm0, xmm0
0x140688a00 movdqu xmmword ptr [rbp + 0xb0], xmm0
0x140688a08 mov rbx, qword ptr [rbp + 0xf8]
0x140688a0f mov r13, qword ptr [rbx + 8]
0x140688a13 imul rsi, qword ptr [rbx + 0x10], 0x38
0x140688a18 add rsi, r13
0x140688a1b lea rax, [rbx + 0x1c0]
0x140688a22 mov qword ptr [rbp + 0xe8], rax
0x140688a29 xor r14d, r14d
0x140688a2c pcmpeqd xmm8, xmm8
0x140688a31 lea r15, [rbp + 0xa0]
0x140688a38 jmp 0x140688a9d
0x140688a3a nop word ptr [rax + rax]
0x140688a40 and bl, 2
0x140688a43 mov rax, qword ptr [rbp + 0xa0]
0x140688a4a shl r14, 6
0x140688a4e movdqa xmm0, xmmword ptr [rbp + 0x40]
0x140688a53 movaps xmm1, xmmword ptr [rbp + 0x50]
0x140688a57 movdqa xmm2, xmmword ptr [rbp + 0x60]
0x140688a5c movdqu xmmword ptr [rax + r14 + 0x20], xmm2
0x140688a63 movups xmmword ptr [rax + r14 + 0x10], xmm1
0x140688a69 movdqu xmmword ptr [rax + r14], xmm0
0x140688a6f mov qword ptr [rax + r14 + 0x30], r12
0x140688a74 mov dword ptr [rax + r14 + 0x38], 0x3f800000
0x140688a7d mov byte ptr [rax + r14 + 0x3c], bl
0x140688a82 mov r14, qword ptr [rbp + 0xb8]
0x140688a89 inc r14
0x140688a8c mov qword ptr [rbp + 0xb8], r14
0x140688a93 mov r13, rdi
0x140688a96 mov rbx, qword ptr [rbp + 0xf8]
0x140688a9d cmp r13, rsi
0x140688aa0 je 0x140688bcf
0x140688aa6 cmp qword ptr [rbx + 0x1b8], 0
0x140688aae je 0x140688bbe
0x140688ab4 lea rdi, [r13 + 0x38]
0x140688ab8 lea rdx, [r13 + 0x30]
0x140688abc mov r12, qword ptr [r13 + 0x30]
0x140688ac0 mov rcx, qword ptr [rbp + 0xe8]
0x140688ac7 call 0x1405b0d70
0x140688acc mov rcx, qword ptr [rbx + 0x1a0]
0x140688ad3 mov rdx, qword ptr [rbx + 0x1a8]
0x140688ada mov r8, rdx
0x140688add and r8, rax
0x140688ae0 shr rax, 0x39
0x140688ae4 movd xmm0, eax
0x140688ae8 punpcklbw xmm0, xmm0
0x140688aec pshuflw xmm0, xmm0, 0
0x140688af1 pshufd xmm0, xmm0, 0
0x140688af6 lea rax, [rcx - 0xc0]
0x140688afd xor r9d, r9d
0x140688b00 movdqu xmm1, xmmword ptr [rcx + r8]
0x140688b06 movdqa xmm2, xmm1
0x140688b0a pcmpeqb xmm2, xmm0
0x140688b0e pmovmskb r10d, xmm2
0x140688b13 test r10d, r10d
0x140688b16 je 0x140688b50
0x140688b18 tzcnt r11d, r10d
0x140688b1d add r11, r8
0x140688b20 and r11, rdx
0x140688b23 neg r11
0x140688b26 lea r11, [r11 + r11*2]
0x140688b2a shl r11, 6
0x140688b2e cmp r12, qword ptr [rax + r11]
0x140688b32 je 0x140688b70
0x140688b34 lea r11d, [r10 - 1]
0x140688b38 and r11w, r10w
0x140688b3c mov r10d, r11d
0x140688b3f jne 0x140688b18
0x140688b41 nop word ptr cs:[rax + rax]
0x140688b50 pcmpeqb xmm1, xmm8
0x140688b55 pmovmskb r10d, xmm1
0x140688b5a test r10d, r10d
0x140688b5d jne 0x140688bbe
0x140688b5f add r8, r9
0x140688b62 add r8, 0x10
0x140688b66 add r9, 0x10
0x140688b6a and r8, rdx
0x140688b6d jmp 0x140688b00
0x140688b6f nop 
0x140688b70 movzx ebx, byte ptr [rcx + r11 - 1]
0x140688b76 movups xmm0, xmmword ptr [r13]
0x140688b7b movups xmm1, xmmword ptr [r13 + 0x10]
0x140688b80 movups xmm2, xmmword ptr [r13 + 0x20]
0x140688b85 movaps xmmword ptr [rbp + 0x60], xmm2
0x140688b89 movaps xmmword ptr [rbp + 0x50], xmm1
0x140688b8d movaps xmmword ptr [rbp + 0x40], xmm0
0x140688b91 cmp r14, qword ptr [rbp + 0xb0]
0x140688b98 jne 0x140688a40
0x140688b9e mov r8d, 1
0x140688ba4 mov rcx, r15
0x140688ba7 mov rdx, r14
0x140688baa mov r9b, 1
0x140688bad call 0x140fe08d0
0x140688bb2 mov r14, qword ptr [rbp + 0xb8]
0x140688bb9 jmp 0x140688a40
0x140688bbe lea rcx, [rip + 0x238f2ab]
0x140688bc5 call 0x1428d9310
0x140688bca jmp 0x14068adad
0x140688bcf cmp r14, 2
0x140688bd3 jb 0x140688c4b
0x140688bd5 mov rax, qword ptr [rbp + 0xa0]
0x140688bdc mov rcx, r14
0x140688bdf shr rcx, 1
0x140688be2 shl r14, 6
0x140688be6 lea rdx, [rax + r14]
0x140688bea add rdx, -0x10
0x140688bee add rax, 0x30
0x140688bf2 nop word ptr cs:[rax + rax]
0x140688c00 movups xmm0, xmmword ptr [rax - 0x30]
0x140688c04 movups xmm1, xmmword ptr [rdx - 0x30]
0x140688c08 movups xmmword ptr [rax - 0x30], xmm1
0x140688c0c movups xmmword ptr [rdx - 0x30], xmm0
0x140688c10 movups xmm0, xmmword ptr [rax - 0x20]
0x140688c14 movups xmm1, xmmword ptr [rdx - 0x20]
0x140688c18 movups xmmword ptr [rax - 0x20], xmm1
0x140688c1c movups xmmword ptr [rdx - 0x20], xmm0
0x140688c20 movups xmm0, xmmword ptr [rax - 0x10]
0x140688c24 movups xmm1, xmmword ptr [rdx - 0x10]
0x140688c28 movups xmmword ptr [rax - 0x10], xmm1
0x140688c2c movups xmmword ptr [rdx - 0x10], xmm0
0x140688c30 movdqu xmm0, xmmword ptr [rax]
0x140688c34 movups xmm1, xmmword ptr [rdx]
0x140688c37 movups xmmword ptr [rax], xmm1
0x140688c3a movdqu xmmword ptr [rdx], xmm0
0x140688c3e add rax, 0x40
0x140688c42 add rdx, -0x40
0x140688c46 dec rcx
0x140688c49 jne 0x140688c00
0x140688c4b lea r14, [rbp + 0x20]
0x140688c4f pcmpeqd xmm8, xmm8
0x140688c54 movss xmm10, dword ptr [rip + 0x229cca7]
0x140688c5d lea r13, [rbp + 0xa0]
0x140688c64 mov rax, qword ptr [rbp + 0xb8]
0x140688c6b test rax, rax
0x140688c6e je 0x140688eb4
0x140688c74 dec rax
0x140688c77 mov qword ptr [rbp + 0xb8], rax
0x140688c7e mov rcx, qword ptr [rbp + 0xa0]
0x140688c85 shl rax, 6
0x140688c89 mov ebx, dword ptr [rcx + rax]
0x140688c8c cmp ebx, 6
0x140688c8f je 0x140688eb4
0x140688c95 lea rdx, [rcx + rax]
0x140688c99 mov rdi, qword ptr [rcx + rax + 0x30]
0x140688c9e movss xmm9, dword ptr [rcx + rax + 0x38]
0x140688ca5 movzx esi, byte ptr [rcx + rax + 0x3c]
0x140688caa mov qword ptr [rbp + 0x20], rdi
0x140688cae movups xmm0, xmmword ptr [rdx + 4]
0x140688cb2 movups xmm1, xmmword ptr [rdx + 0x14]
0x140688cb6 movups xmm2, xmmword ptr [rdx + 0x20]
0x140688cba movups xmmword ptr [rbp + 0x5c], xmm2
0x140688cbe movaps xmmword ptr [rbp + 0x50], xmm1
0x140688cc2 movaps xmmword ptr [rbp + 0x40], xmm0
0x140688cc6 mov r15, qword ptr [rbp - 8]
0x140688cca cmp r15, qword ptr [rbp - 0x18]
0x140688cce jne 0x140688ce0
0x140688cd0 lea rcx, [rbp - 0x18]
0x140688cd4 lea rdx, [rip + 0x238f165]
0x140688cdb call 0x1406319d0
0x140688ce0 mov rax, qword ptr [rbp - 0x10]
0x140688ce4 imul rcx, r15, 0x38
0x140688ce8 mov qword ptr [rax + rcx], rdi
0x140688cec mov dword ptr [rax + rcx + 8], ebx
0x140688cf0 movaps xmm0, xmmword ptr [rbp + 0x40]
0x140688cf4 movaps xmm1, xmmword ptr [rbp + 0x50]
0x140688cf8 movups xmmword ptr [rax + rcx + 0xc], xmm0
0x140688cfd movups xmmword ptr [rax + rcx + 0x1c], xmm1
0x140688d02 movups xmm0, xmmword ptr [rbp + 0x5c]
0x140688d06 movups xmmword ptr [rax + rcx + 0x28], xmm0
0x140688d0b inc r15
0x140688d0e mov qword ptr [rbp - 8], r15
0x140688d12 mov rbx, qword ptr [rbp + 0xf8]
0x140688d19 cmp qword ptr [rbx + 0x1b8], 0
0x140688d21 je 0x140688e7f
0x140688d27 mov rcx, qword ptr [rbp + 0xe8]
0x140688d2e mov rdx, r14
0x140688d31 call 0x1405b0d70
0x140688d36 mov rcx, qword ptr [rbx + 0x1a0]
0x140688d3d mov rdx, qword ptr [rbx + 0x1a8]
0x140688d44 mov r8, rdx
0x140688d47 and r8, rax
0x140688d4a shr rax, 0x39
0x140688d4e movd xmm0, eax
0x140688d52 punpcklbw xmm0, xmm0
0x140688d56 pshuflw xmm0, xmm0, 0
0x140688d5b pshufd xmm0, xmm0, 0
0x140688d60 lea rax, [rcx - 0xc0]
0x140688d67 xor r9d, r9d
0x140688d6a movdqu xmm1, xmmword ptr [rcx + r8]
0x140688d70 movdqa xmm2, xmm1
0x140688d74 pcmpeqb xmm2, xmm0
0x140688d78 pmovmskb r10d, xmm2
0x140688d7d test r10d, r10d
0x140688d80 je 0x140688dab
0x140688d82 tzcnt r11d, r10d
0x140688d87 add r11, r8
0x140688d8a and r11, rdx
0x140688d8d neg r11
0x140688d90 lea r11, [r11 + r11*2]
0x140688d94 shl r11, 6
0x140688d98 cmp rdi, qword ptr [rax + r11]
0x140688d9c je 0x140688dce
0x140688d9e lea r11d, [r10 - 1]
0x140688da2 and r11w, r10w
0x140688da6 mov r10d, r11d
0x140688da9 jne 0x140688d82
0x140688dab pcmpeqb xmm1, xmm8
0x140688db0 pmovmskb r10d, xmm1
0x140688db5 test r10d, r10d
0x140688db8 jne 0x140688e7f
0x140688dbe add r8, r9
0x140688dc1 add r8, 0x10
0x140688dc5 add r9, 0x10
0x140688dc9 and r8, rdx
0x140688dcc jmp 0x140688d6a
0x140688dce lea rdi, [rcx + r11]
0x140688dd2 addss xmm9, dword ptr [rcx + r11 - 0x10]
0x140688dd9 movss dword ptr [rcx + r11 - 0xc], xmm9
0x140688de0 movzx r12d, byte ptr [rcx + r11 - 1]
0x140688de6 mov eax, r12d
0x140688de9 or al, sil
0x140688dec mov byte ptr [rcx + r11 - 1], al
0x140688df1 and r12b, 2
0x140688df5 or r12b, sil
0x140688df8 mov rsi, qword ptr [rcx + r11 - 0xb0]
0x140688e00 mov rax, qword ptr [rcx + r11 - 0xa8]
0x140688e08 lea r15, [rsi + rax*8]
0x140688e0c jmp 0x140688e3e
0x140688e0e nop 
0x140688e10 add rsi, 8
0x140688e14 mov rax, qword ptr [rbp + 0xa0]
0x140688e1b shl rdx, 6
0x140688e1f mov dword ptr [rax + rdx], 5
0x140688e26 mov qword ptr [rax + rdx + 0x30], rbx
0x140688e2b movss dword ptr [rax + rdx + 0x38], xmm9
0x140688e32 mov byte ptr [rax + rdx + 0x3c], r12b
0x140688e37 inc qword ptr [rbp + 0xb8]
0x140688e3e cmp rsi, r15
0x140688e41 je 0x140688c64
0x140688e47 mov rbx, qword ptr [rsi]
0x140688e4a movss xmm9, dword ptr [rdi - 0xc]
0x140688e50 addss xmm9, xmm10
0x140688e55 mov rdx, qword ptr [rbp + 0xb8]
0x140688e5c cmp rdx, qword ptr [rbp + 0xb0]
0x140688e63 jne 0x140688e10
0x140688e65 mov r8d, 1
0x140688e6b mov rcx, r13
0x140688e6e mov r9b, 1
0x140688e71 call 0x140fe08d0
0x140688e76 mov rdx, qword ptr [rbp + 0xb8]
0x140688e7d jmp 0x140688e10
0x140688e7f lea rcx, [rip + 0x238efd2]
0x140688e86 call 0x1428d9310
0x140688e8b jmp 0x14068adad
0x140688e90 test rax, rax
0x140688e93 jne 0x140688ea8
0x140688e95 xor edx, edx
0x140688e97 call 0x1428e6250
0x140688e9c mov rcx, rax
0x140688e9f test rax, rax
0x140688ea2 jne 0x140688732
0x140688ea8 lea rcx, [rip + 0x252c429]
0x140688eaf call 0x1428d80d0
0x140688eb4 mov rdi, qword ptr [rbp - 0x10]
0x140688eb8 mov rsi, qword ptr [rbp - 8]
0x140688ebc mov rax, qword ptr [rbp + 0xb0]
0x140688ec3 test rax, rax
0x140688ec6 je 0x140688eeb
0x140688ec8 mov rcx, qword ptr [rbp + 0xa8]
0x140688ecf mov rcx, qword ptr [rcx + 0x10]
0x140688ed3 mov rdx, qword ptr [rcx + 0x20]
0x140688ed7 cmp rdx, qword ptr [rbp + 0xa0]
0x140688ede jne 0x140688eeb
0x140688ee0 shl rax, 6
0x140688ee4 add rdx, rax
0x140688ee7 mov qword ptr [rcx + 0x20], rdx
0x140688eeb mov rax, qword ptr [rbp - 0x18]
0x140688eef mov qword ptr [rbp + 0xd0], rax
0x140688ef6 lea r14, [rbp + 8]
0x140688efa mov rcx, r14
0x140688efd call 0x1405e6620
0x140688f02 imul rsi, rsi, 0x38
0x140688f06 add rsi, rdi
0x140688f09 pcmpeqd xmm11, xmm11
0x140688f0e lea r12, [rbp + 0xa0]
0x140688f15 mov r15, rsi
0x140688f18 mov rbx, qword ptr [rbp + 0xf8]
0x140688f1f mov qword ptr [rbp + 0xf0], rdi
0x140688f26 cmp rdi, r15
0x140688f29 je 0x140689122
0x140688f2f cmp dword ptr [r15 - 0x30], 6
0x140688f34 je 0x140689122
0x140688f3a mov r13, qword ptr [r15 - 0x38]
0x140688f3e mov qword ptr [rbp + 8], r13
0x140688f42 cmp qword ptr [rbx + 0x1b8], 0
0x140688f4a je 0x140689abc
0x140688f50 add r15, -0x38
0x140688f54 mov rcx, qword ptr [rbp + 0xe8]
0x140688f5b mov rdx, r14
0x140688f5e call 0x1405b0d70
0x140688f63 mov rdx, qword ptr [rbx + 0x1a0]
0x140688f6a mov rcx, qword ptr [rbx + 0x1a8]
0x140688f71 mov r8, rcx
0x140688f74 and r8, rax
0x140688f77 shr rax, 0x39
0x140688f7b movd xmm0, eax
0x140688f7f punpcklbw xmm0, xmm0
0x140688f83 pshuflw xmm0, xmm0, 0
0x140688f88 pshufd xmm0, xmm0, 0
0x140688f8d lea rax, [rdx - 0xc0]
0x140688f94 xor r9d, r9d
0x140688f97 movdqu xmm1, xmmword ptr [rdx + r8]
0x140688f9d movdqa xmm2, xmm1
0x140688fa1 pcmpeqb xmm2, xmm0
0x140688fa5 pmovmskb r10d, xmm2
0x140688faa test r10d, r10d
0x140688fad je 0x140688fd8
0x140688faf tzcnt r11d, r10d
0x140688fb4 add r11, r8
0x140688fb7 and r11, rcx
0x140688fba neg r11
0x140688fbd lea r11, [r11 + r11*2]
0x140688fc1 shl r11, 6
0x140688fc5 cmp r13, qword ptr [rax + r11]
0x140688fc9 je 0x140688ffb
0x140688fcb lea r11d, [r10 - 1]
0x140688fcf and r11w, r10w
0x140688fd3 mov r10d, r11d
0x140688fd6 jne 0x140688faf
0x140688fd8 pcmpeqb xmm1, xmm11
0x140688fdd pmovmskb r10d, xmm1
0x140688fe2 test r10d, r10d
0x140688fe5 jne 0x140689abc
0x140688feb add r8, r9
0x140688fee add r8, 0x10
0x140688ff2 add r9, 0x10
0x140688ff6 and r8, rcx
0x140688ff9 jmp 0x140688f97
0x140688ffb lea rcx, [rdx + r11]
0x140688fff add rcx, -0xa0
0x140689006 movdqu xmm0, xmmword ptr [rdx + r11 - 0x58]
0x14068900d movdqa xmmword ptr [rbp + 0xa0], xmm0
0x140689015 mov rax, qword ptr [rdx + r11 - 0xb0]
0x14068901d mov rdx, qword ptr [rdx + r11 - 0xa8]
0x140689025 lea rdx, [rax + rdx*8]
0x140689029 mov qword ptr [rbp + 0x40], rax
0x14068902d mov qword ptr [rbp + 0x48], rdx
0x140689031 mov rax, qword ptr [rbp - 0x20]
0x140689035 mov qword ptr [rbp + 0x50], rax
0x140689039 mov rdx, r12
0x14068903c lea r8, [rbp + 0x40]
0x140689040 call 0x140fe0e10
0x140689045 cmp qword ptr [rbx + 0x1b8], 0
0x14068904d je 0x140689ad9
0x140689053 movdqa xmm8, xmm0
0x140689058 movdqa xmm9, xmm1
0x14068905d mov rcx, qword ptr [rbp + 0xe8]
0x140689064 mov rdx, r14
0x140689067 call 0x1405b0d70
0x14068906c mov rcx, qword ptr [rbx + 0x1a0]
0x140689073 mov rdx, qword ptr [rbx + 0x1a8]
0x14068907a mov r8, rdx
0x14068907d and r8, rax
0x140689080 shr rax, 0x39
0x140689084 movd xmm0, eax
0x140689088 punpcklbw xmm0, xmm0
0x14068908c pshuflw xmm0, xmm0, 0
0x140689091 pshufd xmm0, xmm0, 0
0x140689096 lea rax, [rcx - 0xc0]
0x14068909d xor r9d, r9d
0x1406890a0 movdqu xmm1, xmmword ptr [rcx + r8]
0x1406890a6 movdqa xmm2, xmm1
0x1406890aa pcmpeqb xmm2, xmm0
0x1406890ae pmovmskb r10d, xmm2
0x1406890b3 test r10d, r10d
0x1406890b6 je 0x1406890e1
0x1406890b8 tzcnt r11d, r10d
0x1406890bd add r11, r8
0x1406890c0 and r11, rdx
0x1406890c3 neg r11
0x1406890c6 lea r11, [r11 + r11*2]
0x1406890ca shl r11, 6
0x1406890ce cmp r13, qword ptr [rax + r11]
0x1406890d2 je 0x140689104
0x1406890d4 lea r11d, [r10 - 1]
0x1406890d8 and r11w, r10w
0x1406890dc mov r10d, r11d
0x1406890df jne 0x1406890b8
0x1406890e1 pcmpeqb xmm1, xmm11
0x1406890e6 pmovmskb r10d, xmm1
0x1406890eb test r10d, r10d
0x1406890ee jne 0x140689ad9
0x1406890f4 add r8, r9
0x1406890f7 add r8, 0x10
0x1406890fb add r9, 0x10
0x1406890ff and r8, rdx
0x140689102 jmp 0x1406890a0
0x140689104 movd dword ptr [rcx + r11 - 0x48], xmm8
0x14068910b movd dword ptr [rcx + r11 - 0x44], xmm9
0x140689112 mov rdi, qword ptr [rbp + 0xf0]
0x140689119 cmp rdi, r15
0x14068911c jne 0x140688f2f
0x140689122 mov qword ptr [rbp + 8], 0
0x14068912a mov qword ptr [rbp + 0x10], 8
0x140689132 mov qword ptr [rbp + 0x18], 0
0x14068913a lea rdx, [rip + 0x238f093]
0x140689141 lea r15, [rbp + 0x20]
0x140689145 pcmpeqd xmm9, xmm9
0x14068914a lea r12, [rbp + 0xa0]
0x140689151 mov r14, rdi
0x140689154 movaps xmm11, xmm10
0x140689158 mov qword ptr [rbp + 0xc8], rsi
0x14068915f cmp r14, rsi
0x140689162 je 0x140689b18
0x140689168 mov esi, dword ptr [r14 + 8]
0x14068916c cmp esi, 6
0x14068916f je 0x140689b18
0x140689175 mov r13, qword ptr [r14]
0x140689178 movss xmm15, dword ptr [r14 + 0xc]
0x14068917e mov eax, dword ptr [r14 + 0x18]
0x140689182 mov edi, dword ptr [r14 + 0x1c]
0x140689186 movsd xmm14, qword ptr [r14 + 0x10]
0x14068918c movsd xmm8, qword ptr [r14 + 0x20]
0x140689192 movsd xmm12, qword ptr [r14 + 0x28]
0x140689198 movsd xmm13, qword ptr [r14 + 0x30]
0x14068919e mov qword ptr [rbp + 0x20], r13
0x1406891a2 mov ecx, esi
0x1406891a4 movsxd rcx, dword ptr [rdx + rcx*4]
0x1406891a8 add rcx, rdx
0x1406891ab jmp rcx
0x1406891ad cmp qword ptr [rbx + 0x1b8], 0
0x1406891b5 je 0x140689b07
0x1406891bb mov rcx, qword ptr [rbp + 0xe8]
0x1406891c2 mov rdx, r15
0x1406891c5 call 0x1405b0d70
0x1406891ca mov rcx, qword ptr [rbx + 0x1a0]
0x1406891d1 mov rdx, qword ptr [rbx + 0x1a8]
0x1406891d8 mov r8, rdx
0x1406891db and r8, rax
0x1406891de shr rax, 0x39
0x1406891e2 movd xmm0, eax
0x1406891e6 punpcklbw xmm0, xmm0
0x1406891ea pshuflw xmm0, xmm0, 0
0x1406891ef pshufd xmm0, xmm0, 0
0x1406891f4 lea rax, [rcx - 0xc0]
0x1406891fb xor r9d, r9d
0x1406891fe movdqu xmm1, xmmword ptr [rcx + r8]
0x140689204 movdqa xmm2, xmm1
0x140689208 pcmpeqb xmm2, xmm0
0x14068920c pmovmskb r10d, xmm2
0x140689211 test r10d, r10d
0x140689214 je 0x14068923f
0x140689216 tzcnt r11d, r10d
0x14068921b add r11, r8
0x14068921e and r11, rdx
0x140689221 neg r11
0x140689224 lea r11, [r11 + r11*2]
0x140689228 shl r11, 6
0x14068922c cmp r13, qword ptr [rax + r11]
0x140689230 je 0x140689262
0x140689232 lea r11d, [r10 - 1]
0x140689236 and r11w, r10w
0x14068923a mov r10d, r11d
0x14068923d jne 0x140689216
0x14068923f pcmpeqb xmm1, xmm9
0x140689244 pmovmskb r10d, xmm1
0x140689249 test r10d, r10d
0x14068924c jne 0x140689b07
0x140689252 add r8, r9
0x140689255 add r8, 0x10
0x140689259 add r9, 0x10
0x14068925d and r8, rdx
0x140689260 jmp 0x1406891fe
0x140689262 movss xmm0, dword ptr [rip + 0x238df96]
0x14068926a cmp esi, 1
0x14068926d je 0x140689280
0x14068926f cmp esi, 2
0x140689272 jne 0x14068ad29
0x140689278 movss xmm0, dword ptr [rip + 0x238df84]
0x140689280 lea rdx, [rcx + r11]
0x140689284 lea rax, [rcx + r11]
0x140689288 add rax, -0xb8
0x14068928e movaps xmm1, xmm7
0x140689291 cmpunordss xmm1, xmm7
0x140689296 movaps xmm2, xmm1
0x140689299 andps xmm2, xmm0
0x14068929c minss xmm0, xmm7
0x1406892a0 andnps xmm1, xmm0
0x1406892a3 orps xmm1, xmm2
0x1406892a6 movaps xmm0, xmm7
0x1406892a9 subss xmm0, xmm1
0x1406892ad mulss xmm0, dword ptr [rip + 0x229c5bb]
0x1406892b5 movss dword ptr [rdx - 0x40], xmm0
0x1406892ba mov dword ptr [rdx - 0x3c], 0
0x1406892c1 movss dword ptr [rdx - 0x48], xmm1
0x1406892c6 mov dword ptr [rdx - 0x44], 0x447a0000
0x1406892cd jmp 0x14068973e
0x1406892d2 cmp qword ptr [rbx + 0x1b8], 0
0x1406892da je 0x140689ca5
0x1406892e0 mov rcx, qword ptr [rbp + 0xe8]
0x1406892e7 mov rdx, r15
0x1406892ea call 0x1405b0d70
0x1406892ef mov rcx, qword ptr [rbx + 0x1a0]
0x1406892f6 mov rdx, qword ptr [rbx + 0x1a8]
0x1406892fd mov r8, rdx
0x140689300 and r8, rax
0x140689303 shr rax, 0x39
0x140689307 movd xmm0, eax
0x14068930b punpcklbw xmm0, xmm0
0x14068930f pshuflw xmm0, xmm0, 0
0x140689314 pshufd xmm0, xmm0, 0
0x140689319 lea rax, [rcx - 0xc0]
0x140689320 xor r9d, r9d
0x140689323 movdqu xmm1, xmmword ptr [rcx + r8]
0x140689329 movdqa xmm2, xmm1
0x14068932d pcmpeqb xmm2, xmm0
0x140689331 pmovmskb r10d, xmm2
0x140689336 test r10d, r10d
0x140689339 je 0x140689368
0x14068933b tzcnt r11d, r10d
0x140689340 add r11, r8
0x140689343 and r11, rdx
0x140689346 neg r11
0x140689349 lea r11, [r11 + r11*2]
0x14068934d shl r11, 6
0x140689351 cmp r13, qword ptr [rax + r11]
0x140689355 je 0x1406895ce
0x14068935b lea r11d, [r10 - 1]
0x14068935f and r11w, r10w
0x140689363 mov r10d, r11d
0x140689366 jne 0x14068933b
0x140689368 pcmpeqb xmm1, xmm9
0x14068936d pmovmskb r10d, xmm1
0x140689372 test r10d, r10d
0x140689375 jne 0x140689ca5
0x14068937b add r8, r9
0x14068937e add r8, 0x10
0x140689382 add r9, 0x10
0x140689386 and r8, rdx
0x140689389 jmp 0x140689323
0x14068938b mov dword ptr [rbp + 0x40], eax
0x14068938e mov dword ptr [rbp + 0x44], edi
0x140689391 cmp qword ptr [rbx + 0x1b8], 0
0x140689399 je 0x140689cb6
0x14068939f movss dword ptr [rbp + 0xc0], xmm15
0x1406893a8 mov rcx, qword ptr [rbp + 0xe8]
0x1406893af lea rdx, [rbp + 0x40]
0x1406893b3 call 0x1405b0d70
0x1406893b8 mov r15, qword ptr [rbx + 0x1a0]
0x1406893bf mov rsi, qword ptr [rbx + 0x1a8]
0x1406893c6 mov rcx, rsi
0x1406893c9 and rcx, rax
0x1406893cc shr rax, 0x39
0x1406893d0 movd xmm0, eax
0x1406893d4 punpcklbw xmm0, xmm0
0x1406893d8 pshuflw xmm0, xmm0, 0
0x1406893dd pshufd xmm0, xmm0, 0
0x1406893e2 lea rdi, [r15 - 0xc0]
0x1406893e9 mov rax, qword ptr [rbp + 0x40]
0x1406893ed xor edx, edx
0x1406893ef movdqu xmm1, xmmword ptr [r15 + rcx]
0x1406893f5 movdqa xmm2, xmm1
0x1406893f9 pcmpeqb xmm2, xmm0
0x1406893fd pmovmskb r8d, xmm2
0x140689402 test r8d, r8d
0x140689405 je 0x140689434
0x140689407 tzcnt r9d, r8d
0x14068940c add r9, rcx
0x14068940f and r9, rsi
0x140689412 neg r9
0x140689415 lea r9, [r9 + r9*2]
0x140689419 shl r9, 6
0x14068941d cmp rax, qword ptr [rdi + r9]
0x140689421 je 0x1406895fc
0x140689427 lea r9d, [r8 - 1]
0x14068942b and r9w, r8w
0x14068942f mov r8d, r9d
0x140689432 jne 0x140689407
0x140689434 pcmpeqb xmm1, xmm9
0x140689439 pmovmskb r8d, xmm1
0x14068943e test r8d, r8d
0x140689441 jne 0x140689cb6
0x140689447 add rcx, rdx
0x14068944a add rcx, 0x10
0x14068944e add rdx, 0x10
0x140689452 and rcx, rsi
0x140689455 jmp 0x1406893ef
0x140689457 cmp qword ptr [rbx + 0x1b8], 0
0x14068945f je 0x140689cc7
0x140689465 movd xmm11, eax
0x14068946a mov rcx, qword ptr [rbp + 0xe8]
0x140689471 mov rdx, r15
0x140689474 call 0x1405b0d70
0x140689479 mov rcx, qword ptr [rbx + 0x1a0]
0x140689480 mov rdx, qword ptr [rbx + 0x1a8]
0x140689487 mov r8, rdx
0x14068948a and r8, rax
0x14068948d shr rax, 0x39
0x140689491 movd xmm0, eax
0x140689495 punpcklbw xmm0, xmm0
0x140689499 pshuflw xmm0, xmm0, 0
0x14068949e pshufd xmm0, xmm0, 0
0x1406894a3 lea rax, [rcx - 0xc0]
0x1406894aa xor r9d, r9d
0x1406894ad movdqu xmm1, xmmword ptr [rcx + r8]
0x1406894b3 movdqa xmm2, xmm1
0x1406894b7 pcmpeqb xmm2, xmm0
0x1406894bb pmovmskb r10d, xmm2
0x1406894c0 test r10d, r10d
0x1406894c3 je 0x1406894f2
0x1406894c5 tzcnt r11d, r10d
0x1406894ca add r11, r8
0x1406894cd and r11, rdx
0x1406894d0 neg r11
0x1406894d3 lea r11, [r11 + r11*2]
0x1406894d7 shl r11, 6
0x1406894db cmp r13, qword ptr [rax + r11]
0x1406894df je 0x1406896ad
0x1406894e5 lea r11d, [r10 - 1]
0x1406894e9 and r11w, r10w
0x1406894ed mov r10d, r11d
0x1406894f0 jne 0x1406894c5
0x1406894f2 pcmpeqb xmm1, xmm9
0x1406894f7 pmovmskb r10d, xmm1
0x1406894fc test r10d, r10d
0x1406894ff jne 0x140689cc7
0x140689505 add r8, r9
0x140689508 add r8, 0x10
0x14068950c add r9, 0x10
0x140689510 and r8, rdx
0x140689513 jmp 0x1406894ad
0x140689515 cmp qword ptr [rbx + 0x1b8], 0
0x14068951d je 0x140689cd8
0x140689523 mov rcx, qword ptr [rbp + 0xe8]
0x14068952a mov rdx, r15
0x14068952d call 0x1405b0d70
0x140689532 mov rcx, qword ptr [rbx + 0x1a0]
0x140689539 mov rdx, qword ptr [rbx + 0x1a8]
0x140689540 mov r8, rdx
0x140689543 and r8, rax
0x140689546 shr rax, 0x39
0x14068954a movd xmm0, eax
0x14068954e punpcklbw xmm0, xmm0
0x140689552 pshuflw xmm0, xmm0, 0
0x140689557 pshufd xmm0, xmm0, 0
0x14068955c lea rax, [rcx - 0xc0]
0x140689563 xor r9d, r9d
0x140689566 movdqu xmm1, xmmword ptr [rcx + r8]
0x14068956c movdqa xmm2, xmm1
0x140689570 pcmpeqb xmm2, xmm0
0x140689574 pmovmskb r10d, xmm2
0x140689579 test r10d, r10d
0x14068957c je 0x1406895ab
0x14068957e tzcnt r11d, r10d
0x140689583 add r11, r8
0x140689586 and r11, rdx
0x140689589 neg r11
0x14068958c lea r11, [r11 + r11*2]
0x140689590 shl r11, 6
0x140689594 cmp r13, qword ptr [rax + r11]
0x140689598 je 0x1406896bd
0x14068959e lea r11d, [r10 - 1]
0x1406895a2 and r11w, r10w
0x1406895a6 mov r10d, r11d
0x1406895a9 jne 0x14068957e
0x1406895ab pcmpeqb xmm1, xmm9
0x1406895b0 pmovmskb r10d, xmm1
0x1406895b5 test r10d, r10d
0x1406895b8 jne 0x140689cd8
0x1406895be add r8, r9
0x1406895c1 add r8, 0x10
0x1406895c5 add r9, 0x10
0x1406895c9 and r8, rdx
0x1406895cc jmp 0x140689566
0x1406895ce lea rax, [rcx + r11]
0x1406895d2 add rax, -0xb8
0x1406895d8 mov dword ptr [rcx + r11 - 0x3c], 0
0x1406895e1 movss dword ptr [rcx + r11 - 0x48], xmm7
0x1406895e8 movd xmm0, dword ptr [rip + 0x22d2040]
0x1406895f0 movq qword ptr [rcx + r11 - 0x44], xmm0
0x1406895f7 jmp 0x14068973e
0x1406895fc addps xmm12, xmmword ptr [rip + 0x22a50ac]
0x140689604 movsd xmm0, qword ptr [r15 + r9 - 0x48]
0x14068960b movsd xmm1, qword ptr [r15 + r9 - 0x40]
0x140689612 mulps xmm12, xmmword ptr [rip + 0x229c2c6]
0x14068961a mulps xmm12, xmm0
0x14068961e addps xmm12, xmm1
0x140689622 mov rcx, qword ptr [rbp + 0xe8]
0x140689629 lea rdx, [rbp + 0x20]
0x14068962d call 0x1405b0d70
0x140689632 mov rcx, rax
0x140689635 shr rcx, 0x39
0x140689639 movd xmm0, ecx
0x14068963d punpcklbw xmm0, xmm0
0x140689641 pshuflw xmm0, xmm0, 0
0x140689646 pshufd xmm0, xmm0, 0
0x14068964b xor ecx, ecx
0x14068964d and rax, rsi
0x140689650 movdqu xmm1, xmmword ptr [r15 + rax]
0x140689656 movdqa xmm2, xmm1
0x14068965a pcmpeqb xmm2, xmm0
0x14068965e pmovmskb edx, xmm2
0x140689662 test edx, edx
0x140689664 je 0x14068968f
0x140689666 tzcnt r8d, edx
0x14068966b add r8, rax
0x14068966e and r8, rsi
0x140689671 neg r8
0x140689674 lea r8, [r8 + r8*2]
0x140689678 shl r8, 6
0x14068967c cmp r13, qword ptr [rdi + r8]
0x140689680 je 0x1406896e8
0x140689682 lea r8d, [rdx - 1]
0x140689686 and r8w, dx
0x14068968a mov edx, r8d
0x14068968d jne 0x140689666
0x14068968f pcmpeqb xmm1, xmm9
0x140689694 pmovmskb edx, xmm1
0x140689698 test edx, edx
0x14068969a jne 0x140689ce9
0x1406896a0 add rax, rcx
0x1406896a3 add rax, 0x10
0x1406896a7 add rcx, 0x10
0x1406896ab jmp 0x14068964d
0x1406896ad lea rdx, [rcx + r11]
0x1406896b1 cmp edi, 1
0x1406896b4 jne 0x140689703
0x1406896b6 movlps qword ptr [rdx - 0x48], xmm8
0x1406896bb jmp 0x140689709
0x1406896bd lea rax, [rcx + r11]
0x1406896c1 add rax, -0xb8
0x1406896c7 mulss xmm11, dword ptr [rax + 0xb0]
0x1406896d0 movss dword ptr [rax + 0xa4], xmm11
0x1406896d9 cmp qword ptr [rbx + 0x1b8], 0
0x1406896e1 jne 0x140689762
0x1406896e3 jmp 0x140689a9f
0x1406896e8 lea rcx, [r15 + r8]
0x1406896ec cmp dword ptr [rbp + 0xc0], 1
0x1406896f3 jne 0x140689a3f
0x1406896f9 movlps qword ptr [rcx - 0x48], xmm14
0x1406896fe jmp 0x140689a45
0x140689703 movsd xmm8, qword ptr [rdx - 0x48]
0x140689709 lea rax, [rcx + r11]
0x14068970d add rax, -0xb8
0x140689713 shufps xmm11, xmm14, 0xd4
0x140689718 shufps xmm11, xmm14, 0xe2
0x14068971d addps xmm11, xmmword ptr [rip + 0x22a4f8b]
0x140689725 mulps xmm11, xmmword ptr [rip + 0x238dac3]
0x14068972d mulps xmm11, xmm8
0x140689731 unpcklps xmm15, xmm14
0x140689735 addps xmm15, xmm11
0x140689739 movlps qword ptr [rdx - 0x40], xmm15
0x14068973e movaps xmm11, xmm10
0x140689742 mulss xmm11, dword ptr [rax + 0xb0]
0x14068974b movss dword ptr [rax + 0xa4], xmm11
0x140689754 cmp qword ptr [rbx + 0x1b8], 0
0x14068975c je 0x140689a9f
0x140689762 add r14, 0x38
0x140689766 mov rcx, qword ptr [rbp + 0xe8]
0x14068976d mov rdx, r15
0x140689770 call 0x1405b0d70
0x140689775 mov rsi, qword ptr [rbx + 0x1a0]
0x14068977c mov rcx, qword ptr [rbx + 0x1a8]
0x140689783 mov rdx, rcx
0x140689786 and rdx, rax
0x140689789 shr rax, 0x39
0x14068978d movd xmm0, eax
0x140689791 punpcklbw xmm0, xmm0
0x140689795 pshuflw xmm0, xmm0, 0
0x14068979a pshufd xmm0, xmm0, 0
0x14068979f lea rax, [rsi - 0xc0]
0x1406897a6 xor r8d, r8d
0x1406897a9 movdqu xmm1, xmmword ptr [rsi + rdx]
0x1406897ae movdqa xmm2, xmm1
0x1406897b2 pcmpeqb xmm2, xmm0
0x1406897b6 pmovmskb r9d, xmm2
0x1406897bb test r9d, r9d
0x1406897be je 0x1406897e9
0x1406897c0 tzcnt r10d, r9d
0x1406897c5 add r10, rdx
0x1406897c8 and r10, rcx
0x1406897cb neg r10
0x1406897ce lea rdi, [r10 + r10*2]
0x1406897d2 shl rdi, 6
0x1406897d6 cmp r13, qword ptr [rax + rdi]
0x1406897da je 0x14068980c
0x1406897dc lea r10d, [r9 - 1]
0x1406897e0 and r10w, r9w
0x1406897e4 mov r9d, r10d
0x1406897e7 jne 0x1406897c0
0x1406897e9 pcmpeqb xmm1, xmm9
0x1406897ee pmovmskb r9d, xmm1
0x1406897f3 test r9d, r9d
0x1406897f6 jne 0x140689a9f
0x1406897fc add rdx, r8
0x1406897ff add rdx, 0x10
0x140689803 add r8, 0x10
0x140689807 and rdx, rcx
0x14068980a jmp 0x1406897a9
0x14068980c mov rax, qword ptr [rsi + rdi - 0xb0]
0x140689814 mov rcx, qword ptr [rsi + rdi - 0xa8]
0x14068981c lea rcx, [rax + rcx*8]
0x140689820 mov qword ptr [rbp + 0x40], rax
0x140689824 mov qword ptr [rbp + 0x48], rcx
0x140689828 mov rax, qword ptr [rbp - 0x20]
0x14068982c mov qword ptr [rbp + 0x50], rax
0x140689830 mov rcx, r12
0x140689833 lea r15, [rbp + 0x40]
0x140689837 mov rdx, r15
0x14068983a lea r8, [rip + 0x238dae7]
0x140689841 call 0x140fcea60
0x140689846 add rsi, rdi
0x140689849 mov r9, qword ptr [rbp + 0xa8]
0x140689850 mov rax, qword ptr [rbp + 0xb0]
0x140689857 lea r8, [rsi - 0x48]
0x14068985b lea rdx, [rsi - 0xa0]
0x140689862 mov qword ptr [rsp + 0x20], rax
0x140689867 mov rcx, r15
0x14068986a mov qword ptr [rbp + 0xd8], r9
0x140689871 call 0x14068ea20
0x140689876 mov qword ptr [rbp + 0x18], 0
0x14068987e mov rdx, qword ptr [rsi - 0xb0]
0x140689885 mov rax, qword ptr [rsi - 0xa8]
0x14068988c lea r8, [rdx + rax*8]
0x140689890 mov byte ptr [rbp + 0xe7], 1
0x140689897 lea rcx, [rbp + 8]
0x14068989b lea r9, [rip + 0x238e53e]
0x1406898a2 call 0x140fce8f0
0x1406898a7 mov rax, qword ptr [rbp + 0x18]
0x1406898ab mov rcx, qword ptr [rbp + 0x40]
0x1406898af mov qword ptr [rbp + 0x98], rcx
0x1406898b6 mov rcx, qword ptr [rbp + 0x48]
0x1406898ba mov qword ptr [rbp + 0xc0], rcx
0x1406898c1 test rax, rax
0x1406898c4 je 0x1406899d0
0x1406898ca mov rsi, qword ptr [rbp + 0x10]
0x1406898ce lea rax, [rsi + rax*8]
0x1406898d2 mov qword ptr [rbp + 0x38], rax
0x1406898d6 mov r15, qword ptr [rbp + 0x50]
0x1406898da shl r15, 4
0x1406898de mov r12, qword ptr [rbp + 0xc0]
0x1406898e5 add r15, r12
0x1406898e8 jmp 0x140689964
0x1406898ea movdqu xmm1, xmmword ptr [rcx + r8]
0x1406898f0 movdqa xmm2, xmm1
0x1406898f4 pcmpeqb xmm2, xmm0
0x1406898f8 pmovmskb r10d, xmm2
0x1406898fd test r10d, r10d
0x140689900 je 0x14068992b
0x140689902 tzcnt r11d, r10d
0x140689907 add r11, r8
0x14068990a and r11, rdx
0x14068990d neg r11
0x140689910 lea r11, [r11 + r11*2]
0x140689914 shl r11, 6
0x140689918 cmp r13, qword ptr [rax + r11]
0x14068991c je 0x14068994e
0x14068991e lea r11d, [r10 - 1]
0x140689922 and r11w, r10w
0x140689926 mov r10d, r11d
0x140689929 jne 0x140689902
0x14068992b pcmpeqb xmm1, xmm9
0x140689930 pmovmskb r10d, xmm1
0x140689935 test r10d, r10d
0x140689938 jne 0x140689aea
0x14068993e add r8, r9
0x140689941 add r8, 0x10
0x140689945 add r9, 0x10
0x140689949 and r8, rdx
0x14068994c jmp 0x1406898ea
0x14068994e movdqu xmm0, xmmword ptr [r12]
0x140689954 movdqu xmmword ptr [rcx + r11 - 0x48], xmm0
0x14068995b mov r12, rdi
0x14068995e cmp rsi, qword ptr [rbp + 0x38]
0x140689962 je 0x1406899d0
0x140689964 cmp r12, r15
0x140689967 je 0x1406899d0
0x140689969 mov r13, qword ptr [rsi]
0x14068996c mov qword ptr [rbp - 0x18], r13
0x140689970 cmp qword ptr [rbx + 0x1b8], 0
0x140689978 je 0x140689aea
0x14068997e add rsi, 8
0x140689982 lea rdi, [r12 + 0x10]
0x140689987 mov rcx, qword ptr [rbp + 0xe8]
0x14068998e lea rdx, [rbp - 0x18]
0x140689992 call 0x1405b0d70
0x140689997 mov rcx, qword ptr [rbx + 0x1a0]
0x14068999e mov rdx, qword ptr [rbx + 0x1a8]
0x1406899a5 mov r8, rdx
0x1406899a8 and r8, rax
0x1406899ab shr rax, 0x39
0x1406899af movd xmm0, eax
0x1406899b3 punpcklbw xmm0, xmm0
0x1406899b7 pshuflw xmm0, xmm0, 0
0x1406899bc pshufd xmm0, xmm0, 0
0x1406899c1 lea rax, [rcx - 0xc0]
0x1406899c8 xor r9d, r9d
0x1406899cb jmp 0x1406898ea
0x1406899d0 mov rdx, qword ptr [rbp + 0x98]
0x1406899d7 test rdx, rdx
0x1406899da je 0x1406899f2
0x1406899dc shl rdx, 4
0x1406899e0 mov r8d, 4
0x1406899e6 mov rcx, qword ptr [rbp + 0xc0]
0x1406899ed call 0x140613c20
0x1406899f2 mov rax, qword ptr [rbp + 0xa0]
0x1406899f9 test rax, rax
0x1406899fc lea r12, [rbp + 0xa0]
0x140689a03 lea r15, [rbp + 0x20]
0x140689a07 mov rsi, qword ptr [rbp + 0xc8]
0x140689a0e je 0x140689a2a
0x140689a10 shl rax, 3
0x140689a14 lea rdx, [rax + rax*2]
0x140689a18 mov r8d, 8
0x140689a1e mov rcx, qword ptr [rbp + 0xd8]
0x140689a25 call 0x140613c20
0x140689a2a lea rdx, [rip + 0x238e7a3]
0x140689a31 cmp r14, rsi
0x140689a34 jne 0x140689168
0x140689a3a jmp 0x140689b18
0x140689a3f movsd xmm14, qword ptr [rcx - 0x48]
0x140689a45 lea rax, [r15 + r8]
0x140689a49 add rax, -0xb8
0x140689a4f addps xmm8, xmm12
0x140689a53 addps xmm13, xmmword ptr [rip + 0x22a4c55]
0x140689a5b mulps xmm13, xmmword ptr [rip + 0x238d78d]
0x140689a63 mulps xmm13, xmm14
0x140689a67 addps xmm13, xmm8
0x140689a6b movlps qword ptr [rcx - 0x40], xmm13
0x140689a70 movaps xmm11, xmm10
0x140689a74 mov rbx, qword ptr [rbp + 0xf8]
0x140689a7b lea r15, [rbp + 0x20]
0x140689a7f mulss xmm11, dword ptr [rax + 0xb0]
0x140689a88 movss dword ptr [rax + 0xa4], xmm11
0x140689a91 cmp qword ptr [rbx + 0x1b8], 0
0x140689a99 jne 0x140689762
0x140689a9f lea rcx, [rip + 0x238d762]
0x140689aa6 lea r8, [rip + 0x238e31b]
0x140689aad mov edx, 0x16
0x140689ab2 call 0x1428d9330
0x140689ab7 jmp 0x14068adad
0x140689abc lea rcx, [rip + 0x238d745]
0x140689ac3 lea r8, [rip + 0x238e346]
0x140689aca mov edx, 0x16
0x140689acf call 0x1428d9330
0x140689ad4 jmp 0x14068adad
0x140689ad9 lea rcx, [rip + 0x238e348]
0x140689ae0 call 0x1428d9310
0x140689ae5 jmp 0x14068adad
0x140689aea lea rcx, [rip + 0x238e307]
0x140689af1 call 0x1428d9310
0x140689af6 jmp 0x14068adad
0x140689afb lea rcx, [rip + 0x252b846]
0x140689b02 call 0x1428d9250
0x140689b07 lea rcx, [rip + 0x238e202]
0x140689b0e call 0x1428d9310
0x140689b13 jmp 0x14068adad
0x140689b18 mov rdx, qword ptr [rbp + 8]
0x140689b1c test rdx, rdx
0x140689b1f je 0x140689b34
0x140689b21 mov rcx, qword ptr [rbp + 0x10]
0x140689b25 shl rdx, 3
0x140689b29 mov r8d, 8
0x140689b2f call 0x140613c20
0x140689b34 mov rax, qword ptr [rbp + 0xd0]
0x140689b3b test rax, rax
0x140689b3e je 0x140689b56
0x140689b40 imul rdx, rax, 0x38
0x140689b44 mov r8d, 8
0x140689b4a mov rcx, qword ptr [rbp + 0xf0]
0x140689b51 call 0x140613c20
0x140689b56 mov r8, qword ptr [rbp + 0xf8]
0x140689b5d mov rax, qword ptr [r8 + 0x1a0]
0x140689b64 mov rcx, qword ptr [r8 + 0x1a8]
0x140689b6b add rcx, rax
0x140689b6e inc rcx
0x140689b71 movdqa xmm0, xmmword ptr [rax]
0x140689b75 pmovmskb edx, xmm0
0x140689b79 not edx
0x140689b7b mov word ptr [rbp + 0x58], dx
0x140689b7f mov qword ptr [rbp + 0x40], rax
0x140689b83 add rax, 0x10
0x140689b87 mov qword ptr [rbp + 0x48], rax
0x140689b8b mov qword ptr [rbp + 0x50], rcx
0x140689b8f mov rax, qword ptr [r8 + 0x1b8]
0x140689b96 mov qword ptr [rbp + 0x60], rax
0x140689b9a lea r14, [rbp + 0x40]
0x140689b9e mov rcx, r14
0x140689ba1 call 0x140fb1730
0x140689ba6 test rax, rax
0x140689ba9 je 0x140689bb9
0x140689bab test rdx, rdx
0x140689bae je 0x140689bb9
0x140689bb0 and byte ptr [rdx + 0xb6], 0xbf
0x140689bb7 jmp 0x140689b9e
0x140689bb9 mov rax, qword ptr [rbp + 0x30]
0x140689bbd lea rcx, [rax + 0x10]
0x140689bc1 mov qword ptr [rbp + 0xd0], rcx
0x140689bc8 cmp dword ptr [rax + 0x10], 2
0x140689bcc jne 0x140689d08
0x140689bd2 mov rax, qword ptr [rbp + 0x30]
0x140689bd6 mov rsi, qword ptr [rax + 0x18]
0x140689bda mov qword ptr [rbp + 0x40], rsi
0x140689bde mov rax, qword ptr [rbp + 0xf8]
0x140689be5 cmp qword ptr [rax + 0x1b8], 0
0x140689bed je 0x140689cfa
0x140689bf3 lea rdx, [rbp + 0x40]
0x140689bf7 mov rcx, qword ptr [rbp + 0xe8]
0x140689bfe call 0x1405b0d70
0x140689c03 mov rdx, qword ptr [rbp + 0xf8]
0x140689c0a mov rcx, qword ptr [rdx + 0x1a0]
0x140689c11 mov rdx, qword ptr [rdx + 0x1a8]
0x140689c18 mov r8, rdx
0x140689c1b and r8, rax
0x140689c1e shr rax, 0x39
0x140689c22 movd xmm0, eax
0x140689c26 punpcklbw xmm0, xmm0
0x140689c2a pshuflw xmm0, xmm0, 0
0x140689c2f pshufd xmm0, xmm0, 0
0x140689c34 lea rax, [rcx - 0xc0]
0x140689c3b xor r9d, r9d
0x140689c3e pcmpeqd xmm1, xmm1
0x140689c42 movdqu xmm2, xmmword ptr [rcx + r8]
0x140689c48 movdqa xmm3, xmm2
0x140689c4c pcmpeqb xmm3, xmm0
0x140689c50 pmovmskb r10d, xmm3
0x140689c55 test r10d, r10d
0x140689c58 je 0x140689c87
0x140689c5a tzcnt r11d, r10d
0x140689c5f add r11, r8
0x140689c62 and r11, rdx
0x140689c65 neg r11
0x140689c68 lea r11, [r11 + r11*2]
0x140689c6c shl r11, 6
0x140689c70 cmp rsi, qword ptr [rax + r11]
0x140689c74 je 0x140689d08
0x140689c7a lea r11d, [r10 - 1]
0x140689c7e and r11w, r10w
0x140689c82 mov r10d, r11d
0x140689c85 jne 0x140689c5a
0x140689c87 pcmpeqb xmm2, xmm1
0x140689c8b pmovmskb r10d, xmm2
0x140689c90 test r10d, r10d
0x140689c93 jne 0x140689cfa
0x140689c95 add r8, r9
0x140689c98 add r8, 0x10
0x140689c9c add r9, 0x10
0x140689ca0 and r8, rdx
0x140689ca3 jmp 0x140689c42
0x140689ca5 lea rcx, [rip + 0x238e04c]
0x140689cac call 0x1428d9310
0x140689cb1 jmp 0x14068adad
0x140689cb6 lea rcx, [rip + 0x238e0ab]
0x140689cbd call 0x1428d9310
0x140689cc2 jmp 0x14068adad
0x140689cc7 lea rcx, [rip + 0x238e0ca]
0x140689cce call 0x1428d9310
0x140689cd3 jmp 0x14068adad
0x140689cd8 lea rcx, [rip + 0x238e0d1]
0x140689cdf call 0x1428d9310
0x140689ce4 jmp 0x14068adad
0x140689ce9 lea rcx, [rip + 0x238e090]
0x140689cf0 call 0x1428d9310
0x140689cf5 jmp 0x14068adad
0x140689cfa mov rax, qword ptr [rbp + 0xd0]
0x140689d01 mov qword ptr [rax], 0
0x140689d08 mov rcx, qword ptr [rbp + 0xd0]
0x140689d0f call 0x1409a9150
0x140689d14 test al, al
0x140689d16 je 0x140689e04
0x140689d1c mov rcx, qword ptr [rbp + 0xd0]
0x140689d23 call 0x1409a9130
0x140689d28 cmp rax, 1
0x140689d2c jne 0x140689e5f
0x140689d32 mov r14, rdx
0x140689d35 mov qword ptr [rbp + 0x40], rdx
0x140689d39 mov rax, qword ptr [rbp + 0xf8]
0x140689d40 cmp qword ptr [rax + 0x1b8], 0
0x140689d48 je 0x14068a9d7
0x140689d4e lea rdx, [rbp + 0x40]
0x140689d52 mov rcx, qword ptr [rbp + 0xe8]
0x140689d59 call 0x1405b0d70
0x140689d5e mov rdx, qword ptr [rbp + 0xf8]
0x140689d65 mov rcx, qword ptr [rdx + 0x1a0]
0x140689d6c mov rdx, qword ptr [rdx + 0x1a8]
0x140689d73 mov r8, rdx
0x140689d76 and r8, rax
0x140689d79 shr rax, 0x39
0x140689d7d movd xmm0, eax
0x140689d81 punpcklbw xmm0, xmm0
0x140689d85 pshuflw xmm0, xmm0, 0
0x140689d8a pshufd xmm0, xmm0, 0
0x140689d8f lea rax, [rcx - 0xc0]
0x140689d96 xor r9d, r9d
0x140689d99 pcmpeqd xmm1, xmm1
0x140689d9d movdqu xmm2, xmmword ptr [rcx + r8]
0x140689da3 movdqa xmm3, xmm2
0x140689da7 pcmpeqb xmm3, xmm0
0x140689dab pmovmskb r10d, xmm3
0x140689db0 test r10d, r10d
0x140689db3 je 0x140689de2
0x140689db5 tzcnt r11d, r10d
0x140689dba add r11, r8
0x140689dbd and r11, rdx
0x140689dc0 neg r11
0x140689dc3 lea r11, [r11 + r11*2]
0x140689dc7 shl r11, 6
0x140689dcb cmp r14, qword ptr [rax + r11]
0x140689dcf je 0x140689e59
0x140689dd5 lea r11d, [r10 - 1]
0x140689dd9 and r11w, r10w
0x140689ddd mov r10d, r11d
0x140689de0 jne 0x140689db5
0x140689de2 pcmpeqb xmm2, xmm1
0x140689de6 pmovmskb r10d, xmm2
0x140689deb test r10d, r10d
0x140689dee jne 0x14068a9d7
0x140689df4 add r8, r9
0x140689df7 add r8, 0x10
0x140689dfb add r9, 0x10
0x140689dff and r8, rdx
0x140689e02 jmp 0x140689d9d
0x140689e04 mov rax, qword ptr [rbp + 0xf8]
0x140689e0b mov ecx, dword ptr [rax + 0xd0]
0x140689e11 movss xmm1, dword ptr [rax + 0xd4]
0x140689e19 call 0x14093fa90
0x140689e1e test al, al
0x140689e20 je 0x14068a0bf
0x140689e26 mov rax, qword ptr [rbp + 0xf8]
0x140689e2d mov ecx, dword ptr [rax + 0xd8]
0x140689e33 movss xmm1, dword ptr [rax + 0xdc]
0x140689e3b call 0x14093fa90
0x140689e40 mov dword ptr [rbp + 0xd8], eax
0x140689e46 mov eax, dword ptr [rbp + 0xd8]
0x140689e4c xor al, 1
0x140689e4e mov dword ptr [rbp + 0xd8], eax
0x140689e54 jmp 0x14068a0c9
0x140689e59 or byte ptr [rcx + r11 - 2], 0x40
0x140689e5f mov r8, qword ptr [rbp + 0xf8]
0x140689e66 mov rax, qword ptr [r8 + 0x1a0]
0x140689e6d mov rcx, qword ptr [r8 + 0x1a8]
0x140689e74 add rcx, rax
0x140689e77 inc rcx
0x140689e7a movdqa xmm0, xmmword ptr [rax]
0x140689e7e pmovmskb edx, xmm0
0x140689e82 not edx
0x140689e84 mov word ptr [rbp + 0x58], dx
0x140689e88 mov qword ptr [rbp + 0x40], rax
0x140689e8c add rax, 0x10
0x140689e90 mov qword ptr [rbp + 0x48], rax
0x140689e94 mov qword ptr [rbp + 0x50], rcx
0x140689e98 mov rax, qword ptr [r8 + 0x1b8]
0x140689e9f mov qword ptr [rbp + 0x60], rax
0x140689ea3 lea r8, [rip + 0x238d47e]
0x140689eaa lea rcx, [rbp + 8]
0x140689eae lea rdx, [rbp + 0x40]
0x140689eb2 call 0x140fcebb0
0x140689eb7 mov rax, qword ptr [rbp + 0x10]
0x140689ebb mov qword ptr [rbp + 0xf0], rax
0x140689ec2 mov r14, qword ptr [rbp + 0x18]
0x140689ec6 lea rax, [rbp + 0xa0]
0x140689ecd mov qword ptr [rbp + 0x40], rax
0x140689ed1 cmp r14, 2
0x140689ed5 jae 0x14068acef
0x140689edb mov rax, qword ptr [rbp + 8]
0x140689edf mov qword ptr [rbp + 0xc8], rax
0x140689ee6 test r14, r14
0x140689ee9 je 0x140689f8b
0x140689eef lea rsi, [rbp + 0x60]
0x140689ef3 shl r14, 3
0x140689ef7 xor edi, edi
0x140689ef9 lea r15, [rbp + 0x40]
0x140689efd movaps xmm7, xmmword ptr [rip + 0x229b9dc]
0x140689f04 jmp 0x140689f0f
0x140689f06 add rdi, 8
0x140689f0a cmp r14, rdi
0x140689f0d je 0x140689f8b
0x140689f0f mov rax, qword ptr [rbp + 0xf0]
0x140689f16 mov rax, qword ptr [rax + rdi]
0x140689f1a mov rcx, qword ptr [rax + 0x80]
0x140689f21 test rcx, rcx
0x140689f24 je 0x140689f06
0x140689f26 mov r8, qword ptr [rax + 0x88]
0x140689f2d movd xmm0, dword ptr [rax + 0xa4]
0x140689f35 movsd xmm1, qword ptr [rax + 0x70]
0x140689f3a movsd xmm2, qword ptr [rax + 0x78]
0x140689f3f movaps xmm3, xmm1
0x140689f42 mulps xmm3, xmm7
0x140689f45 addps xmm3, xmm2
0x140689f48 mov r9, qword ptr [rbp + 0x28]
0x140689f4c mov edx, dword ptr [r9 + 8]
0x140689f50 mov dword ptr [rsi + 8], edx
0x140689f53 mov rdx, qword ptr [r9]
0x140689f56 mov qword ptr [rsi], rdx
0x140689f59 movzx eax, byte ptr [rax + 0xb6]
0x140689f60 mov rdx, qword ptr [rbp + 0x90]
0x140689f67 mov qword ptr [rbp + 0x40], rdx
0x140689f6b movlhps xmm3, xmm1
0x140689f6e movups xmmword ptr [rbp + 0x48], xmm3
0x140689f72 movd dword ptr [rbp + 0x58], xmm0
0x140689f77 movss dword ptr [rbp + 0x5c], xmm6
0x140689f7c mov byte ptr [rbp + 0x6c], al
0x140689f7f mov rdx, r15
0x140689f82 call qword ptr [r8 + 0x68]
0x140689f86 jmp 0x140689f06
0x140689f8b cmp qword ptr [rbp + 0xc8], 0
0x140689f93 je 0x140689fb2
0x140689f95 mov rdx, qword ptr [rbp + 0xc8]
0x140689f9c shl rdx, 3
0x140689fa0 mov r8d, 8
0x140689fa6 mov rcx, qword ptr [rbp + 0xf0]
0x140689fad call 0x140613c20
0x140689fb2 movdqu xmm0, xmmword ptr [rip + 0x238dd2e]
0x140689fba movdqa xmmword ptr [rbp + 0x40], xmm0
0x140689fbf mov byte ptr [rbp + 0x107], 0
0x140689fc6 lea rcx, [rbp + 0x40]
0x140689fca call 0x140c91930
0x140689fcf mov r14d, eax
0x140689fd2 mov byte ptr [rbp + 0x107], 0
0x140689fd9 mov rcx, qword ptr [rbp + 0xd0]
0x140689fe0 call 0x1409a9130
0x140689fe5 test al, 1
0x140689fe7 je 0x14068aaba
0x140689fed mov r15, rdx
0x140689ff0 mov qword ptr [rbp + 0x40], rdx
0x140689ff4 mov rax, qword ptr [rbp + 0xf8]
0x140689ffb cmp qword ptr [rax + 0x1b8], 0
0x14068a003 je 0x14068aaba
0x14068a009 lea rdx, [rbp + 0x40]
0x14068a00d mov rcx, qword ptr [rbp + 0xe8]
0x14068a014 call 0x1405b0d70
0x14068a019 mov rdx, qword ptr [rbp + 0xf8]
0x14068a020 mov rcx, qword ptr [rdx + 0x1a0]
0x14068a027 mov rdx, qword ptr [rdx + 0x1a8]
0x14068a02e mov r8, rdx
0x14068a031 and r8, rax
0x14068a034 shr rax, 0x39
0x14068a038 movd xmm0, eax
0x14068a03c punpcklbw xmm0, xmm0
0x14068a040 pshuflw xmm0, xmm0, 0
0x14068a045 pshufd xmm0, xmm0, 0
0x14068a04a lea rax, [rcx - 0xc0]
0x14068a051 xor r9d, r9d
0x14068a054 pcmpeqd xmm1, xmm1
0x14068a058 movdqu xmm2, xmmword ptr [rcx + r8]
0x14068a05e movdqa xmm3, xmm2
0x14068a062 pcmpeqb xmm3, xmm0
0x14068a066 pmovmskb r10d, xmm3
0x14068a06b test r10d, r10d
0x14068a06e je 0x14068a09d
0x14068a070 tzcnt r11d, r10d
0x14068a075 add r11, r8
0x14068a078 and r11, rdx
0x14068a07b neg r11
0x14068a07e lea r11, [r11 + r11*2]
0x14068a082 shl r11, 6
0x14068a086 cmp r15, qword ptr [rax + r11]
0x14068a08a je 0x14068a9e8
0x14068a090 lea r11d, [r10 - 1]
0x14068a094 and r11w, r10w
0x14068a098 mov r10d, r11d
0x14068a09b jne 0x14068a070
0x14068a09d pcmpeqb xmm2, xmm1
0x14068a0a1 pmovmskb r10d, xmm2
0x14068a0a6 test r10d, r10d
0x14068a0a9 jne 0x14068aaba
0x14068a0af add r8, r9
0x14068a0b2 add r8, 0x10
0x14068a0b6 add r9, 0x10
0x14068a0ba and r8, rdx
0x14068a0bd jmp 0x14068a058
0x14068a0bf mov dword ptr [rbp + 0xd8], 0
0x14068a0c9 mov rax, qword ptr [rbp + 0xf8]
0x14068a0d0 mov ecx, dword ptr [rax + 0xd8]
0x14068a0d6 movss xmm1, dword ptr [rax + 0xdc]
0x14068a0de call 0x14093fa90
0x14068a0e3 test al, al
0x14068a0e5 je 0x14068a117
0x14068a0e7 mov rax, qword ptr [rbp + 0xf8]
0x14068a0ee mov ecx, dword ptr [rax + 0xd0]
0x14068a0f4 movss xmm1, dword ptr [rax + 0xd4]
0x14068a0fc call 0x14093fa90
0x14068a101 mov dword ptr [rbp + 0x98], eax
0x14068a107 mov eax, dword ptr [rbp + 0x98]
0x14068a10d xor al, 1
0x14068a10f mov dword ptr [rbp + 0x98], eax
0x14068a115 jmp 0x14068a121
0x14068a117 mov dword ptr [rbp + 0x98], 0
0x14068a121 mov rax, qword ptr [rbp + 0xf8]
0x14068a128 mov ecx, dword ptr [rax + 0xe0]
0x14068a12e movss xmm1, dword ptr [rax + 0xe4]
0x14068a136 call 0x14093fa90
0x14068a13b test al, al
0x14068a13d je 0x14068a161
0x14068a13f mov rax, qword ptr [rbp + 0xf8]
0x14068a146 mov ecx, dword ptr [rax + 0xe8]
0x14068a14c movss xmm1, dword ptr [rax + 0xec]
0x14068a154 call 0x14093fa90
0x14068a159 mov esi, eax
0x14068a15b xor sil, 1
0x14068a15f jmp 0x14068a163
0x14068a161 xor esi, esi
0x14068a163 mov rax, qword ptr [rbp + 0xf8]
0x14068a16a mov ecx, dword ptr [rax + 0xe8]
0x14068a170 movss xmm1, dword ptr [rax + 0xec]
0x14068a178 call 0x14093fa90
0x14068a17d test al, al
0x14068a17f je 0x14068a1a8
0x14068a181 mov rax, qword ptr [rbp + 0xf8]
0x14068a188 mov ecx, dword ptr [rax + 0xe0]
0x14068a18e movss xmm1, dword ptr [rax + 0xe4]
0x14068a196 call 0x14093fa90
0x14068a19b mov dword ptr [rbp + 0x38], eax
0x14068a19e mov eax, dword ptr [rbp + 0x38]
0x14068a1a1 xor al, 1
0x14068a1a3 mov dword ptr [rbp + 0x38], eax
0x14068a1a6 jmp 0x14068a1af
0x14068a1a8 mov dword ptr [rbp + 0x38], 0
0x14068a1af movzx eax, byte ptr [rbp + 0xd8]
0x14068a1b6 xorps xmm0, xmm0
0x14068a1b9 cvtsi2ss xmm0, eax
0x14068a1bd xorps xmm1, xmm1
0x14068a1c0 subss xmm1, xmm0
0x14068a1c4 movzx eax, byte ptr [rbp + 0x98]
0x14068a1cb xorps xmm0, xmm0
0x14068a1ce cvtsi2ss xmm0, eax
0x14068a1d2 movzx eax, sil
0x14068a1d6 xorps xmm2, xmm2
0x14068a1d9 cvtsi2ss xmm2, eax
0x14068a1dd mov ecx, dword ptr [rbp + 0x38]
0x14068a1e0 movzx eax, cl
0x14068a1e3 xorps xmm3, xmm3
0x14068a1e6 cvtsi2ss xmm3, eax
0x14068a1ea addss xmm0, xmm1
0x14068a1ee or sil, cl
0x14068a1f1 mov dword ptr [rbp], esi
0x14068a1f4 subss xmm2, xmm3
0x14068a1f8 movss dword ptr [rbp - 0x40], xmm0
0x14068a1fd movss dword ptr [rbp - 0x3c], xmm2
0x14068a202 mov qword ptr [rbp + 0x20], 0
0x14068a20a mov r8, qword ptr [rbp + 0xf8]
0x14068a211 mov rax, qword ptr [r8 + 0x1a0]
0x14068a218 mov rcx, qword ptr [r8 + 0x1a8]
0x14068a21f lea rcx, [rax + rcx + 1]
0x14068a224 movdqa xmm0, xmmword ptr [rax]
0x14068a228 pmovmskb edx, xmm0
0x14068a22c not edx
0x14068a22e mov r8, qword ptr [r8 + 0x1b8]
0x14068a235 mov qword ptr [rbp + 0x40], rax
0x14068a239 add rax, 0x10
0x14068a23d mov qword ptr [rbp + 0x48], rax
0x14068a241 mov qword ptr [rbp + 0x50], rcx
0x14068a245 mov word ptr [rbp + 0x58], dx
0x14068a249 mov qword ptr [rbp + 0x60], r8
0x14068a24d xor r15d, r15d
0x14068a250 xor r13d, r13d
0x14068a253 mov qword ptr [rbp + 0xc8], 0
0x14068a25e xor r12d, r12d
0x14068a261 lea rcx, [rbp + 0x40]
0x14068a265 call 0x140fb1730
0x14068a26a test rax, rax
0x14068a26d je 0x14068a35a
0x14068a273 mov r14, rdx
0x14068a276 mov rsi, qword ptr [rax]
0x14068a279 movzx eax, byte ptr [rdx + 0xb7]
0x14068a280 test al, 8
0x14068a282 jne 0x14068a296
0x14068a284 test byte ptr [r14 + 0xb6], 0x10
0x14068a28c jne 0x14068a2aa
0x14068a28e test al, 4
0x14068a290 je 0x14068a2ed
0x14068a292 xor edi, edi
0x14068a294 jmp 0x14068a2c3
0x14068a296 mov bl, 1
0x14068a298 test al, 4
0x14068a29a je 0x14068a2d7
0x14068a29c mov rcx, qword ptr [rbp + 0xd0]
0x14068a2a3 call 0x140985f30
0x14068a2a8 jmp 0x14068a2d7
0x14068a2aa mov edi, eax
0x14068a2ac shr dil, 5
0x14068a2b0 and dil, 1
0x14068a2b4 or r13b, dil
0x14068a2b7 test al, 4
0x14068a2b9 jne 0x14068a2c3
0x14068a2bb mov bl, 1
0x14068a2bd test al, 0x20
0x14068a2bf jne 0x14068a2d7
0x14068a2c1 jmp 0x14068a2ed
0x14068a2c3 mov rcx, qword ptr [rbp + 0xd0]
0x14068a2ca call 0x140985f30
0x14068a2cf or dil, al
0x14068a2d2 mov ebx, r13d
0x14068a2d5 je 0x14068a2ed
0x14068a2d7 mov eax, 1
0x14068a2dc mov qword ptr [rbp + 0xc8], rax
0x14068a2e3 mov r13d, ebx
0x14068a2e6 mov qword ptr [rbp + 0xc0], rsi
0x14068a2ed test r15, r15
0x14068a2f0 je 0x14068a2fa
0x14068a2f2 mov r15d, 1
0x14068a2f8 jmp 0x14068a31c
0x14068a2fa movzx r15d, byte ptr [r14 + 0xb7]
0x14068a302 and r15d, 0x20
0x14068a306 mov rax, qword ptr [rbp + 0xf0]
0x14068a30d cmovne rax, rsi
0x14068a311 mov qword ptr [rbp + 0xf0], rax
0x14068a318 shr r15d, 5
0x14068a31c mov rax, qword ptr [rbp + 0x30]
0x14068a320 cmp dword ptr [rax + 0x10], 2
0x14068a324 jne 0x14068a261
0x14068a32a mov rax, qword ptr [rbp + 0x30]
0x14068a32e cmp rsi, qword ptr [rax + 0x18]
0x14068a332 jne 0x14068a261
0x14068a338 movsd xmm0, qword ptr [r14 + 0x70]
0x14068a33e movsd xmm1, qword ptr [r14 + 0x78]
0x14068a344 mulps xmm0, xmmword ptr [rip + 0x229b595]
0x14068a34b addps xmm0, xmm1
0x14068a34e movlps qword ptr [rbp + 0x20], xmm0
0x14068a352 mov r12b, 1
0x14068a355 jmp 0x14068a261
0x14068a35a test r12b, 1
0x14068a35e jne 0x14068a36e
0x14068a360 mov rax, qword ptr [rbp + 0xd0]
0x14068a367 mov qword ptr [rax], 0
0x14068a36e cmp qword ptr [rbp + 0xc8], 1
0x14068a376 jne 0x14068a3ad
0x14068a378 mov rax, qword ptr [rbp - 0x20]
0x14068a37c mov qword ptr [rbp + 0x40], rax
0x14068a380 lea r8, [rbp + 0x40]
0x14068a384 mov rcx, qword ptr [rbp + 0xf8]
0x14068a38b mov rdx, qword ptr [rbp + 0xc0]
0x14068a392 call 0x140f9e3e0
0x14068a397 cmp rax, 1
0x14068a39b jne 0x14068a3ad
0x14068a39d mov rax, qword ptr [rbp + 0x30]
0x14068a3a1 mov qword ptr [rax + 0x10], 2
0x14068a3a9 mov qword ptr [rax + 0x18], rdx
0x14068a3ad mov rcx, qword ptr [rbp + 0xd0]
0x14068a3b4 call 0x140985f30
0x14068a3b9 test al, r15b
0x14068a3bc je 0x14068a3d5
0x14068a3be mov rax, qword ptr [rbp + 0x30]
0x14068a3c2 mov qword ptr [rax + 0x10], 2
0x14068a3ca mov rcx, qword ptr [rbp + 0xf0]
0x14068a3d1 mov qword ptr [rax + 0x18], rcx
0x14068a3d5 not r12b
0x14068a3d8 or r12b, r13b
0x14068a3db test r12b, 1
0x14068a3df jne 0x140689d1c
0x14068a3e5 mov eax, dword ptr [rbp]
0x14068a3e8 or al, byte ptr [rbp + 0x98]
0x14068a3ee or al, byte ptr [rbp + 0xd8]
0x14068a3f4 xor al, 1
0x14068a3f6 jne 0x140689d1c
0x14068a3fc mov rax, qword ptr [rbp + 0xd0]
0x14068a403 cmp dword ptr [rax], 2
0x14068a406 jne 0x140689d1c
0x14068a40c mov rax, qword ptr [rbp + 0x30]
0x14068a410 mov rax, qword ptr [rax + 0x18]
0x14068a414 mov rbx, rax
0x14068a417 mov qword ptr [rbp - 0x18], rax
0x14068a41b mov rax, qword ptr [rbp + 0xf8]
0x14068a422 cmp qword ptr [rax + 0x1e8], 0
0x14068a42a je 0x14068a8f1
0x14068a430 mov rax, qword ptr [rbp + 0xf8]
0x14068a437 add rax, 0x1f0
0x14068a43d mov qword ptr [rbp + 0xc8], rax
0x14068a444 mov eax, dword ptr [rbp + 0x98]
0x14068a44a mov ecx, dword ptr [rbp + 0xd8]
0x14068a450 or al, cl
0x14068a452 mov dword ptr [rbp + 0x98], eax
0x14068a458 mov eax, dword ptr [rbp + 0x38]
0x14068a45b or al, cl
0x14068a45d mov dword ptr [rbp + 0x38], eax
0x14068a460 mov qword ptr [rbp + 0xc0], 0
0x14068a46b pcmpeqd xmm7, xmm7
0x14068a46f mov r15, rbx
0x14068a472 mov rcx, qword ptr [rbp + 0xc8]
0x14068a479 lea rdx, [rbp - 0x18]
0x14068a47d call 0x1405b0d70
0x14068a482 mov rdx, qword ptr [rbp + 0xf8]
0x14068a489 mov rcx, qword ptr [rdx + 0x1d0]
0x14068a490 mov rdx, qword ptr [rdx + 0x1d8]
0x14068a497 mov r8, rdx
0x14068a49a and r8, rax
0x14068a49d shr rax, 0x39
0x14068a4a1 movd xmm0, eax
0x14068a4a5 punpcklbw xmm0, xmm0
0x14068a4a9 pshuflw xmm0, xmm0, 0
0x14068a4ae pshufd xmm0, xmm0, 0
0x14068a4b3 lea rax, [rcx - 0x10]
0x14068a4b7 xor r9d, r9d
0x14068a4ba movdqu xmm1, xmmword ptr [rcx + r8]
0x14068a4c0 movdqa xmm2, xmm1
0x14068a4c4 pcmpeqb xmm2, xmm0
0x14068a4c8 pmovmskb r10d, xmm2
0x14068a4cd test r10d, r10d
0x14068a4d0 je 0x14068a4fc
0x14068a4d2 tzcnt r11d, r10d
0x14068a4d7 add r11, r8
0x14068a4da and r11, rdx
0x14068a4dd mov rsi, r11
0x14068a4e0 shl rsi, 4
0x14068a4e4 mov rdi, rax
0x14068a4e7 sub rdi, rsi
0x14068a4ea cmp r15, qword ptr [rdi]
0x14068a4ed je 0x14068a51e
0x14068a4ef lea r11d, [r10 - 1]
0x14068a4f3 and r11w, r10w
0x14068a4f7 mov r10d, r11d
0x14068a4fa jne 0x14068a4d2
0x14068a4fc pcmpeqb xmm1, xmm7
0x14068a500 pmovmskb r10d, xmm1
0x14068a505 test r10d, r10d
0x14068a508 jne 0x14068a8f1
0x14068a50e add r8, r9
0x14068a511 add r8, 0x10
0x14068a515 add r9, 0x10
0x14068a519 and r8, rdx
0x14068a51c jmp 0x14068a4ba
0x14068a51e neg r11
0x14068a521 shl r11, 4
0x14068a525 mov rax, qword ptr [rcx + r11 - 8]
0x14068a52a mov rbx, rax
0x14068a52d mov qword ptr [rbp + 8], rax
0x14068a531 mov rax, qword ptr [rbp + 0xf8]
0x14068a538 cmp qword ptr [rax + 0x1b8], 0
0x14068a540 je 0x14068a8f1
0x14068a546 mov rcx, qword ptr [rbp + 0xe8]
0x14068a54d lea rdx, [rbp + 8]
0x14068a551 call 0x1405b0d70
0x14068a556 mov rdx, qword ptr [rbp + 0xf8]
0x14068a55d mov rcx, qword ptr [rdx + 0x1a0]
0x14068a564 mov rdx, qword ptr [rdx + 0x1a8]
0x14068a56b mov r8, rdx
0x14068a56e and r8, rax
0x14068a571 shr rax, 0x39
0x14068a575 movd xmm0, eax
0x14068a579 punpcklbw xmm0, xmm0
0x14068a57d pshuflw xmm0, xmm0, 0
0x14068a582 pshufd xmm0, xmm0, 0
0x14068a587 lea rax, [rcx - 0xc0]
0x14068a58e xor r9d, r9d
0x14068a591 movdqu xmm1, xmmword ptr [rcx + r8]
0x14068a597 movdqa xmm2, xmm1
0x14068a59b pcmpeqb xmm2, xmm0
0x14068a59f pmovmskb r10d, xmm2
0x14068a5a4 test r10d, r10d
0x14068a5a7 je 0x14068a5d2
0x14068a5a9 tzcnt r11d, r10d
0x14068a5ae add r11, r8
0x14068a5b1 and r11, rdx
0x14068a5b4 neg r11
0x14068a5b7 lea r11, [r11 + r11*2]
0x14068a5bb shl r11, 6
0x14068a5bf cmp rbx, qword ptr [rax + r11]
0x14068a5c3 je 0x14068a5f4
0x14068a5c5 lea r11d, [r10 - 1]
0x14068a5c9 and r11w, r10w
0x14068a5cd mov r10d, r11d
0x14068a5d0 jne 0x14068a5a9
0x14068a5d2 pcmpeqb xmm1, xmm7
0x14068a5d6 pmovmskb r10d, xmm1
0x14068a5db test r10d, r10d
0x14068a5de jne 0x14068a8f1
0x14068a5e4 add r8, r9
0x14068a5e7 add r8, 0x10
0x14068a5eb add r9, 0x10
0x14068a5ef and r8, rdx
0x14068a5f2 jmp 0x14068a591
0x14068a5f4 cmp byte ptr [rcx + r11 - 0x5c], 0
0x14068a5fa sete al
0x14068a5fd cmp byte ptr [rbp + 0x98], al
0x14068a603 jne 0x14068a8d8
0x14068a609 add rcx, r11
0x14068a60c mov rdi, qword ptr [rcx - 0xa8]
0x14068a613 test rdi, rdi
0x14068a616 je 0x14068ad43
0x14068a61c mov rcx, qword ptr [rcx - 0xb0]
0x14068a623 lea rax, [rcx + rdi*8]
0x14068a627 mov r12, 0xfffffffffffffff0
0x14068a62e mov rsi, 0xffffffffffffffff
0x14068a635 xor r14d, r14d
0x14068a638 mov qword ptr [rbp + 0xf0], rcx
0x14068a63f cmp r15, qword ptr [rcx]
0x14068a642 je 0x14068a65c
0x14068a644 add rcx, 8
0x14068a648 inc r14
0x14068a64b add r12, 8
0x14068a64f inc rsi
0x14068a652 cmp rcx, rax
0x14068a655 jne 0x14068a63f
0x14068a657 jmp 0x14068ad43
0x14068a65c cmp byte ptr [rbp + 0x38], 0
0x14068a660 je 0x14068a6bc
0x14068a662 mov rax, r14
0x14068a665 shr rax, 0x3d
0x14068a669 jne 0x14068ad60
0x14068a66f lea r15, [r14*8]
0x14068a677 movabs rax, 0x7ffffffffffffff8
0x14068a681 cmp r15, rax
0x14068a684 ja 0x14068ad60
0x14068a68a test r15, r15
0x14068a68d je 0x14068a719
0x14068a693 movzx eax, byte ptr [rip + 0x2faa56f]
0x14068a69a mov edx, 8
0x14068a69f mov rcx, r15
0x14068a6a2 call 0x140613c10
0x14068a6a7 test rax, rax
0x14068a6aa je 0x14068ad51
0x14068a6b0 mov r13, rax
0x14068a6b3 mov qword ptr [rbp + 0xd8], r14
0x14068a6ba jmp 0x14068a72a
0x14068a6bc inc r14
0x14068a6bf sub rdi, r14
0x14068a6c2 mov rax, rdi
0x14068a6c5 shr rax, 0x3d
0x14068a6c9 jne 0x14068ad88
0x14068a6cf lea r15, [rdi*8]
0x14068a6d7 movabs rax, 0x7ffffffffffffff8
0x14068a6e1 cmp r15, rax
0x14068a6e4 ja 0x14068ad88
0x14068a6ea test r15, r15
0x14068a6ed je 0x14068a759
0x14068a6ef movzx eax, byte ptr [rip + 0x2faa513]
0x14068a6f6 mov edx, 8
0x14068a6fb mov rcx, r15
0x14068a6fe call 0x140613c10
0x14068a703 mov qword ptr [rbp], rax
0x14068a707 test rax, rax
0x14068a70a je 0x14068ad78
0x14068a710 mov qword ptr [rbp + 0xd8], rdi
0x14068a717 jmp 0x14068a76d
0x14068a719 mov r13d, 8
0x14068a71f mov qword ptr [rbp + 0xd8], 0
0x14068a72a mov rcx, r13
0x14068a72d mov rdx, qword ptr [rbp + 0xf0]
0x14068a734 mov r8, r15
0x14068a737 call 0x142923c00
0x14068a73c cmp r14, 2
0x14068a740 jb 0x14068a82e
0x14068a746 mov rax, r14
0x14068a749 shr rax, 1
0x14068a74c cmp r14, 8
0x14068a750 jae 0x14068a793
0x14068a752 xor ecx, ecx
0x14068a754 jmp 0x14068a80c
0x14068a759 mov eax, 8
0x14068a75e mov qword ptr [rbp], rax
0x14068a762 mov qword ptr [rbp + 0xd8], 0
0x14068a76d mov rax, qword ptr [rbp + 0xf0]
0x14068a774 lea rdx, [rax + r14*8]
0x14068a778 mov rsi, qword ptr [rbp]
0x14068a77c mov rcx, rsi
0x14068a77f mov r8, r15
0x14068a782 call 0x142923c00
0x14068a787 mov qword ptr [rbp + 0xf0], rsi
0x14068a78e jmp 0x14068a838
0x14068a793 mov rcx, rax
0x14068a796 movabs rdx, 0xffffffffffffffc
0x14068a7a0 and rcx, rdx
0x14068a7a3 add r12, r13
0x14068a7a6 mov rdx, rax
0x14068a7a9 and rdx, 0xfffffffffffffffc
0x14068a7ad xor r8d, r8d
0x14068a7b0 movdqu xmm0, xmmword ptr [r13 + r8*8]
0x14068a7b7 movdqu xmm1, xmmword ptr [r13 + r8*8 + 0x10]
0x14068a7be movdqu xmm2, xmmword ptr [r12 - 0x10]
0x14068a7c5 movdqu xmm3, xmmword ptr [r12]
0x14068a7cb pshufd xmm3, xmm3, 0x4e
0x14068a7d0 pshufd xmm2, xmm2, 0x4e
0x14068a7d5 movdqu xmmword ptr [r13 + r8*8], xmm3
0x14068a7dc movdqu xmmword ptr [r13 + r8*8 + 0x10], xmm2
0x14068a7e3 pshufd xmm0, xmm0, 0x4e
0x14068a7e8 movdqu xmmword ptr [r12], xmm0
0x14068a7ee pshufd xmm0, xmm1, 0x4e
0x14068a7f3 movdqu xmmword ptr [r12 - 0x10], xmm0
0x14068a7fa add r8, 4
0x14068a7fe add r12, -0x20
0x14068a802 cmp rdx, r8
0x14068a805 jne 0x14068a7b0
0x14068a807 cmp rax, rcx
0x14068a80a je 0x14068a82e
0x14068a80c sub rsi, rcx
0x14068a80f mov rdx, qword ptr [r13 + rcx*8]
0x14068a814 mov r8, qword ptr [r13 + rsi*8]
0x14068a819 mov qword ptr [r13 + rcx*8], r8
0x14068a81e mov qword ptr [r13 + rsi*8], rdx
0x14068a823 inc rcx
0x14068a826 dec rsi
0x14068a829 cmp rax, rcx
0x14068a82c jne 0x14068a80f
0x14068a82e mov qword ptr [rbp + 0xf0], r13
0x14068a835 mov rdi, r14
0x14068a838 test rdi, rdi
0x14068a83b je 0x14068a8b1
0x14068a83d shl rdi, 3
0x14068a841 xor esi, esi
0x14068a843 mov rax, qword ptr [rbp + 0xf0]
0x14068a84a mov rdx, qword ptr [rax + rsi]
0x14068a84e mov qword ptr [rbp + 0xa0], 0
0x14068a859 mov dword ptr [rbp - 0x24], 0x7f7fffff
0x14068a860 mov rax, qword ptr [rbp - 0x20]
0x14068a864 mov qword ptr [rbp + 0x40], rax
0x14068a868 lea rax, [rbp + 0x20]
0x14068a86c mov qword ptr [rbp + 0x48], rax
0x14068a870 lea rax, [rbp - 0x40]
0x14068a874 mov qword ptr [rbp + 0x50], rax
0x14068a878 lea rax, [rbp - 0x24]
0x14068a87c mov qword ptr [rbp + 0x58], rax
0x14068a880 lea rax, [rbp + 0xa0]
0x14068a887 mov qword ptr [rbp + 0x60], rax
0x14068a88b mov rcx, qword ptr [rbp + 0xf8]
0x14068a892 lea r8, [rbp + 0x40]
0x14068a896 call 0x140f9e610
0x14068a89b cmp dword ptr [rbp + 0xa0], 1
0x14068a8a2 je 0x14068aca8
0x14068a8a8 add rsi, 8
0x14068a8ac cmp rdi, rsi
0x14068a8af jne 0x14068a843
0x14068a8b1 cmp qword ptr [rbp + 0xd8], 0
0x14068a8b9 je 0x14068a8d8
0x14068a8bb mov rdx, qword ptr [rbp + 0xd8]
0x14068a8c2 shl rdx, 3
0x14068a8c6 mov r8d, 8
0x14068a8cc mov rcx, qword ptr [rbp + 0xf0]
0x14068a8d3 call 0x140613c20
0x14068a8d8 mov qword ptr [rbp - 0x18], rbx
0x14068a8dc mov rax, qword ptr [rbp + 0xf8]
0x14068a8e3 cmp qword ptr [rax + 0x1e8], 0
0x14068a8eb jne 0x14068a46f
0x14068a8f1 mov rcx, qword ptr [rbp + 0xd0]
0x14068a8f8 call 0x1409a9130
0x14068a8fd test al, 1
0x14068a8ff je 0x14068ada1
0x14068a905 mov r14, rdx
0x14068a908 mov qword ptr [rbp + 0x40], rdx
0x14068a90c mov rax, qword ptr [rbp + 0xf8]
0x14068a913 cmp qword ptr [rax + 0x1b8], 0
0x14068a91b je 0x14068ac97
0x14068a921 lea rdx, [rbp + 0x40]
0x14068a925 mov rcx, qword ptr [rbp + 0xe8]
0x14068a92c call 0x1405b0d70
0x14068a931 mov rdx, qword ptr [rbp + 0xf8]
0x14068a938 mov rcx, qword ptr [rdx + 0x1a0]
0x14068a93f mov rdx, qword ptr [rdx + 0x1a8]
0x14068a946 mov r8, rdx
0x14068a949 and r8, rax
0x14068a94c shr rax, 0x39
0x14068a950 movd xmm0, eax
0x14068a954 punpcklbw xmm0, xmm0
0x14068a958 pshuflw xmm0, xmm0, 0
0x14068a95d pshufd xmm0, xmm0, 0
0x14068a962 lea rax, [rcx - 0xc0]
0x14068a969 xor r9d, r9d
0x14068a96c pcmpeqd xmm1, xmm1
0x14068a970 movdqu xmm2, xmmword ptr [rcx + r8]
0x14068a976 movdqa xmm3, xmm2
0x14068a97a pcmpeqb xmm3, xmm0
0x14068a97e pmovmskb r10d, xmm3
0x14068a983 test r10d, r10d
0x14068a986 je 0x14068a9b5
0x14068a988 tzcnt r11d, r10d
0x14068a98d add r11, r8
0x14068a990 and r11, rdx
0x14068a993 neg r11
0x14068a996 lea r11, [r11 + r11*2]
0x14068a99a shl r11, 6
0x14068a99e cmp r14, qword ptr [rax + r11]
0x14068a9a2 je 0x14068ac78
0x14068a9a8 lea r11d, [r10 - 1]
0x14068a9ac and r11w, r10w
0x14068a9b0 mov r10d, r11d
0x14068a9b3 jne 0x14068a988
0x14068a9b5 pcmpeqb xmm2, xmm1
0x14068a9b9 pmovmskb r10d, xmm2
0x14068a9be test r10d, r10d
0x14068a9c1 jne 0x14068ac97
0x14068a9c7 add r8, r9
0x14068a9ca add r8, 0x10
0x14068a9ce add r9, 0x10
0x14068a9d2 and r8, rdx
0x14068a9d5 jmp 0x14068a970
0x14068a9d7 lea rcx, [rip + 0x238d2f2]
0x14068a9de call 0x1428d9310
0x14068a9e3 jmp 0x14068adad
0x14068a9e8 test byte ptr [rcx + r11 - 1], 0x40
0x14068a9ee jne 0x14068aaba
0x14068a9f4 mov rax, qword ptr [rbp + 0x30]
0x14068a9f8 cmp byte ptr [rax + 0x30], 1
0x14068a9fc jne 0x14068aaba
0x14068aa02 add rcx, r11
0x14068aa05 mov rbx, qword ptr [rcx - 0x48]
0x14068aa09 mov r15, qword ptr [rcx - 0x40]
0x14068aa0d mov byte ptr [rbp + 0x107], 0
0x14068aa14 lea rcx, [rbp + 0x40]
0x14068aa18 call 0x140a82b20
0x14068aa1d mov byte ptr [rbp + 0x107], 0
0x14068aa24 lea rcx, [rbp + 0xa0]
0x14068aa2b lea rdx, [rbp + 0x40]
0x14068aa2f mov r8d, r14d
0x14068aa32 call 0x140b920b0
0x14068aa37 mov byte ptr [rbp + 0x107], 0
0x14068aa3e lea rcx, [rbp + 0x40]
0x14068aa42 lea rdx, [rbp + 0xa0]
0x14068aa49 movd xmm2, dword ptr [rip + 0x2342627]
0x14068aa51 call 0x140b92160
0x14068aa56 lea r14, [rbp + 0xa0]
0x14068aa5d lea rdx, [rbp + 0x40]
0x14068aa61 mov rcx, r14
0x14068aa64 call 0x140b92220
0x14068aa69 mov qword ptr [rsp + 0x20], r14
0x14068aa6e movd xmm3, dword ptr [rip + 0x229ae8e]
0x14068aa76 mov rcx, qword ptr [rbp + 0x90]
0x14068aa7d mov rdx, r15
0x14068aa80 mov r8, rbx
0x14068aa83 call 0x140b8f510
0x14068aa88 mov rdx, qword ptr [rbp + 0xa0]
0x14068aa8f test rdx, rdx
0x14068aa92 je 0x14068aaaa
0x14068aa94 mov rcx, qword ptr [rbp + 0xa8]
0x14068aa9b shl rdx, 2
0x14068aa9f mov r8d, 4
0x14068aaa5 call 0x140613c20
0x14068aaaa mov byte ptr [rbp + 0x107], 0
0x14068aab1 lea rcx, [rbp + 0x40]
0x14068aab5 call 0x1405f11f0
0x14068aaba mov rax, qword ptr [rbp + 0xf8]
0x14068aac1 mov rcx, qword ptr [rax + 0x20]
0x14068aac5 mov qword ptr [rbp + 0x28], rcx
0x14068aac9 mov rcx, qword ptr [rax + 0x28]
0x14068aacd mov qword ptr [rbp + 0xe8], rcx
0x14068aad4 mov qword ptr [rax + 0x28], 0
0x14068aadc mov qword ptr [rbp + 0x90], 0
0x14068aae7 mov esi, 0xffffff3d
0x14068aaec mov edi, 1
0x14068aaf1 mov rax, qword ptr [rbp + 0x90]
0x14068aaf8 cmp rax, qword ptr [rbp + 0xe8]
0x14068aaff je 0x14068ab7f
0x14068ab01 imul rcx, rax, 0x70
0x14068ab05 inc rax
0x14068ab08 mov qword ptr [rbp + 0x90], rax
0x14068ab0f mov rax, qword ptr [rbp + 0x28]
0x14068ab13 mov eax, dword ptr [rax + rcx + 0x58]
0x14068ab17 add eax, esi
0x14068ab19 cmp eax, 3
0x14068ab1c cmovae eax, edi
0x14068ab1f test eax, eax
0x14068ab21 je 0x14068aaf1
0x14068ab23 add rcx, qword ptr [rbp + 0x28]
0x14068ab27 cmp eax, 1
0x14068ab2a jne 0x14068ab63
0x14068ab2c mov qword ptr [rbp + 0xf0], rcx
0x14068ab33 add rcx, 0x18
0x14068ab37 call 0x1405f1730
0x14068ab3c mov rcx, qword ptr [rbp + 0xf0]
0x14068ab43 movzx eax, byte ptr [rcx]
0x14068ab46 cmp al, 0x1b
0x14068ab48 je 0x14068aaf1
0x14068ab4a cmp al, 0x18
0x14068ab4c jne 0x14068aaf1
0x14068ab4e mov rax, qword ptr [rcx + 8]
0x14068ab52 lock dec qword ptr [rax]
0x14068ab56 jne 0x14068aaf1
0x14068ab58 add rcx, 8
0x14068ab5c call 0x14064eb30
0x14068ab61 jmp 0x14068aaf1
0x14068ab63 mov rdx, qword ptr [rcx]
0x14068ab66 test rdx, rdx
0x14068ab69 je 0x14068aaf1
0x14068ab6b mov rcx, qword ptr [rcx + 8]
0x14068ab6f mov r8d, 1
0x14068ab75 call 0x140613c20
0x14068ab7a jmp 0x14068aaf1
0x14068ab7f mov r8, qword ptr [rbp + 0xf8]
0x14068ab86 mov rax, qword ptr [r8 + 0x1a0]
0x14068ab8d mov rcx, qword ptr [r8 + 0x1a8]
0x14068ab94 add rcx, rax
0x14068ab97 inc rcx
0x14068ab9a movdqa xmm0, xmmword ptr [rax]
0x14068ab9e pmovmskb edx, xmm0
0x14068aba2 not edx
0x14068aba4 mov r8, qword ptr [r8 + 0x1b8]
0x14068abab mov qword ptr [rbp + 0x40], rax
0x14068abaf add rax, 0x10
0x14068abb3 mov qword ptr [rbp + 0x48], rax
0x14068abb7 mov qword ptr [rbp + 0x50], rcx
0x14068abbb mov word ptr [rbp + 0x58], dx
0x14068abbf mov qword ptr [rbp + 0x60], r8
0x14068abc3 lea rsi, [rbp + 0x40]
0x14068abc7 mov byte ptr [rbp + 0x107], 0
0x14068abce mov rcx, rsi
0x14068abd1 call 0x140fb1730
0x14068abd6 test rax, rax
0x14068abd9 je 0x14068abe9
0x14068abdb test rdx, rdx
0x14068abde je 0x14068abe9
0x14068abe0 and byte ptr [rdx + 0xb6], 0xcf
0x14068abe7 jmp 0x14068abc7
0x14068abe9 cmp qword ptr [rbp - 0x38], 0
0x14068abee je 0x14068ac16
0x14068abf0 lea rax, [rbp - 0x30]
0x14068abf4 mov qword ptr [rbp + 0xa0], rax
0x14068abfb lea rax, [rbp + 0xa0]
0x14068ac02 mov qword ptr [rbp + 0x40], rax
0x14068ac06 lea rcx, [rip + 0x238cd93]
0x14068ac0d lea rdx, [rbp + 0x40]
0x14068ac11 call 0x140fcfc20
0x14068ac16 movaps xmm6, xmmword ptr [rbp + 0x110]
0x14068ac1d movaps xmm7, xmmword ptr [rbp + 0x120]
0x14068ac24 movaps xmm8, xmmword ptr [rbp + 0x130]
0x14068ac2c movaps xmm9, xmmword ptr [rbp + 0x140]
0x14068ac34 movaps xmm10, xmmword ptr [rbp + 0x150]
0x14068ac3c movaps xmm11, xmmword ptr [rbp + 0x160]
0x14068ac44 movaps xmm12, xmmword ptr [rbp + 0x170]
0x14068ac4c movaps xmm13, xmmword ptr [rbp + 0x180]
0x14068ac54 movaps xmm14, xmmword ptr [rbp + 0x190]
0x14068ac5c movaps xmm15, xmmword ptr [rbp + 0x1a0]
0x14068ac64 add rsp, 0x238
0x14068ac6b pop rbx
0x14068ac6c pop rdi
0x14068ac6d pop rsi
0x14068ac6e pop r12
0x14068ac70 pop r13
0x14068ac72 pop r14
0x14068ac74 pop r15
0x14068ac76 pop rbp
0x14068ac77 ret 
0x14068ac78 test byte ptr [rcx + r11 - 1], 0x10
0x14068ac7e je 0x140689d1c
0x14068ac84 mov rax, qword ptr [rbp + 0xd0]
0x14068ac8b mov qword ptr [rax], 0
0x14068ac92 jmp 0x140689d1c
0x14068ac97 lea rcx, [rip + 0x238d24a]
0x14068ac9e call 0x1428d9310
0x14068aca3 jmp 0x14068adad
0x14068aca8 mov rax, qword ptr [rbp + 0xa8]
0x14068acaf mov rcx, qword ptr [rbp + 0x30]
0x14068acb3 mov qword ptr [rcx + 0x10], 2
0x14068acbb mov qword ptr [rcx + 0x18], rax
0x14068acbf cmp qword ptr [rbp + 0xd8], 0
0x14068acc7 je 0x140689d1c
0x14068accd mov rdx, qword ptr [rbp + 0xd8]
0x14068acd4 shl rdx, 3
0x14068acd8 mov r8d, 8
0x14068acde mov rcx, qword ptr [rbp + 0xf0]
0x14068ace5 call 0x140613c20
0x14068acea jmp 0x140689d1c
0x14068acef cmp r14, 0x15
0x14068acf3 jae 0x14068adaf
0x14068acf9 mov byte ptr [rbp + 0x107], 1
0x14068ad00 lea r9, [rbp + 0x40]
0x14068ad04 mov r8d, 1
0x14068ad0a mov rcx, qword ptr [rbp + 0xf0]
0x14068ad11 mov rdx, r14
0x14068ad14 call 0x140fe4040
0x14068ad19 mov rax, qword ptr [rbp + 8]
0x14068ad1d mov qword ptr [rbp + 0xc8], rax
0x14068ad24 jmp 0x140689eef
0x14068ad29 lea rcx, [rip + 0x238cff8]
0x14068ad30 lea r8, [rip + 0x238d019]
0x14068ad37 mov edx, 0x28
0x14068ad3c call 0x1428d9430
0x14068ad41 jmp 0x14068adad
0x14068ad43 lea rcx, [rip + 0x238d16e]
0x14068ad4a call 0x1428d9310
0x14068ad4f jmp 0x14068adad
0x14068ad51 mov eax, 8
0x14068ad56 mov qword ptr [rbp + 0xc0], rax
0x14068ad5d mov r13, r15
0x14068ad60 lea r8, [rip + 0x238c699]
0x14068ad67 mov rcx, qword ptr [rbp + 0xc0]
0x14068ad6e mov rdx, r13
0x14068ad71 call 0x1428d8fe3
0x14068ad76 jmp 0x14068adad
0x14068ad78 mov eax, 8
0x14068ad7d mov qword ptr [rbp + 0xc0], rax
0x14068ad84 mov qword ptr [rbp], r15
0x14068ad88 lea r8, [rip + 0x238c671]
0x14068ad8f mov rcx, qword ptr [rbp + 0xc0]
0x14068ad96 mov rdx, qword ptr [rbp]
0x14068ad9a call 0x1428d8fe3
0x14068ad9f jmp 0x14068adad
0x14068ada1 lea rcx, [rip + 0x238d128]
0x14068ada8 call 0x1428d9310
0x14068adad ud2 
0x14068adaf mov byte ptr [rbp + 0x107], 1
0x14068adb6 lea r8, [rbp + 0x40]
0x14068adba mov rcx, qword ptr [rbp + 0xf0]
0x14068adc1 mov rdx, r14
0x14068adc4 call 0x140fe4a40
0x14068adc9 jmp 0x14068ad19
0x14068adce nop 
0x14068add0 mov qword ptr [rsp + 0x10], rdx
0x14068add5 push rbp
0x14068add6 push r15
0x14068add8 push r14
0x14068adda push r13
0x14068addc push r12
0x14068adde push rsi
0x14068addf push rdi
0x14068ade0 push rbx
0x14068ade1 sub rsp, 0xd8
0x14068ade8 lea rbp, [rdx + 0x80]
0x14068adef movaps xmmword ptr [rsp + 0x30], xmm15
0x14068adf5 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068adfb movaps xmmword ptr [rsp + 0x50], xmm13
0x14068ae01 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068ae07 movdqa xmmword ptr [rsp + 0x70], xmm11
0x14068ae0e movaps xmmword ptr [rsp + 0x80], xmm10
0x14068ae17 movaps xmmword ptr [rsp + 0x90], xmm9
0x14068ae20 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x14068ae2a movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068ae32 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068ae3a mov rax, qword ptr [rbp + 0x28]
0x14068ae3e inc qword ptr [rax]
0x14068ae41 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068ae49 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068ae51 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068ae5a movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068ae63 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068ae6c movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068ae72 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068ae78 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068ae7e movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068ae84 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068ae8a add rsp, 0xd8
0x14068ae91 pop rbx
0x14068ae92 pop rdi
0x14068ae93 pop rsi
0x14068ae94 pop r12
0x14068ae96 pop r13
0x14068ae98 pop r14
0x14068ae9a pop r15
0x14068ae9c pop rbp
0x14068ae9d ret 
0x14068ae9e nop 
0x14068aea0 mov qword ptr [rsp + 0x10], rdx
0x14068aea5 push rbp
0x14068aea6 push r15
0x14068aea8 push r14
0x14068aeaa push r13
0x14068aeac push r12
0x14068aeae push rsi
0x14068aeaf push rdi
0x14068aeb0 push rbx
0x14068aeb1 sub rsp, 0xd8
0x14068aeb8 lea rbp, [rdx + 0x80]
0x14068aebf movaps xmmword ptr [rsp + 0x30], xmm15
0x14068aec5 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068aecb movaps xmmword ptr [rsp + 0x50], xmm13
0x14068aed1 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068aed7 movdqa xmmword ptr [rsp + 0x70], xmm11
0x14068aede movaps xmmword ptr [rsp + 0x80], xmm10
0x14068aee7 movaps xmmword ptr [rsp + 0x90], xmm9
0x14068aef0 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x14068aefa movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068af02 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068af0a cmp qword ptr [rbp - 0x38], 0
0x14068af0f je 0x14068af4b
0x14068af11 call 0x140c18130
0x14068af16 mov rsi, rax
0x14068af19 mov rax, qword ptr [rax]
0x14068af1c cmp rax, 1
0x14068af20 jne 0x14068afa8
0x14068af26 add rsi, 8
0x14068af2a cmp qword ptr [rsi], 0
0x14068af2e jne 0x14068afcf
0x14068af34 mov qword ptr [rsi], 0xffffffffffffffff
0x14068af3b lea rcx, [rsi + 8]
0x14068af3f mov rdx, qword ptr [rbp - 0x30]
0x14068af43 call 0x140c170f0
0x14068af48 inc qword ptr [rsi]
0x14068af4b movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068af53 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068af5b movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068af64 movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068af6d movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068af76 movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068af7c movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068af82 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068af88 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068af8e movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068af94 add rsp, 0xd8
0x14068af9b pop rbx
0x14068af9c pop rdi
0x14068af9d pop rsi
0x14068af9e pop r12
0x14068afa0 pop r13
0x14068afa2 pop r14
0x14068afa4 pop r15
0x14068afa6 pop rbp
0x14068afa7 ret 
0x14068afa8 test rax, rax
0x14068afab jne 0x14068afc3
0x14068afad mov rcx, rsi
0x14068afb0 xor edx, edx
0x14068afb2 call 0x1428e6250
0x14068afb7 mov rsi, rax
0x14068afba test rax, rax
0x14068afbd jne 0x14068af2a
0x14068afc3 lea rcx, [rip + 0x252a30e]
0x14068afca call 0x1428d80d0
0x14068afcf lea rcx, [rip + 0x252a372]
0x14068afd6 call 0x1428d9250
0x14068afdb int3 
0x14068afdc nop dword ptr [rax]
0x14068afe0 mov qword ptr [rsp + 0x10], rdx
0x14068afe5 push rbp
0x14068afe6 push r15
0x14068afe8 push r14
0x14068afea push r13
0x14068afec push r12
0x14068afee push rsi
0x14068afef push rdi
0x14068aff0 push rbx
0x14068aff1 sub rsp, 0xd8
0x14068aff8 lea rbp, [rdx + 0x80]
0x14068afff movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b005 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b00b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b011 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b017 movdqa xmmword ptr [rsp + 0x70], xmm11
0x14068b01e movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b027 movaps xmmword ptr [rsp + 0x90], xmm9
0x14068b030 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x14068b03a movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b042 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b04a mov rax, qword ptr [rbp + 0xb0]
0x14068b051 test rax, rax
0x14068b054 je 0x14068b079
0x14068b056 mov rcx, qword ptr [rbp + 0xa8]
0x14068b05d mov rcx, qword ptr [rcx + 0x10]
0x14068b061 mov rdx, qword ptr [rcx + 0x20]
0x14068b065 cmp rdx, qword ptr [rbp + 0xa0]
0x14068b06c jne 0x14068b079
0x14068b06e shl rax, 6
0x14068b072 add rdx, rax
0x14068b075 mov qword ptr [rcx + 0x20], rdx
0x14068b079 mov rax, qword ptr [rbp + 0x18]
0x14068b07d cmp rax, qword ptr [rip + 0x2fa135c]
0x14068b084 je 0x14068b0ac
0x14068b086 mov rsi, qword ptr [rip + 0x2fa1353]
0x14068b08d nop dword ptr [rax]
0x14068b090 mov rdi, qword ptr [rax + 0x18]
0x14068b094 mov rcx, qword ptr [rax]
0x14068b097 mov r8, qword ptr [rax + 8]
0x14068b09b mov rdx, qword ptr [rax + 0x10]
0x14068b09f call 0x140613c20
0x14068b0a4 mov rax, rdi
0x14068b0a7 cmp rdi, rsi
0x14068b0aa jne 0x14068b090
0x14068b0ac movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b0b4 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b0bc movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x14068b0c6 movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068b0cf movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b0d8 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x14068b0df movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b0e5 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b0eb movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b0f1 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b0f7 add rsp, 0xd8
0x14068b0fe pop rbx
0x14068b0ff pop rdi
0x14068b100 pop rsi
0x14068b101 pop r12
0x14068b103 pop r13
0x14068b105 pop r14
0x14068b107 pop r15
0x14068b109 pop rbp
0x14068b10a ret 
0x14068b10b nop dword ptr [rax + rax]
0x14068b110 mov qword ptr [rsp + 0x10], rdx
0x14068b115 push rbp
0x14068b116 push r15
0x14068b118 push r14
0x14068b11a push r13
0x14068b11c push r12
0x14068b11e push rsi
0x14068b11f push rdi
0x14068b120 push rbx
0x14068b121 sub rsp, 0xd8
0x14068b128 lea rbp, [rdx + 0x80]
0x14068b12f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b135 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b13b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b141 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b147 movdqa xmmword ptr [rsp + 0x70], xmm11
0x14068b14e movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b157 movaps xmmword ptr [rsp + 0x90], xmm9
0x14068b160 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x14068b16a movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b172 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b17a mov rax, qword ptr [rbp - 0x18]
0x14068b17e test rax, rax
0x14068b181 je 0x14068b196
0x14068b183 mov rcx, qword ptr [rbp - 0x10]
0x14068b187 imul rdx, rax, 0x38
0x14068b18b mov r8d, 8
0x14068b191 call 0x140613c20
0x14068b196 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b19e movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b1a6 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x14068b1b0 movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068b1b9 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b1c2 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x14068b1c9 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b1cf movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b1d5 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b1db movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b1e1 add rsp, 0xd8
0x14068b1e8 pop rbx
0x14068b1e9 pop rdi
0x14068b1ea pop rsi
0x14068b1eb pop r12
0x14068b1ed pop r13
0x14068b1ef pop r14
0x14068b1f1 pop r15
0x14068b1f3 pop rbp
0x14068b1f4 ret 
0x14068b1f5 nop word ptr cs:[rax + rax]
0x14068b200 mov qword ptr [rsp + 0x10], rdx
0x14068b205 push rbp
0x14068b206 push r15
0x14068b208 push r14
0x14068b20a push r13
0x14068b20c push r12
0x14068b20e push rsi
0x14068b20f push rdi
0x14068b210 push rbx
0x14068b211 sub rsp, 0xd8
0x14068b218 lea rbp, [rdx + 0x80]
0x14068b21f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b225 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b22b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b231 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b237 movdqa xmmword ptr [rsp + 0x70], xmm11
0x14068b23e movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b247 movaps xmmword ptr [rsp + 0x90], xmm9
0x14068b250 movdqa xmmword ptr [rsp + 0xa0], xmm8
0x14068b25a movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b262 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b26a mov rax, qword ptr [rbp + 0xd0]
0x14068b271 test rax, rax
0x14068b274 je 0x14068b28c
0x14068b276 imul rdx, rax, 0x38
0x14068b27a mov r8d, 8
0x14068b280 mov rcx, qword ptr [rbp + 0xf0]
0x14068b287 call 0x140613c20
0x14068b28c movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b294 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b29c movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x14068b2a6 movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068b2af movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b2b8 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x14068b2bf movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b2c5 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b2cb movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b2d1 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b2d7 add rsp, 0xd8
0x14068b2de pop rbx
0x14068b2df pop rdi
0x14068b2e0 pop rsi
0x14068b2e1 pop r12
0x14068b2e3 pop r13
0x14068b2e5 pop r14
0x14068b2e7 pop r15
0x14068b2e9 pop rbp
0x14068b2ea ret 
0x14068b2eb nop dword ptr [rax + rax]
0x14068b2f0 mov qword ptr [rsp + 0x10], rdx
0x14068b2f5 push rbp
0x14068b2f6 push r15
0x14068b2f8 push r14
0x14068b2fa push r13
0x14068b2fc push r12
0x14068b2fe push rsi
0x14068b2ff push rdi
0x14068b300 push rbx
0x14068b301 sub rsp, 0xd8
0x14068b308 lea rbp, [rdx + 0x80]
0x14068b30f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b315 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b31b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b321 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b327 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b32d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b336 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b340 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b349 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b351 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b359 mov rdx, qword ptr [rbp + 8]
0x14068b35d test rdx, rdx
0x14068b360 je 0x14068b375
0x14068b362 mov rcx, qword ptr [rbp + 0x10]
0x14068b366 shl rdx, 3
0x14068b36a mov r8d, 8
0x14068b370 call 0x140613c20
0x14068b375 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b37d movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b385 movdqa xmm8, xmmword ptr [rsp + 0xa0]
0x14068b38f movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068b398 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b3a1 movdqa xmm11, xmmword ptr [rsp + 0x70]
0x14068b3a8 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b3ae movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b3b4 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b3ba movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b3c0 add rsp, 0xd8
0x14068b3c7 pop rbx
0x14068b3c8 pop rdi
0x14068b3c9 pop rsi
0x14068b3ca pop r12
0x14068b3cc pop r13
0x14068b3ce pop r14
0x14068b3d0 pop r15
0x14068b3d2 pop rbp
0x14068b3d3 ret 
0x14068b3d4 nop word ptr cs:[rax + rax]
0x14068b3e0 mov qword ptr [rsp + 0x10], rdx
0x14068b3e5 push rbp
0x14068b3e6 push r15
0x14068b3e8 push r14
0x14068b3ea push r13
0x14068b3ec push r12
0x14068b3ee push rsi
0x14068b3ef push rdi
0x14068b3f0 push rbx
0x14068b3f1 sub rsp, 0xd8
0x14068b3f8 lea rbp, [rdx + 0x80]
0x14068b3ff movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b405 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b40b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b411 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b417 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b41d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b426 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b430 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b439 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b441 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b449 mov rax, qword ptr [rbp + 0xa0]
0x14068b450 test rax, rax
0x14068b453 je 0x14068b46f
0x14068b455 shl rax, 3
0x14068b459 lea rdx, [rax + rax*2]
0x14068b45d mov r8d, 8
0x14068b463 mov rcx, qword ptr [rbp + 0xd8]
0x14068b46a call 0x140613c20
0x14068b46f movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b477 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b47f movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b488 movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068b491 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b49a movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b4a0 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b4a6 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b4ac movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b4b2 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b4b8 add rsp, 0xd8
0x14068b4bf pop rbx
0x14068b4c0 pop rdi
0x14068b4c1 pop rsi
0x14068b4c2 pop r12
0x14068b4c4 pop r13
0x14068b4c6 pop r14
0x14068b4c8 pop r15
0x14068b4ca pop rbp
0x14068b4cb ret 
0x14068b4cc nop dword ptr [rax]
0x14068b4d0 mov qword ptr [rsp + 0x10], rdx
0x14068b4d5 push rbp
0x14068b4d6 push r15
0x14068b4d8 push r14
0x14068b4da push r13
0x14068b4dc push r12
0x14068b4de push rsi
0x14068b4df push rdi
0x14068b4e0 push rbx
0x14068b4e1 sub rsp, 0xd8
0x14068b4e8 lea rbp, [rdx + 0x80]
0x14068b4ef movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b4f5 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b4fb movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b501 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b507 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b50d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b516 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b520 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b529 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b531 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b539 mov rdx, qword ptr [rbp + 0x98]
0x14068b540 test rdx, rdx
0x14068b543 je 0x14068b55b
0x14068b545 shl rdx, 4
0x14068b549 mov r8d, 4
0x14068b54f mov rcx, qword ptr [rbp + 0xc0]
0x14068b556 call 0x140613c20
0x14068b55b mov byte ptr [rbp + 0xe7], 0
0x14068b562 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b56a movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b572 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b57b movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068b585 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b58e movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b594 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b59a movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b5a0 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b5a6 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b5ac add rsp, 0xd8
0x14068b5b3 pop rbx
0x14068b5b4 pop rdi
0x14068b5b5 pop rsi
0x14068b5b6 pop r12
0x14068b5b8 pop r13
0x14068b5ba pop r14
0x14068b5bc pop r15
0x14068b5be pop rbp
0x14068b5bf ret 
0x14068b5c0 mov qword ptr [rsp + 0x10], rdx
0x14068b5c5 push rbp
0x14068b5c6 push r15
0x14068b5c8 push r14
0x14068b5ca push r13
0x14068b5cc push r12
0x14068b5ce push rsi
0x14068b5cf push rdi
0x14068b5d0 push rbx
0x14068b5d1 sub rsp, 0xd8
0x14068b5d8 lea rbp, [rdx + 0x80]
0x14068b5df movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b5e5 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b5eb movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b5f1 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b5f7 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b5fd movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b606 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b610 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b619 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b621 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b629 cmp byte ptr [rbp + 0xe7], 0
0x14068b630 je 0x14068b6ac
0x14068b632 mov rdx, qword ptr [rbp + 0x40]
0x14068b636 test rdx, rdx
0x14068b639 je 0x14068b64e
0x14068b63b mov rcx, qword ptr [rbp + 0x48]
0x14068b63f shl rdx, 4
0x14068b643 mov r8d, 4
0x14068b649 call 0x140613c20
0x14068b64e movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b656 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b65e movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b667 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068b671 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b67a movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b680 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b686 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b68c movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b692 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b698 add rsp, 0xd8
0x14068b69f pop rbx
0x14068b6a0 pop rdi
0x14068b6a1 pop rsi
0x14068b6a2 pop r12
0x14068b6a4 pop r13
0x14068b6a6 pop r14
0x14068b6a8 pop r15
0x14068b6aa pop rbp
0x14068b6ab ret 
0x14068b6ac movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b6b4 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b6bc movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b6c5 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068b6cf movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b6d8 movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b6de movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b6e4 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b6ea movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b6f0 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b6f6 add rsp, 0xd8
0x14068b6fd pop rbx
0x14068b6fe pop rdi
0x14068b6ff pop rsi
0x14068b700 pop r12
0x14068b702 pop r13
0x14068b704 pop r14
0x14068b706 pop r15
0x14068b708 pop rbp
0x14068b709 ret 
0x14068b70a nop word ptr [rax + rax]
0x14068b710 mov qword ptr [rsp + 0x10], rdx
0x14068b715 push rbp
0x14068b716 push r15
0x14068b718 push r14
0x14068b71a push r13
0x14068b71c push r12
0x14068b71e push rsi
0x14068b71f push rdi
0x14068b720 push rbx
0x14068b721 sub rsp, 0xd8
0x14068b728 lea rbp, [rdx + 0x80]
0x14068b72f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b735 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b73b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b741 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b747 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b74d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b756 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b760 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b769 movdqa xmmword ptr [rsp + 0xb0], xmm7
0x14068b772 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b77a cmp qword ptr [rbp + 0xd8], 0
0x14068b782 je 0x14068b7a1
0x14068b784 mov rdx, qword ptr [rbp + 0xd8]
0x14068b78b shl rdx, 3
0x14068b78f mov r8d, 8
0x14068b795 mov rcx, qword ptr [rbp + 0xf0]
0x14068b79c call 0x140613c20
0x14068b7a1 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b7a9 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b7b1 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b7ba movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068b7c3 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b7cc movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b7d2 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b7d8 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b7de movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b7e4 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b7ea add rsp, 0xd8
0x14068b7f1 pop rbx
0x14068b7f2 pop rdi
0x14068b7f3 pop rsi
0x14068b7f4 pop r12
0x14068b7f6 pop r13
0x14068b7f8 pop r14
0x14068b7fa pop r15
0x14068b7fc pop rbp
0x14068b7fd ret 
0x14068b7fe nop 
0x14068b800 mov qword ptr [rsp + 0x10], rdx
0x14068b805 push rbp
0x14068b806 push r15
0x14068b808 push r14
0x14068b80a push r13
0x14068b80c push r12
0x14068b80e push rsi
0x14068b80f push rdi
0x14068b810 push rbx
0x14068b811 sub rsp, 0xd8
0x14068b818 lea rbp, [rdx + 0x80]
0x14068b81f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b825 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b82b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b831 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b837 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b83d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b846 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b850 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b859 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b861 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b869 cmp qword ptr [rbp + 0xc8], 0
0x14068b871 je 0x14068b890
0x14068b873 mov rdx, qword ptr [rbp + 0xc8]
0x14068b87a shl rdx, 3
0x14068b87e mov r8d, 8
0x14068b884 mov rcx, qword ptr [rbp + 0xf0]
0x14068b88b call 0x140613c20
0x14068b890 mov byte ptr [rbp + 0x107], 0
0x14068b897 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b89f movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b8a7 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b8b0 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068b8ba movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b8c3 movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b8c9 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b8cf movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b8d5 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b8db movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b8e1 add rsp, 0xd8
0x14068b8e8 pop rbx
0x14068b8e9 pop rdi
0x14068b8ea pop rsi
0x14068b8eb pop r12
0x14068b8ed pop r13
0x14068b8ef pop r14
0x14068b8f1 pop r15
0x14068b8f3 pop rbp
0x14068b8f4 ret 
0x14068b8f5 nop word ptr cs:[rax + rax]
0x14068b900 mov qword ptr [rsp + 0x10], rdx
0x14068b905 push rbp
0x14068b906 push r15
0x14068b908 push r14
0x14068b90a push r13
0x14068b90c push r12
0x14068b90e push rsi
0x14068b90f push rdi
0x14068b910 push rbx
0x14068b911 sub rsp, 0xd8
0x14068b918 lea rbp, [rdx + 0x80]
0x14068b91f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068b925 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068b92b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068b931 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068b937 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068b93d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068b946 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068b950 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068b959 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068b961 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068b969 mov rdx, qword ptr [rbp + 0xa0]
0x14068b970 test rdx, rdx
0x14068b973 je 0x14068b98b
0x14068b975 mov rcx, qword ptr [rbp + 0xa8]
0x14068b97c shl rdx, 2
0x14068b980 mov r8d, 4
0x14068b986 call 0x140613c20
0x14068b98b movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068b993 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068b99b movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068b9a4 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068b9ae movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068b9b7 movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068b9bd movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068b9c3 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068b9c9 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068b9cf movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068b9d5 add rsp, 0xd8
0x14068b9dc pop rbx
0x14068b9dd pop rdi
0x14068b9de pop rsi
0x14068b9df pop r12
0x14068b9e1 pop r13
0x14068b9e3 pop r14
0x14068b9e5 pop r15
0x14068b9e7 pop rbp
0x14068b9e8 ret 
0x14068b9e9 nop dword ptr [rax]
0x14068b9f0 mov qword ptr [rsp + 0x10], rdx
0x14068b9f5 push rbp
0x14068b9f6 push r15
0x14068b9f8 push r14
0x14068b9fa push r13
0x14068b9fc push r12
0x14068b9fe push rsi
0x14068b9ff push rdi
0x14068ba00 push rbx
0x14068ba01 sub rsp, 0xd8
0x14068ba08 lea rbp, [rdx + 0x80]
0x14068ba0f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068ba15 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068ba1b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068ba21 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068ba27 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068ba2d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068ba36 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068ba40 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068ba49 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068ba51 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068ba59 mov rax, qword ptr [rbp + 0xf0]
0x14068ba60 mov rcx, rax
0x14068ba63 movzx eax, byte ptr [rax]
0x14068ba66 cmp al, 0x1b
0x14068ba68 je 0x14068ba81
0x14068ba6a cmp al, 0x18
0x14068ba6c jne 0x14068ba81
0x14068ba6e mov rax, qword ptr [rcx + 8]
0x14068ba72 lock dec qword ptr [rax]
0x14068ba76 jne 0x14068ba81
0x14068ba78 add rcx, 8
0x14068ba7c call 0x14064eb30
0x14068ba81 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068ba89 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068ba91 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068ba9a movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068baa4 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068baad movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068bab3 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068bab9 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068babf movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068bac5 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068bacb add rsp, 0xd8
0x14068bad2 pop rbx
0x14068bad3 pop rdi
0x14068bad4 pop rsi
0x14068bad5 pop r12
0x14068bad7 pop r13
0x14068bad9 pop r14
0x14068badb pop r15
0x14068badd pop rbp
0x14068bade ret 
0x14068badf nop 
0x14068bae0 mov qword ptr [rsp + 0x10], rdx
0x14068bae5 push rbp
0x14068bae6 push r15
0x14068bae8 push r14
0x14068baea push r13
0x14068baec push r12
0x14068baee push rsi
0x14068baef push rdi
0x14068baf0 push rbx
0x14068baf1 sub rsp, 0xd8
0x14068baf8 lea rbp, [rdx + 0x80]
0x14068baff movaps xmmword ptr [rsp + 0x30], xmm15
0x14068bb05 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068bb0b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068bb11 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068bb17 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068bb1d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068bb26 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068bb30 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068bb39 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068bb41 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068bb49 mov rax, qword ptr [rbp + 0x90]
0x14068bb50 cmp rax, qword ptr [rbp + 0xe8]
0x14068bb57 je 0x14068bb87
0x14068bb59 mov rsi, qword ptr [rbp + 0x90]
0x14068bb60 imul rcx, rsi, 0x70
0x14068bb64 add rcx, qword ptr [rbp + 0x28]
0x14068bb68 mov byte ptr [rbp + 0x107], 0
0x14068bb6f call 0x140682070
0x14068bb74 inc rsi
0x14068bb77 mov qword ptr [rbp + 0x90], rsi
0x14068bb7e cmp rsi, qword ptr [rbp + 0xe8]
0x14068bb85 jne 0x14068bb59
0x14068bb87 mov byte ptr [rbp + 0x107], 0
0x14068bb8e movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068bb96 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068bb9e movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068bba7 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068bbb1 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068bbba movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068bbc0 movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068bbc6 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068bbcc movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068bbd2 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068bbd8 add rsp, 0xd8
0x14068bbdf pop rbx
0x14068bbe0 pop rdi
0x14068bbe1 pop rsi
0x14068bbe2 pop r12
0x14068bbe4 pop r13
0x14068bbe6 pop r14
0x14068bbe8 pop r15
0x14068bbea pop rbp
0x14068bbeb ret 
0x14068bbec nop dword ptr [rax]
0x14068bbf0 mov qword ptr [rsp + 0x10], rdx
0x14068bbf5 push rbp
0x14068bbf6 push r15
0x14068bbf8 push r14
0x14068bbfa push r13
0x14068bbfc push r12
0x14068bbfe push rsi
0x14068bbff push rdi
0x14068bc00 push rbx
0x14068bc01 sub rsp, 0xd8
0x14068bc08 lea rbp, [rdx + 0x80]
0x14068bc0f movaps xmmword ptr [rsp + 0x30], xmm15
0x14068bc15 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068bc1b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068bc21 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068bc27 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068bc2d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068bc36 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068bc40 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068bc49 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068bc51 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068bc59 cmp byte ptr [rbp + 0x107], 0
0x14068bc60 je 0x14068bc7e
0x14068bc62 mov rdx, qword ptr [rbp + 8]
0x14068bc66 test rdx, rdx
0x14068bc69 je 0x14068bc7e
0x14068bc6b mov rcx, qword ptr [rbp + 0x10]
0x14068bc6f shl rdx, 3
0x14068bc73 mov r8d, 8
0x14068bc79 call 0x140613c20
0x14068bc7e movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068bc86 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068bc8e movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068bc97 movaps xmm9, xmmword ptr [rsp + 0x90]
0x14068bca0 movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068bca9 movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068bcaf movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068bcb5 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068bcbb movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068bcc1 movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068bcc7 add rsp, 0xd8
0x14068bcce pop rbx
0x14068bccf pop rdi
0x14068bcd0 pop rsi
0x14068bcd1 pop r12
0x14068bcd3 pop r13
0x14068bcd5 pop r14
0x14068bcd7 pop r15
0x14068bcd9 pop rbp
0x14068bcda ret 
0x14068bcdb nop dword ptr [rax + rax]
0x14068bce0 mov qword ptr [rsp + 0x10], rdx
0x14068bce5 push rbp
0x14068bce6 push r15
0x14068bce8 push r14
0x14068bcea push r13
0x14068bcec push r12
0x14068bcee push rsi
0x14068bcef push rdi
0x14068bcf0 push rbx
0x14068bcf1 sub rsp, 0xd8
0x14068bcf8 lea rbp, [rdx + 0x80]
0x14068bcff movaps xmmword ptr [rsp + 0x30], xmm15
0x14068bd05 movaps xmmword ptr [rsp + 0x40], xmm14
0x14068bd0b movaps xmmword ptr [rsp + 0x50], xmm13
0x14068bd11 movaps xmmword ptr [rsp + 0x60], xmm12
0x14068bd17 movaps xmmword ptr [rsp + 0x70], xmm11
0x14068bd1d movaps xmmword ptr [rsp + 0x80], xmm10
0x14068bd26 movdqa xmmword ptr [rsp + 0x90], xmm9
0x14068bd30 movaps xmmword ptr [rsp + 0xa0], xmm8
0x14068bd39 movaps xmmword ptr [rsp + 0xb0], xmm7
0x14068bd41 movaps xmmword ptr [rsp + 0xc0], xmm6
0x14068bd49 lea rcx, [rbp + 0x40]
0x14068bd4d call 0x1405f11f0
0x14068bd52 mov byte ptr [rbp + 0x107], 0
0x14068bd59 movaps xmm6, xmmword ptr [rsp + 0xc0]
0x14068bd61 movaps xmm7, xmmword ptr [rsp + 0xb0]
0x14068bd69 movaps xmm8, xmmword ptr [rsp + 0xa0]
0x14068bd72 movdqa xmm9, xmmword ptr [rsp + 0x90]
0x14068bd7c movaps xmm10, xmmword ptr [rsp + 0x80]
0x14068bd85 movaps xmm11, xmmword ptr [rsp + 0x70]
0x14068bd8b movaps xmm12, xmmword ptr [rsp + 0x60]
0x14068bd91 movaps xmm13, xmmword ptr [rsp + 0x50]
0x14068bd97 movaps xmm14, xmmword ptr [rsp + 0x40]
0x14068bd9d movaps xmm15, xmmword ptr [rsp + 0x30]
0x14068bda3 add rsp, 0xd8
0x14068bdaa pop rbx
0x14068bdab pop rdi
0x14068bdac pop rsi
0x14068bdad pop r12
0x14068bdaf pop r13
0x14068bdb1 pop r14
0x14068bdb3 pop r15
0x14068bdb5 pop rbp
0x14068bdb6 ret 
