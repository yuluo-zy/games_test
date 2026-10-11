141243740 push r15
141243742 push r14
141243744 push r12
141243746 push rsi
141243747 push rdi
141243748 push rbp
141243749 push rbx
14124374a sub rsp, 0x120
141243751 movaps xmmword ptr [rsp + 0x110], xmm12
14124375a movaps xmmword ptr [rsp + 0x100], xmm11
141243763 movaps xmmword ptr [rsp + 0xf0], xmm10
14124376c movaps xmmword ptr [rsp + 0xe0], xmm9
141243775 movaps xmmword ptr [rsp + 0xd0], xmm8
14124377e movaps xmmword ptr [rsp + 0xc0], xmm7
141243786 movaps xmmword ptr [rsp + 0xb0], xmm6
14124378e mov rsi, qword ptr [rsp + 0x180]
141243796 mov qword ptr [rsp + 0x68], rcx
14124379b mov ecx, edx
14124379d cmp rcx, rsi
1412437a0 jae 0x141243b8c
1412437a6 lea rax, [rcx + 1]
1412437aa cmp rax, rsi
1412437ad jae 0x141243b9b
1412437b3 mov rbx, qword ptr [rsp + 0x1a0]
1412437bb cmp rcx, rbx
1412437be jae 0x141243bad
1412437c4 cmp rax, rbx
1412437c7 jae 0x141243bbc
1412437cd mov rdi, r9
1412437d0 movaps xmm6, xmm2
1412437d3 mov r14, qword ptr [rsp + 0x198]
1412437db movzx r8d, byte ptr [rsp + 0x190]
1412437e4 movss xmm0, dword ptr [rsp + 0x188]
1412437ed movss xmm1, dword ptr [r9 + rcx*4]
1412437f3 movss xmm8, dword ptr [r9 + rcx*4 + 4]
1412437fa subss xmm8, xmm1
1412437ff mulss xmm8, xmm2
141243804 addss xmm8, xmm1
141243809 addss xmm8, dword ptr [rip + 0x183af5e]
141243812 addss xmm0, dword ptr [rip + 0x19dbfca]
14124381a movss xmm10, dword ptr [rip + 0x16e214d]
141243823 mulss xmm10, xmm8
141243828 movaps xmm1, xmm0
14124382b minss xmm1, xmm10
141243830 cmpunordss xmm10, xmm10
141243836 movaps xmm2, xmm10
14124383a andnps xmm2, xmm1
14124383d andps xmm10, xmm0
141243841 orps xmm10, xmm2
141243845 lea rcx, [rcx + rcx*2]
141243849 movss xmm9, dword ptr [r14 + rcx*4]
14124384f lea rax, [rax + rax*2]
141243853 movss xmm7, dword ptr [r14 + rax*4]
141243859 movsd xmm11, qword ptr [r14 + rcx*4 + 4]
141243860 movsd xmm12, qword ptr [r14 + rax*4 + 4]
141243867 mov qword ptr [rsp + 0x28], r14
14124386c mov qword ptr [rsp + 0x30], rbx
141243871 lea rax, [rip - 0x3cc38]
141243878 mov qword ptr [rsp + 0x38], rax
14124387d mov dword ptr [rsp + 0x40], edx
141243881 movss dword ptr [rsp + 0x44], xmm6
141243887 mov dword ptr [rsp + 0x48], edx
14124388b movss dword ptr [rsp + 0x4c], xmm6
141243891 movss xmm1, dword ptr [rip + 0x19dbf53]
141243899 addss xmm1, xmm10
14124389e addss xmm1, dword ptr [rip + 0x183892a]
1412438a6 test r8b, r8b
1412438a9 je 0x1412438c1
1412438ab lea r12, [rsp + 0x48]
1412438b0 lea r15, [rsp + 0x4c]
1412438b5 lea rcx, [rsp + 0x28]
1412438ba call 0x141207c10
1412438bf jmp 0x1412438d5
1412438c1 lea r12, [rsp + 0x40]
1412438c6 lea r15, [rsp + 0x44]
1412438cb lea rcx, [rsp + 0x28]
1412438d0 call 0x141206ed0
1412438d5 mov ecx, dword ptr [r12]
1412438d9 cmp rcx, rsi
1412438dc jae 0x141243bce
1412438e2 lea rax, [rcx + 1]
1412438e6 cmp rax, rsi
1412438e9 jae 0x141243bdd
1412438ef cmp rcx, rbx
1412438f2 jae 0x141243bef
1412438f8 cmp rax, rbx
1412438fb jae 0x141243bfe
141243901 subss xmm8, xmm10
141243906 subss xmm7, xmm9
14124390b subps xmm12, xmm11
14124390f movsldup xmm0, xmm6
141243913 mulps xmm0, xmm12
141243917 addss xmm8, dword ptr [rip + 0x19dbec8]
141243920 xorps xmm1, xmm1
141243923 mulss xmm7, xmm6
141243927 mulss xmm1, xmm8
14124392c addss xmm7, xmm9
141243931 addps xmm0, xmm11
141243935 unpcklps xmm8, xmm1
141243939 addps xmm8, xmm0
14124393d mov ebx, dword ptr [rsp + 0x1c0]
141243944 mov ebp, dword ptr [rsp + 0x1b8]
14124394b mov rsi, qword ptr [rsp + 0x1a8]
141243953 addss xmm7, xmm1
141243957 movss xmm2, dword ptr [r15]
14124395c movss xmm0, dword ptr [rdi + rcx*4]
141243961 movss xmm1, dword ptr [rdi + rcx*4 + 4]
141243967 subss xmm1, xmm0
14124396b mulss xmm1, xmm2
14124396f addss xmm1, xmm0
141243973 lea rcx, [rcx + rcx*2]
141243977 movss xmm3, dword ptr [r14 + rcx*4]
14124397d lea rax, [rax + rax*2]
141243981 movss xmm0, dword ptr [r14 + rax*4]
141243987 subss xmm0, xmm3
14124398b mulss xmm0, xmm2
14124398f addss xmm0, xmm3
141243993 addss xmm1, dword ptr [rip + 0x183add9]
14124399b xorps xmm3, xmm3
14124399e mulss xmm3, xmm1
1412439a2 addss xmm0, xmm3
1412439a6 subss xmm0, xmm7
1412439aa movaps xmm4, xmm0
1412439ad mulss xmm4, xmm0
1412439b1 movsd xmm5, qword ptr [r14 + rcx*4 + 4]
1412439b8 movsd xmm6, qword ptr [r14 + rax*4 + 4]
1412439bf subps xmm6, xmm5
1412439c2 movsldup xmm2, xmm2
1412439c6 mulps xmm2, xmm6
1412439c9 addps xmm2, xmm5
1412439cc unpcklps xmm1, xmm3
1412439cf addps xmm1, xmm2
1412439d2 subps xmm1, xmm8
1412439d6 movaps xmm2, xmm1
1412439d9 mulps xmm2, xmm1
1412439dc addss xmm4, xmm2
1412439e0 movshdup xmm2, xmm2
1412439e4 addss xmm2, xmm4
1412439e8 xorps xmm3, xmm3
1412439eb sqrtss xmm3, xmm2
1412439ef movss xmm2, dword ptr [rip + 0x179b775]
1412439f7 addss xmm2, xmm3
1412439fb movss xmm4, dword ptr [rip + 0x16e1f01]
141243a03 divss xmm4, xmm3
141243a07 mulss xmm0, xmm4
141243a0b movsldup xmm3, xmm4
141243a0f mulps xmm3, xmm1
141243a12 movlhps xmm0, xmm3
141243a15 movaps xmm4, xmm0
141243a18 shufps xmm4, xmm3, 0xd8
141243a1c shufps xmm0, xmm3, 0x58
141243a20 movaps xmm5, xmm4
141243a23 shufps xmm5, xmm4, 0xd2
141243a27 movsd xmm1, qword ptr [rip + 0x16f0b81]
141243a2f mulps xmm1, xmm5
141243a32 movaps xmm3, xmmword ptr [rip + 0x1765647]
141243a39 mulps xmm3, xmm0
141243a3c addps xmm3, xmm1
141243a3f movaps xmm1, xmm3
141243a42 shufps xmm1, xmm3, 0x52
141243a46 mulps xmm3, xmm3
141243a49 movaps xmm6, xmm3
141243a4c unpckhpd xmm6, xmm3
141243a50 addss xmm6, xmm3
141243a54 movshdup xmm3, xmm3
141243a58 addss xmm3, xmm6
141243a5c shufps xmm3, xmm3, 0
141243a60 sqrtps xmm3, xmm3
141243a63 divps xmm1, xmm3
141243a66 movaps xmm3, xmm1
141243a69 shufps xmm3, xmm1, 0xd2
141243a6d mulps xmm3, xmm4
141243a70 mulps xmm5, xmm1
141243a73 subps xmm3, xmm5
141243a76 shufps xmm3, xmm3, 0x52
141243a7a movlhps xmm7, xmm8
141243a7e shufps xmm7, xmm8, 0x58
141243a83 movq xmm2, xmm2
141243a87 movaps xmm5, xmmword ptr [rip + 0x19dbd62]
141243a8e mulps xmm5, xmm1
141243a91 movaps xmm4, xmmword ptr [rip + 0x19dbd68]
141243a98 movaps xmm6, xmm0
141243a9b mulps xmm6, xmm4
141243a9e addps xmm6, xmm5
141243aa1 mulps xmm4, xmm3
141243aa4 addps xmm4, xmm6
141243aa7 xorps xmm5, xmm5
141243aaa mulps xmm1, xmm5
141243aad shufps xmm2, xmm2, 0x80
141243ab1 mulps xmm2, xmm0
141243ab4 addps xmm2, xmm1
141243ab7 movaps xmm6, xmm3
141243aba mulps xmm6, xmm5
141243abd addps xmm2, xmm6
141243ac0 mulps xmm0, xmm5
141243ac3 mulps xmm3, xmmword ptr [rip + 0x19dbd46]
141243aca addps xmm0, xmm1
141243acd addps xmm3, xmm0
141243ad0 addps xmm0, xmm6
141243ad3 addps xmm0, xmm7
141243ad6 movaps xmmword ptr [rsp + 0x70], xmm4
141243adb movaps xmmword ptr [rsp + 0x80], xmm2
141243ae3 movaps xmmword ptr [rsp + 0x90], xmm3
141243aeb movaps xmmword ptr [rsp + 0xa0], xmm0
141243af3 lea rdi, [rsp + 0x28]
141243af8 lea rdx, [rsp + 0x70]
141243afd mov rcx, rdi
141243b00 call 0x14089d560
141243b05 lea rcx, [rsp + 0x68]
141243b0a call 0x1400885d0
141243b0f mov dword ptr [rsp + 0x58], ebp
141243b13 mov dword ptr [rsp + 0x5c], ebx
141243b17 mov dword ptr [rsp + 0x60], eax
141243b1b mov dword ptr [rsp + 0x64], 0
141243b23 mov rcx, rsi
141243b26 mov rdx, rdi
141243b29 call 0x14120c430
141243b2e test al, al
141243b30 je 0x141243b3d
141243b32 mov rax, qword ptr [rsp + 0x1b0]
141243b3a mov byte ptr [rax], 1
141243b3d movaps xmm6, xmmword ptr [rsp + 0xb0]
141243b45 movaps xmm7, xmmword ptr [rsp + 0xc0]
141243b4d movaps xmm8, xmmword ptr [rsp + 0xd0]
141243b56 movaps xmm9, xmmword ptr [rsp + 0xe0]
141243b5f movaps xmm10, xmmword ptr [rsp + 0xf0]
141243b68 movaps xmm11, xmmword ptr [rsp + 0x100]
141243b71 movaps xmm12, xmmword ptr [rsp + 0x110]
141243b7a add rsp, 0x120
141243b81 pop rbx
141243b82 pop rbp
141243b83 pop rdi
141243b84 pop rsi
141243b85 pop r12
141243b87 pop r14
141243b89 pop r15
141243b8b ret 
141243b8c lea r8, [rip + 0x19dbe15]
141243b93 mov rdx, rsi
141243b96 call 0x1428d9518
141243b9b lea r8, [rip + 0x19dbe06]
141243ba2 mov rcx, rax
141243ba5 mov rdx, rsi
141243ba8 call 0x1428d9518
141243bad lea r8, [rip + 0x19dbe0c]
141243bb4 mov rdx, rbx
141243bb7 call 0x1428d9518
141243bbc lea r8, [rip + 0x19dbdfd]
141243bc3 mov rcx, rax
141243bc6 mov rdx, rbx
141243bc9 call 0x1428d9518
141243bce lea r8, [rip + 0x19dbe03]
141243bd5 mov rdx, rsi
141243bd8 call 0x1428d9518
141243bdd lea r8, [rip + 0x19dbdf4]
141243be4 mov rcx, rax
141243be7 mov rdx, rsi
141243bea call 0x1428d9518
141243bef lea r8, [rip + 0x19dbdfa]
141243bf6 mov rdx, rbx
141243bf9 call 0x1428d9518
141243bfe lea r8, [rip + 0x19dbdeb]
141243c05 mov rcx, rax
141243c08 mov rdx, rbx
141243c0b call 0x1428d9518
141243c10 int3 
