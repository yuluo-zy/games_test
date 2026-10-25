140755690 push rbp
140755691 push r15
140755693 push r14
140755695 push r13
140755697 push r12
140755699 push rsi
14075569a push rdi
14075569b push rbx
14075569c sub rsp, 0x158
1407556a3 lea rbp, [rsp + 0x80]
1407556ab movdqa xmmword ptr [rbp + 0xc0], xmm6
1407556b3 mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1407556be mov r15, qword ptr [rcx]
1407556c1 mov r10, qword ptr [rcx + 8]
1407556c5 mov r11, qword ptr [r15]
1407556c8 xor eax, eax
1407556ca mov rcx, r11
1407556cd sub rcx, qword ptr [r10 + 0x18]
1407556d1 cmovb rcx, rax
1407556d5 sub r11, qword ptr [r10 + 0x38]
1407556d9 cmovb r11, rax
1407556dd mov r14, rcx
1407556e0 shl r14, 4
1407556e4 add r14, qword ptr [r10 + 8]
1407556e8 mov rsi, qword ptr [r10 + 0x10]
1407556ec mov rdi, qword ptr [r10 + 0x30]
1407556f0 sub rsi, rcx
1407556f3 cmovb rsi, rax
1407556f7 mov ecx, 8
1407556fc cmovb r14, rcx
140755700 mov rbx, r11
140755703 shl rbx, 4
140755707 add rbx, qword ptr [r10 + 0x28]
14075570b sub rdi, r11
14075570e cmovb rdi, rax
140755712 cmovb rbx, rcx
140755716 mov r10, qword ptr [r10 + 0x40]
14075571a sub r10, rdi
14075571d sub r10, rsi
140755720 mov qword ptr [rbp + 0xb0], r15
140755727 mov qword ptr [r15], r10
14075572a shl rsi, 4
14075572e mov qword ptr [rbp + 0x98], r14
140755735 add rsi, r14
140755738 mov qword ptr [rbp - 0x28], rsi
14075573c shl rdi, 4
140755740 mov qword ptr [rbp], rbx
140755744 add rdi, rbx
140755747 mov qword ptr [rbp - 0x18], rdi
14075574b mov rsi, qword ptr [rdx]
14075574e mov rdx, qword ptr [rdx + 8]
140755752 mov r10, qword ptr [rsi]
140755755 mov r11, r10
140755758 sub r11, qword ptr [rdx + 0x18]
14075575c cmovb r11, rax
140755760 sub r10, qword ptr [rdx + 0x38]
140755764 cmovb r10, rax
140755768 mov r13, r11
14075576b shl r13, 6
14075576f add r13, qword ptr [rdx + 8]
140755773 mov rdi, qword ptr [rdx + 0x10]
140755777 sub rdi, r11
14075577a cmovb rdi, rax
14075577e cmovb r13, rcx
140755782 mov r12, r10
140755785 shl r12, 6
140755789 add r12, qword ptr [rdx + 0x28]
14075578d mov r11, qword ptr [rdx + 0x30]
140755791 sub r11, r10
140755794 cmovb r11, rax
140755798 cmovb r12, rcx
14075579c mov rdx, qword ptr [rdx + 0x40]
1407557a0 sub rdx, r11
1407557a3 sub rdx, rdi
1407557a6 mov qword ptr [rbp + 8], rsi
1407557aa mov qword ptr [rsi], rdx
1407557ad shl rdi, 6
1407557b1 add rdi, r13
1407557b4 mov qword ptr [rbp - 0x20], rdi
1407557b8 shl r11, 6
1407557bc add r11, r12
1407557bf mov qword ptr [rbp - 0x10], r11
1407557c3 mov r11, qword ptr [r8]
1407557c6 mov rdx, qword ptr [r8 + 8]
1407557ca mov r8, qword ptr [r11]
1407557cd mov r10, r8
1407557d0 sub r10, qword ptr [rdx + 0x18]
1407557d4 cmovb r10, rax
1407557d8 sub r8, qword ptr [rdx + 0x38]
1407557dc cmovb r8, rax
1407557e0 mov rbx, r10
1407557e3 shl rbx, 5
1407557e7 add rbx, qword ptr [rdx + 8]
1407557eb mov rsi, qword ptr [rdx + 0x10]
1407557ef sub rsi, r10
1407557f2 cmovb rsi, rax
1407557f6 cmovb rbx, rcx
1407557fa mov rdi, r8
1407557fd shl rdi, 5
140755801 add rdi, qword ptr [rdx + 0x28]
140755805 mov r10, qword ptr [rdx + 0x30]
140755809 sub r10, r8
14075580c cmovb r10, rax
140755810 cmovb rdi, rcx
140755814 mov qword ptr [rbp - 8], r9
140755818 mov rax, qword ptr [rdx + 0x40]
14075581c sub rax, r10
14075581f sub rax, rsi
140755822 mov qword ptr [rbp + 0x70], r11
140755826 mov qword ptr [r11], rax
140755829 shl rsi, 5
14075582d mov qword ptr [rbp + 0xa8], rbx
140755834 add rsi, rbx
140755837 mov qword ptr [rbp + 0x28], rsi
14075583b shl r10, 5
14075583f mov qword ptr [rbp + 0x80], rdi
140755846 add r10, rdi
140755849 mov qword ptr [rbp - 0x30], r10
14075584d mov rax, qword ptr [rbp + 0x148]
140755854 mov ecx, dword ptr [rax + 0x1c]
140755857 mov dword ptr [rbp + 0x7c], ecx
14075585a mov rcx, qword ptr [rax]
14075585d mov rax, qword ptr [rax + 0x10]
140755861 mov qword ptr [rbp + 0x10], rax
140755865 mov rax, qword ptr [rbp + 0x150]
14075586c mov rdx, qword ptr [rax]
14075586f mov qword ptr [rbp + 0x18], rdx
140755873 mov rdi, qword ptr [rax + 8]
140755877 mov rsi, qword ptr [rax + 0x18]
14075587b mov r15, qword ptr [rax + 0x10]
14075587f pcmpeqd xmm6, xmm6
140755883 mov qword ptr [rbp + 0x88], rcx
14075588a lea rax, [rcx + 0x20]
14075588e mov qword ptr [rbp + 0x68], rax
140755892 mov r10b, 1
140755895 mov qword ptr [rbp + 0x20], r15
140755899 test r10b, 1
14075589d jne 0x140755900
14075589f jmp 0x1407558bd
1407558a1 nop word ptr cs:[rax + rax]
1407558b0 mov r10, qword ptr [rbp + 0xa0]
1407558b7 test r10b, 1
1407558bb jne 0x140755900
1407558bd mov rcx, qword ptr [rbp + 0x80]
1407558c4 mov rdx, qword ptr [rbp + 0xa8]
1407558cb test rdx, rdx
1407558ce je 0x140755a30
1407558d4 cmp rdx, qword ptr [rbp + 0x28]
1407558d8 je 0x140755a30
1407558de mov rax, rdx
1407558e1 add rdx, 0x20
1407558e5 mov qword ptr [rbp + 0xa0], 0
1407558f0 mov rcx, qword ptr [rbp + 0x70]
1407558f4 mov qword ptr [rbp + 0xa8], rdx
1407558fb jmp 0x140755a6b
140755900 cmp qword ptr [rbp + 0xb0], 0
140755908 je 0x140755940
14075590a mov rcx, qword ptr [rbp + 0x98]
140755911 test rcx, rcx
140755914 je 0x140755980
140755916 cmp rcx, qword ptr [rbp - 0x28]
14075591a je 0x140755980
14075591c mov al, 1
14075591e mov qword ptr [rbp + 0xa0], rax
140755925 mov rax, rcx
140755928 lea rdx, [rcx + 0x10]
14075592c mov rcx, qword ptr [rbp + 0xb0]
140755933 mov qword ptr [rbp + 0x98], rdx
14075593a jmp 0x140755a6b
14075593f nop 
140755940 mov rax, qword ptr [rbp + 0x98]
140755947 test r13, r13
14075594a mov qword ptr [rbp + 0x98], rax
140755951 je 0x1407559d0
140755953 cmp r13, qword ptr [rbp - 0x20]
140755957 je 0x1407559d0
140755959 mov cl, 1
14075595b mov qword ptr [rbp + 0xa0], rcx
140755962 mov rax, r13
140755965 add r13, 0x40
140755969 mov rcx, qword ptr [rbp + 8]
14075596d mov qword ptr [rbp + 0xb0], 0
140755978 jmp 0x140755a6b
14075597d nop dword ptr [rax]
140755980 mov rcx, qword ptr [rbp]
140755984 test rcx, rcx
140755987 je 0x140755cea
14075598d mov eax, 0
140755992 cmp rcx, qword ptr [rbp - 0x18]
140755996 je 0x140755947
140755998 mov al, 1
14075599a mov qword ptr [rbp + 0xa0], rax
1407559a1 mov rax, rcx
1407559a4 lea rdx, [rcx + 0x10]
1407559a8 mov rcx, qword ptr [rbp + 0xb0]
1407559af mov qword ptr [rbp], rdx
1407559b3 mov qword ptr [rbp + 0x98], 0
1407559be jmp 0x140755a6b
1407559c3 nop word ptr cs:[rax + rax]
1407559d0 test r12, r12
1407559d3 je 0x1407559ff
1407559d5 cmp r12, qword ptr [rbp - 0x10]
1407559d9 je 0x1407559ff
1407559db mov cl, 1
1407559dd mov qword ptr [rbp + 0xa0], rcx
1407559e4 mov rax, r12
1407559e7 add r12, 0x40
1407559eb mov rcx, qword ptr [rbp + 8]
1407559ef xor r13d, r13d
1407559f2 mov qword ptr [rbp + 0xb0], 0
1407559fd jmp 0x140755a6b
1407559ff xor r13d, r13d
140755a02 mov qword ptr [rbp + 0xb0], 0
140755a0d mov rcx, qword ptr [rbp + 0x80]
140755a14 mov rdx, qword ptr [rbp + 0xa8]
140755a1b test rdx, rdx
140755a1e jne 0x1407558d4
140755a24 nop word ptr cs:[rax + rax]
140755a30 test rcx, rcx
140755a33 je 0x140755d6d
140755a39 cmp rcx, qword ptr [rbp - 0x30]
140755a3d je 0x140755d6d
140755a43 mov rax, rcx
140755a46 lea rdx, [rcx + 0x20]
140755a4a mov qword ptr [rbp + 0xa0], 0
140755a55 mov rcx, qword ptr [rbp + 0x70]
140755a59 mov qword ptr [rbp + 0xa8], 0
140755a64 mov qword ptr [rbp + 0x80], rdx
140755a6b inc qword ptr [rcx]
140755a6e mov rdx, qword ptr [rax + 8]
140755a72 mov eax, dword ptr [rbp + 0x7c]
140755a75 mov rcx, qword ptr [rbp + 0x10]
140755a79 mov dword ptr [rcx], eax
140755a7b mov qword ptr [rbp + 0x90], rdx
140755a82 mov qword ptr [rbp + 0x38], rdx
140755a86 mov rcx, qword ptr [rbp + 0x68]
140755a8a lea rdx, [rbp + 0x38]
140755a8e call 0x1405b0d70
140755a93 mov r15, rax
140755a96 shr r15, 0x39
140755a9a mov rdx, qword ptr [rbp + 0x88]
140755aa1 mov rcx, qword ptr [rdx]
140755aa4 mov rbx, qword ptr [rdx + 8]
140755aa8 mov r8, rbx
140755aab mov qword ptr [rbp + 0x60], rax
140755aaf and r8, rax
140755ab2 movd xmm0, r15d
140755ab7 punpcklbw xmm0, xmm0
140755abb pshuflw xmm0, xmm0, 0
140755ac0 pshufd xmm0, xmm0, 0
140755ac5 lea r9, [rcx - 0x20]
140755ac9 xor r10d, r10d
140755acc mov rax, r8
140755acf movdqu xmm1, xmmword ptr [rcx + rax]
140755ad4 movdqa xmm2, xmm1
140755ad8 pcmpeqb xmm2, xmm0
140755adc pmovmskb r14d, xmm2
140755ae1 test r14d, r14d
140755ae4 je 0x140755b50
140755ae6 mov qword ptr [rbp + 0x30], r8
140755aea mov r8, r10
140755aed mov r11, r15
140755af0 mov rdx, rsi
140755af3 mov rsi, rdi
140755af6 mov rdi, r13
140755af9 mov r13, r12
140755afc tzcnt r10d, r14d
140755b01 add r10, rax
140755b04 and r10, rbx
140755b07 mov r12, r10
140755b0a shl r12, 5
140755b0e mov r15, rbx
140755b11 mov rbx, r9
140755b14 sub r9, r12
140755b17 mov r12, qword ptr [rbp + 0x90]
140755b1e cmp qword ptr [r9], r12
140755b21 je 0x140755b80
140755b23 lea r9d, [r14 - 1]
140755b27 and r9w, r14w
140755b2b mov r14d, r9d
140755b2e mov r12, r13
140755b31 mov r13, rdi
140755b34 mov rdi, rsi
140755b37 mov rsi, rdx
140755b3a mov r9, rbx
140755b3d mov rbx, r15
140755b40 mov r15, r11
140755b43 mov r10, r8
140755b46 mov r8, qword ptr [rbp + 0x30]
140755b4a jne 0x140755aea
140755b4c nop dword ptr [rax]
140755b50 mov rdx, r9
140755b53 pcmpeqb xmm1, xmm6
140755b57 pmovmskb r9d, xmm1
140755b5c test r9d, r9d
140755b5f jne 0x140755c40
140755b65 add rax, r10
140755b68 add rax, 0x10
140755b6c add r10, 0x10
140755b70 and rax, rbx
140755b73 mov r9, rdx
140755b76 jmp 0x140755acf
140755b7b nop dword ptr [rax + rax]
140755b80 neg r10
140755b83 shl r10, 5
140755b87 lea rax, [rcx + r10]
140755b8b add rax, -0x18
140755b8f mov r8, qword ptr [rcx + r10 - 0x10]
140755b94 mov rbx, qword ptr [rcx + r10 - 8]
140755b99 mov qword ptr [rcx + r10 - 8], 0
140755ba2 lea rcx, [r8 + rbx*8]
140755ba6 mov qword ptr [rbp - 0x40], rbx
140755baa mov qword ptr [rbp - 0x38], 0
140755bb2 mov qword ptr [rbp - 0x50], rcx
140755bb6 mov qword ptr [rbp - 0x48], rax
140755bba test rbx, rbx
140755bbd mov r14, qword ptr [rbp + 0x18]
140755bc1 mov r12, r13
140755bc4 mov r13, rdi
140755bc7 mov rdi, rsi
140755bca mov rsi, rdx
140755bcd mov r15, qword ptr [rbp + 0x20]
140755bd1 je 0x1407558b0
140755bd7 add r8, 8
140755bdb shl rbx, 3
140755bdf nop 
140755be0 mov rax, qword ptr [r8 - 8]
140755be4 mov rcx, rax
140755be7 shr rcx, 0x20
140755beb je 0x1407558b0
140755bf1 mov qword ptr [rbp + 0x90], r8
140755bf8 mov qword ptr [rbp + 0x58], rax
140755bfc mov qword ptr [rbp + 0x38], r14
140755c00 mov qword ptr [rbp + 0x40], rdi
140755c04 mov qword ptr [rbp + 0x48], r15
140755c08 mov qword ptr [rbp + 0x50], rsi
140755c0c lea rcx, [rbp + 0x38]
140755c10 lea rdx, [rip + 0x2326ae9]
140755c17 call 0x140d14770
140755c1c mov r8, qword ptr [rbp + 0x90]
140755c23 add r8, 8
140755c27 add rbx, -8
140755c2b jne 0x140755be0
140755c2d jmp 0x1407558b0
140755c32 nop word ptr cs:[rax + rax]
140755c40 mov r9, qword ptr [rbp + 0x88]
140755c47 cmp qword ptr [r9 + 0x10], 0
140755c4c je 0x140755cf1
140755c52 movdqu xmm0, xmmword ptr [rcx + r8]
140755c58 pmovmskb eax, xmm0
140755c5c test eax, eax
140755c5e je 0x140755d2c
140755c64 mov r10, qword ptr [rbp + 0xa0]
140755c6b tzcnt eax, eax
140755c6f add rax, r8
140755c72 and rax, rbx
140755c75 movzx r8d, byte ptr [rcx + rax]
140755c7a test r8b, r8b
140755c7d jns 0x140755d57
140755c83 lea r9, [rax - 0x10]
140755c87 and r9, rbx
140755c8a mov byte ptr [rcx + rax], r15b
140755c8e mov byte ptr [rcx + r9 + 0x10], r15b
140755c93 shl rax, 5
140755c97 mov rdx, rcx
140755c9a sub rdx, rax
140755c9d and r8b, 1
140755ca1 movzx r8d, r8b
140755ca5 mov r9, qword ptr [rbp + 0x88]
140755cac sub qword ptr [r9 + 0x10], r8
140755cb0 neg rax
140755cb3 mov r8, qword ptr [rbp + 0x90]
140755cba mov qword ptr [rcx + rax - 0x20], r8
140755cbf mov qword ptr [rdx - 0x18], 0
140755cc7 mov qword ptr [rdx - 0x10], 8
140755ccf mov qword ptr [rdx - 8], 0
140755cd7 inc qword ptr [r9 + 0x18]
140755cdb test r10b, 1
140755cdf jne 0x140755900
140755ce5 jmp 0x1407558bd
140755cea xor eax, eax
140755cec jmp 0x140755947
140755cf1 mov edx, 1
140755cf6 mov r14, qword ptr [rbp + 0x88]
140755cfd mov rcx, r14
140755d00 mov r8, qword ptr [rbp + 0x68]
140755d04 mov r9b, 1
140755d07 call 0x1428a9ca0
140755d0c mov rcx, qword ptr [r14]
140755d0f mov rbx, qword ptr [r14 + 8]
140755d13 mov r8, qword ptr [rbp + 0x60]
140755d17 and r8, rbx
140755d1a movdqu xmm0, xmmword ptr [rcx + r8]
140755d20 pmovmskb eax, xmm0
140755d24 test eax, eax
140755d26 jne 0x140755c64
140755d2c mov r9d, 0x10
140755d32 mov r10, qword ptr [rbp + 0xa0]
140755d39 add r8, r9
140755d3c and r8, rbx
140755d3f movdqu xmm0, xmmword ptr [rcx + r8]
140755d45 pmovmskb eax, xmm0
140755d49 add r9, 0x10
140755d4d test eax, eax
140755d4f jne 0x140755c6b
140755d55 jmp 0x140755d39
140755d57 movdqa xmm0, xmmword ptr [rcx]
140755d5b pmovmskb eax, xmm0
140755d5f tzcnt eax, eax
140755d63 movzx r8d, byte ptr [rcx + rax]
140755d68 jmp 0x140755c83
140755d6d mov rax, qword ptr [rbp - 8]
140755d71 mov r11, qword ptr [rax]
140755d74 mov rdx, qword ptr [rax + 8]
140755d78 mov r9, qword ptr [r11]
140755d7b xor r8d, r8d
140755d7e mov rcx, r9
140755d81 sub rcx, qword ptr [rdx + 0x18]
140755d85 cmovb rcx, r8
140755d89 sub r9, qword ptr [rdx + 0x38]
140755d8d cmovb r9, r8
140755d91 mov rax, rcx
140755d94 shl rax, 4
140755d98 add rax, qword ptr [rdx + 8]
140755d9c mov rbx, qword ptr [rdx + 0x10]
140755da0 mov r14, qword ptr [rdx + 0x30]
140755da4 sub rbx, rcx
140755da7 cmovb rbx, r8
140755dab mov r10d, 8
140755db1 cmovb rax, r10
140755db5 mov rcx, r9
140755db8 shl rcx, 4
140755dbc add rcx, qword ptr [rdx + 0x28]
140755dc0 sub r14, r9
140755dc3 cmovb r14, r8
140755dc7 cmovb rcx, r10
140755dcb mov rdx, qword ptr [rdx + 0x40]
140755dcf sub rdx, r14
140755dd2 sub rdx, rbx
140755dd5 mov qword ptr [rbp + 0xb0], r11
140755ddc mov qword ptr [r11], rdx
140755ddf shl rbx, 4
140755de3 add rbx, rax
140755de6 mov qword ptr [rbp + 0x60], rbx
140755dea shl r14, 4
140755dee add r14, rcx
140755df1 mov qword ptr [rbp + 0x70], r14
140755df5 pcmpeqd xmm6, xmm6
140755df9 mov qword ptr [rbp + 0x80], rdi
140755e00 mov qword ptr [rbp + 0x30], rsi
140755e04 mov rdi, qword ptr [rbp + 0x18]
140755e08 mov r14, qword ptr [rbp + 0x30]
140755e0c jmp 0x140755e1e
140755e0e nop 
140755e10 mov rax, qword ptr [rbp + 0x98]
140755e17 mov rcx, qword ptr [rbp + 0xa8]
140755e1e test rax, rax
140755e21 je 0x140755e40
140755e23 cmp rax, qword ptr [rbp + 0x60]
140755e27 je 0x140755e40
140755e29 lea rdx, [rax + 0x10]
140755e2d mov qword ptr [rbp + 0x98], rdx
140755e34 mov qword ptr [rbp + 0xa8], rcx
140755e3b jmp 0x140755e6c
140755e3d nop dword ptr [rax]
140755e40 test rcx, rcx
140755e43 je 0x140756033
140755e49 cmp rcx, qword ptr [rbp + 0x70]
140755e4d je 0x140756033
140755e53 lea rax, [rcx + 0x10]
140755e57 mov qword ptr [rbp + 0xa8], rax
140755e5e mov qword ptr [rbp + 0x98], 0
140755e69 mov rax, rcx
140755e6c mov rcx, qword ptr [rbp + 0xb0]
140755e73 inc qword ptr [rcx]
140755e76 mov rbx, qword ptr [rax + 8]
140755e7a mov qword ptr [rbp - 0x58], rbx
140755e7e mov rax, qword ptr [rbp + 0x88]
140755e85 cmp qword ptr [rax + 0x18], 0
140755e8a lea r13, [rbp + 0x38]
140755e8e je 0x140755fd7
140755e94 mov rcx, qword ptr [rbp + 0x68]
140755e98 lea rdx, [rbp - 0x58]
140755e9c call 0x1405b0d70
140755ea1 mov rdx, qword ptr [rbp + 0x88]
140755ea8 mov rcx, qword ptr [rdx]
140755eab mov rdx, qword ptr [rdx + 8]
140755eaf mov r8, rdx
140755eb2 and r8, rax
140755eb5 shr rax, 0x39
140755eb9 movd xmm0, eax
140755ebd punpcklbw xmm0, xmm0
140755ec1 pshuflw xmm0, xmm0, 0
140755ec6 pshufd xmm0, xmm0, 0
140755ecb lea rax, [rcx - 0x20]
140755ecf xor r9d, r9d
140755ed2 movdqu xmm1, xmmword ptr [rcx + r8]
140755ed8 movdqa xmm2, xmm1
140755edc pcmpeqb xmm2, xmm0
140755ee0 pmovmskb r10d, xmm2
140755ee5 test r10d, r10d
140755ee8 je 0x140755f20
140755eea tzcnt r11d, r10d
140755eef add r11, r8
140755ef2 and r11, rdx
140755ef5 mov r15, r11
140755ef8 shl r15, 5
140755efc mov r12, rax
140755eff sub r12, r15
140755f02 cmp rbx, qword ptr [r12]
140755f06 je 0x140755f50
140755f08 lea r11d, [r10 - 1]
140755f0c and r11w, r10w
140755f10 mov r10d, r11d
140755f13 jne 0x140755eea
140755f15 nop word ptr cs:[rax + rax]
140755f20 pcmpeqb xmm1, xmm6
140755f24 pmovmskb r10d, xmm1
140755f29 test r10d, r10d
140755f2c lea r13, [rbp + 0x38]
140755f30 jne 0x140755fd7
140755f36 add r8, r9
140755f39 add r8, 0x10
140755f3d add r9, 0x10
140755f41 and r8, rdx
140755f44 jmp 0x140755ed2
140755f46 nop word ptr cs:[rax + rax]
140755f50 neg r11
140755f53 shl r11, 5
140755f57 mov rax, qword ptr [rcx + r11 - 8]
140755f5c mov qword ptr [rbp + 0x90], rax
140755f63 test rax, rax
140755f66 mov rsi, qword ptr [rbp + 0x80]
140755f6d mov r15, qword ptr [rbp + 0x20]
140755f71 lea r13, [rbp + 0x38]
140755f75 lea rbx, [rip + 0x232676c]
140755f7c je 0x140755fd7
140755f7e mov rax, qword ptr [rcx + r11 - 0x10]
140755f83 mov qword ptr [rbp + 0xa0], rax
140755f8a shl qword ptr [rbp + 0x90], 3
140755f92 xor r12d, r12d
140755f95 nop word ptr cs:[rax + rax]
140755fa0 mov rax, qword ptr [rbp + 0xa0]
140755fa7 mov rax, qword ptr [rax + r12]
140755fab mov qword ptr [rbp + 0x58], rax
140755faf mov qword ptr [rbp + 0x38], rdi
140755fb3 mov qword ptr [rbp + 0x40], rsi
140755fb7 mov qword ptr [rbp + 0x48], r15
140755fbb mov qword ptr [rbp + 0x50], r14
140755fbf mov rcx, r13
140755fc2 mov rdx, rbx
140755fc5 call 0x140d14770
140755fca add r12, 8
140755fce cmp qword ptr [rbp + 0x90], r12
140755fd5 jne 0x140755fa0
140755fd7 mov eax, dword ptr [rbp + 0x7c]
140755fda mov rcx, qword ptr [rbp + 0x10]
140755fde mov dword ptr [rcx], eax
140755fe0 mov rcx, qword ptr [rbp + 0x68]
140755fe4 lea rsi, [rbp - 0x58]
140755fe8 mov rdx, rsi
140755feb call 0x1405b0d70
140755ff0 mov rcx, r13
140755ff3 mov rdx, qword ptr [rbp + 0x88]
140755ffa mov r8, rax
140755ffd mov r9, rsi
140756000 call 0x140b777f0
140756005 mov rdx, qword ptr [rbp + 0x40]
140756009 mov rax, rdx
14075600c neg rax
14075600f jo 0x140755e10
140756015 jae 0x140755e10
14075601b mov rcx, qword ptr [rbp + 0x48]
14075601f shl rdx, 3
140756023 mov r8d, 8
140756029 call 0x140613c20
14075602e jmp 0x140755e10
140756033 mov rax, qword ptr [rbp + 0x140]
14075603a mov rsi, qword ptr [rax]
14075603d mov rcx, qword ptr [rax + 8]
140756041 mov r8, qword ptr [rsi]
140756044 xor edx, edx
140756046 mov rax, r8
140756049 sub rax, qword ptr [rcx + 0x18]
14075604d cmovb rax, rdx
140756051 sub r8, qword ptr [rcx + 0x38]
140756055 cmovb r8, rdx
140756059 lea rdi, [rax + rax*2]
14075605d shl rdi, 3
140756061 add rdi, qword ptr [rcx + 8]
140756065 mov r9, qword ptr [rcx + 0x10]
140756069 mov r10, qword ptr [rcx + 0x30]
14075606d sub r9, rax
140756070 cmovb r9, rdx
140756074 mov r11d, 8
14075607a cmovb rdi, r11
14075607e lea rax, [r8 + r8*2]
140756082 shl rax, 3
140756086 add rax, qword ptr [rcx + 0x28]
14075608a sub r10, r8
14075608d cmovb r10, rdx
140756091 cmovb rax, r11
140756095 mov rcx, qword ptr [rcx + 0x40]
140756099 sub rcx, r10
14075609c sub rcx, r9
14075609f mov qword ptr [rbp + 0x60], rsi
1407560a3 mov qword ptr [rsi], rcx
1407560a6 lea rcx, [r9 + r9*2]
1407560aa lea rcx, [rdi + rcx*8]
1407560ae mov qword ptr [rbp + 0x70], rcx
1407560b2 lea rcx, [r10 + r10*2]
1407560b6 lea rcx, [rax + rcx*8]
1407560ba mov qword ptr [rbp + 0x28], rcx
1407560be pcmpeqd xmm6, xmm6
1407560c2 mov r15, qword ptr [rbp + 0x30]
1407560c6 jmp 0x1407560de
1407560c8 nop dword ptr [rax + rax]
1407560d0 mov rdi, qword ptr [rbp + 0xb0]
1407560d7 mov rax, qword ptr [rbp + 0x98]
1407560de test rdi, rdi
1407560e1 je 0x140756100
1407560e3 cmp rdi, qword ptr [rbp + 0x70]
1407560e7 je 0x140756100
1407560e9 lea rcx, [rdi + 0x18]
1407560ed mov qword ptr [rbp + 0xb0], rcx
1407560f4 mov qword ptr [rbp + 0x98], rax
1407560fb jmp 0x14075612c
1407560fd nop dword ptr [rax]
140756100 test rax, rax
140756103 je 0x1407562fc
140756109 cmp rax, qword ptr [rbp + 0x28]
14075610d je 0x1407562fc
140756113 lea rcx, [rax + 0x18]
140756117 mov qword ptr [rbp + 0x98], rcx
14075611e mov qword ptr [rbp + 0xb0], 0
140756129 mov rdi, rax
14075612c mov rax, qword ptr [rbp + 0x60]
140756130 inc qword ptr [rax]
140756133 add rdi, 8
140756137 mov qword ptr [rbp + 0xa8], rdi
14075613e mov rax, qword ptr [rbp + 0x88]
140756145 cmp qword ptr [rax + 0x18], 0
14075614a lea r13, [rbp + 0x38]
14075614e je 0x14075629d
140756154 mov rcx, qword ptr [rbp + 0x68]
140756158 mov rsi, qword ptr [rbp + 0xa8]
14075615f mov rdx, rsi
140756162 call 0x1405b0d70
140756167 mov rdx, qword ptr [rbp + 0x88]
14075616e mov rcx, qword ptr [rdx]
140756171 mov rdx, qword ptr [rdx + 8]
140756175 mov r8, rdx
140756178 and r8, rax
14075617b shr rax, 0x39
14075617f movd xmm0, eax
140756183 punpcklbw xmm0, xmm0
140756187 pshuflw xmm0, xmm0, 0
14075618c pshufd xmm0, xmm0, 0
140756191 lea rax, [rcx - 0x20]
140756195 xor r9d, r9d
140756198 movdqu xmm1, xmmword ptr [rcx + r8]
14075619e movdqa xmm2, xmm1
1407561a2 pcmpeqb xmm2, xmm0
1407561a6 pmovmskb r10d, xmm2
1407561ab test r10d, r10d
1407561ae je 0x1407561e0
1407561b0 mov r11, qword ptr [rsi]
1407561b3 tzcnt ebx, r10d
1407561b8 add rbx, r8
1407561bb and rbx, rdx
1407561be mov r14, rbx
1407561c1 shl r14, 5
1407561c5 mov r12, rax
1407561c8 sub r12, r14
1407561cb cmp r11, qword ptr [r12]
1407561cf je 0x140756210
1407561d1 lea ebx, [r10 - 1]
1407561d5 and bx, r10w
1407561d9 mov r10d, ebx
1407561dc jne 0x1407561b3
1407561de nop 
1407561e0 pcmpeqb xmm1, xmm6
1407561e4 pmovmskb r10d, xmm1
1407561e9 test r10d, r10d
1407561ec lea r13, [rbp + 0x38]
1407561f0 jne 0x14075629d
1407561f6 add r8, r9
1407561f9 add r8, 0x10
1407561fd add r9, 0x10
140756201 and r8, rdx
140756204 mov rsi, qword ptr [rbp + 0xa8]
14075620b jmp 0x140756198
14075620d nop dword ptr [rax]
140756210 neg rbx
140756213 shl rbx, 5
140756217 mov rax, qword ptr [rcx + rbx - 8]
14075621c mov qword ptr [rbp + 0x90], rax
140756223 test rax, rax
140756226 mov rdi, qword ptr [rbp + 0x18]
14075622a mov rsi, qword ptr [rbp + 0x80]
140756231 mov r10, qword ptr [rbp + 0x20]
140756235 lea r13, [rbp + 0x38]
140756239 lea r14, [rip + 0x2326490]
140756240 je 0x14075629d
140756242 mov rax, qword ptr [rcx + rbx - 0x10]
140756247 mov qword ptr [rbp + 0xa0], rax
14075624e shl qword ptr [rbp + 0x90], 3
140756256 xor r12d, r12d
140756259 nop dword ptr [rax]
140756260 mov rax, qword ptr [rbp + 0xa0]
140756267 mov rax, qword ptr [rax + r12]
14075626b mov qword ptr [rbp + 0x58], rax
14075626f mov qword ptr [rbp + 0x38], rdi
140756273 mov qword ptr [rbp + 0x40], rsi
140756277 mov qword ptr [rbp + 0x48], r10
14075627b mov qword ptr [rbp + 0x50], r15
14075627f mov rcx, r13
140756282 mov rdx, r14
140756285 mov rbx, r10
140756288 call 0x140d14770
14075628d mov r10, rbx
140756290 add r12, 8
140756294 cmp qword ptr [rbp + 0x90], r12
14075629b jne 0x140756260
14075629d mov eax, dword ptr [rbp + 0x7c]
1407562a0 mov rcx, qword ptr [rbp + 0x10]
1407562a4 mov dword ptr [rcx], eax
1407562a6 mov rcx, qword ptr [rbp + 0x68]
1407562aa mov rsi, qword ptr [rbp + 0xa8]
1407562b1 mov rdx, rsi
1407562b4 call 0x1405b0d70
1407562b9 mov rcx, r13
1407562bc mov rdx, qword ptr [rbp + 0x88]
1407562c3 mov r8, rax
1407562c6 mov r9, rsi
1407562c9 call 0x140b777f0
1407562ce mov rdx, qword ptr [rbp + 0x40]
1407562d2 mov rax, rdx
1407562d5 neg rax
1407562d8 jo 0x1407560d0
1407562de jae 0x1407560d0
1407562e4 mov rcx, qword ptr [rbp + 0x48]
1407562e8 shl rdx, 3
1407562ec mov r8d, 8
1407562f2 call 0x140613c20
1407562f7 jmp 0x1407560d0
1407562fc movaps xmm6, xmmword ptr [rbp + 0xc0]
140756303 add rsp, 0x158
14075630a pop rbx
14075630b pop rdi
14075630c pop rsi
14075630d pop r12
14075630f pop r13
140756311 pop r14
140756313 pop r15
140756315 pop rbp
140756316 ret 
