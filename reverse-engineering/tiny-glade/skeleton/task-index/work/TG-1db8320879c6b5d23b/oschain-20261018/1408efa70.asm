00000001408efa70: 4157                             push r15
00000001408efa72: 4156                             push r14
00000001408efa74: 4155                             push r13
00000001408efa76: 4154                             push r12
00000001408efa78: 56                               push rsi
00000001408efa79: 57                               push rdi
00000001408efa7a: 55                               push rbp
00000001408efa7b: 53                               push rbx
00000001408efa7c: 4883ec10                         sub rsp, 0x10
00000001408efa80: 4889542408                       mov qword ptr [rsp + 8], rdx
00000001408efa85: 488b01                           mov rax, qword ptr [rcx]
00000001408efa88: 4c8b4908                         mov r9, qword ptr [rcx + 8]
00000001408efa8c: 8b4914                           mov ecx, dword ptr [rcx + 0x14]
00000001408efa8f: 4c8b90f8000000                   mov r10, qword ptr [rax + 0xf8]
00000001408efa96: 488b9000010000                   mov rdx, qword ptr [rax + 0x100]
00000001408efa9d: 4d8d1c92                         lea r11, [r10 + rdx*4]
00000001408efaa1: 410fb610                         movzx edx, byte ptr [r8]
00000001408efaa5: 88542407                         mov byte ptr [rsp + 7], dl
00000001408efaa9: 4983c004                         add r8, 4
00000001408efaad: 80b80801000000                   cmp byte ptr [rax + 0x108], 0
00000001408efab4: 0f8423010000                     je 0x1408efbdd
00000001408efaba: 31ff                             xor edi, edi
00000001408efabc: f30f10055c4b0402                 movss xmm0, dword ptr [rip + 0x2044b5c]
00000001408efac4: 0f280dc56d1b02                   movaps xmm1, xmmword ptr [rip + 0x21b6dc5]
00000001408efacb: 4531f6                           xor r14d, r14d
00000001408eface: 31f6                             xor esi, esi
00000001408efad0: 4531e4                           xor r12d, r12d
00000001408efad3: 488b542408                       mov rdx, qword ptr [rsp + 8]
00000001408efad8: eb36                             jmp 0x1408efb10
00000001408efada: 660f1f440000                     nop word ptr [rax + rax]
00000001408efae0: 488b2a                           mov rbp, qword ptr [rdx]
00000001408efae3: f30f105514                       movss xmm2, dword ptr [rbp + 0x14]
00000001408efae8: f30f59d0                         mulss xmm2, xmm0
00000001408efaec: f30f585508                       addss xmm2, dword ptr [rbp + 8]
00000001408efaf1: f20f105d00                       movsd xmm3, qword ptr [rbp]
00000001408efaf6: f20f10650c                       movsd xmm4, qword ptr [rbp + 0xc]
00000001408efafb: 0f59e1                           mulps xmm4, xmm1
00000001408efafe: 0f58e3                           addps xmm4, xmm3
00000001408efb01: 41890c9f                         mov dword ptr [r15 + rbx*4], ecx
00000001408efb05: 410f136510                       movlps qword ptr [r13 + 0x10], xmm4
00000001408efb0a: f3410f115518                     movss dword ptr [r13 + 0x18], xmm2
00000001408efb10: 4939f4                           cmp r12, rsi
00000001408efb13: 0f8585000000                     jne 0x1408efb9e
00000001408efb19: 4d39da                           cmp r10, r11
00000001408efb1c: 0f840a020000                     je 0x1408efd2c
00000001408efb22: 498bb9a8010000                   mov rdi, qword ptr [r9 + 0x1a8]
00000001408efb29: 0f1f8000000000                   nop dword ptr [rax]
00000001408efb30: 418b32                           mov esi, dword ptr [r10]
00000001408efb33: 488d1cf6                         lea rbx, [rsi + rsi*8]
00000001408efb37: 488b74df10                       mov rsi, qword ptr [rdi + rbx*8 + 0x10]
00000001408efb3c: 4885f6                           test rsi, rsi
00000001408efb3f: 750f                             jne 0x1408efb50
00000001408efb41: 4983c204                         add r10, 4
00000001408efb45: 4d39da                           cmp r10, r11
00000001408efb48: 75e6                             jne 0x1408efb30
00000001408efb4a: e9dd010000                       jmp 0x1408efd2c
00000001408efb4f: 90                               nop 
00000001408efb50: 488d3cdf                         lea rdi, [rdi + rbx*8]
00000001408efb54: 488b9810010000                   mov rbx, qword ptr [rax + 0x110]
00000001408efb5b: 4c8bb818010000                   mov r15, qword ptr [rax + 0x118]
00000001408efb62: 4c8b6718                         mov r12, qword ptr [rdi + 0x18]
00000001408efb66: 488b7f38                         mov rdi, qword ptr [rdi + 0x38]
00000001408efb6a: 488b1cdf                         mov rbx, qword ptr [rdi + rbx*8]
00000001408efb6e: 48f7d3                           not rbx
00000001408efb71: 488d1c5b                         lea rbx, [rbx + rbx*2]
00000001408efb75: 48c1e304                         shl rbx, 4
00000001408efb79: 4d8b741c10                       mov r14, qword ptr [r12 + rbx + 0x10]
00000001408efb7e: 4a8b3cff                         mov rdi, qword ptr [rdi + r15*8]
00000001408efb82: 48f7d7                           not rdi
00000001408efb85: 488d1c7f                         lea rbx, [rdi + rdi*2]
00000001408efb89: 48c1e304                         shl rbx, 4
00000001408efb8d: 498b7c1c10                       mov rdi, qword ptr [r12 + rbx + 0x10]
00000001408efb92: 4d8b7c1c28                       mov r15, qword ptr [r12 + rbx + 0x28]
00000001408efb97: 4983c204                         add r10, 4
00000001408efb9b: 4531e4                           xor r12d, r12d
00000001408efb9e: 4489e3                           mov ebx, r12d
00000001408efba1: 4c8d2c5b                         lea r13, [rbx + rbx*2]
00000001408efba5: 49c1e504                         shl r13, 4
00000001408efba9: 4901fd                           add r13, rdi
00000001408efbac: 49ffc4                           inc r12
00000001408efbaf: 41803c1e00                       cmp byte ptr [r14 + rbx], 0
00000001408efbb4: 0f8526ffffff                     jne 0x1408efae0
00000001408efbba: f644240701                       test byte ptr [rsp + 7], 1
00000001408efbbf: 0f844bffffff                     je 0x1408efb10
00000001408efbc5: 41890c9f                         mov dword ptr [r15 + rbx*4], ecx
00000001408efbc9: 418b5808                         mov ebx, dword ptr [r8 + 8]
00000001408efbcd: 41895d18                         mov dword ptr [r13 + 0x18], ebx
00000001408efbd1: 498b18                           mov rbx, qword ptr [r8]
00000001408efbd4: 49895d10                         mov qword ptr [r13 + 0x10], rbx
00000001408efbd8: e933ffffff                       jmp 0x1408efb10
00000001408efbdd: 41bf08000000                     mov r15d, 8
00000001408efbe3: 31ff                             xor edi, edi
00000001408efbe5: f30f1005334a0402                 movss xmm0, dword ptr [rip + 0x2044a33]
00000001408efbed: 0f280d9c6c1b02                   movaps xmm1, xmmword ptr [rip + 0x21b6c9c]
00000001408efbf4: 4531f6                           xor r14d, r14d
00000001408efbf7: 31f6                             xor esi, esi
00000001408efbf9: 4531ed                           xor r13d, r13d
00000001408efbfc: eb34                             jmp 0x1408efc32
00000001408efbfe: 6690                             nop 
00000001408efc00: 488b542408                       mov rdx, qword ptr [rsp + 8]
00000001408efc05: 488b12                           mov rdx, qword ptr [rdx]
00000001408efc08: f30f105214                       movss xmm2, dword ptr [rdx + 0x14]
00000001408efc0d: f30f59d0                         mulss xmm2, xmm0
00000001408efc11: f30f585208                       addss xmm2, dword ptr [rdx + 8]
00000001408efc16: f20f101a                         movsd xmm3, qword ptr [rdx]
00000001408efc1a: f20f10620c                       movsd xmm4, qword ptr [rdx + 0xc]
00000001408efc1f: 0f59e1                           mulps xmm4, xmm1
00000001408efc22: 0f58e3                           addps xmm4, xmm3
00000001408efc25: 41890c9c                         mov dword ptr [r12 + rbx*4], ecx
00000001408efc29: 0f136510                         movlps qword ptr [rbp + 0x10], xmm4
00000001408efc2d: f30f115518                       movss dword ptr [rbp + 0x18], xmm2
00000001408efc32: 4939f5                           cmp r13, rsi
00000001408efc35: 0f85aa000000                     jne 0x1408efce5
00000001408efc3b: 4d39da                           cmp r10, r11
00000001408efc3e: 0f84e8000000                     je 0x1408efd2c
00000001408efc44: 4d8bb900010000                   mov r15, qword ptr [r9 + 0x100]
00000001408efc4b: 0f1f440000                       nop dword ptr [rax + rax]
00000001408efc50: 418b32                           mov esi, dword ptr [r10]
00000001408efc53: 488d3cb6                         lea rdi, [rsi + rsi*4]
00000001408efc57: 48c1e705                         shl rdi, 5
00000001408efc5b: 498b743f58                       mov rsi, qword ptr [r15 + rdi + 0x58]
00000001408efc60: 4885f6                           test rsi, rsi
00000001408efc63: 751b                             jne 0x1408efc80
00000001408efc65: 4983c204                         add r10, 4
00000001408efc69: 4d39da                           cmp r10, r11
00000001408efc6c: 75e2                             jne 0x1408efc50
00000001408efc6e: e9b9000000                       jmp 0x1408efd2c
00000001408efc73: 666666662e0f1f840000000000       nop word ptr cs:[rax + rax]
00000001408efc80: 4901ff                           add r15, rdi
00000001408efc83: 418bbf94000000                   mov edi, dword ptr [r15 + 0x94]
00000001408efc8a: 498b99a8010000                   mov rbx, qword ptr [r9 + 0x1a8]
00000001408efc91: 488d3cff                         lea rdi, [rdi + rdi*8]
00000001408efc95: 4c8bb010010000                   mov r14, qword ptr [rax + 0x110]
00000001408efc9c: 4c8ba018010000                   mov r12, qword ptr [rax + 0x118]
00000001408efca3: 4c8b6cfb18                       mov r13, qword ptr [rbx + rdi*8 + 0x18]
00000001408efca8: 488b7cfb38                       mov rdi, qword ptr [rbx + rdi*8 + 0x38]
00000001408efcad: 4a8b1cf7                         mov rbx, qword ptr [rdi + r14*8]
00000001408efcb1: 48f7d3                           not rbx
00000001408efcb4: 488d1c5b                         lea rbx, [rbx + rbx*2]
00000001408efcb8: 48c1e304                         shl rbx, 4
00000001408efcbc: 4d8b741d10                       mov r14, qword ptr [r13 + rbx + 0x10]
00000001408efcc1: 4a8b3ce7                         mov rdi, qword ptr [rdi + r12*8]
00000001408efcc5: 48f7d7                           not rdi
00000001408efcc8: 488d1c7f                         lea rbx, [rdi + rdi*2]
00000001408efccc: 48c1e304                         shl rbx, 4
00000001408efcd0: 498b7c1d10                       mov rdi, qword ptr [r13 + rbx + 0x10]
00000001408efcd5: 4d8b641d28                       mov r12, qword ptr [r13 + rbx + 0x28]
00000001408efcda: 4d8b7f50                         mov r15, qword ptr [r15 + 0x50]
00000001408efcde: 4983c204                         add r10, 4
00000001408efce2: 4531ed                           xor r13d, r13d
00000001408efce5: 4c89eb                           mov rbx, r13
00000001408efce8: 48c1e304                         shl rbx, 4
00000001408efcec: 418b5c1f08                       mov ebx, dword ptr [r15 + rbx + 8]
00000001408efcf1: 488d2c5b                         lea rbp, [rbx + rbx*2]
00000001408efcf5: 48c1e504                         shl rbp, 4
00000001408efcf9: 4801fd                           add rbp, rdi
00000001408efcfc: 49ffc5                           inc r13
00000001408efcff: 41803c1e00                       cmp byte ptr [r14 + rbx], 0
00000001408efd04: 0f85f6feffff                     jne 0x1408efc00
00000001408efd0a: f644240701                       test byte ptr [rsp + 7], 1
00000001408efd0f: 0f841dffffff                     je 0x1408efc32
00000001408efd15: 41890c9c                         mov dword ptr [r12 + rbx*4], ecx
00000001408efd19: 418b5008                         mov edx, dword ptr [r8 + 8]
00000001408efd1d: 895518                           mov dword ptr [rbp + 0x18], edx
00000001408efd20: 498b10                           mov rdx, qword ptr [r8]
00000001408efd23: 48895510                         mov qword ptr [rbp + 0x10], rdx
00000001408efd27: e906ffffff                       jmp 0x1408efc32
00000001408efd2c: 4883c410                         add rsp, 0x10
00000001408efd30: 5b                               pop rbx
00000001408efd31: 5d                               pop rbp
00000001408efd32: 5f                               pop rdi
00000001408efd33: 5e                               pop rsi
00000001408efd34: 415c                             pop r12
00000001408efd36: 415d                             pop r13
00000001408efd38: 415e                             pop r14
00000001408efd3a: 415f                             pop r15
00000001408efd3c: c3                               ret 
