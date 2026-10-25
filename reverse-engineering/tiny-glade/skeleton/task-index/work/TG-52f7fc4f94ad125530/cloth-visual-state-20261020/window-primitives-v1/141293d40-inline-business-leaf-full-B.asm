141293d40: 55 push rbp
141293d41: 4157 push r15
141293d43: 4156 push r14
141293d45: 4155 push r13
141293d47: 4154 push r12
141293d49: 56 push rsi
141293d4a: 57 push rdi
141293d4b: 53 push rbx
141293d4c: 4881ecf8000000 sub rsp, 0xf8
141293d53: 488dac2480000000 lea rbp, [rsp + 0x80]
141293d5b: 440f296d60 movaps xmmword ptr [rbp + 0x60], xmm13
141293d60: 440f296550 movaps xmmword ptr [rbp + 0x50], xmm12
141293d65: 440f295d40 movaps xmmword ptr [rbp + 0x40], xmm11
141293d6a: 440f295530 movaps xmmword ptr [rbp + 0x30], xmm10
141293d6f: 440f294d20 movaps xmmword ptr [rbp + 0x20], xmm9
141293d74: 440f294510 movaps xmmword ptr [rbp + 0x10], xmm8
141293d79: 0f297d00 movaps xmmword ptr [rbp], xmm7
141293d7d: 0f2975f0 movaps xmmword ptr [rbp - 0x10], xmm6
141293d81: 48c745e8feffffff mov qword ptr [rbp - 0x18], 0xfffffffffffffffe
141293d89: 4c8b02 mov r8, qword ptr [rdx]
141293d8c: 4c8b4a08 mov r9, qword ptr [rdx + 8]
141293d90: 488b4120 mov rax, qword ptr [rcx + 0x20]
141293d94: 4889c3 mov rbx, rax
141293d97: 482b5928 sub rbx, qword ptr [rcx + 0x28]
141293d9b: 0f84de010000 je 0x141293f7f
141293da1: 4c8945d8 mov qword ptr [rbp - 0x28], r8
141293da5: 4c8b7138 mov r14, qword ptr [rcx + 0x38]
141293da9: 488b5210 mov rdx, qword ptr [rdx + 0x10]
141293dad: 4d8d7e1c lea r15, [r14 + 0x1c]
141293db1: 4c894de0 mov qword ptr [rbp - 0x20], r9
141293db5: 4f8d0449 lea r8, [r9 + r9*2]
141293db9: 49c1e004 shl r8, 4
141293dbd: 488d0440 lea rax, [rax + rax*2]
141293dc1: 4c8d248504000000 lea r12, [rax*4 + 4]
141293dc9: 4c8b6910 mov r13, qword ptr [rcx + 0x10]
141293dcd: 4d01e5 add r13, r12
141293dd0: 4c0321 add r12, qword ptr [rcx]
141293dd3: 498d3410 lea rsi, [r8 + rdx]
141293dd7: 4883c61c add rsi, 0x1c
141293ddb: 31ff xor edi, edi
141293ddd: 0f28358c937301 movaps xmm6, xmmword ptr [rip + 0x173938c]
141293de4: 0f283d95937301 movaps xmm7, xmmword ptr [rip + 0x1739395]
141293deb: 440f28059d937301 movaps xmm8, xmmword ptr [rip + 0x173939d]
141293df3: 31c0 xor eax, eax
141293df5: 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
141293e00: 488945d0 mov qword ptr [rbp - 0x30], rax
141293e04: f3450f10643cfc movss xmm12, dword ptr [r12 + rdi - 4]
141293e0b: f2450f102c3c movsd xmm13, qword ptr [r12 + rdi]
141293e11: f3410f10443dfc movss xmm0, dword ptr [r13 + rdi - 4]
141293e18: 498b443d00 mov rax, qword ptr [r13 + rdi]
141293e1d: f30f1145c4 movss dword ptr [rbp - 0x3c], xmm0
141293e22: 488945c8 mov qword ptr [rbp - 0x38], rax
141293e26: 450f281e movaps xmm11, xmmword ptr [r14]
141293e2a: f2450f105610 movsd xmm10, qword ptr [r14 + 0x10]
141293e30: f3450f104e18 movss xmm9, dword ptr [r14 + 0x18]
141293e36: 418b4710 mov eax, dword ptr [r15 + 0x10]
141293e3a: 8945c0 mov dword ptr [rbp - 0x40], eax
141293e3d: 410f1007 movups xmm0, xmmword ptr [r15]
141293e41: 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
141293e45: 488d4da0 lea rcx, [rbp - 0x60]
141293e49: 488d55c4 lea rdx, [rbp - 0x3c]
141293e4d: e8eecb9fff call 0x140c90a40
141293e52: 410f28d3 movaps xmm2, xmm11
141293e56: 410f58d3 addps xmm2, xmm11
141293e5a: 0fc6d2ff shufps xmm2, xmm2, 0xff
141293e5e: 450f16e5 movlhps xmm12, xmm13
141293e62: 410f28c4 movaps xmm0, xmm12
141293e66: 410fc6c5d8 shufps xmm0, xmm13, 0xd8
141293e6b: 450fc6e558 shufps xmm12, xmm13, 0x58
141293e70: 410f28cb movaps xmm1, xmm11
141293e74: 410fc6cbd2 shufps xmm1, xmm11, 0xd2
141293e79: 0f59c8 mulps xmm1, xmm0
141293e7c: 0f28d8 movaps xmm3, xmm0
141293e7f: 0fc6d8d2 shufps xmm3, xmm0, 0xd2
141293e83: 410f59db mulps xmm3, xmm11
141293e87: 0f5ccb subps xmm1, xmm3
141293e8a: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
141293e8e: 0f59ca mulps xmm1, xmm2
141293e91: 410f28d3 movaps xmm2, xmm11
141293e95: 410f59d3 mulps xmm2, xmm11
141293e99: 0f28da movaps xmm3, xmm2
141293e9c: 0fc6daff shufps xmm3, xmm2, 0xff
141293ea0: f30f16e2 movshdup xmm4, xmm2
141293ea4: f30f58e2 addss xmm4, xmm2
141293ea8: 0f12d2 movhlps xmm2, xmm2
141293eab: f30f58d4 addss xmm2, xmm4
141293eaf: 0fc6d200 shufps xmm2, xmm2, 0
141293eb3: 0f5cda subps xmm3, xmm2
141293eb6: 410f59dc mulps xmm3, xmm12
141293eba: 410f59c3 mulps xmm0, xmm11
141293ebe: f30f16d0 movshdup xmm2, xmm0
141293ec2: f30f58d0 addss xmm2, xmm0
141293ec6: 0f12c0 movhlps xmm0, xmm0
141293ec9: f30f58c2 addss xmm0, xmm2
141293ecd: f30f58c0 addss xmm0, xmm0
141293ed1: 0fc6c000 shufps xmm0, xmm0, 0
141293ed5: 410f59c3 mulps xmm0, xmm11
141293ed9: 0f58c3 addps xmm0, xmm3
141293edc: 0f58c1 addps xmm0, xmm1
141293edf: 440f58d0 addps xmm10, xmm0
141293ee3: 0f12c0 movhlps xmm0, xmm0
141293ee6: f3440f58c8 addss xmm9, xmm0
141293eeb: 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
141293eef: 410f28cb movaps xmm1, xmm11
141293ef3: 410fc6cb00 shufps xmm1, xmm11, 0
141293ef8: 410f28d3 movaps xmm2, xmm11
141293efc: 410fc6d355 shufps xmm2, xmm11, 0x55
141293f01: 410f28db movaps xmm3, xmm11
141293f05: 410fc6dbaa shufps xmm3, xmm11, 0xaa
141293f0a: 450fc6dbff shufps xmm11, xmm11, 0xff
141293f0f: 440f59d8 mulps xmm11, xmm0
141293f13: 0f28e0 movaps xmm4, xmm0
141293f16: 0fc6e01b shufps xmm4, xmm0, 0x1b
141293f1a: 0f59cc mulps xmm1, xmm4
141293f1d: 0fc6e4b1 shufps xmm4, xmm4, 0xb1
141293f21: 0f59e2 mulps xmm4, xmm2
141293f24: 0f59ce mulps xmm1, xmm6
141293f27: 410f58cb addps xmm1, xmm11
141293f2b: 0fc6c0b1 shufps xmm0, xmm0, 0xb1
141293f2f: 0f59c3 mulps xmm0, xmm3
141293f32: 0f59e7 mulps xmm4, xmm7
141293f35: 410f59c0 mulps xmm0, xmm8
141293f39: 0f58c4 addps xmm0, xmm4
141293f3c: 0f58c1 addps xmm0, xmm1
141293f3f: 0f2944bee4 movaps xmmword ptr [rsi + rdi*4 - 0x1c], xmm0
141293f44: 440f1354bef4 movlps qword ptr [rsi + rdi*4 - 0xc], xmm10
141293f4a: f3440f114cbefc movss dword ptr [rsi + rdi*4 - 4], xmm9
141293f51: 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
141293f55: 0f1104be movups xmmword ptr [rsi + rdi*4], xmm0
141293f59: 8b45c0 mov eax, dword ptr [rbp - 0x40]
141293f5c: 8944be10 mov dword ptr [rsi + rdi*4 + 0x10], eax
141293f60: 488b45d0 mov rax, qword ptr [rbp - 0x30]
141293f64: 48ffc8 dec rax
141293f67: 4883c70c add rdi, 0xc
141293f6b: 4839c3 cmp rbx, rax
141293f6e: 0f858cfeffff jne 0x141293e00
141293f74: 4c8b4de0 mov r9, qword ptr [rbp - 0x20]
141293f78: 4929c1 sub r9, rax
141293f7b: 4c8b45d8 mov r8, qword ptr [rbp - 0x28]
141293f7f: 4d8908 mov qword ptr [r8], r9
141293f82: 0f2875f0 movaps xmm6, xmmword ptr [rbp - 0x10]
141293f86: 0f287d00 movaps xmm7, xmmword ptr [rbp]
141293f8a: 440f284510 movaps xmm8, xmmword ptr [rbp + 0x10]
141293f8f: 440f284d20 movaps xmm9, xmmword ptr [rbp + 0x20]
141293f94: 440f285530 movaps xmm10, xmmword ptr [rbp + 0x30]
141293f99: 440f285d40 movaps xmm11, xmmword ptr [rbp + 0x40]
141293f9e: 440f286550 movaps xmm12, xmmword ptr [rbp + 0x50]
141293fa3: 440f286d60 movaps xmm13, xmmword ptr [rbp + 0x60]
141293fa8: 4881c4f8000000 add rsp, 0xf8
141293faf: 5b pop rbx
141293fb0: 5f pop rdi
141293fb1: 5e pop rsi
141293fb2: 415c pop r12
141293fb4: 415d pop r13
141293fb6: 415e pop r14
141293fb8: 415f pop r15
141293fba: 5d pop rbp
141293fbb: c3 ret 
141293fbc: 0f1f4000 nop dword ptr [rax]
141293fc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
141293fc5: 55 push rbp
141293fc6: 4157 push r15
141293fc8: 4156 push r14
141293fca: 4155 push r13
141293fcc: 4154 push r12
141293fce: 56 push rsi
141293fcf: 57 push rdi
141293fd0: 53 push rbx
141293fd1: 4881eca8000000 sub rsp, 0xa8
141293fd8: 488daa80000000 lea rbp, [rdx + 0x80]
141293fdf: 440f296c2420 movaps xmmword ptr [rsp + 0x20], xmm13
141293fe5: 440f29642430 movaps xmmword ptr [rsp + 0x30], xmm12
141293feb: 440f295c2440 movaps xmmword ptr [rsp + 0x40], xmm11
141293ff1: 440f29542450 movaps xmmword ptr [rsp + 0x50], xmm10
141293ff7: 440f294c2460 movaps xmmword ptr [rsp + 0x60], xmm9
141293ffd: 440f29442470 movaps xmmword ptr [rsp + 0x70], xmm8
141294003: 0f29bc2480000000 movaps xmmword ptr [rsp + 0x80], xmm7
14129400b: 0f29b42490000000 movaps xmmword ptr [rsp + 0x90], xmm6
141294013: 488b4de0 mov rcx, qword ptr [rbp - 0x20]
141294017: 482b4dd0 sub rcx, qword ptr [rbp - 0x30]
14129401b: 488b45d8 mov rax, qword ptr [rbp - 0x28]
14129401f: 488908 mov qword ptr [rax], rcx
141294022: 0f28b42490000000 movaps xmm6, xmmword ptr [rsp + 0x90]
14129402a: 0f28bc2480000000 movaps xmm7, xmmword ptr [rsp + 0x80]
141294032: 440f28442470 movaps xmm8, xmmword ptr [rsp + 0x70]
141294038: 440f284c2460 movaps xmm9, xmmword ptr [rsp + 0x60]
14129403e: 440f28542450 movaps xmm10, xmmword ptr [rsp + 0x50]
141294044: 440f285c2440 movaps xmm11, xmmword ptr [rsp + 0x40]
14129404a: 440f28642430 movaps xmm12, xmmword ptr [rsp + 0x30]
141294050: 440f286c2420 movaps xmm13, xmmword ptr [rsp + 0x20]
141294056: 4881c4a8000000 add rsp, 0xa8
14129405d: 5b pop rbx
14129405e: 5f pop rdi
14129405f: 5e pop rsi
141294060: 415c pop r12
141294062: 415d pop r13
141294064: 415e pop r14
141294066: 415f pop r15
141294068: 5d pop rbp
141294069: c3 ret 
