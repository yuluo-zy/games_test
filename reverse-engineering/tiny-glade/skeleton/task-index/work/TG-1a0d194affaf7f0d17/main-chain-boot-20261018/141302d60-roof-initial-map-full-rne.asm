141302d60: push rbp
141302d61: push r15
141302d63: push r14
141302d65: push r13
141302d67: push r12
141302d69: push rsi
141302d6a: push rdi
141302d6b: push rbx
141302d6c: sub rsp, 0x168
141302d73: lea rbp, [rsp + 0x80]
141302d7b: movaps xmmword ptr [rbp + 0xd0], xmm15
141302d83: movaps xmmword ptr [rbp + 0xc0], xmm14
141302d8b: movdqa xmmword ptr [rbp + 0xb0], xmm13
141302d94: movaps xmmword ptr [rbp + 0xa0], xmm12
141302d9c: movaps xmmword ptr [rbp + 0x90], xmm11
141302da4: movaps xmmword ptr [rbp + 0x80], xmm10
141302dac: movaps xmmword ptr [rbp + 0x70], xmm9
141302db1: movaps xmmword ptr [rbp + 0x60], xmm8
141302db6: movaps xmmword ptr [rbp + 0x50], xmm7
141302dba: movaps xmmword ptr [rbp + 0x40], xmm6
141302dbe: mov qword ptr [rbp + 0x38], 0xfffffffffffffffe
141302dc6: mov qword ptr [rbp + 0x28], r8
141302dca: movzx r8d, byte ptr [rdx]
141302dce: mov rdi, qword ptr [rdx + 8]
141302dd2: mov rbx, qword ptr [rdx + 0x10]
141302dd6: mov rax, rbx
141302dd9: sub rax, rdi
141302ddc: xor r15d, r15d
141302ddf: mov byte ptr [rbp + 0x37], r8b
141302de3: test r8b, r8b
141302de6: mov r14d, 0
141302dec: mov qword ptr [rbp - 0x38], rax
141302df0: cmovne r14, rax
141302df4: mov qword ptr [rbp - 0x50], rcx
141302df8: movzx eax, byte ptr [rdx + 0x30]
141302dfc: mov rsi, qword ptr [rdx + 0x38]
141302e00: mov r8, qword ptr [rdx + 0x40]
141302e04: mov rcx, r8
141302e07: sub rcx, rsi
141302e0a: mov byte ptr [rbp + 0x36], al
141302e0d: test al, al
141302e0f: mov eax, 0
141302e14: mov qword ptr [rbp - 0x40], rcx
141302e18: cmovne rax, rcx
141302e1c: mov rcx, qword ptr [rdx + 0x60]
141302e20: mov qword ptr [rbp + 0x18], rcx
141302e24: test rcx, rcx
141302e27: je 0x141302e39
141302e29: mov ecx, dword ptr [rdx + 0x6c]
141302e2c: sub ecx, dword ptr [rdx + 0x68]
141302e2f: cmovb ecx, r15d
141302e33: add rcx, rcx
141302e36: mov r15, rcx
141302e39: mov qword ptr [rbp + 0x20], rdx
141302e3d: mov qword ptr [rbp - 0x48], r8
141302e41: add r14, rax
141302e44: jb 0x1413033d6
141302e4a: add r15, r14
141302e4d: jb 0x1413033d6
141302e53: xor r13d, r13d
141302e56: mov ecx, 0xc
141302e5b: mov rax, r15
141302e5e: mul rcx
141302e61: mov r12, rax
141302e64: jo 0x141303405
141302e6a: movabs rax, 0x7ffffffffffffffc
141302e74: cmp r12, rax
141302e77: ja 0x141303405
141302e7d: xor edx, edx
141302e7f: test r12, r12
141302e82: je 0x141302eab
141302e84: movzx eax, byte ptr [rip + 0x2331d7e]
141302e8b: mov r13d, 4
141302e91: mov edx, 4
141302e96: mov rcx, r12
141302e99: call 0x140613c10
141302e9e: test rax, rax
141302ea1: je 0x141303405
141302ea7: xor edx, edx
141302ea9: jmp 0x141302eb3
141302eab: mov eax, 4
141302eb0: xor r15d, r15d
141302eb3: mov qword ptr [rbp], r15
141302eb7: mov qword ptr [rbp + 8], rax
141302ebb: mov qword ptr [rbp + 0x10], 0
141302ec3: mov rcx, qword ptr [rbp + 0x20]
141302ec7: mov r13d, dword ptr [rcx + 0x68]
141302ecb: mov r12d, dword ptr [rcx + 0x6c]
141302ecf: mov ecx, r12d
141302ed2: sub ecx, r13d
141302ed5: cmovb ecx, edx
141302ed8: add rcx, rcx
141302edb: mov rdx, qword ptr [rbp + 0x18]
141302edf: test rdx, rdx
141302ee2: cmove rcx, rdx
141302ee6: add r14, rcx
141302ee9: jb 0x141303414
141302eef: cmp r15, r14
141302ef2: jb 0x14130339f
141302ef8: xor ecx, ecx
141302efa: lea rdx, [rbp - 0x20]
141302efe: test byte ptr [rbp + 0x37], 1
141302f02: je 0x141302f96
141302f08: mov r8, qword ptr [rbp + 0x20]
141302f0c: add r8, 0x18
141302f10: mov r9, qword ptr [r8 + 0x10]
141302f14: mov qword ptr [rbp - 0x10], r9
141302f18: movups xmm0, xmmword ptr [r8]
141302f1c: movups xmmword ptr [rbp - 0x20], xmm0
141302f20: cmp rbx, rdi
141302f23: je 0x141302f96
141302f25: lea r8, [rdi + 1]
141302f29: test byte ptr [rbp - 0x38], 1
141302f2d: je 0x141302f4f
141302f2f: lea r9, [rdi + rdi*2]
141302f33: lea r10, [rcx + rcx*2]
141302f37: mov r11d, dword ptr [rdx + r9*4 + 8]
141302f3c: mov dword ptr [rax + r10*4 + 8], r11d
141302f41: mov r9, qword ptr [rdx + r9*4]
141302f45: mov qword ptr [rax + r10*4], r9
141302f49: inc rcx
141302f4c: mov rdi, r8
141302f4f: cmp rbx, r8
141302f52: je 0x141302f96
141302f54: lea r8, [rbp - 0x14]
141302f58: lea r9, [rcx + rcx*2]
141302f5c: lea r9, [rax + r9*4]
141302f60: add rcx, 2
141302f64: lea r10, [rdi + rdi*2]
141302f68: mov r11d, dword ptr [rdx + r10*4 + 8]
141302f6d: mov dword ptr [r9 + 8], r11d
141302f71: mov r10, qword ptr [rdx + r10*4]
141302f75: mov qword ptr [r9], r10
141302f78: mov r10, qword ptr [r8]
141302f7b: mov qword ptr [r9 + 0xc], r10
141302f7f: mov r10d, dword ptr [r8 + 8]
141302f83: mov dword ptr [r9 + 0x14], r10d
141302f87: add r9, 0x18
141302f8b: mov edi, 2
141302f90: cmp rbx, 2
141302f94: jne 0x141302f60
141302f96: cmp qword ptr [rbp + 0x18], 0
141302f9b: je 0x141303287
141302fa1: cmp r13d, r12d
141302fa4: jae 0x141303287
141302faa: mov r8, qword ptr [rbp + 0x18]
141302fae: mov r8d, dword ptr [r8]
141302fb1: xorps xmm0, xmm0
141302fb4: cvtsi2ss xmm0, r8
141302fb9: mov r8d, r13d
141302fbc: not r8d
141302fbf: add r8d, r12d
141302fc2: cmp r8d, 0xb
141302fc6: jae 0x1413030c8
141302fcc: mov r9d, r12d
141302fcf: sub r9d, r13d
141302fd2: lea r8d, [r13 + 1]
141302fd6: test r9b, 1
141302fda: je 0x141303027
141302fdc: mov r9d, r13d
141302fdf: xorps xmm1, xmm1
141302fe2: cvtsi2ss xmm1, r9
141302fe7: divss xmm1, xmm0
141302feb: addss xmm1, dword ptr [rip + 0x16570f5]
141302ff3: lea r9, [rcx + rcx*2]
141302ff7: movss xmm2, dword ptr [rip + 0x16570e9]
141302fff: movlps qword ptr [rax + r9*4], xmm2
141303004: movss dword ptr [rax + r9*4 + 8], xmm1
14130300b: movss xmm2, dword ptr [rip + 0x162285d]
141303013: movlps qword ptr [rax + r9*4 + 0xc], xmm2
141303019: movss dword ptr [rax + r9*4 + 0x14], xmm1
141303020: add rcx, 2
141303024: mov r13d, r8d
141303027: cmp r12d, r8d
14130302a: je 0x141303287
141303030: lea r8, [rcx + rcx*2]
141303034: lea r8, [rax + r8*4]
141303038: add r8, 0x2c
14130303c: movss xmm1, dword ptr [rip + 0x16570a4]
141303044: movss xmm2, dword ptr [rip + 0x165709c]
14130304c: movss xmm3, dword ptr [rip + 0x162281c]
141303054: nop word ptr cs:[rax + rax]
141303060: mov r9d, r13d
141303063: xorps xmm4, xmm4
141303066: cvtsi2ss xmm4, r9
14130306b: divss xmm4, xmm0
14130306f: addss xmm4, xmm1
141303073: movlps qword ptr [r8 - 0x2c], xmm2
141303078: movss dword ptr [r8 - 0x24], xmm4
14130307e: lea r9d, [r13 + 1]
141303082: xorps xmm5, xmm5
141303085: cvtsi2ss xmm5, r9
14130308a: movlps qword ptr [r8 - 0x20], xmm3
14130308f: movss dword ptr [r8 - 0x18], xmm4
141303095: divss xmm5, xmm0
141303099: addss xmm5, xmm1
14130309d: movlps qword ptr [r8 - 0x14], xmm2
1413030a2: movss dword ptr [r8 - 0xc], xmm5
1413030a8: movlps qword ptr [r8 - 8], xmm3
1413030ad: movss dword ptr [r8], xmm5
1413030b2: add rcx, 4
1413030b6: add r8, 0x30
1413030ba: add r13d, 2
1413030be: cmp r13d, r12d
1413030c1: jne 0x141303060
1413030c3: jmp 0x141303287
1413030c8: lea r9, [rcx + rcx*2]
1413030cc: lea r11, [rax + r9*4]
1413030d0: add r11, 0x10
1413030d4: lea r10, [r8*8]
1413030dc: lea r10, [r10 + r10*2]
1413030e0: lea rdi, [r11 + r10]
1413030e4: cmp rdi, r11
1413030e7: jb 0x141302fcc
1413030ed: lea r11, [rax + r9*4]
1413030f1: add r11, 0x14
1413030f5: lea rdi, [r11 + r10]
1413030f9: cmp rdi, r11
1413030fc: jb 0x141302fcc
141303102: lea r11, [rax + r9*4]
141303106: add r11, 0xc
14130310a: lea rdi, [r11 + r10]
14130310e: cmp rdi, r11
141303111: jb 0x141302fcc
141303117: lea r11, [rax + r9*4]
14130311b: add r11, 8
14130311f: lea rdi, [r11 + r10]
141303123: cmp rdi, r11
141303126: jb 0x141302fcc
14130312c: lea r11, [rax + r9*4]
141303130: add r11, 4
141303134: lea rdi, [r11 + r10]
141303138: cmp rdi, r11
14130313b: jb 0x141302fcc
141303141: lea r9, [rax + r9*4]
141303145: add r10, r9
141303148: cmp r10, r9
14130314b: jb 0x141302fcc
141303151: inc r8
141303154: mov r10, r8
141303157: and r10, 0xfffffffffffffffc
14130315b: lea rcx, [rcx + r10*2]
14130315f: movd xmm1, r13d
141303164: add r13d, r10d
141303167: pshufd xmm1, xmm1, 0
14130316c: paddd xmm1, xmmword ptr [rip + 0x17d17ec]
141303174: movaps xmm2, xmm0
141303177: shufps xmm2, xmm0, 0
14130317b: movaps xmm6, xmmword ptr [rip + 0x191a29e]
141303182: movaps xmm7, xmmword ptr [rip + 0x182b3c7]
141303189: movaps xmm4, xmmword ptr [rip + 0x1637ad0]
141303190: movss xmm5, dword ptr [rip + 0x1656f50]
141303198: movaps xmm10, xmmword ptr [rip + 0x19357f0]
1413031a0: movsd xmm11, qword ptr [rip + 0x177db37]
1413031a9: xorps xmm12, xmm12
1413031ad: movdqa xmm13, xmmword ptr [rip + 0x17d17ba]
1413031b6: mov r11, r10
1413031b9: nop dword ptr [rax]
1413031c0: movdqa xmm15, xmm1
1413031c5: pand xmm15, xmmword ptr [rip + 0x18179a2]
1413031ce: por xmm15, xmmword ptr [rip + 0x1881659]
1413031d7: movdqa xmm14, xmm1
1413031dc: psrld xmm14, 0x10
1413031e2: por xmm14, xmmword ptr [rip + 0x191a225]
1413031eb: subps xmm14, xmm6
1413031ef: addps xmm14, xmm15
1413031f3: divps xmm14, xmm2
1413031f7: addps xmm14, xmm7
1413031fb: movaps xmm15, xmm4
1413031ff: shufps xmm15, xmm14, 3
141303204: movaps xmm9, xmm5
141303208: shufps xmm9, xmm15, 0x24
14130320d: movq xmm15, xmm14
141303212: movaps xmm8, xmm4
141303216: shufps xmm8, xmm14, 0x23
14130321b: movaps xmm3, xmm5
14130321e: shufps xmm3, xmm8, 0x24
141303223: movaps xmm8, xmm11
141303227: shufps xmm8, xmm14, 0xf9
14130322c: shufps xmm8, xmm8, 0xd2
141303231: movups xmmword ptr [r9 + 0x50], xmm8
141303236: movaps xmm8, xmm11
14130323a: shufps xmm8, xmm14, 0x59
14130323f: unpckhpd xmm14, xmm12
141303244: shufps xmm14, xmm10, 0xe2
141303249: movups xmmword ptr [r9 + 0x40], xmm14
14130324e: movups xmmword ptr [r9 + 0x30], xmm3
141303253: shufps xmm8, xmm8, 0xd2
141303258: movups xmmword ptr [r9 + 0x20], xmm8
14130325d: shufps xmm15, xmm10, 0xe2
141303262: movups xmmword ptr [r9 + 0x10], xmm15
141303267: movups xmmword ptr [r9], xmm9
14130326b: paddd xmm1, xmm13
141303270: add r9, 0x60
141303274: add r11, -4
141303278: jne 0x1413031c0
14130327e: cmp r8, r10
141303281: jne 0x141302fcc
141303287: test byte ptr [rbp + 0x36], 1
14130328b: je 0x141303336
141303291: mov r9, qword ptr [rbp + 0x20]
141303295: add r9, 0x48
141303299: mov r8, qword ptr [r9 + 0x10]
14130329d: mov qword ptr [rbp - 0x10], r8
1413032a1: movups xmm0, xmmword ptr [r9]
1413032a5: movups xmmword ptr [rbp - 0x20], xmm0
1413032a9: mov rdi, qword ptr [rbp - 0x48]
1413032ad: cmp rdi, rsi
1413032b0: je 0x141303336
1413032b6: lea r8, [rsi + 1]
1413032ba: test byte ptr [rbp - 0x40], 1
1413032be: je 0x1413032e0
1413032c0: lea r9, [rsi + rsi*2]
1413032c4: lea r10, [rcx + rcx*2]
1413032c8: mov r11d, dword ptr [rdx + r9*4 + 8]
1413032cd: mov dword ptr [rax + r10*4 + 8], r11d
1413032d2: mov r9, qword ptr [rdx + r9*4]
1413032d6: mov qword ptr [rax + r10*4], r9
1413032da: inc rcx
1413032dd: mov rsi, r8
1413032e0: cmp rdi, r8
1413032e3: je 0x141303336
1413032e5: lea r8, [rbp - 0x14]
1413032e9: lea r9, [rcx + rcx*2]
1413032ed: lea rax, [rax + r9*4]
1413032f1: nop word ptr cs:[rax + rax]
141303300: add rcx, 2
141303304: lea r9, [rsi + rsi*2]
141303308: mov r10d, dword ptr [rdx + r9*4 + 8]
14130330d: mov dword ptr [rax + 8], r10d
141303311: mov r9, qword ptr [rdx + r9*4]
141303315: mov qword ptr [rax], r9
141303318: mov r9, qword ptr [r8]
14130331b: mov qword ptr [rax + 0xc], r9
14130331f: mov r9d, dword ptr [r8 + 8]
141303323: mov dword ptr [rax + 0x14], r9d
141303327: add rax, 0x18
14130332b: mov esi, 2
141303330: cmp rdi, 2
141303334: jne 0x141303300
141303336: mov qword ptr [rbp + 0x10], rcx
14130333a: mov rax, qword ptr [rbp - 0x50]
14130333e: mov qword ptr [rax + 0x10], rcx
141303342: movups xmm0, xmmword ptr [rbp]
141303346: movups xmmword ptr [rax], xmm0
141303349: movaps xmm6, xmmword ptr [rbp + 0x40]
14130334d: movaps xmm7, xmmword ptr [rbp + 0x50]
141303351: movaps xmm8, xmmword ptr [rbp + 0x60]
141303356: movaps xmm9, xmmword ptr [rbp + 0x70]
14130335b: movaps xmm10, xmmword ptr [rbp + 0x80]
141303363: movaps xmm11, xmmword ptr [rbp + 0x90]
14130336b: movaps xmm12, xmmword ptr [rbp + 0xa0]
141303373: movaps xmm13, xmmword ptr [rbp + 0xb0]
14130337b: movaps xmm14, xmmword ptr [rbp + 0xc0]
141303383: movaps xmm15, xmmword ptr [rbp + 0xd0]
14130338b: add rsp, 0x168
141303392: pop rbx
141303393: pop rdi
141303394: pop rsi
141303395: pop r12
141303397: pop r13
141303399: pop r14
14130339b: pop r15
14130339d: pop rbp
14130339e: ret 
14130339f: mov qword ptr [rsp + 0x20], 0xc
1413033a8: mov rcx, rbp
1413033ab: mov r9d, 4
1413033b1: xor edx, edx
1413033b3: mov r8, r14
1413033b6: call 0x1428a9a60
1413033bb: mov rax, qword ptr [rbp + 8]
1413033bf: mov rcx, qword ptr [rbp + 0x10]
1413033c3: lea rdx, [rbp - 0x20]
1413033c7: test byte ptr [rbp + 0x37], 1
1413033cb: jne 0x141302f08
1413033d1: jmp 0x141302f96
1413033d6: lea rax, [rip + 0x19356cb]
1413033dd: mov qword ptr [rbp - 0x30], rax
1413033e1: mov qword ptr [rbp - 0x28], 1
1413033e9: mov qword ptr [rbp - 0x20], 8
1413033f1: xorps xmm0, xmm0
1413033f4: movups xmmword ptr [rbp - 0x18], xmm0
1413033f8: lea rcx, [rbp - 0x30]
1413033fc: mov rdx, qword ptr [rbp + 0x28]
141303400: call 0x1428d9390
141303405: mov rcx, r13
141303408: mov rdx, r12
14130340b: mov r8, qword ptr [rbp + 0x28]
14130340f: call 0x1428d8fe3
141303414: lea rax, [rip + 0x193568d]
14130341b: mov qword ptr [rbp - 0x30], rax
14130341f: mov qword ptr [rbp - 0x28], 1
141303427: mov qword ptr [rbp - 0x20], 8
14130342f: xorps xmm0, xmm0
141303432: movups xmmword ptr [rbp - 0x18], xmm0
141303436: lea rcx, [rbp - 0x30]
14130343a: mov rdx, qword ptr [rbp + 0x28]
14130343e: call 0x1428d9390
141303443: ud2 
141303445: nop word ptr cs:[rax + rax]
141303450: mov qword ptr [rsp + 0x10], rdx
141303455: push rbp
141303456: push r15
141303458: push r14
14130345a: push r13
14130345c: push r12
14130345e: push rsi
14130345f: push rdi
141303460: push rbx
141303461: sub rsp, 0xc8
141303468: lea rbp, [rdx + 0x80]
14130346f: movaps xmmword ptr [rsp + 0x20], xmm15
141303475: movaps xmmword ptr [rsp + 0x30], xmm14
14130347b: movdqa xmmword ptr [rsp + 0x40], xmm13
141303482: movaps xmmword ptr [rsp + 0x50], xmm12
141303488: movaps xmmword ptr [rsp + 0x60], xmm11
14130348e: movaps xmmword ptr [rsp + 0x70], xmm10
141303494: movaps xmmword ptr [rsp + 0x80], xmm9
14130349d: movaps xmmword ptr [rsp + 0x90], xmm8
1413034a6: movaps xmmword ptr [rsp + 0xa0], xmm7
1413034ae: movaps xmmword ptr [rsp + 0xb0], xmm6
1413034b6: mov rax, qword ptr [rbp]
1413034ba: test rax, rax
1413034bd: je 0x1413034d6
1413034bf: mov rcx, qword ptr [rbp + 8]
1413034c3: shl rax, 2
1413034c7: lea rdx, [rax + rax*2]
1413034cb: mov r8d, 4
1413034d1: call 0x140613c20
1413034d6: movaps xmm6, xmmword ptr [rsp + 0xb0]
1413034de: movaps xmm7, xmmword ptr [rsp + 0xa0]
1413034e6: movaps xmm8, xmmword ptr [rsp + 0x90]
1413034ef: movaps xmm9, xmmword ptr [rsp + 0x80]
1413034f8: movaps xmm10, xmmword ptr [rsp + 0x70]
1413034fe: movaps xmm11, xmmword ptr [rsp + 0x60]
141303504: movaps xmm12, xmmword ptr [rsp + 0x50]
14130350a: movaps xmm13, xmmword ptr [rsp + 0x40]
141303510: movaps xmm14, xmmword ptr [rsp + 0x30]
141303516: movaps xmm15, xmmword ptr [rsp + 0x20]
14130351c: add rsp, 0xc8
141303523: pop rbx
141303524: pop rdi
141303525: pop rsi
141303526: pop r12
141303528: pop r13
14130352a: pop r14
14130352c: pop r15
14130352e: pop rbp
14130352f: ret 
