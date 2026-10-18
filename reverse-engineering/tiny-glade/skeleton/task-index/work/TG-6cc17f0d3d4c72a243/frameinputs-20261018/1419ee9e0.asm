1419ee9e0: push rsi
1419ee9e1: push rdi
1419ee9e2: sub rsp, 0x78
1419ee9e6: movaps xmmword ptr [rsp + 0x60], xmm8
1419ee9ec: movaps xmmword ptr [rsp + 0x50], xmm7
1419ee9f1: movaps xmmword ptr [rsp + 0x40], xmm6
1419ee9f6: mov rdi, r8
1419ee9f9: mov rsi, rcx
1419ee9fc: movss xmm6, dword ptr [r9 + 4]
1419eea02: mov rcx, r8
1419eea05: call 0x1408e3250
1419eea0a: xor ecx, ecx
1419eea0c: mov edx, 0
1419eea11: test al, al
1419eea13: je 0x1419eeb10
1419eea19: mov rcx, rdi
1419eea1c: call 0x1408e2690
1419eea21: mov qword ptr [rsp + 0x20], rax
1419eea26: lea rcx, [rsp + 0x20]
1419eea2b: call 0x1400885d0
1419eea30: movd xmm0, eax
1419eea34: shr rax, 0x20
1419eea38: movd xmm1, eax
1419eea3c: movdqa xmm2, xmm0
1419eea40: divss xmm2, xmm1
1419eea44: divss xmm1, xmm0
1419eea48: movaps xmm0, xmm2
1419eea4b: cmpunordss xmm0, xmm2
1419eea50: movaps xmm3, xmm0
1419eea53: andps xmm3, xmm1
1419eea56: maxss xmm1, xmm2
1419eea5a: andnps xmm0, xmm1
1419eea5d: orps xmm0, xmm3
1419eea60: xor ecx, ecx
1419eea62: movss xmm1, dword ptr [rip + 0x14bb13a]
1419eea6a: ucomiss xmm1, xmm0
1419eea6d: mov edx, 0
1419eea72: jbe 0x1419eeb10
1419eea78: lea rcx, [rsp + 0x34]
1419eea7d: mov rdx, rdi
1419eea80: call 0x1408e3220
1419eea85: mov rcx, rdi
1419eea88: call 0x1408e3780
1419eea8d: movss dword ptr [rsp + 0x2c], xmm0
1419eea93: movss dword ptr [rsp + 0x30], xmm1
1419eea99: lea rcx, [rsp + 0x20]
1419eea9e: lea rdx, [rsp + 0x2c]
1419eeaa3: call 0x140c9cdc0
1419eeaa8: movss xmm7, dword ptr [rsp + 0x34]
1419eeaae: addss xmm7, dword ptr [rsp + 0x20]
1419eeab4: movsd xmm0, qword ptr [rsp + 0x38]
1419eeaba: movsd xmm8, qword ptr [rsp + 0x24]
1419eeac1: addps xmm8, xmm0
1419eeac5: mov rcx, rdi
1419eeac8: call 0x1408e3260
1419eeacd: xorps xmm1, xmm1
1419eead0: mulss xmm1, xmm0
1419eead4: addss xmm7, xmm1
1419eead8: unpcklps xmm0, xmm1
1419eeadb: addps xmm0, xmm8
1419eeadf: xorps xmm1, xmm1
1419eeae2: movss xmm1, xmm7
1419eeae6: shufps xmm1, xmm0, 0x4c
1419eeaea: shufps xmm1, xmm1, 0x78
1419eeaee: movss xmm0, dword ptr [rdi + 0x2c]
1419eeaf3: movaps xmmword ptr [rsi + 0x10], xmm1
1419eeaf7: movss dword ptr [rsi + 0x20], xmm6
1419eeafc: movss dword ptr [rsi + 0x24], xmm0
1419eeb01: mov qword ptr [rsi + 0x28], 0x3e800000
1419eeb09: mov ecx, 1
1419eeb0e: xor edx, edx
1419eeb10: mov qword ptr [rsi], rcx
1419eeb13: mov qword ptr [rsi + 8], rdx
1419eeb17: mov rax, rsi
1419eeb1a: movaps xmm6, xmmword ptr [rsp + 0x40]
1419eeb1f: movaps xmm7, xmmword ptr [rsp + 0x50]
1419eeb24: movaps xmm8, xmmword ptr [rsp + 0x60]
1419eeb2a: add rsp, 0x78
1419eeb2e: pop rdi
1419eeb2f: pop rsi
1419eeb30: ret 
