140949e70: push rsi
140949e71: push rdi
140949e72: push rbx
140949e73: sub rsp, 0x70
140949e77: movaps xmmword ptr [rsp + 0x60], xmm8
140949e7d: movaps xmmword ptr [rsp + 0x50], xmm7
140949e82: movaps xmmword ptr [rsp + 0x40], xmm6
140949e87: mov rdi, rdx
140949e8a: mov rsi, rcx
140949e8d: mov rbx, qword ptr [rsp + 0xb0]
140949e95: lea rcx, [rsp + 0x28]
140949e9a: mov rdx, rbx
140949e9d: call 0x140c96d40
140949ea2: movss xmm8, dword ptr [rsp + 0x2c]
140949ea9: lea rcx, [rsp + 0x34]
140949eae: mov rdx, rbx
140949eb1: call 0x140c96d80
140949eb6: movss xmm0, dword ptr [rsp + 0x38]
140949ebc: xorps xmm0, xmmword ptr [rip + 0x1fdba2d]
140949ec3: divss xmm8, xmm0
140949ec8: maxss xmm8, dword ptr [rip + 0x1fdba33]
140949ed1: movss xmm6, dword ptr [rsi + 0x54]
140949ed6: mulss xmm6, dword ptr [rip + 0x21615d2]
140949ede: mulss xmm6, dword ptr [rip + 0x1fdb98a]
140949ee6: movaps xmm0, xmm6
140949ee9: call 0x142923fd0
140949eee: movaps xmm7, xmm0
140949ef1: movaps xmm0, xmm6
140949ef4: call 0x142923e80
140949ef9: movq xmm1, xmm7
140949efd: xorps xmm3, xmm3
140949f00: movdqa xmm2, xmm1
140949f04: shufps xmm2, xmm3, 0xe2
140949f08: movq xmm0, xmm0
140949f0c: shufps xmm1, xmm0, 0x22
140949f10: divss xmm8, dword ptr [rip + 0x2083157]
140949f19: movsd xmm3, qword ptr [rdi + 4]
140949f1e: movss xmm5, dword ptr [rdi]
140949f22: movlhps xmm5, xmm3
140949f25: movaps xmm0, xmm5
140949f28: shufps xmm0, xmm3, 0xd8
140949f2c: shufps xmm5, xmm3, 0x58
140949f30: movaps xmm4, xmm1
140949f33: mulps xmm4, xmm1
140949f36: movshdup xmm3, xmm4
140949f3a: addss xmm3, xmm4
140949f3e: movaps xmm6, xmm4
140949f41: unpckhpd xmm6, xmm4
140949f45: addss xmm6, xmm3
140949f49: shufps xmm6, xmm6, 0
140949f4d: shufps xmm4, xmm4, 0xff
140949f51: subps xmm4, xmm6
140949f54: mulps xmm4, xmm5
140949f57: movaps xmm3, xmm0
140949f5a: mulps xmm3, xmm2
140949f5d: movshdup xmm5, xmm3
140949f61: addss xmm5, xmm3
140949f65: movhlps xmm3, xmm3
140949f68: addss xmm3, xmm5
140949f6c: addss xmm3, xmm3
140949f70: shufps xmm3, xmm3, 0
140949f74: mulps xmm3, xmm1
140949f77: addps xmm3, xmm4
140949f7a: movaps xmm4, xmm2
140949f7d: shufps xmm4, xmm2, 0xd2
140949f81: mulps xmm4, xmm0
140949f84: shufps xmm0, xmm0, 0xd2
140949f88: mulps xmm0, xmm2
140949f8b: subps xmm4, xmm0
140949f8e: shufps xmm4, xmm4, 0xd2
140949f92: addps xmm1, xmm1
140949f95: shufps xmm1, xmm1, 0xff
140949f99: mulps xmm1, xmm4
140949f9c: addps xmm1, xmm3
140949f9f: movsldup xmm0, xmm8
140949fa4: mulps xmm0, xmm1
140949fa7: movhlps xmm1, xmm1
140949faa: mulss xmm1, xmm8
140949faf: movsd xmm2, qword ptr [rsi + 0x44]
140949fb4: addps xmm2, xmm0
140949fb7: movlps qword ptr [rsi + 0x44], xmm2
140949fbb: addss xmm1, dword ptr [rsi + 0x4c]
140949fc0: movss dword ptr [rsi + 0x4c], xmm1
140949fc5: movaps xmm6, xmmword ptr [rsp + 0x40]
140949fca: movaps xmm7, xmmword ptr [rsp + 0x50]
140949fcf: movaps xmm8, xmmword ptr [rsp + 0x60]
140949fd5: add rsp, 0x70
140949fd9: pop rbx
140949fda: pop rdi
140949fdb: pop rsi
140949fdc: ret 
