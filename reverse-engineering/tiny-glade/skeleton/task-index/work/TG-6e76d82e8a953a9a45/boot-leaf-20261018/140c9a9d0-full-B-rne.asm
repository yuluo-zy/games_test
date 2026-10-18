140c9a9d0: 56 push rsi
140c9a9d1: 57 push rdi
140c9a9d2: 4881ece8000000 sub rsp, 0xe8
140c9a9d9: 440f29bc24d0000000 movaps xmmword ptr [rsp + 0xd0], xmm15
140c9a9e2: 440f29b424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm14
140c9a9eb: 440f29ac24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm13
140c9a9f4: 440f29a424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm12
140c9a9fd: 440f299c2490000000 movaps xmmword ptr [rsp + 0x90], xmm11
140c9aa06: 440f29942480000000 movaps xmmword ptr [rsp + 0x80], xmm10
140c9aa0f: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140c9aa15: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140c9aa1b: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140c9aa20: 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140c9aa25: f20f105910 movsd xmm3, qword ptr [rcx + 0x10]
140c9aa2a: 0f28c3 movaps xmm0, xmm3
140c9aa2d: 0fc6c311 shufps xmm0, xmm3, 0x11
140c9aa31: 0f59d8 mulps xmm3, xmm0
140c9aa34: 0f57e4 xorps xmm4, xmm4
140c9aa37: 0f2edc ucomiss xmm3, xmm4
140c9aa3a: 0f864d020000 jbe 0x140c9ac8d
140c9aa40: f20f1019 movsd xmm3, qword ptr [rcx]
140c9aa44: f20f106108 movsd xmm4, qword ptr [rcx + 8]
140c9aa49: 0f590510b3e001 mulps xmm0, xmmword ptr [rip + 0x1e0b310]
140c9aa50: 0f28eb movaps xmm5, xmm3
140c9aa53: 0f14e8 unpcklps xmm5, xmm0
140c9aa56: 0f572d93aec801 xorps xmm5, xmmword ptr [rip + 0x1c8ae93]
140c9aa5d: f20f10eb movsd xmm5, xmm3
140c9aa61: 0fc6dd24 shufps xmm3, xmm5, 0x24
140c9aa65: f30f16e8 movshdup xmm5, xmm0
140c9aa69: 0f28f0 movaps xmm6, xmm0
140c9aa6c: 660f15f0 unpckhpd xmm6, xmm0
140c9aa70: 0f28f8 movaps xmm7, xmm0
140c9aa73: 0fc6f8ff shufps xmm7, xmm0, 0xff
140c9aa77: 440f28c0 movaps xmm8, xmm0
140c9aa7b: 440fc6c091 shufps xmm8, xmm0, 0x91
140c9aa80: 440f28c8 movaps xmm9, xmm0
140c9aa84: 440fc6c83b shufps xmm9, xmm0, 0x3b
140c9aa89: 0f14ca unpcklps xmm1, xmm2
140c9aa8c: ba01000000 mov edx, 1
140c9aa91: f3440f1015960bcc01 movss xmm10, dword ptr [rip + 0x1cc0b96]
140c9aa9a: 31c0 xor eax, eax
140c9aa9c: f3440f101d5faec801 movss xmm11, dword ptr [rip + 0x1c8ae5f]
140c9aaa5: b904000000 mov ecx, 4
140c9aaaa: 4531c0 xor r8d, r8d
140c9aaad: 31ff xor edi, edi
140c9aaaf: 4531d2 xor r10d, r10d
140c9aab2: e946010000 jmp 0x140c9abfd
140c9aab7: 660f1f840000000000 nop word ptr [rax + rax]
140c9aac0: 41bb04000000 mov r11d, 4
140c9aac6: 41f6c001 test r8b, 1
140c9aaca: 0f8467010000 je 0x140c9ac37
140c9aad0: 4983fa04 cmp r10, 4
140c9aad4: 4c89de mov rsi, r11
140c9aad7: 4989fb mov r11, rdi
140c9aada: bf00000000 mov edi, 0
140c9aadf: 490f45fa cmovne rdi, r10
140c9aae3: f30f116c2420 movss dword ptr [rsp + 0x20], xmm5
140c9aae9: 0f11442424 movups xmmword ptr [rsp + 0x24], xmm0
140c9aaee: f30f11742434 movss dword ptr [rsp + 0x34], xmm6
140c9aaf4: f30f117c2438 movss dword ptr [rsp + 0x38], xmm7
140c9aafa: f30f1144243c movss dword ptr [rsp + 0x3c], xmm0
140c9ab00: f2440f106cfc20 movsd xmm13, qword ptr [rsp + rdi*8 + 0x20]
140c9ab07: 450f14ed unpcklps xmm13, xmm13
140c9ab0b: 440f59eb mulps xmm13, xmm3
140c9ab0f: 450f28e5 movaps xmm12, xmm13
140c9ab13: 66450f15e5 unpckhpd xmm12, xmm13
140c9ab18: 450f58e5 addps xmm12, xmm13
140c9ab1c: 4983fb04 cmp r11, 4
140c9ab20: bf00000000 mov edi, 0
140c9ab25: 490f45fb cmovne rdi, r11
140c9ab29: 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
140c9ab2f: 440f294c2430 movaps xmmword ptr [rsp + 0x30], xmm9
140c9ab35: f2440f1074fc20 movsd xmm14, qword ptr [rsp + rdi*8 + 0x20]
140c9ab3c: 450f14f6 unpcklps xmm14, xmm14
140c9ab40: 440f59f3 mulps xmm14, xmm3
140c9ab44: 450f28ee movaps xmm13, xmm14
140c9ab48: 66450f15ee unpckhpd xmm13, xmm14
140c9ab4d: 450f58ee addps xmm13, xmm14
140c9ab51: 440f58e4 addps xmm12, xmm4
140c9ab55: 440f58ec addps xmm13, xmm4
140c9ab59: 440f28f1 movaps xmm14, xmm1
140c9ab5d: 450f5cf4 subps xmm14, xmm12
140c9ab61: 450f28fd movaps xmm15, xmm13
140c9ab65: 450f5cfc subps xmm15, xmm12
140c9ab69: 450f59f7 mulps xmm14, xmm15
140c9ab6d: f3410f16d6 movshdup xmm2, xmm14
140c9ab72: f3410f58d6 addss xmm2, xmm14
140c9ab77: 450f59ff mulps xmm15, xmm15
140c9ab7b: f3450f16f7 movshdup xmm14, xmm15
140c9ab80: f3450f58f7 addss xmm14, xmm15
140c9ab85: f3440f5f35120fde01 maxss xmm14, dword ptr [rip + 0x1de0f12]
140c9ab8e: f3410f5ed6 divss xmm2, xmm14
140c9ab93: 450f57f6 xorps xmm14, xmm14
140c9ab97: f3440f5ff2 maxss xmm14, xmm2
140c9ab9c: 410f28d3 movaps xmm2, xmm11
140c9aba0: f3410f5dd6 minss xmm2, xmm14
140c9aba5: 450f28f3 movaps xmm14, xmm11
140c9aba9: f3440f5cf2 subss xmm14, xmm2
140c9abae: f3450f12f6 movsldup xmm14, xmm14
140c9abb3: 450f59f4 mulps xmm14, xmm12
140c9abb7: f30f12d2 movsldup xmm2, xmm2
140c9abbb: 410f59d5 mulps xmm2, xmm13
140c9abbf: 410f58d6 addps xmm2, xmm14
140c9abc3: 440f28e1 movaps xmm12, xmm1
140c9abc7: 440f5ce2 subps xmm12, xmm2
140c9abcb: 450f59e4 mulps xmm12, xmm12
140c9abcf: f3410f16d4 movshdup xmm2, xmm12
140c9abd4: f3410f58d4 addss xmm2, xmm12
140c9abd9: f30f51d2 sqrtss xmm2, xmm2
140c9abdd: 440f2ed2 ucomiss xmm10, xmm2
140c9abe1: 490f47d3 cmova rdx, r11
140c9abe5: f3410f5dd2 minss xmm2, xmm10
140c9abea: 490f47c2 cmova rax, r10
140c9abee: 440f28d2 movaps xmm10, xmm2
140c9abf2: 4889f7 mov rdi, rsi
140c9abf5: 4d89da mov r10, r11
140c9abf8: 4584c9 test r9b, r9b
140c9abfb: 753a jne 0x140c9ac37
140c9abfd: 4883ff04 cmp rdi, 4
140c9ac01: 410f93c1 setae r9b
140c9ac05: 0f83b5feffff jae 0x140c9aac0
140c9ac0b: 4c8d5f01 lea r11, [rdi + 1]
140c9ac0f: 41f6c001 test r8b, 1
140c9ac13: 0f85b7feffff jne 0x140c9aad0
140c9ac19: 488d7702 lea rsi, [rdi + 2]
140c9ac1d: 4883ff03 cmp rdi, 3
140c9ac21: 410f94c1 sete r9b
140c9ac25: 480f44f1 cmove rsi, rcx
140c9ac29: 41b801000000 mov r8d, 1
140c9ac2f: 4989fa mov r10, rdi
140c9ac32: e9acfeffff jmp 0x140c9aae3
140c9ac37: 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140c9ac3c: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140c9ac41: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140c9ac47: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140c9ac4d: 440f28942480000000 movaps xmm10, xmmword ptr [rsp + 0x80]
140c9ac56: 440f289c2490000000 movaps xmm11, xmmword ptr [rsp + 0x90]
140c9ac5f: 440f28a424a0000000 movaps xmm12, xmmword ptr [rsp + 0xa0]
140c9ac68: 440f28ac24b0000000 movaps xmm13, xmmword ptr [rsp + 0xb0]
140c9ac71: 440f28b424c0000000 movaps xmm14, xmmword ptr [rsp + 0xc0]
140c9ac7a: 440f28bc24d0000000 movaps xmm15, xmmword ptr [rsp + 0xd0]
140c9ac83: 4881c4e8000000 add rsp, 0xe8
140c9ac8a: 5f pop rdi
140c9ac8b: 5e pop rsi
140c9ac8c: c3 ret 
140c9ac8d: 488d0dd839e901 lea rcx, [rip + 0x1e939d8] ; @142b2e66c 'assertion failed: self.area() > 0.0crates/utils/src/geometry/rectangle.rs'
140c9ac94: 4c8d051d3ae901 lea r8, [rip + 0x1e93a1d]
140c9ac9b: ba23000000 mov edx, 0x23
140c9aca0: e88be7c301 call 0x1428d9430 ; core::panicking::panic
140c9aca5: cc int3 
