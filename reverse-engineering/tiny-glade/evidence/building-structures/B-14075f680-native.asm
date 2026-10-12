14075f680 push rbp
14075f681 push r15
14075f683 push r14
14075f685 push r13
14075f687 push r12
14075f689 push rsi
14075f68a push rdi
14075f68b push rbx
14075f68c sub rsp, 0x428
14075f693 lea rbp, [rsp + 0x80]
14075f69b movaps xmmword ptr [rbp + 0x390], xmm15
14075f6a3 movaps xmmword ptr [rbp + 0x380], xmm14
14075f6ab movaps xmmword ptr [rbp + 0x370], xmm13
14075f6b3 movaps xmmword ptr [rbp + 0x360], xmm12
14075f6bb movdqa xmmword ptr [rbp + 0x350], xmm11
14075f6c4 movaps xmmword ptr [rbp + 0x340], xmm10
14075f6cc movdqa xmmword ptr [rbp + 0x330], xmm9
14075f6d5 movaps xmmword ptr [rbp + 0x320], xmm8
14075f6dd movaps xmmword ptr [rbp + 0x310], xmm7
14075f6e4 movaps xmmword ptr [rbp + 0x300], xmm6
14075f6eb mov qword ptr [rbp + 0x2f8], 0xfffffffffffffffe
14075f6f6 mov qword ptr [rbp + 0x2d0], r9
14075f6fd mov qword ptr [rbp + 0x2e8], r8
14075f704 mov rbx, rdx
14075f707 mov r14, rcx
14075f70a call 0x140c16a00
14075f70f mov edi, eax
14075f711 test dil, dil
14075f714 je 0x14075f92c
14075f71a mov r15d, 0x3b
14075f720 mov ecx, 0x3b
14075f725 call 0x14075f250
14075f72a lea r12, [rip + 0x231dc67]
14075f731 cmp rax, 1
14075f735 jne 0x14075f7dc
14075f73b mov r9, rdx
14075f73e test rdx, rdx
14075f741 je 0x14075f779
14075f743 cmp r9, 0x3b
14075f747 jae 0x14075f777
14075f749 lea rax, [rip + 0x231dc48]
14075f750 cmp byte ptr [r9 + rax], 0xbf
14075f755 jg 0x14075f779
14075f757 lea rax, [rip + 0x231dcf2]
14075f75e mov qword ptr [rsp + 0x20], rax
14075f763 lea rcx, [rip + 0x231dc2e]
14075f76a mov edx, 0x3b
14075f76f xor r8d, r8d
14075f772 call 0x1428d9940
14075f777 jne 0x14075f757
14075f779 mov rcx, r9
14075f77c call 0x14075f250
14075f781 cmp rax, 1
14075f785 jne 0x14075f7dc
14075f787 mov r8, rdx
14075f78a add r8, 2
14075f78e je 0x14075f7c9
14075f790 cmp r8, 0x3b
14075f794 jae 0x14075f7c7
14075f796 lea rax, [rip + 0x231dbfb]
14075f79d cmp byte ptr [r8 + rax], 0xbf
14075f7a2 jg 0x14075f7c9
14075f7a4 lea rax, [rip + 0x231dcbd]
14075f7ab mov qword ptr [rsp + 0x20], rax
14075f7b0 lea rcx, [rip + 0x231dbe1]
14075f7b7 mov edx, 0x3b
14075f7bc mov r9d, 0x3b
14075f7c2 call 0x1428d9940
14075f7c7 jne 0x14075f7a4
14075f7c9 mov r15d, 0x39
14075f7cf sub r15, rdx
14075f7d2 lea r12, [rip + 0x231dbbf]
14075f7d9 add r12, r8
14075f7dc lea r8, [rip + 0x231dcf0]
14075f7e3 lea r13, [rip + 0x231dcae]
14075f7ea nop word ptr [rax + rax]
14075f7f0 movsx eax, byte ptr [r8 - 1]
14075f7f5 test eax, eax
14075f7f7 js 0x14075f810
14075f7f9 dec r8
14075f7fc cmp eax, 0x5c
14075f7ff jne 0x14075f864
14075f801 jmp 0x14075f875
14075f803 nop word ptr cs:[rax + rax]
14075f810 movzx ecx, byte ptr [r8 - 2]
14075f815 cmp cl, 0xc0
14075f818 jge 0x14075f83d
14075f81a movzx edx, byte ptr [r8 - 3]
14075f81f cmp dl, 0xc0
14075f822 jge 0x14075f846
14075f824 movzx r9d, byte ptr [r8 - 4]
14075f829 add r8, -4
14075f82d and r9d, 7
14075f831 shl r9d, 6
14075f835 and edx, 0x3f
14075f838 or edx, r9d
14075f83b jmp 0x14075f84d
14075f83d add r8, -2
14075f841 and ecx, 0x1f
14075f844 jmp 0x14075f855
14075f846 add r8, -3
14075f84a and edx, 0xf
14075f84d shl edx, 6
14075f850 and ecx, 0x3f
14075f853 or ecx, edx
14075f855 shl ecx, 6
14075f858 and al, 0x3f
14075f85a movzx eax, al
14075f85d or eax, ecx
14075f85f cmp eax, 0x5c
14075f862 je 0x14075f875
14075f864 cmp eax, 0x2f
14075f867 je 0x14075f875
14075f869 cmp r8, r13
14075f86c jne 0x14075f7f0
14075f86e mov esi, 0x3b
14075f873 jmp 0x14075f8c1
14075f875 lea r13, [rip + 0x231dc1c]
14075f87c sub r8, r13
14075f87f inc r8
14075f882 je 0x14075f8b6
14075f884 cmp r8, 0x3b
14075f888 jae 0x14075f8b4
14075f88a cmp byte ptr [r8 + r13], 0xbf
14075f88f jg 0x14075f8b6
14075f891 lea rax, [rip + 0x231db98]
14075f898 mov qword ptr [rsp + 0x20], rax
14075f89d lea rcx, [rip + 0x231dbf4]
14075f8a4 mov edx, 0x3b
14075f8a9 mov r9d, 0x3b
14075f8af call 0x1428d9940
14075f8b4 jne 0x14075f891
14075f8b6 mov esi, 0x3b
14075f8bb sub rsi, r8
14075f8be add r13, r8
14075f8c1 call 0x140c18130
14075f8c6 mov rcx, rax
14075f8c9 mov rax, qword ptr [rax]
14075f8cc cmp rax, 1
14075f8d0 jne 0x140763185
14075f8d6 add rcx, 8
14075f8da cmp qword ptr [rcx], 0
14075f8de jne 0x1407632dc
14075f8e4 mov qword ptr [rcx], 0xffffffffffffffff
14075f8eb mov qword ptr [rbp + 0x2c0], rcx
14075f8f2 add rcx, 8
14075f8f6 mov qword ptr [rsp + 0x20], rsi
14075f8fb mov qword ptr [rsp + 0x30], 0
14075f904 mov qword ptr [rsp + 0x28], 1
14075f90d mov rdx, r12
14075f910 mov r8, r15
14075f913 mov r9, r13
14075f916 call 0x140c16d40
14075f91b mov qword ptr [rbp + 0xa8], rax
14075f922 mov rax, qword ptr [rbp + 0x2c0]
14075f929 inc qword ptr [rax]
14075f92c mov byte ptr [rbp + 0x2e7], dil
14075f933 mov rax, qword ptr [rbp + 0x418]
14075f93a mov rcx, qword ptr [rax]
14075f93d lea rdx, [rip + 0x24baf34]
14075f944 call 0x141209c40
14075f949 mov r15, rax
14075f94c test rax, rax
14075f94f je 0x140763641
14075f955 lea rax, [rip + 0x24bb049]
14075f95c mov qword ptr [rbp - 0x18], rax
14075f960 mov qword ptr [rbp - 0x10], 0x3a
14075f968 mov r12, r15
14075f96b add r12, 0x10
14075f96f cmp qword ptr [r15 + 0x18], 0x3a
14075f974 jne 0x14076321a
14075f97a mov rdx, qword ptr [r12]
14075f97e lea rcx, [rip + 0x24bb020]
14075f985 mov r8d, 0x3a
14075f98b call 0x142923bf0
14075f990 test eax, eax
14075f992 jne 0x14076321a
14075f998 mov rcx, qword ptr [r15]
14075f99b mov rax, qword ptr [r15 + 8]
14075f99f call qword ptr [rax + 0x18]
14075f9a2 mov qword ptr [rbp + 0x138], rax
14075f9a9 cmp qword ptr [rbp + 0x138], 0
14075f9b1 je 0x140763641
14075f9b7 mov byte ptr [rbp + 0x2e6], 0
14075f9be mov rax, qword ptr [r14]
14075f9c1 mov rdx, qword ptr [r14 + 8]
14075f9c5 mov r10, qword ptr [rax]
14075f9c8 xor r9d, r9d
14075f9cb mov r11, r10
14075f9ce sub r11, qword ptr [rdx + 0x18]
14075f9d2 cmovb r11, r9
14075f9d6 sub r10, qword ptr [rdx + 0x38]
14075f9da cmovb r10, r9
14075f9de mov rcx, r11
14075f9e1 shl rcx, 4
14075f9e5 add rcx, qword ptr [rdx + 8]
14075f9e9 mov r8, qword ptr [rdx + 0x10]
14075f9ed sub r8, r11
14075f9f0 cmovb r8, r9
14075f9f4 mov r14d, 8
14075f9fa cmovb rcx, r14
14075f9fe mov r11, r10
14075fa01 shl r11, 4
14075fa05 add r11, qword ptr [rdx + 0x28]
14075fa09 mov rsi, qword ptr [rdx + 0x30]
14075fa0d sub rsi, r10
14075fa10 cmovb rsi, r9
14075fa14 cmovb r11, r14
14075fa18 lea r9, [rsi + r8]
14075fa1c mov rdx, qword ptr [rdx + 0x40]
14075fa20 sub rdx, r9
14075fa23 mov qword ptr [rax], rdx
14075fa26 shl r8, 4
14075fa2a add r8, rcx
14075fa2d shl rsi, 4
14075fa31 add rsi, r11
14075fa34 mov qword ptr [rbp + 0x200], rax
14075fa3b mov qword ptr [rbp + 0x208], rcx
14075fa42 mov qword ptr [rbp + 0x210], r8
14075fa49 mov qword ptr [rbp + 0x218], r11
14075fa50 mov qword ptr [rbp + 0x220], rsi
14075fa57 mov qword ptr [rbp + 0x228], r9
14075fa5e lea r8, [rip + 0x231d91b]
14075fa65 lea rcx, [rbp + 0x1e8]
14075fa6c lea rdx, [rbp + 0x200]
14075fa73 call 0x1412036d0
14075fa78 mov r15, qword ptr [rbx]
14075fa7b mov rdx, qword ptr [rbx + 8]
14075fa7f mov r9, qword ptr [r15]
14075fa82 xor r8d, r8d
14075fa85 mov rcx, r9
14075fa88 sub rcx, qword ptr [rdx + 0x18]
14075fa8c cmovb rcx, r8
14075fa90 sub r9, qword ptr [rdx + 0x38]
14075fa94 cmovb r9, r8
14075fa98 mov rax, rcx
14075fa9b shl rax, 6
14075fa9f add rax, qword ptr [rdx + 8]
14075faa3 mov r12, qword ptr [rdx + 0x10]
14075faa7 sub r12, rcx
14075faaa cmovb r12, r8
14075faae mov r13, qword ptr [rdx + 0x30]
14075fab2 cmovb rax, r14
14075fab6 mov rcx, r9
14075fab9 shl rcx, 6
14075fabd add rcx, qword ptr [rdx + 0x28]
14075fac1 sub r13, r9
14075fac4 cmovb r13, r8
14075fac8 cmovb rcx, r14
14075facc mov rdx, qword ptr [rdx + 0x40]
14075fad0 sub rdx, r13
14075fad3 sub rdx, r12
14075fad6 mov qword ptr [r15], rdx
14075fad9 shl r12, 6
14075fadd add r12, rax
14075fae0 shl r13, 6
14075fae4 add r13, rcx
14075fae7 jmp 0x14075fb38
14075fae9 test rsi, rsi
14075faec je 0x14075fba8
14075faf2 mov r8, r13
14075faf5 sub r8, rsi
14075faf8 shr r8, 6
14075fafc inc r8
14075faff mov qword ptr [rsp + 0x20], 8
14075fb08 mov r9d, 8
14075fb0e lea rcx, [rbp + 0x1e8]
14075fb15 mov rdx, r14
14075fb18 call 0x1428a7de0
14075fb1d mov rax, qword ptr [rbp + 0x1f0]
14075fb24 mov qword ptr [rax + r14*8], rbx
14075fb28 inc r14
14075fb2b mov qword ptr [rbp + 0x1f8], r14
14075fb32 mov rcx, rsi
14075fb35 mov rax, rdi
14075fb38 test rax, rax
14075fb3b je 0x14075fb50
14075fb3d cmp rax, r12
14075fb40 je 0x14075fb50
14075fb42 lea rdi, [rax + 0x40]
14075fb46 mov rsi, rcx
14075fb49 jmp 0x14075fb63
14075fb4b nop dword ptr [rax + rax]
14075fb50 test rcx, rcx
14075fb53 je 0x14075fbb0
14075fb55 cmp rcx, r13
14075fb58 je 0x14075fbb0
14075fb5a lea rsi, [rcx + 0x40]
14075fb5e xor edi, edi
14075fb60 mov rax, rcx
14075fb63 inc qword ptr [r15]
14075fb66 mov rbx, qword ptr [rax + 8]
14075fb6a mov r14, qword ptr [rbp + 0x1f8]
14075fb71 cmp r14, qword ptr [rbp + 0x1e8]
14075fb78 jne 0x14075fb1d
14075fb7a test rdi, rdi
14075fb7d je 0x14075fae9
14075fb83 mov r8, r12
14075fb86 sub r8, rdi
14075fb89 shr r8, 6
14075fb8d test rsi, rsi
14075fb90 je 0x14075fafc
14075fb96 mov rax, r13
14075fb99 sub rax, rsi
14075fb9c shr rax, 6
14075fba0 add r8, rax
14075fba3 jmp 0x14075fafc
14075fba8 xor r8d, r8d
14075fbab jmp 0x14075fafc
14075fbb0 mov rax, qword ptr [rbp + 0x2e8]
14075fbb7 mov r14, qword ptr [rax]
14075fbba mov rdx, qword ptr [rax + 8]
14075fbbe mov r9, qword ptr [r14]
14075fbc1 xor r8d, r8d
14075fbc4 mov rcx, r9
14075fbc7 sub rcx, qword ptr [rdx + 0x18]
14075fbcb cmovb rcx, r8
14075fbcf sub r9, qword ptr [rdx + 0x38]
14075fbd3 cmovb r9, r8
14075fbd7 mov rax, rcx
14075fbda shl rax, 5
14075fbde add rax, qword ptr [rdx + 8]
14075fbe2 mov r15, qword ptr [rdx + 0x10]
14075fbe6 sub r15, rcx
14075fbe9 cmovb r15, r8
14075fbed mov r10d, 8
14075fbf3 cmovb rax, r10
14075fbf7 mov rcx, r9
14075fbfa shl rcx, 5
14075fbfe add rcx, qword ptr [rdx + 0x28]
14075fc02 mov r12, qword ptr [rdx + 0x30]
14075fc06 sub r12, r9
14075fc09 cmovb r12, r8
14075fc0d cmovb rcx, r10
14075fc11 mov rdx, qword ptr [rdx + 0x40]
14075fc15 sub rdx, r12
14075fc18 sub rdx, r15
14075fc1b mov qword ptr [r14], rdx
14075fc1e shl r15, 5
14075fc22 add r15, rax
14075fc25 shl r12, 5
14075fc29 add r12, rcx
14075fc2c jmp 0x14075fc7d
14075fc2e test rsi, rsi
14075fc31 je 0x14075fce9
14075fc37 mov r8, r12
14075fc3a sub r8, rsi
14075fc3d shr r8, 5
14075fc41 inc r8
14075fc44 mov qword ptr [rsp + 0x20], 8
14075fc4d mov r9d, 8
14075fc53 lea rcx, [rbp + 0x1e8]
14075fc5a mov rdx, rbx
14075fc5d call 0x1428a7de0
14075fc62 mov rax, qword ptr [rbp + 0x1f0]
14075fc69 mov qword ptr [rax + rbx*8], rdi
14075fc6d inc rbx
14075fc70 mov qword ptr [rbp + 0x1f8], rbx
14075fc77 mov rcx, rsi
14075fc7a mov rax, r13
14075fc7d test rax, rax
14075fc80 je 0x14075fc90
14075fc82 cmp rax, r15
14075fc85 je 0x14075fc90
14075fc87 lea r13, [rax + 0x20]
14075fc8b mov rsi, rcx
14075fc8e jmp 0x14075fca4
14075fc90 test rcx, rcx
14075fc93 je 0x14075fcf1
14075fc95 cmp rcx, r12
14075fc98 je 0x14075fcf1
14075fc9a lea rsi, [rcx + 0x20]
14075fc9e xor r13d, r13d
14075fca1 mov rax, rcx
14075fca4 inc qword ptr [r14]
14075fca7 mov rdi, qword ptr [rax + 8]
14075fcab mov rbx, qword ptr [rbp + 0x1f8]
14075fcb2 cmp rbx, qword ptr [rbp + 0x1e8]
14075fcb9 jne 0x14075fc62
14075fcbb test r13, r13
14075fcbe je 0x14075fc2e
14075fcc4 mov r8, r15
14075fcc7 sub r8, r13
14075fcca shr r8, 5
14075fcce test rsi, rsi
14075fcd1 je 0x14075fc41
14075fcd7 mov rax, r12
14075fcda sub rax, rsi
14075fcdd shr rax, 5
14075fce1 add r8, rax
14075fce4 jmp 0x14075fc41
14075fce9 xor r8d, r8d
14075fcec jmp 0x14075fc41
14075fcf1 mov rbx, qword ptr [rbp + 0x1f0]
14075fcf8 mov rdi, qword ptr [rbp + 0x1f8]
14075fcff cmp rdi, 1
14075fd03 ja 0x14076324b
14075fd09 lea r14, [rbx + rdi*8]
14075fd0d mov rcx, qword ptr [rbp + 0x2d0]
14075fd14 mov rdi, qword ptr [rcx]
14075fd17 mov rax, qword ptr [rcx + 0x10]
14075fd1b mov qword ptr [rbp + 0x70], rax
14075fd1f mov rdx, qword ptr [rbp + 0x410]
14075fd26 mov eax, dword ptr [rdx + 0x1c]
14075fd29 mov dword ptr [rbp + 0x100], eax
14075fd2f mov r15, qword ptr [rdx]
14075fd32 mov rax, qword ptr [rdx + 0x10]
14075fd36 mov qword ptr [rbp + 0x78], rax
14075fd3a mov rax, qword ptr [rbp + 0x420]
14075fd41 mov rax, qword ptr [rax]
14075fd44 mov qword ptr [rbp + 0x120], rax
14075fd4b mov eax, dword ptr [rcx + 0x1c]
14075fd4e mov dword ptr [rbp + 0xfc], eax
14075fd54 mov rcx, qword ptr [rbp + 0x430]
14075fd5b mov eax, dword ptr [rcx + 0x1c]
14075fd5e mov dword ptr [rbp + 0x104], eax
14075fd64 mov rax, qword ptr [rcx]
14075fd67 mov qword ptr [rbp + 0x190], rax
14075fd6e mov rax, qword ptr [rcx + 0x10]
14075fd72 mov qword ptr [rbp + 0x88], rax
14075fd79 mov rcx, qword ptr [rbp + 0x428]
14075fd80 mov eax, dword ptr [rcx + 0x1c]
14075fd83 mov dword ptr [rbp + 0xf8], eax
14075fd89 mov rax, qword ptr [rcx]
14075fd8c mov rcx, qword ptr [rcx + 0x10]
14075fd90 mov qword ptr [rbp + 0x50], rcx
14075fd94 movss xmm6, dword ptr [rip + 0x21c5b68]
14075fd9c movss xmm12, dword ptr [rip + 0x21c5b33]
14075fda5 movss xmm15, dword ptr [rip + 0x231d4c2]
14075fdae lea rcx, [rdi + 0x20]
14075fdb2 mov qword ptr [rbp + 0xa0], rcx
14075fdb9 lea rcx, [r15 + 0x20]
14075fdbd mov qword ptr [rbp + 0x80], rcx
14075fdc4 mov qword ptr [rbp + 0x180], rax
14075fdcb add rax, 0x20
14075fdcf mov qword ptr [rbp + 0x58], rax
14075fdd3 mov qword ptr [rbp + 0x288], rdi
14075fdda mov qword ptr [rbp + 0x90], r15
14075fde1 cmp rbx, r14
14075fde4 je 0x140762bbf
14075fdea mov rsi, qword ptr [rbx]
14075fded mov qword ptr [rbp + 0x2b0], rsi
14075fdf4 cmp qword ptr [rdi + 0x18], 0
14075fdf9 je 0x140762b44
14075fdff add rbx, 8
14075fe03 mov rcx, qword ptr [rbp + 0xa0]
14075fe0a lea rdx, [rbp + 0x2b0]
14075fe11 call 0x1405b0d70
14075fe16 mov r12, qword ptr [rdi]
14075fe19 mov rcx, qword ptr [rdi + 8]
14075fe1d mov rdx, rcx
14075fe20 and rdx, rax
14075fe23 shr rax, 0x39
14075fe27 movd xmm0, eax
14075fe2b punpcklbw xmm0, xmm0
14075fe2f pshuflw xmm0, xmm0, 0
14075fe34 pshufd xmm0, xmm0, 0
14075fe39 lea rax, [r12 - 0x2a8]
14075fe41 xor r8d, r8d
14075fe44 movdqu xmm1, xmmword ptr [r12 + rdx]
14075fe4a movdqa xmm2, xmm1
14075fe4e pcmpeqb xmm2, xmm0
14075fe52 pmovmskb r9d, xmm2
14075fe57 test r9d, r9d
14075fe5a je 0x14075fe90
14075fe5c tzcnt r10d, r9d
14075fe61 add r10, rdx
14075fe64 and r10, rcx
14075fe67 neg r10
14075fe6a imul rdi, r10, 0x2a8
14075fe71 cmp rsi, qword ptr [rax + rdi]
14075fe75 je 0x14075fec0
14075fe77 lea r10d, [r9 - 1]
14075fe7b and r10w, r9w
14075fe7f mov r9d, r10d
14075fe82 jne 0x14075fe5c
14075fe84 nop word ptr cs:[rax + rax]
14075fe90 pcmpeqb xmm1, xmmword ptr [rip + 0x21c5af8]
14075fe98 pmovmskb r9d, xmm1
14075fe9d test r9d, r9d
14075fea0 jne 0x140762b44
14075fea6 add rdx, r8
14075fea9 add rdx, 0x10
14075fead add r8, 0x10
14075feb1 and rdx, rcx
14075feb4 jmp 0x14075fe44
14075feb6 nop word ptr cs:[rax + rax]
14075fec0 lea rcx, [r12 + rdi]
14075fec4 add rcx, -0x2a0
14075fecb mov qword ptr [rbp + 0x2d0], rcx
14075fed2 call 0x140b2a770
14075fed7 mov dword ptr [rbp + 0x1a0], eax
14075fedd add r12, rdi
14075fee0 mov rdi, qword ptr [r12 - 0x180]
14075fee8 test rdi, rdi
14075feeb je 0x14076367b
14075fef1 movdqa xmm9, xmm0
14075fef6 lea rcx, [rdi + 0x18]
14075fefa xor eax, eax
14075fefc mov dl, 1
14075fefe lock cmpxchg byte ptr [rdi + 0x18], dl
14075ff03 jne 0x140762b22
14075ff09 call 0x140c81c70
14075ff0e mov rax, qword ptr [rdi + 0x10]
14075ff12 test rax, rax
14075ff15 sete cl
14075ff18 shl rax, 2
14075ff1c add rax, qword ptr [rdi + 8]
14075ff20 add rax, -4
14075ff24 sete dl
14075ff27 or dl, cl
14075ff29 je 0x14075ff50
14075ff2b mov rdx, qword ptr [rbp + 0x138]
14075ff32 mov rax, qword ptr [rdx + 0x28]
14075ff36 test rax, rax
14075ff39 mov rsi, qword ptr [rbp + 0x288]
14075ff40 je 0x1407636a3
14075ff46 lea ecx, [rax - 1]
14075ff49 jmp 0x14075ff64
14075ff4b nop dword ptr [rax + rax]
14075ff50 mov ecx, dword ptr [rax]
14075ff52 mov rdx, qword ptr [rbp + 0x138]
14075ff59 mov rax, qword ptr [rdx + 0x28]
14075ff5d mov rsi, qword ptr [rbp + 0x288]
14075ff64 mov r8, rdx
14075ff67 mov rdx, qword ptr [rdx + 0x20]
14075ff6b movdqu xmm0, xmmword ptr [r8 + 0x30]
14075ff71 movdqu xmm1, xmmword ptr [r8 + 0x40]
14075ff77 movdqu xmm2, xmmword ptr [r8 + 0x50]
14075ff7d mov dword ptr [rbp + 0x38], ecx
14075ff80 mov qword ptr [rbp - 0x18], rdx
14075ff84 mov qword ptr [rbp - 0x10], rax
14075ff88 movdqu xmmword ptr [rbp - 8], xmm0
14075ff8d mov qword ptr [rbp + 8], rdi
14075ff91 movdqu xmmword ptr [rbp + 0x10], xmm1
14075ff96 mov qword ptr [rbp + 0x20], r8
14075ff9a movdqu xmmword ptr [rbp + 0x28], xmm2
14075ff9f lea rcx, [rbp - 0x18]
14075ffa3 call 0x14120c9d0
14075ffa8 mov qword ptr [rbp + 0x160], 0
14075ffb3 mov qword ptr [rbp + 0x168], 8
14075ffbe mov qword ptr [rbp + 0x170], 0
14075ffc9 mov qword ptr [rbp + 0x148], 0
14075ffd4 mov qword ptr [rbp + 0x150], 4
14075ffdf mov qword ptr [rbp + 0x158], 0
14075ffea mov qword ptr [rbp + 0x178], 0
14075fff5 cmp byte ptr [r12 - 0xa], 0
14075fffb je 0x140760040
14075fffd pxor xmm0, xmm0
140760001 movd dword ptr [rbp + 0x2ac], xmm0
140760009 mov qword ptr [rbp + 0x140], 0
140760014 mov qword ptr [rbp + 0x188], 0
14076001f movd dword ptr [rbp + 0x2a8], xmm0
140760027 movd dword ptr [rbp + 0x2a4], xmm0
14076002f jmp 0x140761142
140760034 nop word ptr cs:[rax + rax]
140760040 mov eax, dword ptr [rbp + 0x100]
140760046 mov rcx, qword ptr [rbp + 0x78]
14076004a mov dword ptr [rcx], eax
14076004c cmp qword ptr [r15 + 0x18], 0
140760051 je 0x140762b6d
140760057 mov rcx, qword ptr [rbp + 0x80]
14076005e lea rdx, [rbp + 0x2b0]
140760065 call 0x1405b0d70
14076006a mov rcx, qword ptr [r15]
14076006d mov rdx, qword ptr [r15 + 8]
140760071 mov r8, rdx
140760074 and r8, rax
140760077 shr rax, 0x39
14076007b movd xmm0, eax
14076007f punpcklbw xmm0, xmm0
140760083 pshuflw xmm0, xmm0, 0
140760088 pshufd xmm0, xmm0, 0
14076008d lea rax, [rcx - 0x20]
140760091 mov r9, qword ptr [rbp + 0x2b0]
140760098 xor r10d, r10d
14076009b movdqu xmm1, xmmword ptr [rcx + r8]
1407600a1 movdqa xmm2, xmm1
1407600a5 pcmpeqb xmm2, xmm0
1407600a9 pmovmskb r11d, xmm2
1407600ae test r11d, r11d
1407600b1 je 0x1407600eb
1407600b3 mov qword ptr [rbp + 0x198], rbx
1407600ba tzcnt esi, r11d
1407600bf add rsi, r8
1407600c2 and rsi, rdx
1407600c5 mov rdi, rsi
1407600c8 shl rdi, 5
1407600cc mov rbx, rax
1407600cf sub rbx, rdi
1407600d2 cmp r9, qword ptr [rbx]
1407600d5 je 0x140760111
1407600d7 lea esi, [r11 - 1]
1407600db and si, r11w
1407600df mov r11d, esi
1407600e2 mov rbx, qword ptr [rbp + 0x198]
1407600e9 jne 0x1407600ba
1407600eb pcmpeqb xmm1, xmmword ptr [rip + 0x21c589d]
1407600f3 pmovmskb r11d, xmm1
1407600f8 test r11d, r11d
1407600fb jne 0x140762b6d
140760101 add r8, r10
140760104 add r8, 0x10
140760108 add r10, 0x10
14076010c and r8, rdx
14076010f jmp 0x14076009b
140760111 neg rsi
140760114 shl rsi, 5
140760118 mov rdx, qword ptr [rcx + rsi - 0x10]
14076011d mov rax, qword ptr [rcx + rsi - 8]
140760122 mov rcx, rax
140760125 shl rcx, 7
140760129 lea rax, [rcx + rax*8]
14076012d add rax, rdx
140760130 mov qword ptr [rbp + 0x68], rax
140760134 pxor xmm0, xmm0
140760138 movd dword ptr [rbp + 0x2ac], xmm0
140760140 mov qword ptr [rbp + 0x2c0], 0
14076014b mov qword ptr [rbp + 0x140], 0
140760156 mov qword ptr [rbp + 0x188], 0
140760161 movd dword ptr [rbp + 0x2a8], xmm0
140760169 movd dword ptr [rbp + 0x2a4], xmm0
140760171 mov qword ptr [rbp + 0x130], r14
140760178 mov qword ptr [rbp + 0x290], r12
14076017f jmp 0x1407601a2
140760181 nop word ptr cs:[rax + rax]
140760190 mov rdx, qword ptr [rbp + 0x60]
140760194 mov r14, qword ptr [rbp + 0x130]
14076019b mov r12, qword ptr [rbp + 0x290]
1407601a2 mov qword ptr [rbp + 0x2e8], rdx
1407601a9 cmp rdx, qword ptr [rbp + 0x68]
1407601ad je 0x14076112b
1407601b3 mov rax, qword ptr [rbp + 0x170]
1407601ba mov qword ptr [rbp + 0x2d8], rax
1407601c1 mov rax, qword ptr [r12 - 0x1d8]
1407601c9 mov qword ptr [rbp + 0x2b8], rax
1407601d0 mov rax, qword ptr [r12 - 0x1d0]
1407601d8 mov qword ptr [rbp + 0x298], rax
1407601df mov r14, qword ptr [r12 - 0x1c0]
1407601e7 mov r15, qword ptr [r12 - 0x1b8]
1407601ef mov rax, r12
1407601f2 mov r12, qword ptr [r12 - 0x210]
1407601fa mov r13, qword ptr [rax - 0x208]
140760201 mov rbx, qword ptr [rax - 0x1f8]
140760208 mov rsi, qword ptr [rax - 0x1f0]
14076020f mov edi, dword ptr [rax - 0x48]
140760212 mov byte ptr [rbp + 0x2f7], 1
140760219 mov rcx, qword ptr [rbp + 0x2d0]
140760220 call 0x140b2a770
140760225 mov dword ptr [rbp + 0x200], eax
14076022b movd dword ptr [rbp + 0x204], xmm0
140760233 mov byte ptr [rbp + 0x2f7], 1
14076023a cmp edi, 2
14076023d setb byte ptr [rsp + 0x50]
140760242 lea rax, [rbp + 0x200]
140760249 mov qword ptr [rsp + 0x58], rax
14076024e mov qword ptr [rsp + 0x48], rsi
140760253 mov qword ptr [rsp + 0x40], rbx
140760258 mov qword ptr [rsp + 0x38], r13
14076025d mov qword ptr [rsp + 0x30], r12
140760262 mov qword ptr [rsp + 0x28], r15
140760267 mov qword ptr [rsp + 0x20], r14
14076026c lea rcx, [rbp + 0x108]
140760273 mov rdx, qword ptr [rbp + 0x2e8]
14076027a mov r8, qword ptr [rbp + 0x2b8]
140760281 mov r9, qword ptr [rbp + 0x298]
140760288 call 0x14122ff90
14076028d mov rax, qword ptr [rbp + 0x110]
140760294 mov r8, qword ptr [rbp + 0x118]
14076029b mov qword ptr [rbp + 0x118], 0
1407602a6 imul rcx, r8, 0x68
1407602aa add rcx, rax
1407602ad mov qword ptr [rbp + 0xb0], rax
1407602b4 mov qword ptr [rbp + 0xb8], rcx
1407602bb lea rax, [rbp + 0x108]
1407602c2 mov qword ptr [rbp + 0xc0], rax
1407602c9 mov qword ptr [rbp + 0xc8], r8
1407602d0 mov qword ptr [rbp + 0xd0], 0
1407602db mov rax, qword ptr [rbp + 0x160]
1407602e2 mov rdx, qword ptr [rbp + 0x2d8]
1407602e9 sub rax, rdx
1407602ec mov rsi, rdx
1407602ef cmp rax, r8
1407602f2 jb 0x1407610fd
1407602f8 mov rdi, qword ptr [rbp + 0x168]
1407602ff mov rax, qword ptr [rbp + 0xb0]
140760306 mov rcx, qword ptr [rbp + 0xb8]
14076030d mov rdx, qword ptr [rbp + 0xc0]
140760314 mov r8, qword ptr [rbp + 0xc8]
14076031b mov qword ptr [rbp + 0x1b0], rax
140760322 mov qword ptr [rbp + 0x1b8], rcx
140760329 mov rax, qword ptr [rbp + 0xd0]
140760330 mov qword ptr [rbp + 0x1d0], rax
140760337 mov qword ptr [rbp + 0x1c0], rdx
14076033e mov qword ptr [rbp + 0x1c8], r8
140760345 mov r9, qword ptr [rbp + 0x1b0]
14076034c mov rax, qword ptr [rbp + 0x1b8]
140760353 cmp r9, rax
140760356 je 0x140760439
14076035c imul rcx, rsi, 0x68
140760360 add rcx, rdi
140760363 add rcx, 8
140760367 add r9, 0x68
14076036b nop dword ptr [rax + rax]
140760370 mov rdx, r9
140760373 mov r9, qword ptr [r9 - 0x68]
140760377 mov r8, r9
14076037a neg r8
14076037d jo 0x140760432
140760383 lea r8, [rdx - 0x68]
140760387 movups xmm0, xmmword ptr [r8 + 0x58]
14076038c movaps xmmword ptr [rbp + 0x250], xmm0
140760393 movups xmm0, xmmword ptr [r8 + 0x48]
140760398 movaps xmmword ptr [rbp + 0x240], xmm0
14076039f movups xmm0, xmmword ptr [r8 + 8]
1407603a4 movups xmm1, xmmword ptr [r8 + 0x18]
1407603a9 movups xmm2, xmmword ptr [r8 + 0x28]
1407603ae movups xmm3, xmmword ptr [r8 + 0x38]
1407603b3 movaps xmmword ptr [rbp + 0x230], xmm3
1407603ba movaps xmmword ptr [rbp + 0x220], xmm2
1407603c1 movaps xmmword ptr [rbp + 0x210], xmm1
1407603c8 movaps xmmword ptr [rbp + 0x200], xmm0
1407603cf mov qword ptr [rcx - 8], r9
1407603d3 movaps xmm0, xmmword ptr [rbp + 0x200]
1407603da movdqa xmm1, xmmword ptr [rbp + 0x210]
1407603e2 movdqa xmm2, xmmword ptr [rbp + 0x220]
1407603ea movaps xmm3, xmmword ptr [rbp + 0x230]
1407603f1 movups xmmword ptr [rcx], xmm0
1407603f4 movdqu xmmword ptr [rcx + 0x10], xmm1
1407603f9 movdqu xmmword ptr [rcx + 0x20], xmm2
1407603fe movups xmmword ptr [rcx + 0x30], xmm3
140760402 movaps xmm0, xmmword ptr [rbp + 0x240]
140760409 movups xmmword ptr [rcx + 0x40], xmm0
14076040d movdqa xmm0, xmmword ptr [rbp + 0x250]
140760415 movdqu xmmword ptr [rcx + 0x50], xmm0
14076041a inc rsi
14076041d add rcx, 0x68
140760421 lea r9, [rdx + 0x68]
140760425 add r8, 0x68
140760429 cmp r8, rax
14076042c jne 0x140760370
140760432 mov qword ptr [rbp + 0x1b0], rdx
140760439 mov qword ptr [rbp + 0x170], rsi
140760440 mov byte ptr [rbp + 0x2e5], 0
140760447 lea rcx, [rbp + 0x1b0]
14076044e call 0x1411fd610
140760453 mov byte ptr [rbp + 0x2f7], 1
14076045a lea rcx, [rbp + 0x108]
140760461 call 0x1405d3750
140760466 mov rdx, qword ptr [rbp + 0x2b0]
14076046d mov byte ptr [rbp + 0x2f7], 1
140760474 mov rcx, qword ptr [rbp + 0x288]
14076047b call 0x140a92100
140760480 mov r13d, eax
140760483 mov rax, rsi
140760486 mov r15, qword ptr [rbp + 0x2d8]
14076048d sub rax, r15
140760490 cmp rax, 1
140760494 sete al
140760497 and r13b, al
14076049a je 0x1407604e0
14076049c test rsi, rsi
14076049f movss xmm4, dword ptr [rip + 0x21c53c9]
1407604a7 xorps xmm5, xmm5
1407604aa mov r14, qword ptr [rbp + 0x290]
1407604b1 je 0x140763625
1407604b7 mov rax, qword ptr [rdi + 0x28]
1407604bb test rax, rax
1407604be je 0x140760518
1407604c0 mov rcx, qword ptr [rdi + 0x20]
1407604c4 cmp rax, 1
1407604c8 jne 0x140760521
1407604ca pxor xmm0, xmm0
1407604ce xor edx, edx
1407604d0 jmp 0x140760586
1407604d5 nop word ptr cs:[rax + rax]
1407604e0 movss xmm0, dword ptr [rbp + 0x2a4]
1407604e8 mov rax, qword ptr [rbp + 0x2e8]
1407604ef addss xmm0, dword ptr [rax + 0x84]
1407604f7 movss dword ptr [rbp + 0x2a4], xmm0
1407604ff mov r14, qword ptr [rbp + 0x290]
140760506 xor ebx, ebx
140760508 cmp byte ptr [r14 - 0xc], 0
14076050d jne 0x140760628
140760513 jmp 0x1407606f5
140760518 pxor xmm0, xmm0
14076051c jmp 0x1407605ac
140760521 mov r8, rax
140760524 and r8, 0xfffffffffffffffe
140760528 lea r9, [rcx + 0x10]
14076052c pxor xmm1, xmm1
140760530 xor edx, edx
140760532 nop word ptr cs:[rax + rax]
140760540 movdqa xmm2, xmm1
140760544 addss xmm2, xmm6
140760548 movss xmm3, dword ptr [r9 - 0xc]
14076054e movss xmm0, dword ptr [r9]
140760553 cmpltss xmm3, xmm5
140760558 andps xmm2, xmm3
14076055b andnps xmm3, xmm1
14076055e orps xmm3, xmm2
140760561 cmpltss xmm0, xmm5
140760566 movaps xmm1, xmm0
140760569 andnps xmm1, xmm3
14076056c addss xmm3, xmm6
140760570 andps xmm0, xmm3
140760573 orps xmm0, xmm1
140760576 add rdx, 2
14076057a add r9, 0x18
14076057e movaps xmm1, xmm0
140760581 cmp r8, rdx
140760584 jne 0x140760540
140760586 test al, 1
140760588 je 0x1407605ac
14076058a lea rdx, [rdx + rdx*2]
14076058e movss xmm1, dword ptr [rcx + rdx*4 + 4]
140760594 movaps xmm2, xmm0
140760597 addss xmm2, xmm6
14076059b cmpltss xmm1, xmm5
1407605a0 andps xmm2, xmm1
1407605a3 andnps xmm1, xmm0
1407605a6 orps xmm1, xmm2
1407605a9 movaps xmm0, xmm1
1407605ac xorps xmm1, xmm1
1407605af cvtsi2ss xmm1, rax
1407605b4 divss xmm0, xmm1
1407605b8 ucomiss xmm0, xmm4
1407605bb mov rax, qword ptr [rbp + 0x2e8]
1407605c2 movss xmm0, dword ptr [rax + 0x84]
1407605ca jbe 0x140760600
1407605cc movss xmm1, dword ptr [rbp + 0x2ac]
1407605d4 addss xmm1, xmm0
1407605d8 movss dword ptr [rbp + 0x2ac], xmm1
1407605e0 inc qword ptr [rbp + 0x140]
1407605e7 xor ebx, ebx
1407605e9 cmp byte ptr [r14 - 0xc], 0
1407605ee jne 0x140760628
1407605f0 jmp 0x1407606f5
1407605f5 nop word ptr cs:[rax + rax]
140760600 movss xmm1, dword ptr [rbp + 0x2a8]
140760608 addss xmm1, xmm0
14076060c movss dword ptr [rbp + 0x2a8], xmm1
140760614 inc qword ptr [rbp + 0x188]
14076061b xor ebx, ebx
14076061d cmp byte ptr [r14 - 0xc], 0
140760622 je 0x1407606f5
140760628 cmp rsi, r15
14076062b jb 0x1407631bd
140760631 mov ebx, 1
140760636 xorps xmm13, xmm13
14076063a jne 0x140760650
14076063c xorps xmm7, xmm7
14076063f jmp 0x1407606f5
140760644 nop word ptr cs:[rax + rax]
140760650 imul rax, rsi, 0x68
140760654 add rax, rdi
140760657 imul rdx, r15, 0x68
14076065b add rdx, rdi
14076065e xorps xmm7, xmm7
140760661 jmp 0x140760679
140760663 nop word ptr cs:[rax + rax]
140760670 add rdx, 0x68
140760674 cmp rdx, rax
140760677 je 0x1407606f0
140760679 mov r8, qword ptr [rdx + 0x10]
14076067d test r8, r8
140760680 je 0x140760670
140760682 mov r9, qword ptr [rdx + 8]
140760686 mov rcx, qword ptr [rdx + 0x28]
14076068a shl r8, 2
14076068e mov r10d, 1
140760694 movaps xmm0, xmm7
140760697 xor r11d, r11d
14076069a nop word ptr [rax + rax]
1407606a0 cmp rcx, r11
1407606a3 je 0x1407631ff
1407606a9 mov rbx, qword ptr [rdx + 0x20]
1407606ad movss xmm1, dword ptr [rbx + r10*4]
1407606b3 movss xmm2, dword ptr [r9 + r11*4]
1407606b9 addss xmm2, xmm1
1407606bd inc r11
1407606c0 movaps xmm7, xmm2
1407606c3 maxss xmm7, xmm0
1407606c7 cmpltss xmm0, xmm2
1407606cc andps xmm1, xmm0
1407606cf andnps xmm0, xmm13
1407606d3 orps xmm0, xmm1
1407606d6 add r10, 3
1407606da movaps xmm13, xmm0
1407606de movaps xmm0, xmm7
1407606e1 add r8, -4
1407606e5 jne 0x1407606a0
1407606e7 jmp 0x140760670
1407606e9 nop dword ptr [rax]
1407606f0 mov ebx, 1
1407606f5 cmp rsi, r15
1407606f8 jb 0x14076319f
1407606fe mov rax, qword ptr [rbp + 0x2e8]
140760705 add rax, 0x88
14076070b mov qword ptr [rbp + 0x60], rax
14076070f imul r15, r15, 0x68
140760713 add r15, rdi
140760716 imul r12, rsi, 0x68
14076071a add r12, rdi
14076071d mov rsi, r15
140760720 mov qword ptr [rbp + 0x40], r12
140760724 mov byte ptr [rbp + 0x2cf], r13b
14076072b mov qword ptr [rbp + 0x48], r15
14076072f mov dword ptr [rbp + 0xf4], ebx
140760735 cmp rsi, r12
140760738 je 0x140760c04
14076073e nop 
140760740 inc qword ptr [rbp + 0x178]
140760747 movd xmm1, dword ptr [r14 - 0x18]
14076074d mov r8, qword ptr [r14 - 0x208]
140760754 mov dword ptr [rbp + 0x200], ebx
14076075a movss dword ptr [rbp + 0x204], xmm7
140760762 movss dword ptr [rbp + 0x208], xmm13
14076076b mov byte ptr [rbp + 0x2f7], 1
140760772 mov rax, qword ptr [rbp + 0x2d0]
140760779 mov qword ptr [rsp + 0x28], rax
14076077e lea rax, [rbp + 0x200]
140760785 mov qword ptr [rsp + 0x20], rax
14076078a mov rcx, rsi
14076078d mov r9, qword ptr [rbp + 0x120]
140760794 call 0x1407645f0
140760799 movaps xmm10, xmm0
14076079d movdqa xmm11, xmm1
1407607a2 mov rdi, qword ptr [rbp + 0x158]
1407607a9 cmp rdi, qword ptr [rbp + 0x148]
1407607b0 jne 0x1407607cc
1407607b2 mov byte ptr [rbp + 0x2f7], 1
1407607b9 lea rcx, [rbp + 0x148]
1407607c0 lea rdx, [rip + 0x231cf41]
1407607c7 call 0x140562e70
1407607cc mov rax, qword ptr [rbp + 0x150]
1407607d3 movss dword ptr [rax + rdi*8], xmm10
1407607d9 movd dword ptr [rax + rdi*8 + 4], xmm11
1407607e0 inc rdi
1407607e3 mov qword ptr [rbp + 0x158], rdi
1407607ea mov qword ptr [rbp + 0xb0], 0
1407607f5 test r13b, r13b
1407607f8 je 0x140760b34
1407607fe cmp qword ptr [rsi + 0x28], 2
140760803 jb 0x140760b34
140760809 mov r15, qword ptr [rsi + 0x10]
14076080d lea rbx, [r15*4]
140760815 mov rax, r15
140760818 shr rax, 0x3e
14076081c jne 0x1407635a0
140760822 mov r14d, 0
140760828 movabs rax, 0x7ffffffffffffffc
140760832 cmp rbx, rax
140760835 ja 0x1407635a3
14076083b mov rdi, qword ptr [rsi + 8]
14076083f test rbx, rbx
140760842 je 0x140760870
140760844 movzx eax, byte ptr [rip + 0x2ed43be]
14076084b mov r14d, 4
140760851 mov edx, 4
140760856 mov rcx, rbx
140760859 call 0x140613c10
14076085e test rax, rax
140760861 je 0x1407635a3
140760867 mov qword ptr [rbp + 0x2d8], r15
14076086e jmp 0x140760880
140760870 mov eax, 4
140760875 mov qword ptr [rbp + 0x2d8], 0
140760880 mov qword ptr [rbp + 0x2b8], rax
140760887 mov rcx, rax
14076088a mov rdx, rdi
14076088d mov r8, rbx
140760890 call 0x142923c00
140760895 mov rbx, qword ptr [rsi + 0x28]
140760899 mov rax, rbx
14076089c mov ecx, 0xc
1407608a1 mul rcx
1407608a4 mov r14, rax
1407608a7 jo 0x1407635c1
1407608ad mov r12d, 0
1407608b3 movabs rax, 0x7ffffffffffffffc
1407608bd cmp r14, rax
1407608c0 ja 0x1407635c4
1407608c6 mov rdi, qword ptr [rsi + 0x20]
1407608ca test r14, r14
1407608cd mov qword ptr [rbp + 0x128], rbx
1407608d4 je 0x140760902
1407608d6 movzx eax, byte ptr [rip + 0x2ed432c]
1407608dd mov r12d, 4
1407608e3 mov edx, 4
1407608e8 mov rcx, r14
1407608eb call 0x140613c10
1407608f0 test rax, rax
1407608f3 je 0x1407635c4
1407608f9 mov qword ptr [rbp + 0x298], rbx
140760900 jmp 0x140760912
140760902 mov eax, 4
140760907 mov qword ptr [rbp + 0x298], 0
140760912 mov rbx, r15
140760915 mov qword ptr [rbp + 0x1a8], rax
14076091c mov rcx, rax
14076091f mov rdx, rdi
140760922 mov r8, r14
140760925 call 0x142923c00
14076092a mov r12, qword ptr [rsi + 0x40]
14076092e lea r14, [r12*4]
140760936 mov rax, r12
140760939 shr rax, 0x3e
14076093d jne 0x1407635db
140760943 mov r13d, 0
140760949 movabs rax, 0x7ffffffffffffffc
140760953 cmp r14, rax
140760956 ja 0x1407635de
14076095c mov r15, qword ptr [rsi + 0x38]
140760960 test r14, r14
140760963 je 0x140760990
140760965 movzx eax, byte ptr [rip + 0x2ed429d]
14076096c mov r13d, 4
140760972 mov edx, 4
140760977 mov rcx, r14
14076097a call 0x140613c10
14076097f test rax, rax
140760982 je 0x1407635de
140760988 mov rdi, rax
14076098b mov r13, r12
14076098e jmp 0x140760998
140760990 mov edi, 4
140760995 xor r13d, r13d
140760998 mov rcx, rdi
14076099b mov rdx, r15
14076099e mov r8, r14
1407609a1 call 0x142923c00
1407609a6 mov rax, qword ptr [rsi + 0x58]
1407609aa mov qword ptr [rbp + 0x1b0], rax
1407609b1 mov eax, dword ptr [rsi + 0x60]
1407609b4 mov dword ptr [rbp + 0x1b8], eax
1407609ba mov rax, qword ptr [rbp + 0x178]
1407609c1 mov rcx, qword ptr [rbp + 0x2b0]
1407609c8 movdqu xmm0, xmmword ptr [rsi + 0x48]
1407609cd mov edx, dword ptr [rbp + 0xf8]
1407609d3 mov r8, qword ptr [rbp + 0x50]
1407609d7 mov dword ptr [r8], edx
1407609da mov r8, qword ptr [rbp + 0x180]
1407609e1 mov rdx, qword ptr [r8 + 0x40]
1407609e5 mov qword ptr [rbp + 0x278], rdx
1407609ec mov rdx, qword ptr [rbp + 0x2d8]
1407609f3 mov qword ptr [rbp + 0x200], rdx
1407609fa mov rdx, qword ptr [rbp + 0x2b8]
140760a01 mov qword ptr [rbp + 0x208], rdx
140760a08 mov qword ptr [rbp + 0x210], rbx
140760a0f mov rdx, qword ptr [rbp + 0x298]
140760a16 mov qword ptr [rbp + 0x218], rdx
140760a1d mov rdx, qword ptr [rbp + 0x1a8]
140760a24 mov qword ptr [rbp + 0x220], rdx
140760a2b mov rdx, qword ptr [rbp + 0x128]
140760a32 mov qword ptr [rbp + 0x228], rdx
140760a39 mov qword ptr [rbp + 0x230], r13
140760a40 mov qword ptr [rbp + 0x238], rdi
140760a47 mov qword ptr [rbp + 0x240], r12
140760a4e movdqu xmmword ptr [rbp + 0x248], xmm0
140760a56 mov edx, dword ptr [rbp + 0x1b8]
140760a5c lea r9, [rbp + 0x208]
140760a63 mov dword ptr [r9 + 0x58], edx
140760a67 mov rdx, qword ptr [rbp + 0x1b0]
140760a6e mov qword ptr [r9 + 0x50], rdx
140760a72 mov qword ptr [rbp + 0x268], rcx
140760a79 mov qword ptr [rbp + 0x270], rax
140760a80 mov rdi, qword ptr [r8 + 0x30]
140760a84 cmp rdi, qword ptr [r8 + 0x20]
140760a88 jne 0x140760a9a
140760a8a mov rcx, qword ptr [rbp + 0x58]
140760a8e lea rdx, [rip + 0x24bba33]
140760a95 call 0x1405626f0
140760a9a mov rdx, qword ptr [rbp + 0x180]
140760aa1 mov rax, qword ptr [rdx + 0x28]
140760aa5 mov rcx, rdi
140760aa8 shl rcx, 7
140760aac movups xmm0, xmmword ptr [rbp + 0x200]
140760ab3 movdqu xmm1, xmmword ptr [rbp + 0x210]
140760abb movdqu xmm2, xmmword ptr [rbp + 0x220]
140760ac3 movups xmm3, xmmword ptr [rbp + 0x230]
140760aca movups xmm4, xmmword ptr [rbp + 0x240]
140760ad1 movups xmm5, xmmword ptr [rbp + 0x250]
140760ad8 movups xmm8, xmmword ptr [rbp + 0x260]
140760ae0 movups xmm10, xmmword ptr [rbp + 0x270]
140760ae8 movups xmmword ptr [rax + rcx + 0x70], xmm10
140760aee movups xmmword ptr [rax + rcx + 0x60], xmm8
140760af4 movups xmmword ptr [rax + rcx + 0x50], xmm5
140760af9 movups xmmword ptr [rax + rcx + 0x40], xmm4
140760afe movups xmmword ptr [rax + rcx + 0x30], xmm3
140760b03 movdqu xmmword ptr [rax + rcx + 0x20], xmm2
140760b09 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
140760b0f movups xmmword ptr [rax + rcx], xmm0
140760b13 inc rdi
140760b16 mov qword ptr [rdx + 0x30], rdi
140760b1a inc qword ptr [rdx + 0x40]
140760b1e mov r12, qword ptr [rbp + 0x40]
140760b22 movzx r13d, byte ptr [rbp + 0x2cf]
140760b2a mov r15, qword ptr [rbp + 0x48]
140760b2e mov ebx, dword ptr [rbp + 0xf4]
140760b34 movsd xmm0, qword ptr [rip + 0x231c724]
140760b3c movsd qword ptr [rbp + 0x1b8], xmm0
140760b44 mov eax, dword ptr [rbp + 0x1a0]
140760b4a mov dword ptr [rbp + 0x1b0], eax
140760b50 movd dword ptr [rbp + 0x1b4], xmm9
140760b59 mov rdx, qword ptr [rsi + 8]
140760b5d mov r8, qword ptr [rsi + 0x10]
140760b61 mov r9, qword ptr [rsi + 0x20]
140760b65 mov rax, qword ptr [rsi + 0x28]
140760b69 mov rcx, qword ptr [rsi + 0x38]
140760b6d mov r10, qword ptr [rsi + 0x40]
140760b71 lea r11, [rbp + 0x178]
140760b78 mov qword ptr [rbp + 0x200], r11
140760b7f lea r11, [rbp + 0xb0]
140760b86 mov qword ptr [rbp + 0x208], r11
140760b8d lea r11, [rbp + 0x2b0]
140760b94 mov qword ptr [rbp + 0x210], r11
140760b9b mov r11, qword ptr [rbp + 0x2d0]
140760ba2 mov qword ptr [rbp + 0x218], r11
140760ba9 lea r11, [rbp - 0x18]
140760bad mov qword ptr [rbp + 0x220], r11
140760bb4 lea r11, [rbp + 0x2e6]
140760bbb mov qword ptr [rbp + 0x228], r11
140760bc2 mov byte ptr [rbp + 0x2f7], 1
140760bc9 lea r11, [rbp + 0x200]
140760bd0 mov qword ptr [rsp + 0x38], r11
140760bd5 mov qword ptr [rsp + 0x30], r10
140760bda mov qword ptr [rsp + 0x28], rcx
140760bdf mov qword ptr [rsp + 0x20], rax
140760be4 lea rcx, [rbp + 0x1b0]
140760beb call 0x141232bb0
140760bf0 add rsi, 0x68
140760bf4 mov r14, qword ptr [rbp + 0x290]
140760bfb cmp rsi, r12
140760bfe jne 0x140760740
140760c04 mov rax, qword ptr [rbp + 0x2e8]
140760c0b lea rbx, [rax + 0x30]
140760c0f lea rcx, [rax + 0x60]
140760c13 mov qword ptr [rbp + 0x2d8], rcx
140760c1a lea rsi, [rax + 0x48]
140760c1e nop 
140760c20 cmp r15, r12
140760c23 je 0x140760190
140760c29 mov r13, r15
140760c2c cmp qword ptr [r15 + 0x10], 0
140760c31 je 0x14076331e
140760c37 mov rax, qword ptr [r13 + 8]
140760c3b movss xmm0, dword ptr [rax]
140760c3f ucomiss xmm0, xmm12
140760c43 jbe 0x140760ef8
140760c49 lea rcx, [r13 + 0x58]
140760c4d mov byte ptr [rbp + 0x2f7], 1
140760c54 call 0x141200370
140760c59 mov byte ptr [rbp + 0x2f7], 1
140760c60 mov ecx, eax
140760c62 call 0x141200280
140760c67 test rax, rax
140760c6a sete dil
140760c6e mov rax, qword ptr [r13 + 0x28]
140760c72 test rax, rax
140760c75 je 0x140763482
140760c7b cmp qword ptr [r13 + 0x10], 0
140760c80 je 0x140763466
140760c86 cmp rax, 1
140760c8a je 0x14076349e
140760c90 cmp qword ptr [r13 + 0x40], 0
140760c95 je 0x14076344a
140760c9b mov r14, qword ptr [r13 + 0x48]
140760c9f mov rax, qword ptr [r13 + 0x20]
140760ca3 mov rcx, qword ptr [r13 + 8]
140760ca7 movss xmm8, dword ptr [rcx]
140760cac movss xmm0, dword ptr [rax + 0xc]
140760cb1 subss xmm0, dword ptr [rax]
140760cb5 mov rcx, qword ptr [r13 + 0x38]
140760cb9 movaps xmm1, xmm0
140760cbc mulss xmm1, xmm0
140760cc0 movss xmm10, dword ptr [rax + 0x14]
140760cc6 subss xmm10, dword ptr [rax + 8]
140760ccc movaps xmm2, xmm10
140760cd0 mulss xmm2, xmm10
140760cd5 addss xmm2, xmm1
140760cd9 xorps xmm1, xmm1
140760cdc sqrtss xmm1, xmm2
140760ce0 movaps xmm14, xmm6
140760ce4 divss xmm14, xmm1
140760ce9 mulss xmm10, xmm14
140760cee movd xmm11, dword ptr [rcx]
140760cf3 mov ecx, dword ptr [rax + 8]
140760cf6 mov dword ptr [rbp + 0x208], ecx
140760cfc mov rax, qword ptr [rax]
140760cff mov qword ptr [rbp + 0x200], rax
140760d06 mov rdx, qword ptr [rbp + 0x2e8]
140760d0d mov rax, qword ptr [rdx + 0x70]
140760d11 test rax, rax
140760d14 sete cl
140760d17 shl rax, 3
140760d1b add rax, qword ptr [rdx + 0x68]
140760d1f mulss xmm14, xmm0
140760d24 add rax, -8
140760d28 sete dl
140760d2b or dl, cl
140760d2d jne 0x140760e00
140760d33 cmp qword ptr [rax], r14
140760d36 jne 0x140760e00
140760d3c mov rdx, qword ptr [rbp + 0x2e8]
140760d43 mov rax, qword ptr [rdx + 0x58]
140760d47 test rax, rax
140760d4a je 0x14076360d
140760d50 mov rcx, qword ptr [rdx + 0x50]
140760d54 lea rax, [rcx + rax*4]
140760d58 add rax, -4
140760d5c test rax, rax
140760d5f je 0x14076360d
140760d65 movss xmm0, dword ptr [rax]
140760d69 movaps xmm1, xmm8
140760d6d minss xmm1, xmm0
140760d71 cmpunordss xmm0, xmm0
140760d76 movaps xmm2, xmm0
140760d79 andps xmm2, xmm8
140760d7d andnps xmm0, xmm1
140760d80 orps xmm0, xmm2
140760d83 movss dword ptr [rax], xmm0
140760d87 mov rcx, qword ptr [rdx + 0x40]
140760d8b test rcx, rcx
140760d8e je 0x1407635f5
140760d94 mov rax, qword ptr [rdx + 0x38]
140760d98 lea rdx, [rcx + rcx*8]
140760d9c lea rdx, [rdx + rdx*2]
140760da0 add rdx, rcx
140760da3 lea rcx, [rax + rdx]
140760da7 add rcx, -0x1c
140760dab test rcx, rcx
140760dae je 0x1407635f5
140760db4 add rax, rdx
140760db7 ucomiss xmm8, dword ptr [rax - 8]
140760dbc jbe 0x140760ee7
140760dc2 movss dword ptr [rax - 0x1c], xmm14
140760dc8 movss dword ptr [rax - 0x18], xmm10
140760dce mov rcx, qword ptr [rbp + 0x200]
140760dd5 mov qword ptr [rax - 0x14], rcx
140760dd9 mov ecx, dword ptr [rbp + 0x208]
140760ddf mov dword ptr [rax - 0xc], ecx
140760de2 movss dword ptr [rax - 8], xmm8
140760de8 movd dword ptr [rax - 4], xmm11
140760dee jmp 0x140760ee7
140760df3 nop word ptr cs:[rax + rax]
140760e00 mov rax, qword ptr [rbp + 0x2e8]
140760e07 mov r15, qword ptr [rax + 0x58]
140760e0b cmp r15, qword ptr [rax + 0x48]
140760e0f jne 0x140760e27
140760e11 mov byte ptr [rbp + 0x2f7], 1
140760e18 mov rcx, rsi
140760e1b lea rdx, [rip + 0x231b656]
140760e22 call 0x140650050
140760e27 mov rcx, qword ptr [rbp + 0x2e8]
140760e2e mov rax, qword ptr [rcx + 0x50]
140760e32 movss dword ptr [rax + r15*4], xmm8
140760e38 inc r15
140760e3b mov qword ptr [rcx + 0x58], r15
140760e3f mov r15, qword ptr [rcx + 0x40]
140760e43 cmp r15, qword ptr [rcx + 0x30]
140760e47 jne 0x140760e5f
140760e49 mov byte ptr [rbp + 0x2f7], 1
140760e50 mov rcx, rbx
140760e53 lea rdx, [rip + 0x231b636]
140760e5a call 0x14078cb80
140760e5f mov r8, qword ptr [rbp + 0x2e8]
140760e66 mov rax, qword ptr [r8 + 0x38]
140760e6a lea rcx, [r15 + r15*8]
140760e6e lea rcx, [rcx + rcx*2]
140760e72 add rcx, r15
140760e75 movss dword ptr [rax + rcx], xmm14
140760e7b movss dword ptr [rax + rcx + 4], xmm10
140760e82 mov rdx, qword ptr [rbp + 0x200]
140760e89 mov qword ptr [rax + rcx + 8], rdx
140760e8e mov edx, dword ptr [rbp + 0x208]
140760e94 mov dword ptr [rax + rcx + 0x10], edx
140760e98 movss dword ptr [rax + rcx + 0x14], xmm8
140760e9f movd dword ptr [rax + rcx + 0x18], xmm11
140760ea6 inc r15
140760ea9 mov qword ptr [r8 + 0x40], r15
140760ead mov r15, qword ptr [r8 + 0x70]
140760eb1 cmp r15, qword ptr [r8 + 0x60]
140760eb5 jne 0x140760ed1
140760eb7 mov byte ptr [rbp + 0x2f7], 1
140760ebe mov rcx, qword ptr [rbp + 0x2d8]
140760ec5 lea rdx, [rip + 0x231b5dc]
140760ecc call 0x140b81df0
140760ed1 mov rcx, qword ptr [rbp + 0x2e8]
140760ed8 mov rax, qword ptr [rcx + 0x68]
140760edc mov qword ptr [rax + r15*8], r14
140760ee0 inc r15
140760ee3 mov qword ptr [rcx + 0x70], r15
140760ee7 mov rax, qword ptr [rbp + 0x2c0]
140760eee or al, dil
140760ef1 mov qword ptr [rbp + 0x2c0], rax
140760ef8 mov byte ptr [rbp + 0x2f7], 1
140760eff mov rcx, r13
140760f02 call 0x141234970
140760f07 lea rcx, [rax - 1]
140760f0b mov rdx, qword ptr [r13 + 0x10]
140760f0f cmp rcx, rdx
140760f12 jae 0x14076333a
140760f18 lea r15, [r13 + 0x68]
140760f1c mov rcx, qword ptr [r13 + 8]
140760f20 movss xmm0, dword ptr [rcx + rax*4 - 4]
140760f26 ucomiss xmm0, xmm12
140760f2a jbe 0x140760c20
140760f30 mov byte ptr [rbp + 0x2f7], 1
140760f37 mov rcx, r13
140760f3a call 0x141234980
140760f3f ucomiss xmm0, xmm12
140760f43 jbe 0x140760c20
140760f49 test byte ptr [rbp + 0x2c0], 1
140760f50 je 0x140760f7a
140760f52 lea rcx, [r13 + 0x58]
140760f56 mov byte ptr [rbp + 0x2f7], 1
140760f5d call 0x141200380
140760f62 mov byte ptr [rbp + 0x2f7], 1
140760f69 mov ecx, eax
140760f6b call 0x141200280
140760f70 cmp rax, 1
140760f74 je 0x140760c20
140760f7a mov r14, qword ptr [r13 + 0x50]
140760f7e mov byte ptr [rbp + 0x2f7], 1
140760f85 mov rcx, r13
140760f88 call 0x141234970
140760f8d dec rax
140760f90 mov rdx, qword ptr [r13 + 0x28]
140760f94 cmp rax, rdx
140760f97 jae 0x140763552
140760f9d mov rcx, qword ptr [r13 + 0x20]
140760fa1 lea rax, [rax + rax*2]
140760fa5 mov edx, dword ptr [rcx + rax*4 + 8]
140760fa9 mov dword ptr [rbp + 0x1b8], edx
140760faf mov rax, qword ptr [rcx + rax*4]
140760fb3 mov qword ptr [rbp + 0x1b0], rax
140760fba mov byte ptr [rbp + 0x2f7], 1
140760fc1 mov rcx, r13
140760fc4 call 0x141234970
140760fc9 lea rcx, [rax - 1]
140760fcd mov rdx, qword ptr [r13 + 0x10]
140760fd1 cmp rcx, rdx
140760fd4 jae 0x14076353a
140760fda mov rcx, qword ptr [r13 + 8]
140760fde movss xmm8, dword ptr [rcx + rax*4 - 4]
140760fe5 mov byte ptr [rbp + 0x2f7], 1
140760fec mov rcx, r13
140760fef call 0x141234970
140760ff4 dec rax
140760ff7 mov rdx, qword ptr [r13 + 0x28]
140760ffb cmp rax, rdx
140760ffe jae 0x14076351f
140761004 mov rcx, qword ptr [r13 + 0x20]
140761008 lea rax, [rax + rax*2]
14076100c movss xmm10, dword ptr [rcx + rax*4]
140761012 movss xmm11, dword ptr [rcx + rax*4 + 8]
140761019 mov byte ptr [rbp + 0x2f7], 1
140761020 mov rcx, r13
140761023 call 0x141234970
140761028 add rax, -2
14076102c mov rdx, qword ptr [r13 + 0x28]
140761030 cmp rax, rdx
140761033 jae 0x140763585
140761039 mov rcx, qword ptr [r13 + 0x20]
14076103d lea rax, [rax + rax*2]
140761041 movss xmm14, dword ptr [rcx + rax*4]
140761047 movss xmm15, dword ptr [rcx + rax*4 + 8]
14076104e mov byte ptr [rbp + 0x2f7], 1
140761055 mov rcx, r13
140761058 call 0x141234970
14076105d lea rcx, [rax - 1]
140761061 mov rdx, qword ptr [r13 + 0x40]
140761065 cmp rcx, rdx
140761068 jae 0x14076356d
14076106e dec r14
140761071 unpcklps xmm10, xmm11
140761075 unpcklps xmm14, xmm15
140761079 subps xmm10, xmm14
14076107d movaps xmm0, xmm10
140761081 mulps xmm0, xmm10
140761085 movshdup xmm1, xmm0
140761089 addss xmm1, xmm0
14076108d xorps xmm0, xmm0
140761090 sqrtss xmm0, xmm1
140761094 movaps xmm1, xmm6
140761097 divss xmm1, xmm0
14076109b movsldup xmm0, xmm1
14076109f mulps xmm0, xmm10
1407610a3 mov rcx, qword ptr [r13 + 0x38]
1407610a7 movd xmm1, dword ptr [rcx + rax*4 - 4]
1407610ad mov eax, dword ptr [rbp + 0x1b8]
1407610b3 lea rcx, [rbp + 0x208]
1407610ba mov dword ptr [rcx + 8], eax
1407610bd mov rax, qword ptr [rbp + 0x1b0]
1407610c4 mov qword ptr [rcx], rax
1407610c7 movlps qword ptr [rbp + 0x200], xmm0
1407610ce movss dword ptr [rbp + 0x214], xmm8
1407610d7 movd dword ptr [rbp + 0x218], xmm1
1407610df mov byte ptr [rbp + 0x2f7], 1
1407610e6 mov rcx, rbx
1407610e9 mov rdx, r14
1407610ec lea r8, [rbp + 0x200]
1407610f3 call 0x1407534d0
1407610f8 jmp 0x140760c20
1407610fd mov byte ptr [rbp + 0x2e5], 1
140761104 mov qword ptr [rsp + 0x20], 0x68
14076110d mov r9d, 8
140761113 lea rcx, [rbp + 0x160]
14076111a call 0x1428a7de0
14076111f mov rsi, qword ptr [rbp + 0x170]
140761126 jmp 0x1407602f8
14076112b mov rsi, qword ptr [rbp + 0x288]
140761132 mov rbx, qword ptr [rbp + 0x198]
140761139 movss xmm15, dword ptr [rip + 0x231c12e]
140761142 mov rax, qword ptr [rbp + 0x70]
140761146 mov ecx, dword ptr [rbp + 0xfc]
14076114c mov dword ptr [rax], ecx
14076114e cmp qword ptr [rsi + 0x18], 0
140761153 je 0x140762b55
140761159 mov rcx, qword ptr [rbp + 0xa0]
140761160 lea rdx, [rbp + 0x2b0]
140761167 call 0x1405b0d70
14076116c mov r12, qword ptr [rsi]
14076116f mov rcx, qword ptr [rsi + 8]
140761173 mov rdx, rcx
140761176 and rdx, rax
140761179 shr rax, 0x39
14076117d movd xmm0, eax
140761181 punpcklbw xmm0, xmm0
140761185 pshuflw xmm0, xmm0, 0
14076118a pshufd xmm0, xmm0, 0
14076118f lea rax, [r12 - 0x2a8]
140761197 mov r8, qword ptr [rbp + 0x2b0]
14076119e xor r9d, r9d
1407611a1 movdqu xmm1, xmmword ptr [r12 + rdx]
1407611a7 movdqa xmm2, xmm1
1407611ab pcmpeqb xmm2, xmm0
1407611af pmovmskb r10d, xmm2
1407611b4 test r10d, r10d
1407611b7 je 0x1407611f0
1407611b9 tzcnt r11d, r10d
1407611be add r11, rdx
1407611c1 and r11, rcx
1407611c4 neg r11
1407611c7 imul r11, r11, 0x2a8
1407611ce cmp r8, qword ptr [rax + r11]
1407611d2 je 0x140761220
1407611d4 lea r11d, [r10 - 1]
1407611d8 and r11w, r10w
1407611dc mov r10d, r11d
1407611df jne 0x1407611b9
1407611e1 nop word ptr cs:[rax + rax]
1407611f0 pcmpeqb xmm1, xmmword ptr [rip + 0x21c4798]
1407611f8 pmovmskb r10d, xmm1
1407611fd test r10d, r10d
140761200 jne 0x140762b55
140761206 add rdx, r9
140761209 add rdx, 0x10
14076120d add r9, 0x10
140761211 and rdx, rcx
140761214 jmp 0x1407611a1
140761216 nop word ptr cs:[rax + rax]
140761220 xorps xmm14, xmm14
140761224 mov qword ptr [rbp + 0x130], r14
14076122b mov qword ptr [rbp + 0x198], rbx
140761232 add r12, r11
140761235 imul rax, qword ptr [rbp + 0x170], 0x68
14076123d mov r13, qword ptr [rbp + 0x168]
140761244 add rax, r13
140761247 mov qword ptr [rbp + 0x1a8], rax
14076124e mov r14, r13
140761251 xor edi, edi
140761253 mov qword ptr [rbp + 0x2e8], r12
14076125a mov qword ptr [rbp + 0x128], r13
140761261 jmp 0x140761288
140761263 nop word ptr cs:[rax + rax]
140761270 mov r14, qword ptr [rbp + 0x298]
140761277 mov rdi, r15
14076127a mov r13, qword ptr [rbp + 0x128]
140761281 mov r12, qword ptr [rbp + 0x2e8]
140761288 cmp r14, qword ptr [rbp + 0x1a8]
14076128f je 0x140761770
140761295 lea rax, [r14 + 0x68]
140761299 mov qword ptr [rbp + 0x298], rax
1407612a0 lea r15, [rdi + 1]
1407612a4 lea rax, [r14 + 0x58]
1407612a8 mov qword ptr [rbp + 0x2d8], rax
1407612af mov eax, dword ptr [r14 + 0x60]
1407612b3 lea rcx, [rbp + 0x110]
1407612ba mov dword ptr [rcx + 8], eax
1407612bd mov rax, qword ptr [r14 + 0x58]
1407612c1 mov qword ptr [rcx], rax
1407612c4 mov qword ptr [rbp + 0x108], 0
1407612cf mov qword ptr [rbp + 0x2b8], r15
1407612d6 nop word ptr cs:[rax + rax]
1407612e0 mov byte ptr [rbp + 0x2f7], 1
1407612e7 lea rcx, [rbp + 0x200]
1407612ee lea rdx, [rbp + 0x110]
1407612f5 call 0x1412002f0
1407612fa test byte ptr [rbp + 0x200], 1
140761301 je 0x140761270
140761307 mov r13d, dword ptr [rbp + 0x204]
14076130e mov r12d, dword ptr [rbp + 0x208]
140761315 mov rsi, qword ptr [rbp + 0x108]
14076131c lea rax, [rsi + 1]
140761320 mov qword ptr [rbp + 0x108], rax
140761327 mov rdx, qword ptr [rbp + 0x158]
14076132e cmp rdi, rdx
140761331 jae 0x1407632e8
140761337 mov rax, qword ptr [rbp + 0x150]
14076133e movss xmm10, dword ptr [rax + rdi*8]
140761344 movss xmm9, dword ptr [rax + r15*8 - 4]
14076134b ucomiss xmm10, xmm14
14076134f jne 0x1407613b0
140761351 jp 0x1407613b0
140761353 ucomiss xmm9, xmm6
140761357 jne 0x1407613b0
140761359 jp 0x1407613b0
14076135b mov byte ptr [rbp + 0x2f7], 1
140761362 mov rcx, qword ptr [rbp + 0x2d8]
140761369 call 0x141200370
14076136e mov r9d, edx
140761371 mov byte ptr [rbp + 0x2f7], 1
140761378 mov ecx, r13d
14076137b mov edx, r12d
14076137e mov r8d, eax
140761381 call 0x1412002e0
140761386 mov rdx, qword ptr [r14 + 0x10]
14076138a cmp rax, rdx
14076138d jae 0x1407633c3
140761393 mov rcx, qword ptr [r14 + 8]
140761397 movss xmm8, dword ptr [rcx + rax*4]
14076139d jmp 0x14076170c
1407613a2 nop word ptr cs:[rax + rax]
1407613b0 mov byte ptr [rbp + 0x2f7], 1
1407613b7 mov rcx, r14
1407613ba call 0x141234970
1407613bf test rax, rax
1407613c2 js 0x140761400
1407613c4 cvtsi2ss xmm11, rax
1407613c9 test rsi, rsi
1407613cc mov dword ptr [rbp + 0x2d0], r13d
1407613d3 mov dword ptr [rbp + 0x2c0], r12d
1407613da jns 0x140761429
1407613dc mov rax, rsi
1407613df shr rax, 1
1407613e2 and esi, 1
1407613e5 or rsi, rax
1407613e8 xorps xmm8, xmm8
1407613ec cvtsi2ss xmm8, rsi
1407613f1 addss xmm8, xmm8
1407613f6 jmp 0x140761432
1407613f8 nop dword ptr [rax + rax]
140761400 mov rcx, rax
140761403 shr rcx, 1
140761406 and eax, 1
140761409 or rax, rcx
14076140c cvtsi2ss xmm11, rax
140761411 addss xmm11, xmm11
140761416 test rsi, rsi
140761419 mov dword ptr [rbp + 0x2d0], r13d
140761420 mov dword ptr [rbp + 0x2c0], r12d
140761427 js 0x1407613dc
140761429 xorps xmm8, xmm8
14076142d cvtsi2ss xmm8, rsi
140761432 addss xmm11, dword ptr [rip + 0x21f8ca9]
14076143b mov rbx, qword ptr [r14 + 0x28]
14076143f mov rax, rbx
140761442 mov ecx, 0xc
140761447 mul rcx
14076144a mov r12, rax
14076144d jo 0x1407634c0
140761453 mov r15, rdi
140761456 mov edi, 0
14076145b movabs rax, 0x7ffffffffffffffc
140761465 cmp r12, rax
140761468 ja 0x1407634c2
14076146e mov r13, r14
140761471 mov rcx, qword ptr [r13 + 8]
140761475 mov qword ptr [rbp + 0x1a0], rcx
14076147c mov rcx, qword ptr [r13 + 0x10]
140761480 mov qword ptr [rbp + 0x290], rcx
140761487 mov rsi, qword ptr [r13 + 0x20]
14076148b test r12, r12
14076148e je 0x1407614ba
140761490 movzx eax, byte ptr [rip + 0x2ed3772]
140761497 mov edi, 4
14076149c mov edx, 4
1407614a1 mov rcx, r12
1407614a4 call 0x140613c10
1407614a9 test rax, rax
1407614ac je 0x1407634c2
1407614b2 mov r14, rax
1407614b5 mov rdi, rbx
1407614b8 jmp 0x1407614c2
1407614ba mov r14d, 4
1407614c0 xor edi, edi
1407614c2 mov rcx, r14
1407614c5 mov rdx, rsi
1407614c8 mov r8, r12
1407614cb call 0x142923c00
1407614d0 mov qword ptr [rbp + 0x1b0], rdi
1407614d7 mov qword ptr [rbp + 0x1b8], r14
1407614de mov qword ptr [rbp + 0x1c0], rbx
1407614e5 mov byte ptr [rbp + 0x2f7], 1
1407614ec lea rcx, [rbp + 0x200]
1407614f3 lea rdx, [rbp + 0x1b0]
1407614fa xor r8d, r8d
1407614fd call 0x1411f9be0
140761502 mov rdi, r15
140761505 mov r14, r13
140761508 mov r15, qword ptr [rbp + 0x2b8]
14076150f mov r13d, dword ptr [rbp + 0x2d0]
140761516 mov r12d, dword ptr [rbp + 0x2c0]
14076151d xor eax, eax
14076151f cmp rax, qword ptr [rbp + 0x200]
140761526 jo 0x1407633de
14076152c mov rax, qword ptr [rbp + 0x230]
140761533 mov qword ptr [rbp + 0x1e0], rax
14076153a movdqu xmm0, xmmword ptr [rbp + 0x200]
140761542 movdqu xmm1, xmmword ptr [rbp + 0x210]
14076154a movdqu xmm2, xmmword ptr [rbp + 0x220]
140761552 movdqa xmmword ptr [rbp + 0x1d0], xmm2
14076155a movdqa xmmword ptr [rbp + 0x1c0], xmm1
140761562 movdqa xmmword ptr [rbp + 0x1b0], xmm0
14076156a mov rbx, qword ptr [rbp + 0x1b8]
140761571 mov rax, qword ptr [rbp + 0x1c0]
140761578 lea rax, [rax + rax*2]
14076157c lea r8, [rbx + rax*4]
140761580 lea rsi, [rbp + 0xb0]
140761587 mov rcx, rsi
14076158a mov rdx, rbx
14076158d lea r9, [rip + 0x24b75fc]
140761594 call 0x1402ba9e0
140761599 lea rcx, [rbp + 0x200]
1407615a0 mov rdx, rsi
1407615a3 xor r8d, r8d
1407615a6 call 0x1411f9920
1407615ab xor eax, eax
1407615ad cmp rax, qword ptr [rbp + 0x200]
1407615b4 jo 0x140763383
1407615ba mov rax, qword ptr [rbp + 0x230]
1407615c1 mov qword ptr [rbp + 0xe0], rax
1407615c8 movups xmm0, xmmword ptr [rbp + 0x200]
1407615cf movups xmm1, xmmword ptr [rbp + 0x210]
1407615d6 movdqu xmm2, xmmword ptr [rbp + 0x220]
1407615de movdqa xmmword ptr [rbp + 0xd0], xmm2
1407615e6 movaps xmmword ptr [rbp + 0xc0], xmm1
1407615ed movaps xmmword ptr [rbp + 0xb0], xmm0
1407615f4 mov rax, qword ptr [rbp + 0x1b0]
1407615fb test rax, rax
1407615fe je 0x140761616
140761600 shl rax, 2
140761604 lea rdx, [rax + rax*2]
140761608 mov r8d, 4
14076160e mov rcx, rbx
140761611 call 0x140613c20
140761616 mov rdx, qword ptr [rbp + 0x1c8]
14076161d test rdx, rdx
140761620 je 0x140761638
140761622 mov rcx, qword ptr [rbp + 0x1d0]
140761629 shl rdx, 2
14076162d mov r8d, 4
140761633 call 0x140613c20
140761638 divss xmm8, xmm11
14076163d subss xmm8, xmm10
140761642 subss xmm9, xmm10
140761647 divss xmm8, xmm9
14076164c ucomiss xmm8, xmm8
140761650 jp 0x140762b85
140761656 movd eax, xmm8
14076165b and eax, 0x7fffffff
140761660 cmp eax, 0x7f800000
140761665 je 0x140762ba2
14076166b xorps xmm0, xmm0
14076166e maxss xmm0, xmm8
140761673 movaps xmm1, xmm6
140761676 minss xmm1, xmm0
14076167a mov rcx, rsi
14076167d call 0x1411f8c10
140761682 mov ecx, eax
140761684 mov rdx, qword ptr [rbp + 0x290]
14076168b cmp rdx, rcx
14076168e jbe 0x140763425
140761694 lea rax, [rcx + 1]
140761698 cmp rax, rdx
14076169b jae 0x140763436
1407616a1 movaps xmm9, xmm0
1407616a5 mov rax, qword ptr [rbp + 0x1a0]
1407616ac movss xmm10, dword ptr [rax + rcx*4]
1407616b2 movss xmm8, dword ptr [rax + rcx*4 + 4]
1407616b9 mov rdx, qword ptr [rbp + 0xb0]
1407616c0 test rdx, rdx
1407616c3 je 0x1407616db
1407616c5 mov rcx, qword ptr [rbp + 0xb8]
1407616cc shl rdx, 3
1407616d0 mov r8d, 4
1407616d6 call 0x140613c20
1407616db mov rdx, qword ptr [rbp + 0xc8]
1407616e2 test rdx, rdx
1407616e5 je 0x1407616fd
1407616e7 mov rcx, qword ptr [rbp + 0xd0]
1407616ee shl rdx, 2
1407616f2 mov r8d, 4
1407616f8 call 0x140613c20
1407616fd subss xmm8, xmm10
140761702 mulss xmm8, xmm9
140761707 addss xmm8, xmm10
14076170c mov byte ptr [rbp + 0x2f7], 1
140761713 mov ecx, r13d
140761716 mov edx, r12d
140761719 call 0x141200280
14076171e mov rcx, qword ptr [rbp + 0x2e8]
140761725 mov rdx, qword ptr [rcx - 0x1a0]
14076172c cmp rax, rdx
14076172f jae 0x140763303
140761735 mov rcx, qword ptr [rcx - 0x1a8]
14076173c movss xmm0, dword ptr [rcx + rax*4]
140761741 movaps xmm1, xmm8
140761745 maxss xmm1, xmm0
140761749 cmpunordss xmm0, xmm0
14076174e movaps xmm2, xmm0
140761751 andnps xmm2, xmm1
140761754 andps xmm0, xmm8
140761758 orps xmm0, xmm2
14076175b movss dword ptr [rcx + rax*4], xmm0
140761760 jmp 0x1407612e0
140761765 nop word ptr cs:[rax + rax]
140761770 cmp dword ptr [r12 - 0x48], 2
140761776 jb 0x140762953
14076177c mov rsi, r12
14076177f lea rcx, [r12 - 0xb]
140761784 mov byte ptr [rbp + 0x2f7], 1
14076178b call 0x140833230
140761790 test al, al
140761792 mov r12, rsi
140761795 je 0x14076294b
14076179b movss xmm0, dword ptr [r12 - 0x18]
1407617a2 addss xmm0, xmm15
1407617a7 ucomiss xmm0, dword ptr [r12 - 0x1c]
1407617ad jbe 0x14076294b
1407617b3 mov r15, qword ptr [r12 - 0x1b8]
1407617bb lea rsi, [r15*4]
1407617c3 mov rax, r15
1407617c6 shr rax, 0x3e
1407617ca jne 0x14076371f
1407617d0 mov ebx, 0
1407617d5 movabs rax, 0x7ffffffffffffffc
1407617df cmp rsi, rax
1407617e2 movss xmm10, dword ptr [rip + 0x21c4085]
1407617eb ja 0x140763721
1407617f1 movaps xmm11, xmm15
1407617f5 mov rdi, qword ptr [r12 - 0x1c0]
1407617fd test rsi, rsi
140761800 je 0x140761830
140761802 movzx eax, byte ptr [rip + 0x2ed3400]
140761809 mov ebx, 4
14076180e mov edx, 4
140761813 mov rcx, rsi
140761816 call 0x140613c10
14076181b test rax, rax
14076181e je 0x140763721
140761824 mov rcx, rax
140761827 mov qword ptr [rbp + 0x2c0], r15
14076182e jmp 0x140761840
140761830 mov ecx, 4
140761835 mov qword ptr [rbp + 0x2c0], 0
140761840 mov qword ptr [rbp + 0x2d0], rcx
140761847 mov rdx, rdi
14076184a mov r8, rsi
14076184d call 0x142923c00
140761852 jmp 0x140761863
140761854 nop word ptr cs:[rax + rax]
140761860 mov r13, r14
140761863 cmp r13, qword ptr [rbp + 0x1a8]
14076186a je 0x140761a17
140761870 lea rsi, [r13 + 0x58]
140761874 mov rcx, rsi
140761877 call 0x141200370
14076187c mov ecx, eax
14076187e call 0x141200280
140761883 mov rdx, qword ptr [r12 - 0x208]
14076188b cmp rax, rdx
14076188e jae 0x140763652
140761894 cmp qword ptr [r13 + 0x10], 0
140761899 je 0x140763666
14076189f lea r14, [r13 + 0x68]
1407618a3 mov rcx, qword ptr [r13 + 8]
1407618a7 mov rdx, qword ptr [r12 - 0x210]
1407618af lea rax, [rax + rax*2]
1407618b3 movss xmm8, dword ptr [rdx + rax*4 + 4]
1407618ba addss xmm8, dword ptr [rcx]
1407618bf mov eax, dword ptr [rsi + 8]
1407618c2 mov dword ptr [rbp + 0x1b8], eax
1407618c8 mov rax, qword ptr [rsi]
1407618cb mov qword ptr [rbp + 0x1b0], rax
1407618d2 nop word ptr cs:[rax + rax]
1407618e0 lea rcx, [rbp + 0x200]
1407618e7 lea rdx, [rbp + 0x1b0]
1407618ee call 0x1412002f0
1407618f3 cmp dword ptr [rbp + 0x200], 1
1407618fa jne 0x140761990
140761900 mov edi, dword ptr [rbp + 0x204]
140761906 mov ebx, dword ptr [rbp + 0x208]
14076190c mov ecx, edi
14076190e mov edx, ebx
140761910 call 0x141200280
140761915 mov rdx, qword ptr [r12 - 0x208]
14076191d cmp rax, rdx
140761920 jae 0x1407634f4
140761926 mov rcx, qword ptr [r12 - 0x210]
14076192e lea rax, [rax + rax*2]
140761932 movss xmm9, dword ptr [rcx + rax*4 + 4]
140761939 mov rcx, rsi
14076193c call 0x141200370
140761941 mov r9d, edx
140761944 mov ecx, edi
140761946 mov edx, ebx
140761948 mov r8d, eax
14076194b call 0x1412002e0
140761950 mov rdx, qword ptr [r13 + 0x10]
140761954 cmp rax, rdx
140761957 jae 0x1407634e0
14076195d mov rcx, qword ptr [r13 + 8]
140761961 addss xmm9, dword ptr [rcx + rax*4]
140761967 movaps xmm0, xmm8
14076196b cmpunordss xmm0, xmm8
140761971 movaps xmm1, xmm0
140761974 andps xmm1, xmm9
140761978 maxss xmm9, xmm8
14076197d andnps xmm0, xmm9
140761981 orps xmm0, xmm1
140761984 movaps xmm8, xmm0
140761988 jmp 0x1407618e0
14076198d nop dword ptr [rax]
140761990 mov eax, dword ptr [rsi + 8]
140761993 mov dword ptr [rbp + 0x1b8], eax
140761999 mov rax, qword ptr [rsi]
14076199c mov qword ptr [rbp + 0x1b0], rax
1407619a3 addss xmm8, xmm11
1407619a8 nop dword ptr [rax + rax]
1407619b0 lea rcx, [rbp + 0x200]
1407619b7 lea rdx, [rbp + 0x1b0]
1407619be call 0x1412002f0
1407619c3 test byte ptr [rbp + 0x200], 1
1407619ca je 0x140761860
1407619d0 mov ecx, dword ptr [rbp + 0x204]
1407619d6 mov edx, dword ptr [rbp + 0x208]
1407619dc call 0x141200280
1407619e1 cmp rax, r15
1407619e4 jae 0x140763508
1407619ea mov rcx, qword ptr [rbp + 0x2d0]
1407619f1 movss xmm0, dword ptr [rcx + rax*4]
1407619f6 movaps xmm1, xmm8
1407619fa maxss xmm1, xmm0
1407619fe cmpunordss xmm0, xmm0
140761a03 movaps xmm2, xmm0
140761a06 andnps xmm2, xmm1
140761a09 andps xmm0, xmm8
140761a0d orps xmm0, xmm2
140761a10 movss dword ptr [rcx + rax*4], xmm0
140761a15 jmp 0x1407619b0
140761a17 cmp r15, 3
140761a1b movaps xmm5, xmmword ptr [rip + 0x21d923e]
140761a22 mov r10, qword ptr [rbp + 0x2d0]
140761a29 jb 0x140762821
140761a2f lea rax, [r15 - 2]
140761a33 movsd xmm0, qword ptr [r10]
140761a38 mov ecx, 1
140761a3d movaps xmm1, xmm0
140761a40 cmp rax, 8
140761a44 jb 0x140761ad9
140761a4a movaps xmm2, xmm0
140761a4d shufps xmm2, xmm0, 0
140761a51 mov rdx, rax
140761a54 and rdx, 0xfffffffffffffff8
140761a58 lea rcx, [rdx + 1]
140761a5c movddup xmm1, xmm0
140761a60 xor r8d, r8d
140761a63 nop word ptr cs:[rax + rax]
140761a70 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140761a76 shufps xmm2, xmm1, 0xff
140761a7a shufps xmm1, xmm3, 3
140761a7e shufps xmm1, xmm3, 0x98
140761a82 shufps xmm2, xmm3, 0x48
140761a86 addps xmm2, xmm3
140761a89 mulps xmm2, xmm5
140761a8c subps xmm2, xmm1
140761a8f mulps xmm2, xmm5
140761a92 addps xmm2, xmm1
140761a95 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
140761a9b movaps xmm4, xmm3
140761a9e shufps xmm4, xmm1, 3
140761aa2 shufps xmm4, xmm1, 0x98
140761aa6 shufps xmm3, xmm1, 0x4e
140761aaa addps xmm3, xmm1
140761aad mulps xmm3, xmm5
140761ab0 subps xmm3, xmm4
140761ab3 mulps xmm3, xmm5
140761ab6 addps xmm3, xmm4
140761ab9 movups xmmword ptr [r10 + r8*4 + 4], xmm2
140761abf movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140761ac5 add r8, 8
140761ac9 movaps xmm2, xmm4
140761acc cmp rdx, r8
140761acf jne 0x140761a70
140761ad1 cmp rax, rdx
140761ad4 je 0x140761b1d
140761ad6 movhlps xmm1, xmm1
140761ad9 inc rcx
140761adc nop dword ptr [rax]
140761ae0 movss xmm2, dword ptr [r10 + rcx*4]
140761ae6 movaps xmm3, xmm1
140761ae9 addss xmm3, xmm2
140761aed mulss xmm3, xmm10
140761af2 movshdup xmm4, xmm1
140761af6 subss xmm3, xmm4
140761afa mulss xmm3, xmm10
140761aff addss xmm3, xmm4
140761b03 movss dword ptr [r10 + rcx*4 - 4], xmm3
140761b0a shufps xmm2, xmm1, 0xd4
140761b0e shufps xmm2, xmm1, 0xf2
140761b12 inc rcx
140761b15 movaps xmm1, xmm2
140761b18 cmp r15, rcx
140761b1b jne 0x140761ae0
140761b1d movss xmm1, dword ptr [r10 + 4]
140761b23 mov ecx, 1
140761b28 cmp rax, 8
140761b2c jae 0x140761b36
140761b2e movaps xmm2, xmm0
140761b31 jmp 0x140761bc1
140761b36 mov rdx, rax
140761b39 and rdx, 0xfffffffffffffff8
140761b3d lea rcx, [rdx + 1]
140761b41 movaps xmm2, xmm0
140761b44 shufps xmm2, xmm0, 0
140761b48 shufps xmm1, xmm1, 0
140761b4c xor r8d, r8d
140761b4f nop 
140761b50 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140761b56 shufps xmm2, xmm1, 0xff
140761b5a shufps xmm1, xmm3, 3
140761b5e shufps xmm1, xmm3, 0x98
140761b62 shufps xmm2, xmm3, 0x48
140761b66 addps xmm2, xmm3
140761b69 mulps xmm2, xmm5
140761b6c subps xmm2, xmm1
140761b6f mulps xmm2, xmm5
140761b72 addps xmm2, xmm1
140761b75 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
140761b7b movaps xmm4, xmm3
140761b7e shufps xmm4, xmm1, 3
140761b82 shufps xmm4, xmm1, 0x98
140761b86 shufps xmm3, xmm1, 0x4e
140761b8a addps xmm3, xmm1
140761b8d mulps xmm3, xmm5
140761b90 subps xmm3, xmm4
140761b93 mulps xmm3, xmm5
140761b96 addps xmm3, xmm4
140761b99 movups xmmword ptr [r10 + r8*4 + 4], xmm2
140761b9f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140761ba5 add r8, 8
140761ba9 movaps xmm2, xmm4
140761bac cmp rdx, r8
140761baf jne 0x140761b50
140761bb1 cmp rax, rdx
140761bb4 je 0x140761c01
140761bb6 movaps xmm2, xmm1
140761bb9 unpckhpd xmm2, xmm1
140761bbd shufps xmm1, xmm1, 0xff
140761bc1 inc rcx
140761bc4 nop word ptr cs:[rax + rax]
140761bd0 movss xmm3, dword ptr [r10 + rcx*4]
140761bd6 addss xmm2, xmm3
140761bda mulss xmm2, xmm10
140761bdf subss xmm2, xmm1
140761be3 mulss xmm2, xmm10
140761be8 addss xmm2, xmm1
140761bec movss dword ptr [r10 + rcx*4 - 4], xmm2
140761bf3 inc rcx
140761bf6 movaps xmm2, xmm1
140761bf9 movaps xmm1, xmm3
140761bfc cmp r15, rcx
140761bff jne 0x140761bd0
140761c01 movss xmm1, dword ptr [r10 + 4]
140761c07 mov ecx, 1
140761c0c cmp rax, 8
140761c10 jae 0x140761c1a
140761c12 movaps xmm2, xmm0
140761c15 jmp 0x140761cb1
140761c1a mov rdx, rax
140761c1d and rdx, 0xfffffffffffffff8
140761c21 lea rcx, [rdx + 1]
140761c25 movaps xmm2, xmm0
140761c28 shufps xmm2, xmm0, 0
140761c2c shufps xmm1, xmm1, 0
140761c30 xor r8d, r8d
140761c33 nop word ptr cs:[rax + rax]
140761c40 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140761c46 shufps xmm2, xmm1, 0xff
140761c4a shufps xmm1, xmm3, 3
140761c4e shufps xmm1, xmm3, 0x98
140761c52 shufps xmm2, xmm3, 0x48
140761c56 addps xmm2, xmm3
140761c59 mulps xmm2, xmm5
140761c5c subps xmm2, xmm1
140761c5f mulps xmm2, xmm5
140761c62 addps xmm2, xmm1
140761c65 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
140761c6b movaps xmm4, xmm3
140761c6e shufps xmm4, xmm1, 3
140761c72 shufps xmm4, xmm1, 0x98
140761c76 shufps xmm3, xmm1, 0x4e
140761c7a addps xmm3, xmm1
140761c7d mulps xmm3, xmm5
140761c80 subps xmm3, xmm4
140761c83 mulps xmm3, xmm5
140761c86 addps xmm3, xmm4
140761c89 movups xmmword ptr [r10 + r8*4 + 4], xmm2
140761c8f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140761c95 add r8, 8
140761c99 movaps xmm2, xmm4
140761c9c cmp rdx, r8
140761c9f jne 0x140761c40
140761ca1 cmp rax, rdx
140761ca4 je 0x140761cf1
140761ca6 movaps xmm2, xmm1
140761ca9 unpckhpd xmm2, xmm1
140761cad shufps xmm1, xmm1, 0xff
140761cb1 inc rcx
140761cb4 nop word ptr cs:[rax + rax]
140761cc0 movss xmm3, dword ptr [r10 + rcx*4]
140761cc6 addss xmm2, xmm3
140761cca mulss xmm2, xmm10
140761ccf subss xmm2, xmm1
140761cd3 mulss xmm2, xmm10
140761cd8 addss xmm2, xmm1
140761cdc movss dword ptr [r10 + rcx*4 - 4], xmm2
140761ce3 inc rcx
140761ce6 movaps xmm2, xmm1
140761ce9 movaps xmm1, xmm3
140761cec cmp r15, rcx
140761cef jne 0x140761cc0
140761cf1 movss xmm1, dword ptr [r10 + 4]
140761cf7 mov ecx, 1
140761cfc cmp rax, 8
140761d00 jae 0x140761d0a
140761d02 movaps xmm2, xmm0
140761d05 jmp 0x140761da1
140761d0a mov rdx, rax
140761d0d and rdx, 0xfffffffffffffff8
140761d11 lea rcx, [rdx + 1]
140761d15 movaps xmm2, xmm0
140761d18 shufps xmm2, xmm0, 0
140761d1c shufps xmm1, xmm1, 0
140761d20 xor r8d, r8d
140761d23 nop word ptr cs:[rax + rax]
140761d30 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140761d36 shufps xmm2, xmm1, 0xff
140761d3a shufps xmm1, xmm3, 3
140761d3e shufps xmm1, xmm3, 0x98
140761d42 shufps xmm2, xmm3, 0x48
140761d46 addps xmm2, xmm3
140761d49 mulps xmm2, xmm5
140761d4c subps xmm2, xmm1
140761d4f mulps xmm2, xmm5
140761d52 addps xmm2, xmm1
140761d55 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
140761d5b movaps xmm4, xmm3
140761d5e shufps xmm4, xmm1, 3
140761d62 shufps xmm4, xmm1, 0x98
140761d66 shufps xmm3, xmm1, 0x4e
140761d6a addps xmm3, xmm1
140761d6d mulps xmm3, xmm5
140761d70 subps xmm3, xmm4
140761d73 mulps xmm3, xmm5
140761d76 addps xmm3, xmm4
140761d79 movups xmmword ptr [r10 + r8*4 + 4], xmm2
140761d7f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140761d85 add r8, 8
140761d89 movaps xmm2, xmm4
140761d8c cmp rdx, r8
140761d8f jne 0x140761d30
140761d91 cmp rax, rdx
140761d94 je 0x140761de1
140761d96 movaps xmm2, xmm1
140761d99 unpckhpd xmm2, xmm1
140761d9d shufps xmm1, xmm1, 0xff
140761da1 inc rcx
140761da4 nop word ptr cs:[rax + rax]
140761db0 movss xmm3, dword ptr [r10 + rcx*4]
140761db6 addss xmm2, xmm3
140761dba mulss xmm2, xmm10
140761dbf subss xmm2, xmm1
140761dc3 mulss xmm2, xmm10
140761dc8 addss xmm2, xmm1
140761dcc movss dword ptr [r10 + rcx*4 - 4], xmm2
140761dd3 inc rcx
140761dd6 movaps xmm2, xmm1
140761dd9 movaps xmm1, xmm3
140761ddc cmp r15, rcx
140761ddf jne 0x140761db0
140761de1 movss xmm1, dword ptr [r10 + 4]
140761de7 mov ecx, 1
140761dec cmp rax, 8
140761df0 jae 0x140761dfa
140761df2 movaps xmm2, xmm0
140761df5 jmp 0x140761e91
140761dfa mov rdx, rax
140761dfd and rdx, 0xfffffffffffffff8
140761e01 lea rcx, [rdx + 1]
140761e05 movaps xmm2, xmm0
140761e08 shufps xmm2, xmm0, 0
140761e0c shufps xmm1, xmm1, 0
140761e10 xor r8d, r8d
140761e13 nop word ptr cs:[rax + rax]
140761e20 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140761e26 shufps xmm2, xmm1, 0xff
140761e2a shufps xmm1, xmm3, 3
140761e2e shufps xmm1, xmm3, 0x98
140761e32 shufps xmm2, xmm3, 0x48
140761e36 addps xmm2, xmm3
140761e39 mulps xmm2, xmm5
140761e3c subps xmm2, xmm1
140761e3f mulps xmm2, xmm5
140761e42 addps xmm2, xmm1
140761e45 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
140761e4b movaps xmm4, xmm3
140761e4e shufps xmm4, xmm1, 3
140761e52 shufps xmm4, xmm1, 0x98
140761e56 shufps xmm3, xmm1, 0x4e
140761e5a addps xmm3, xmm1
140761e5d mulps xmm3, xmm5
140761e60 subps xmm3, xmm4
140761e63 mulps xmm3, xmm5
140761e66 addps xmm3, xmm4
140761e69 movups xmmword ptr [r10 + r8*4 + 4], xmm2
140761e6f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140761e75 add r8, 8
140761e79 movaps xmm2, xmm4
140761e7c cmp rdx, r8
140761e7f jne 0x140761e20
140761e81 cmp rax, rdx
140761e84 je 0x140761ed1
140761e86 movaps xmm2, xmm1
140761e89 unpckhpd xmm2, xmm1
140761e8d shufps xmm1, xmm1, 0xff
140761e91 inc rcx
140761e94 nop word ptr cs:[rax + rax]
140761ea0 movss xmm3, dword ptr [r10 + rcx*4]
140761ea6 addss xmm2, xmm3
140761eaa mulss xmm2, xmm10
140761eaf subss xmm2, xmm1
140761eb3 mulss xmm2, xmm10
140761eb8 addss xmm2, xmm1
140761ebc movss dword ptr [r10 + rcx*4 - 4], xmm2
140761ec3 inc rcx
140761ec6 movaps xmm2, xmm1
140761ec9 movaps xmm1, xmm3
140761ecc cmp r15, rcx
140761ecf jne 0x140761ea0
140761ed1 movss xmm1, dword ptr [r10 + 4]
140761ed7 mov ecx, 1
140761edc cmp rax, 8
140761ee0 jae 0x140761eea
140761ee2 movaps xmm2, xmm0
140761ee5 jmp 0x140761f81
140761eea mov rdx, rax
140761eed and rdx, 0xfffffffffffffff8
140761ef1 lea rcx, [rdx + 1]
140761ef5 movaps xmm2, xmm0
140761ef8 shufps xmm2, xmm0, 0
140761efc shufps xmm1, xmm1, 0
140761f00 xor r8d, r8d
140761f03 nop word ptr cs:[rax + rax]
140761f10 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140761f16 shufps xmm2, xmm1, 0xff
140761f1a shufps xmm1, xmm3, 3
140761f1e shufps xmm1, xmm3, 0x98
140761f22 shufps xmm2, xmm3, 0x48
140761f26 addps xmm2, xmm3
140761f29 mulps xmm2, xmm5
140761f2c subps xmm2, xmm1
140761f2f mulps xmm2, xmm5
140761f32 addps xmm2, xmm1
140761f35 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
140761f3b movaps xmm4, xmm3
140761f3e shufps xmm4, xmm1, 3
140761f42 shufps xmm4, xmm1, 0x98
140761f46 shufps xmm3, xmm1, 0x4e
140761f4a addps xmm3, xmm1
140761f4d mulps xmm3, xmm5
140761f50 subps xmm3, xmm4
140761f53 mulps xmm3, xmm5
140761f56 addps xmm3, xmm4
140761f59 movups xmmword ptr [r10 + r8*4 + 4], xmm2
140761f5f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140761f65 add r8, 8
140761f69 movaps xmm2, xmm4
140761f6c cmp rdx, r8
140761f6f jne 0x140761f10
140761f71 cmp rax, rdx
140761f74 je 0x140761fc1
140761f76 movaps xmm2, xmm1
140761f79 unpckhpd xmm2, xmm1
140761f7d shufps xmm1, xmm1, 0xff
140761f81 inc rcx
140761f84 nop word ptr cs:[rax + rax]
140761f90 movss xmm3, dword ptr [r10 + rcx*4]
140761f96 addss xmm2, xmm3
140761f9a mulss xmm2, xmm10
140761f9f subss xmm2, xmm1
140761fa3 mulss xmm2, xmm10
140761fa8 addss xmm2, xmm1
140761fac movss dword ptr [r10 + rcx*4 - 4], xmm2
140761fb3 inc rcx
140761fb6 movaps xmm2, xmm1
140761fb9 movaps xmm1, xmm3
140761fbc cmp r15, rcx
140761fbf jne 0x140761f90
140761fc1 movss xmm1, dword ptr [r10 + 4]
140761fc7 mov ecx, 1
140761fcc cmp rax, 8
140761fd0 jae 0x140761fda
140761fd2 movaps xmm2, xmm0
140761fd5 jmp 0x140762071
140761fda mov rdx, rax
140761fdd and rdx, 0xfffffffffffffff8
140761fe1 lea rcx, [rdx + 1]
140761fe5 movaps xmm2, xmm0
140761fe8 shufps xmm2, xmm0, 0
140761fec shufps xmm1, xmm1, 0
140761ff0 xor r8d, r8d
140761ff3 nop word ptr cs:[rax + rax]
140762000 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140762006 shufps xmm2, xmm1, 0xff
14076200a shufps xmm1, xmm3, 3
14076200e shufps xmm1, xmm3, 0x98
140762012 shufps xmm2, xmm3, 0x48
140762016 addps xmm2, xmm3
140762019 mulps xmm2, xmm5
14076201c subps xmm2, xmm1
14076201f mulps xmm2, xmm5
140762022 addps xmm2, xmm1
140762025 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
14076202b movaps xmm4, xmm3
14076202e shufps xmm4, xmm1, 3
140762032 shufps xmm4, xmm1, 0x98
140762036 shufps xmm3, xmm1, 0x4e
14076203a addps xmm3, xmm1
14076203d mulps xmm3, xmm5
140762040 subps xmm3, xmm4
140762043 mulps xmm3, xmm5
140762046 addps xmm3, xmm4
140762049 movups xmmword ptr [r10 + r8*4 + 4], xmm2
14076204f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140762055 add r8, 8
140762059 movaps xmm2, xmm4
14076205c cmp rdx, r8
14076205f jne 0x140762000
140762061 cmp rax, rdx
140762064 je 0x1407620b1
140762066 movaps xmm2, xmm1
140762069 unpckhpd xmm2, xmm1
14076206d shufps xmm1, xmm1, 0xff
140762071 inc rcx
140762074 nop word ptr cs:[rax + rax]
140762080 movss xmm3, dword ptr [r10 + rcx*4]
140762086 addss xmm2, xmm3
14076208a mulss xmm2, xmm10
14076208f subss xmm2, xmm1
140762093 mulss xmm2, xmm10
140762098 addss xmm2, xmm1
14076209c movss dword ptr [r10 + rcx*4 - 4], xmm2
1407620a3 inc rcx
1407620a6 movaps xmm2, xmm1
1407620a9 movaps xmm1, xmm3
1407620ac cmp r15, rcx
1407620af jne 0x140762080
1407620b1 movss xmm1, dword ptr [r10 + 4]
1407620b7 mov ecx, 1
1407620bc cmp rax, 8
1407620c0 jae 0x1407620ca
1407620c2 movaps xmm2, xmm0
1407620c5 jmp 0x140762161
1407620ca mov rdx, rax
1407620cd and rdx, 0xfffffffffffffff8
1407620d1 lea rcx, [rdx + 1]
1407620d5 movaps xmm2, xmm0
1407620d8 shufps xmm2, xmm0, 0
1407620dc shufps xmm1, xmm1, 0
1407620e0 xor r8d, r8d
1407620e3 nop word ptr cs:[rax + rax]
1407620f0 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
1407620f6 shufps xmm2, xmm1, 0xff
1407620fa shufps xmm1, xmm3, 3
1407620fe shufps xmm1, xmm3, 0x98
140762102 shufps xmm2, xmm3, 0x48
140762106 addps xmm2, xmm3
140762109 mulps xmm2, xmm5
14076210c subps xmm2, xmm1
14076210f mulps xmm2, xmm5
140762112 addps xmm2, xmm1
140762115 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
14076211b movaps xmm4, xmm3
14076211e shufps xmm4, xmm1, 3
140762122 shufps xmm4, xmm1, 0x98
140762126 shufps xmm3, xmm1, 0x4e
14076212a addps xmm3, xmm1
14076212d mulps xmm3, xmm5
140762130 subps xmm3, xmm4
140762133 mulps xmm3, xmm5
140762136 addps xmm3, xmm4
140762139 movups xmmword ptr [r10 + r8*4 + 4], xmm2
14076213f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140762145 add r8, 8
140762149 movaps xmm2, xmm4
14076214c cmp rdx, r8
14076214f jne 0x1407620f0
140762151 cmp rax, rdx
140762154 je 0x1407621a1
140762156 movaps xmm2, xmm1
140762159 unpckhpd xmm2, xmm1
14076215d shufps xmm1, xmm1, 0xff
140762161 inc rcx
140762164 nop word ptr cs:[rax + rax]
140762170 movss xmm3, dword ptr [r10 + rcx*4]
140762176 addss xmm2, xmm3
14076217a mulss xmm2, xmm10
14076217f subss xmm2, xmm1
140762183 mulss xmm2, xmm10
140762188 addss xmm2, xmm1
14076218c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762193 inc rcx
140762196 movaps xmm2, xmm1
140762199 movaps xmm1, xmm3
14076219c cmp r15, rcx
14076219f jne 0x140762170
1407621a1 movss xmm1, dword ptr [r10 + 4]
1407621a7 mov ecx, 1
1407621ac cmp rax, 8
1407621b0 jae 0x1407621ba
1407621b2 movaps xmm2, xmm0
1407621b5 jmp 0x140762251
1407621ba mov rdx, rax
1407621bd and rdx, 0xfffffffffffffff8
1407621c1 lea rcx, [rdx + 1]
1407621c5 movaps xmm2, xmm0
1407621c8 shufps xmm2, xmm0, 0
1407621cc shufps xmm1, xmm1, 0
1407621d0 xor r8d, r8d
1407621d3 nop word ptr cs:[rax + rax]
1407621e0 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
1407621e6 shufps xmm2, xmm1, 0xff
1407621ea shufps xmm1, xmm3, 3
1407621ee shufps xmm1, xmm3, 0x98
1407621f2 shufps xmm2, xmm3, 0x48
1407621f6 addps xmm2, xmm3
1407621f9 mulps xmm2, xmm5
1407621fc subps xmm2, xmm1
1407621ff mulps xmm2, xmm5
140762202 addps xmm2, xmm1
140762205 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
14076220b movaps xmm4, xmm3
14076220e shufps xmm4, xmm1, 3
140762212 shufps xmm4, xmm1, 0x98
140762216 shufps xmm3, xmm1, 0x4e
14076221a addps xmm3, xmm1
14076221d mulps xmm3, xmm5
140762220 subps xmm3, xmm4
140762223 mulps xmm3, xmm5
140762226 addps xmm3, xmm4
140762229 movups xmmword ptr [r10 + r8*4 + 4], xmm2
14076222f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140762235 add r8, 8
140762239 movaps xmm2, xmm4
14076223c cmp rdx, r8
14076223f jne 0x1407621e0
140762241 cmp rax, rdx
140762244 je 0x140762291
140762246 movaps xmm2, xmm1
140762249 unpckhpd xmm2, xmm1
14076224d shufps xmm1, xmm1, 0xff
140762251 inc rcx
140762254 nop word ptr cs:[rax + rax]
140762260 movss xmm3, dword ptr [r10 + rcx*4]
140762266 addss xmm2, xmm3
14076226a mulss xmm2, xmm10
14076226f subss xmm2, xmm1
140762273 mulss xmm2, xmm10
140762278 addss xmm2, xmm1
14076227c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762283 inc rcx
140762286 movaps xmm2, xmm1
140762289 movaps xmm1, xmm3
14076228c cmp r15, rcx
14076228f jne 0x140762260
140762291 movss xmm1, dword ptr [r10 + 4]
140762297 mov ecx, 1
14076229c cmp rax, 8
1407622a0 jae 0x1407622aa
1407622a2 movaps xmm2, xmm0
1407622a5 jmp 0x140762341
1407622aa mov rdx, rax
1407622ad and rdx, 0xfffffffffffffff8
1407622b1 lea rcx, [rdx + 1]
1407622b5 movaps xmm2, xmm0
1407622b8 shufps xmm2, xmm0, 0
1407622bc shufps xmm1, xmm1, 0
1407622c0 xor r8d, r8d
1407622c3 nop word ptr cs:[rax + rax]
1407622d0 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
1407622d6 shufps xmm2, xmm1, 0xff
1407622da shufps xmm1, xmm3, 3
1407622de shufps xmm1, xmm3, 0x98
1407622e2 shufps xmm2, xmm3, 0x48
1407622e6 addps xmm2, xmm3
1407622e9 mulps xmm2, xmm5
1407622ec subps xmm2, xmm1
1407622ef mulps xmm2, xmm5
1407622f2 addps xmm2, xmm1
1407622f5 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
1407622fb movaps xmm4, xmm3
1407622fe shufps xmm4, xmm1, 3
140762302 shufps xmm4, xmm1, 0x98
140762306 shufps xmm3, xmm1, 0x4e
14076230a addps xmm3, xmm1
14076230d mulps xmm3, xmm5
140762310 subps xmm3, xmm4
140762313 mulps xmm3, xmm5
140762316 addps xmm3, xmm4
140762319 movups xmmword ptr [r10 + r8*4 + 4], xmm2
14076231f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140762325 add r8, 8
140762329 movaps xmm2, xmm4
14076232c cmp rdx, r8
14076232f jne 0x1407622d0
140762331 cmp rax, rdx
140762334 je 0x140762381
140762336 movaps xmm2, xmm1
140762339 unpckhpd xmm2, xmm1
14076233d shufps xmm1, xmm1, 0xff
140762341 inc rcx
140762344 nop word ptr cs:[rax + rax]
140762350 movss xmm3, dword ptr [r10 + rcx*4]
140762356 addss xmm2, xmm3
14076235a mulss xmm2, xmm10
14076235f subss xmm2, xmm1
140762363 mulss xmm2, xmm10
140762368 addss xmm2, xmm1
14076236c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762373 inc rcx
140762376 movaps xmm2, xmm1
140762379 movaps xmm1, xmm3
14076237c cmp r15, rcx
14076237f jne 0x140762350
140762381 movss xmm1, dword ptr [r10 + 4]
140762387 mov ecx, 1
14076238c cmp rax, 8
140762390 jae 0x14076239a
140762392 movaps xmm2, xmm0
140762395 jmp 0x140762431
14076239a mov rdx, rax
14076239d and rdx, 0xfffffffffffffff8
1407623a1 lea rcx, [rdx + 1]
1407623a5 movaps xmm2, xmm0
1407623a8 shufps xmm2, xmm0, 0
1407623ac shufps xmm1, xmm1, 0
1407623b0 xor r8d, r8d
1407623b3 nop word ptr cs:[rax + rax]
1407623c0 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
1407623c6 shufps xmm2, xmm1, 0xff
1407623ca shufps xmm1, xmm3, 3
1407623ce shufps xmm1, xmm3, 0x98
1407623d2 shufps xmm2, xmm3, 0x48
1407623d6 addps xmm2, xmm3
1407623d9 mulps xmm2, xmm5
1407623dc subps xmm2, xmm1
1407623df mulps xmm2, xmm5
1407623e2 addps xmm2, xmm1
1407623e5 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
1407623eb movaps xmm4, xmm3
1407623ee shufps xmm4, xmm1, 3
1407623f2 shufps xmm4, xmm1, 0x98
1407623f6 shufps xmm3, xmm1, 0x4e
1407623fa addps xmm3, xmm1
1407623fd mulps xmm3, xmm5
140762400 subps xmm3, xmm4
140762403 mulps xmm3, xmm5
140762406 addps xmm3, xmm4
140762409 movups xmmword ptr [r10 + r8*4 + 4], xmm2
14076240f movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140762415 add r8, 8
140762419 movaps xmm2, xmm4
14076241c cmp rdx, r8
14076241f jne 0x1407623c0
140762421 cmp rax, rdx
140762424 je 0x140762471
140762426 movaps xmm2, xmm1
140762429 unpckhpd xmm2, xmm1
14076242d shufps xmm1, xmm1, 0xff
140762431 inc rcx
140762434 nop word ptr cs:[rax + rax]
140762440 movss xmm3, dword ptr [r10 + rcx*4]
140762446 addss xmm2, xmm3
14076244a mulss xmm2, xmm10
14076244f subss xmm2, xmm1
140762453 mulss xmm2, xmm10
140762458 addss xmm2, xmm1
14076245c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762463 inc rcx
140762466 movaps xmm2, xmm1
140762469 movaps xmm1, xmm3
14076246c cmp r15, rcx
14076246f jne 0x140762440
140762471 movss xmm1, dword ptr [r10 + 4]
140762477 mov ecx, 1
14076247c cmp rax, 8
140762480 jae 0x14076248a
140762482 movaps xmm2, xmm0
140762485 jmp 0x140762521
14076248a mov rdx, rax
14076248d and rdx, 0xfffffffffffffff8
140762491 lea rcx, [rdx + 1]
140762495 movaps xmm2, xmm0
140762498 shufps xmm2, xmm0, 0
14076249c shufps xmm1, xmm1, 0
1407624a0 xor r8d, r8d
1407624a3 nop word ptr cs:[rax + rax]
1407624b0 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
1407624b6 shufps xmm2, xmm1, 0xff
1407624ba shufps xmm1, xmm3, 3
1407624be shufps xmm1, xmm3, 0x98
1407624c2 shufps xmm2, xmm3, 0x48
1407624c6 addps xmm2, xmm3
1407624c9 mulps xmm2, xmm5
1407624cc subps xmm2, xmm1
1407624cf mulps xmm2, xmm5
1407624d2 addps xmm2, xmm1
1407624d5 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
1407624db movaps xmm4, xmm3
1407624de shufps xmm4, xmm1, 3
1407624e2 shufps xmm4, xmm1, 0x98
1407624e6 shufps xmm3, xmm1, 0x4e
1407624ea addps xmm3, xmm1
1407624ed mulps xmm3, xmm5
1407624f0 subps xmm3, xmm4
1407624f3 mulps xmm3, xmm5
1407624f6 addps xmm3, xmm4
1407624f9 movups xmmword ptr [r10 + r8*4 + 4], xmm2
1407624ff movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
140762505 add r8, 8
140762509 movaps xmm2, xmm4
14076250c cmp rdx, r8
14076250f jne 0x1407624b0
140762511 cmp rax, rdx
140762514 je 0x140762561
140762516 movaps xmm2, xmm1
140762519 unpckhpd xmm2, xmm1
14076251d shufps xmm1, xmm1, 0xff
140762521 inc rcx
140762524 nop word ptr cs:[rax + rax]
140762530 movss xmm3, dword ptr [r10 + rcx*4]
140762536 addss xmm2, xmm3
14076253a mulss xmm2, xmm10
14076253f subss xmm2, xmm1
140762543 mulss xmm2, xmm10
140762548 addss xmm2, xmm1
14076254c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762553 inc rcx
140762556 movaps xmm2, xmm1
140762559 movaps xmm1, xmm3
14076255c cmp r15, rcx
14076255f jne 0x140762530
140762561 movss xmm1, dword ptr [r10 + 4]
140762567 mov ecx, 1
14076256c cmp rax, 8
140762570 jae 0x14076257a
140762572 movaps xmm2, xmm0
140762575 jmp 0x140762611
14076257a mov rdx, rax
14076257d and rdx, 0xfffffffffffffff8
140762581 lea rcx, [rdx + 1]
140762585 movaps xmm2, xmm0
140762588 shufps xmm2, xmm0, 0
14076258c shufps xmm1, xmm1, 0
140762590 xor r8d, r8d
140762593 nop word ptr cs:[rax + rax]
1407625a0 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
1407625a6 shufps xmm2, xmm1, 0xff
1407625aa shufps xmm1, xmm3, 3
1407625ae shufps xmm1, xmm3, 0x98
1407625b2 shufps xmm2, xmm3, 0x48
1407625b6 addps xmm2, xmm3
1407625b9 mulps xmm2, xmm5
1407625bc subps xmm2, xmm1
1407625bf mulps xmm2, xmm5
1407625c2 addps xmm2, xmm1
1407625c5 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
1407625cb movaps xmm4, xmm3
1407625ce shufps xmm4, xmm1, 3
1407625d2 shufps xmm4, xmm1, 0x98
1407625d6 shufps xmm3, xmm1, 0x4e
1407625da addps xmm3, xmm1
1407625dd mulps xmm3, xmm5
1407625e0 subps xmm3, xmm4
1407625e3 mulps xmm3, xmm5
1407625e6 addps xmm3, xmm4
1407625e9 movups xmmword ptr [r10 + r8*4 + 4], xmm2
1407625ef movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
1407625f5 add r8, 8
1407625f9 movaps xmm2, xmm4
1407625fc cmp rdx, r8
1407625ff jne 0x1407625a0
140762601 cmp rax, rdx
140762604 je 0x140762651
140762606 movaps xmm2, xmm1
140762609 unpckhpd xmm2, xmm1
14076260d shufps xmm1, xmm1, 0xff
140762611 inc rcx
140762614 nop word ptr cs:[rax + rax]
140762620 movss xmm3, dword ptr [r10 + rcx*4]
140762626 addss xmm2, xmm3
14076262a mulss xmm2, xmm10
14076262f subss xmm2, xmm1
140762633 mulss xmm2, xmm10
140762638 addss xmm2, xmm1
14076263c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762643 inc rcx
140762646 movaps xmm2, xmm1
140762649 movaps xmm1, xmm3
14076264c cmp r15, rcx
14076264f jne 0x140762620
140762651 movss xmm1, dword ptr [r10 + 4]
140762657 mov ecx, 1
14076265c cmp rax, 8
140762660 jae 0x14076266a
140762662 movaps xmm2, xmm0
140762665 jmp 0x140762701
14076266a mov rdx, rax
14076266d and rdx, 0xfffffffffffffff8
140762671 lea rcx, [rdx + 1]
140762675 movaps xmm2, xmm0
140762678 shufps xmm2, xmm0, 0
14076267c shufps xmm1, xmm1, 0
140762680 xor r8d, r8d
140762683 nop word ptr cs:[rax + rax]
140762690 movups xmm3, xmmword ptr [r10 + r8*4 + 8]
140762696 shufps xmm2, xmm1, 0xff
14076269a shufps xmm1, xmm3, 3
14076269e shufps xmm1, xmm3, 0x98
1407626a2 shufps xmm2, xmm3, 0x48
1407626a6 addps xmm2, xmm3
1407626a9 mulps xmm2, xmm5
1407626ac subps xmm2, xmm1
1407626af mulps xmm2, xmm5
1407626b2 addps xmm2, xmm1
1407626b5 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
1407626bb movaps xmm4, xmm3
1407626be shufps xmm4, xmm1, 3
1407626c2 shufps xmm4, xmm1, 0x98
1407626c6 shufps xmm3, xmm1, 0x4e
1407626ca addps xmm3, xmm1
1407626cd mulps xmm3, xmm5
1407626d0 subps xmm3, xmm4
1407626d3 mulps xmm3, xmm5
1407626d6 addps xmm3, xmm4
1407626d9 movups xmmword ptr [r10 + r8*4 + 4], xmm2
1407626df movups xmmword ptr [r10 + r8*4 + 0x14], xmm3
1407626e5 add r8, 8
1407626e9 movaps xmm2, xmm4
1407626ec cmp rdx, r8
1407626ef jne 0x140762690
1407626f1 cmp rax, rdx
1407626f4 je 0x140762741
1407626f6 movaps xmm2, xmm1
1407626f9 unpckhpd xmm2, xmm1
1407626fd shufps xmm1, xmm1, 0xff
140762701 inc rcx
140762704 nop word ptr cs:[rax + rax]
140762710 movss xmm3, dword ptr [r10 + rcx*4]
140762716 addss xmm2, xmm3
14076271a mulss xmm2, xmm10
14076271f subss xmm2, xmm1
140762723 mulss xmm2, xmm10
140762728 addss xmm2, xmm1
14076272c movss dword ptr [r10 + rcx*4 - 4], xmm2
140762733 inc rcx
140762736 movaps xmm2, xmm1
140762739 movaps xmm1, xmm3
14076273c cmp r15, rcx
14076273f jne 0x140762710
140762741 movss xmm1, dword ptr [r10 + 4]
140762747 mov ecx, 1
14076274c cmp rax, 8
140762750 jb 0x1407627e1
140762756 mov rdx, rax
140762759 and rdx, 0xfffffffffffffff8
14076275d lea rcx, [rdx + 1]
140762761 shufps xmm0, xmm0, 0
140762765 shufps xmm1, xmm1, 0
140762769 xor r8d, r8d
14076276c nop dword ptr [rax]
140762770 movups xmm2, xmmword ptr [r10 + r8*4 + 8]
140762776 shufps xmm0, xmm1, 0xff
14076277a shufps xmm1, xmm2, 3
14076277e shufps xmm1, xmm2, 0x98
140762782 shufps xmm0, xmm2, 0x48
140762786 addps xmm0, xmm2
140762789 mulps xmm0, xmm5
14076278c subps xmm0, xmm1
14076278f mulps xmm0, xmm5
140762792 addps xmm0, xmm1
140762795 movups xmm1, xmmword ptr [r10 + r8*4 + 0x18]
14076279b movaps xmm3, xmm2
14076279e shufps xmm3, xmm1, 3
1407627a2 shufps xmm3, xmm1, 0x98
1407627a6 shufps xmm2, xmm1, 0x4e
1407627aa addps xmm2, xmm1
1407627ad mulps xmm2, xmm5
1407627b0 subps xmm2, xmm3
1407627b3 mulps xmm2, xmm5
1407627b6 addps xmm2, xmm3
1407627b9 movups xmmword ptr [r10 + r8*4 + 4], xmm0
1407627bf movups xmmword ptr [r10 + r8*4 + 0x14], xmm2
1407627c5 add r8, 8
1407627c9 movaps xmm0, xmm3
1407627cc cmp rdx, r8
1407627cf jne 0x140762770
1407627d1 cmp rax, rdx
1407627d4 je 0x140762821
1407627d6 movaps xmm0, xmm1
1407627d9 unpckhpd xmm0, xmm1
1407627dd shufps xmm1, xmm1, 0xff
1407627e1 inc rcx
1407627e4 nop word ptr cs:[rax + rax]
1407627f0 movss xmm2, dword ptr [r10 + rcx*4]
1407627f6 addss xmm0, xmm2
1407627fa mulss xmm0, xmm10
1407627ff subss xmm0, xmm1
140762803 mulss xmm0, xmm10
140762808 addss xmm0, xmm1
14076280c movss dword ptr [r10 + rcx*4 - 4], xmm0
140762813 inc rcx
140762816 movaps xmm0, xmm1
140762819 movaps xmm1, xmm2
14076281c cmp r15, rcx
14076281f jne 0x1407627f0
140762821 mov rax, qword ptr [r12 - 0x1b8]
140762829 test rax, rax
14076282c je 0x14076292d
140762832 mov rcx, qword ptr [r12 - 0x1c0]
14076283a lea r8, [rax - 1]
14076283e cmp r15, r8
140762841 mov rdx, r8
140762844 cmovb rdx, r15
140762848 cmp rdx, 8
14076284c jae 0x140762855
14076284e xor edx, edx
140762850 jmp 0x1407628f0
140762855 cmp r15, r8
140762858 cmovb r8, r15
14076285c lea r9, [r10 + r8*4]
140762860 add r9, 4
140762864 cmp rcx, r9
140762867 jae 0x14076287a
140762869 lea r8, [rcx + r8*4]
14076286d add r8, 4
140762871 cmp r8, r10
140762874 jbe 0x14076287a
140762876 xor edx, edx
140762878 jmp 0x1407628f0
14076287a inc rdx
14076287d mov r8d, edx
140762880 and r8d, 7
140762884 mov r9d, 8
14076288a cmove r8, r9
14076288e sub rdx, r8
140762891 xor r8d, r8d
140762894 movups xmm0, xmmword ptr [rcx + r8*4]
140762899 movups xmm1, xmmword ptr [rcx + r8*4 + 0x10]
14076289f movups xmm2, xmmword ptr [r10 + r8*4]
1407628a4 movups xmm3, xmmword ptr [r10 + r8*4 + 0x10]
1407628aa movaps xmm4, xmm2
1407628ad maxps xmm4, xmm0
1407628b0 cmpunordps xmm0, xmm0
1407628b4 andps xmm2, xmm0
1407628b7 andnps xmm0, xmm4
1407628ba orps xmm0, xmm2
1407628bd movaps xmm2, xmm3
1407628c0 maxps xmm2, xmm1
1407628c3 cmpunordps xmm1, xmm1
1407628c7 andps xmm3, xmm1
1407628ca andnps xmm1, xmm2
1407628cd orps xmm1, xmm3
1407628d0 movups xmmword ptr [rcx + r8*4], xmm0
1407628d5 movups xmmword ptr [rcx + r8*4 + 0x10], xmm1
1407628db add r8, 8
1407628df cmp rdx, r8
1407628e2 jne 0x140762894
1407628e4 nop word ptr cs:[rax + rax]
1407628f0 cmp r15, rdx
1407628f3 je 0x14076368c
1407628f9 movss xmm0, dword ptr [rcx + rdx*4]
1407628fe movss xmm1, dword ptr [r10 + rdx*4]
140762904 movaps xmm2, xmm0
140762907 cmpunordss xmm2, xmm0
14076290c movaps xmm3, xmm2
14076290f andps xmm3, xmm1
140762912 maxss xmm1, xmm0
140762916 andnps xmm2, xmm1
140762919 orps xmm2, xmm3
14076291c movss dword ptr [rcx + rdx*4], xmm2
140762921 lea r8, [rdx + 1]
140762925 mov rdx, r8
140762928 cmp rax, r8
14076292b jne 0x1407628f0
14076292d mov rdx, qword ptr [rbp + 0x2c0]
140762934 test rdx, rdx
140762937 je 0x14076294b
140762939 shl rdx, 2
14076293d mov r8d, 4
140762943 mov rcx, r10
140762946 call 0x140613c20
14076294b cmp dword ptr [r12 - 0x48], 2
140762951 je 0x1407629b1
140762953 mov rcx, qword ptr [r12 - 0x1a0]
14076295b test rcx, rcx
14076295e je 0x14076370a
140762964 mov rax, qword ptr [r12 - 0x1a8]
14076296c lea rcx, [rax + rcx*4]
140762970 add rcx, -4
140762974 je 0x1407636f5
14076297a movd xmm0, dword ptr [rax]
14076297e movd dword ptr [rcx], xmm0
140762982 mov rcx, qword ptr [r12 - 0x1b8]
14076298a test rcx, rcx
14076298d je 0x1407636e0
140762993 mov rax, qword ptr [r12 - 0x1c0]
14076299b lea rcx, [rax + rcx*4]
14076299f add rcx, -4
1407629a3 je 0x1407636cb
1407629a9 movd xmm0, dword ptr [rax]
1407629ad movd dword ptr [rcx], xmm0
1407629b1 add r12, -0x118
1407629b8 mov rdx, qword ptr [rbp + 0x178]
1407629bf sub rdx, qword ptr [rbp + 0x188]
1407629c6 mov byte ptr [rbp + 0x2f7], 1
1407629cd mov rsi, r12
1407629d0 mov rcx, r12
1407629d3 call 0x140a885f0
1407629d8 mov byte ptr [rbp + 0x2f7], 1
1407629df mov rcx, rsi
1407629e2 movd xmm1, dword ptr [rbp + 0x2a4]
1407629ea call 0x140a884a0
1407629ef mov byte ptr [rbp + 0x2f7], 1
1407629f6 mov rcx, rsi
1407629f9 movd xmm1, dword ptr [rbp + 0x2a8]
140762a01 call 0x140a884f0
140762a06 mov byte ptr [rbp + 0x2f7], 1
140762a0d mov rcx, rsi
140762a10 mov rdx, qword ptr [rbp + 0x188]
140762a17 call 0x140a88600
140762a1c mov byte ptr [rbp + 0x2f7], 1
140762a23 mov rcx, rsi
140762a26 movd xmm1, dword ptr [rbp + 0x2ac]
140762a2e call 0x140a88540
140762a33 mov byte ptr [rbp + 0x2f7], 1
140762a3a mov rcx, rsi
140762a3d mov rdx, qword ptr [rbp + 0x140]
140762a44 call 0x140a88610
140762a49 mov eax, dword ptr [rbp + 0x104]
140762a4f mov rcx, qword ptr [rbp + 0x88]
140762a56 mov dword ptr [rcx], eax
140762a58 mov rax, qword ptr [rbp + 0x170]
140762a5f mov qword ptr [rbp + 0x210], rax
140762a66 movdqu xmm0, xmmword ptr [rbp + 0x160]
140762a6e movdqa xmmword ptr [rbp + 0x200], xmm0
140762a76 mov r8, qword ptr [rbp + 0x2b0]
140762a7d mov byte ptr [rbp + 0x2f7], 0
140762a84 lea rcx, [rbp + 0x1b0]
140762a8b mov rdx, qword ptr [rbp + 0x190]
140762a92 lea r9, [rbp + 0x200]
140762a99 call 0x1411fbd70
140762a9e xor eax, eax
140762aa0 cmp rax, qword ptr [rbp + 0x1b0]
140762aa7 jo 0x140762abc
140762aa9 mov byte ptr [rbp + 0x2f7], 0
140762ab0 lea rcx, [rbp + 0x1b0]
140762ab7 call 0x1405d3750
140762abc mov rdx, qword ptr [rbp + 0x148]
140762ac3 test rdx, rdx
140762ac6 mov r15, qword ptr [rbp + 0x90]
140762acd je 0x140762ae5
140762acf mov rcx, qword ptr [rbp + 0x150]
140762ad6 shl rdx, 3
140762ada mov r8d, 4
140762ae0 call 0x140613c20
140762ae5 mov rsi, qword ptr [rbp + 8]
140762ae9 add rsi, 0x18
140762aed mov rcx, rsi
140762af0 call 0x140c81e10
140762af5 mov rbx, qword ptr [rbp + 0x198]
140762afc mov r14, qword ptr [rbp + 0x130]
140762b03 mov al, 1
140762b05 xor ecx, ecx
140762b07 lock cmpxchg byte ptr [rsi], cl
140762b0b jne 0x140762b38
140762b0d mov rdi, qword ptr [rbp + 0x288]
140762b14 cmp rbx, r14
140762b17 jne 0x14075fdea
140762b1d jmp 0x140762bbf
140762b22 mov rsi, rcx
140762b25 mov r8d, 0x3b9aca00
140762b2b call 0x1428cd210
140762b30 mov rcx, rsi
140762b33 jmp 0x14075ff09
140762b38 mov rcx, rsi
140762b3b xor edx, edx
140762b3d call 0x1428cdb90
140762b42 jmp 0x140762b0d
140762b44 lea rcx, [rip + 0x231aa3d]
140762b4b call 0x1428d9310
140762b50 jmp 0x14076373a
140762b55 mov byte ptr [rbp + 0x2f7], 1
140762b5c lea rcx, [rip + 0x231abd5]
140762b63 call 0x1428d9310
140762b68 jmp 0x14076373a
140762b6d mov byte ptr [rbp + 0x2f7], 1
140762b74 lea rcx, [rip + 0x231aa25]
140762b7b call 0x1428d9310
140762b80 jmp 0x14076373a
140762b85 lea rcx, [rip + 0x24b7a5c]
140762b8c lea r8, [rip + 0x231acc5]
140762b93 mov edx, 0x1d
140762b98 call 0x1428d9430
140762b9d jmp 0x14076373a
140762ba2 lea rcx, [rip + 0x24b7aef]
140762ba9 lea r8, [rip + 0x231aca8]
140762bb0 mov edx, 0x22
140762bb5 call 0x1428d9430
140762bba jmp 0x14076373a
140762bbf cmp byte ptr [rbp + 0x2e6], 1
140762bc6 jne 0x140762c56
140762bcc mov rax, qword ptr [rip + 0x2ec97d5]
140762bd3 mov rax, qword ptr [rax]
140762bd6 test rax, rax
140762bd9 je 0x140762c56
140762bdb lea rax, [rip + 0x231a936]
140762be2 mov qword ptr [rbp + 0x200], rax
140762be9 mov qword ptr [rbp + 0x208], 1
140762bf4 mov qword ptr [rbp + 0x210], 8
140762bff pxor xmm0, xmm0
140762c03 movdqu xmmword ptr [rbp + 0x218], xmm0
140762c0b lea rcx, [rip + 0x231a916]
140762c12 call 0x14067dd80
140762c17 lea rcx, [rip + 0x231a922]
140762c1e mov qword ptr [rbp - 0x18], rcx
140762c22 mov qword ptr [rbp - 0x10], 0x29
140762c2a mov qword ptr [rbp - 8], rcx
140762c2e mov qword ptr [rbp], 0x29
140762c36 mov qword ptr [rbp + 8], rax
140762c3a lea rcx, [rbp + 0x200]
140762c41 lea r8, [rbp - 0x18]
140762c45 mov edx, 1
140762c4a call 0x1404b1430
140762c4f mov rdi, qword ptr [rbp + 0x288]
140762c56 lea rcx, [rbp + 0x200]
140762c5d mov rdx, rdi
140762c60 call 0x140a1b100
140762c65 mov r12, qword ptr [rbp + 0x200]
140762c6c mov rdi, qword ptr [rbp + 0x208]
140762c73 movzx r15d, word ptr [rbp + 0x218]
140762c7b mov rbx, qword ptr [rbp + 0x220]
140762c82 mov rax, qword ptr [rbp + 0x448]
140762c89 mov rcx, qword ptr [rax]
140762c8c mov qword ptr [rbp + 0x2e8], rcx
140762c93 mov rsi, qword ptr [rax + 8]
140762c97 mov rax, qword ptr [rbp + 0x438]
140762c9e mov rcx, qword ptr [rax]
140762ca1 mov rdx, qword ptr [rax + 0x10]
140762ca5 mov qword ptr [rbp + 0x2c0], rdx
140762cac mov qword ptr [rbp + 0x2d0], rcx
140762cb3 add rcx, 0x20
140762cb7 mov qword ptr [rbp + 0x1a0], rcx
140762cbe mov rcx, qword ptr [rbp + 0x190]
140762cc5 add rcx, 0x20
140762cc9 mov qword ptr [rbp + 0x120], rcx
140762cd0 mov eax, dword ptr [rax + 0x1c]
140762cd3 mov dword ptr [rbp + 0x290], eax
140762cd9 pcmpeqd xmm6, xmm6
140762cdd nop dword ptr [rax]
140762ce0 test rbx, rbx
140762ce3 je 0x1407630aa
140762ce9 test r15w, r15w
140762ced je 0x140762d10
140762cef test r12, r12
140762cf2 je 0x1407630aa
140762cf8 lea eax, [r15 - 1]
140762cfc and eax, r15d
140762cff jmp 0x140762d3d
140762d01 nop word ptr cs:[rax + rax]
140762d10 movdqa xmm0, xmmword ptr [rdi]
140762d14 pmovmskb r15d, xmm0
140762d19 add r12, -0x2a80
140762d20 add rdi, 0x10
140762d24 cmp r15d, 0xffff
140762d2b je 0x140762d10
140762d2d mov ecx, 0xfffffffe
140762d32 sub ecx, r15d
140762d35 not r15d
140762d38 mov eax, r15d
140762d3b and eax, ecx
140762d3d mov ecx, r15d
140762d40 tzcnt ecx, ecx
140762d44 mov r15d, eax
140762d47 neg rcx
140762d4a imul rax, rcx, 0x2a8
140762d51 mov r13, qword ptr [r12 + rax - 0x2a8]
140762d59 mov qword ptr [rbp + 0x98], r13
140762d60 mov rcx, qword ptr [rbp + 0x288]
140762d67 mov rdx, r13
140762d6a call 0x140a922b0
140762d6f mov r14, rax
140762d72 dec rbx
140762d75 shr rax, 0x20
140762d79 je 0x140762ce0
140762d7f mov ecx, r14d
140762d82 cmp rcx, qword ptr [rsi + 0x10]
140762d86 jae 0x140762ce0
140762d8c mov rdx, qword ptr [rsi + 8]
140762d90 lea rcx, [rcx + rcx*4]
140762d94 cmp dword ptr [rdx + rcx*4], eax
140762d97 jne 0x140762ce0
140762d9d lea rax, [rdx + rcx*4]
140762da1 mov ecx, dword ptr [rax + 4]
140762da4 mov edx, 0xffffffff
140762da9 cmp rcx, rdx
140762dac je 0x140762ce0
140762db2 mov rdx, qword ptr [rbp + 0x2e8]
140762db9 cmp qword ptr [rdx + 0x38], rcx
140762dbd jbe 0x140762ce0
140762dc3 mov rdx, qword ptr [rbp + 0x2e8]
140762dca mov rdx, qword ptr [rdx + 0x28]
140762dce mov r8d, ecx
140762dd1 shr r8d, 6
140762dd5 mov rdx, qword ptr [rdx + r8*8]
140762dd9 bt rdx, rcx
140762ddd jae 0x140762ce0
140762de3 mov ecx, dword ptr [rax + 0xc]
140762de6 mov eax, dword ptr [rax + 0x10]
140762de9 mov rdx, qword ptr [rsi + 0x1a8]
140762df0 lea rcx, [rcx + rcx*8]
140762df4 mov r8, qword ptr [rbp + 0x2e8]
140762dfb mov r8, qword ptr [r8 + 0x110]
140762e02 mov r9, qword ptr [rdx + rcx*8 + 0x18]
140762e07 mov rcx, qword ptr [rdx + rcx*8 + 0x38]
140762e0c mov rcx, qword ptr [rcx + r8*8]
140762e10 not rcx
140762e13 lea rdx, [rcx + rcx*2]
140762e17 shl rdx, 4
140762e1b imul rcx, rax, 0x58
140762e1f add rcx, qword ptr [r9 + rdx + 0x10]
140762e24 call 0x1408e3380
140762e29 test al, al
140762e2b jne 0x140762ce0
140762e31 mov rax, qword ptr [rbp + 0x2d0]
140762e38 cmp qword ptr [rax + 0x18], 0
140762e3d je 0x140762f16
140762e43 mov rcx, qword ptr [rbp + 0x1a0]
140762e4a lea rdx, [rbp + 0x98]
140762e51 call 0x1405b0d70
140762e56 mov rdx, qword ptr [rbp + 0x2d0]
140762e5d mov rcx, qword ptr [rdx]
140762e60 mov rdx, qword ptr [rdx + 8]
140762e64 mov r8, rdx
140762e67 and r8, rax
140762e6a shr rax, 0x39
140762e6e movd xmm0, eax
140762e72 punpcklbw xmm0, xmm0
140762e76 pshuflw xmm0, xmm0, 0
140762e7b pshufd xmm0, xmm0, 0
140762e80 lea rax, [rcx - 0x10]
140762e84 xor r10d, r10d
140762e87 movdqu xmm1, xmmword ptr [rcx + r8]
140762e8d movdqa xmm2, xmm1
140762e91 pcmpeqb xmm2, xmm0
140762e95 pmovmskb r9d, xmm2
140762e9a test r9d, r9d
140762e9d je 0x140762ef1
140762e9f mov qword ptr [rbp + 0x2b8], r10
140762ea6 mov qword ptr [rbp + 0x2d8], rcx
140762ead tzcnt r11d, r9d
140762eb2 add r11, r8
140762eb5 and r11, rdx
140762eb8 mov r10, r9
140762ebb mov r9, r11
140762ebe shl r9, 4
140762ec2 mov rcx, rdx
140762ec5 mov rdx, rax
140762ec8 sub rax, r9
140762ecb cmp r13, qword ptr [rax]
140762ece je 0x140762f1b
140762ed0 lea eax, [r10 - 1]
140762ed4 and ax, r10w
140762ed8 mov r9d, eax
140762edb mov rax, rdx
140762ede mov rdx, rcx
140762ee1 mov rcx, qword ptr [rbp + 0x2d8]
140762ee8 mov r10, qword ptr [rbp + 0x2b8]
140762eef jne 0x140762ead
140762ef1 mov r9, rax
140762ef4 pcmpeqb xmm1, xmm6
140762ef8 pmovmskb eax, xmm1
140762efc test eax, eax
140762efe jne 0x140762f16
140762f00 add r8, r10
140762f03 add r8, 0x10
140762f07 add r10, 0x10
140762f0b and r8, rdx
140762f0e mov rax, r9
140762f11 jmp 0x140762e87
140762f16 xor r9d, r9d
140762f19 jmp 0x140762f2f
140762f1b neg r11
140762f1e shl r11, 4
140762f22 mov rax, qword ptr [rbp + 0x2d8]
140762f29 movzx r9d, byte ptr [rax + r11 - 8]
140762f2f mov rax, qword ptr [rbp + 0x190]
140762f36 cmp qword ptr [rax + 0x18], 0
140762f3b je 0x14076301f
140762f41 mov dword ptr [rbp + 0x2d8], r9d
140762f48 mov rcx, qword ptr [rbp + 0x120]
140762f4f lea rdx, [rbp + 0x98]
140762f56 call 0x1405b0d70
140762f5b mov rdx, qword ptr [rbp + 0x190]
140762f62 mov rcx, qword ptr [rdx]
140762f65 mov rdx, qword ptr [rdx + 8]
140762f69 mov r8, rdx
140762f6c and r8, rax
140762f6f shr rax, 0x39
140762f73 movd xmm0, eax
140762f77 punpcklbw xmm0, xmm0
140762f7b pshuflw xmm0, xmm0, 0
140762f80 pshufd xmm0, xmm0, 0
140762f85 lea rax, [rcx - 0x20]
140762f89 xor r11d, r11d
140762f8c movdqu xmm1, xmmword ptr [rcx + r8]
140762f92 movdqa xmm2, xmm1
140762f96 pcmpeqb xmm2, xmm0
140762f9a pmovmskb r10d, xmm2
140762f9f test r10d, r10d
140762fa2 je 0x140762ff3
140762fa4 mov qword ptr [rbp + 0x180], r11
140762fab mov qword ptr [rbp + 0x2b8], rcx
140762fb2 tzcnt r11d, r10d
140762fb7 add r11, r8
140762fba and r11, rdx
140762fbd mov r9, r11
140762fc0 shl r9, 5
140762fc4 mov rcx, rdx
140762fc7 mov rdx, rax
140762fca sub rax, r9
140762fcd cmp r13, qword ptr [rax]
140762fd0 je 0x140763029
140762fd2 lea eax, [r10 - 1]
140762fd6 and ax, r10w
140762fda mov r10d, eax
140762fdd mov rax, rdx
140762fe0 mov rdx, rcx
140762fe3 mov rcx, qword ptr [rbp + 0x2b8]
140762fea mov r11, qword ptr [rbp + 0x180]
140762ff1 jne 0x140762fb2
140762ff3 mov r10, rax
140762ff6 pcmpeqb xmm1, xmm6
140762ffa pmovmskb eax, xmm1
140762ffe test eax, eax
140763000 mov r9d, dword ptr [rbp + 0x2d8]
140763007 jne 0x14076301f
140763009 add r8, r11
14076300c add r8, 0x10
140763010 add r11, 0x10
140763014 and r8, rdx
140763017 mov rax, r10
14076301a jmp 0x140762f8c
14076301f xor r8d, r8d
140763022 test r9b, r9b
140763025 jne 0x14076304a
140763027 jmp 0x140763087
140763029 neg r11
14076302c shl r11, 5
140763030 mov rax, qword ptr [rbp + 0x2b8]
140763037 cmp qword ptr [rax + r11 - 8], 2
14076303d setae r8b
140763041 cmp byte ptr [rbp + 0x2d8], r8b
140763048 je 0x140763087
14076304a mov dword ptr [rbp + 0x2d8], r8d
140763051 mov qword ptr [rbp + 0x208], r13
140763058 mov byte ptr [rbp + 0x210], 3
14076305f mov qword ptr [rbp + 0x200], r14
140763066 mov rcx, qword ptr [rbp + 0x440]
14076306d lea rdx, [rbp + 0x200]
140763074 lea r8, [rip + 0x231a4f5]
14076307b call 0x1412154d0
140763080 mov r8d, dword ptr [rbp + 0x2d8]
140763087 mov rax, qword ptr [rbp + 0x2c0]
14076308e mov ecx, dword ptr [rbp + 0x290]
140763094 mov dword ptr [rax], ecx
140763096 mov rcx, qword ptr [rbp + 0x2d0]
14076309d mov rdx, r13
1407630a0 call 0x140eec230
1407630a5 jmp 0x140762ce0
1407630aa mov rdx, qword ptr [rbp + 0x1e8]
1407630b1 test rdx, rdx
1407630b4 je 0x1407630cc
1407630b6 mov rcx, qword ptr [rbp + 0x1f0]
1407630bd shl rdx, 3
1407630c1 mov r8d, 8
1407630c7 call 0x140613c20
1407630cc cmp byte ptr [rbp + 0x2e7], 0
1407630d3 mov rsi, qword ptr [rbp + 0xa8]
1407630da je 0x140763123
1407630dc call 0x140c18130
1407630e1 mov rcx, rax
1407630e4 mov rax, qword ptr [rax]
1407630e7 cmp rax, 1
1407630eb jne 0x1407631db
1407630f1 add rcx, 8
1407630f5 cmp qword ptr [rcx], 0
1407630f9 jne 0x1407632dc
1407630ff mov qword ptr [rcx], 0xffffffffffffffff
140763106 mov qword ptr [rbp + 0x2e8], rcx
14076310d add rcx, 8
140763111 mov rdx, rsi
140763114 call 0x140c170f0
140763119 mov rax, qword ptr [rbp + 0x2e8]
140763120 inc qword ptr [rax]
140763123 movaps xmm6, xmmword ptr [rbp + 0x300]
14076312a movaps xmm7, xmmword ptr [rbp + 0x310]
140763131 movaps xmm8, xmmword ptr [rbp + 0x320]
140763139 movaps xmm9, xmmword ptr [rbp + 0x330]
140763141 movaps xmm10, xmmword ptr [rbp + 0x340]
140763149 movaps xmm11, xmmword ptr [rbp + 0x350]
140763151 movaps xmm12, xmmword ptr [rbp + 0x360]
140763159 movaps xmm13, xmmword ptr [rbp + 0x370]
140763161 movaps xmm14, xmmword ptr [rbp + 0x380]
140763169 movaps xmm15, xmmword ptr [rbp + 0x390]
140763171 add rsp, 0x428
140763178 pop rbx
140763179 pop rdi
14076317a pop rsi
14076317b pop r12
14076317d pop r13
14076317f pop r14
140763181 pop r15
140763183 pop rbp
140763184 ret 
140763185 test rax, rax
140763188 jne 0x1407631f3
14076318a xor edx, edx
14076318c call 0x1428f15c0
140763191 mov rcx, rax
140763194 test rax, rax
140763197 jne 0x14075f8da
14076319d jmp 0x1407631f3
14076319f mov byte ptr [rbp + 0x2f7], 1
1407631a6 lea r8, [rip + 0x231a43b]
1407631ad mov rcx, r15
1407631b0 mov rdx, rsi
1407631b3 call 0x1428d9830
1407631b8 jmp 0x14076373a
1407631bd mov byte ptr [rbp + 0x2f7], 1
1407631c4 lea r8, [rip + 0x231a405]
1407631cb mov rcx, r15
1407631ce mov rdx, rsi
1407631d1 call 0x1428d9830
1407631d6 jmp 0x14076373a
1407631db test rax, rax
1407631de jne 0x1407631f3
1407631e0 xor edx, edx
1407631e2 call 0x1428f15c0
1407631e7 mov rcx, rax
1407631ea test rax, rax
1407631ed jne 0x1407630f5
1407631f3 lea rcx, [rip + 0x24b62ae]
1407631fa call 0x1428d80d0
1407631ff mov byte ptr [rbp + 0x2f7], 1
140763206 lea r8, [rip + 0x231a513]
14076320d mov rdx, rcx
140763210 call 0x1428d9518
140763215 jmp 0x14076373a
14076321a mov qword ptr [rbp + 0x200], 0
140763225 lea rax, [rip + 0x24b7634]
14076322c mov qword ptr [rsp + 0x20], rax
140763231 lea rdx, [rbp - 0x18]
140763235 lea r9, [rbp + 0x200]
14076323c xor ecx, ecx
14076323e mov r8, r12
140763241 call 0x1428cab41
140763246 jmp 0x14076373a
14076324b cmp rdi, 0x15
14076324f jae 0x1407636b4
140763255 lea rax, [rbx + rdi*8]
140763259 lea rcx, [rbx + 8]
14076325d mov edx, 8
140763262 jmp 0x140763277
140763264 mov r9, rbx
140763267 mov qword ptr [r9], r8
14076326a add rcx, 8
14076326e add rdx, 8
140763272 cmp rcx, rax
140763275 je 0x1407632a3
140763277 mov r10, qword ptr [rcx - 8]
14076327b mov r8, qword ptr [rcx]
14076327e cmp r8, r10
140763281 jae 0x14076326a
140763283 mov r9, rdx
140763286 mov qword ptr [rbx + r9], r10
14076328a cmp r9, 8
14076328e je 0x140763264
140763290 mov r10, qword ptr [rbx + r9 - 0x10]
140763295 add r9, -8
140763299 cmp r8, r10
14076329c jb 0x140763286
14076329e add r9, rbx
1407632a1 jmp 0x140763267
1407632a3 mov rbx, qword ptr [rbp + 0x1f0]
1407632aa mov rdi, qword ptr [rbp + 0x1f8]
1407632b1 cmp rdi, 2
1407632b5 jb 0x14075fd09
1407632bb mov eax, 1
1407632c0 mov rcx, qword ptr [rbx + rax*8]
1407632c4 cmp rcx, qword ptr [rbx + rax*8 - 8]
1407632c9 je 0x140763352
1407632cf inc rax
1407632d2 cmp rdi, rax
1407632d5 jne 0x1407632c0
1407632d7 jmp 0x14075fd09
1407632dc lea rcx, [rip + 0x24b6235]
1407632e3 call 0x1428d9250
1407632e8 mov byte ptr [rbp + 0x2f7], 1
1407632ef lea r8, [rip + 0x231a532]
1407632f6 mov rcx, rdi
1407632f9 call 0x1428d9518
1407632fe jmp 0x14076373a
140763303 mov byte ptr [rbp + 0x2f7], 1
14076330a lea r8, [rip + 0x231a58f]
140763311 mov rcx, rax
140763314 call 0x1428d9518
140763319 jmp 0x14076373a
14076331e mov byte ptr [rbp + 0x2f7], 1
140763325 lea r8, [rip + 0x231a2d4]
14076332c xor ecx, ecx
14076332e xor edx, edx
140763330 call 0x1428d9518
140763335 jmp 0x14076373a
14076333a mov byte ptr [rbp + 0x2f7], 1
140763341 lea r8, [rip + 0x231a330]
140763348 call 0x1428d9518
14076334d jmp 0x14076373a
140763352 lea rcx, [rax + 1]
140763356 jmp 0x14076335b
140763358 inc rcx
14076335b cmp rcx, rdi
14076335e jae 0x140763374
140763360 mov rdx, qword ptr [rbx + rcx*8]
140763364 cmp rdx, qword ptr [rbx + rax*8 - 8]
140763369 je 0x140763358
14076336b mov qword ptr [rbx + rax*8], rdx
14076336f inc rax
140763372 jmp 0x140763358
140763374 mov qword ptr [rbp + 0x1f8], rax
14076337b mov rdi, rax
14076337e jmp 0x14075fd09
140763383 movdqu xmm0, xmmword ptr [rbp + 0x208]
14076338b movdqa xmmword ptr [rbp + 0xb0], xmm0
140763393 lea rax, [rip + 0x24b5fde]
14076339a mov qword ptr [rsp + 0x20], rax
14076339f lea rcx, [rip + 0x24b5822]
1407633a6 lea r9, [rip + 0x24b57fb]
1407633ad lea r8, [rbp + 0xb0]
1407633b4 mov edx, 0x2b
1407633b9 call 0x1428d9760
1407633be jmp 0x14076373a
1407633c3 mov byte ptr [rbp + 0x2f7], 1
1407633ca lea r8, [rip + 0x231a4b7]
1407633d1 mov rcx, rax
1407633d4 call 0x1428d9518
1407633d9 jmp 0x14076373a
1407633de movdqu xmm0, xmmword ptr [rbp + 0x208]
1407633e6 movdqa xmmword ptr [rbp + 0x1b0], xmm0
1407633ee mov byte ptr [rbp + 0x2f7], 1
1407633f5 lea rax, [rip + 0x231a444]
1407633fc mov qword ptr [rsp + 0x20], rax
140763401 lea rcx, [rip + 0x24b57c0]
140763408 lea r9, [rip + 0x24b5799]
14076340f lea r8, [rbp + 0x1b0]
140763416 mov edx, 0x2b
14076341b call 0x1428d9760
140763420 jmp 0x14076373a
140763425 lea r8, [rip + 0x231a444]
14076342c call 0x1428d9518
140763431 jmp 0x14076373a
140763436 lea r8, [rip + 0x231a433]
14076343d mov rcx, rax
140763440 call 0x1428d9518
140763445 jmp 0x14076373a
14076344a mov byte ptr [rbp + 0x2f7], 1
140763451 lea r8, [rip + 0x231a208]
140763458 xor ecx, ecx
14076345a xor edx, edx
14076345c call 0x1428d9518
140763461 jmp 0x14076373a
140763466 mov byte ptr [rbp + 0x2f7], 1
14076346d lea r8, [rip + 0x231a1bc]
140763474 xor ecx, ecx
140763476 xor edx, edx
140763478 call 0x1428d9518
14076347d jmp 0x14076373a
140763482 mov byte ptr [rbp + 0x2f7], 1
140763489 lea r8, [rip + 0x231a188]
140763490 xor ecx, ecx
140763492 xor edx, edx
140763494 call 0x1428d9518
140763499 jmp 0x14076373a
14076349e mov byte ptr [rbp + 0x2f7], 1
1407634a5 lea r8, [rip + 0x231a19c]
1407634ac mov ecx, 1
1407634b1 mov edx, 1
1407634b6 call 0x1428d9518
1407634bb jmp 0x14076373a
1407634c0 xor edi, edi
1407634c2 mov byte ptr [rbp + 0x2f7], 1
1407634c9 lea r8, [rip + 0x24b6aa0]
1407634d0 mov rcx, rdi
1407634d3 mov rdx, r12
1407634d6 call 0x1428d8fe3
1407634db jmp 0x14076373a
1407634e0 lea r8, [rip + 0x231a2c9]
1407634e7 mov rcx, rax
1407634ea call 0x1428d9518
1407634ef jmp 0x14076373a
1407634f4 lea r8, [rip + 0x231a29d]
1407634fb mov rcx, rax
1407634fe call 0x1428d9518
140763503 jmp 0x14076373a
140763508 lea r8, [rip + 0x231a271]
14076350f mov rcx, rax
140763512 mov rdx, r15
140763515 call 0x1428d9518
14076351a jmp 0x14076373a
14076351f mov byte ptr [rbp + 0x2f7], 1
140763526 lea r8, [rip + 0x231a193]
14076352d mov rcx, rax
140763530 call 0x1428d9518
140763535 jmp 0x14076373a
14076353a mov byte ptr [rbp + 0x2f7], 1
140763541 lea r8, [rip + 0x231a160]
140763548 call 0x1428d9518
14076354d jmp 0x14076373a
140763552 mov byte ptr [rbp + 0x2f7], 1
140763559 lea r8, [rip + 0x231a130]
140763560 mov rcx, rax
140763563 call 0x1428d9518
140763568 jmp 0x14076373a
14076356d mov byte ptr [rbp + 0x2f7], 1
140763574 lea r8, [rip + 0x231a175]
14076357b call 0x1428d9518
140763580 jmp 0x14076373a
140763585 mov byte ptr [rbp + 0x2f7], 1
14076358c lea r8, [rip + 0x231a145]
140763593 mov rcx, rax
140763596 call 0x1428d9518
14076359b jmp 0x14076373a
1407635a0 xor r14d, r14d
1407635a3 mov byte ptr [rbp + 0x2f7], 1
1407635aa lea r8, [rip + 0x24b69bf]
1407635b1 mov rcx, r14
1407635b4 mov rdx, rbx
1407635b7 call 0x1428d8fe3
1407635bc jmp 0x14076373a
1407635c1 xor r12d, r12d
1407635c4 lea r8, [rip + 0x24b69a5]
1407635cb mov rcx, r12
1407635ce mov rdx, r14
1407635d1 call 0x1428d8fe3
1407635d6 jmp 0x14076373a
1407635db xor r13d, r13d
1407635de lea r8, [rip + 0x24b698b]
1407635e5 mov rcx, r13
1407635e8 mov rdx, r14
1407635eb call 0x1428d8fe3
1407635f0 jmp 0x14076373a
1407635f5 mov byte ptr [rbp + 0x2f7], 1
1407635fc lea rcx, [rip + 0x2318ed5]
140763603 call 0x1428d9310
140763608 jmp 0x14076373a
14076360d mov byte ptr [rbp + 0x2f7], 1
140763614 lea rcx, [rip + 0x2318ea5]
14076361b call 0x1428d9310
140763620 jmp 0x14076373a
140763625 mov byte ptr [rbp + 0x2f7], 1
14076362c lea r8, [rip + 0x2319f85]
140763633 xor ecx, ecx
140763635 xor edx, edx
140763637 call 0x1428d9518
14076363c jmp 0x14076373a
140763641 lea rcx, [rip + 0x2319e90]
140763648 call 0x1428d9310
14076364d jmp 0x14076373a
140763652 lea r8, [rip + 0x231a0f7]
140763659 mov rcx, rax
14076365c call 0x1428d9518
140763661 jmp 0x14076373a
140763666 lea r8, [rip + 0x231a0fb]
14076366d xor ecx, ecx
14076366f xor edx, edx
140763671 call 0x1428d9518
140763676 jmp 0x14076373a
14076367b lea rcx, [rip + 0x24b7426]
140763682 call 0x1428d9310
140763687 jmp 0x14076373a
14076368c lea r8, [rip + 0x231a225]
140763693 mov rcx, r15
140763696 mov rdx, r15
140763699 call 0x1428d9518
14076369e jmp 0x14076373a
1407636a3 lea rcx, [rip + 0x24b7416]
1407636aa call 0x1428d9310
1407636af jmp 0x14076373a
1407636b4 lea r8, [rbp + 0x200]
1407636bb mov rcx, rbx
1407636be mov rdx, rdi
1407636c1 call 0x140958d60
1407636c6 jmp 0x1407632bb
1407636cb mov byte ptr [rbp + 0x2f7], 1
1407636d2 lea rcx, [rip + 0x231a137]
1407636d9 call 0x1428d9310
1407636de jmp 0x14076373a
1407636e0 mov byte ptr [rbp + 0x2f7], 1
1407636e7 lea rcx, [rip + 0x231a10a]
1407636ee call 0x1428d9310
1407636f3 jmp 0x14076373a
1407636f5 mov byte ptr [rbp + 0x2f7], 1
1407636fc lea rcx, [rip + 0x231a0dd]
140763703 call 0x1428d9310
140763708 jmp 0x14076373a
14076370a mov byte ptr [rbp + 0x2f7], 1
140763711 lea rcx, [rip + 0x231a0b0]
140763718 call 0x1428d9310
14076371d jmp 0x14076373a
14076371f xor ebx, ebx
140763721 mov byte ptr [rbp + 0x2f7], 1
140763728 lea r8, [rip + 0x24b6841]
14076372f mov rcx, rbx
140763732 mov rdx, rsi
140763735 call 0x1428d8fe3
14076373a ud2 
