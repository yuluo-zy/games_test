140769760 push rbp
140769761 push r15
140769763 push r14
140769765 push r13
140769767 push r12
140769769 push rsi
14076976a push rdi
14076976b push rbx
14076976c sub rsp, 0x5e8
140769773 lea rbp, [rsp + 0x80]
14076977b movaps xmmword ptr [rbp + 0x550], xmm15
140769783 movaps xmmword ptr [rbp + 0x540], xmm14
14076978b movaps xmmword ptr [rbp + 0x530], xmm13
140769793 movaps xmmword ptr [rbp + 0x520], xmm12
14076979b movaps xmmword ptr [rbp + 0x510], xmm11
1407697a3 movaps xmmword ptr [rbp + 0x500], xmm10
1407697ab movaps xmmword ptr [rbp + 0x4f0], xmm9
1407697b3 movaps xmmword ptr [rbp + 0x4e0], xmm8
1407697bb movaps xmmword ptr [rbp + 0x4d0], xmm7
1407697c2 movaps xmmword ptr [rbp + 0x4c0], xmm6
1407697c9 mov qword ptr [rbp + 0x4b8], 0xfffffffffffffffe
1407697d4 mov qword ptr [rbp + 0x358], r9
1407697db mov dword ptr [rbp + 0x22c], r8d
1407697e2 mov qword ptr [rbp + 0x408], rdx
1407697e9 mov rdi, rcx
1407697ec call 0x140c16a00
1407697f1 mov ebx, eax
1407697f3 test al, al
1407697f5 je 0x140769854
1407697f7 mov r14d, 0x46
1407697fd mov ecx, 0x46
140769802 call 0x140769330
140769807 lea r15, [rip + 0x2314c72]
14076980e cmp rax, 1
140769812 jne 0x1407698c0
140769818 mov r9, rdx
14076981b test rdx, rdx
14076981e je 0x14076985d
140769820 cmp r9, 0x46
140769824 jae 0x14076985b
140769826 lea rax, [rip + 0x2314c53]
14076982d cmp byte ptr [r9 + rax], 0xbf
140769832 jg 0x14076985d
140769834 lea rax, [rip + 0x2314d15]
14076983b mov qword ptr [rsp + 0x20], rax
140769840 lea rcx, [rip + 0x2314c39]
140769847 mov edx, 0x46
14076984c xor r8d, r8d
14076984f call 0x1428d9940
140769854 xor esi, esi
140769856 jmp 0x140769a14
14076985b jne 0x140769834
14076985d mov rcx, r9
140769860 call 0x140769330
140769865 cmp rax, 1
140769869 jne 0x1407698c0
14076986b mov r8, rdx
14076986e add r8, 2
140769872 je 0x1407698ad
140769874 cmp r8, 0x46
140769878 jae 0x1407698ab
14076987a lea rax, [rip + 0x2314bff]
140769881 cmp byte ptr [r8 + rax], 0xbf
140769886 jg 0x1407698ad
140769888 lea rax, [rip + 0x2314cd9]
14076988f mov qword ptr [rsp + 0x20], rax
140769894 lea rcx, [rip + 0x2314be5]
14076989b mov edx, 0x46
1407698a0 mov r9d, 0x46
1407698a6 call 0x1428d9940
1407698ab jne 0x140769888
1407698ad mov r14d, 0x44
1407698b3 sub r14, rdx
1407698b6 lea r15, [rip + 0x2314bc3]
1407698bd add r15, r8
1407698c0 lea r8, [rip + 0x2314d08]
1407698c7 lea r12, [rip + 0x2314cca]
1407698ce nop 
1407698d0 movsx eax, byte ptr [r8 - 1]
1407698d5 test eax, eax
1407698d7 js 0x1407698f0
1407698d9 dec r8
1407698dc cmp eax, 0x5c
1407698df jne 0x140769944
1407698e1 jmp 0x140769955
1407698e3 nop word ptr cs:[rax + rax]
1407698f0 movzx ecx, byte ptr [r8 - 2]
1407698f5 cmp cl, 0xc0
1407698f8 jge 0x14076991d
1407698fa movzx edx, byte ptr [r8 - 3]
1407698ff cmp dl, 0xc0
140769902 jge 0x140769926
140769904 movzx r9d, byte ptr [r8 - 4]
140769909 add r8, -4
14076990d and r9d, 7
140769911 shl r9d, 6
140769915 and edx, 0x3f
140769918 or edx, r9d
14076991b jmp 0x14076992d
14076991d add r8, -2
140769921 and ecx, 0x1f
140769924 jmp 0x140769935
140769926 add r8, -3
14076992a and edx, 0xf
14076992d shl edx, 6
140769930 and ecx, 0x3f
140769933 or ecx, edx
140769935 shl ecx, 6
140769938 and al, 0x3f
14076993a movzx eax, al
14076993d or eax, ecx
14076993f cmp eax, 0x5c
140769942 je 0x140769955
140769944 cmp eax, 0x2f
140769947 je 0x140769955
140769949 cmp r8, r12
14076994c jne 0x1407698d0
14076994e mov esi, 0x37
140769953 jmp 0x1407699a1
140769955 lea r12, [rip + 0x2314c3c]
14076995c sub r8, r12
14076995f inc r8
140769962 je 0x140769996
140769964 cmp r8, 0x37
140769968 jae 0x140769994
14076996a cmp byte ptr [r8 + r12], 0xbf
14076996f jg 0x140769996
140769971 lea rax, [rip + 0x2314bb8]
140769978 mov qword ptr [rsp + 0x20], rax
14076997d lea rcx, [rip + 0x2314c14]
140769984 mov edx, 0x37
140769989 mov r9d, 0x37
14076998f call 0x1428d9940
140769994 jne 0x140769971
140769996 mov esi, 0x37
14076999b sub rsi, r8
14076999e add r12, r8
1407699a1 call 0x140c18130
1407699a6 mov rcx, rax
1407699a9 mov rax, qword ptr [rax]
1407699ac cmp rax, 1
1407699b0 jne 0x14076c6db
1407699b6 add rcx, 8
1407699ba cmp qword ptr [rcx], 0
1407699be jne 0x14076c750
1407699c4 mov qword ptr [rcx], 0xffffffffffffffff
1407699cb mov qword ptr [rbp + 0x4a8], rcx
1407699d2 add rcx, 8
1407699d6 mov qword ptr [rsp + 0x20], rsi
1407699db mov qword ptr [rsp + 0x30], 0
1407699e4 mov qword ptr [rsp + 0x28], 1
1407699ed mov rdx, r15
1407699f0 mov r8, r14
1407699f3 mov r9, r12
1407699f6 call 0x140c16d40
1407699fb mov r12, rax
1407699fe mov rax, qword ptr [rbp + 0x4a8]
140769a05 inc qword ptr [rax]
140769a08 mov qword ptr [rbp + 0x128], r12
140769a0f mov esi, 1
140769a14 mov qword ptr [rbp + 0x120], rsi
140769a1b mov rax, qword ptr [rbp + 0x408]
140769a22 mov rax, qword ptr [rax + 0x10]
140769a26 cmp rax, 1
140769a2a jbe 0x14076c75c
140769a30 mov rcx, qword ptr [rbp + 0x5d0]
140769a37 mov qword ptr [rbp + 0x80], rcx
140769a3e mov qword ptr [rbp + 0x2d0], rax
140769a45 cmp rax, rcx
140769a48 jne 0x14076c780
140769a4e mov rax, qword ptr [rbp + 0x5e0]
140769a55 mov qword ptr [rbp + 0x428], rax
140769a5c mov qword ptr [rbp + 0x2d0], rcx
140769a63 cmp rcx, rax
140769a66 jne 0x14076c7bf
140769a6c mov rax, qword ptr [rbp + 0x5f0]
140769a73 mov qword ptr [rbp + 0x428], rax
140769a7a mov qword ptr [rbp + 0x2d0], rcx
140769a81 cmp rcx, rax
140769a84 jne 0x14076c7fe
140769a8a movss xmm6, dword ptr [rbp + 0x650]
140769a92 movss xmm7, dword ptr [rbp + 0x648]
140769a9a movss xmm0, dword ptr [rbp + 0x638]
140769aa2 movss dword ptr [rbp + 0x328], xmm0
140769aaa movss xmm8, dword ptr [rbp + 0x620]
140769ab3 mov r14, qword ptr [rbp + 0x608]
140769aba mov rdx, rcx
140769abd mov r13, qword ptr [rbp + 0x600]
140769ac4 mov rax, qword ptr [rbp + 0x408]
140769acb movss xmm0, dword ptr [rax + 0x30]
140769ad0 movaps xmmword ptr [rbp + 0x2a0], xmm0
140769ad7 mov rcx, qword ptr [rbp + 0x358]
140769ade movss xmm10, dword ptr [rcx]
140769ae3 lea r15, [rdx*4]
140769aeb xor eax, eax
140769aed movaps xmm0, xmm10
140769af1 nop word ptr cs:[rax + rax]
140769b00 movss xmm1, dword ptr [rcx + rax]
140769b05 cmpunordss xmm0, xmm0
140769b0a movaps xmm2, xmm0
140769b0d andps xmm2, xmm1
140769b10 maxss xmm1, xmm10
140769b15 andnps xmm0, xmm1
140769b18 orps xmm0, xmm2
140769b1b movaps xmm10, xmm0
140769b1f add rax, 4
140769b23 cmp r15, rax
140769b26 jne 0x140769b00
140769b28 xorps xmm0, xmm0
140769b2b ucomiss xmm10, xmm0
140769b2f jne 0x140769b7d
140769b31 jp 0x140769b7d
140769b33 test bl, bl
140769b35 je 0x14076c679
140769b3b call 0x140c18130
140769b40 mov rcx, rax
140769b43 mov rax, qword ptr [rax]
140769b46 cmp rax, 1
140769b4a jne 0x14076c6f5
140769b50 add rcx, 8
140769b54 cmp qword ptr [rcx], 0
140769b58 jne 0x14076c750
140769b5e mov qword ptr [rcx], 0xffffffffffffffff
140769b65 mov qword ptr [rbp + 0x4a8], rcx
140769b6c add rcx, 8
140769b70 mov rdx, r12
140769b73 call 0x140c170f0
140769b78 jmp 0x14076c66f
140769b7d mov rax, qword ptr [rbp + 0x628]
140769b84 movzx ecx, byte ptr [rax + 0x14]
140769b88 mov byte ptr [rbp + 0x4b6], cl
140769b8e cmp cl, 2
140769b91 jne 0x140769c0f
140769b93 movzx eax, byte ptr [rip + 0x2ecb06f]
140769b9a mov ecx, 8
140769b9f mov edx, 4
140769ba4 call 0x140613c10
140769ba9 cmp byte ptr [rbp + 0x5f8], 3
140769bb0 jne 0x140769c4c
140769bb6 test rax, rax
140769bb9 je 0x14076c92e
140769bbf addss xmm10, dword ptr [rip + 0x2314758]
140769bc8 addss xmm10, dword ptr [rip + 0x21bbd4f]
140769bd1 movss xmm0, dword ptr [rip + 0x21bbdab]
140769bd9 addss xmm0, xmm8
140769bde movaps xmm1, xmm10
140769be2 cmpunordss xmm1, xmm10
140769be8 movaps xmm2, xmm1
140769beb andps xmm2, xmm0
140769bee maxss xmm0, xmm10
140769bf3 andnps xmm1, xmm0
140769bf6 orps xmm1, xmm2
140769bf9 movss dword ptr [rax], xmm8
140769bfe movss dword ptr [rax + 4], xmm1
140769c03 mov r8d, 2
140769c09 movaps xmm10, xmm1
140769c0d jmp 0x140769c66
140769c0f movss xmm9, dword ptr [rax]
140769c14 movzx eax, byte ptr [rip + 0x2ecafee]
140769c1b mov ecx, 0xc
140769c20 mov edx, 4
140769c25 call 0x140613c10
140769c2a test rax, rax
140769c2d je 0x14076c8ef
140769c33 movss dword ptr [rax], xmm8
140769c38 movss dword ptr [rax + 4], xmm9
140769c3e movss dword ptr [rax + 8], xmm10
140769c44 mov r8d, 3
140769c4a jmp 0x140769c66
140769c4c test rax, rax
140769c4f je 0x14076c946
140769c55 movss dword ptr [rax], xmm8
140769c5a movss dword ptr [rax + 4], xmm10
140769c60 mov r8d, 2
140769c66 mov qword ptr [rbp + 0x118], rsi
140769c6d mov qword ptr [rsp + 0x20], r13
140769c72 lea rcx, [rbp + 0xc8]
140769c79 mov qword ptr [rbp + 0x170], rax
140769c80 mov rdx, rax
140769c83 mov qword ptr [rbp + 0x178], r8
140769c8a movaps xmm3, xmm10
140769c8e call 0x14123e110
140769c93 mov rbx, qword ptr [rbp + 0xd8]
140769c9a cmp byte ptr [rbp + 0x658], 0
140769ca1 mov rsi, qword ptr [rbp + 0x358]
140769ca8 je 0x140769cd8
140769caa movzx eax, byte ptr [rip + 0x2ecaf58]
140769cb1 mov edx, 4
140769cb6 mov rcx, r15
140769cb9 call 0x140613c10
140769cbe test rax, rax
140769cc1 je 0x14076c907
140769cc7 mov rcx, qword ptr [rbp + 0x5d0]
140769cce cmp rcx, 7
140769cd2 ja 0x140769cfb
140769cd4 xor edx, edx
140769cd6 jmp 0x140769d3d
140769cd8 mov eax, 4
140769cdd mov qword ptr [rbp + 0x298], rax
140769ce4 mov qword ptr [rbp + 0x290], 0
140769cef mov rax, rsi
140769cf2 mov rcx, qword ptr [rbp + 0x5d0]
140769cf9 jmp 0x140769d74
140769cfb mov rdx, rcx
140769cfe and rdx, 0xfffffffffffffff8
140769d02 xor ecx, ecx
140769d04 movaps xmm0, xmmword ptr [rip + 0x2314625]
140769d0b nop dword ptr [rax + rax]
140769d10 movups xmm1, xmmword ptr [rsi + rcx*4]
140769d14 movups xmm2, xmmword ptr [rsi + rcx*4 + 0x10]
140769d19 addps xmm1, xmm0
140769d1c addps xmm2, xmm0
140769d1f movups xmmword ptr [rax + rcx*4], xmm1
140769d23 movups xmmword ptr [rax + rcx*4 + 0x10], xmm2
140769d28 add rcx, 8
140769d2c cmp rdx, rcx
140769d2f jne 0x140769d10
140769d31 mov rcx, qword ptr [rbp + 0x5d0]
140769d38 cmp rdx, rcx
140769d3b je 0x140769d66
140769d3d movss xmm0, dword ptr [rip + 0x2236a1f]
140769d45 nop word ptr cs:[rax + rax]
140769d50 movss xmm1, dword ptr [rsi + rdx*4]
140769d55 addss xmm1, xmm0
140769d59 movss dword ptr [rax + rdx*4], xmm1
140769d5e inc rdx
140769d61 cmp rcx, rdx
140769d64 jne 0x140769d50
140769d66 mov qword ptr [rbp + 0x290], rcx
140769d6d mov qword ptr [rbp + 0x298], rax
140769d74 mov rdx, qword ptr [rbp + 0xd0]
140769d7b cmp byte ptr [rbp + 0x4b6], 2
140769d82 setne byte ptr [rsp + 0x40]
140769d87 movaps xmm0, xmmword ptr [rbp + 0x2a0]
140769d8e movss dword ptr [rsp + 0x38], xmm0
140769d94 movss dword ptr [rsp + 0x30], xmm10
140769d9b mov qword ptr [rsp + 0x28], rcx
140769da0 mov qword ptr [rsp + 0x20], rax
140769da5 lea rcx, [rbp + 0xf8]
140769dac mov r8, rbx
140769daf mov r9, qword ptr [rbp + 0x408]
140769db6 call 0x14123eef0
140769dbb mov rcx, r14
140769dbe call 0x14067dd80
140769dc3 mov r14, rax
140769dc6 mov rcx, rax
140769dc9 call 0x140f86cf0
140769dce unpcklps xmm0, xmm10
140769dd2 movaps xmm1, xmmword ptr [rip + 0x21c48d7]
140769dd9 divps xmm1, xmm0
140769ddc movlps qword ptr [rbp + 0x2d0], xmm1
140769de3 mov qword ptr [rdi + 0x10], 0
140769deb movss xmm9, dword ptr [rip + 0x2236970]
140769df4 lea r8, [rbp + 0x2d0]
140769dfb mov rcx, r14
140769dfe movaps xmm1, xmm9
140769e02 mov r9, rdi
140769e05 call 0x14123fc70
140769e0a mov rcx, qword ptr [rdi + 8]
140769e0e mov rdx, qword ptr [rdi + 0x10]
140769e12 lea rax, [rbp + 0x428]
140769e19 mov qword ptr [rbp + 0x3a0], rax
140769e20 cmp rdx, 2
140769e24 jae 0x14076c83d
140769e2a mov rax, qword ptr [rdi + 8]
140769e2e mov qword ptr [rbp + 0x160], rax
140769e35 mov rsi, qword ptr [rdi + 0x10]
140769e39 mov qword ptr [rbp + 0x340], 0
140769e44 mov qword ptr [rbp + 0x348], 8
140769e4f mov qword ptr [rbp + 0x350], 0
140769e5a mov qword ptr [rbp + 0x2b0], 0
140769e65 mov qword ptr [rbp + 0x2b8], 4
140769e70 mov qword ptr [rbp + 0x2c0], 0
140769e7b movzx eax, byte ptr [rip + 0x2ecad87]
140769e82 mov ecx, 0x500
140769e87 mov edx, 4
140769e8c call 0x140613c10
140769e91 test rax, rax
140769e94 je 0x14076c8bb
140769e9a xorps xmm0, xmm0
140769e9d movups xmmword ptr [rbp + 0x438], xmm0
140769ea4 mov qword ptr [rbp + 0x428], 0x20
140769eaf mov qword ptr [rbp + 0x430], rax
140769eb6 movaps xmm0, xmmword ptr [rbp + 0x2a0]
140769ebd divss xmm0, dword ptr [rip + 0x223689f]
140769ec5 call 0x142923e60
140769eca cvttss2si rax, xmm0
140769ecf mov rcx, rax
140769ed2 sar rcx, 0x3f
140769ed6 movaps xmm1, xmm0
140769ed9 subss xmm1, dword ptr [rip + 0x21f01fb]
140769ee1 cvttss2si rdx, xmm1
140769ee6 and rdx, rcx
140769ee9 or rdx, rax
140769eec xor edi, edi
140769eee xorps xmm1, xmm1
140769ef1 ucomiss xmm0, xmm1
140769ef4 cmovb rdx, rdi
140769ef8 ucomiss xmm0, dword ptr [rip + 0x21f01e1]
140769eff mov r12, 0xffffffffffffffff
140769f06 cmovbe r12, rdx
140769f0a lea r14, [r12*4 + 4]
140769f12 inc r12
140769f15 mov rax, r12
140769f18 shr rax, 0x3e
140769f1c jne 0x140769f50
140769f1e movabs rax, 0x7ffffffffffffffc
140769f28 cmp r14, rax
140769f2b ja 0x140769f50
140769f2d test r14, r14
140769f30 je 0x140769f67
140769f32 movzx eax, byte ptr [rip + 0x2ecacd0]
140769f39 mov edi, 4
140769f3e mov edx, 4
140769f43 mov rcx, r14
140769f46 call 0x140613c10
140769f4b test rax, rax
140769f4e jne 0x140769f6f
140769f50 lea r8, [rip + 0x2314719]
140769f57 mov rcx, rdi
140769f5a mov rdx, r14
140769f5d call 0x1428d8fe3
140769f62 jmp 0x14076c95c
140769f67 mov eax, 4
140769f6c xor r12d, r12d
140769f6f mov qword ptr [rbp + 0x308], r12
140769f76 mov qword ptr [rbp + 0x310], rax
140769f7d mov qword ptr [rbp + 0x318], 0
140769f88 imul rax, r13, 0x17d32
140769f8f imul rcx, rbx, 0x8ba
140769f96 add rcx, rax
140769f99 mov qword ptr [rbp + 0x78], rcx
140769f9d test r13, r13
140769fa0 js 0x140769fac
140769fa2 xorps xmm0, xmm0
140769fa5 cvtsi2ss xmm0, r13
140769faa jmp 0x140769fc7
140769fac mov rax, r13
140769faf shr rax, 1
140769fb2 mov ecx, r13d
140769fb5 and ecx, 1
140769fb8 or rcx, rax
140769fbb xorps xmm0, xmm0
140769fbe cvtsi2ss xmm0, rcx
140769fc3 addss xmm0, xmm0
140769fc7 movss dword ptr [rbp + 0x32c], xmm0
140769fcf mov r10, qword ptr [rbp + 0x628]
140769fd6 mov rax, qword ptr [rbp + 0x108]
140769fdd mov qword ptr [rbp + 0x498], rax
140769fe4 movss xmm0, dword ptr [rip + 0x2273a00]
140769fec mov eax, dword ptr [rbp + 0x640]
140769ff2 test al, 1
140769ff4 je 0x140769ffa
140769ff6 mulss xmm0, xmm7
140769ffa movss dword ptr [rbp + 0x324], xmm0
14076a002 movaps xmm8, xmm9
14076a006 divss xmm8, dword ptr [rbp + 0x2a0]
14076a00f mov rax, qword ptr [rbp + 0x100]
14076a016 mov qword ptr [rbp + 0x458], rax
14076a01d mov rax, qword ptr [rbp + 0x498]
14076a024 shl rax, 5
14076a028 mov qword ptr [rbp + 0x148], rax
14076a02f imul rax, r13, 0x154df
14076a036 imul rcx, rbx, 0x159
14076a03d mov qword ptr [rbp + 0x150], rcx
14076a044 imul rcx, r13, 0x91e
14076a04b imul rdx, rbx, 0xdf
14076a052 mov qword ptr [rbp + 0x158], rdx
14076a059 imul rdx, r13, 0x15b3
14076a060 mov r9, rbx
14076a063 imul r9, rbx
14076a067 shl rsi, 4
14076a06b imul r8d, ebx, 0x924
14076a072 cmp byte ptr [rbp + 0x5f8], 2
14076a079 sete byte ptr [rbp + 0x4b5]
14076a080 movss xmm1, dword ptr [rip + 0x23142b8]
14076a088 xorps xmm15, xmm15
14076a08c mov r11d, dword ptr [rbp + 0x630]
14076a093 test r11b, 1
14076a097 jne 0x14076a0a4
14076a099 xorps xmm0, xmm0
14076a09c movss dword ptr [rbp + 0x328], xmm0
14076a0a4 movaps xmmword ptr [rbp + 0x1a0], xmm10
14076a0ac divss xmm9, xmm10
14076a0b1 mov r11, qword ptr [rbp + 0x458]
14076a0b8 add qword ptr [rbp + 0x148], r11
14076a0bf add qword ptr [rbp + 0x150], rax
14076a0c6 add qword ptr [rbp + 0x158], rcx
14076a0cd lea rax, [rdx + r9]
14076a0d1 add rax, 0x2a6
14076a0d7 mov qword ptr [rbp + 0x88], rax
14076a0de add rsi, qword ptr [rbp + 0x160]
14076a0e5 movss xmm10, dword ptr [rip + 0x21bb782]
14076a0ee mulss xmm10, xmm8
14076a0f3 lea eax, [r8 + r13]
14076a0f7 add eax, 0xa
14076a0fa mov dword ptr [rbp + 0x244], eax
14076a100 movss xmm0, dword ptr [r10]
14076a105 movss dword ptr [rbp + 0x248], xmm0
14076a10d movss xmm0, dword ptr [r10 + 4]
14076a113 movss dword ptr [rbp + 0x234], xmm0
14076a11b movaps xmm2, xmmword ptr [rbp + 0x2a0]
14076a122 divss xmm1, xmm2
14076a126 movss dword ptr [rbp + 0x250], xmm1
14076a12e mov rax, qword ptr [rbp + 0x408]
14076a135 mov rdx, qword ptr [rax + 8]
14076a139 mov rax, qword ptr [rax + 0x20]
14076a13d mov qword ptr [rbp + 0x420], rax
14076a144 mov rcx, qword ptr [rbp + 0x5d0]
14076a14b mov qword ptr [rbp + 0x400], rdx
14076a152 lea rax, [rdx + rcx*8]
14076a156 mov qword ptr [rbp + 0x3f8], rax
14076a15d movss xmm0, dword ptr [r10 + 8]
14076a163 movss dword ptr [rbp + 0x238], xmm0
14076a16b movss xmm0, dword ptr [r10 + 0x10]
14076a171 movss dword ptr [rbp + 0x23c], xmm0
14076a179 movss xmm0, dword ptr [r10 + 0xc]
14076a17f movss dword ptr [rbp + 0x240], xmm0
14076a187 movss xmm0, dword ptr [rip + 0x23141b5]
14076a18f cmpltps xmm0, xmm2
14076a193 movss xmm1, dword ptr [rip + 0x2262e95]
14076a19b cmpltps xmm1, xmm6
14076a19f andps xmm1, xmm0
14076a1a2 movd eax, xmm1
14076a1a6 and byte ptr [rbp + 0x4b5], al
14076a1ac movabs rax, 0x7ffffffe8
14076a1b6 lea rax, [rax + rcx*8]
14076a1ba shr rax, 3
14076a1be inc eax
14076a1c0 mov qword ptr [rbp + 0x98], rax
14076a1c7 movsldup xmm0, xmm2
14076a1cb movaps xmmword ptr [rbp + 0x10], xmm0
14076a1cf lea rax, [rcx*8 - 0x10]
14076a1d7 mov qword ptr [rbp + 0x90], rax
14076a1de xor r15d, r15d
14076a1e1 mov r13, r11
14076a1e4 movss dword ptr [rbp + 0x478], xmm10
14076a1ed mov rax, qword ptr [rbp + 0x498]
14076a1f4 dec rax
14076a1f7 mov qword ptr [rbp + 0x4a0], rax
14076a1fe mov qword ptr [rbp + 0x460], r15
14076a205 cmp r13, qword ptr [rbp + 0x148]
14076a20c je 0x14076c513
14076a212 nop word ptr cs:[rax + rax]
14076a220 mov rcx, qword ptr [rbp + 0x460]
14076a227 imul rax, rcx, 0x96a3dd
14076a22e add rax, qword ptr [rbp + 0x150]
14076a235 mov qword ptr [rbp + 0x70], rax
14076a239 imul rax, rcx, 0x91fd
14076a240 add rax, qword ptr [rbp + 0x158]
14076a247 mov qword ptr [rbp + 0x68], rax
14076a24b mov rax, qword ptr [rbp + 0x88]
14076a252 add rax, rcx
14076a255 mov qword ptr [rbp + 0x60], rax
14076a259 lea rcx, [rbp + 0x78]
14076a25d call 0x140caf380
14076a262 lea rax, [r13 + 0x20]
14076a266 mov qword ptr [rbp + 0x130], rax
14076a26d mov rax, qword ptr [rbp + 0x460]
14076a274 inc rax
14076a277 addss xmm0, xmm0
14076a27b addss xmm0, dword ptr [rip + 0x21efe61]
14076a283 mulss xmm0, dword ptr [rbp + 0x610]
14076a28b movss dword ptr [rbp + 0x228], xmm0
14076a293 mov qword ptr [rbp + 0x350], 0
14076a29e mov r14, qword ptr [rbp + 0x160]
14076a2a5 jmp 0x14076a2cc
14076a2a7 nop word ptr [rax + rax]
14076a2b0 mov rax, qword ptr [rbp + 0x348]
14076a2b7 mov qword ptr [rax + rdi*8], r14
14076a2bb inc rdi
14076a2be mov qword ptr [rbp + 0x350], rdi
14076a2c5 mov rax, r15
14076a2c8 add r14, 0x10
14076a2cc mov r15, rax
14076a2cf cmp r14, rsi
14076a2d2 je 0x14076a340
14076a2d4 lea rcx, [rbp + 0x2d0]
14076a2db mov rdx, r14
14076a2de movaps xmm2, xmm8
14076a2e2 movaps xmm3, xmm9
14076a2e6 call 0x141240300
14076a2eb movups xmm0, xmmword ptr [r13 + 0x10]
14076a2f0 shufps xmm0, xmm0, 0x72
14076a2f4 movaps xmmword ptr [rbp + 0x3a0], xmm0
14076a2fb lea rcx, [rbp + 0x3a0]
14076a302 lea rdx, [rbp + 0x2d0]
14076a309 call 0x141240170
14076a30e test al, al
14076a310 mov rax, r15
14076a313 je 0x14076a2c8
14076a315 mov rdi, qword ptr [rbp + 0x350]
14076a31c cmp rdi, qword ptr [rbp + 0x340]
14076a323 jne 0x14076a2b0
14076a325 lea rcx, [rbp + 0x340]
14076a32c lea rdx, [rip + 0x23143e5]
14076a333 call 0x140562870
14076a338 jmp 0x14076a2b0
14076a33d nop dword ptr [rax]
14076a340 mov r14, qword ptr [rbp + 0x350]
14076a347 mov r12, qword ptr [rbp + 0x2c0]
14076a34e mov rbx, r14
14076a351 sub rbx, r12
14076a354 jbe 0x14076a3e0
14076a35a mov rax, qword ptr [rbp + 0x2b0]
14076a361 sub rax, r12
14076a364 cmp rax, rbx
14076a367 mov rax, r12
14076a36a movss xmm10, dword ptr [rbp + 0x478]
14076a373 jb 0x14076a4db
14076a379 mov rdi, qword ptr [rbp + 0x2b8]
14076a380 lea rcx, [rdi + rax*4]
14076a384 cmp rbx, 2
14076a388 jb 0x14076a3cc
14076a38a mov r8, r12
14076a38d not r8
14076a390 add r8, r14
14076a393 shl r8, 2
14076a397 xor edx, edx
14076a399 mov qword ptr [rbp + 0x4a8], r13
14076a3a0 mov r13, rsi
14076a3a3 mov rsi, rax
14076a3a6 call 0x142923c20
14076a3ab mov rax, rsi
14076a3ae mov rsi, r13
14076a3b1 mov r13, qword ptr [rbp + 0x4a8]
14076a3b8 add r14, rax
14076a3bb sub r14, r12
14076a3be lea rcx, [rdi + r14*4]
14076a3c2 add rcx, -4
14076a3c6 add rax, rbx
14076a3c9 dec rax
14076a3cc mov dword ptr [rcx], 0
14076a3d2 inc rax
14076a3d5 mov r14, qword ptr [rbp + 0x350]
14076a3dc jmp 0x14076a3ec
14076a3de nop 
14076a3e0 mov rax, r14
14076a3e3 movss xmm10, dword ptr [rbp + 0x478]
14076a3ec mov qword ptr [rbp + 0x2c0], rax
14076a3f3 mov rcx, qword ptr [rbp + 0x2b8]
14076a3fa mov qword ptr [rbp + 0x4a8], rcx
14076a401 mov qword ptr [rbp + 0x450], rax
14076a408 test rax, rax
14076a40b sete al
14076a40e test r14, r14
14076a411 sete cl
14076a414 or cl, al
14076a416 jne 0x14076a498
14076a41c mov rax, qword ptr [rbp + 0x348]
14076a423 lea rcx, [r14*8 - 8]
14076a42b mov rdx, qword ptr [rbp + 0x450]
14076a432 lea rdx, [rdx*4 - 4]
14076a43a movss xmm1, dword ptr [rip + 0x21bb53e]
14076a442 movaps xmm0, xmm1
14076a445 nop word ptr cs:[rax + rax]
14076a450 mov r8, qword ptr [rax + rcx]
14076a454 movss xmm2, dword ptr [r8]
14076a459 subss xmm2, xmm10
14076a45e cmpunordss xmm0, xmm0
14076a463 movaps xmm3, xmm0
14076a466 andps xmm3, xmm2
14076a469 minss xmm2, xmm1
14076a46d andnps xmm0, xmm2
14076a470 orps xmm0, xmm3
14076a473 mov r8, qword ptr [rbp + 0x4a8]
14076a47a movss dword ptr [r8 + rdx], xmm0
14076a480 test rcx, rcx
14076a483 je 0x14076a498
14076a485 add rcx, -8
14076a489 lea r8, [rdx - 4]
14076a48d movaps xmm1, xmm0
14076a490 test rdx, rdx
14076a493 mov rdx, r8
14076a496 jne 0x14076a450
14076a498 movsd xmm7, qword ptr [r13 + 0x18]
14076a49e movshdup xmm11, xmm7
14076a4a3 movaps xmm6, xmm11
14076a4a7 subss xmm6, xmm7
14076a4ab mulss xmm6, dword ptr [rbp + 0x2a0]
14076a4b3 ucomiss xmm6, xmm15
14076a4b7 jne 0x14076a530
14076a4b9 jp 0x14076a530
14076a4bb mov qword ptr [rbp + 0x460], r15
14076a4c2 mov r13, qword ptr [rbp + 0x130]
14076a4c9 cmp r13, qword ptr [rbp + 0x148]
14076a4d0 jne 0x14076a220
14076a4d6 jmp 0x14076c513
14076a4db mov qword ptr [rsp + 0x20], 4
14076a4e4 mov r9d, 4
14076a4ea lea rcx, [rbp + 0x2b0]
14076a4f1 mov rdx, r12
14076a4f4 mov r8, rbx
14076a4f7 call 0x1428a7de0
14076a4fc mov rax, qword ptr [rbp + 0x2c0]
14076a503 movss xmm10, dword ptr [rbp + 0x478]
14076a50c mov rdi, qword ptr [rbp + 0x2b8]
14076a513 lea rcx, [rdi + rax*4]
14076a517 cmp rbx, 2
14076a51b jae 0x14076a38a
14076a521 jmp 0x14076a3cc
14076a526 nop word ptr cs:[rax + rax]
14076a530 movss xmm0, dword ptr [r13 + 0x10]
14076a536 movss dword ptr [rbp + 0x490], xmm0
14076a53e movss xmm0, dword ptr [r13 + 0x14]
14076a544 movss dword ptr [rbp + 0x418], xmm0
14076a54c mov rax, qword ptr [rbp + 0x4a0]
14076a553 cmp qword ptr [rbp + 0x460], rax
14076a55a setne byte ptr [rbp + 0x4a0]
14076a561 cmp byte ptr [rbp + 0x5f8], 1
14076a568 setne byte ptr [rbp + 0x4b7]
14076a56f mov qword ptr [rbp + 0x318], 0
14076a57a movzx eax, byte ptr [rip + 0x2eca688]
14076a581 mov ecx, 4
14076a586 mov edx, 4
14076a58b call 0x140613c10
14076a590 test rax, rax
14076a593 je 0x14076c8de
14076a599 mov dword ptr [rax], 0
14076a59f mov qword ptr [rbp + 0x380], 1
14076a5aa mov qword ptr [rbp + 0x388], rax
14076a5b1 mov qword ptr [rbp + 0x390], 1
14076a5bc mov edi, 1
14076a5c1 cmp byte ptr [rbp + 0x4b6], 2
14076a5c8 je 0x14076a712
14076a5ce mov rcx, qword ptr [rbp + 0x628]
14076a5d5 mov rax, qword ptr [rcx + 0x10]
14076a5d9 mov qword ptr [rbp + 0x2e0], rax
14076a5e0 movups xmm0, xmmword ptr [rcx]
14076a5e3 movaps xmmword ptr [rbp + 0x2d0], xmm0
14076a5ea mov qword ptr [rbp + 0x3c0], 0
14076a5f5 mov ebx, 4
14076a5fa xor r14d, r14d
14076a5fd mov rdx, qword ptr [rbp + 0x3b0]
14076a604 mov rcx, qword ptr [rbp + 0x3a0]
14076a60b mov r8, qword ptr [rbp + 0x3a8]
14076a612 mov rax, qword ptr [rbp + 0x3e0]
14076a619 nop dword ptr [rax]
14076a620 test r14b, 1
14076a624 je 0x14076a62b
14076a626 cmp rdx, r8
14076a629 jne 0x14076a66a
14076a62b cmp rbx, 0x14
14076a62f je 0x14076a709
14076a635 lea rax, [rbx + rbp]
14076a639 add rax, 0x2d8
14076a63f mov rcx, qword ptr [rbp + rbx + 0x2d0]
14076a647 add rbx, 8
14076a64b mov qword ptr [rbp + 0x3b8], rcx
14076a652 mov edx, 2
14076a657 mov ecx, 1
14076a65c mov r14d, 1
14076a662 xor r8d, r8d
14076a665 cmp rdx, r8
14076a668 je 0x14076a62b
14076a66a movss xmm2, dword ptr [rbp + r8*4 + 0x3b8]
14076a674 inc r8
14076a677 ucomiss xmm2, xmm15
14076a67b jne 0x14076a67f
14076a67d jnp 0x14076a620
14076a67f ucomiss xmm2, dword ptr [rip + 0x21bb27e]
14076a686 jne 0x14076a68a
14076a688 jnp 0x14076a620
14076a68a ucomiss xmm2, xmm7
14076a68d jb 0x14076a620
14076a68f ucomiss xmm11, xmm2
14076a693 jbe 0x14076a620
14076a695 mov qword ptr [rbp + 0x3b0], rdx
14076a69c mov qword ptr [rbp + 0x3a8], r8
14076a6a3 mov qword ptr [rbp + 0x3e0], rax
14076a6aa mov qword ptr [rbp + 0x3a0], rcx
14076a6b1 movaps xmm0, xmm7
14076a6b4 movaps xmm1, xmm11
14076a6b8 call 0x140c904a0
14076a6bd movaps xmm12, xmm0
14076a6c1 mov rax, qword ptr [rbp + 0x390]
14076a6c8 mov rdi, rax
14076a6cb cmp rax, qword ptr [rbp + 0x380]
14076a6d2 jne 0x14076a6e7
14076a6d4 lea rcx, [rbp + 0x380]
14076a6db lea rdx, [rip + 0x2313fa6]
14076a6e2 call 0x140650050
14076a6e7 mov rax, qword ptr [rbp + 0x388]
14076a6ee mov rcx, rdi
14076a6f1 movss dword ptr [rax + rdi*4], xmm12
14076a6f7 inc rcx
14076a6fa mov rdi, rcx
14076a6fd mov qword ptr [rbp + 0x390], rcx
14076a704 jmp 0x14076a5fd
14076a709 cmp rdi, qword ptr [rbp + 0x380]
14076a710 jne 0x14076a725
14076a712 lea rcx, [rbp + 0x380]
14076a719 lea rdx, [rip + 0x2313f80]
14076a720 call 0x140650050
14076a725 movzx eax, byte ptr [rbp + 0x4a0]
14076a72c or byte ptr [rbp + 0x4b7], al
14076a732 mov rax, qword ptr [rbp + 0x388]
14076a739 mov dword ptr [rax + rdi*4], 0x3f800000
14076a740 inc rdi
14076a743 mov qword ptr [rbp + 0x390], rdi
14076a74a jne 0x14076a75f
14076a74c xor r10d, r10d
14076a74f mov qword ptr [rbp + 0x288], 0
14076a75a jmp 0x14076abb6
14076a75f mov rax, qword ptr [rbp + 0x388]
14076a766 lea rdx, [rax + rdi*4]
14076a76a xor r10d, r10d
14076a76d mov qword ptr [rbp + 0x288], 0
14076a778 xor ecx, ecx
14076a77a mov qword ptr [rbp + 0x180], rdx
14076a781 mov r9, rax
14076a784 add rax, 4
14076a788 movss xmm11, dword ptr [r9]
14076a78d test cl, 1
14076a790 je 0x14076a7b0
14076a792 movaps xmm0, xmmword ptr [rbp + 0x1d0]
14076a799 unpcklps xmm0, xmm11
14076a79d movaps xmm11, xmm0
14076a7a1 jmp 0x14076a7cd
14076a7a3 nop word ptr cs:[rax + rax]
14076a7b0 cmp rax, rdx
14076a7b3 je 0x14076abb6
14076a7b9 mov r8, r9
14076a7bc mov rax, r9
14076a7bf movss xmm0, dword ptr [r8 + 4]
14076a7c5 add rax, 8
14076a7c9 unpcklps xmm11, xmm0
14076a7cd mov qword ptr [rbp + 0x3a0], 0
14076a7d8 mov qword ptr [rbp + 0x3a8], 4
14076a7e3 mov qword ptr [rbp + 0x3b0], 0
14076a7ee movshdup xmm0, xmm11
14076a7f3 movaps xmmword ptr [rbp + 0x1d0], xmm0
14076a7fa ucomiss xmm0, xmm11
14076a7fe jbe 0x14076a820
14076a800 mov qword ptr [rbp + 0x1f0], r10
14076a807 mov qword ptr [rbp + 0x4a0], rax
14076a80e movaps xmm1, xmm11
14076a812 jmp 0x14076a88c
14076a814 nop word ptr cs:[rax + rax]
14076a820 cmp rax, rdx
14076a823 je 0x14076abb6
14076a829 movaps xmm1, xmmword ptr [rbp + 0x1d0]
14076a830 movss xmm0, dword ptr [rax]
14076a834 mov qword ptr [rbp + 0x3a0], 0
14076a83f mov qword ptr [rbp + 0x3a8], 4
14076a84a mov qword ptr [rbp + 0x3b0], 0
14076a855 ucomiss xmm0, xmm1
14076a858 ja 0x14076a86b
14076a85a add rax, 4
14076a85e movaps xmm1, xmm0
14076a861 cmp rax, rdx
14076a864 jne 0x14076a830
14076a866 jmp 0x14076abaf
14076a86b mov qword ptr [rbp + 0x1f0], r10
14076a872 movaps xmm11, xmm1
14076a876 unpcklps xmm11, xmm0
14076a87a add rax, 4
14076a87e mov qword ptr [rbp + 0x4a0], rax
14076a885 movaps xmmword ptr [rbp + 0x1d0], xmm0
14076a88c movaps xmm12, xmmword ptr [rbp + 0x1d0]
14076a894 subss xmm12, xmm1
14076a899 mulss xmm12, xmm6
14076a89e movaps xmm0, xmm12
14076a8a2 divss xmm0, dword ptr [rip + 0x2235eba]
14076a8aa call 0x142923e60
14076a8af cvttss2si rax, xmm0
14076a8b4 mov rcx, rax
14076a8b7 sar rcx, 0x3f
14076a8bb movaps xmm1, xmm0
14076a8be subss xmm1, dword ptr [rip + 0x21ef816]
14076a8c6 cvttss2si rdx, xmm1
14076a8cb and rdx, rcx
14076a8ce or rdx, rax
14076a8d1 ucomiss xmm0, xmm15
14076a8d5 mov ecx, 0
14076a8da cmovae rcx, rdx
14076a8de ucomiss xmm0, dword ptr [rip + 0x21ef7fb]
14076a8e5 mov rax, 0xffffffffffffffff
14076a8ec cmova rcx, rax
14076a8f0 movss xmm1, dword ptr [rip + 0x2313a50]
14076a8f8 cmp byte ptr [rbp + 0x4b7], 0
14076a8ff jne 0x14076a963
14076a901 movaps xmm0, xmm12
14076a905 divss xmm0, dword ptr [rip + 0x2311f6f]
14076a90d call 0x142923e60
14076a912 cvttss2si rax, xmm0
14076a917 mov rcx, rax
14076a91a sar rcx, 0x3f
14076a91e movaps xmm1, xmm0
14076a921 subss xmm1, dword ptr [rip + 0x21ef7b3]
14076a929 cvttss2si rdx, xmm1
14076a92e and rdx, rcx
14076a931 or rdx, rax
14076a934 ucomiss xmm0, xmm15
14076a938 mov eax, 0
14076a93d cmovae rax, rdx
14076a941 ucomiss xmm0, dword ptr [rip + 0x21ef798]
14076a948 mov rcx, 0xffffffffffffffff
14076a94f cmova rax, rcx
14076a953 mov ecx, eax
14076a955 and ecx, 1
14076a958 add rcx, rax
14076a95b movss xmm1, dword ptr [rip + 0x21c640d]
14076a963 mov rdi, rcx
14076a966 inc rcx
14076a969 divss xmm1, xmm12
14076a96e lea rax, [rip + 0x2313d8b]
14076a975 mov qword ptr [rsp + 0x20], rax
14076a97a lea r8, [rbp + 0x70]
14076a97e lea r9, [rbp + 0x3a0]
14076a985 call 0x140c90050
14076a98a mov rbx, qword ptr [rbp + 0x3a8]
14076a991 mov r14, qword ptr [rbp + 0x3b0]
14076a998 mov rax, qword ptr [rbp + 0x308]
14076a99f mov r10, qword ptr [rbp + 0x1f0]
14076a9a6 sub rax, r10
14076a9a9 cmp rax, r14
14076a9ac jb 0x14076ab71
14076a9b2 test r14, r14
14076a9b5 je 0x14076ab20
14076a9bb mov rax, qword ptr [rbp + 0x310]
14076a9c2 cmp r14, 8
14076a9c6 jb 0x14076a9d9
14076a9c8 lea rcx, [rax + r10*4]
14076a9cc sub rcx, rbx
14076a9cf cmp rcx, 0x20
14076a9d3 jae 0x14076aa93
14076a9d9 xor ecx, ecx
14076a9db mov rdx, rcx
14076a9de or rdx, 1
14076a9e2 test r14b, 1
14076a9e6 je 0x14076aa14
14076a9e8 movss xmm0, dword ptr [rbx + rcx*4]
14076a9ed movss xmm1, dword ptr [rip + 0x21baf0f]
14076a9f5 subss xmm1, xmm0
14076a9f9 unpcklps xmm1, xmm0
14076a9fc mulps xmm1, xmm11
14076aa00 movshdup xmm0, xmm1
14076aa04 addss xmm0, xmm1
14076aa08 movss dword ptr [rax + r10*4], xmm0
14076aa0e inc r10
14076aa11 mov rcx, rdx
14076aa14 cmp r14, rdx
14076aa17 je 0x14076ab20
14076aa1d lea rax, [rax + r10*4]
14076aa21 add rax, 4
14076aa25 lea rdx, [rbx + rcx*4]
14076aa29 add rdx, 4
14076aa2d sub r14, rcx
14076aa30 xor ecx, ecx
14076aa32 movss xmm2, dword ptr [rip + 0x21baeca]
14076aa3a nop word ptr [rax + rax]
14076aa40 movss xmm0, dword ptr [rdx + rcx*4 - 4]
14076aa46 movaps xmm1, xmm2
14076aa49 subss xmm1, xmm0
14076aa4d unpcklps xmm1, xmm0
14076aa50 mulps xmm1, xmm11
14076aa54 movshdup xmm0, xmm1
14076aa58 addss xmm0, xmm1
14076aa5c movss dword ptr [rax + rcx*4 - 4], xmm0
14076aa62 movss xmm0, dword ptr [rdx + rcx*4]
14076aa67 movaps xmm1, xmm2
14076aa6a subss xmm1, xmm0
14076aa6e unpcklps xmm1, xmm0
14076aa71 mulps xmm1, xmm11
14076aa75 movshdup xmm0, xmm1
14076aa79 addss xmm0, xmm1
14076aa7d movss dword ptr [rax + rcx*4], xmm0
14076aa82 add rcx, 2
14076aa86 cmp r14, rcx
14076aa89 jne 0x14076aa40
14076aa8b add r10, rcx
14076aa8e jmp 0x14076ab20
14076aa93 lea rdx, [r10*4]
14076aa9b mov rcx, r14
14076aa9e and rcx, 0xfffffffffffffff8
14076aaa2 add r10, rcx
14076aaa5 movaps xmm0, xmm11
14076aaa9 shufps xmm0, xmm11, 0
14076aaae movaps xmm1, xmm11
14076aab2 shufps xmm1, xmm11, 0x55
14076aab7 add rdx, rax
14076aaba add rdx, 0x10
14076aabe xor r8d, r8d
14076aac1 movaps xmm12, xmmword ptr [rip + 0x21bae67]
14076aac9 nop dword ptr [rax]
14076aad0 movups xmm2, xmmword ptr [rbx + r8*4]
14076aad5 movups xmm3, xmmword ptr [rbx + r8*4 + 0x10]
14076aadb movaps xmm4, xmm12
14076aadf subps xmm4, xmm2
14076aae2 movaps xmm5, xmm12
14076aae6 subps xmm5, xmm3
14076aae9 mulps xmm4, xmm0
14076aaec mulps xmm5, xmm0
14076aaef mulps xmm2, xmm1
14076aaf2 addps xmm2, xmm4
14076aaf5 mulps xmm3, xmm1
14076aaf8 addps xmm3, xmm5
14076aafb movups xmmword ptr [rdx + r8*4 - 0x10], xmm2
14076ab01 movups xmmword ptr [rdx + r8*4], xmm3
14076ab06 add r8, 8
14076ab0a cmp rcx, r8
14076ab0d jne 0x14076aad0
14076ab0f cmp r14, rcx
14076ab12 jne 0x14076a9db
14076ab18 nop dword ptr [rax + rax]
14076ab20 mov qword ptr [rbp + 0x318], r10
14076ab27 mov rdx, qword ptr [rbp + 0x3a0]
14076ab2e test rdx, rdx
14076ab31 je 0x14076ab4f
14076ab33 mov rcx, qword ptr [rbp + 0x3a8]
14076ab3a shl rdx, 2
14076ab3e mov r8d, 4
14076ab44 mov rbx, r10
14076ab47 call 0x140613c20
14076ab4c mov r10, rbx
14076ab4f add qword ptr [rbp + 0x288], rdi
14076ab56 mov cl, 1
14076ab58 mov rdx, qword ptr [rbp + 0x180]
14076ab5f mov rax, qword ptr [rbp + 0x4a0]
14076ab66 cmp rax, rdx
14076ab69 jne 0x14076a781
14076ab6f jmp 0x14076abb6
14076ab71 mov qword ptr [rsp + 0x20], 4
14076ab7a mov r9d, 4
14076ab80 lea rcx, [rbp + 0x308]
14076ab87 mov rdx, r10
14076ab8a mov r8, r14
14076ab8d call 0x1428a7de0
14076ab92 mov rax, qword ptr [rbp + 0x310]
14076ab99 mov r10, qword ptr [rbp + 0x318]
14076aba0 cmp r14, 8
14076aba4 jb 0x14076a9d9
14076abaa jmp 0x14076a9c8
14076abaf movaps xmmword ptr [rbp + 0x1d0], xmm0
14076abb6 mov rbx, r10
14076abb9 mov rdx, qword ptr [rbp + 0x380]
14076abc0 test rdx, rdx
14076abc3 je 0x14076abdb
14076abc5 mov rcx, qword ptr [rbp + 0x388]
14076abcc shl rdx, 2
14076abd0 mov r8d, 4
14076abd6 call 0x140613c20
14076abdb mov rdi, qword ptr [rbp + 0x310]
14076abe2 mov rcx, rbx
14076abe5 lea rbx, [rdi + rbx*4]
14076abe9 test rcx, rcx
14076abec movss xmm2, dword ptr [rip + 0x21bad10]
14076abf4 je 0x14076acb6
14076abfa shl rcx, 2
14076abfe add rcx, -4
14076ac02 mov rax, rdi
14076ac05 cmp rcx, 0x1b
14076ac09 jbe 0x14076ac90
14076ac0f shr rcx, 2
14076ac13 inc rcx
14076ac16 mov r8, rcx
14076ac19 and rcx, 0xfffffffffffffff8
14076ac1d lea rax, [rdi + rcx*4]
14076ac21 movaps xmm0, xmm7
14076ac24 shufps xmm0, xmm7, 0
14076ac28 movaps xmm1, xmm7
14076ac2b shufps xmm1, xmm7, 0x55
14076ac2f xor edx, edx
14076ac31 movaps xmm6, xmmword ptr [rip + 0x21bacf8]
14076ac38 nop dword ptr [rax + rax]
14076ac40 movups xmm2, xmmword ptr [rdi + rdx*4]
14076ac44 movups xmm3, xmmword ptr [rdi + rdx*4 + 0x10]
14076ac49 movaps xmm4, xmm6
14076ac4c subps xmm4, xmm2
14076ac4f movaps xmm5, xmm6
14076ac52 subps xmm5, xmm3
14076ac55 mulps xmm4, xmm0
14076ac58 mulps xmm5, xmm0
14076ac5b mulps xmm2, xmm1
14076ac5e addps xmm2, xmm4
14076ac61 mulps xmm3, xmm1
14076ac64 addps xmm3, xmm5
14076ac67 movups xmmword ptr [rdi + rdx*4], xmm2
14076ac6b movups xmmword ptr [rdi + rdx*4 + 0x10], xmm3
14076ac70 add rdx, 8
14076ac74 cmp rcx, rdx
14076ac77 jne 0x14076ac40
14076ac79 cmp r8, rcx
14076ac7c movss xmm2, dword ptr [rip + 0x21bac80]
14076ac84 je 0x14076acb6
14076ac86 nop word ptr cs:[rax + rax]
14076ac90 movss xmm0, dword ptr [rax]
14076ac94 movaps xmm1, xmm2
14076ac97 subss xmm1, xmm0
14076ac9b unpcklps xmm1, xmm0
14076ac9e mulps xmm1, xmm7
14076aca1 movshdup xmm0, xmm1
14076aca5 addss xmm0, xmm1
14076aca9 movss dword ptr [rax], xmm0
14076acad add rax, 4
14076acb1 cmp rax, rbx
14076acb4 jne 0x14076ac90
14076acb6 movaps xmm6, xmmword ptr [rbp + 0x1a0]
14076acbd movaps xmm0, xmm6
14076acc0 mulss xmm0, dword ptr [rbp + 0x490]
14076acc8 movss xmm12, dword ptr [rip + 0x21c98d7]
14076acd1 mulss xmm0, xmm12
14076acd6 movss xmm11, dword ptr [rbp + 0x32c]
14076acdf addss xmm0, xmm11
14076ace4 movss xmm14, dword ptr [rip + 0x231365f]
14076aced mulss xmm0, xmm14
14076acf2 call 0x142923fd0
14076acf7 movaps xmm13, xmm0
14076acfb movss xmm15, dword ptr [rip + 0x231364c]
14076ad04 mulss xmm13, xmm15
14076ad09 movaps xmm0, xmm13
14076ad0d call 0x142924060
14076ad12 subss xmm13, xmm0
14076ad17 movaps xmm0, xmm6
14076ad1a mulss xmm0, dword ptr [rbp + 0x418]
14076ad22 movaps xmmword ptr [rbp + 0xe0], xmm0
14076ad29 mulss xmm0, xmm12
14076ad2e addss xmm0, xmm11
14076ad33 mulss xmm0, xmm14
14076ad38 call 0x142923fd0
14076ad3d movaps xmm6, xmm0
14076ad40 mulss xmm6, xmm15
14076ad45 movaps xmm0, xmm6
14076ad48 call 0x142924060
14076ad4d subss xmm6, xmm0
14076ad51 movss dword ptr [rbp + 0x330], xmm13
14076ad5a addss xmm13, xmm13
14076ad5f mulss xmm13, xmm14
14076ad64 movaps xmm0, xmm13
14076ad68 call 0x142923fd0
14076ad6d mulss xmm0, xmm15
14076ad72 movaps xmm12, xmm0
14076ad76 call 0x142924060
14076ad7b movaps xmm11, xmm0
14076ad7f movss dword ptr [rbp + 0x334], xmm6
14076ad87 addss xmm6, xmm6
14076ad8b mulss xmm6, xmm14
14076ad90 movaps xmm0, xmm6
14076ad93 call 0x142923fd0
14076ad98 mulss xmm0, xmm15
14076ad9d movaps xmm6, xmm0
14076ada0 call 0x142924060
14076ada5 xorps xmm5, xmm5
14076ada8 ucomiss xmm7, xmm5
14076adab jb 0x14076c85e
14076adb1 mov qword ptr [rbp + 0x140], rbx
14076adb8 mov qword ptr [rbp + 0x168], rdi
14076adbf mov rax, qword ptr [rbp + 0x420]
14076adc6 movss xmm4, dword ptr [rax + 4]
14076adcb movaps xmm2, xmm4
14076adce subss xmm2, dword ptr [rax]
14076add2 andps xmm2, xmmword ptr [rip + 0x21c3ba7]
14076add9 movaps xmm1, xmm2
14076addc subss xmm1, xmm7
14076ade0 xor r14d, r14d
14076ade3 ucomiss xmm1, xmm5
14076ade6 jae 0x14076ae80
14076adec movss xmm2, dword ptr [rip + 0x21bab10]
14076adf4 movaps xmm3, xmm2
14076adf7 cmp qword ptr [rbp + 0x5d0], 2
14076adff je 0x14076ae93
14076ae05 mov rax, qword ptr [rbp + 0x420]
14076ae0c movss xmm3, dword ptr [rax + 8]
14076ae11 movaps xmm2, xmm3
14076ae14 subss xmm2, xmm4
14076ae18 andps xmm2, xmmword ptr [rip + 0x21c3b61]
14076ae1f addss xmm1, xmm2
14076ae23 ucomiss xmm1, xmm5
14076ae26 jae 0x14076ae7a
14076ae28 mov r14d, 3
14076ae2e mov rax, qword ptr [rbp + 0x90]
14076ae35 nop word ptr cs:[rax + rax]
14076ae40 add rax, -8
14076ae44 je 0x14076c4fc
14076ae4a mov rcx, qword ptr [rbp + 0x420]
14076ae51 movss xmm4, dword ptr [rcx + r14*4]
14076ae57 inc r14
14076ae5a movaps xmm2, xmm4
14076ae5d subss xmm2, xmm3
14076ae61 andps xmm2, xmmword ptr [rip + 0x21c3b18]
14076ae68 addss xmm1, xmm2
14076ae6c ucomiss xmm1, xmm5
14076ae6f movaps xmm3, xmm4
14076ae72 jb 0x14076ae40
14076ae74 add r14, -2
14076ae78 jmp 0x14076ae80
14076ae7a mov r14d, 1
14076ae80 divss xmm1, xmm2
14076ae84 movss xmm2, dword ptr [rip + 0x21baa78]
14076ae8c movaps xmm3, xmm2
14076ae8f subss xmm3, xmm1
14076ae93 movss xmm1, dword ptr [rbp + 0x418]
14076ae9b subss xmm1, dword ptr [rbp + 0x490]
14076aea3 mulss xmm1, dword ptr [rbp + 0x1a0]
14076aeab movss dword ptr [rbp + 0x24c], xmm1
14076aeb3 subss xmm12, xmm11
14076aeb8 movss xmm1, dword ptr [rip + 0x21f0770]
14076aec0 mulss xmm12, xmm1
14076aec5 movss dword ptr [rbp + 0x338], xmm12
14076aece subss xmm6, xmm0
14076aed2 mulss xmm6, xmm1
14076aed6 movss dword ptr [rbp + 0x33c], xmm6
14076aede mov rax, qword ptr [rbp + 0x348]
14076aee5 mov qword ptr [rbp + 0x370], rax
14076aeec mov rax, qword ptr [rbp + 0x350]
14076aef3 mov qword ptr [rbp + 0x1c0], rax
14076aefa mov eax, r14d
14076aefd movaps xmm0, xmm2
14076af00 subss xmm0, xmm3
14076af04 mov rcx, qword ptr [rbp + 0x5e8]
14076af0b movsd xmm1, qword ptr [rcx + rax*4]
14076af10 unpcklps xmm0, xmm3
14076af13 mulps xmm1, xmm0
14076af16 movshdup xmm2, xmm1
14076af1a addss xmm2, xmm1
14076af1e movaps xmmword ptr [rbp + 0x1f0], xmm2
14076af25 mov rcx, qword ptr [rbp + 0x5d8]
14076af2c movsd xmm1, qword ptr [rcx + rax*4]
14076af31 mulps xmm1, xmm0
14076af34 movshdup xmm2, xmm1
14076af38 addss xmm2, xmm1
14076af3c movaps xmmword ptr [rbp + 0x180], xmm2
14076af43 mov rcx, qword ptr [rbp + 0x358]
14076af4a movsd xmm1, qword ptr [rcx + rax*4]
14076af4f mulps xmm1, xmm0
14076af52 movshdup xmm0, xmm1
14076af56 addss xmm0, xmm1
14076af5a movaps xmmword ptr [rbp + 0x190], xmm0
14076af61 imul eax, dword ptr [rbp + 0x460], 0x54
14076af68 add eax, dword ptr [rbp + 0x244]
14076af6e mov dword ptr [rbp + 0x230], eax
14076af74 movzx eax, byte ptr [rbp + 0x4b7]
14076af7b movd xmm0, eax
14076af7f pshufd xmm0, xmm0, 0x44
14076af84 mov qword ptr [rbp + 0x360], 0
14076af8f pshufd xmm0, xmm0, 0xe8
14076af94 pslld xmm0, 0x1f
14076af99 psrad xmm0, 0x1f
14076af9e movdqa xmmword ptr [rbp + 0x20], xmm0
14076afa3 mov r11, qword ptr [rbp + 0x168]
14076afaa mov qword ptr [rbp + 0x278], 0
14076afb5 mov qword ptr [rbp + 0x378], 0
14076afc0 movaps xmm11, xmm3
14076afc4 mov rax, qword ptr [rbp + 0x458]
14076afcb lea rdi, [rax + 0x1c]
14076afcf jmp 0x14076afec
14076afd1 nop word ptr cs:[rax + rax]
14076afe0 mov r11, qword ptr [rbp + 0x138]
14076afe7 movaps xmm11, xmmword ptr [rbp + 0x30]
14076afec movaps xmm0, xmmword ptr [rbp + 0x1f0]
14076aff3 movaps xmmword ptr [rbp + 0x460], xmm0
14076affa movaps xmm13, xmmword ptr [rbp + 0x180]
14076b002 movaps xmm0, xmmword ptr [rbp + 0x190]
14076b009 mov rax, qword ptr [rbp + 0x140]
14076b010 cmp r11, rax
14076b013 je 0x14076b040
14076b015 lea rdx, [r11 + 4]
14076b019 mov r8, qword ptr [rbp + 0x360]
14076b020 lea rax, [r8 + 1]
14076b024 cmp qword ptr [rbp + 0x278], 0
14076b02c je 0x14076b070
14076b02e xorps xmm15, xmm15
14076b032 mov rcx, rax
14076b035 mov rax, r8
14076b038 mov r8, r11
14076b03b jmp 0x14076b0e3
14076b040 xorps xmm15, xmm15
14076b044 mov r11, rax
14076b047 lea rax, [rbp + 0x260]
14076b04e xor r9d, r9d
14076b051 mov qword ptr [rax], r9
14076b054 mov rax, qword ptr [rbp + 0x260]
14076b05b test rax, rax
14076b05e jne 0x14076b138
14076b064 jmp 0x14076c4f0
14076b069 nop dword ptr [rax]
14076b070 mov rcx, qword ptr [rbp + 0x140]
14076b077 cmp rdx, rcx
14076b07a je 0x14076b09b
14076b07c mov rcx, qword ptr [rbp + 0x360]
14076b083 add rcx, 2
14076b087 mov qword ptr [rbp + 0x3a0], r11
14076b08e add r11, 8
14076b092 lea r8, [rbp + 0x2d0]
14076b099 jmp 0x14076b0aa
14076b09b mov r11, rcx
14076b09e mov rcx, rax
14076b0a1 lea r8, [rbp + 0x3a0]
14076b0a8 xor edx, edx
14076b0aa mov qword ptr [r8], rdx
14076b0ad mov rdx, qword ptr [rbp + 0x3a0]
14076b0b4 mov r8, qword ptr [rbp + 0x2d0]
14076b0bb test rdx, rdx
14076b0be je 0x14076b605
14076b0c4 mov qword ptr [rbp + 0x268], rdx
14076b0cb xorps xmm15, xmm15
14076b0cf mov rdx, r11
14076b0d2 mov r9, qword ptr [rbp + 0x360]
14076b0d9 mov qword ptr [rbp + 0x270], r9
14076b0e0 mov r11, r8
14076b0e3 mov r9, r11
14076b0e6 mov r10, qword ptr [rbp + 0x270]
14076b0ed mov qword ptr [rbp + 0x280], r10
14076b0f4 mov r10, qword ptr [rbp + 0x268]
14076b0fb mov qword ptr [rbp + 0x260], r10
14076b102 mov r11, rdx
14076b105 mov qword ptr [rbp + 0x360], rcx
14076b10c mov qword ptr [rbp + 0x278], r10
14076b113 mov qword ptr [rbp + 0x270], rax
14076b11a mov qword ptr [rbp + 0x268], r8
14076b121 lea rax, [rbp + 0x58]
14076b125 mov qword ptr [rax], r9
14076b128 mov rax, qword ptr [rbp + 0x260]
14076b12f test rax, rax
14076b132 je 0x14076c4f0
14076b138 movss dword ptr [rbp + 0x47c], xmm0
14076b140 mov qword ptr [rbp + 0x138], r11
14076b147 movss xmm12, dword ptr [rax]
14076b14c mov rax, qword ptr [rbp + 0x58]
14076b150 movss xmm0, dword ptr [rax]
14076b154 movss dword ptr [rbp + 0x4a0], xmm0
14076b15c lea rcx, [rbp + 0x68]
14076b160 call 0x140caf380
14076b165 movaps xmm14, xmm0
14076b169 lea rcx, [rbp + 0x60]
14076b16d call 0x140caf380
14076b172 movaps xmm15, xmm0
14076b176 addss xmm15, dword ptr [rip + 0x21eef69]
14076b17f mulss xmm15, dword ptr [rip + 0x23131cc]
14076b188 addss xmm15, dword ptr [rbp + 0x324]
14076b191 maxss xmm15, dword ptr [rip + 0x21c5bd6]
14076b19a movss xmm7, dword ptr [rbp + 0x4a0]
14076b1a2 subss xmm7, xmm12
14076b1a7 movaps xmm1, xmm7
14076b1aa mulss xmm1, dword ptr [rip + 0x21ba6be]
14076b1b2 xorps xmm6, xmm6
14076b1b5 ucomiss xmm1, xmm6
14076b1b8 jb 0x14076c733
14076b1be mov eax, r14d
14076b1c1 mov rcx, qword ptr [rbp + 0x400]
14076b1c8 test r14d, r14d
14076b1cb jne 0x14076b6a5
14076b1d1 lea rdx, [rcx + 8]
14076b1d5 cmp rdx, qword ptr [rbp + 0x3f8]
14076b1dc je 0x14076b6d0
14076b1e2 mov rdx, qword ptr [rbp + 0x420]
14076b1e9 movss xmm0, dword ptr [rdx + rax*4 + 4]
14076b1ef movaps xmm3, xmm0
14076b1f2 subss xmm3, dword ptr [rdx + rax*4]
14076b1f7 andps xmm3, xmmword ptr [rip + 0x21c3782]
14076b1fe movss xmm2, dword ptr [rip + 0x21ba6fe]
14076b206 subss xmm2, xmm11
14076b20b mulss xmm2, xmm3
14076b20f subss xmm2, xmm1
14076b213 ucomiss xmm2, xmm6
14076b216 jae 0x14076b288
14076b218 add rcx, 0x10
14076b21c cmp rcx, qword ptr [rbp + 0x3f8]
14076b223 je 0x14076b269
14076b225 add rax, 2
14076b229 nop dword ptr [rax]
14076b230 movaps xmm4, xmm0
14076b233 mov rdx, qword ptr [rbp + 0x420]
14076b23a movss xmm0, dword ptr [rdx + rax*4]
14076b23f movaps xmm3, xmm0
14076b242 subss xmm3, xmm4
14076b246 andps xmm3, xmmword ptr [rip + 0x21c3733]
14076b24d addss xmm2, xmm3
14076b251 ucomiss xmm2, xmm6
14076b254 jae 0x14076b285
14076b256 add rcx, 8
14076b25a inc r14d
14076b25d inc rax
14076b260 cmp rcx, qword ptr [rbp + 0x3f8]
14076b267 jne 0x14076b230
14076b269 movss xmm0, dword ptr [rip + 0x21ba693]
14076b271 mov eax, r14d
14076b274 mov rcx, qword ptr [rbp + 0x400]
14076b27b test r14d, r14d
14076b27e je 0x14076b2ab
14076b280 jmp 0x14076b6e7
14076b285 inc r14d
14076b288 divss xmm2, xmm3
14076b28c movss xmm0, dword ptr [rip + 0x21ba670]
14076b294 subss xmm0, xmm2
14076b298 mov eax, r14d
14076b29b mov rcx, qword ptr [rbp + 0x400]
14076b2a2 test r14d, r14d
14076b2a5 jne 0x14076b6e7
14076b2ab movss xmm2, dword ptr [rip + 0x21ba651]
14076b2b3 subss xmm2, xmm0
14076b2b7 lea rdx, [rcx + 8]
14076b2bb cmp rdx, qword ptr [rbp + 0x3f8]
14076b2c2 je 0x14076b360
14076b2c8 mov rdx, qword ptr [rbp + 0x420]
14076b2cf movss xmm3, dword ptr [rdx + rax*4 + 4]
14076b2d5 movaps xmm4, xmm3
14076b2d8 subss xmm4, dword ptr [rdx + rax*4]
14076b2dd andps xmm4, xmmword ptr [rip + 0x21c369c]
14076b2e4 movaps xmm11, xmm2
14076b2e8 mulss xmm2, xmm4
14076b2ec subss xmm2, xmm1
14076b2f0 ucomiss xmm2, xmm6
14076b2f3 jae 0x14076b374
14076b2f5 add rcx, 0x10
14076b2f9 cmp rcx, qword ptr [rbp + 0x3f8]
14076b300 je 0x14076b34a
14076b302 lea rdx, [rax + 2]
14076b306 nop word ptr cs:[rax + rax]
14076b310 movaps xmm1, xmm3
14076b313 mov r8, qword ptr [rbp + 0x420]
14076b31a movss xmm3, dword ptr [r8 + rdx*4]
14076b320 movaps xmm4, xmm3
14076b323 subss xmm4, xmm1
14076b327 andps xmm4, xmmword ptr [rip + 0x21c3652]
14076b32e addss xmm2, xmm4
14076b332 ucomiss xmm2, xmm6
14076b335 jae 0x14076b371
14076b337 add rcx, 8
14076b33b inc r14d
14076b33e inc rdx
14076b341 cmp rcx, qword ptr [rbp + 0x3f8]
14076b348 jne 0x14076b310
14076b34a movss xmm1, dword ptr [rip + 0x21ba5b2]
14076b352 movaps xmm4, xmm1
14076b355 jmp 0x14076b387
14076b357 nop word ptr [rax + rax]
14076b360 movaps xmm11, xmm2
14076b364 movaps xmm4, xmm0
14076b367 movss xmm1, dword ptr [rip + 0x21ba595]
14076b36f jmp 0x14076b387
14076b371 inc r14d
14076b374 divss xmm2, xmm4
14076b378 movss xmm1, dword ptr [rip + 0x21ba584]
14076b380 movaps xmm4, xmm1
14076b383 subss xmm4, xmm2
14076b387 mov ecx, r14d
14076b38a subss xmm1, xmm4
14076b38e mov rdx, qword ptr [rbp + 0x5e8]
14076b395 movss xmm2, dword ptr [rdx + rcx*4]
14076b39a mulss xmm2, xmm1
14076b39e movss xmm3, dword ptr [rdx + rcx*4 + 4]
14076b3a4 mulss xmm3, xmm4
14076b3a8 addss xmm3, xmm2
14076b3ac movaps xmmword ptr [rbp + 0x1f0], xmm3
14076b3b3 mov rdx, qword ptr [rbp + 0x5d8]
14076b3ba movsd xmm2, qword ptr [rdx + rcx*4]
14076b3bf movaps xmm3, xmm1
14076b3c2 unpcklps xmm3, xmm4
14076b3c5 mulps xmm3, xmm2
14076b3c8 movshdup xmm5, xmm3
14076b3cc addss xmm5, xmm3
14076b3d0 mov rdx, qword ptr [rbp + 0x358]
14076b3d7 mulss xmm1, dword ptr [rdx + rcx*4]
14076b3dc movss xmm2, dword ptr [rdx + rcx*4 + 4]
14076b3e2 movaps xmmword ptr [rbp + 0x30], xmm4
14076b3e6 mulss xmm2, xmm4
14076b3ea addss xmm2, xmm1
14076b3ee movaps xmmword ptr [rbp + 0x190], xmm2
14076b3f5 movaps xmm1, xmm13
14076b3f9 cmpunordss xmm1, xmm13
14076b3ff movaps xmm2, xmm1
14076b402 andps xmm2, xmm5
14076b405 movaps xmmword ptr [rbp + 0x180], xmm5
14076b40c movaps xmm3, xmm5
14076b40f minss xmm3, xmm13
14076b414 andnps xmm1, xmm3
14076b417 orps xmm1, xmm2
14076b41a ucomiss xmm1, dword ptr [rbp + 0xe0]
14076b421 movss xmm5, dword ptr [rbp + 0x4a0]
14076b429 ja 0x14076afe0
14076b42f movaps xmm4, xmm13
14076b433 movss dword ptr [rbp + 0x368], xmm14
14076b43c movaps xmm14, xmm12
14076b440 mov rcx, qword ptr [rbp + 0x5e8]
14076b447 movss xmm1, dword ptr [rcx + rax*4]
14076b44c mulss xmm1, xmm11
14076b451 movss xmm2, dword ptr [rcx + rax*4 + 4]
14076b457 mulss xmm2, xmm0
14076b45b addss xmm2, xmm1
14076b45f movaps xmm1, xmmword ptr [rbp + 0x460]
14076b466 addss xmm1, xmm2
14076b46a movaps xmm3, xmmword ptr [rbp + 0x1f0]
14076b471 addss xmm1, xmm3
14076b475 ucomiss xmm6, xmm1
14076b478 movaps xmm13, xmm3
14076b47c jb 0x14076b48d
14076b47e movss xmm3, dword ptr [rip + 0x2242eba]
14076b486 movaps xmm2, xmm3
14076b489 movaps xmm13, xmm3
14076b48d mov rcx, qword ptr [rbp + 0x5d8]
14076b494 movss xmm10, dword ptr [rcx + rax*4]
14076b49a mulss xmm10, xmm11
14076b49f movss xmm3, dword ptr [rcx + rax*4 + 4]
14076b4a5 mulss xmm3, xmm0
14076b4a9 addss xmm3, xmm10
14076b4ae addss xmm3, xmm2
14076b4b2 mov rcx, qword ptr [rbp + 0x358]
14076b4b9 mulss xmm11, dword ptr [rcx + rax*4]
14076b4bf mulss xmm0, dword ptr [rcx + rax*4 + 4]
14076b4c5 cmpnless xmm1, xmm6
14076b4ca movaps xmm10, xmmword ptr [rbp + 0x460]
14076b4d2 andps xmm10, xmm1
14076b4d6 movss xmm2, dword ptr [rip + 0x2242e62]
14076b4de andnps xmm1, xmm2
14076b4e1 orps xmm1, xmm10
14076b4e5 addss xmm4, xmm1
14076b4e9 addss xmm13, dword ptr [rbp + 0x180]
14076b4f2 ucomiss xmm13, dword ptr [rbp + 0xe0]
14076b4fa movss dword ptr [rbp + 0x460], xmm14
14076b503 movss xmm14, dword ptr [rbp + 0x368]
14076b50c jbe 0x14076b524
14076b50e ucomiss xmm4, dword ptr [rbp + 0xe0]
14076b515 jbe 0x14076b524
14076b517 ucomiss xmm3, dword ptr [rbp + 0xe0]
14076b51e ja 0x14076afe0
14076b524 addss xmm11, xmm0
14076b529 cmp byte ptr [rbp + 0x4b6], 2
14076b530 je 0x14076b56f
14076b532 movaps xmm1, xmm5
14076b535 movss xmm2, dword ptr [rip + 0x21ba333]
14076b53d mulss xmm1, xmm2
14076b541 movss xmm0, dword ptr [rbp + 0x460]
14076b549 mulss xmm0, xmm2
14076b54d addss xmm0, xmm1
14076b551 ucomiss xmm0, dword ptr [rbp + 0x234]
14076b558 jb 0x14076b668
14076b55e movss xmm1, dword ptr [rbp + 0x238]
14076b566 ucomiss xmm1, xmm0
14076b569 jbe 0x14076b668
14076b56f movaps xmm2, xmmword ptr [rbp + 0x190]
14076b576 imul eax, dword ptr [rbp + 0x280], 0x171
14076b580 add eax, dword ptr [rbp + 0x230]
14076b586 mov dword ptr [rbp + 0x480], eax
14076b58c mov rax, qword ptr [r13]
14076b590 mov rcx, qword ptr [rbp + 0x498]
14076b597 cmp rax, rcx
14076b59a cmova rcx, rax
14076b59e mov rdx, qword ptr [r13 + 8]
14076b5a2 mov r8, rax
14076b5a5 shl r8, 5
14076b5a9 add r8, rdi
14076b5ac nop dword ptr [rax]
14076b5b0 cmp rax, rdx
14076b5b3 setae bl
14076b5b6 jae 0x14076b62e
14076b5b8 cmp rax, qword ptr [rbp + 0x498]
14076b5bf jae 0x14076c8a3
14076b5c5 movss xmm1, dword ptr [r8 - 4]
14076b5cb subss xmm1, xmm8
14076b5d0 movss xmm0, dword ptr [r8]
14076b5d5 addss xmm0, xmm8
14076b5da movss xmm12, dword ptr [rbp + 0x460]
14076b5e3 ucomiss xmm12, xmm1
14076b5e7 jb 0x14076b5f2
14076b5e9 ucomiss xmm0, dword ptr [rbp + 0x460]
14076b5f0 jae 0x14076b649
14076b5f2 inc rax
14076b5f5 add r8, 0x20
14076b5f9 ucomiss xmm5, xmm1
14076b5fc jb 0x14076b5b0
14076b5fe ucomiss xmm0, xmm5
14076b601 jb 0x14076b5b0
14076b603 jmp 0x14076b649
14076b605 xorps xmm15, xmm15
14076b609 mov qword ptr [rbp + 0x360], rcx
14076b610 mov qword ptr [rbp + 0x278], 0
14076b61b mov qword ptr [rbp + 0x270], rax
14076b622 mov qword ptr [rbp + 0x268], r8
14076b629 jmp 0x14076b047
14076b62e movss xmm0, dword ptr [rip + 0x2235142]
14076b636 ucomiss xmm0, xmm14
14076b63a seta al
14076b63d test byte ptr [rbp + 0x4b5], al
14076b643 jne 0x14076afe0
14076b649 cmp byte ptr [rbp + 0x4b7], 0
14076b650 jne 0x14076b6f9
14076b656 test byte ptr [rbp + 0x280], 1
14076b65d je 0x14076afe0
14076b663 jmp 0x14076b6f7
14076b668 movss xmm1, dword ptr [rbp + 0x23c]
14076b670 ucomiss xmm1, xmm0
14076b673 jbe 0x14076b689
14076b675 ucomiss xmm0, dword ptr [rbp + 0x240]
14076b67c movaps xmm2, xmmword ptr [rbp + 0x190]
14076b683 jae 0x14076b576
14076b689 movss xmm0, dword ptr [rbp + 0x248]
14076b691 movaps xmm2, xmm0
14076b694 movaps xmm11, xmm0
14076b698 movss dword ptr [rbp + 0x47c], xmm0
14076b6a0 jmp 0x14076b576
14076b6a5 cmp rax, qword ptr [rbp + 0x5d0]
14076b6ac jae 0x14076b6d0
14076b6ae mov rcx, qword ptr [rbp + 0x400]
14076b6b5 lea rcx, [rcx + rax*8]
14076b6b9 lea rdx, [rcx + 8]
14076b6bd cmp rdx, qword ptr [rbp + 0x3f8]
14076b6c4 jne 0x14076b1e2
14076b6ca nop word ptr [rax + rax]
14076b6d0 movaps xmm0, xmm11
14076b6d4 mov eax, r14d
14076b6d7 mov rcx, qword ptr [rbp + 0x400]
14076b6de test r14d, r14d
14076b6e1 je 0x14076b2ab
14076b6e7 mov rcx, qword ptr [rbp + 0x400]
14076b6ee lea rcx, [rcx + rax*8]
14076b6f2 jmp 0x14076b2ab
14076b6f7 xor ebx, ebx
14076b6f9 movss dword ptr [rbp + 0x488], xmm2
14076b701 movss dword ptr [rbp + 0x484], xmm3
14076b709 movss dword ptr [rbp + 0x48c], xmm13
14076b712 movss dword ptr [rbp + 0x41c], xmm11
14076b71b movss dword ptr [rbp + 0x414], xmm4
14076b723 movss xmm0, dword ptr [rbp + 0x2c8]
14076b72b ucomiss xmm0, dword ptr [rbp + 0x460]
14076b732 setnp al
14076b735 sete cl
14076b738 and cl, al
14076b73a test byte ptr [rbp + 0x378], cl
14076b740 jne 0x14076b814
14076b746 movaps xmm13, xmmword ptr [rbp + 0x2a0]
14076b74e mulss xmm13, dword ptr [rbp + 0x460]
14076b757 movaps xmm0, xmm13
14076b75b mulss xmm0, dword ptr [rip + 0x2261905]
14076b763 call 0x142923fd0
14076b768 movaps xmm14, xmm0
14076b76c movss xmm0, dword ptr [rbp + 0x490]
14076b774 ucomiss xmm0, dword ptr [rip + 0x21f4ef9]
14076b77b movss xmm0, dword ptr [rip + 0x21ba0ed]
14076b783 mulss xmm14, xmm0
14076b788 addss xmm14, xmm0
14076b78d xorps xmm0, xmm0
14076b790 movaps xmmword ptr [rbp + 0x200], xmm0
14076b797 movss dword ptr [rbp + 0x494], xmm0
14076b79f jb 0x14076b7d8
14076b7a1 movss xmm0, dword ptr [rbp + 0x330]
14076b7a9 mulss xmm0, xmm14
14076b7ae addss xmm0, dword ptr [rip + 0x21ba14e]
14076b7b6 mulss xmm0, xmm13
14076b7bb addss xmm0, dword ptr [rbp + 0x338]
14076b7c3 call 0x142923fd0
14076b7c8 mulss xmm0, dword ptr [rip + 0x23110b8]
14076b7d0 movss dword ptr [rbp + 0x494], xmm0
14076b7d8 test bl, bl
14076b7da jne 0x14076b814
14076b7dc mulss xmm14, dword ptr [rbp + 0x334]
14076b7e5 addss xmm14, dword ptr [rip + 0x21ba116]
14076b7ee mulss xmm13, xmm14
14076b7f3 addss xmm13, dword ptr [rbp + 0x33c]
14076b7fc movaps xmm0, xmm13
14076b800 call 0x142923fd0
14076b805 mulss xmm0, dword ptr [rip + 0x231107b]
14076b80d movaps xmmword ptr [rbp + 0x200], xmm0
14076b814 movaps xmm13, xmmword ptr [rbp + 0x2a0]
14076b81c mulss xmm13, dword ptr [rbp + 0x4a0]
14076b825 movaps xmm0, xmm13
14076b829 mulss xmm0, dword ptr [rip + 0x2261837]
14076b831 call 0x142923fd0
14076b836 movaps xmm14, xmm0
14076b83a movss xmm0, dword ptr [rbp + 0x490]
14076b842 ucomiss xmm0, dword ptr [rip + 0x21f4e2b]
14076b849 movss xmm0, dword ptr [rip + 0x21ba01f]
14076b851 mulss xmm14, xmm0
14076b856 addss xmm14, xmm0
14076b85b xorps xmm0, xmm0
14076b85e movaps xmmword ptr [rbp + 0xb0], xmm0
14076b865 movaps xmmword ptr [rbp + 0x1e0], xmm0
14076b86c jb 0x14076b8a4
14076b86e movss xmm0, dword ptr [rbp + 0x330]
14076b876 mulss xmm0, xmm14
14076b87b addss xmm0, dword ptr [rip + 0x21ba081]
14076b883 mulss xmm0, xmm13
14076b888 addss xmm0, dword ptr [rbp + 0x338]
14076b890 call 0x142923fd0
14076b895 mulss xmm0, dword ptr [rip + 0x2310feb]
14076b89d movaps xmmword ptr [rbp + 0x1e0], xmm0
14076b8a4 test bl, bl
14076b8a6 jne 0x14076b8e0
14076b8a8 mulss xmm14, dword ptr [rbp + 0x334]
14076b8b1 addss xmm14, dword ptr [rip + 0x21ba04a]
14076b8ba mulss xmm13, xmm14
14076b8bf addss xmm13, dword ptr [rbp + 0x33c]
14076b8c8 movaps xmm0, xmm13
14076b8cc call 0x142923fd0
14076b8d1 mulss xmm0, dword ptr [rip + 0x2310faf]
14076b8d9 movaps xmmword ptr [rbp + 0xb0], xmm0
14076b8e0 movss xmm0, dword ptr [rbp + 0x610]
14076b8e8 ucomiss xmm0, xmm6
14076b8eb movss xmm0, dword ptr [rbp + 0x460]
14076b8f3 movss dword ptr [rbp + 0x218], xmm0
14076b8fb movss xmm0, dword ptr [rbp + 0x490]
14076b903 movss dword ptr [rbp + 0x21c], xmm0
14076b90b movss xmm0, dword ptr [rbp + 0x4a0]
14076b913 movss dword ptr [rbp + 0x220], xmm0
14076b91b movss xmm0, dword ptr [rbp + 0x418]
14076b923 movss dword ptr [rbp + 0x224], xmm0
14076b92b movss xmm6, dword ptr [rbp + 0x368]
14076b933 jbe 0x14076b97e
14076b935 mov r9, qword ptr [rbp + 0x318]
14076b93c movss xmm0, dword ptr [rbp + 0x228]
14076b944 movss dword ptr [rsp + 0x30], xmm0
14076b94a movss xmm0, dword ptr [rbp + 0x250]
14076b952 movss dword ptr [rsp + 0x28], xmm0
14076b958 mov rax, qword ptr [rbp + 0x288]
14076b95f mov qword ptr [rsp + 0x20], rax
14076b964 lea rcx, [rbp + 0x218]
14076b96b mov rdx, qword ptr [rbp + 0x280]
14076b972 mov r8, qword ptr [rbp + 0x168]
14076b979 call 0x141240d30
14076b97e cmp qword ptr [rbp + 0x440], 0
14076b986 je 0x14076b993
14076b988 mov qword ptr [rbp + 0x440], 0
14076b993 mov qword ptr [rbp + 0x438], 0
14076b99e movups xmm0, xmmword ptr [rbp + 0x218]
14076b9a5 movaps xmmword ptr [rbp + 0x3a0], xmm0
14076b9ac mov rax, qword ptr [rbp + 0x428]
14076b9b3 test rax, rax
14076b9b6 je 0x14076b9bc
14076b9b8 xor ecx, ecx
14076b9ba jmp 0x14076b9e4
14076b9bc lea rcx, [rbp + 0x428]
14076b9c3 lea rdx, [rip + 0x2312d06]
14076b9ca call 0x141241120
14076b9cf mov rax, qword ptr [rbp + 0x428]
14076b9d6 mov rcx, qword ptr [rbp + 0x438]
14076b9dd add rcx, qword ptr [rbp + 0x440]
14076b9e4 movaps xmm2, xmmword ptr [rbp + 0x200]
14076b9eb unpcklps xmm2, xmmword ptr [rbp + 0xb0]
14076b9f2 movaps xmm0, xmm2
14076b9f5 movaps xmm1, xmmword ptr [rip + 0x21c5364]
14076b9fc mulps xmm0, xmm1
14076b9ff addps xmm0, xmm1
14076ba02 movaps xmm13, xmmword ptr [rbp + 0x20]
14076ba07 andps xmm2, xmm13
14076ba0b andnps xmm13, xmm0
14076ba0f orps xmm13, xmm2
14076ba13 cmp rcx, rax
14076ba16 mov edx, 0
14076ba1b cmovae rdx, rax
14076ba1f sub rcx, rdx
14076ba22 mov rax, qword ptr [rbp + 0x430]
14076ba29 lea rcx, [rcx + rcx*4]
14076ba2d movaps xmm0, xmmword ptr [rbp + 0x3a0]
14076ba34 movups xmmword ptr [rax + rcx*8], xmm0
14076ba38 movss xmm0, dword ptr [rbp + 0x494]
14076ba40 movss dword ptr [rax + rcx*8 + 0x10], xmm0
14076ba46 movaps xmm0, xmmword ptr [rbp + 0x1e0]
14076ba4d movss dword ptr [rax + rcx*8 + 0x14], xmm0
14076ba53 movlps qword ptr [rax + rcx*8 + 0x18], xmm13
14076ba59 mov edx, dword ptr [rbp + 0x480]
14076ba5f mov dword ptr [rax + rcx*8 + 0x20], edx
14076ba63 mov word ptr [rax + rcx*8 + 0x24], 0
14076ba6a inc qword ptr [rbp + 0x440]
14076ba71 movups xmm0, xmmword ptr [rbp + 0x218]
14076ba78 movaps xmmword ptr [rbp + 0x2d0], xmm0
14076ba7f mov rax, qword ptr [rbp + 0x450]
14076ba86 mov rcx, qword ptr [rbp + 0x1c0]
14076ba8d cmp rax, rcx
14076ba90 mov rdx, rcx
14076ba93 cmovb rdx, rax
14076ba97 mov qword ptr [rbp + 0x458], rdx
14076ba9e test rdx, rdx
14076baa1 je 0x14076bcaf
14076baa7 mov al, 1
14076baa9 mov dword ptr [rbp + 0x1b0], eax
14076baaf xor edi, edi
14076bab1 xor ebx, ebx
14076bab3 jmp 0x14076bada
14076bab5 nop word ptr cs:[rax + rax]
14076bac0 mov dword ptr [rbp + 0x1b0], 0
14076baca inc rbx
14076bacd cmp rbx, qword ptr [rbp + 0x458]
14076bad4 je 0x14076bc49
14076bada mov rax, rdi
14076badd mov rcx, qword ptr [rbp + 0x4a8]
14076bae4 movss xmm0, dword ptr [rcx + rbx*4]
14076bae9 mov rcx, qword ptr [rbp + 0x370]
14076baf0 mov rdx, qword ptr [rcx + rbx*8]
14076baf4 movss xmm1, dword ptr [rbp + 0x2d0]
14076bafc movss xmm2, dword ptr [rdx + 8]
14076bb01 addss xmm2, dword ptr [rbp + 0x478]
14076bb09 xor edi, edi
14076bb0b ucomiss xmm1, xmm2
14076bb0e seta dil
14076bb12 add rdi, rax
14076bb15 ucomiss xmm0, dword ptr [rbp + 0x2d8]
14076bb1c ja 0x14076bc49
14076bb22 lea rcx, [rbp + 0x3a0]
14076bb29 movaps xmm2, xmm8
14076bb2d movaps xmm3, xmm9
14076bb31 call 0x141240300
14076bb36 lea rcx, [rbp + 0x3a0]
14076bb3d lea rdx, [rbp + 0x2d0]
14076bb44 call 0x141240170
14076bb49 mov qword ptr [rbp + 0x498], rbx
14076bb50 test al, al
14076bb52 je 0x14076bc07
14076bb58 mov rax, qword ptr [rbp + 0x440]
14076bb5f test rax, rax
14076bb62 je 0x14076bb97
14076bb64 mov r8, qword ptr [rbp + 0x428]
14076bb6b mov rcx, qword ptr [rbp + 0x438]
14076bb72 cmp rcx, r8
14076bb75 mov edx, 0
14076bb7a cmovae rdx, r8
14076bb7e sub rcx, rdx
14076bb81 mov rdx, r8
14076bb84 sub rdx, rcx
14076bb87 mov r9, rax
14076bb8a sub r9, rdx
14076bb8d ja 0x14076bb9f
14076bb8f add rax, rcx
14076bb92 mov r8, rax
14076bb95 jmp 0x14076bb9c
14076bb97 xor ecx, ecx
14076bb99 xor r8d, r8d
14076bb9c xor r9d, r9d
14076bb9f mov rax, qword ptr [rbp + 0x430]
14076bba6 lea rcx, [rcx + rcx*4]
14076bbaa lea rdx, [rax + rcx*8]
14076bbae lea rcx, [r8 + r8*4]
14076bbb2 lea rcx, [rax + rcx*8]
14076bbb6 lea r8, [r9 + r9*4]
14076bbba lea r9, [rax + r8*8]
14076bbbe jmp 0x14076bbef
14076bbc0 mov r8, rdx
14076bbc3 mov rdx, r9
14076bbc6 mov r9, rdx
14076bbc9 lea rdx, [r8 + 0x28]
14076bbcd mov r10d, dword ptr [r8 + 0x20]
14076bbd1 mov r11, r10
14076bbd4 mov ebx, 0xcccccccd
14076bbd9 imul r11, rbx
14076bbdd shr r11, 0x23
14076bbe1 add r11d, r11d
14076bbe4 lea r11d, [r11 + r11*4]
14076bbe8 sub r10d, r11d
14076bbeb mov dword ptr [r8 + 0x20], r10d
14076bbef cmp rdx, rcx
14076bbf2 jne 0x14076bbc0
14076bbf4 test rax, rax
14076bbf7 je 0x14076bc07
14076bbf9 mov r8, rax
14076bbfc mov rcx, r9
14076bbff cmp rax, r9
14076bc02 mov rax, rdx
14076bc05 jne 0x14076bbc6
14076bc07 mov rax, qword ptr [rbp + 0x370]
14076bc0e mov rbx, qword ptr [rbp + 0x498]
14076bc15 mov rcx, qword ptr [rax + rbx*8]
14076bc19 lea rdx, [rbp + 0x2d0]
14076bc20 call 0x141240170
14076bc25 test al, al
14076bc27 je 0x14076baca
14076bc2d mov rax, qword ptr [rbp + 0x370]
14076bc34 mov rcx, qword ptr [rax + rbx*8]
14076bc38 lea rdx, [rbp + 0x428]
14076bc3f call 0x1412411c0
14076bc44 jmp 0x14076bac0
14076bc49 mov rdx, qword ptr [rbp + 0x1c0]
14076bc50 mov rax, rdx
14076bc53 sub rax, rdi
14076bc56 mov qword ptr [rbp + 0x200], rax
14076bc5d jb 0x14076c87b
14076bc63 mov rdx, qword ptr [rbp + 0x450]
14076bc6a mov rax, rdx
14076bc6d sub rax, rdi
14076bc70 mov qword ptr [rbp + 0x2c8], rax
14076bc77 jb 0x14076c88f
14076bc7d mov rax, qword ptr [rbp + 0x370]
14076bc84 lea rax, [rax + rdi*8]
14076bc88 mov qword ptr [rbp + 0x370], rax
14076bc8f mov rax, qword ptr [rbp + 0x4a8]
14076bc96 lea rax, [rax + rdi*4]
14076bc9a mov qword ptr [rbp + 0x4a8], rax
14076bca1 test byte ptr [rbp + 0x1b0], 1
14076bca8 jne 0x14076bcbd
14076bcaa jmp 0x14076bde7
14076bcaf mov qword ptr [rbp + 0x2c8], rax
14076bcb6 mov qword ptr [rbp + 0x200], rcx
14076bcbd pshufd xmm1, xmm13, 0xf5
14076bcc3 movaps xmm0, xmm13
14076bcc7 cmpunordss xmm0, xmm13
14076bccd movaps xmm2, xmm0
14076bcd0 andps xmm2, xmm1
14076bcd3 maxss xmm1, xmm13
14076bcd8 andnps xmm0, xmm1
14076bcdb orps xmm0, xmm2
14076bcde movss xmm4, dword ptr [rbp + 0x494]
14076bce6 movaps xmm1, xmm4
14076bce9 cmpunordss xmm1, xmm4
14076bcee movaps xmm2, xmm1
14076bcf1 movaps xmm3, xmmword ptr [rbp + 0x1e0]
14076bcf8 andps xmm2, xmm3
14076bcfb minss xmm3, xmm4
14076bcff andnps xmm1, xmm3
14076bd02 orps xmm1, xmm2
14076bd05 subss xmm0, xmm1
14076bd09 movss xmm1, dword ptr [rbp + 0x24c]
14076bd11 addss xmm0, xmm1
14076bd15 movaps xmm2, xmm0
14076bd18 divss xmm2, xmm1
14076bd1c movss xmm3, dword ptr [rip + 0x21b9be0]
14076bd24 movaps xmm1, xmm3
14076bd27 subss xmm1, xmm2
14076bd2b movaps xmm2, xmm3
14076bd2e subss xmm2, xmm1
14076bd32 mulss xmm2, dword ptr [rip + 0x21b9c36]
14076bd3a mulss xmm1, dword ptr [rip + 0x22aaf76]
14076bd42 cmp byte ptr [rbp + 0x4b7], 0
14076bd49 jne 0x14076bd5c
14076bd4b xorps xmm1, xmm1
14076bd4e ucomiss xmm0, dword ptr [rip + 0x2234a0f]
14076bd55 ja 0x14076bd69
14076bd57 jmp 0x14076bde7
14076bd5c addss xmm1, xmm2
14076bd60 ucomiss xmm0, dword ptr [rip + 0x22349fd]
14076bd67 jbe 0x14076bde7
14076bd69 ucomiss xmm1, xmm6
14076bd6c jbe 0x14076bde7
14076bd6e cmp qword ptr [rbp + 0x440], 0
14076bd76 je 0x14076bd83
14076bd78 mov qword ptr [rbp + 0x440], 0
14076bd83 mov qword ptr [rbp + 0x438], 0
14076bd8e divss xmm6, dword ptr [rip + 0x21b9bda]
14076bd96 movss xmm0, dword ptr [rbp + 0x494]
14076bd9e movss dword ptr [rbp + 0x3a0], xmm0
14076bda6 movaps xmm0, xmmword ptr [rbp + 0x1e0]
14076bdad movss dword ptr [rbp + 0x3a4], xmm0
14076bdb5 movlpd qword ptr [rbp + 0x3a8], xmm13
14076bdbe lea rax, [rbp + 0x428]
14076bdc5 mov qword ptr [rsp + 0x20], rax
14076bdca movaps xmm0, xmm6
14076bdcd lea rdx, [rbp + 0x218]
14076bdd4 lea r8, [rbp + 0x3a0]
14076bddb mov r9d, dword ptr [rbp + 0x480]
14076bde2 call 0x141240ec0
14076bde7 movss xmm0, dword ptr [rbp + 0x254]
14076bdef ucomiss xmm0, dword ptr [rbp + 0x460]
14076bdf6 setnp al
14076bdf9 sete cl
14076bdfc and cl, al
14076bdfe test byte ptr [rbp + 0x378], cl
14076be04 jne 0x14076be52
14076be06 mov rcx, qword ptr [rbp + 0x408]
14076be0d movss xmm1, dword ptr [rbp + 0x460]
14076be15 call 0x1411f8c10
14076be1a mov eax, eax
14076be1c movss xmm1, dword ptr [rip + 0x21b9ae0]
14076be24 subss xmm1, xmm0
14076be28 mov rcx, qword ptr [rbp + 0x400]
14076be2f movsd xmm2, qword ptr [rcx + rax*8]
14076be34 movsd xmm3, qword ptr [rcx + rax*8 + 8]
14076be3a movsldup xmm1, xmm1
14076be3e mulps xmm1, xmm2
14076be41 movsldup xmm0, xmm0
14076be45 mulps xmm0, xmm3
14076be48 addps xmm0, xmm1
14076be4b movaps xmmword ptr [rbp + 0xa0], xmm0
14076be52 mov rcx, qword ptr [rbp + 0x408]
14076be59 movss xmm1, dword ptr [rbp + 0x4a0]
14076be61 call 0x1411f8c10
14076be66 mov eax, eax
14076be68 movss xmm4, dword ptr [rip + 0x21b9a94]
14076be70 movaps xmm1, xmm4
14076be73 subss xmm1, xmm0
14076be77 mov rcx, qword ptr [rbp + 0x400]
14076be7e movsd xmm2, qword ptr [rcx + rax*8]
14076be83 movsd xmm3, qword ptr [rcx + rax*8 + 8]
14076be89 movsldup xmm1, xmm1
14076be8d mulps xmm1, xmm2
14076be90 movsldup xmm0, xmm0
14076be94 mulps xmm0, xmm3
14076be97 addps xmm0, xmm1
14076be9a movaps xmmword ptr [rbp + 0x1b0], xmm0
14076bea1 subps xmm0, xmmword ptr [rbp + 0xa0]
14076bea8 movaps xmm1, xmm0
14076beab mulps xmm1, xmm0
14076beae movshdup xmm2, xmm1
14076beb2 addss xmm2, xmm1
14076beb6 xorps xmm1, xmm1
14076beb9 sqrtss xmm1, xmm2
14076bebd movaps xmm2, xmm4
14076bec0 divss xmm2, xmm1
14076bec4 movsldup xmm1, xmm2
14076bec8 mulps xmm1, xmm0
14076becb movlps qword ptr [rbp + 0x258], xmm1
14076bed2 lea rcx, [rbp + 0x3a0]
14076bed9 lea rdx, [rbp + 0x258]
14076bee0 call 0x140c9cdc0
14076bee5 movsd xmm6, qword ptr [rbp + 0x3a4]
14076beed movss xmm11, dword ptr [rbp + 0x3a0]
14076bef6 movss xmm0, dword ptr [rbp + 0x258]
14076befe movss xmm1, dword ptr [rbp + 0x25c]
14076bf06 xorps xmm1, xmmword ptr [rip + 0x21b99e3]
14076bf0d movss dword ptr [rbp + 0x2d0], xmm1
14076bf15 movss dword ptr [rbp + 0x2d4], xmm0
14076bf1d lea rcx, [rbp + 0x3a0]
14076bf24 lea rdx, [rbp + 0x2d0]
14076bf2b call 0x140c9cdc0
14076bf30 mov r8, qword ptr [rbp + 0x440]
14076bf37 test r8, r8
14076bf3a je 0x14076bf79
14076bf3c mov rcx, qword ptr [rbp + 0x428]
14076bf43 mov rax, qword ptr [rbp + 0x438]
14076bf4a cmp rax, rcx
14076bf4d mov edx, 0
14076bf52 cmovae rdx, rcx
14076bf56 sub rax, rdx
14076bf59 mov r9, rcx
14076bf5c sub r9, rax
14076bf5f mov rdx, r8
14076bf62 sub rdx, r9
14076bf65 movss xmm0, dword ptr [rip + 0x21b9997]
14076bf6d ja 0x14076bf87
14076bf6f add r8, rax
14076bf72 mov rcx, r8
14076bf75 xor edx, edx
14076bf77 jmp 0x14076bf87
14076bf79 xor eax, eax
14076bf7b xor ecx, ecx
14076bf7d xor edx, edx
14076bf7f movss xmm0, dword ptr [rip + 0x21b997d]
14076bf87 divss xmm0, xmm7
14076bf8b movss dword ptr [rbp + 0x378], xmm0
14076bf93 movlhps xmm11, xmm6
14076bf97 shufps xmm11, xmm6, 0x58
14076bf9c movaps xmmword ptr [rbp + 0x1c0], xmm11
14076bfa4 movss xmm0, dword ptr [rbp + 0x3a0]
14076bfac movsd xmm1, qword ptr [rbp + 0x3a4]
14076bfb4 mov r8, qword ptr [rbp + 0x430]
14076bfbb lea rax, [rax + rax*4]
14076bfbf lea rbx, [r8 + rax*8]
14076bfc3 lea rax, [rcx + rcx*4]
14076bfc7 lea rax, [r8 + rax*8]
14076bfcb mov qword ptr [rbp + 0x498], rax
14076bfd2 lea rax, [rdx + rdx*4]
14076bfd6 mov qword ptr [rbp + 0x450], r8
14076bfdd lea rax, [r8 + rax*8]
14076bfe1 mov qword ptr [rbp + 0x368], rax
14076bfe8 movlhps xmm0, xmm1
14076bfeb shufps xmm0, xmm1, 0x58
14076bfef shufps xmm15, xmm15, 0
14076bff4 mulps xmm15, xmm0
14076bff8 jmp 0x14076c0a0
14076bffd movaps xmm0, xmm13
14076c001 subss xmm0, xmm11
14076c006 addss xmm11, xmm13
14076c00b unpcklps xmm0, xmm11
14076c00f mulps xmm0, xmmword ptr [rbp + 0x10]
14076c013 mov r8, qword ptr [rbp + 0x618]
14076c01a mov rax, qword ptr [r8 + 8]
14076c01e lea rcx, [rdi + rdi*2]
14076c022 shl rcx, 5
14076c026 movaps xmm1, xmmword ptr [rbp + 0x3a0]
14076c02d movaps xmm2, xmmword ptr [rbp + 0x3b0]
14076c034 movaps xmm3, xmmword ptr [rbp + 0x3c0]
14076c03b movaps xmm4, xmmword ptr [rbp + 0x3d0]
14076c042 movups xmmword ptr [rax + rcx + 0x30], xmm4
14076c047 movups xmmword ptr [rax + rcx + 0x20], xmm3
14076c04c movups xmmword ptr [rax + rcx + 0x10], xmm2
14076c051 movups xmmword ptr [rax + rcx], xmm1
14076c055 movss dword ptr [rax + rcx + 0x40], xmm12
14076c05c movss dword ptr [rax + rcx + 0x44], xmm10
14076c063 movss xmm1, dword ptr [rbp + 0x480]
14076c06b movss dword ptr [rax + rcx + 0x48], xmm1
14076c071 mov edx, dword ptr [rbp + 0x458]
14076c077 mov dword ptr [rax + rcx + 0x4c], edx
14076c07b mov edx, dword ptr [rbp + 0x22c]
14076c081 mov dword ptr [rax + rcx + 0x50], edx
14076c085 movlps qword ptr [rax + rcx + 0x54], xmm0
14076c08a mov dword ptr [rax + rcx + 0x5c], r12d
14076c08f inc rdi
14076c092 mov qword ptr [r8 + 0x10], rdi
14076c096 nop word ptr cs:[rax + rax]
14076c0a0 cmp rbx, qword ptr [rbp + 0x498]
14076c0a7 je 0x14076c0b0
14076c0a9 mov rdi, rbx
14076c0ac jmp 0x14076c0ed
14076c0ae nop 
14076c0b0 cmp qword ptr [rbp + 0x450], 0
14076c0b8 je 0x14076c455
14076c0be mov rax, qword ptr [rbp + 0x450]
14076c0c5 mov rdi, rax
14076c0c8 mov rcx, qword ptr [rbp + 0x368]
14076c0cf mov qword ptr [rbp + 0x498], rcx
14076c0d6 cmp rax, rcx
14076c0d9 mov qword ptr [rbp + 0x368], rbx
14076c0e0 mov qword ptr [rbp + 0x450], rbx
14076c0e7 je 0x14076c455
14076c0ed mov rcx, rdi
14076c0f0 call 0x1412402e0
14076c0f5 movaps xmm11, xmm0
14076c0f9 mov rcx, rdi
14076c0fc call 0x1412402f0
14076c101 movaps xmm14, xmm0
14076c105 mov rcx, rdi
14076c108 call 0x1412400c0
14076c10d movaps xmm13, xmm0
14076c111 lea rbx, [rdi + 0x28]
14076c115 xorps xmm0, xmm0
14076c118 ucomiss xmm0, xmm11
14076c11c jae 0x14076c0a0
14076c11e ucomiss xmm0, xmm14
14076c122 jae 0x14076c0a0
14076c128 movaps xmm7, xmmword ptr [rbp + 0x2a0]
14076c12f mulss xmm7, xmm11
14076c134 movss xmm0, dword ptr [rip + 0x2260ef0]
14076c13c ucomiss xmm0, xmm7
14076c13f ja 0x14076c0a0
14076c145 movaps xmm0, xmm13
14076c149 subss xmm0, dword ptr [rbp + 0x460]
14076c151 mulss xmm0, dword ptr [rbp + 0x378]
14076c159 movss xmm2, dword ptr [rip + 0x21b97a3]
14076c161 subss xmm2, xmm0
14076c165 movsldup xmm2, xmm2
14076c169 mulps xmm2, xmmword ptr [rbp + 0xa0]
14076c170 movsldup xmm0, xmm0
14076c174 mulps xmm0, xmmword ptr [rbp + 0x1b0]
14076c17b addps xmm0, xmm2
14076c17e movlps qword ptr [rbp + 0x2d0], xmm0
14076c185 mulss xmm1, dword ptr [rbp + 0x1a0]
14076c18d lea rcx, [rbp + 0x3a0]
14076c194 lea rdx, [rbp + 0x2d0]
14076c19b movaps xmm2, xmm1
14076c19e call 0x140c9cde0
14076c1a3 mulss xmm14, dword ptr [rbp + 0x1a0]
14076c1ac movsd xmm0, qword ptr [rbp + 0x3a4]
14076c1b4 movss xmm6, dword ptr [rbp + 0x3a0]
14076c1bc movlhps xmm6, xmm0
14076c1bf shufps xmm6, xmm0, 0x58
14076c1c3 cmp byte ptr [rbp + 0x4b7], 0
14076c1ca je 0x14076c28e
14076c1d0 cmp byte ptr [rbp + 0x4b6], 2
14076c1d7 jne 0x14076c2cf
14076c1dd movshdup xmm0, xmm6
14076c1e1 cmp byte ptr [rbp + 0x658], 0
14076c1e8 movss xmm2, dword ptr [rbp + 0x414]
14076c1f0 movss xmm3, dword ptr [rbp + 0x484]
14076c1f8 je 0x14076c245
14076c1fa movaps xmm1, xmm14
14076c1fe mulss xmm1, dword ptr [rip + 0x21b966a]
14076c206 addss xmm1, xmm0
14076c20a movss xmm12, dword ptr [rbp + 0x47c]
14076c213 ucomiss xmm1, xmm12
14076c217 mov r12d, 0xc
14076c21d ja 0x14076c359
14076c223 movss xmm4, dword ptr [rbp + 0x488]
14076c22b ucomiss xmm1, xmm4
14076c22e movss xmm10, dword ptr [rbp + 0x41c]
14076c237 movss xmm3, dword ptr [rbp + 0x484]
14076c23f ja 0x14076c386
14076c245 subss xmm0, dword ptr [rbp + 0x1a0]
14076c24d andps xmm0, xmmword ptr [rip + 0x21c272c]
14076c254 movss xmm1, dword ptr [rip + 0x2311674]
14076c25c ucomiss xmm1, xmm0
14076c25f mov r12d, 4
14076c265 movaps xmm12, xmm2
14076c269 movaps xmm10, xmm3
14076c26d movss xmm4, dword ptr [rbp + 0x48c]
14076c275 jbe 0x14076c386
14076c27b mov r12d, 0x14
14076c281 movaps xmm12, xmm2
14076c285 movaps xmm10, xmm3
14076c289 jmp 0x14076c37e
14076c28e movss xmm1, dword ptr [rip + 0x23120c2]
14076c296 ucomiss xmm1, xmm7
14076c299 ja 0x14076c0a0
14076c29f ucomiss xmm14, xmm7
14076c2a3 jbe 0x14076c1d0
14076c2a9 subss xmm14, xmm7
14076c2ae mulss xmm14, dword ptr [rip + 0x21ede31]
14076c2b7 addss xmm0, xmm14
14076c2bc movlhps xmm0, xmm6
14076c2bf shufps xmm0, xmm6, 0xe2
14076c2c3 movaps xmm6, xmm0
14076c2c6 movaps xmm14, xmm7
14076c2ca jmp 0x14076c1d0
14076c2cf mov rcx, qword ptr [rbp + 0x628]
14076c2d6 mov rax, qword ptr [rcx + 0x10]
14076c2da mov qword ptr [rbp + 0x3b0], rax
14076c2e1 movups xmm0, xmmword ptr [rcx]
14076c2e4 movaps xmmword ptr [rbp + 0x3a0], xmm0
14076c2eb lea rcx, [rbp + 0x3b4]
14076c2f2 call 0x1408c5680
14076c2f7 mov r12d, 4
14076c2fd test al, al
14076c2ff je 0x14076c36c
14076c301 movshdup xmm0, xmm6
14076c305 ucomiss xmm0, dword ptr [rbp + 0x3a0]
14076c30c movss xmm12, dword ptr [rbp + 0x414]
14076c315 movss xmm10, dword ptr [rbp + 0x484]
14076c31e jbe 0x14076c37e
14076c320 movaps xmm1, xmm14
14076c324 mulss xmm1, dword ptr [rip + 0x21b9544]
14076c32c addss xmm0, xmm1
14076c330 ucomiss xmm0, dword ptr [rbp + 0x47c]
14076c337 ja 0x14076c34a
14076c339 ucomiss xmm0, dword ptr [rbp + 0x488]
14076c340 movss xmm4, dword ptr [rbp + 0x48c]
14076c348 jbe 0x14076c386
14076c34a mov r12d, 0xc
14076c350 movss xmm12, dword ptr [rbp + 0x47c]
14076c359 movss xmm10, dword ptr [rbp + 0x41c]
14076c362 movss xmm4, dword ptr [rbp + 0x488]
14076c36a jmp 0x14076c386
14076c36c movss xmm12, dword ptr [rbp + 0x414]
14076c375 movss xmm10, dword ptr [rbp + 0x484]
14076c37e movss xmm4, dword ptr [rbp + 0x48c]
14076c386 movss dword ptr [rbp + 0x480], xmm4
14076c38e addss xmm7, dword ptr [rbp + 0x328]
14076c396 shufps xmm7, xmm7, 0
14076c39a mulps xmm7, xmmword ptr [rbp + 0x1c0]
14076c3a1 shufps xmm14, xmm14, 0
14076c3a6 mulps xmm14, xmmword ptr [rip + 0x21c8202]
14076c3ae movaps xmmword ptr [rbp - 0x30], xmm7
14076c3b2 movaps xmmword ptr [rbp - 0x20], xmm14
14076c3b7 movaps xmmword ptr [rbp - 0x10], xmm15
14076c3bc movaps xmmword ptr [rbp], xmm6
14076c3c0 movups xmm0, xmmword ptr [rdi + 0x10]
14076c3c4 movaps xmmword ptr [rbp + 0x380], xmm0
14076c3cb lea rcx, [rbp + 0x2d0]
14076c3d2 lea rdx, [rbp - 0x30]
14076c3d6 call 0x14089d560
14076c3db mov eax, dword ptr [rdi + 0x20]
14076c3de mov dword ptr [rbp + 0x458], eax
14076c3e4 mulss xmm11, dword ptr [rip + 0x21b9483]
14076c3ed movups xmm0, xmmword ptr [rbp + 0x2d0]
14076c3f4 movups xmm1, xmmword ptr [rbp + 0x2e0]
14076c3fb movups xmm2, xmmword ptr [rbp + 0x2f0]
14076c402 movaps xmmword ptr [rbp + 0x3c0], xmm2
14076c409 movaps xmmword ptr [rbp + 0x3b0], xmm1
14076c410 movaps xmmword ptr [rbp + 0x3a0], xmm0
14076c417 movaps xmm0, xmmword ptr [rbp + 0x380]
14076c41e lea rax, [rbp + 0x3b4]
14076c425 movups xmmword ptr [rax + 0x1c], xmm0
14076c429 mov rax, qword ptr [rbp + 0x618]
14076c430 mov rdi, qword ptr [rax + 0x10]
14076c434 cmp rdi, qword ptr [rax]
14076c437 jne 0x14076bffd
14076c43d mov rcx, qword ptr [rbp + 0x618]
14076c444 lea rdx, [rip + 0x231229d]
14076c44b call 0x14078cac0
14076c450 jmp 0x14076bffd
14076c455 mov rax, qword ptr [rbp + 0x100]
14076c45c mov qword ptr [rbp + 0x458], rax
14076c463 mov rax, qword ptr [rbp + 0x108]
14076c46a mov qword ptr [rbp + 0x498], rax
14076c471 mov al, 1
14076c473 mov qword ptr [rbp + 0x378], rax
14076c47a mov rax, qword ptr [rbp + 0x200]
14076c481 mov qword ptr [rbp + 0x1c0], rax
14076c488 mov rax, qword ptr [rbp + 0x2c8]
14076c48f mov qword ptr [rbp + 0x450], rax
14076c496 movss xmm0, dword ptr [rbp + 0x4a0]
14076c49e movss dword ptr [rbp + 0x2c8], xmm0
14076c4a6 movaps xmm1, xmmword ptr [rbp + 0x1e0]
14076c4ad movss dword ptr [rbp + 0x494], xmm1
14076c4b5 movaps xmm1, xmmword ptr [rbp + 0xb0]
14076c4bc movaps xmmword ptr [rbp + 0x200], xmm1
14076c4c3 movss dword ptr [rbp + 0x254], xmm0
14076c4cb movaps xmm0, xmmword ptr [rbp + 0x1b0]
14076c4d2 movaps xmmword ptr [rbp + 0xa0], xmm0
14076c4d9 mov r11, qword ptr [rbp + 0x138]
14076c4e0 movaps xmm11, xmmword ptr [rbp + 0x30]
14076c4e5 jmp 0x14076afc4
14076c4ea nop word ptr [rax + rax]
14076c4f0 mov r13, qword ptr [rbp + 0x130]
14076c4f7 jmp 0x14076a1ed
14076c4fc mov r14, qword ptr [rbp + 0x98]
14076c503 movss xmm2, dword ptr [rip + 0x21b93f9]
14076c50b movaps xmm3, xmm2
14076c50e jmp 0x14076ae93
14076c513 mov rdx, qword ptr [rbp + 0x308]
14076c51a test rdx, rdx
14076c51d je 0x14076c535
14076c51f mov rcx, qword ptr [rbp + 0x310]
14076c526 shl rdx, 2
14076c52a mov r8d, 4
14076c530 call 0x140613c20
14076c535 mov rax, qword ptr [rbp + 0x428]
14076c53c test rax, rax
14076c53f mov rsi, qword ptr [rbp + 0x178]
14076c546 je 0x14076c562
14076c548 mov rcx, qword ptr [rbp + 0x430]
14076c54f shl rax, 3
14076c553 lea rdx, [rax + rax*4]
14076c557 mov r8d, 4
14076c55d call 0x140613c20
14076c562 mov rdx, qword ptr [rbp + 0x2b0]
14076c569 test rdx, rdx
14076c56c je 0x14076c584
14076c56e mov rcx, qword ptr [rbp + 0x2b8]
14076c575 shl rdx, 2
14076c579 mov r8d, 4
14076c57f call 0x140613c20
14076c584 mov rdx, qword ptr [rbp + 0x340]
14076c58b test rdx, rdx
14076c58e je 0x14076c5a6
14076c590 mov rcx, qword ptr [rbp + 0x348]
14076c597 shl rdx, 3
14076c59b mov r8d, 8
14076c5a1 call 0x140613c20
14076c5a6 mov rdx, qword ptr [rbp + 0xf8]
14076c5ad test rdx, rdx
14076c5b0 je 0x14076c5c8
14076c5b2 shl rdx, 5
14076c5b6 mov r8d, 8
14076c5bc mov rcx, qword ptr [rbp + 0x458]
14076c5c3 call 0x140613c20
14076c5c8 mov rdx, qword ptr [rbp + 0x290]
14076c5cf test rdx, rdx
14076c5d2 je 0x14076c5ea
14076c5d4 shl rdx, 2
14076c5d8 mov r8d, 4
14076c5de mov rcx, qword ptr [rbp + 0x298]
14076c5e5 call 0x140613c20
14076c5ea mov rdx, qword ptr [rbp + 0xc8]
14076c5f1 test rdx, rdx
14076c5f4 je 0x14076c60c
14076c5f6 mov rcx, qword ptr [rbp + 0xd0]
14076c5fd shl rdx, 2
14076c601 mov r8d, 4
14076c607 call 0x140613c20
14076c60c shl esi, 2
14076c60f mov r8d, 4
14076c615 mov rcx, qword ptr [rbp + 0x170]
14076c61c mov rdx, rsi
14076c61f call 0x140613c20
14076c624 cmp qword ptr [rbp + 0x120], 0
14076c62c je 0x14076c679
14076c62e call 0x140c18130
14076c633 mov rcx, rax
14076c636 mov rax, qword ptr [rax]
14076c639 cmp rax, 1
14076c63d jne 0x14076c70f
14076c643 add rcx, 8
14076c647 cmp qword ptr [rcx], 0
14076c64b jne 0x14076c750
14076c651 mov qword ptr [rcx], 0xffffffffffffffff
14076c658 mov qword ptr [rbp + 0x4a8], rcx
14076c65f add rcx, 8
14076c663 mov rdx, qword ptr [rbp + 0x128]
14076c66a call 0x140c170f0
14076c66f mov rax, qword ptr [rbp + 0x4a8]
14076c676 inc qword ptr [rax]
14076c679 movaps xmm6, xmmword ptr [rbp + 0x4c0]
14076c680 movaps xmm7, xmmword ptr [rbp + 0x4d0]
14076c687 movaps xmm8, xmmword ptr [rbp + 0x4e0]
14076c68f movaps xmm9, xmmword ptr [rbp + 0x4f0]
14076c697 movaps xmm10, xmmword ptr [rbp + 0x500]
14076c69f movaps xmm11, xmmword ptr [rbp + 0x510]
14076c6a7 movaps xmm12, xmmword ptr [rbp + 0x520]
14076c6af movaps xmm13, xmmword ptr [rbp + 0x530]
14076c6b7 movaps xmm14, xmmword ptr [rbp + 0x540]
14076c6bf movaps xmm15, xmmword ptr [rbp + 0x550]
14076c6c7 add rsp, 0x5e8
14076c6ce pop rbx
14076c6cf pop rdi
14076c6d0 pop rsi
14076c6d1 pop r12
14076c6d3 pop r13
14076c6d5 pop r14
14076c6d7 pop r15
14076c6d9 pop rbp
14076c6da ret 
14076c6db test rax, rax
14076c6de jne 0x14076c727
14076c6e0 xor edx, edx
14076c6e2 call 0x1428f15c0
14076c6e7 mov rcx, rax
14076c6ea test rax, rax
14076c6ed jne 0x1407699ba
14076c6f3 jmp 0x14076c727
14076c6f5 test rax, rax
14076c6f8 jne 0x14076c727
14076c6fa xor edx, edx
14076c6fc call 0x1428f15c0
14076c701 mov rcx, rax
14076c704 test rax, rax
14076c707 jne 0x140769b54
14076c70d jmp 0x14076c727
14076c70f test rax, rax
14076c712 jne 0x14076c727
14076c714 xor edx, edx
14076c716 call 0x1428f15c0
14076c71b mov rcx, rax
14076c71e test rax, rax
14076c721 jne 0x14076c647
14076c727 lea rcx, [rip + 0x24acd7a]
14076c72e call 0x1428d80d0
14076c733 lea rcx, [rip + 0x24ad956]
14076c73a lea r8, [rip + 0x24ad9a7]
14076c741 mov edx, 0x21
14076c746 call 0x1428d9430
14076c74b jmp 0x14076c95c
14076c750 lea rcx, [rip + 0x24acdc1]
14076c757 call 0x1428d9250
14076c75c mov qword ptr [rbp + 0x448], rsi
14076c763 lea rcx, [rip + 0x2311e65]
14076c76a lea r8, [rip + 0x2311e87]
14076c771 mov edx, 0x29
14076c776 call 0x1428d9430
14076c77b jmp 0x14076c95c
14076c780 mov qword ptr [rbp + 0x3a0], 0
14076c78b mov qword ptr [rbp + 0x448], rsi
14076c792 lea rax, [rip + 0x2311e77]
14076c799 mov qword ptr [rsp + 0x20], rax
14076c79e lea rdx, [rbp + 0x80]
14076c7a5 lea r8, [rbp + 0x2d0]
14076c7ac lea r9, [rbp + 0x3a0]
14076c7b3 xor ecx, ecx
14076c7b5 call 0x1428d9596
14076c7ba jmp 0x14076c95c
14076c7bf mov qword ptr [rbp + 0x3a0], 0
14076c7ca mov qword ptr [rbp + 0x448], rsi
14076c7d1 lea rax, [rip + 0x2311e50]
14076c7d8 mov qword ptr [rsp + 0x20], rax
14076c7dd lea rdx, [rbp + 0x428]
14076c7e4 lea r8, [rbp + 0x2d0]
14076c7eb lea r9, [rbp + 0x3a0]
14076c7f2 xor ecx, ecx
14076c7f4 call 0x1428d9596
14076c7f9 jmp 0x14076c95c
14076c7fe mov qword ptr [rbp + 0x3a0], 0
14076c809 mov qword ptr [rbp + 0x448], rsi
14076c810 lea rax, [rip + 0x2311e29]
14076c817 mov qword ptr [rsp + 0x20], rax
14076c81c lea rdx, [rbp + 0x428]
14076c823 lea r8, [rbp + 0x2d0]
14076c82a lea r9, [rbp + 0x3a0]
14076c831 xor ecx, ecx
14076c833 call 0x1428d9596
14076c838 jmp 0x14076c95c
14076c83d cmp rdx, 0x15
14076c841 jae 0x14076c91d
14076c847 lea r9, [rbp + 0x3a0]
14076c84e mov r8d, 1
14076c854 call 0x14122cf10
14076c859 jmp 0x140769e2a
14076c85e lea rcx, [rip + 0x24ad82b]
14076c865 lea r8, [rip + 0x24ad87c]
14076c86c mov edx, 0x21
14076c871 call 0x1428d9430
14076c876 jmp 0x14076c95c
14076c87b lea r8, [rip + 0x2311eae]
14076c882 mov rcx, rdi
14076c885 call 0x1428d9830
14076c88a jmp 0x14076c95c
14076c88f lea r8, [rip + 0x2311eb2]
14076c896 mov rcx, rdi
14076c899 call 0x1428d9830
14076c89e jmp 0x14076c95c
14076c8a3 lea r8, [rip + 0x2311e0e]
14076c8aa mov rdx, qword ptr [rbp + 0x498]
14076c8b1 call 0x1428d9518
14076c8b6 jmp 0x14076c95c
14076c8bb mov qword ptr [rbp + 0x110], 0
14076c8c6 lea r8, [rip + 0x2311d8b]
14076c8cd mov ecx, 4
14076c8d2 mov edx, 0x500
14076c8d7 call 0x1428d8fe3
14076c8dc jmp 0x14076c95c
14076c8de mov ecx, 4
14076c8e3 mov edx, 4
14076c8e8 call 0x1428d9000
14076c8ed jmp 0x14076c95c
14076c8ef mov qword ptr [rbp + 0x448], rsi
14076c8f6 mov ecx, 4
14076c8fb mov edx, 0xc
14076c900 call 0x1428d9000
14076c905 jmp 0x14076c95c
14076c907 lea r8, [rip + 0x2311b5a]
14076c90e mov ecx, 4
14076c913 mov rdx, r15
14076c916 call 0x1428d8fe3
14076c91b jmp 0x14076c95c
14076c91d lea r8, [rbp + 0x3a0]
14076c924 call 0x14122f410
14076c929 jmp 0x140769e2a
14076c92e mov qword ptr [rbp + 0x448], rsi
14076c935 mov ecx, 4
14076c93a mov edx, 8
14076c93f call 0x1428d9000
14076c944 jmp 0x14076c95c
14076c946 mov qword ptr [rbp + 0x448], rsi
14076c94d mov ecx, 4
14076c952 mov edx, 8
14076c957 call 0x1428d9000
14076c95c ud2 
14076c95e nop 
14076c960 mov qword ptr [rsp + 0x10], rdx
14076c965 push rbp
14076c966 push r15
14076c968 push r14
14076c96a push r13
14076c96c push r12
14076c96e push rsi
14076c96f push rdi
14076c970 push rbx
14076c971 sub rsp, 0xe8
14076c978 lea rbp, [rdx + 0x80]
14076c97f movaps xmmword ptr [rsp + 0x40], xmm15
14076c985 movaps xmmword ptr [rsp + 0x50], xmm14
14076c98b movaps xmmword ptr [rsp + 0x60], xmm13
14076c991 movaps xmmword ptr [rsp + 0x70], xmm12
14076c997 movaps xmmword ptr [rsp + 0x80], xmm11
14076c9a0 movaps xmmword ptr [rsp + 0x90], xmm10
14076c9a9 movaps xmmword ptr [rsp + 0xa0], xmm9
14076c9b2 movaps xmmword ptr [rsp + 0xb0], xmm8
14076c9bb movaps xmmword ptr [rsp + 0xc0], xmm7
14076c9c3 movaps xmmword ptr [rsp + 0xd0], xmm6
14076c9cb mov rax, qword ptr [rbp + 0x4a8]
14076c9d2 inc qword ptr [rax]
14076c9d5 movaps xmm6, xmmword ptr [rsp + 0xd0]
14076c9dd movaps xmm7, xmmword ptr [rsp + 0xc0]
14076c9e5 movaps xmm8, xmmword ptr [rsp + 0xb0]
14076c9ee movaps xmm9, xmmword ptr [rsp + 0xa0]
14076c9f7 movaps xmm10, xmmword ptr [rsp + 0x90]
14076ca00 movaps xmm11, xmmword ptr [rsp + 0x80]
14076ca09 movaps xmm12, xmmword ptr [rsp + 0x70]
14076ca0f movaps xmm13, xmmword ptr [rsp + 0x60]
14076ca15 movaps xmm14, xmmword ptr [rsp + 0x50]
14076ca1b movaps xmm15, xmmword ptr [rsp + 0x40]
14076ca21 add rsp, 0xe8
14076ca28 pop rbx
14076ca29 pop rdi
14076ca2a pop rsi
14076ca2b pop r12
14076ca2d pop r13
14076ca2f pop r14
14076ca31 pop r15
14076ca33 pop rbp
14076ca34 ret 
14076ca35 nop word ptr cs:[rax + rax]
14076ca40 mov qword ptr [rsp + 0x10], rdx
14076ca45 push rbp
14076ca46 push r15
14076ca48 push r14
14076ca4a push r13
14076ca4c push r12
14076ca4e push rsi
14076ca4f push rdi
14076ca50 push rbx
14076ca51 sub rsp, 0xe8
14076ca58 lea rbp, [rdx + 0x80]
