140c029f0: 55                               push     rbp
140c029f1: 56                               push     rsi
140c029f2: 57                               push     rdi
140c029f3: 53                               push     rbx
140c029f4: 4881ec08010000                   sub      rsp, 0x108
140c029fb: 488dac2480000000                 lea      rbp, [rsp + 0x80]
140c02a03: 48c78580000000feffffff           mov      qword ptr [rbp + 0x80], 0xfffffffffffffffe
140c02a0e: 4889d7                           mov      rdi, rdx
140c02a11: 4889ce                           mov      rsi, rcx
140c02a14: c6450029                         mov      byte ptr [rbp], 0x29
140c02a18: 488d0d2124a302                   lea      rcx, [rip + 0x2a32421]
140c02a1f: 4889eb                           mov      rbx, rbp
140c02a22: 41b801000000                     mov      r8d, 1
140c02a28: 4889da                           mov      rdx, rbx
140c02a2b: e850e9cb00                       call     0x1418c1380
140c02a30: 488b057199a202                   mov      rax, qword ptr [rip + 0x2a29971]
140c02a37: 488b00                           mov      rax, qword ptr [rax]
140c02a3a: 4883f804                         cmp      rax, 4
140c02a3e: 0f8685000000                     jbe      0x140c02ac9
140c02a44: 488d0dd555f101                   lea      rcx, [rip + 0x1f155d5]
140c02a4b: e830b3a7ff                       call     0x14067dd80
140c02a50: 0f1000                           movups   xmm0, xmmword ptr [rax]
140c02a53: 8b4010                           mov      eax, dword ptr [rax + 0x10]
140c02a56: 48c7453005000000                 mov      qword ptr [rbp + 0x30], 5
140c02a5e: 488d0dd355f101                   lea      rcx, [rip + 0x1f155d3]
140c02a65: 48894d38                         mov      qword ptr [rbp + 0x38], rcx
140c02a69: 48c7454020000000                 mov      qword ptr [rbp + 0x40], 0x20
140c02a71: 488d159855f101                   lea      rdx, [rip + 0x1f15598]
140c02a78: 48895550                         mov      qword ptr [rbp + 0x50], rdx
140c02a7c: 48c7455801000000                 mov      qword ptr [rbp + 0x58], 1
140c02a84: 48c7456008000000                 mov      qword ptr [rbp + 0x60], 8
140c02a8c: 48c7450000000000                 mov      qword ptr [rbp], 0
140c02a94: 0f57c9                           xorps    xmm1, xmm1
140c02a97: 0f114d68                         movups   xmmword ptr [rbp + 0x68], xmm1
140c02a9b: 48894d08                         mov      qword ptr [rbp + 8], rcx
140c02a9f: 48c7451020000000                 mov      qword ptr [rbp + 0x10], 0x20
140c02aa7: 48c7451800000000                 mov      qword ptr [rbp + 0x18], 0
140c02aaf: 0f114520                         movups   xmmword ptr [rbp + 0x20], xmm0
140c02ab3: c7454801000000                   mov      dword ptr [rbp + 0x48], 1
140c02aba: 89454c                           mov      dword ptr [rbp + 0x4c], eax
140c02abd: 488d4db0                         lea      rcx, [rbp - 0x50]
140c02ac1: 4889ea                           mov      rdx, rbp
140c02ac4: e867781400                       call     0x140d4a330
140c02ac9: 488b8758060000                   mov      rax, qword ptr [rdi + 0x658]
140c02ad0: 488b8040030000                   mov      rax, qword ptr [rax + 0x340]
140c02ad7: 483d00000100                     cmp      rax, 0x10000
140c02add: 735e                             jae      0x140c02b3d
140c02adf: 48c7450000000400                 mov      qword ptr [rbp], 0x40000
140c02ae7: c7450833000200                   mov      dword ptr [rbp + 8], 0x20033
140c02aee: 6689450c                         mov      word ptr [rbp + 0xc], ax
140c02af2: c6450e02                         mov      byte ptr [rbp + 0xe], 2
140c02af6: 48895c2420                       mov      qword ptr [rsp + 0x20], rbx
140c02afb: 4c8d055655f101                   lea      r8, [rip + 0x1f15556]
140c02b02: 488d4db0                         lea      rcx, [rbp - 0x50]
140c02b06: 41b918000000                     mov      r9d, 0x18
140c02b0c: 4889fa                           mov      rdx, rdi
140c02b0f: e8ac0d0000                       call     0x140c038c0
140c02b14: 837db00b                         cmp      dword ptr [rbp - 0x50], 0xb
140c02b18: 754b                             jne      0x140c02b65
140c02b1a: 488b45d8                         mov      rax, qword ptr [rbp - 0x28]
140c02b1e: 48894620                         mov      qword ptr [rsi + 0x20], rax
140c02b22: 0f1045b8                         movups   xmm0, xmmword ptr [rbp - 0x48]
140c02b26: 0f104dc8                         movups   xmm1, xmmword ptr [rbp - 0x38]
140c02b2a: 0f114e10                         movups   xmmword ptr [rsi + 0x10], xmm1
140c02b2e: 0f1106                           movups   xmmword ptr [rsi], xmm0
140c02b31: 4881c408010000                   add      rsp, 0x108
140c02b38: 5b                               pop      rbx
140c02b39: 5f                               pop      rdi
140c02b3a: 5e                               pop      rsi
140c02b3b: 5d                               pop      rbp
140c02b3c: c3                               ret      
140c02b3d: 488d058ccdf001                   lea      rax, [rip + 0x1f0cd8c]
140c02b44: 4889442420                       mov      qword ptr [rsp + 0x20], rax
140c02b49: 488d0d78cdf001                   lea      rcx, [rip + 0x1f0cd78]
140c02b50: 4c8d0d11ccf001                   lea      r9, [rip + 0x1f0cc11]
140c02b57: 4c8d45b0                         lea      r8, [rbp - 0x50]
140c02b5b: ba05000000                       mov      edx, 5
140c02b60: e8fb6bcd01                       call     0x1428d9760
140c02b65: 0f1045f0                         movups   xmm0, xmmword ptr [rbp - 0x10]
140c02b69: 0f294540                         movaps   xmmword ptr [rbp + 0x40], xmm0
140c02b6d: 0f1045b0                         movups   xmm0, xmmword ptr [rbp - 0x50]
140c02b71: 0f104dc0                         movups   xmm1, xmmword ptr [rbp - 0x40]
140c02b75: 0f1055d0                         movups   xmm2, xmmword ptr [rbp - 0x30]
140c02b79: 0f105de0                         movups   xmm3, xmmword ptr [rbp - 0x20]
140c02b7d: 0f295d30                         movaps   xmmword ptr [rbp + 0x30], xmm3
140c02b81: 0f295520                         movaps   xmmword ptr [rbp + 0x20], xmm2
140c02b85: 0f294d10                         movaps   xmmword ptr [rbp + 0x10], xmm1
140c02b89: 0f294500                         movaps   xmmword ptr [rbp], xmm0
140c02b8d: 488d05dc54f101                   lea      rax, [rip + 0x1f154dc]
140c02b94: 4889442420                       mov      qword ptr [rsp + 0x20], rax
140c02b99: 488d0d1052f101                   lea      rcx, [rip + 0x1f15210]
140c02ba0: 4c8d0de951f101                   lea      r9, [rip + 0x1f151e9]
140c02ba7: 4989e8                           mov      r8, rbp
140c02baa: ba2b000000                       mov      edx, 0x2b
140c02baf: e8ac6bcd01                       call     0x1428d9760
140c02bb4: 0f0b                             ud2      
140c02bb6: 662e0f1f840000000000             nop      word ptr cs:[rax + rax]
140c02bc0: 4889542410                       mov      qword ptr [rsp + 0x10], rdx
140c02bc5: 55                               push     rbp
140c02bc6: 56                               push     rsi
140c02bc7: 57                               push     rdi
140c02bc8: 53                               push     rbx
140c02bc9: 4883ec28                         sub      rsp, 0x28
140c02bcd: 488daa80000000                   lea      rbp, [rdx + 0x80]
140c02bd4: 4889e9                           mov      rcx, rbp
140c02bd7: e894bebcff                       call     0x1407cea70
140c02bdc: 90                               nop      
140c02bdd: 4883c428                         add      rsp, 0x28
140c02be1: 5b                               pop      rbx
140c02be2: 5f                               pop      rdi
140c02be3: 5e                               pop      rsi
140c02be4: 5d                               pop      rbp
140c02be5: c3                               ret      
