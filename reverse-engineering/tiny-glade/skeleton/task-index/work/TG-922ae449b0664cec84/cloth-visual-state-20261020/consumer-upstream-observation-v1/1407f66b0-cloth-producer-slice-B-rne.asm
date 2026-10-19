1407f7e39: 488b85b8010000 mov rax, qword ptr [rbp + 0x1b8]
1407f7e40: 83b85802000002 cmp dword ptr [rax + 0x258], 2
1407f7e47: 0f8351040000 jae 0x1407f829e
1407f7e4d: 488b9508020000 mov rdx, qword ptr [rbp + 0x208]
1407f7e54: 8b4ae4 mov ecx, dword ptr [rdx - 0x1c]
1407f7e57: 69c1b6aaaaaa imul eax, ecx, 0xaaaaaab6
1407f7e5d: 054b885655 add eax, 0x5556884b
1407f7e62: 3d55555555 cmp eax, 0x55555555
1407f7e67: 0f871b010000 ja 0x1407f7f88
1407f7e6d: 0fb642fc movzx eax, byte ptr [rdx - 4]
1407f7e71: 83f803 cmp eax, 3
1407f7e74: 0f830e010000 jae 0x1407f7f88
1407f7e7a: 83f801 cmp eax, 1
1407f7e7d: 488b9d68010000 mov rbx, qword ptr [rbp + 0x168]
1407f7e84: 740d je 0x1407f7e93
1407f7e86: 83f802 cmp eax, 2
1407f7e89: 0f855a080000 jne 0x1407f86e9
1407f7e8f: 440f28c7 movaps xmm8, xmm7
1407f7e93: 0fb6056fcde302 movzx eax, byte ptr [rip + 0x2e3cd6f]
1407f7e9a: b950000000 mov ecx, 0x50
1407f7e9f: ba10000000 mov edx, 0x10
1407f7ea4: e867bde1ff call 0x140613c10 ; unknown
1407f7ea9: 4885c0 test rax, rax
1407f7eac: 0f8426080000 je 0x1407f86d8
1407f7eb2: 4889c7 mov rdi, rax
1407f7eb5: 0f280dd43d2902 movaps xmm1, xmmword ptr [rip + 0x2293dd4]
1407f7ebc: 440f59d9 mulps xmm11, xmm1
1407f7ec0: 410f28c2 movaps xmm0, xmm10
1407f7ec4: 0f5905d53d2902 mulps xmm0, xmmword ptr [rip + 0x2293dd5]
1407f7ecb: 410f58c3 addps xmm0, xmm11
1407f7ecf: 0fc6c0d2 shufps xmm0, xmm0, 0xd2
1407f7ed3: 440f59f8 mulps xmm15, xmm0
1407f7ed7: 0f59f1 mulps xmm6, xmm1
1407f7eda: 410f28c2 movaps xmm0, xmm10
1407f7ede: 0f59c1 mulps xmm0, xmm1
1407f7ee1: f30f16c8 movshdup xmm1, xmm0
1407f7ee5: f30f58c8 addss xmm1, xmm0
1407f7ee9: 0f12c0 movhlps xmm0, xmm0
1407f7eec: f30f58c1 addss xmm0, xmm1
1407f7ef0: f30f58c0 addss xmm0, xmm0
1407f7ef4: 0fc6c000 shufps xmm0, xmm0, 0
1407f7ef8: 410f59c2 mulps xmm0, xmm10
1407f7efc: 0f58c6 addps xmm0, xmm6
1407f7eff: 410f58c7 addps xmm0, xmm15
1407f7f03: 0f28c8 movaps xmm1, xmm0
1407f7f06: 660f15c8 unpckhpd xmm1, xmm0
1407f7f0a: f3410f58c9 addss xmm1, xmm9
1407f7f0f: 440f2910 movaps xmmword ptr [rax], xmm10
1407f7f13: 0f5885b0000000 addps xmm0, xmmword ptr [rbp + 0xb0]
1407f7f1a: 0f134010 movlps qword ptr [rax + 0x10], xmm0
1407f7f1e: f30f114818 movss dword ptr [rax + 0x18], xmm1
1407f7f23: 48b80000803f0000803f movabs rax, 0x3f8000003f800000
1407f7f2d: 4889471c mov qword ptr [rdi + 0x1c], rax
1407f7f31: c747240000803f mov dword ptr [rdi + 0x24], 0x3f800000
1407f7f38: 488b85c0000000 mov rax, qword ptr [rbp + 0xc0]
1407f7f3f: 48894730 mov qword ptr [rdi + 0x30], rax
1407f7f43: f3440f114738 movss dword ptr [rdi + 0x38], xmm8
1407f7f49: f3440f11673c movss dword ptr [rdi + 0x3c], xmm12
1407f7f4f: 895f40 mov dword ptr [rdi + 0x40], ebx
1407f7f52: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7f59: 488b8810010000 mov rcx, qword ptr [rax + 0x110]
1407f7f60: 4885c9 test rcx, rcx
1407f7f63: 7410 je 0x1407f7f75
1407f7f65: ba50000000 mov edx, 0x50
1407f7f6a: 41b810000000 mov r8d, 0x10
1407f7f70: e8abbce1ff call 0x140613c20 ; unknown
1407f7f75: 488b8500020000 mov rax, qword ptr [rbp + 0x200]
1407f7f7c: 4889b810010000 mov qword ptr [rax + 0x110], rdi
