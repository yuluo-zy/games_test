00000001409447e0: push rbp
00000001409447e1: push r15
00000001409447e3: push r14
00000001409447e5: push rsi
00000001409447e6: push rdi
00000001409447e7: push rbx
00000001409447e8: sub rsp, 0x348
00000001409447ef: lea rbp, [rsp + 0x80]
00000001409447f7: movaps xmmword ptr [rbp + 0x2b0], xmm6
00000001409447fe: and rsp, 0xffffffffffffff80
0000000140944802: mov rbx, rsp
0000000140944805: mov qword ptr [rbx + 0x320], rbp
000000014094480c: mov qword ptr [rbp + 0x2a8], 0xfffffffffffffffe
0000000140944817: mov rsi, rcx
000000014094481a: mov qword ptr [rbx + 0x280], 1
0000000140944825: mov qword ptr [rbx + 0x288], 1
0000000140944830: mov byte ptr [rbx + 0x290], 0
0000000140944837: xorps xmm6, xmm6
000000014094483a: movaps xmmword ptr [rbx + 0x100], xmm6
0000000140944841: movaps xmmword ptr [rbx + 0x180], xmm6
0000000140944848: mov word ptr [rbx + 0x200], 0
0000000140944851: mov qword ptr [rbx + 0x208], 0
000000014094485c: mov qword ptr [rbx + 0x210], 8
0000000140944867: movups xmmword ptr [rbx + 0x218], xmm6
000000014094486e: mov qword ptr [rbx + 0x228], 8
0000000140944879: mov qword ptr [rbx + 0x230], 0
0000000140944884: mov byte ptr [rbx + 0x238], 1
000000014094488b: movzx eax, byte ptr [rip + 0x2cf0377]
0000000140944892: mov ecx, 0x200
0000000140944897: mov edx, 0x80
000000014094489c: call 0x140613c10
00000001409448a1: test rax, rax
00000001409448a4: je 0x140944e19
00000001409448aa: mov rdi, rax
00000001409448ad: lea rdx, [rbx + 0x100]
00000001409448b4: mov r8d, 0x200
00000001409448ba: mov rcx, rax
00000001409448bd: call 0x142923c00
00000001409448c2: mov qword ptr [rbx + 0x90], 1
00000001409448cd: mov qword ptr [rbx + 0x98], rdi
00000001409448d4: mov qword ptr [rbx + 0xa0], 1
00000001409448df: mov qword ptr [rbx + 0xa8], rdi
00000001409448e6: mov qword ptr [rbx + 0x280], 1
00000001409448f1: mov qword ptr [rbx + 0x288], 1
00000001409448fc: mov byte ptr [rbx + 0x290], 0
0000000140944903: movaps xmmword ptr [rbx + 0x100], xmm6
000000014094490a: movaps xmmword ptr [rbx + 0x180], xmm6
0000000140944911: mov word ptr [rbx + 0x200], 0
000000014094491a: mov qword ptr [rbx + 0x208], 0
0000000140944925: mov qword ptr [rbx + 0x210], 8
0000000140944930: movups xmmword ptr [rbx + 0x218], xmm6
0000000140944937: mov qword ptr [rbx + 0x228], 8
0000000140944942: mov qword ptr [rbx + 0x230], 0
000000014094494d: mov byte ptr [rbx + 0x238], 1
0000000140944954: movzx eax, byte ptr [rip + 0x2cf02ae]
000000014094495b: mov ecx, 0x200
0000000140944960: mov edx, 0x80
0000000140944965: call 0x140613c10
000000014094496a: test rax, rax
000000014094496d: je 0x140944e2d
0000000140944973: mov r14, rax
0000000140944976: lea rdx, [rbx + 0x100]
000000014094497d: mov r8d, 0x200
0000000140944983: mov rcx, rax
0000000140944986: call 0x142923c00
000000014094498b: mov qword ptr [rbx + 0xb0], 1
0000000140944996: mov qword ptr [rbx + 0xb8], r14
000000014094499d: mov qword ptr [rbx + 0xc0], 1
00000001409449a8: mov qword ptr [rbx + 0xc8], r14
00000001409449af: movzx eax, byte ptr [rip + 0x2cf0253]
00000001409449b6: mov ecx, 0x30
00000001409449bb: mov edx, 8
00000001409449c0: call 0x140613c10
00000001409449c5: test rax, rax
00000001409449c8: je 0x140944e3e
00000001409449ce: mov qword ptr [rax], 1
00000001409449d5: mov qword ptr [rax + 8], 1
00000001409449dd: mov byte ptr [rax + 0x10], 0
00000001409449e1: mov qword ptr [rax + 0x18], 0
00000001409449e9: mov qword ptr [rax + 0x20], 8
00000001409449f1: mov qword ptr [rax + 0x28], 0
00000001409449f9: mov qword ptr [rbx + 0x48], rax
00000001409449fd: lock inc qword ptr [rax]
0000000140944a01: jle 0x140944eb6
0000000140944a07: mov qword ptr [rbx + 0x40], rax
0000000140944a0b: mov qword ptr [rbx + 0x68], rax
0000000140944a0f: movabs r15, 0x8000000000000000
0000000140944a19: mov qword ptr [rbx + 0x110], r15
0000000140944a20: mov qword ptr [rbx + 0x100], 0
0000000140944a2b: mov byte ptr [rbx + 0x128], 0
0000000140944a32: movzx eax, byte ptr [rip + 0x2cf01d0]
0000000140944a39: mov ecx, 0xa
0000000140944a3e: mov edx, 1
0000000140944a43: call 0x140613c10
0000000140944a48: test rax, rax
0000000140944a4b: mov qword ptr [rbx + 0x38], rax
0000000140944a4f: je 0x140944e57
0000000140944a55: movabs rcx, 0x617320636e797341
0000000140944a5f: mov qword ptr [rax], rcx
0000000140944a62: mov word ptr [rax + 8], 0x6576
0000000140944a68: mov qword ptr [rbx + 0x50], 0xa
0000000140944a70: mov qword ptr [rbx + 0x58], rax
0000000140944a74: mov qword ptr [rbx + 0x60], 0xa
0000000140944a7c: lea rcx, [rbx + 0xd0]
0000000140944a83: lea rdx, [rbx + 0x100]
0000000140944a8a: lea r8, [rbx + 0x50]
0000000140944a8e: call 0x140d558f0
0000000140944a93: mov qword ptr [rbx + 0x100], 1
0000000140944a9e: mov qword ptr [rbx + 0x108], rdi
0000000140944aa5: mov qword ptr [rbx + 0x110], 1
0000000140944ab0: mov qword ptr [rbx + 0x118], r14
0000000140944ab7: mov rax, qword ptr [rbx + 0x40]
0000000140944abb: mov qword ptr [rbx + 0x120], rax
0000000140944ac2: mov byte ptr [rbx + 0x31], 1
0000000140944ac6: mov byte ptr [rbx + 0x32], 0
0000000140944aca: lea rcx, [rbx + 0x50]
0000000140944ace: lea rdx, [rbx + 0xd0]
0000000140944ad5: lea r8, [rbx + 0x100]
0000000140944adc: call 0x140a6d8b0
0000000140944ae1: cmp qword ptr [rbx + 0x50], 0
0000000140944ae6: je 0x140944e6f
0000000140944aec: mov rax, qword ptr [rbx + 0x60]
0000000140944af0: mov qword ptr [rbx + 0x80], rax
0000000140944af7: movups xmm0, xmmword ptr [rbx + 0x50]
0000000140944afb: movaps xmmword ptr [rbx + 0x70], xmm0
0000000140944aff: lea rcx, [rsi + 0x38]
0000000140944b03: mov byte ptr [rbx + 0x2f], 1
0000000140944b07: mov byte ptr [rbx + 0x30], 1
0000000140944b0b: movabs rdx, 0x4ab328be485143c6
0000000140944b15: movabs r8, 0xf7c86db5568a7d07
0000000140944b1f: call 0x1400dd6f0
0000000140944b24: test rax, rax
0000000140944b27: je 0x140944b5c
0000000140944b29: cmp qword ptr [rsi + 0x218], rdx
0000000140944b30: jbe 0x140944b5c
0000000140944b32: mov rax, qword ptr [rsi + 0x210]
0000000140944b39: mov rax, qword ptr [rax + rdx*8]
0000000140944b3d: test rax, rax
0000000140944b40: je 0x140944b5c
0000000140944b42: mov rcx, qword ptr [rsi + 0x1e0]
0000000140944b49: not rax
0000000140944b4c: lea rax, [rax + rax*2]
0000000140944b50: shl rax, 5
0000000140944b54: cmp qword ptr [rcx + rax + 0x20], 0
0000000140944b5a: jne 0x140944b6c
0000000140944b5c: mov byte ptr [rbx + 0x2f], 1
0000000140944b60: mov byte ptr [rbx + 0x30], 1
0000000140944b64: mov rcx, rsi
0000000140944b67: call 0x141521240
0000000140944b6c: mov qword ptr [rbx + 0xd0], r15
0000000140944b73: movzx eax, byte ptr [rip + 0x2cf008f]
0000000140944b7a: mov ecx, 0x20
0000000140944b7f: mov edx, 8
0000000140944b84: call 0x140613c10
0000000140944b89: test rax, rax
0000000140944b8c: je 0x140944ea7
0000000140944b92: mov dword ptr [rax + 8], 0
0000000140944b99: mov qword ptr [rax], r15
0000000140944b9c: movups xmm0, xmmword ptr [rbx + 0xd0]
0000000140944ba3: movups xmm1, xmmword ptr [rbx + 0xe0]
0000000140944baa: movups xmm2, xmmword ptr [rbx + 0xf0]
0000000140944bb1: movups xmmword ptr [rbx + 0x138], xmm2
0000000140944bb8: movups xmmword ptr [rbx + 0x128], xmm1
0000000140944bbf: movups xmmword ptr [rbx + 0x118], xmm0
0000000140944bc6: mov byte ptr [rbx + 0x148], 2
0000000140944bcd: mov qword ptr [rbx + 0x100], 1
0000000140944bd8: mov qword ptr [rbx + 0x108], rax
0000000140944bdf: mov qword ptr [rbx + 0x110], 1
0000000140944bea: mov byte ptr [rbx + 0x2f], 1
0000000140944bee: mov byte ptr [rbx + 0x30], 1
0000000140944bf2: lea rdx, [rbx + 0x100]
0000000140944bf9: mov rcx, rsi
0000000140944bfc: call 0x141541420
0000000140944c01: mov rcx, qword ptr [rbx + 0x80]
0000000140944c08: mov qword ptr [rbx + 0x210], rcx
0000000140944c0f: movaps xmm0, xmmword ptr [rbx + 0x70]
0000000140944c13: movups xmmword ptr [rbx + 0x200], xmm0
0000000140944c1a: mov qword ptr [rbx + 0x220], 0
0000000140944c25: mov qword ptr [rbx + 0x100], 1
0000000140944c30: mov qword ptr [rbx + 0x108], rdi
0000000140944c37: mov byte ptr [rbx + 0x228], 0
0000000140944c3e: mov qword ptr [rbx + 0x230], 1
0000000140944c49: mov qword ptr [rbx + 0x238], r14
0000000140944c50: mov qword ptr [rbx + 0x110], 0
0000000140944c5b: mov qword ptr [rbx + 0x118], 8
0000000140944c66: mov qword ptr [rbx + 0x120], 0
0000000140944c71: mov qword ptr [rbx + 0x128], r15
0000000140944c78: mov dword ptr [rbx + 0x240], 0
0000000140944c82: movsd xmm0, qword ptr [rip + 0x216d726]
0000000140944c8a: movsd qword ptr [rbx + 0x244], xmm0
0000000140944c92: mov rcx, qword ptr [rbx + 0x40]
0000000140944c96: mov qword ptr [rbx + 0x218], rcx
0000000140944c9d: mov byte ptr [rbx + 0x2f], 0
0000000140944ca1: mov byte ptr [rbx + 0x30], 0
0000000140944ca5: lea rdx, [rbx + 0x100]
0000000140944cac: mov rcx, rax
0000000140944caf: call 0x1415496b0
0000000140944cb4: mov rsi, rax
0000000140944cb7: lea rdi, [rax + 0x38]
0000000140944cbb: mov byte ptr [rbx + 0x2f], 0
0000000140944cbf: mov byte ptr [rbx + 0x30], 0
0000000140944cc3: movabs rdx, 0x4d00324fb817a8a1
0000000140944ccd: movabs r8, 0x5846f8d25a034525
0000000140944cd7: mov rcx, rdi
0000000140944cda: call 0x1400dd6f0
0000000140944cdf: test rax, rax
0000000140944ce2: je 0x140944d17
0000000140944ce4: cmp qword ptr [rsi + 0x218], rdx
0000000140944ceb: jbe 0x140944d17
0000000140944ced: mov rax, qword ptr [rsi + 0x210]
0000000140944cf4: mov rax, qword ptr [rax + rdx*8]
0000000140944cf8: test rax, rax
0000000140944cfb: je 0x140944d17
0000000140944cfd: mov rcx, qword ptr [rsi + 0x1e0]
0000000140944d04: not rax
0000000140944d07: lea rax, [rax + rax*2]
0000000140944d0b: shl rax, 5
0000000140944d0f: cmp qword ptr [rcx + rax + 0x20], 0
0000000140944d15: jne 0x140944d27
0000000140944d17: mov byte ptr [rbx + 0x2f], 0
0000000140944d1b: mov byte ptr [rbx + 0x30], 0
0000000140944d1f: mov rcx, rsi
0000000140944d22: call 0x14151ffb0
0000000140944d27: mov byte ptr [rbx + 0x2f], 0
0000000140944d2b: mov byte ptr [rbx + 0x30], 0
0000000140944d2f: movabs rdx, 0x400333187b2ad3ac
0000000140944d39: movabs r8, 0xd1d03ce17e24337e
0000000140944d43: mov rcx, rdi
0000000140944d46: call 0x1400dd6f0
0000000140944d4b: test rax, rax
0000000140944d4e: je 0x140944d83
0000000140944d50: cmp qword ptr [rsi + 0x218], rdx
0000000140944d57: jbe 0x140944d83
0000000140944d59: mov rax, qword ptr [rsi + 0x210]
0000000140944d60: mov rax, qword ptr [rax + rdx*8]
0000000140944d64: test rax, rax
0000000140944d67: je 0x140944d83
0000000140944d69: mov rcx, qword ptr [rsi + 0x1e0]
0000000140944d70: not rax
0000000140944d73: lea rax, [rax + rax*2]
0000000140944d77: shl rax, 5
0000000140944d7b: cmp qword ptr [rcx + rax + 0x20], 0
0000000140944d81: jne 0x140944d93
0000000140944d83: mov byte ptr [rbx + 0x2f], 0
0000000140944d87: mov byte ptr [rbx + 0x30], 0
0000000140944d8b: mov rcx, rsi
0000000140944d8e: call 0x14151e6f0
0000000140944d93: mov byte ptr [rbx + 0x2f], 0
0000000140944d97: mov byte ptr [rbx + 0x30], 0
0000000140944d9b: movabs rdx, 0xeb08402348b616b
0000000140944da5: movabs r8, 0x3c4b18b205b89fdb
0000000140944daf: mov rcx, rdi
0000000140944db2: call 0x1400dd6f0
0000000140944db7: test rax, rax
0000000140944dba: je 0x140944def
0000000140944dbc: cmp qword ptr [rsi + 0x218], rdx
0000000140944dc3: jbe 0x140944def
0000000140944dc5: mov rax, qword ptr [rsi + 0x210]
0000000140944dcc: mov rax, qword ptr [rax + rdx*8]
0000000140944dd0: test rax, rax
0000000140944dd3: je 0x140944def
0000000140944dd5: mov rcx, qword ptr [rsi + 0x1e0]
0000000140944ddc: not rax
0000000140944ddf: lea rax, [rax + rax*2]
0000000140944de3: shl rax, 5
0000000140944de7: cmp qword ptr [rcx + rax + 0x20], 0
0000000140944ded: jne 0x140944dff
0000000140944def: mov byte ptr [rbx + 0x2f], 0
0000000140944df3: mov byte ptr [rbx + 0x30], 0
0000000140944df7: mov rcx, rsi
0000000140944dfa: call 0x141521870
0000000140944dff: mov rax, rsi
0000000140944e02: movaps xmm6, xmmword ptr [rbp + 0x2b0]
0000000140944e09: lea rsp, [rbp + 0x2c8]
0000000140944e10: pop rbx
0000000140944e11: pop rdi
0000000140944e12: pop rsi
0000000140944e13: pop r14
0000000140944e15: pop r15
0000000140944e17: pop rbp
0000000140944e18: ret 
0000000140944e19: mov ecx, 0x80
0000000140944e1e: mov edx, 0x200
0000000140944e23: call 0x1428d9000
0000000140944e28: jmp 0x140944eb6
0000000140944e2d: mov ecx, 0x80
0000000140944e32: mov edx, 0x200
0000000140944e37: call 0x1428d9000
0000000140944e3c: jmp 0x140944eb6
0000000140944e3e: mov byte ptr [rbx + 0x35], 1
0000000140944e42: mov byte ptr [rbx + 0x36], 1
0000000140944e46: mov ecx, 8
0000000140944e4b: mov edx, 0x30
0000000140944e50: call 0x1428d9000
0000000140944e55: jmp 0x140944eb6
0000000140944e57: lea r8, [rip + 0x216d662]
0000000140944e5e: mov ecx, 1
0000000140944e63: mov edx, 0xa
0000000140944e68: call 0x1428d8fe3
0000000140944e6d: jmp 0x140944eb6
0000000140944e6f: mov rax, qword ptr [rbx + 0x58]
0000000140944e73: mov qword ptr [rbx + 0x100], rax
0000000140944e7a: lea rax, [rip + 0x216db57]
0000000140944e81: mov qword ptr [rsp + 0x20], rax
0000000140944e86: lea rcx, [rip + 0x216db2b]
0000000140944e8d: lea r9, [rip + 0x216d5bc]
0000000140944e94: lea r8, [rbx + 0x100]
0000000140944e9b: mov edx, 0x1a
0000000140944ea0: call 0x1428d9760
0000000140944ea5: jmp 0x140944eb6
