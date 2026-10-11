140765830 push rbp
140765831 push r15
140765833 push r14
140765835 push r13
140765837 push r12
140765839 push rsi
14076583a push rdi
14076583b push rbx
14076583c sub rsp, 0x168
140765843 lea rbp, [rsp + 0x80]
14076584b movdqa xmmword ptr [rbp + 0xd0], xmm6
140765853 mov qword ptr [rbp + 0xc8], 0xfffffffffffffffe
14076585e mov rdi, r8
140765861 mov r12, rdx
140765864 mov r13, rcx
140765867 call 0x140c16a00
14076586c mov ebx, eax
14076586e test bl, bl
140765870 je 0x140765a93
140765876 mov r14d, 0x3b
14076587c mov ecx, 0x3b
140765881 call 0x140765400
140765886 lea r15, [rip + 0x23182e3]
14076588d cmp rax, 1
140765891 jne 0x140765938
140765897 mov r9, rdx
14076589a test rdx, rdx
14076589d je 0x1407658d5
14076589f cmp r9, 0x3b
1407658a3 jae 0x1407658d3
1407658a5 lea rax, [rip + 0x23182c4]
1407658ac cmp byte ptr [r9 + rax], 0xbf
1407658b1 jg 0x1407658d5
1407658b3 lea rax, [rip + 0x231836e]
1407658ba mov qword ptr [rsp + 0x20], rax
1407658bf lea rcx, [rip + 0x23182aa]
1407658c6 mov edx, 0x3b
1407658cb xor r8d, r8d
1407658ce call 0x1428d9940
1407658d3 jne 0x1407658b3
1407658d5 mov rcx, r9
1407658d8 call 0x140765400
1407658dd cmp rax, 1
1407658e1 jne 0x140765938
1407658e3 mov r8, rdx
1407658e6 add r8, 2
1407658ea je 0x140765925
1407658ec cmp r8, 0x3b
1407658f0 jae 0x140765923
1407658f2 lea rax, [rip + 0x2318277]
1407658f9 cmp byte ptr [r8 + rax], 0xbf
1407658fe jg 0x140765925
140765900 lea rax, [rip + 0x2318339]
140765907 mov qword ptr [rsp + 0x20], rax
14076590c lea rcx, [rip + 0x231825d]
140765913 mov edx, 0x3b
140765918 mov r9d, 0x3b
14076591e call 0x1428d9940
140765923 jne 0x140765900
140765925 mov r14d, 0x39
14076592b sub r14, rdx
14076592e lea r15, [rip + 0x231823b]
140765935 add r15, r8
140765938 mov qword ptr [rbp + 0xb8], r12
14076593f lea r8, [rip + 0x2318365]
140765946 lea r12, [rip + 0x2318323]
14076594d nop dword ptr [rax]
140765950 movsx eax, byte ptr [r8 - 1]
140765955 test eax, eax
140765957 js 0x140765970
140765959 dec r8
14076595c cmp eax, 0x5c
14076595f jne 0x1407659c4
140765961 jmp 0x1407659d5
140765963 nop word ptr cs:[rax + rax]
140765970 movzx ecx, byte ptr [r8 - 2]
140765975 cmp cl, 0xc0
140765978 jge 0x14076599d
14076597a movzx edx, byte ptr [r8 - 3]
14076597f cmp dl, 0xc0
140765982 jge 0x1407659a6
140765984 movzx r9d, byte ptr [r8 - 4]
140765989 add r8, -4
14076598d and r9d, 7
140765991 shl r9d, 6
140765995 and edx, 0x3f
140765998 or edx, r9d
14076599b jmp 0x1407659ad
14076599d add r8, -2
1407659a1 and ecx, 0x1f
1407659a4 jmp 0x1407659b5
1407659a6 add r8, -3
1407659aa and edx, 0xf
1407659ad shl edx, 6
1407659b0 and ecx, 0x3f
1407659b3 or ecx, edx
1407659b5 shl ecx, 6
1407659b8 and al, 0x3f
1407659ba movzx eax, al
1407659bd or eax, ecx
1407659bf cmp eax, 0x5c
1407659c2 je 0x1407659d5
1407659c4 cmp eax, 0x2f
1407659c7 je 0x1407659d5
1407659c9 cmp r8, r12
1407659cc jne 0x140765950
1407659ce mov esi, 0x3b
1407659d3 jmp 0x140765a21
1407659d5 lea r12, [rip + 0x2318294]
1407659dc sub r8, r12
1407659df inc r8
1407659e2 je 0x140765a16
1407659e4 cmp r8, 0x3b
1407659e8 jae 0x140765a14
1407659ea cmp byte ptr [r8 + r12], 0xbf
1407659ef jg 0x140765a16
1407659f1 lea rax, [rip + 0x2318210]
1407659f8 mov qword ptr [rsp + 0x20], rax
1407659fd lea rcx, [rip + 0x231826c]
140765a04 mov edx, 0x3b
140765a09 mov r9d, 0x3b
140765a0f call 0x1428d9940
140765a14 jne 0x1407659f1
140765a16 mov esi, 0x3b
140765a1b sub rsi, r8
140765a1e add r12, r8
140765a21 call 0x140c18130
140765a26 mov rcx, rax
140765a29 mov rax, qword ptr [rax]
140765a2c cmp rax, 1
140765a30 jne 0x140765fa3
140765a36 add rcx, 8
140765a3a cmp qword ptr [rcx], 0
140765a3e jne 0x14076600b
140765a44 mov qword ptr [rcx], 0xffffffffffffffff
140765a4b mov qword ptr [rbp + 0xa0], rcx
140765a52 add rcx, 8
140765a56 mov qword ptr [rsp + 0x20], rsi
140765a5b mov qword ptr [rsp + 0x30], 0
140765a64 mov qword ptr [rsp + 0x28], 1
140765a6d mov rdx, r15
140765a70 mov r8, r14
140765a73 mov r9, r12
140765a76 call 0x140c16d40
140765a7b mov qword ptr [rbp + 0xb0], rax
140765a82 mov rax, qword ptr [rbp + 0xa0]
140765a89 inc qword ptr [rax]
140765a8c mov r12, qword ptr [rbp + 0xb8]
140765a93 mov byte ptr [rbp + 0xc7], bl
140765a99 mov rcx, qword ptr [rdi]
140765a9c lea rdx, [rip + 0x24b4d2d]
140765aa3 call 0x141209c40
140765aa8 mov rdi, rax
140765aab test rax, rax
140765aae je 0x140766017
140765ab4 lea rax, [rip + 0x24b4ead]
140765abb mov qword ptr [rbp - 0x40], rax
140765abf mov qword ptr [rbp - 0x38], 0x3d
140765ac7 mov r14, rdi
140765aca add r14, 0x10
140765ace cmp qword ptr [rdi + 0x18], 0x3d
140765ad3 jne 0x140765fe4
140765ad9 mov rdx, qword ptr [r14]
140765adc lea rcx, [rip + 0x24b4e85]
140765ae3 mov r8d, 0x3d
140765ae9 call 0x142923bf0
140765aee test eax, eax
140765af0 jne 0x140765fe4
140765af6 mov rcx, qword ptr [rdi]
140765af9 mov rax, qword ptr [rdi + 8]
140765afd call qword ptr [rax + 0x18]
140765b00 mov qword ptr [rbp + 0xa8], rax
140765b07 cmp qword ptr [rbp + 0xa8], 0
140765b0f je 0x140766017
140765b15 mov byte ptr [rbp + 0xc6], 0
140765b1c mov r10, qword ptr [r13]
140765b20 mov rdx, qword ptr [r13 + 8]
140765b24 mov r8, qword ptr [r10]
140765b27 xor r11d, r11d
140765b2a mov rcx, r8
140765b2d sub rcx, qword ptr [rdx + 0x18]
140765b31 cmovb rcx, r11
140765b35 sub r8, qword ptr [rdx + 0x38]
140765b39 cmovb r8, r11
140765b3d mov rax, rcx
140765b40 shl rax, 4
140765b44 add rax, qword ptr [rdx + 8]
140765b48 mov rsi, qword ptr [rdx + 0x10]
140765b4c sub rsi, rcx
140765b4f cmovb rsi, r11
140765b53 mov r9d, 8
140765b59 cmovb rax, r9
140765b5d mov rcx, r8
140765b60 shl rcx, 4
140765b64 add rcx, qword ptr [rdx + 0x28]
140765b68 mov rbx, qword ptr [rdx + 0x30]
140765b6c sub rbx, r8
140765b6f cmovb rbx, r11
140765b73 cmovb rcx, r9
140765b77 mov rdx, qword ptr [rdx + 0x40]
140765b7b sub rdx, rbx
140765b7e sub rdx, rsi
140765b81 mov qword ptr [rbp + 0xb8], r10
140765b88 mov qword ptr [r10], rdx
140765b8b shl rsi, 4
140765b8f add rsi, rax
140765b92 shl rbx, 4
140765b96 add rbx, rcx
140765b99 mov rdx, qword ptr [r12]
140765b9d mov qword ptr [rbp + 0xa0], rdx
140765ba4 add rdx, 0x20
140765ba8 mov qword ptr [rbp + 0x80], rdx
140765baf pcmpeqd xmm6, xmm6
140765bb3 test rax, rax
140765bb6 je 0x140765bd0
140765bb8 cmp rax, rsi
140765bbb je 0x140765bd0
140765bbd lea r13, [rax + 0x10]
140765bc1 mov r12, rcx
140765bc4 jmp 0x140765bec
140765bc6 nop word ptr cs:[rax + rax]
140765bd0 test rcx, rcx
140765bd3 je 0x140765e8f
140765bd9 cmp rcx, rbx
140765bdc je 0x140765e8f
140765be2 lea r12, [rcx + 0x10]
140765be6 xor r13d, r13d
140765be9 mov rax, rcx
140765bec mov rcx, qword ptr [rbp + 0xb8]
140765bf3 inc qword ptr [rcx]
140765bf6 mov r14, qword ptr [rax + 8]
140765bfa mov qword ptr [rbp + 0x98], r14
140765c01 mov rdi, qword ptr [rbp + 0xa0]
140765c08 cmp qword ptr [rdi + 0x18], 0
140765c0d je 0x140765e72
140765c13 mov rcx, qword ptr [rbp + 0x80]
140765c1a lea rdx, [rbp + 0x98]
140765c21 call 0x1405b0d70
140765c26 mov r15, qword ptr [rdi]
140765c29 mov rcx, qword ptr [rdi + 8]
140765c2d mov rdx, rcx
140765c30 and rdx, rax
140765c33 shr rax, 0x39
140765c37 movd xmm0, eax
140765c3b punpcklbw xmm0, xmm0
140765c3f pshuflw xmm0, xmm0, 0
140765c44 pshufd xmm0, xmm0, 0
140765c49 lea rax, [r15 - 0x2a8]
140765c50 xor r8d, r8d
140765c53 movdqu xmm1, xmmword ptr [r15 + rdx]
140765c59 movdqa xmm2, xmm1
140765c5d pcmpeqb xmm2, xmm0
140765c61 pmovmskb r9d, xmm2
140765c66 test r9d, r9d
140765c69 je 0x140765ca0
140765c6b tzcnt r10d, r9d
140765c70 add r10, rdx
140765c73 and r10, rcx
140765c76 neg r10
140765c79 imul rdi, r10, 0x2a8
140765c80 cmp r14, qword ptr [rax + rdi]
140765c84 je 0x140765cd0
140765c86 lea r10d, [r9 - 1]
140765c8a and r10w, r9w
140765c8e mov r9d, r10d
140765c91 jne 0x140765c6b
140765c93 nop word ptr cs:[rax + rax]
140765ca0 pcmpeqb xmm1, xmm6
140765ca4 pmovmskb r9d, xmm1
140765ca9 test r9d, r9d
140765cac jne 0x140765e72
140765cb2 add rdx, r8
140765cb5 add rdx, 0x10
140765cb9 add r8, 0x10
140765cbd and rdx, rcx
140765cc0 jmp 0x140765c53
140765cc2 nop word ptr cs:[rax + rax]
140765cd0 mov qword ptr [rbp + 0x90], r13
140765cd7 mov qword ptr [rbp + 0x88], r12
140765cde lea r12, [r15 + rdi]
140765ce2 cmp byte ptr [r15 + rdi - 0xa], 1
140765ce8 jne 0x140765d60
140765cea mov r13, qword ptr [r12 - 0x170]
140765cf2 test r13, r13
140765cf5 je 0x140766025
140765cfb lea rcx, [r13 + 0x18]
140765cff xor eax, eax
140765d01 mov dl, 1
140765d03 lock cmpxchg byte ptr [r13 + 0x18], dl
140765d09 jne 0x140765e50
140765d0f call 0x140c81c70
140765d14 mov rax, qword ptr [r13 + 0x10]
140765d18 test rax, rax
140765d1b sete cl
140765d1e shl rax, 2
140765d22 add rax, qword ptr [r13 + 8]
140765d26 add rax, -4
140765d2a sete dl
140765d2d or dl, cl
140765d2f je 0x140765d6b
140765d31 mov rax, qword ptr [rbp + 0xa8]
140765d38 mov rax, qword ptr [rax + 0x28]
140765d3c test rax, rax
140765d3f je 0x140766033
140765d45 lea ecx, [rax - 1]
140765d48 mov r9, qword ptr [rbp + 0xa8]
140765d4f jmp 0x140765d78
140765d51 nop word ptr cs:[rax + rax]
140765d60 xor r14d, r14d
140765d63 xor r13d, r13d
140765d66 jmp 0x140765e20
140765d6b mov ecx, dword ptr [rax]
140765d6d mov r9, qword ptr [rbp + 0xa8]
140765d74 mov rax, qword ptr [r9 + 0x28]
140765d78 lea r8, [r15 + rdi]
140765d7c add r8, -0x2a0
140765d83 mov rdx, qword ptr [r9 + 0x20]
140765d87 movdqu xmm0, xmmword ptr [r9 + 0x30]
140765d8d movdqu xmm1, xmmword ptr [r9 + 0x40]
140765d93 movdqu xmm2, xmmword ptr [r9 + 0x50]
140765d99 mov dword ptr [rbp + 0x50], ecx
140765d9c mov qword ptr [rbp], rdx
140765da0 mov qword ptr [rbp + 8], rax
140765da4 movdqu xmmword ptr [rbp + 0x10], xmm0
140765da9 mov qword ptr [rbp + 0x20], r13
140765dad movdqu xmmword ptr [rbp + 0x28], xmm1
140765db2 mov qword ptr [rbp + 0x38], r9
140765db6 movdqu xmmword ptr [rbp + 0x40], xmm2
140765dbb mov rax, rbp
140765dbe mov qword ptr [rbp - 0x10], rax
140765dc2 lea rax, [rbp + 0xc6]
140765dc9 mov qword ptr [rbp - 8], rax
140765dcd mov rdx, qword ptr [rbp + 0x98]
140765dd4 lea rcx, [rbp - 0x40]
140765dd8 lea r9, [rbp - 0x10]
140765ddc call 0x1412388f0
140765de1 mov r14, qword ptr [rbp - 0x40]
140765de5 mov r13, qword ptr [rbp - 0x30]
140765de9 mov rdx, qword ptr [rbp - 0x28]
140765ded test rdx, rdx
140765df0 je 0x140765e05
140765df2 mov rcx, qword ptr [rbp - 0x20]
140765df6 shl rdx, 4
140765dfa mov r8d, 8
140765e00 call 0x140613c20
140765e05 mov r15, qword ptr [rbp + 0x20]
140765e09 add r15, 0x18
140765e0d mov rcx, r15
140765e10 call 0x140c81e10
140765e15 mov al, 1
140765e17 xor ecx, ecx
140765e19 lock cmpxchg byte ptr [r15], cl
140765e1e jne 0x140765e66
140765e20 add r12, -0x118
140765e27 mov rcx, r12
140765e2a mov rdx, r14
140765e2d call 0x140a88620
140765e32 mov rcx, r12
140765e35 mov rdx, r13
140765e38 call 0x140a88630
140765e3d mov rax, qword ptr [rbp + 0x90]
140765e44 mov rcx, qword ptr [rbp + 0x88]
140765e4b jmp 0x140765bb3
140765e50 mov r14, rcx
140765e53 mov r8d, 0x3b9aca00
140765e59 call 0x1428cd210
140765e5e mov rcx, r14
140765e61 jmp 0x140765d0f
140765e66 mov rcx, r15
140765e69 xor edx, edx
140765e6b call 0x1428cdb90
140765e70 jmp 0x140765e20
140765e72 lea rcx, [rip + 0x2317c2f]
140765e79 lea r8, [rip + 0x2317ed0]
140765e80 mov edx, 0x16
140765e85 call 0x1428d9330
140765e8a jmp 0x14076603f
140765e8f cmp byte ptr [rbp + 0xc6], 0
140765e96 je 0x140765f38
140765e9c mov rax, qword ptr [rip + 0x2ec6505]
140765ea3 mov rax, qword ptr [rax]
140765ea6 test rax, rax
140765ea9 je 0x140765f38
140765eaf lea rcx, [rip + 0x2317e52]
140765eb6 call 0x14067dd80
140765ebb movdqu xmm0, xmmword ptr [rax]
140765ebf mov eax, dword ptr [rax + 0x10]
140765ec2 mov qword ptr [rbp + 0x30], 1
140765eca lea rcx, [rip + 0x2317e4f]
140765ed1 mov qword ptr [rbp + 0x38], rcx
140765ed5 mov qword ptr [rbp + 0x40], 0x29
140765edd lea rdx, [rip + 0x2317e14]
140765ee4 mov qword ptr [rbp + 0x50], rdx
140765ee8 mov qword ptr [rbp + 0x58], 1
140765ef0 mov qword ptr [rbp + 0x60], 8
140765ef8 mov qword ptr [rbp], 0
140765f00 pxor xmm1, xmm1
140765f04 movdqu xmmword ptr [rbp + 0x68], xmm1
140765f09 mov qword ptr [rbp + 8], rcx
140765f0d mov qword ptr [rbp + 0x10], 0x29
140765f15 mov qword ptr [rbp + 0x18], 0
140765f1d movdqu xmmword ptr [rbp + 0x20], xmm0
140765f22 mov dword ptr [rbp + 0x48], 1
140765f29 mov dword ptr [rbp + 0x4c], eax
140765f2c lea rcx, [rbp - 0x40]
140765f30 mov rdx, rbp
140765f33 call 0x140d4a330
140765f38 cmp byte ptr [rbp + 0xc7], 0
140765f3f je 0x140765f88
140765f41 call 0x140c18130
140765f46 mov rcx, rax
140765f49 mov rax, qword ptr [rax]
140765f4c cmp rax, 1
140765f50 jne 0x140765fbd
140765f52 add rcx, 8
140765f56 mov rdx, qword ptr [rbp + 0xb0]
140765f5d cmp qword ptr [rcx], 0
140765f61 jne 0x14076600b
140765f67 mov qword ptr [rcx], 0xffffffffffffffff
140765f6e mov qword ptr [rbp + 0xb8], rcx
140765f75 add rcx, 8
140765f79 call 0x140c170f0
140765f7e mov rax, qword ptr [rbp + 0xb8]
140765f85 inc qword ptr [rax]
140765f88 movaps xmm6, xmmword ptr [rbp + 0xd0]
140765f8f add rsp, 0x168
140765f96 pop rbx
140765f97 pop rdi
140765f98 pop rsi
140765f99 pop r12
140765f9b pop r13
140765f9d pop r14
140765f9f pop r15
140765fa1 pop rbp
140765fa2 ret 
140765fa3 test rax, rax
140765fa6 jne 0x140765fd8
140765fa8 xor edx, edx
140765faa call 0x1428f15c0
140765faf mov rcx, rax
140765fb2 test rax, rax
140765fb5 jne 0x140765a3a
140765fbb jmp 0x140765fd8
140765fbd test rax, rax
140765fc0 jne 0x140765fd8
140765fc2 xor edx, edx
140765fc4 call 0x1428f15c0
140765fc9 mov rcx, rax
140765fcc mov rdx, qword ptr [rbp + 0xb0]
140765fd3 test rax, rax
140765fd6 jne 0x140765f5d
140765fd8 lea rcx, [rip + 0x24b34c9]
140765fdf call 0x1428d80d0
140765fe4 mov qword ptr [rbp], 0
140765fec lea rax, [rip + 0x24b486d]
140765ff3 mov qword ptr [rsp + 0x20], rax
140765ff8 lea rdx, [rbp - 0x40]
140765ffc mov r9, rbp
140765fff xor ecx, ecx
140766001 mov r8, r14
140766004 call 0x1428cab41
140766009 jmp 0x14076603f
14076600b lea rcx, [rip + 0x24b3506]
140766012 call 0x1428d9250
140766017 lea rcx, [rip + 0x2317c92]
14076601e call 0x1428d9310
140766023 jmp 0x14076603f
140766025 lea rcx, [rip + 0x24b4a7c]
14076602c call 0x1428d9310
140766031 jmp 0x14076603f
140766033 lea rcx, [rip + 0x24b4a86]
14076603a call 0x1428d9310
14076603f ud2 
