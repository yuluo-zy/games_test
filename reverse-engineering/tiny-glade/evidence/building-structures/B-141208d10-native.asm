141208d10 push rsi
141208d11 push rdi
141208d12 push rbp
141208d13 push rbx
141208d14 sub rsp, 0x28
141208d18 mov r10d, r8d
141208d1b mov r8, qword ptr [rsp + 0x70]
141208d20 mov edi, dword ptr [rdx + 0x18]
141208d23 test edi, edi
141208d25 je 0x141208de7
141208d2b mov r9, rdx
141208d2e add r10d, dword ptr [rdx + 0x10]
141208d32 mov ebx, edi
141208d34 not ebx
141208d36 lea eax, [r10 - 0x80000000]
141208d3d or eax, ebx
141208d3f je 0x141208ddf
141208d45 mov eax, r10d
141208d48 cdq 
141208d49 idiv edi
141208d4b mov ebp, edi
141208d4d neg ebp
141208d4f cmovs ebp, edi
141208d52 mov eax, edx
141208d54 sar eax, 0x1f
141208d57 and eax, ebp
141208d59 add eax, edx
141208d5b movsxd r11, eax
141208d5e mov rsi, qword ptr [r9 + 8]
141208d62 cmp rsi, r11
141208d65 jbe 0x141208def
141208d6b lea eax, [r10 - 0x7fffffff]
141208d72 or ebx, eax
141208d74 je 0x141208ddf
141208d76 inc r10d
141208d79 mov eax, r10d
141208d7c cdq 
141208d7d idiv edi
141208d7f mov eax, edx
141208d81 sar eax, 0x1f
141208d84 and eax, ebp
141208d86 add eax, edx
141208d88 cdqe 
141208d8a cmp rsi, rax
141208d8d jbe 0x141208dfa
141208d8f mov rdx, qword ptr [r9]
141208d92 lea r8, [r11 + r11*2]
141208d96 movsd xmm0, qword ptr [rdx + r8*4]
141208d9c movss xmm1, dword ptr [rdx + r8*4 + 8]
141208da3 lea rax, [rax + rax*2]
141208da7 movss xmm2, dword ptr [rdx + rax*4 + 8]
141208dad subss xmm2, xmm1
141208db1 mulss xmm2, xmm3
141208db5 addss xmm2, xmm1
141208db9 movsd xmm1, qword ptr [rdx + rax*4]
141208dbe subps xmm1, xmm0
141208dc1 movsldup xmm3, xmm3
141208dc5 mulps xmm3, xmm1
141208dc8 addps xmm3, xmm0
141208dcb movlps qword ptr [rcx], xmm3
141208dce movss dword ptr [rcx + 8], xmm2
141208dd3 mov rax, rcx
141208dd6 add rsp, 0x28
141208dda pop rbx
141208ddb pop rbp
141208ddc pop rdi
141208ddd pop rsi
141208dde ret 
141208ddf mov rcx, r8
141208de2 call 0x1428d99a0
141208de7 mov rcx, r8
141208dea call 0x1428d9a20
141208def mov rcx, r11
141208df2 mov rdx, rsi
141208df5 call 0x1428d9518
141208dfa mov rcx, rax
141208dfd mov rdx, rsi
141208e00 call 0x1428d9518
141208e05 int3 
