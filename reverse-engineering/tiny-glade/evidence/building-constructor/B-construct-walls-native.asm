140757800 push rbp
140757801 push r15
140757803 push r14
140757805 push r13
140757807 push r12
140757809 push rsi
14075780a push rdi
14075780b push rbx
14075780c sub rsp, 0x568
140757813 lea rbp, [rsp + 0x80]
14075781b movaps xmmword ptr [rbp + 0x4d0], xmm15
140757823 movaps xmmword ptr [rbp + 0x4c0], xmm14
14075782b movdqa xmmword ptr [rbp + 0x4b0], xmm13
140757834 movdqa xmmword ptr [rbp + 0x4a0], xmm12
14075783d movdqa xmmword ptr [rbp + 0x490], xmm11
140757846 movaps xmmword ptr [rbp + 0x480], xmm10
14075784e movaps xmmword ptr [rbp + 0x470], xmm9
140757856 movaps xmmword ptr [rbp + 0x460], xmm8
14075785e movaps xmmword ptr [rbp + 0x450], xmm7
140757865 movdqa xmmword ptr [rbp + 0x440], xmm6
14075786d mov qword ptr [rbp + 0x438], 0xfffffffffffffffe
140757878 mov r14, r9
14075787b mov qword ptr [rbp + 0x418], r8
140757882 mov qword ptr [rbp + 0x2c8], rdx
140757889 mov rax, qword ptr [rbp + 0x590]
140757890 mov rdx, qword ptr [rcx]
140757893 mov qword ptr [rbp + 0x3f8], rdx
14075789a mov rdx, qword ptr [rcx + 0x20]
14075789e mov qword ptr [rbp + 0x2a8], rdx
1407578a5 mov rdi, qword ptr [rcx + 0x40]
1407578a9 mov rbx, qword ptr [rcx + 0x48]
1407578ad mov rcx, qword ptr [rcx + 0x58]
1407578b1 mov qword ptr [rbp + 0x220], rcx
1407578b8 mov rcx, qword ptr [rax]
1407578bb mov qword ptr [rbp + 0xf0], rcx
1407578c2 movdqu xmm0, xmmword ptr [rax + 0x20]
1407578c7 movdqu xmm1, xmmword ptr [rax + 0x30]
1407578cc movdqu xmm2, xmmword ptr [rax + 0x40]
1407578d1 movdqu xmm3, xmmword ptr [rax + 0x50]
1407578d6 movdqa xmmword ptr [rbp + 0x60], xmm3
1407578db movdqa xmmword ptr [rbp + 0x50], xmm2
1407578e0 movdqa xmmword ptr [rbp + 0x40], xmm1
1407578e5 movdqa xmmword ptr [rbp + 0x30], xmm0
1407578ea call 0x140c16a00
1407578ef mov byte ptr [rbp + 0x431], al
1407578f5 test al, al
1407578f7 je 0x140757b0c
1407578fd mov r15d, 0x39
140757903 mov ecx, 0x39
140757908 call 0x140757310
14075790d lea r12, [rip + 0x23250e7]
140757914 cmp rax, 1
140757918 jne 0x1407579bf
14075791e mov r9, rdx
140757921 test rdx, rdx
140757924 je 0x14075795c
140757926 cmp r9, 0x39
14075792a jae 0x14075795a
14075792c lea rax, [rip + 0x23250c8]
140757933 cmp byte ptr [r9 + rax], 0xbf
140757938 jg 0x14075795c
14075793a lea rax, [rip + 0x232516f]
140757941 mov qword ptr [rsp + 0x20], rax
140757946 lea rcx, [rip + 0x23250ae]
14075794d mov edx, 0x39
140757952 xor r8d, r8d
140757955 call 0x1428d9940
14075795a jne 0x14075793a
14075795c mov rcx, r9
14075795f call 0x140757310
140757964 cmp rax, 1
140757968 jne 0x1407579bf
14075796a mov r8, rdx
14075796d add r8, 2
140757971 je 0x1407579ac
140757973 cmp r8, 0x39
140757977 jae 0x1407579aa
140757979 lea rax, [rip + 0x232507b]
140757980 cmp byte ptr [r8 + rax], 0xbf
140757985 jg 0x1407579ac
140757987 lea rax, [rip + 0x232513a]
14075798e mov qword ptr [rsp + 0x20], rax
140757993 lea rcx, [rip + 0x2325061]
14075799a mov edx, 0x39
14075799f mov r9d, 0x39
1407579a5 call 0x1428d9940
1407579aa jne 0x140757987
1407579ac mov r15d, 0x37
1407579b2 sub r15, rdx
1407579b5 lea r12, [rip + 0x232503f]
1407579bc add r12, r8
1407579bf lea r8, [rip + 0x2325230]
1407579c6 lea r13, [rip + 0x23251f3]
1407579cd nop dword ptr [rax]
1407579d0 movsx eax, byte ptr [r8 - 1]
1407579d5 test eax, eax
1407579d7 js 0x1407579f0
1407579d9 dec r8
1407579dc cmp eax, 0x5c
1407579df jne 0x140757a44
1407579e1 jmp 0x140757a55
1407579e3 nop word ptr cs:[rax + rax]
1407579f0 movzx ecx, byte ptr [r8 - 2]
1407579f5 cmp cl, 0xc0
1407579f8 jge 0x140757a1d
1407579fa movzx edx, byte ptr [r8 - 3]
1407579ff cmp dl, 0xc0
140757a02 jge 0x140757a26
140757a04 movzx r9d, byte ptr [r8 - 4]
140757a09 add r8, -4
140757a0d and r9d, 7
140757a11 shl r9d, 6
140757a15 and edx, 0x3f
140757a18 or edx, r9d
140757a1b jmp 0x140757a2d
140757a1d add r8, -2
140757a21 and ecx, 0x1f
140757a24 jmp 0x140757a35
140757a26 add r8, -3
140757a2a and edx, 0xf
140757a2d shl edx, 6
140757a30 and ecx, 0x3f
140757a33 or ecx, edx
140757a35 shl ecx, 6
140757a38 and al, 0x3f
140757a3a movzx eax, al
140757a3d or eax, ecx
140757a3f cmp eax, 0x5c
140757a42 je 0x140757a55
140757a44 cmp eax, 0x2f
140757a47 je 0x140757a55
140757a49 cmp r8, r13
140757a4c jne 0x1407579d0
140757a4e mov esi, 0x36
140757a53 jmp 0x140757aa1
140757a55 lea r13, [rip + 0x2325164]
140757a5c sub r8, r13
140757a5f inc r8
140757a62 je 0x140757a96
140757a64 cmp r8, 0x36
140757a68 jae 0x140757a94
140757a6a cmp byte ptr [r8 + r13], 0xbf
140757a6f jg 0x140757a96
140757a71 lea rax, [rip + 0x2325018]
140757a78 mov qword ptr [rsp + 0x20], rax
140757a7d lea rcx, [rip + 0x232513c]
140757a84 mov edx, 0x36
140757a89 mov r9d, 0x36
140757a8f call 0x1428d9940
140757a94 jne 0x140757a71
140757a96 mov esi, 0x36
140757a9b sub rsi, r8
140757a9e add r13, r8
140757aa1 call 0x140c18130
140757aa6 mov rcx, rax
140757aa9 mov rax, qword ptr [rax]
140757aac cmp rax, 1
140757ab0 jne 0x140759cba
140757ab6 add rcx, 8
140757aba cmp qword ptr [rcx], 0
140757abe jne 0x140759dbc
140757ac4 mov qword ptr [rcx], 0xffffffffffffffff
140757acb mov qword ptr [rbp + 0x420], rcx
140757ad2 add rcx, 8
140757ad6 mov qword ptr [rsp + 0x20], rsi
140757adb mov qword ptr [rsp + 0x30], 0
140757ae4 mov qword ptr [rsp + 0x28], 1
140757aed mov rdx, r12
140757af0 mov r8, r15
140757af3 mov r9, r13
140757af6 call 0x140c16d40
140757afb mov qword ptr [rbp + 0x2e8], rax
140757b02 mov rax, qword ptr [rbp + 0x420]
140757b09 inc qword ptr [rax]
140757b0c mov qword ptr [rbp + 0x3e8], r14
140757b13 mov qword ptr [rbp + 0xf8], rbx
140757b1a mov qword ptr [rbp + 0x100], rdi
140757b21 mov rcx, qword ptr [rbp + 0x558]
140757b28 mov rax, qword ptr [rcx]
140757b2b mov rdx, qword ptr [rcx + 8]
140757b2f mov r10, qword ptr [rax]
140757b32 xor r9d, r9d
140757b35 mov r11, r10
140757b38 sub r11, qword ptr [rdx + 0x18]
140757b3c cmovb r11, r9
140757b40 sub r10, qword ptr [rdx + 0x38]
140757b44 cmovb r10, r9
140757b48 mov rcx, r11
140757b4b shl rcx, 4
140757b4f add rcx, qword ptr [rdx + 8]
140757b53 mov r8, qword ptr [rdx + 0x10]
140757b57 sub r8, r11
140757b5a cmovb r8, r9
140757b5e mov edi, 8
140757b63 cmovb rcx, rdi
140757b67 mov r11, r10
140757b6a shl r11, 4
140757b6e add r11, qword ptr [rdx + 0x28]
140757b72 mov rsi, qword ptr [rdx + 0x30]
140757b76 sub rsi, r10
140757b79 cmovb rsi, r9
140757b7d cmovb r11, rdi
140757b81 lea r9, [rsi + r8]
140757b85 mov rdx, qword ptr [rdx + 0x40]
140757b89 sub rdx, r9
140757b8c mov qword ptr [rax], rdx
140757b8f shl r8, 4
140757b93 add r8, rcx
140757b96 shl rsi, 4
140757b9a add rsi, r11
140757b9d mov qword ptr [rbp + 0x2f0], rax
140757ba4 mov qword ptr [rbp + 0x2f8], rcx
140757bab mov qword ptr [rbp + 0x300], r8
140757bb2 mov qword ptr [rbp + 0x308], r11
140757bb9 mov qword ptr [rbp + 0x310], rsi
140757bc0 mov qword ptr [rbp + 0x318], r9
140757bc7 lea r8, [rip + 0x2324dca]
140757bce lea rcx, [rbp + 0x3b0]
140757bd5 lea rdx, [rbp + 0x2f0]
140757bdc call 0x1412036d0
140757be1 mov rax, qword ptr [rbp + 0x560]
140757be8 mov rbx, qword ptr [rax]
140757beb mov rax, qword ptr [rax + 8]
140757bef mov rdx, qword ptr [rbx]
140757bf2 xor ecx, ecx
140757bf4 mov r8, rdx
140757bf7 sub r8, qword ptr [rax + 0x18]
140757bfb cmovb r8, rcx
140757bff sub rdx, qword ptr [rax + 0x38]
140757c03 cmovb rdx, rcx
140757c07 mov r15, r8
140757c0a shl r15, 4
140757c0e add r15, qword ptr [rax + 8]
140757c12 mov r9, qword ptr [rax + 0x10]
140757c16 sub r9, r8
140757c19 cmovb r9, rcx
140757c1d cmovb r15, rdi
140757c21 mov r14, rdx
140757c24 shl r14, 4
140757c28 add r14, qword ptr [rax + 0x28]
140757c2c mov r12, qword ptr [rax + 0x30]
140757c30 sub r12, rdx
140757c33 cmovb r12, rcx
140757c37 cmovb r14, rdi
140757c3b mov rax, qword ptr [rax + 0x40]
140757c3f sub rax, r12
140757c42 sub rax, r9
140757c45 mov qword ptr [rbx], rax
140757c48 shl r9, 4
140757c4c add r9, r15
140757c4f mov qword ptr [rbp + 0x420], r9
140757c56 shl r12, 4
140757c5a add r12, r14
140757c5d jmp 0x140757c75
140757c5f nop 
140757c60 mov rax, qword ptr [rbp + 0x3b8]
140757c67 mov qword ptr [rax + rdi*8], r13
140757c6b inc rdi
140757c6e mov qword ptr [rbp + 0x3c0], rdi
140757c75 mov rcx, r14
140757c78 mov rdx, r15
140757c7b test rdx, rdx
140757c7e jne 0x140757ca0
140757c80 test rcx, rcx
140757c83 je 0x140757d34
140757c89 cmp rcx, r12
140757c8c je 0x140757d34
140757c92 lea r14, [rcx + 0x10]
140757c96 mov rdi, r14
140757c99 xor esi, esi
140757c9b mov rdx, rcx
140757c9e jmp 0x140757cb3
140757ca0 cmp rdx, qword ptr [rbp + 0x420]
140757ca7 je 0x140757ce4
140757ca9 lea r15, [rdx + 0x10]
140757cad mov rdi, rcx
140757cb0 mov rsi, r15
140757cb3 inc qword ptr [rbx]
140757cb6 mov r13, qword ptr [rdx + 8]
140757cba mov byte ptr [rbp + 0x433], 1
140757cc1 mov rcx, qword ptr [rbp + 0x3f8]
140757cc8 mov rdx, r13
140757ccb lea r8, [rip + 0x24c4776]
140757cd2 call 0x140a91ef0
140757cd7 mov rcx, rdi
140757cda mov rdx, rsi
140757cdd test rax, rax
140757ce0 jne 0x140757cf0
140757ce2 jmp 0x140757c7b
140757ce4 xor r15d, r15d
140757ce7 test rcx, rcx
140757cea jne 0x140757c89
140757cec jmp 0x140757d34
140757cee nop 
140757cf0 mov rdi, qword ptr [rbp + 0x3c0]
140757cf7 cmp rdi, qword ptr [rbp + 0x3b0]
140757cfe jne 0x140757c60
140757d04 mov byte ptr [rbp + 0x433], 1
140757d0b mov qword ptr [rsp + 0x20], 8
140757d14 mov r8d, 1
140757d1a mov r9d, 8
140757d20 lea rcx, [rbp + 0x3b0]
140757d27 mov rdx, rdi
140757d2a call 0x1428a7de0
140757d2f jmp 0x140757c60
140757d34 mov rdx, qword ptr [rbp + 0x418]
140757d3b mov eax, dword ptr [rdx + 0x1c]
140757d3e mov rcx, qword ptr [rdx + 0x10]
140757d42 mov dword ptr [rbp + 0x394], eax
140757d48 mov qword ptr [rbp + 0x210], rcx
140757d4f mov dword ptr [rcx], eax
140757d51 mov rax, qword ptr [rdx]
140757d54 mov rdi, qword ptr [rax]
140757d57 mov qword ptr [rbp + 0x410], rax
140757d5e mov rbx, qword ptr [rax + 0x18]
140757d62 movdqa xmm0, xmmword ptr [rdi]
140757d66 pmovmskb ecx, xmm0
140757d6a not ecx
140757d6c lea r15, [rdi + 0x10]
140757d70 lea r14, [rip + 0x2324ee1]
140757d77 jmp 0x140757d97
140757d79 nop dword ptr [rax]
140757d80 mov rax, qword ptr [rbp + 0x3b8]
140757d87 mov qword ptr [rax + r12*8], r13
140757d8b inc r12
140757d8e mov qword ptr [rbp + 0x3c0], r12
140757d95 mov ecx, esi
140757d97 test rbx, rbx
140757d9a je 0x140757e70
140757da0 test cx, cx
140757da3 je 0x140757db0
140757da5 lea esi, [rcx - 1]
140757da8 and esi, ecx
140757daa jmp 0x140757dd9
140757dac nop dword ptr [rax]
140757db0 movdqa xmm0, xmmword ptr [r15]
140757db5 pmovmskb ecx, xmm0
140757db9 add rdi, -0x680
140757dc0 add r15, 0x10
140757dc4 cmp ecx, 0xffff
140757dca je 0x140757db0
140757dcc mov eax, 0xfffffffe
140757dd1 sub eax, ecx
140757dd3 not ecx
140757dd5 mov esi, ecx
140757dd7 and esi, eax
140757dd9 tzcnt eax, ecx
140757ddd neg rax
140757de0 imul r12, rax, 0x68
140757de4 mov r13, qword ptr [rdi + r12 - 0x68]
140757de9 mov byte ptr [rbp + 0x433], 1
140757df0 mov rcx, qword ptr [rbp + 0x3f8]
140757df7 mov rdx, r13
140757dfa mov r8, r14
140757dfd call 0x140a91ef0
140757e02 dec rbx
140757e05 mov ecx, esi
140757e07 test rax, rax
140757e0a je 0x140757d97
140757e0c lea rcx, [rdi + r12]
140757e10 add rcx, -0x60
140757e14 movd xmm1, dword ptr [rax + 0x28c]
140757e1c cmp dword ptr [rax + 0x258], 2
140757e23 setne r8b
140757e27 mov byte ptr [rbp + 0x433], 1
140757e2e call 0x140f87300
140757e33 mov ecx, esi
140757e35 test al, al
140757e37 je 0x140757d97
140757e3d mov r12, qword ptr [rbp + 0x3c0]
140757e44 cmp r12, qword ptr [rbp + 0x3b0]
140757e4b jne 0x140757d80
140757e51 mov byte ptr [rbp + 0x433], 1
140757e58 lea rcx, [rbp + 0x3b0]
140757e5f lea rdx, [rip + 0x2324e0a]
140757e66 call 0x140562870
140757e6b jmp 0x140757d80
140757e70 mov rax, qword ptr [rbp + 0x3b8]
140757e77 mov qword ptr [rbp + 0x408], rax
140757e7e mov rdx, qword ptr [rbp + 0x3c0]
140757e85 cmp rdx, 1
140757e89 mov r12, qword ptr [rbp + 0x3f8]
140757e90 ja 0x140759d1e
140757e96 mov r10, qword ptr [rbp + 0x3e8]
140757e9d mov rcx, qword ptr [rbp + 0x588]
140757ea4 mov qword ptr [rbp + 0x1b0], 0
140757eaf mov qword ptr [rbp + 0x1b8], 4
140757eba mov qword ptr [rbp + 0x1c0], 0
140757ec5 mov rax, qword ptr [rbp + 0x408]
140757ecc lea rbx, [rax + rdx*8]
140757ed0 mov rcx, qword ptr [rcx]
140757ed3 mov qword ptr [rbp + 0x2d0], rcx
140757eda mov rdx, qword ptr [rbp + 0x2c8]
140757ee1 mov rcx, qword ptr [rdx]
140757ee4 mov qword ptr [rbp + 0x1c8], rcx
140757eeb mov rcx, qword ptr [rdx + 0x10]
140757eef mov qword ptr [rbp + 0xe0], rcx
140757ef6 mov r8, qword ptr [rbp + 0x550]
140757efd mov rcx, qword ptr [r8]
140757f00 mov qword ptr [rbp + 0xc8], rcx
140757f07 mov rcx, qword ptr [r8 + 8]
140757f0b mov qword ptr [rbp + 0xd0], rcx
140757f12 mov ecx, dword ptr [r8 + 0x14]
140757f16 mov dword ptr [rbp + 0x38c], ecx
140757f1c mov ecx, dword ptr [rdx + 0x1c]
140757f1f mov dword ptr [rbp + 0x2a4], ecx
140757f25 mov r8, qword ptr [rbp + 0x578]
140757f2c mov ecx, dword ptr [r8 + 0x1c]
140757f30 mov dword ptr [rbp + 0x29c], ecx
140757f36 mov rdx, qword ptr [r8]
140757f39 mov rcx, qword ptr [r8 + 0x10]
140757f3d mov qword ptr [rbp + 0xb8], rcx
140757f44 mov r9, qword ptr [rbp + 0x580]
140757f4b mov ecx, dword ptr [r9 + 0x1c]
140757f4f mov dword ptr [rbp + 0x2a0], ecx
140757f55 mov r8, qword ptr [r9]
140757f58 mov rcx, qword ptr [r9 + 0x10]
140757f5c mov qword ptr [rbp + 0xd8], rcx
140757f63 mov ecx, dword ptr [r10 + 0x1c]
140757f67 mov dword ptr [rbp + 0x388], ecx
140757f6d mov rcx, qword ptr [r10]
140757f70 mov qword ptr [rbp + 0x1d8], rcx
140757f77 mov rcx, qword ptr [r10 + 0x10]
140757f7b mov qword ptr [rbp + 0x1e0], rcx
140757f82 mov r10, qword ptr [rbp + 0x568]
140757f89 mov ecx, dword ptr [r10 + 0x1c]
140757f8d mov dword ptr [rbp + 0x390], ecx
140757f93 mov r9, qword ptr [r10]
140757f96 mov rcx, qword ptr [r10 + 0x10]
140757f9a mov qword ptr [rbp + 0x1f8], rcx
140757fa1 pcmpeqd xmm6, xmm6
140757fa5 xorps xmm7, xmm7
140757fa8 movss xmm15, dword ptr [rip + 0x23248bf]
140757fb1 movss xmm8, dword ptr [rip + 0x21cd8b6]
140757fba movaps xmm14, xmmword ptr [rip + 0x232489e]
140757fc2 movaps xmm10, xmmword ptr [rip + 0x21e2c96]
140757fca mov rcx, qword ptr [rbp + 0x3b0]
140757fd1 mov qword ptr [rbp + 0x208], rcx
140757fd8 lea r14, [r12 + 0x20]
140757fdd mov qword ptr [rbp + 0x2c8], rdx
140757fe4 lea rcx, [rdx + 0x20]
140757fe8 mov qword ptr [rbp + 0xc0], rcx
140757fef mov rcx, qword ptr [rbp + 0x410]
140757ff6 lea rcx, [rcx + 0x20]
140757ffa mov qword ptr [rbp + 0x2d8], rcx
140758001 mov qword ptr [rbp + 0x1f0], r8
140758008 lea rcx, [r8 + 0x20]
14075800c mov qword ptr [rbp + 0x108], rcx
140758013 mov qword ptr [rbp + 0x3e8], r9
14075801a lea rcx, [r9 + 0x20]
14075801e mov qword ptr [rbp + 0x218], rcx
140758025 mov r15, rax
140758028 mov qword ptr [rbp + 0x3a8], rax
14075802f mov qword ptr [rbp + 0x200], rbx
140758036 mov qword ptr [rbp + 0x1e8], r14
14075803d cmp r15, rbx
140758040 je 0x140759bc0
140758046 mov rsi, qword ptr [r15]
140758049 mov qword ptr [rbp + 0x428], rsi
140758050 cmp qword ptr [r12 + 0x18], 0
140758056 je 0x140759baf
14075805c add r15, 8
140758060 mov rcx, r14
140758063 lea rdx, [rbp + 0x428]
14075806a call 0x1405b0d70
14075806f mov rdi, qword ptr [r12]
140758073 mov rcx, qword ptr [r12 + 8]
140758078 mov rdx, rcx
14075807b and rdx, rax
14075807e shr rax, 0x39
140758082 movd xmm0, eax
140758086 punpcklbw xmm0, xmm0
14075808a pshuflw xmm0, xmm0, 0
14075808f pshufd xmm0, xmm0, 0
140758094 lea rax, [rdi - 0x2a8]
14075809b xor r8d, r8d
14075809e movdqu xmm1, xmmword ptr [rdi + rdx]
1407580a3 movdqa xmm2, xmm1
1407580a7 pcmpeqb xmm2, xmm0
1407580ab pmovmskb r9d, xmm2
1407580b0 test r9d, r9d
1407580b3 je 0x1407580e0
1407580b5 tzcnt r10d, r9d
1407580ba add r10, rdx
1407580bd and r10, rcx
1407580c0 neg r10
1407580c3 imul rbx, r10, 0x2a8
1407580ca cmp rsi, qword ptr [rax + rbx]
1407580ce je 0x140758110
1407580d0 lea r10d, [r9 - 1]
1407580d4 and r10w, r9w
1407580d8 mov r9d, r10d
1407580db jne 0x1407580b5
1407580dd nop dword ptr [rax]
1407580e0 pcmpeqb xmm1, xmm6
1407580e4 pmovmskb r9d, xmm1
1407580e9 test r9d, r9d
1407580ec jne 0x140759baf
1407580f2 add rdx, r8
1407580f5 add rdx, 0x10
1407580f9 add r8, 0x10
1407580fd and rdx, rcx
140758100 jmp 0x14075809e
140758102 nop word ptr cs:[rax + rax]
140758110 mov qword ptr [rbp + 0x1d0], r15
140758117 mov qword ptr [rbp + 0x2b0], 0
140758122 mov qword ptr [rbp + 0x2b8], 4
14075812d mov qword ptr [rbp + 0x2c0], 0
140758138 mov eax, dword ptr [rbp + 0x29c]
14075813e mov rcx, qword ptr [rbp + 0xb8]
140758145 mov dword ptr [rcx], eax
140758147 mov rax, qword ptr [rbp + 0x2c8]
14075814e mov r14, qword ptr [rax + 0x30]
140758152 mov r15, qword ptr [rax + 0x40]
140758156 cmp r14, qword ptr [rax + 0x20]
14075815a jne 0x140758176
14075815c mov byte ptr [rbp + 0x437], 1
140758163 mov rcx, qword ptr [rbp + 0xc0]
14075816a lea rdx, [rip + 0x24c4357]
140758171 call 0x140562630
140758176 mov rdx, qword ptr [rbp + 0x2c8]
14075817d mov rax, qword ptr [rdx + 0x28]
140758181 mov rcx, r14
140758184 shl rcx, 4
140758188 mov qword ptr [rax + rcx], r15
14075818c mov qword ptr [rax + rcx + 8], rsi
140758191 inc r14
140758194 mov qword ptr [rdx + 0x30], r14
140758198 inc qword ptr [rdx + 0x40]
14075819c lea rsi, [rdi + rbx]
1407581a0 lea rax, [rdi + rbx]
1407581a4 add rax, -0x2a0
1407581aa mov qword ptr [rbp + 0x420], rax
1407581b1 cmp byte ptr [rsi - 0xa], 0
1407581b5 jne 0x14075936e
1407581bb mov rdx, qword ptr [rbp + 0x428]
1407581c2 mov byte ptr [rbp + 0x437], 1
1407581c9 mov rcx, r12
1407581cc call 0x140a922b0
1407581d1 mov rdx, rax
1407581d4 shr rdx, 0x20
1407581d8 mov qword ptr [rbp + 0x418], rsi
1407581df jne 0x140758215
1407581e1 lea rcx, [rsi - 0xb]
1407581e5 mov byte ptr [rbp + 0x437], 1
1407581ec call 0x140833230
1407581f1 mov qword ptr [rbp + 0x3f0], rax
1407581f8 mov rax, qword ptr [rbp + 0x3f0]
1407581ff add al, al
140758201 mov qword ptr [rbp + 0x3f0], rax
140758208 xor r13d, r13d
14075820b xor r12d, r12d
14075820e xor edi, edi
140758210 jmp 0x14075837a
140758215 mov r9d, eax
140758218 mov rsi, qword ptr [rbp + 0xf8]
14075821f cmp r9, qword ptr [rsi + 0x10]
140758223 jae 0x140759def
140758229 mov rcx, qword ptr [rsi + 8]
14075822d mov rdx, rax
140758230 shr rdx, 0x20
140758234 lea r8, [r9 + r9*4]
140758238 cmp dword ptr [rcx + r8*4], edx
14075823c mov r11, qword ptr [rbp + 0x100]
140758243 jne 0x140759def
140758249 lea rcx, [rcx + r8*4]
14075824d mov r8d, dword ptr [rcx + 4]
140758251 mov r10d, 0xffffffff
140758257 cmp r8, r10
14075825a je 0x140759def
140758260 xor r9d, r9d
140758263 cmp qword ptr [r11 + 0x38], r8
140758267 jbe 0x140759dff
14075826d mov rdx, qword ptr [r11 + 0x28]
140758271 mov r10d, r8d
140758274 shr r10d, 6
140758278 mov rdx, qword ptr [rdx + r10*8]
14075827c bt rdx, r8
140758280 jae 0x140759dff
140758286 mov eax, dword ptr [rcx + 0xc]
140758289 mov rdx, qword ptr [rsi + 0x1a8]
140758290 lea r8, [rax + rax*8]
140758294 mov rax, qword ptr [r11 + 0x110]
14075829b cmp qword ptr [rdx + r8*8 + 0x40], rax
1407582a0 jbe 0x1407582d0
1407582a2 lea rdx, [rdx + r8*8]
1407582a6 mov r8, qword ptr [rdx + 0x38]
1407582aa mov rax, qword ptr [r8 + rax*8]
1407582ae test rax, rax
1407582b1 mov rsi, qword ptr [rbp + 0x418]
1407582b8 je 0x1407582db
1407582ba mov rdx, qword ptr [rdx + 0x18]
1407582be not rax
1407582c1 lea rax, [rax + rax*2]
1407582c5 shl rax, 4
1407582c9 mov rax, qword ptr [rdx + rax + 0x10]
1407582ce jmp 0x1407582dd
1407582d0 xor eax, eax
1407582d2 mov rsi, qword ptr [rbp + 0x418]
1407582d9 jmp 0x1407582dd
1407582db xor eax, eax
1407582dd mov ecx, dword ptr [rcx + 0x10]
1407582e0 imul rdi, rcx, 0x58
1407582e4 add rdi, rax
1407582e7 mov byte ptr [rbp + 0x437], 1
1407582ee mov rcx, rdi
1407582f1 call 0x1408e33b0
1407582f6 mov r13d, eax
1407582f9 mov byte ptr [rbp + 0x437], 1
140758300 mov rcx, rdi
140758303 call 0x1408e3250
140758308 test al, al
14075830a jne 0x14075833d
14075830c lea rcx, [rdi + 0x3d]
140758310 mov byte ptr [rbp + 0x437], 1
140758317 call 0x140833230
14075831c test al, al
14075831e je 0x14075833d
140758320 lea rcx, [rsi - 7]
140758324 mov byte ptr [rbp + 0x437], 1
14075832b call 0x1408c5670
140758330 add al, al
140758332 inc al
140758334 mov qword ptr [rbp + 0x3f0], rax
14075833b jmp 0x140758348
14075833d mov qword ptr [rbp + 0x3f0], 0
140758348 mov byte ptr [rbp + 0x437], 1
14075834f lea rcx, [rbp + 0x2f0]
140758356 mov rdx, rdi
140758359 call 0x1408e33e0
14075835e mov r12, qword ptr [rbp + 0x2f0]
140758365 mov r14, qword ptr [rbp + 0x2f8]
14075836c mov rax, qword ptr [rbp + 0x300]
140758373 mov qword ptr [rbp + 0x3d0], rax
14075837a mov byte ptr [rbp + 0x437], 1
140758381 lea rcx, [rbp + 0x428]
140758388 call 0x1400885d0
14075838d imul rax, rax, 0x4b28021
140758394 add rax, qword ptr [rsi - 0x258]
14075839b mov qword ptr [rbp + 0xe8], rax
1407583a2 mov r15, qword ptr [rsi - 0x208]
1407583a9 test r15, r15
1407583ac mov qword ptr [rbp + 0x3c8], r12
1407583b3 je 0x1407584ac
1407583b9 mov rsi, qword ptr [rsi - 0x210]
1407583c0 lea rbx, [r15*4]
1407583c8 movzx eax, byte ptr [rip + 0x2edc83a]
1407583cf mov edx, 4
1407583d4 mov rcx, rbx
1407583d7 call 0x140613c10
1407583dc test rax, rax
1407583df je 0x140759f5d
1407583e5 cmp r15, 7
1407583e9 mov qword ptr [rbp + 0x400], rax
1407583f0 jbe 0x14075841a
1407583f2 lea r8, [rax + r15*4]
1407583f6 lea rcx, [rsi + 4]
1407583fa lea rdx, [r15 + r15*2]
1407583fe lea rdx, [rsi + rdx*4]
140758402 add rdx, -4
140758406 cmp rax, rdx
140758409 setb dl
14075840c cmp rcx, r8
14075840f setb cl
140758412 test dl, cl
140758414 je 0x1407584ba
14075841a xor r10d, r10d
14075841d mov r8, r15
140758420 mov rcx, r10
140758423 and r8, 3
140758427 je 0x140758457
140758429 lea rcx, [r10 + r10*2]
14075842d lea rdx, [rsi + rcx*4]
140758431 add rdx, 4
140758435 lea r8, [r8 + r8*2]
140758439 xor r9d, r9d
14075843c mov rcx, r10
14075843f nop 
140758440 movd xmm0, dword ptr [rdx + r9*4]
140758446 movd dword ptr [rax + rcx*4], xmm0
14075844b inc rcx
14075844e add r9, 3
140758452 cmp r8, r9
140758455 jne 0x140758440
140758457 sub r10, r15
14075845a cmp r10, -4
14075845e ja 0x140758530
140758464 lea rdx, [rcx + rcx*2]
140758468 lea rdx, [rsi + rdx*4]
14075846c add rdx, 0x28
140758470 movss xmm0, dword ptr [rdx - 0x24]
140758475 movss dword ptr [rax + rcx*4], xmm0
14075847a movss xmm0, dword ptr [rdx - 0x18]
14075847f movss dword ptr [rax + rcx*4 + 4], xmm0
140758485 movss xmm0, dword ptr [rdx - 0xc]
14075848a movss dword ptr [rax + rcx*4 + 8], xmm0
140758490 movd xmm0, dword ptr [rdx]
140758494 movd dword ptr [rax + rcx*4 + 0xc], xmm0
14075849a add rcx, 4
14075849e add rdx, 0x30
1407584a2 cmp r15, rcx
1407584a5 jne 0x140758470
1407584a7 jmp 0x140758530
1407584ac mov eax, 4
1407584b1 mov qword ptr [rbp + 0x400], rax
1407584b8 jmp 0x140758537
1407584ba mov r10, r15
1407584bd and r10, 0xfffffffffffffff8
1407584c1 lea rcx, [rsi + 0x58]
1407584c5 xor edx, edx
1407584c7 nop word ptr [rax + rax]
1407584d0 movd xmm0, dword ptr [rcx - 0x54]
1407584d5 movd xmm1, dword ptr [rcx - 0x48]
1407584da punpckldq xmm0, xmm1
1407584de movd xmm1, dword ptr [rcx - 0x3c]
1407584e3 movd xmm2, dword ptr [rcx - 0x30]
1407584e8 punpckldq xmm1, xmm2
1407584ec punpcklqdq xmm0, xmm1
1407584f0 movd xmm1, dword ptr [rcx]
1407584f4 movd xmm2, dword ptr [rcx - 0xc]
1407584f9 punpckldq xmm2, xmm1
1407584fd movd xmm1, dword ptr [rcx - 0x18]
140758502 movd xmm3, dword ptr [rcx - 0x24]
140758507 punpckldq xmm3, xmm1
14075850b punpcklqdq xmm3, xmm2
14075850f movdqu xmmword ptr [rax + rdx*4], xmm0
140758514 movdqu xmmword ptr [rax + rdx*4 + 0x10], xmm3
14075851a add rdx, 8
14075851e add rcx, 0x60
140758522 cmp r10, rdx
140758525 jne 0x1407584d0
140758527 cmp r15, r10
14075852a jne 0x14075841d
140758530 mov rsi, qword ptr [rbp + 0x418]
140758537 mov qword ptr [rbp + 0x3d8], r15
14075853e mov rax, qword ptr [rsi - 0x1b8]
140758545 lea r12, [rax*4]
14075854d mov qword ptr [rbp + 0x3a0], rax
140758554 shr rax, 0x3e
140758558 jne 0x140759f88
14075855e mov r15d, 0
140758564 movabs rax, 0x7ffffffffffffffc
14075856e cmp r12, rax
140758571 ja 0x140759f8b
140758577 mov rbx, qword ptr [rsi - 0x1c0]
14075857e test r12, r12
140758581 je 0x1407585b6
140758583 movzx eax, byte ptr [rip + 0x2edc67f]
14075858a mov r15d, 4
140758590 mov edx, 4
140758595 mov rcx, r12
140758598 call 0x140613c10
14075859d test rax, rax
1407585a0 je 0x140759f8b
1407585a6 mov rcx, qword ptr [rbp + 0x3a0]
1407585ad mov qword ptr [rbp + 0x2e0], rcx
1407585b4 jmp 0x1407585c6
1407585b6 mov eax, 4
1407585bb mov qword ptr [rbp + 0x2e0], 0
1407585c6 mov qword ptr [rbp + 0x3e0], rax
1407585cd mov rcx, rax
1407585d0 mov rdx, rbx
1407585d3 mov r8, r12
1407585d6 call 0x142923c00
1407585db test rdi, rdi
1407585de je 0x1407586b0
1407585e4 mov rcx, rdi
1407585e7 call 0x1408e3380
1407585ec test al, al
1407585ee je 0x1407586b0
1407585f4 lea rcx, [rdi + 0x3d]
1407585f8 call 0x140833230
1407585fd test al, al
1407585ff jne 0x1407586b0
140758605 cmp qword ptr [rbp + 0x3a0], 0
14075860d je 0x1407586b0
140758613 add r12, -4
140758617 mov rax, qword ptr [rbp + 0x3e0]
14075861e cmp r12, 0x1b
140758622 jbe 0x140758674
140758624 shr r12, 2
140758628 inc r12
14075862b mov rcx, r12
14075862e and rcx, 0xfffffffffffffff8
140758632 mov r8, qword ptr [rbp + 0x3e0]
140758639 lea rax, [r8 + rcx*4]
14075863d xor edx, edx
14075863f nop 
140758640 movups xmm0, xmmword ptr [r8 + rdx*4]
140758645 movups xmm1, xmmword ptr [r8 + rdx*4 + 0x10]
14075864b addps xmm0, xmm14
14075864f addps xmm1, xmm14
140758653 maxps xmm0, xmm10
140758657 maxps xmm1, xmm10
14075865b movups xmmword ptr [r8 + rdx*4], xmm0
140758660 movups xmmword ptr [r8 + rdx*4 + 0x10], xmm1
140758666 add rdx, 8
14075866a cmp rcx, rdx
14075866d jne 0x140758640
14075866f cmp r12, rcx
140758672 je 0x1407586b0
140758674 mov rcx, qword ptr [rbp + 0x3e0]
14075867b mov rdx, qword ptr [rbp + 0x3a0]
140758682 lea rcx, [rcx + rdx*4]
140758686 nop word ptr cs:[rax + rax]
140758690 movss xmm0, dword ptr [rax]
140758694 addss xmm0, xmm15
140758699 maxss xmm0, xmm8
14075869e movss dword ptr [rax], xmm0
1407586a2 lea rdx, [rax + 4]
1407586a6 mov rax, rdx
1407586a9 cmp rdx, rcx
1407586ac jne 0x140758690
1407586ae nop 
1407586b0 mov rdx, qword ptr [rsi - 0x210]
1407586b7 mov rax, qword ptr [rsi - 0x208]
1407586be lea rax, [rax + rax*2]
1407586c2 lea r8, [rdx + rax*4]
1407586c6 lea rcx, [rbp + 0x230]
1407586cd lea r9, [rip + 0x24c04bc]
1407586d4 call 0x1402ba9e0
1407586d9 lea rcx, [rbp + 0x2f0]
1407586e0 lea rdx, [rbp + 0x230]
1407586e7 xor r8d, r8d
1407586ea call 0x1411f9920
1407586ef xor eax, eax
1407586f1 cmp rax, qword ptr [rbp + 0x2f0]
1407586f8 jo 0x140759f20
1407586fe mov rax, qword ptr [rbp + 0x320]
140758705 mov qword ptr [rbp + 0x170], rax
14075870c movdqu xmm0, xmmword ptr [rbp + 0x2f0]
140758714 movdqu xmm1, xmmword ptr [rbp + 0x300]
14075871c movdqu xmm2, xmmword ptr [rbp + 0x310]
140758724 movdqa xmmword ptr [rbp + 0x160], xmm2
14075872c movdqa xmmword ptr [rbp + 0x150], xmm1
140758734 movdqa xmmword ptr [rbp + 0x140], xmm0
14075873c mov r12b, 2
14075873f test r13b, r13b
140758742 je 0x1407587dd
140758748 mov rcx, qword ptr [rbp + 0x420]
14075874f call 0x140b2a620
140758754 movd dword ptr [rbp + 0x398], xmm0
14075875c lea rcx, [rsi - 7]
140758760 call 0x1408c5670
140758765 mov r12b, al
140758768 mov rcx, qword ptr [rbp + 0x420]
14075876f call 0x140b2a5d0
140758774 mov rcx, rax
140758777 call 0x1411ff740
14075877c test byte ptr [rbp + 0x3c8], 1
140758783 je 0x140759f7a
140758789 mov r13, rax
14075878c mov rcx, rax
14075878f mov rdx, r14
140758792 call 0x140c996d0
140758797 movdqa xmm11, xmm0
14075879c inc r14
14075879f mov rcx, r13
1407587a2 mov rdx, r14
1407587a5 call 0x140c996d0
1407587aa movdqa xmm12, xmm0
1407587af mov rcx, r13
1407587b2 mov rdx, qword ptr [rbp + 0x3d0]
1407587b9 call 0x140c996d0
1407587be movdqa xmm13, xmm0
1407587c3 mov rdx, qword ptr [rbp + 0x3d0]
1407587ca inc rdx
1407587cd mov rcx, r13
1407587d0 call 0x140c996d0
1407587d5 movd dword ptr [rbp + 0x39c], xmm0
1407587dd lea r15, [rsi - 7]
1407587e1 mov byte ptr [rbp + 0x434], 0
1407587e8 mov rcx, r15
1407587eb call 0x1408c5670
1407587f0 test al, al
1407587f2 je 0x1407588bf
1407587f8 mov eax, dword ptr [rbp + 0x394]
1407587fe mov rcx, qword ptr [rbp + 0x210]
140758805 mov dword ptr [rcx], eax
140758807 mov rsi, qword ptr [rbp + 0x428]
14075880e mov qword ptr [rbp + 0x2f0], rsi
140758815 mov rcx, qword ptr [rbp + 0x2d8]
14075881c lea rdx, [rbp + 0x2f0]
140758823 call 0x1405b0d70
140758828 mov rbx, rax
14075882b shr rbx, 0x39
14075882f mov rdx, qword ptr [rbp + 0x410]
140758836 mov rcx, qword ptr [rdx]
140758839 mov rdx, qword ptr [rdx + 8]
14075883d mov r8, rdx
140758840 and r8, rax
140758843 movd xmm0, ebx
140758847 punpcklbw xmm0, xmm0
14075884b pshuflw xmm0, xmm0, 0
140758850 pshufd xmm0, xmm0, 0
140758855 lea r9, [rcx - 0x68]
140758859 xor r10d, r10d
14075885c movdqu xmm1, xmmword ptr [rcx + r8]
140758862 movdqa xmm2, xmm1
140758866 pcmpeqb xmm2, xmm0
14075886a pmovmskb r11d, xmm2
14075886f test r11d, r11d
140758872 je 0x14075889d
140758874 tzcnt r14d, r11d
140758879 add r14, r8
14075887c and r14, rdx
14075887f neg r14
140758882 imul r14, r14, 0x68
140758886 cmp qword ptr [r9 + r14], rsi
14075888a je 0x140758982
140758890 lea r14d, [r11 - 1]
140758894 and r14w, r11w
140758898 mov r11d, r14d
14075889b jne 0x140758874
14075889d pcmpeqb xmm1, xmm6
1407588a1 pmovmskb r11d, xmm1
1407588a6 test r11d, r11d
1407588a9 jne 0x140758992
1407588af add r8, r10
1407588b2 add r8, 0x10
1407588b6 add r10, 0x10
1407588ba and r8, rdx
1407588bd jmp 0x14075885c
1407588bf mov eax, dword ptr [rbp + 0x394]
1407588c5 mov rcx, qword ptr [rbp + 0x210]
1407588cc mov dword ptr [rcx], eax
1407588ce mov rsi, qword ptr [rbp + 0x428]
1407588d5 mov qword ptr [rbp + 0x2f0], rsi
1407588dc mov rcx, qword ptr [rbp + 0x2d8]
1407588e3 lea rdx, [rbp + 0x2f0]
1407588ea call 0x1405b0d70
1407588ef mov rbx, rax
1407588f2 shr rbx, 0x39
1407588f6 mov rcx, qword ptr [rbp + 0x410]
1407588fd mov rdx, qword ptr [rcx]
140758900 mov rcx, qword ptr [rcx + 8]
140758904 mov r8, rcx
140758907 and r8, rax
14075890a movd xmm0, ebx
14075890e punpcklbw xmm0, xmm0
140758912 pshuflw xmm0, xmm0, 0
140758917 pshufd xmm0, xmm0, 0
14075891c lea r9, [rdx - 0x68]
140758920 xor r10d, r10d
140758923 movdqu xmm1, xmmword ptr [rdx + r8]
140758929 movdqa xmm2, xmm1
14075892d pcmpeqb xmm2, xmm0
140758931 pmovmskb r11d, xmm2
140758936 test r11d, r11d
140758939 je 0x140758960
14075893b tzcnt r14d, r11d
140758940 add r14, r8
140758943 and r14, rcx
140758946 neg r14
140758949 imul r14, r14, 0x68
14075894d cmp qword ptr [r9 + r14], rsi
140758951 je 0x14075898a
140758953 lea r14d, [r11 - 1]
140758957 and r14w, r11w
14075895b mov r11d, r14d
14075895e jne 0x14075893b
140758960 pcmpeqb xmm1, xmm6
140758964 pmovmskb r11d, xmm1
140758969 test r11d, r11d
14075896c jne 0x140758ac0
140758972 add r8, r10
140758975 add r8, 0x10
140758979 add r10, 0x10
14075897d and r8, rcx
140758980 jmp 0x140758923
140758982 add rcx, r14
140758985 jmp 0x140758a92
14075898a add rdx, r14
14075898d jmp 0x140758bbc
140758992 mov r14, rax
140758995 mov rax, qword ptr [rbp + 0x410]
14075899c cmp qword ptr [rax + 0x10], 0
1407589a1 je 0x140759b33
1407589a7 mov byte ptr [rbp + 0x434], 0
1407589ae lea rcx, [rbp + 0x2f8]
1407589b5 call 0x140f86e50
1407589ba mov qword ptr [rbp + 0x2f0], rsi
1407589c1 mov rcx, qword ptr [rbp + 0x410]
1407589c8 mov rax, qword ptr [rcx]
1407589cb mov rcx, qword ptr [rcx + 8]
1407589cf mov rdx, r14
1407589d2 and rdx, rcx
1407589d5 movdqu xmm0, xmmword ptr [rax + rdx]
1407589da pmovmskb r8d, xmm0
1407589df test r8d, r8d
1407589e2 je 0x140759ae9
1407589e8 tzcnt r8d, r8d
1407589ed add r8, rdx
1407589f0 and r8, rcx
1407589f3 movzx edx, byte ptr [rax + r8]
1407589f8 test dl, dl
1407589fa jns 0x140759b81
140758a00 lea r9, [r8 - 0x10]
140758a04 and r9, rcx
140758a07 mov byte ptr [rax + r8], bl
140758a0b mov byte ptr [rax + r9 + 0x10], bl
140758a10 neg r8
140758a13 imul r8, r8, 0x68
140758a17 lea rcx, [rax + r8]
140758a1b and dl, 1
140758a1e movzx edx, dl
140758a21 mov r9, qword ptr [rbp + 0x410]
140758a28 sub qword ptr [r9 + 0x10], rdx
140758a2c mov rdx, qword ptr [rbp + 0x350]
140758a33 mov qword ptr [rax + r8 - 8], rdx
140758a38 movups xmm0, xmmword ptr [rbp + 0x340]
140758a3f movups xmmword ptr [rax + r8 - 0x18], xmm0
140758a45 movups xmm0, xmmword ptr [rbp + 0x330]
140758a4c movups xmmword ptr [rax + r8 - 0x28], xmm0
140758a52 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
140758a5a movdqu xmm1, xmmword ptr [rbp + 0x300]
140758a62 movdqu xmm2, xmmword ptr [rbp + 0x310]
140758a6a movdqu xmm3, xmmword ptr [rbp + 0x320]
140758a72 movdqu xmmword ptr [rax + r8 - 0x38], xmm3
140758a79 movdqu xmmword ptr [rax + r8 - 0x48], xmm2
140758a80 movdqu xmmword ptr [rax + r8 - 0x58], xmm1
140758a87 movdqu xmmword ptr [rax + r8 - 0x68], xmm0
140758a8e inc qword ptr [r9 + 0x18]
140758a92 add rcx, -0x60
140758a96 mov byte ptr [rbp + 0x434], 0
140758a9d call 0x140f86eb0
140758aa2 mov r14, rax
140758aa5 movaps xmm9, xmm14
140758aa9 movaps xmm14, xmm8
140758aad movaps xmm8, xmm15
140758ab1 mov dword ptr [rbp + 0x3c8], 0
140758abb jmp 0x140758d49
140758ac0 mov r14, rax
140758ac3 mov rax, qword ptr [rbp + 0x410]
140758aca cmp qword ptr [rax + 0x10], 0
140758acf je 0x140759b5a
140758ad5 mov byte ptr [rbp + 0x434], 0
140758adc lea rcx, [rbp + 0x2f8]
140758ae3 call 0x140f86e50
140758ae8 mov qword ptr [rbp + 0x2f0], rsi
140758aef mov rcx, qword ptr [rbp + 0x410]
140758af6 mov rax, qword ptr [rcx]
140758af9 mov rcx, qword ptr [rcx + 8]
140758afd mov rdx, r14
140758b00 and rdx, rcx
140758b03 movdqu xmm0, xmmword ptr [rax + rdx]
140758b08 pmovmskb r8d, xmm0
140758b0d test r8d, r8d
140758b10 je 0x140759b0e
140758b16 tzcnt r8d, r8d
140758b1b add r8, rdx
140758b1e and r8, rcx
140758b21 movzx r9d, byte ptr [rax + r8]
140758b26 test r9b, r9b
140758b29 jns 0x140759b98
140758b2f lea rdx, [r8 - 0x10]
140758b33 and rdx, rcx
140758b36 mov byte ptr [rax + r8], bl
140758b3a mov byte ptr [rax + rdx + 0x10], bl
140758b3e neg r8
140758b41 imul rcx, r8, 0x68
140758b45 lea rdx, [rax + rcx]
140758b49 and r9b, 1
140758b4d movzx r8d, r9b
140758b51 mov r9, qword ptr [rbp + 0x410]
140758b58 sub qword ptr [r9 + 0x10], r8
140758b5c mov r8, qword ptr [rbp + 0x350]
140758b63 mov qword ptr [rax + rcx - 8], r8
140758b68 movups xmm0, xmmword ptr [rbp + 0x340]
140758b6f movups xmmword ptr [rax + rcx - 0x18], xmm0
140758b74 movups xmm0, xmmword ptr [rbp + 0x330]
140758b7b movups xmmword ptr [rax + rcx - 0x28], xmm0
140758b80 movdqu xmm0, xmmword ptr [rbp + 0x2f0]
140758b88 movdqu xmm1, xmmword ptr [rbp + 0x300]
140758b90 movdqu xmm2, xmmword ptr [rbp + 0x310]
140758b98 movdqu xmm3, xmmword ptr [rbp + 0x320]
140758ba0 movdqu xmmword ptr [rax + rcx - 0x38], xmm3
140758ba6 movdqu xmmword ptr [rax + rcx - 0x48], xmm2
140758bac movdqu xmmword ptr [rax + rcx - 0x58], xmm1
140758bb2 movdqu xmmword ptr [rax + rcx - 0x68], xmm0
140758bb8 inc qword ptr [r9 + 0x18]
140758bbc add rdx, -0x60
140758bc0 mov byte ptr [rbp + 0x434], 0
140758bc7 lea rcx, [rbp + 0x2f0]
140758bce call 0x140f86f50
140758bd3 movups xmm0, xmmword ptr [rbp + 0x340]
140758bda movaps xmmword ptr [rbp + 0x280], xmm0
140758be1 movups xmm0, xmmword ptr [rbp + 0x330]
140758be8 movaps xmmword ptr [rbp + 0x270], xmm0
140758bef movdqu xmm0, xmmword ptr [rbp + 0x2f0]
140758bf7 movdqu xmm1, xmmword ptr [rbp + 0x300]
140758bff movdqu xmm2, xmmword ptr [rbp + 0x310]
140758c07 movdqu xmm3, xmmword ptr [rbp + 0x320]
140758c0f movdqa xmmword ptr [rbp + 0x260], xmm3
140758c17 movdqa xmmword ptr [rbp + 0x250], xmm2
140758c1f movdqa xmmword ptr [rbp + 0x240], xmm1
140758c27 movdqa xmmword ptr [rbp + 0x230], xmm0
140758c2f mov byte ptr [rbp + 0x434], 1
140758c36 lea rcx, [rbp + 0x230]
140758c3d call 0x140f86f00
140758c42 movss xmm0, dword ptr [rbp + 0x398]
140758c4a movss dword ptr [rbp + 0x2f0], xmm0
140758c52 movd dword ptr [rbp + 0x2f4], xmm11
140758c5b movd dword ptr [rbp + 0x2f8], xmm12
140758c64 movd dword ptr [rbp + 0x2fc], xmm13
140758c6d movd xmm0, dword ptr [rbp + 0x39c]
140758c75 movd dword ptr [rbp + 0x300], xmm0
140758c7d mov byte ptr [rbp + 0x304], r12b
140758c84 mov rdx, qword ptr [rbp + 0x428]
140758c8b mov byte ptr [rbp + 0x434], 1
140758c92 mov rcx, qword ptr [rbp + 0x2d0]
140758c99 mov qword ptr [rsp + 0x50], rcx
140758c9e lea rcx, [rbp + 0x2f0]
140758ca5 mov qword ptr [rsp + 0x48], rcx
140758caa mov rcx, qword ptr [rbp + 0x3f0]
140758cb1 mov byte ptr [rsp + 0x40], cl
140758cb5 mov qword ptr [rsp + 0x38], rax
140758cba lea rax, [rbp + 0x2b0]
140758cc1 mov qword ptr [rsp + 0x30], rax
140758cc6 mov rax, qword ptr [rbp + 0x3a0]
140758ccd mov qword ptr [rsp + 0x28], rax
140758cd2 mov rax, qword ptr [rbp + 0x3e0]
140758cd9 mov qword ptr [rsp + 0x20], rax
140758cde mov rcx, qword ptr [rbp + 0x420]
140758ce5 mov r8, qword ptr [rbp + 0x400]
140758cec mov r9, qword ptr [rbp + 0x3d8]
140758cf3 call 0x141215540
140758cf8 mov rax, qword ptr [rbp + 0x418]
140758cff movd xmm1, dword ptr [rax - 0x14]
140758d04 cmp dword ptr [rax - 0x48], 2
140758d08 setne r8b
140758d0c mov byte ptr [rbp + 0x434], 1
140758d13 lea rcx, [rbp + 0x230]
140758d1a call 0x140f87300
140758d1f mov byte ptr [rbp + 0x434], 1
140758d26 lea rcx, [rbp + 0x230]
140758d2d call 0x140f86eb0
140758d32 mov r14, rax
140758d35 movaps xmm9, xmm14
140758d39 movaps xmm14, xmm8
140758d3d movaps xmm8, xmm15
140758d41 mov al, 1
140758d43 mov dword ptr [rbp + 0x3c8], eax
140758d49 mov qword ptr [rbp + 0x110], 0
140758d54 mov qword ptr [rbp + 0x118], 4
140758d5f mov qword ptr [rbp + 0x120], 0
140758d6a mov byte ptr [rbp + 0x436], 1
140758d71 mov rcx, r15
140758d74 call 0x1408c5670
140758d79 test rdi, rdi
140758d7c setne cl
140758d7f test cl, al
140758d81 je 0x140758e74
140758d87 mov byte ptr [rbp + 0x436], 1
140758d8e lea rcx, [rbp + 0x30]
140758d92 call 0x140b26a70
140758d97 mov qword ptr [rbp + 0x2f0], rax
140758d9e mov qword ptr [rbp + 0x2f8], rdx
140758da5 mov rax, qword ptr [rbp + 0x418]
140758dac movdqu xmm0, xmmword ptr [rax - 0x1a8]
140758db4 mov rdx, qword ptr [rbp + 0x428]
140758dbb mov byte ptr [rbp + 0x436], 1
140758dc2 lea rax, [rbp + 0x110]
140758dc9 mov qword ptr [rsp + 0x68], rax
140758dce mov qword ptr [rsp + 0x60], r14
140758dd3 mov qword ptr [rsp + 0x58], rdi
140758dd8 mov rax, qword ptr [rbp + 0x2d0]
140758ddf mov qword ptr [rsp + 0x50], rax
140758de4 mov rax, qword ptr [rbp + 0x3d8]
140758deb mov qword ptr [rsp + 0x48], rax
140758df0 mov rax, qword ptr [rbp + 0x400]
140758df7 mov qword ptr [rsp + 0x40], rax
140758dfc movdqu xmmword ptr [rsp + 0x30], xmm0
140758e02 mov rax, qword ptr [rbp + 0x2a8]
140758e09 mov qword ptr [rsp + 0x28], rax
140758e0e mov rax, qword ptr [rbp + 0x220]
140758e15 mov qword ptr [rsp + 0x20], rax
140758e1a mov rcx, qword ptr [rbp + 0x420]
140758e21 mov r8, qword ptr [rbp + 0xf0]
140758e28 lea r9, [rbp + 0x2f0]
140758e2f call 0x141218e30
140758e34 mov byte ptr [rbp + 0x436], 1
140758e3b mov rcx, rdi
140758e3e call 0x1408e33b0
140758e43 test al, al
140758e45 je 0x140758e74
140758e47 mov rdx, qword ptr [rbp + 0x428]
140758e4e mov byte ptr [rbp + 0x436], 1
140758e55 mov qword ptr [rsp + 0x20], rdi
140758e5a mov rcx, qword ptr [rbp + 0x420]
140758e61 mov r8, qword ptr [rbp + 0x2a8]
140758e68 mov r9, qword ptr [rbp + 0x2d0]
140758e6f call 0x141217be0
140758e74 mov rax, qword ptr [rbp + 0x110]
140758e7b mov qword ptr [rbp + 0x3d0], rax
140758e82 mov rax, qword ptr [rbp + 0x118]
140758e89 mov qword ptr [rbp + 0x228], rax
140758e90 mov rdi, qword ptr [rbp + 0x120]
140758e97 mov rbx, qword ptr [rbp + 0x428]
140758e9e mov eax, dword ptr [rbp + 0x2a0]
140758ea4 mov rcx, qword ptr [rbp + 0xd8]
140758eab mov dword ptr [rcx], eax
140758ead mov rax, qword ptr [rbp + 0x1f0]
140758eb4 mov rsi, qword ptr [rax + 0x30]
140758eb8 mov r13, qword ptr [rax + 0x40]
140758ebc cmp rsi, qword ptr [rax + 0x20]
140758ec0 jne 0x140758ed5
140758ec2 mov rcx, qword ptr [rbp + 0x108]
140758ec9 lea rdx, [rip + 0x24c35f8]
140758ed0 call 0x140562ab0
140758ed5 mov rdx, qword ptr [rbp + 0x1f0]
140758edc mov rax, qword ptr [rdx + 0x28]
140758ee0 lea rcx, [rsi + rsi*4]
140758ee4 mov r8, qword ptr [rbp + 0x3d0]
140758eeb mov qword ptr [rax + rcx*8], r8
140758eef mov r8, qword ptr [rbp + 0x228]
140758ef6 mov qword ptr [rax + rcx*8 + 8], r8
140758efb mov qword ptr [rax + rcx*8 + 0x10], rdi
140758f00 mov qword ptr [rax + rcx*8 + 0x18], rbx
140758f05 mov qword ptr [rax + rcx*8 + 0x20], r13
140758f0a inc rsi
140758f0d mov qword ptr [rdx + 0x30], rsi
140758f11 inc qword ptr [rdx + 0x40]
140758f15 cmp byte ptr [r15], 1
140758f19 jne 0x140758f75
140758f1b mov rax, qword ptr [rbp + 0x418]
140758f22 movdqu xmm0, xmmword ptr [rax - 0x1a8]
140758f2a mov rdx, qword ptr [rbp + 0x428]
140758f31 mov byte ptr [rbp + 0x436], 0
140758f38 mov qword ptr [rsp + 0x40], r14
140758f3d movdqu xmmword ptr [rsp + 0x30], xmm0
140758f43 mov rax, qword ptr [rbp + 0x3d8]
140758f4a mov qword ptr [rsp + 0x28], rax
140758f4f mov rax, qword ptr [rbp + 0x400]
140758f56 mov qword ptr [rsp + 0x20], rax
140758f5b mov rcx, qword ptr [rbp + 0x420]
140758f62 lea r8, [rbp + 0x2b0]
140758f69 lea r9, [rbp + 0x140]
140758f70 call 0x1412221f0
140758f75 mov rax, qword ptr [rbp + 0x3f0]
140758f7c dec eax
140758f7e test al, 0xfd
140758f80 jne 0x140758fbb
140758f82 mov rdx, qword ptr [rbp + 0x428]
140758f89 mov byte ptr [rbp + 0x436], 0
140758f90 mov qword ptr [rsp + 0x28], r14
140758f95 mov rax, qword ptr [rbp + 0x2d0]
140758f9c mov qword ptr [rsp + 0x20], rax
140758fa1 mov rcx, qword ptr [rbp + 0x420]
140758fa8 lea r8, [rbp + 0x2b0]
140758faf mov r9, qword ptr [rbp + 0x220]
140758fb6 call 0x141223d10
140758fbb mov rcx, qword ptr [rbp + 0x428]
140758fc2 mov byte ptr [rbp + 0x436], 0
140758fc9 call 0x140858790
140758fce mov edi, eax
140758fd0 mov rax, qword ptr [rbp + 0x418]
140758fd7 mov rbx, qword ptr [rax - 0x1a8]
140758fde mov r15, qword ptr [rax - 0x1a0]
140758fe5 pxor xmm6, xmm6
140758fe9 cmp r12b, 2
140758fed jne 0x140759056
140758fef lea rcx, [rax - 0xb]
140758ff3 mov byte ptr [rbp + 0x436], 0
140758ffa call 0x140833230
140758fff movd xmm6, dword ptr [rip + 0x232386d]
140759007 test al, al
140759009 mov rax, qword ptr [rbp + 0x418]
140759010 je 0x140759056
140759012 movss xmm0, dword ptr [rax - 0x14]
140759017 addss xmm0, dword ptr [rip + 0x2323859]
14075901f divss xmm0, dword ptr [rip + 0x2323855]
140759027 pxor xmm1, xmm1
14075902b maxss xmm1, xmm0
14075902f movss xmm0, dword ptr [rip + 0x21cc8cd]
140759037 minss xmm0, xmm1
14075903b movaps xmm1, xmm0
14075903e mulss xmm1, xmm0
140759042 addss xmm0, xmm0
140759046 movss xmm6, dword ptr [rip + 0x227402a]
14075904e subss xmm6, xmm0
140759052 mulss xmm6, xmm1
140759056 movss xmm7, dword ptr [rax - 0x2c]
14075905b movss xmm0, dword ptr [rbp + 0x398]
140759063 movss dword ptr [rbp + 0x2f0], xmm0
14075906b movd dword ptr [rbp + 0x2f4], xmm11
140759074 movd dword ptr [rbp + 0x2f8], xmm12
14075907d movd dword ptr [rbp + 0x2fc], xmm13
140759086 movss xmm0, dword ptr [rbp + 0x39c]
14075908e movss dword ptr [rbp + 0x300], xmm0
140759096 mov byte ptr [rbp + 0x304], r12b
14075909d movzx eax, byte ptr [rbp + 0x7a]
1407590a1 lea rcx, [rbp + 0x2f8]
1407590a8 mov byte ptr [rcx + 0xf], al
1407590ab movzx eax, word ptr [rbp + 0x78]
1407590af mov word ptr [rcx + 0xd], ax
1407590b3 mov byte ptr [rbp + 0x436], 0
1407590ba mov rcx, qword ptr [rbp + 0x420]
1407590c1 call 0x140b2a600
1407590c6 xor esi, esi
1407590c8 test rax, rax
1407590cb je 0x140759120
1407590cd test byte ptr [rax], 1
1407590d0 jne 0x140759120
1407590d2 movd xmm2, dword ptr [rax + 0xc]
1407590d7 mov byte ptr [rbp + 0x436], 0
1407590de movss xmm0, dword ptr [rip + 0x232379a]
1407590e6 movss xmm1, dword ptr [rip + 0x2323796]
1407590ee call 0x140c904a0
1407590f3 xorps xmm1, xmm1
1407590f6 maxss xmm1, xmm0
1407590fa movss xmm15, dword ptr [rip + 0x21cc801]
140759103 movaps xmm0, xmm15
140759107 minss xmm0, xmm1
14075910b subss xmm15, xmm0
140759110 mulss xmm15, dword ptr [rip + 0x232376f]
140759119 mov esi, 1
14075911e jmp 0x140759120
140759120 mov byte ptr [rbp + 0x436], 0
140759127 mov rcx, qword ptr [rbp + 0x420]
14075912e call 0x140b2a600
140759133 xor ecx, ecx
140759135 test rax, rax
140759138 je 0x140759194
14075913a test byte ptr [rax], 1
14075913d mov rdx, qword ptr [rbp + 0x400]
140759144 jne 0x14075919b
140759146 movd xmm2, dword ptr [rax + 0xc]
14075914b mov byte ptr [rbp + 0x436], 0
140759152 movss xmm0, dword ptr [rip + 0x2323726]
14075915a movss xmm1, dword ptr [rip + 0x2273f16]
140759162 call 0x140c904a0
140759167 xorps xmm1, xmm1
14075916a maxss xmm1, xmm0
14075916e movss xmm0, dword ptr [rip + 0x21cc78e]
140759176 movaps xmm2, xmm0
140759179 minss xmm2, xmm1
14075917d subss xmm0, xmm2
140759181 mulss xmm0, dword ptr [rip + 0x2323703]
140759189 addss xmm0, xmm2
14075918d mov ecx, 1
140759192 jmp 0x140759194
140759194 mov rdx, qword ptr [rbp + 0x400]
14075919b mov rax, qword ptr [rbp + 0x418]
1407591a2 movd xmm1, dword ptr [rax - 0x1c]
1407591a7 mov byte ptr [rbp + 0x436], 0
1407591ae mov dword ptr [rsp + 0x90], ecx
1407591b5 mov dword ptr [rsp + 0x80], esi
1407591bc mov qword ptr [rsp + 0x58], r14
1407591c1 mov rax, qword ptr [rbp + 0xe8]
1407591c8 mov qword ptr [rsp + 0x50], rax
1407591cd mov rax, qword ptr [rbp + 0x3f0]
1407591d4 mov byte ptr [rsp + 0x48], al
1407591d8 mov qword ptr [rsp + 0x40], r15
1407591dd movd dword ptr [rsp + 0xa0], xmm1
1407591e6 movss dword ptr [rsp + 0x98], xmm0
1407591ef movss dword ptr [rsp + 0x88], xmm15
1407591f9 lea rax, [rbp + 0x2f0]
140759200 mov qword ptr [rsp + 0x78], rax
140759205 movss dword ptr [rsp + 0x70], xmm7
14075920b lea rax, [rbp + 0x2b0]
140759212 mov qword ptr [rsp + 0x68], rax
140759217 movd dword ptr [rsp + 0x60], xmm6
14075921d mov qword ptr [rsp + 0x38], rbx
140759222 mov rax, qword ptr [rbp + 0x3d8]
140759229 mov qword ptr [rsp + 0x30], rax
14075922e mov qword ptr [rsp + 0x28], rdx
140759233 mov rax, qword ptr [rbp + 0x3a0]
14075923a mov qword ptr [rsp + 0x20], rax
14075923f mov byte ptr [rsp + 0xa8], 0
140759247 lea rcx, [rbp + 0x1b0]
14075924e lea rdx, [rbp + 0x140]
140759255 mov r8d, edi
140759258 mov r9, qword ptr [rbp + 0x3e0]
14075925f call 0x140769760
140759264 cmp byte ptr [rbp + 0x3c8], 0
14075926b mov r12, qword ptr [rbp + 0x3f8]
140759272 xorps xmm7, xmm7
140759275 mov rsi, qword ptr [rbp + 0x3d8]
14075927c je 0x1407592da
14075927e test byte ptr [rbp + 0x230], 1
140759285 je 0x140759aa7
14075928b mov rax, qword ptr [rbp + 0x238]
140759292 test rax, rax
140759295 je 0x1407592b1
140759297 mov rcx, qword ptr [rbp + 0x240]
14075929e shl rax, 2
1407592a2 lea rdx, [rax + rax*4]
1407592a6 mov r8d, 4
1407592ac call 0x140613c20
1407592b1 mov rax, qword ptr [rbp + 0x258]
1407592b8 lea rcx, [rbp + 0x260]
1407592bf test rax, rax
1407592c2 je 0x1407592da
1407592c4 mov rcx, qword ptr [rcx]
1407592c7 shl rax, 2
1407592cb lea rdx, [rax + rax*4]
1407592cf mov r8d, 4
1407592d5 call 0x140613c20
1407592da mov rdx, qword ptr [rbp + 0x140]
1407592e1 test rdx, rdx
1407592e4 movaps xmm15, xmm8
1407592e8 je 0x140759300
1407592ea mov rcx, qword ptr [rbp + 0x148]
1407592f1 shl rdx, 3
1407592f5 mov r8d, 4
1407592fb call 0x140613c20
140759300 mov rdx, qword ptr [rbp + 0x158]
140759307 test rdx, rdx
14075930a movaps xmm8, xmm14
14075930e je 0x140759326
140759310 mov rcx, qword ptr [rbp + 0x160]
140759317 shl rdx, 2
14075931b mov r8d, 4
140759321 call 0x140613c20
140759326 mov rdx, qword ptr [rbp + 0x2e0]
14075932d test rdx, rdx
140759330 pcmpeqd xmm6, xmm6
140759334 movaps xmm14, xmm9
140759338 je 0x140759350
14075933a shl rdx, 2
14075933e mov r8d, 4
140759344 mov rcx, qword ptr [rbp + 0x3e0]
14075934b call 0x140613c20
140759350 test rsi, rsi
140759353 je 0x14075936e
140759355 shl rsi, 2
140759359 mov r8d, 4
14075935f mov rcx, qword ptr [rbp + 0x400]
140759366 mov rdx, rsi
140759369 call 0x140613c20
14075936e mov rdx, qword ptr [rbp + 0x428]
140759375 mov byte ptr [rbp + 0x437], 1
14075937c mov rcx, qword ptr [rbp + 0x1c8]
140759383 call 0x140a91e00
140759388 mov rdx, rax
14075938b shr rdx, 0x20
14075938f jne 0x1407594a0
140759395 mov qword ptr [rbp + 0x140], 0
1407593a0 mov qword ptr [rbp + 0x148], 4
1407593ab lea rax, [rbp + 0x150]
1407593b2 movups xmmword ptr [rax], xmm7
1407593b5 mov qword ptr [rbp + 0x160], 4
1407593c0 movups xmmword ptr [rax + 0x18], xmm7
1407593c4 mov qword ptr [rbp + 0x178], 4
1407593cf movups xmmword ptr [rax + 0x30], xmm7
1407593d3 mov qword ptr [rbp + 0x190], 4
1407593de mov qword ptr [rbp + 0x198], 0
1407593e9 mov byte ptr [rbp + 0x1a0], 0
1407593f0 mov byte ptr [rbp + 0x435], 1
1407593f7 lea rcx, [rbp + 0x140]
1407593fe mov rdx, qword ptr [rbp + 0x420]
140759405 call 0x1408e9ed0
14075940a mov r14, qword ptr [rbp + 0x1e8]
140759411 lea r15, [rip + 0x24c158d]
140759418 mov eax, dword ptr [rbp + 0x388]
14075941e mov rcx, qword ptr [rbp + 0x1e0]
140759425 mov dword ptr [rcx], eax
140759427 mov rdx, qword ptr [rbp + 0x428]
14075942e mov byte ptr [rbp + 0x435], 1
140759435 mov rcx, qword ptr [rbp + 0x1d8]
14075943c call 0x1412259c0
140759441 mov rdi, rax
140759444 mov rdx, qword ptr [rbp + 0x428]
14075944b mov byte ptr [rbp + 0x435], 1
140759452 lea rcx, [rbp + 0x2f0]
140759459 call 0x1408bc510
14075945e test byte ptr [rbp + 0x2f0], 1
140759465 je 0x140759663
14075946b mov rax, qword ptr [rbp + 0x2f8]
140759472 mov qword ptr [rbp + 0x230], rax
140759479 mov byte ptr [rbp + 0x435], 1
140759480 lea rcx, [rbp + 0x230]
140759487 call 0x1400885d0
14075948c or eax, 0x1000000
140759491 jmp 0x140759669
140759496 nop word ptr cs:[rax + rax]
1407594a0 mov ecx, eax
1407594a2 mov rdi, qword ptr [rbp + 0xd0]
1407594a9 cmp rcx, qword ptr [rdi + 0x10]
1407594ad mov r11, qword ptr [rbp + 0xc8]
1407594b4 jae 0x140759e4b
1407594ba mov rdx, qword ptr [rdi + 8]
1407594be shr rax, 0x20
1407594c2 lea r9, [rcx + rcx*4]
1407594c6 mov r8d, 1
1407594cc cmp dword ptr [rdx + r9*4], eax
1407594d0 jne 0x140759dea
1407594d6 lea r9, [rdx + r9*4]
1407594da mov edx, dword ptr [r9 + 4]
1407594de mov r10d, 0xffffffff
1407594e4 cmp rdx, r10
1407594e7 je 0x140759dea
1407594ed cmp qword ptr [r11 + 0x38], rdx
1407594f1 jbe 0x140759de0
1407594f7 mov r8, qword ptr [r11 + 0x28]
1407594fb mov r10d, edx
1407594fe shr r10d, 6
140759502 mov r8, qword ptr [r8 + r10*8]
140759506 bt r8, rdx
14075950a jae 0x140759de0
140759510 mov eax, dword ptr [r9 + 0xc]
140759514 mov esi, dword ptr [r9 + 0x10]
140759518 mov rcx, qword ptr [rdi + 0x1a8]
14075951f lea rax, [rax + rax*8]
140759523 mov rdx, qword ptr [r11 + 0x110]
14075952a mov r8, qword ptr [r11 + 0x118]
140759531 mov r9, qword ptr [rcx + rax*8 + 0x18]
140759536 mov rax, qword ptr [rcx + rax*8 + 0x38]
14075953b mov rcx, qword ptr [rax + rdx*8]
14075953f not rcx
140759542 lea rdx, [rcx + rcx*2]
140759546 shl rdx, 4
14075954a mov r10, qword ptr [r9 + rdx + 0x28]
14075954f mov rax, qword ptr [rax + r8*8]
140759553 not rax
140759556 lea rax, [rax + rax*2]
14075955a shl rax, 4
14075955e mov rbx, qword ptr [r9 + rax + 0x28]
140759563 lea rcx, [rsi + rsi*2]
140759567 shl rcx, 4
14075956b add rcx, qword ptr [r9 + rdx + 0x10]
140759570 lea r8, [rsi*4]
140759578 add r8, qword ptr [r9 + rdx + 0x20]
14075957d mov qword ptr [rbp + 0x3a8], r8
140759584 imul rdi, rsi, 0x68
140759588 add rdi, qword ptr [r9 + rax + 0x10]
14075958d mov eax, dword ptr [rbp + 0x38c]
140759593 mov dword ptr [r10 + rsi*4], eax
140759597 mov rax, qword ptr [rbp + 0x2c0]
14075959e mov qword ptr [rbp + 0x300], rax
1407595a5 movups xmm0, xmmword ptr [rbp + 0x2b0]
1407595ac movaps xmmword ptr [rbp + 0x2f0], xmm0
1407595b3 mov byte ptr [rbp + 0x437], 0
1407595ba lea rdx, [rbp + 0x2f0]
1407595c1 call 0x140858830
1407595c6 mov eax, dword ptr [rbp + 0x38c]
1407595cc mov dword ptr [rbx + rsi*4], eax
1407595cf mov byte ptr [rbp + 0x437], 0
1407595d6 mov rcx, rdi
1407595d9 mov rdx, qword ptr [rbp + 0x420]
1407595e0 call 0x1408e9ed0
1407595e5 mov eax, dword ptr [rbp + 0x388]
1407595eb mov rcx, qword ptr [rbp + 0x1e0]
1407595f2 mov dword ptr [rcx], eax
1407595f4 mov rdx, qword ptr [rbp + 0x428]
1407595fb mov byte ptr [rbp + 0x437], 0
140759602 mov rcx, qword ptr [rbp + 0x1d8]
140759609 call 0x1412259c0
14075960e mov rbx, rax
140759611 mov rdx, qword ptr [rbp + 0x428]
140759618 mov byte ptr [rbp + 0x437], 0
14075961f lea rcx, [rbp + 0x2f0]
140759626 call 0x1408bc510
14075962b test byte ptr [rbp + 0x2f0], 1
140759632 je 0x1407599c0
140759638 mov rax, qword ptr [rbp + 0x2f8]
14075963f mov qword ptr [rbp + 0x230], rax
140759646 mov byte ptr [rbp + 0x437], 0
14075964d lea rcx, [rbp + 0x230]
140759654 call 0x1400885d0
140759659 or eax, 0x1000000
14075965e jmp 0x1407599c6
140759663 mov eax, dword ptr [rbp + 0x2f4]
140759669 mov dword ptr [rdi + 0x58], eax
14075966c mov qword ptr [rdi + 0x50], 0
140759674 mov byte ptr [rbp + 0x435], 1
14075967b lea rcx, [rbp + 0x140]
140759682 call 0x1408e80e0
140759687 add rdi, 0x40
14075968b lea rcx, [rdx + rdx*8]
14075968f lea rcx, [rax + rcx*4]
140759693 mov qword ptr [rbp + 0x2f0], 0
14075969e mov qword ptr [rbp + 0x330], 0
1407596a9 mov qword ptr [rbp + 0x370], rax
1407596b0 mov qword ptr [rbp + 0x378], rcx
1407596b7 mov byte ptr [rbp + 0x435], 1
1407596be mov rcx, rdi
1407596c1 lea rdx, [rbp + 0x2f0]
1407596c8 lea r8, [rip + 0x23235b9]
1407596cf call 0x141201e70
1407596d4 mov eax, dword ptr [rbp + 0x390]
1407596da mov rcx, qword ptr [rbp + 0x1f8]
1407596e1 mov dword ptr [rcx], eax
1407596e3 mov rax, qword ptr [rbp + 0x3e8]
1407596ea mov rsi, qword ptr [rax + 0x30]
1407596ee mov rdi, qword ptr [rax + 0x40]
1407596f2 cmp rsi, qword ptr [rax + 0x20]
1407596f6 jne 0x140759712
1407596f8 mov byte ptr [rbp + 0x435], 1
1407596ff mov rcx, qword ptr [rbp + 0x218]
140759706 lea rdx, [rip + 0x24c3b1b]
14075970d call 0x140562870
140759712 mov rcx, qword ptr [rbp + 0x3e8]
140759719 mov rax, qword ptr [rcx + 0x28]
14075971d mov qword ptr [rax + rsi*8], rdi
140759721 inc rsi
140759724 mov qword ptr [rcx + 0x30], rsi
140759728 inc qword ptr [rcx + 0x40]
14075972c mov byte ptr [rbp + 0x435], 1
140759733 mov rcx, qword ptr [rbp + 0x2a8]
14075973a lea rdx, [rip + 0x24c1137]
140759741 call 0x141209c40
140759746 mov rbx, rax
140759749 test rax, rax
14075974c je 0x140759dc8
140759752 mov qword ptr [rbp + 0x230], r15
140759759 mov qword ptr [rbp + 0x238], 0x3a
140759764 mov rdi, rbx
140759767 add rdi, 0x10
14075976b cmp qword ptr [rbx + 0x18], 0x3a
140759770 jne 0x140759d81
140759776 mov rdx, qword ptr [rdi]
140759779 mov r8d, 0x3a
14075977f mov rcx, r15
140759782 call 0x142923bf0
140759787 test eax, eax
140759789 jne 0x140759d81
14075978f mov rcx, qword ptr [rbx]
140759792 mov rax, qword ptr [rbx + 8]
140759796 mov rax, qword ptr [rax + 0x18]
14075979a mov byte ptr [rbp + 0x435], 1
1407597a1 call rax
1407597a3 mov rdi, rax
1407597a6 test rax, rax
1407597a9 je 0x140759dc8
1407597af mov rax, qword ptr [rbp + 0x1a0]
1407597b6 mov qword ptr [rbp + 0x290], rax
1407597bd movups xmm0, xmmword ptr [rbp + 0x190]
1407597c4 movaps xmmword ptr [rbp + 0x280], xmm0
1407597cb movups xmm0, xmmword ptr [rbp + 0x180]
1407597d2 movaps xmmword ptr [rbp + 0x270], xmm0
1407597d9 movups xmm0, xmmword ptr [rbp + 0x140]
1407597e0 movdqu xmm1, xmmword ptr [rbp + 0x150]
1407597e8 movdqu xmm2, xmmword ptr [rbp + 0x160]
1407597f0 movdqu xmm3, xmmword ptr [rbp + 0x170]
1407597f8 movdqa xmmword ptr [rbp + 0x260], xmm3
140759800 movdqa xmmword ptr [rbp + 0x250], xmm2
140759808 movdqa xmmword ptr [rbp + 0x240], xmm1
140759810 movaps xmmword ptr [rbp + 0x230], xmm0
140759817 mov rax, qword ptr [rbp + 0x2c0]
14075981e mov qword ptr [rbp + 0xb0], rax
140759825 movups xmm0, xmmword ptr [rbp + 0x2b0]
14075982c movaps xmmword ptr [rbp + 0xa0], xmm0
140759833 movzx eax, byte ptr [rip + 0x2edb3cf]
14075983a mov ecx, 0x20
14075983f mov edx, 8
140759844 call 0x140613c10
140759849 test rax, rax
14075984c je 0x140759f08
140759852 mov rcx, rax
140759855 mov qword ptr [rax], 0
14075985c mov qword ptr [rax + 8], 4
140759864 mov qword ptr [rax + 0x10], 0
14075986c mov byte ptr [rax + 0x18], 0
140759870 mov r9, qword ptr [rdi + 0x60]
140759874 jmp 0x140759882
140759876 nop word ptr cs:[rax + rax]
140759880 pause 
140759882 mov rax, qword ptr [r9 + 8]
140759886 nop word ptr cs:[rax + rax]
140759890 cmp rax, -1
140759894 je 0x140759880
140759896 test rax, rax
140759899 js 0x140759cd4
14075989f lea rdx, [rax + 1]
1407598a3 lock cmpxchg qword ptr [r9 + 8], rdx
1407598a9 jne 0x140759890
1407598ab mov rdx, qword ptr [rbp + 0x428]
1407598b2 mov byte ptr [rbp + 0x430], 1
1407598b9 mov qword ptr [rsp + 0x20], rcx
1407598be lea rcx, [rbp + 0x110]
1407598c5 lea r8, [rbp + 0xa0]
1407598cc call 0x140858800
1407598d1 mov rax, qword ptr [rbp + 0x290]
1407598d8 lea rcx, [rbp + 0x2f8]
1407598df mov qword ptr [rcx + 0x88], rax
1407598e6 movaps xmm0, xmmword ptr [rbp + 0x280]
1407598ed movups xmmword ptr [rcx + 0x78], xmm0
1407598f1 movaps xmm0, xmmword ptr [rbp + 0x270]
1407598f8 movups xmmword ptr [rcx + 0x68], xmm0
1407598fc movaps xmm0, xmmword ptr [rbp + 0x230]
140759903 movaps xmm1, xmmword ptr [rbp + 0x240]
14075990a movaps xmm2, xmmword ptr [rbp + 0x250]
140759911 movdqa xmm3, xmmword ptr [rbp + 0x260]
140759919 movdqu xmmword ptr [rcx + 0x58], xmm3
14075991e movups xmmword ptr [rcx + 0x48], xmm2
140759922 movups xmmword ptr [rcx + 0x38], xmm1
140759926 movups xmmword ptr [rcx + 0x28], xmm0
14075992a movups xmm0, xmmword ptr [rbp + 0x110]
140759931 movdqu xmm1, xmmword ptr [rbp + 0x120]
140759939 movdqu xmm2, xmmword ptr [rbp + 0x130]
140759941 movaps xmmword ptr [rbp + 0x2f0], xmm0
140759948 movdqa xmmword ptr [rbp + 0x300], xmm1
140759950 movdqa xmmword ptr [rbp + 0x310], xmm2
140759958 mov byte ptr [rbp + 0x430], 0
14075995f lea rcx, [rbp + 0x78]
140759963 mov rdx, qword ptr [rbp + 0x570]
14075996a lea r8, [rbp + 0x2f0]
140759971 lea r9, [rip + 0x23233b8]
140759978 call 0x14120f550
14075997d mov r8, qword ptr [rbp + 0x98]
140759984 mov rax, qword ptr [rbp + 0xe0]
14075998b mov ecx, dword ptr [rbp + 0x2a4]
140759991 mov dword ptr [rax], ecx
140759993 mov rdx, qword ptr [rbp + 0x428]
14075999a mov byte ptr [rbp + 0x435], 0
1407599a1 mov rcx, qword ptr [rbp + 0x1c8]
1407599a8 call 0x140eec970
1407599ad mov rbx, qword ptr [rbp + 0x200]
1407599b4 mov r15, qword ptr [rbp + 0x1d0]
1407599bb jmp 0x14075803d
1407599c0 mov eax, dword ptr [rbp + 0x2f4]
1407599c6 mov dword ptr [rbx + 0x58], eax
1407599c9 mov qword ptr [rbx + 0x50], 0
1407599d1 mov byte ptr [rbp + 0x437], 0
1407599d8 mov rcx, rdi
1407599db call 0x1408e80e0
1407599e0 add rbx, 0x40
1407599e4 lea rcx, [rdx + rdx*8]
1407599e8 lea rcx, [rax + rcx*4]
1407599ec mov qword ptr [rbp + 0x2f0], 0
1407599f7 mov qword ptr [rbp + 0x330], 0
140759a02 mov qword ptr [rbp + 0x370], rax
140759a09 mov qword ptr [rbp + 0x378], rcx
140759a10 mov byte ptr [rbp + 0x437], 0
140759a17 mov rcx, rbx
140759a1a lea rdx, [rbp + 0x2f0]
140759a21 lea r8, [rip + 0x2323260]
140759a28 call 0x141201e70
140759a2d mov eax, dword ptr [rbp + 0x390]
140759a33 mov rcx, qword ptr [rbp + 0x1f8]
140759a3a mov dword ptr [rcx], eax
140759a3c mov rax, qword ptr [rbp + 0x3e8]
140759a43 mov rsi, qword ptr [rax + 0x30]
140759a47 mov rdi, qword ptr [rax + 0x40]
140759a4b cmp rsi, qword ptr [rax + 0x20]
140759a4f jne 0x140759a6b
140759a51 mov byte ptr [rbp + 0x437], 0
140759a58 mov rcx, qword ptr [rbp + 0x218]
140759a5f lea rdx, [rip + 0x24c37c2]
140759a66 call 0x140562870
140759a6b shr qword ptr [rbp + 0x3a8], 0x20
140759a73 mov rcx, qword ptr [rbp + 0x3e8]
140759a7a mov rax, qword ptr [rcx + 0x28]
140759a7e mov qword ptr [rax + rsi*8], rdi
140759a82 inc rsi
140759a85 mov qword ptr [rcx + 0x30], rsi
140759a89 inc qword ptr [rcx + 0x40]
140759a8d mov rbx, qword ptr [rbp + 0x200]
140759a94 mov r14, qword ptr [rbp + 0x1e8]
140759a9b mov r15, qword ptr [rbp + 0x1d0]
140759aa2 jmp 0x14075803d
140759aa7 mov rax, qword ptr [rbp + 0x238]
140759aae test rax, rax
140759ab1 je 0x140759acd
140759ab3 mov rcx, qword ptr [rbp + 0x240]
140759aba shl rax, 2
140759abe lea rdx, [rax + rax*4]
140759ac2 mov r8d, 4
140759ac8 call 0x140613c20
140759acd mov rax, qword ptr [rbp + 0x250]
140759ad4 lea rcx, [rbp + 0x258]
140759adb test rax, rax
140759ade jne 0x1407592c4
140759ae4 jmp 0x1407592da
140759ae9 mov r9d, 0x10
140759aef add rdx, r9
140759af2 and rdx, rcx
140759af5 movdqu xmm0, xmmword ptr [rax + rdx]
140759afa pmovmskb r8d, xmm0
140759aff add r9, 0x10
140759b03 test r8d, r8d
140759b06 jne 0x1407589e8
140759b0c jmp 0x140759aef
140759b0e mov r9d, 0x10
140759b14 add rdx, r9
140759b17 and rdx, rcx
140759b1a movdqu xmm0, xmmword ptr [rax + rdx]
140759b1f pmovmskb r8d, xmm0
140759b24 add r9, 0x10
140759b28 test r8d, r8d
140759b2b jne 0x140758b16
140759b31 jmp 0x140759b14
140759b33 mov byte ptr [rbp + 0x434], 0
140759b3a mov edx, 1
140759b3f mov rcx, qword ptr [rbp + 0x410]
140759b46 mov r8, qword ptr [rbp + 0x2d8]
140759b4d mov r9b, 1
140759b50 call 0x1428b80c0
140759b55 jmp 0x1407589a7
140759b5a mov byte ptr [rbp + 0x434], 0
140759b61 mov edx, 1
140759b66 mov rcx, qword ptr [rbp + 0x410]
140759b6d mov r8, qword ptr [rbp + 0x2d8]
140759b74 mov r9b, 1
140759b77 call 0x1428b80c0
140759b7c jmp 0x140758ad5
140759b81 movdqa xmm0, xmmword ptr [rax]
140759b85 pmovmskb edx, xmm0
140759b89 tzcnt r8d, edx
140759b8e movzx edx, byte ptr [rax + r8]
140759b93 jmp 0x140758a00
140759b98 movdqa xmm0, xmmword ptr [rax]
140759b9c pmovmskb edx, xmm0
140759ba0 tzcnt r8d, edx
140759ba5 movzx r9d, byte ptr [rax + r8]
140759baa jmp 0x140758b2f
140759baf lea rcx, [rip + 0x2323042]
140759bb6 call 0x1428d9310
140759bbb jmp 0x140759f9d
140759bc0 mov rdx, qword ptr [rbp + 0x208]
140759bc7 test rdx, rdx
140759bca je 0x140759be2
140759bcc shl rdx, 3
140759bd0 mov r8d, 8
140759bd6 mov rcx, qword ptr [rbp + 0x408]
140759bdd call 0x140613c20
140759be2 mov rdx, qword ptr [rbp + 0x1b0]
140759be9 test rdx, rdx
140759bec je 0x140759c04
140759bee mov rcx, qword ptr [rbp + 0x1b8]
140759bf5 shl rdx, 4
140759bf9 mov r8d, 4
140759bff call 0x140613c20
140759c04 cmp byte ptr [rbp + 0x431], 0
140759c0b je 0x140759c58
140759c0d call 0x140c18130
140759c12 mov rcx, rax
140759c15 mov rax, qword ptr [rax]
140759c18 cmp rax, 1
140759c1c jne 0x140759cf3
140759c22 add rcx, 8
140759c26 mov rdx, qword ptr [rbp + 0x2e8]
140759c2d cmp qword ptr [rcx], 0
140759c31 jne 0x140759dbc
140759c37 mov qword ptr [rcx], 0xffffffffffffffff
140759c3e mov qword ptr [rbp + 0x3f8], rcx
140759c45 add rcx, 8
140759c49 call 0x140c170f0
140759c4e mov rax, qword ptr [rbp + 0x3f8]
140759c55 inc qword ptr [rax]
140759c58 movaps xmm6, xmmword ptr [rbp + 0x440]
140759c5f movaps xmm7, xmmword ptr [rbp + 0x450]
140759c66 movaps xmm8, xmmword ptr [rbp + 0x460]
140759c6e movaps xmm9, xmmword ptr [rbp + 0x470]
140759c76 movaps xmm10, xmmword ptr [rbp + 0x480]
140759c7e movaps xmm11, xmmword ptr [rbp + 0x490]
140759c86 movaps xmm12, xmmword ptr [rbp + 0x4a0]
140759c8e movaps xmm13, xmmword ptr [rbp + 0x4b0]
140759c96 movaps xmm14, xmmword ptr [rbp + 0x4c0]
140759c9e movaps xmm15, xmmword ptr [rbp + 0x4d0]
140759ca6 add rsp, 0x568
140759cad pop rbx
140759cae pop rdi
140759caf pop rsi
140759cb0 pop r12
140759cb2 pop r13
140759cb4 pop r14
140759cb6 pop r15
140759cb8 pop rbp
140759cb9 ret 
140759cba test rax, rax
140759cbd jne 0x140759d12
140759cbf xor edx, edx
140759cc1 call 0x1428f15c0
140759cc6 mov rcx, rax
140759cc9 test rax, rax
140759ccc jne 0x140757aba
140759cd2 jmp 0x140759d12
140759cd4 mov byte ptr [rbp + 0x432], 1
140759cdb lea rcx, [rip + 0x24c3576]
140759ce2 lea rdx, [rip + 0x24c35cf]
140759ce9 call 0x1428a9a00
140759cee jmp 0x140759f9d
140759cf3 test rax, rax
140759cf6 jne 0x140759d12
140759cf8 xor edx, edx
140759cfa call 0x1428f15c0
140759cff mov rdx, qword ptr [rbp + 0x2e8]
140759d06 mov rcx, rax
140759d09 test rax, rax
140759d0c jne 0x140759c2d
140759d12 lea rcx, [rip + 0x24bf78f]
140759d19 call 0x1428d80d0
140759d1e cmp rdx, 0x15
140759d22 jae 0x140759f9f
140759d28 mov r11, qword ptr [rbp + 0x408]
140759d2f lea rax, [r11 + rdx*8]
140759d33 lea rcx, [r11 + 8]
140759d37 mov edx, 8
140759d3c jmp 0x140759d55
140759d3e mov r9, r11
140759d41 mov qword ptr [r9], r8
140759d44 add rcx, 8
140759d48 add rdx, 8
140759d4c cmp rcx, rax
140759d4f je 0x140759fb9
140759d55 mov r10, qword ptr [rcx - 8]
140759d59 mov r8, qword ptr [rcx]
140759d5c cmp r8, r10
140759d5f jae 0x140759d44
140759d61 mov r9, rdx
140759d64 mov qword ptr [r11 + r9], r10
140759d68 cmp r9, 8
140759d6c je 0x140759d3e
140759d6e mov r10, qword ptr [r11 + r9 - 0x10]
140759d73 add r9, -8
140759d77 cmp r8, r10
140759d7a jb 0x140759d64
140759d7c add r9, r11
140759d7f jmp 0x140759d41
140759d81 mov qword ptr [rbp + 0x2f0], 0
140759d8c mov byte ptr [rbp + 0x435], 1
140759d93 lea rax, [rip + 0x24c0ac6]
140759d9a mov qword ptr [rsp + 0x20], rax
140759d9f lea rdx, [rbp + 0x230]
140759da6 lea r9, [rbp + 0x2f0]
140759dad xor ecx, ecx
140759daf mov r8, rdi
140759db2 call 0x1428cab41
140759db7 jmp 0x140759f9d
140759dbc lea rcx, [rip + 0x24bf755]
140759dc3 call 0x1428d9250
140759dc8 mov byte ptr [rbp + 0x435], 1
140759dcf lea rcx, [rip + 0x2322e6a]
140759dd6 call 0x1428d9310
140759ddb jmp 0x140759f9d
140759de0 mov qword ptr [rbp + 0x3a8], rdx
140759de7 xor r8d, r8d
140759dea mov rdx, rax
140759ded jmp 0x140759e51
140759def shl rdx, 0x20
140759df3 or rdx, r9
140759df6 mov r9d, 1
140759dfc mov rax, rdx
140759dff mov dword ptr [rbp + 0x2f0], r9d
140759e06 mov dword ptr [rbp + 0x2f4], r8d
140759e0d mov qword ptr [rbp + 0x2f8], rax
140759e14 mov byte ptr [rbp + 0x437], 1
140759e1b lea rax, [rip + 0x2322dee]
140759e22 mov qword ptr [rsp + 0x20], rax
140759e27 lea rcx, [rip + 0x2322ba2]
140759e2e lea r9, [rip + 0x2322b7b]
140759e35 lea r8, [rbp + 0x2f0]
140759e3c mov edx, 0x2b
140759e41 call 0x1428d9760
140759e46 jmp 0x140759f9d
140759e4b mov r8d, 1
140759e51 mov rax, qword ptr [rbp + 0x3a8]
140759e58 shl rax, 0x20
140759e5c or rax, r8
140759e5f mov qword ptr [rbp + 0x140], rax
140759e66 shl rdx, 0x20
140759e6a or rdx, rcx
140759e6d mov qword ptr [rbp + 0x148], rdx
140759e74 lea rax, [rbp + 0x428]
140759e7b mov qword ptr [rbp + 0x230], rax
140759e82 lea rax, [rip + 0x3db037]
140759e89 mov qword ptr [rbp + 0x238], rax
140759e90 lea rax, [rbp + 0x140]
140759e97 mov qword ptr [rbp + 0x240], rax
140759e9e lea rax, [rip - 0x2765]
140759ea5 mov qword ptr [rbp + 0x248], rax
140759eac lea rax, [rip + 0x2322e45]
140759eb3 mov qword ptr [rbp + 0x2f0], rax
140759eba mov qword ptr [rbp + 0x2f8], 2
140759ec5 mov qword ptr [rbp + 0x310], 0
140759ed0 lea rax, [rbp + 0x230]
140759ed7 mov qword ptr [rbp + 0x300], rax
140759ede mov qword ptr [rbp + 0x308], 2
140759ee9 mov byte ptr [rbp + 0x437], 1
140759ef0 lea rdx, [rip + 0x2322e21]
140759ef7 lea rcx, [rbp + 0x2f0]
140759efe call 0x1428d9390
140759f03 jmp 0x140759f9d
140759f08 mov byte ptr [rbp + 0x432], 1
140759f0f mov ecx, 8
140759f14 mov edx, 0x20
140759f19 call 0x1428d9000
140759f1e jmp 0x140759f9d
140759f20 movdqu xmm0, xmmword ptr [rbp + 0x2f8]
140759f28 movdqa xmmword ptr [rbp + 0x230], xmm0
140759f30 lea rax, [rip + 0x24bf441]
140759f37 mov qword ptr [rsp + 0x20], rax
140759f3c lea rcx, [rip + 0x24bec85]
140759f43 lea r9, [rip + 0x24bec5e]
140759f4a lea r8, [rbp + 0x230]
140759f51 mov edx, 0x2b
140759f56 call 0x1428d9760
140759f5b jmp 0x140759f9d
140759f5d mov byte ptr [rbp + 0x437], 1
140759f64 lea r8, [rip + 0x2322a2d]
140759f6b mov ecx, 4
140759f70 mov rdx, rbx
140759f73 call 0x1428d8fe3
140759f78 jmp 0x140759f9d
140759f7a lea rcx, [rip + 0x2322ca7]
140759f81 call 0x1428d9310
140759f86 jmp 0x140759f9d
140759f88 xor r15d, r15d
140759f8b lea r8, [rip + 0x24bffde]
140759f92 mov rcx, r15
140759f95 mov rdx, r12
140759f98 call 0x1428d8fe3
140759f9d ud2 
140759f9f mov byte ptr [rbp + 0x433], 1
140759fa6 lea r8, [rbp + 0x2f0]
140759fad mov rcx, qword ptr [rbp + 0x408]
140759fb4 call 0x140958d60
140759fb9 mov rax, qword ptr [rbp + 0x3b8]
140759fc0 mov qword ptr [rbp + 0x408], rax
140759fc7 mov rdx, qword ptr [rbp + 0x3c0]
140759fce cmp rdx, 2
140759fd2 jae 0x140759fe0
140759fd4 mov r12, qword ptr [rbp + 0x3f8]
140759fdb jmp 0x140757e96
140759fe0 mov eax, 1
140759fe5 mov r12, qword ptr [rbp + 0x3f8]
140759fec mov r10, qword ptr [rbp + 0x3e8]
140759ff3 mov r8, qword ptr [rbp + 0x408]
140759ffa mov rcx, qword ptr [r8 + rax*8]
