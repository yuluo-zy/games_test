14045dc60: push r15
14045dc62: push r14
14045dc64: push r13
14045dc66: push r12
14045dc68: push rsi
14045dc69: push rdi
14045dc6a: push rbp
14045dc6b: push rbx
14045dc6c: sub rsp, 0x188
14045dc73: mov rdi, rdx
14045dc76: mov rsi, rcx
14045dc79: call 0x1404a93a0
14045dc7e: mov ebx, 1
14045dc83: lock xadd dword ptr [rdi + 0x5b0], ebx
14045dc8b: test byte ptr [rsi], 1
14045dc8e: je 0x14045e29b
14045dc94: mov rax, qword ptr [rsi + 8]
14045dc98: cmp qword ptr [rdi + 0x218], rax
14045dc9f: jbe 0x14045e0f9
14045dca5: mov rcx, qword ptr [rdi + 0x210]
14045dcac: mov rax, qword ptr [rcx + rax*8]
14045dcb0: test rax, rax
14045dcb3: je 0x14045e0f9
14045dcb9: mov rcx, qword ptr [rdi + 0x1e0]
14045dcc0: not rax
14045dcc3: lea rax, [rax + rax*2]
14045dcc7: shl rax, 5
14045dccb: cmp qword ptr [rcx + rax + 0x20], 0
14045dcd1: je 0x14045e0f9
14045dcd7: add rcx, rax
14045dcda: mov r11, qword ptr [rcx + 0x10]
14045dcde: mov eax, dword ptr [rdi + 0x5b0]
14045dce4: mov rax, qword ptr [rsi + 0x10]
14045dce8: cmp qword ptr [rdi + 0x218], rax
14045dcef: jbe 0x14045e117
14045dcf5: mov rdx, qword ptr [rdi + 0x210]
14045dcfc: mov rax, qword ptr [rdx + rax*8]
14045dd00: test rax, rax
14045dd03: je 0x14045e117
14045dd09: mov r8, qword ptr [rdi + 0x1e0]
14045dd10: not rax
14045dd13: lea rdx, [rax + rax*2]
14045dd17: shl rdx, 5
14045dd1b: cmp qword ptr [r8 + rdx + 0x20], 0
14045dd21: je 0x14045e117
14045dd27: mov eax, dword ptr [rsi + 0x188]
14045dd2d: add r8, rdx
14045dd30: mov r15, qword ptr [r8 + 0x10]
14045dd34: mov edx, dword ptr [rdi + 0x5b0]
14045dd3a: mov rdx, qword ptr [rsi + 0x18]
14045dd3e: cmp qword ptr [rdi + 0x218], rdx
14045dd45: jbe 0x14045e127
14045dd4b: mov r9, qword ptr [rdi + 0x210]
14045dd52: mov rdx, qword ptr [r9 + rdx*8]
14045dd56: test rdx, rdx
14045dd59: je 0x14045e127
14045dd5f: mov r10, qword ptr [rdi + 0x1e0]
14045dd66: not rdx
14045dd69: lea rdx, [rdx + rdx*2]
14045dd6d: shl rdx, 5
14045dd71: cmp qword ptr [r10 + rdx + 0x20], 0
14045dd77: je 0x14045e127
14045dd7d: add r10, rdx
14045dd80: mov r13, qword ptr [r10 + 0x10]
14045dd84: mov edx, dword ptr [rdi + 0x5b0]
14045dd8a: mov rdx, qword ptr [rsi + 0x20]
14045dd8e: cmp qword ptr [rdi + 0x218], rdx
14045dd95: jbe 0x14045e137
14045dd9b: mov r9, qword ptr [rdi + 0x210]
14045dda2: mov rdx, qword ptr [r9 + rdx*8]
14045dda6: test rdx, rdx
14045dda9: je 0x14045e137
14045ddaf: mov r14, qword ptr [rdi + 0x1e0]
14045ddb6: not rdx
14045ddb9: lea rdx, [rdx + rdx*2]
14045ddbd: shl rdx, 5
14045ddc1: cmp qword ptr [r14 + rdx + 0x20], 0
14045ddc7: je 0x14045e137
14045ddcd: add r14, rdx
14045ddd0: mov rdx, qword ptr [r14 + 0x10]
14045ddd4: mov qword ptr [rsp + 0x180], rdx
14045dddc: mov edx, dword ptr [rdi + 0x5b0]
14045dde2: mov rdx, qword ptr [rsi + 0x28]
14045dde6: cmp qword ptr [rdi + 0x218], rdx
14045dded: jbe 0x14045e155
14045ddf3: mov r9, qword ptr [rdi + 0x210]
14045ddfa: mov rdx, qword ptr [r9 + rdx*8]
14045ddfe: test rdx, rdx
14045de01: je 0x14045e155
14045de07: mov r12, qword ptr [rdi + 0x1e0]
14045de0e: not rdx
14045de11: lea rdx, [rdx + rdx*2]
14045de15: shl rdx, 5
14045de19: cmp qword ptr [r12 + rdx + 0x20], 0
14045de1f: je 0x14045e155
14045de25: mov qword ptr [rsp + 0x170], r13
14045de2d: mov qword ptr [rsp + 0x178], r15
14045de35: add r12, rdx
14045de38: mov rdx, qword ptr [r12 + 0x10]
14045de3d: mov qword ptr [rsp + 0x168], rdx
14045de45: mov edx, dword ptr [rdi + 0x5b0]
14045de4b: mov rdx, qword ptr [rsi + 0x30]
14045de4f: cmp qword ptr [rdi + 0x218], rdx
14045de56: jbe 0x14045e173
14045de5c: mov r9, qword ptr [rdi + 0x210]
14045de63: mov rdx, qword ptr [r9 + rdx*8]
14045de67: test rdx, rdx
14045de6a: je 0x14045e173
14045de70: mov rbp, qword ptr [rdi + 0x1e0]
14045de77: not rdx
14045de7a: lea rdx, [rdx + rdx*2]
14045de7e: shl rdx, 5
14045de82: cmp qword ptr [rbp + rdx + 0x20], 0
14045de88: je 0x14045e173
14045de8e: mov qword ptr [rsp + 0x160], r11
14045de96: add rbp, rdx
14045de99: mov rdx, qword ptr [rbp + 0x10]
14045de9d: mov r9d, dword ptr [rdi + 0x5b0]
14045dea4: mov r11, qword ptr [rsi + 0x38]
14045dea8: mov r9, qword ptr [rdi + 0x218]
14045deaf: cmp r9, r11
14045deb2: jbe 0x14045e1f8
14045deb8: mov r15, qword ptr [rdi + 0x210]
14045debf: mov r13, qword ptr [r15 + r11*8]
14045dec3: test r13, r13
14045dec6: je 0x14045e1f8
14045decc: mov r11, qword ptr [rdi + 0x1e0]
14045ded3: not r13
14045ded6: lea rdi, [r13*2]
14045dede: add rdi, r13
14045dee1: shl rdi, 5
14045dee5: cmp qword ptr [r11 + rdi + 0x20], 0
14045deeb: je 0x14045e1f8
14045def1: mov r13, qword ptr [rsi + 0x48]
14045def5: cmp r9, r13
14045def8: jbe 0x14045e216
14045defe: mov r9, qword ptr [r15 + r13*8]
14045df02: test r9, r9
14045df05: je 0x14045e216
14045df0b: not r9
14045df0e: lea r9, [r9 + r9*2]
14045df12: shl r9, 5
14045df16: cmp qword ptr [r11 + r9 + 0x20], 0
14045df1c: je 0x14045e216
14045df22: add rdi, r11
14045df25: add r9, r11
14045df28: mov r11, qword ptr [rdi + 0x10]
14045df2c: mov r15, qword ptr [r9 + 0x10]
14045df30: mov r13, qword ptr [rsp + 0x160]
14045df38: mov qword ptr [rsp + 0x58], r13
14045df3d: lea r13, [rcx + 0x58]
14045df41: mov qword ptr [rsp + 0x60], r13
14045df46: add rcx, 0x5c
14045df4a: mov qword ptr [rsp + 0x68], rcx
14045df4f: mov dword ptr [rsp + 0x70], eax
14045df53: mov dword ptr [rsp + 0x74], ebx
14045df57: mov rcx, qword ptr [rsp + 0x178]
14045df5f: mov qword ptr [rsp + 0x78], rcx
14045df64: lea rcx, [r8 + 0x58]
14045df68: mov qword ptr [rsp + 0x80], rcx
14045df70: add r8, 0x5c
14045df74: mov qword ptr [rsp + 0x88], r8
14045df7c: mov dword ptr [rsp + 0x90], eax
14045df83: mov dword ptr [rsp + 0x94], ebx
14045df8a: mov rcx, qword ptr [rsp + 0x170]
14045df92: mov qword ptr [rsp + 0x98], rcx
14045df9a: lea rcx, [r10 + 0x58]
14045df9e: mov qword ptr [rsp + 0xa0], rcx
14045dfa6: add r10, 0x5c
14045dfaa: mov qword ptr [rsp + 0xa8], r10
14045dfb2: mov dword ptr [rsp + 0xb0], eax
14045dfb9: mov dword ptr [rsp + 0xb4], ebx
14045dfc0: mov rcx, qword ptr [rsp + 0x180]
14045dfc8: mov qword ptr [rsp + 0xb8], rcx
14045dfd0: lea rcx, [r14 + 0x58]
14045dfd4: mov qword ptr [rsp + 0xc0], rcx
14045dfdc: add r14, 0x5c
14045dfe0: mov qword ptr [rsp + 0xc8], r14
14045dfe8: mov dword ptr [rsp + 0xd0], eax
14045dfef: mov dword ptr [rsp + 0xd4], ebx
14045dff6: mov rcx, qword ptr [rsp + 0x168]
14045dffe: mov qword ptr [rsp + 0xd8], rcx
14045e006: lea rcx, [r12 + 0x58]
14045e00b: mov qword ptr [rsp + 0xe0], rcx
14045e013: add r12, 0x5c
14045e017: mov qword ptr [rsp + 0xe8], r12
14045e01f: mov dword ptr [rsp + 0xf0], eax
14045e026: mov dword ptr [rsp + 0xf4], ebx
14045e02d: mov qword ptr [rsp + 0xf8], rdx
14045e035: lea rcx, [rbp + 0x5c]
14045e039: add rbp, 0x58
14045e03d: mov qword ptr [rsp + 0x100], rbp
14045e045: mov qword ptr [rsp + 0x108], rcx
14045e04d: mov dword ptr [rsp + 0x110], eax
14045e054: mov dword ptr [rsp + 0x114], ebx
14045e05b: mov qword ptr [rsp + 0x118], r11
14045e063: lea rcx, [rdi + 0x5c]
14045e067: add rdi, 0x58
14045e06b: mov qword ptr [rsp + 0x120], rdi
14045e073: mov qword ptr [rsp + 0x128], rcx
14045e07b: lea rcx, [rsi + 0x40]
14045e07f: mov dword ptr [rsp + 0x130], eax
14045e086: mov dword ptr [rsp + 0x134], ebx
14045e08d: mov qword ptr [rsp + 0x138], rcx
14045e095: mov qword ptr [rsp + 0x140], r15
14045e09d: lea rcx, [r9 + 0x58]
14045e0a1: mov qword ptr [rsp + 0x148], rcx
14045e0a9: add r9, 0x5c
14045e0ad: mov qword ptr [rsp + 0x150], r9
14045e0b5: lea rdx, [rsp + 0x118]
14045e0bd: mov dword ptr [rsp + 0x158], eax
14045e0c4: lea r8, [rsp + 0x138]
14045e0cc: mov dword ptr [rsp + 0x15c], ebx
14045e0d3: lea rcx, [rsp + 0x58]
14045e0d8: call 0x140957bf0
14045e0dd: mov dword ptr [rsi + 0x188], ebx
14045e0e3: xor eax, eax
14045e0e5: add rsp, 0x188
14045e0ec: pop rbx
14045e0ed: pop rbp
14045e0ee: pop rdi
14045e0ef: pop rsi
14045e0f0: pop r12
14045e0f2: pop r13
14045e0f4: pop r14
14045e0f6: pop r15
14045e0f8: ret 
14045e0f9: add rsi, 0xf0
14045e100: lea rax, [rip + 0x257cd49]
14045e107: mov qword ptr [rsp + 0x28], rax
14045e10c: mov qword ptr [rsp + 0x30], 0x36
14045e115: jmp 0x14045e18f
14045e117: add rsi, 0xf0
14045e11e: lea rax, [rip + 0x257c4e0]
14045e125: jmp 0x14045e145
14045e127: add rsi, 0xf0
14045e12e: lea rax, [rip + 0x257ba7c]
14045e135: jmp 0x14045e145
14045e137: add rsi, 0xf0
14045e13e: lea rax, [rip + 0x257d8c1]
14045e145: mov qword ptr [rsp + 0x28], rax
14045e14a: mov qword ptr [rsp + 0x30], 0x3e
14045e153: jmp 0x14045e18f
14045e155: add rsi, 0xf0
14045e15c: lea rax, [rip + 0x257a21f]
14045e163: mov qword ptr [rsp + 0x28], rax
14045e168: mov qword ptr [rsp + 0x30], 0x37
14045e171: jmp 0x14045e18f
14045e173: add rsi, 0xf0
14045e17a: lea rax, [rip + 0x257d8c3]
14045e181: mov qword ptr [rsp + 0x28], rax
14045e186: mov qword ptr [rsp + 0x30], 0x38
14045e18f: mov qword ptr [rsp + 0x38], rsi
14045e194: lea rax, [rip - 0x4226cb]
14045e19b: mov qword ptr [rsp + 0x40], rax
14045e1a0: lea rax, [rsp + 0x28]
14045e1a5: mov qword ptr [rsp + 0x48], rax
14045e1aa: lea rax, [rip - 0x354321]
14045e1b1: mov qword ptr [rsp + 0x50], rax
14045e1b6: lea rax, [rip + 0x257818b]
14045e1bd: mov qword ptr [rsp + 0x58], rax
14045e1c2: mov qword ptr [rsp + 0x60], 2
14045e1cb: mov qword ptr [rsp + 0x78], 0
14045e1d4: lea rax, [rsp + 0x38]
14045e1d9: mov qword ptr [rsp + 0x68], rax
14045e1de: mov qword ptr [rsp + 0x70], 2
14045e1e7: lea rdx, [rip + 0x257ba02]
14045e1ee: lea rcx, [rsp + 0x58]
14045e1f3: call 0x1428d9390
14045e1f8: add rsi, 0xf0
14045e1ff: lea rax, [rip + 0x25782c8]
14045e206: mov qword ptr [rsp + 0x28], rax
14045e20b: mov qword ptr [rsp + 0x30], 0x3d
14045e214: jmp 0x14045e232
14045e216: add rsi, 0xf0
14045e21d: lea rax, [rip + 0x257abef]
14045e224: mov qword ptr [rsp + 0x28], rax
14045e229: mov qword ptr [rsp + 0x30], 0x71
14045e232: mov qword ptr [rsp + 0x38], rsi
14045e237: lea rax, [rip - 0x42276e]
14045e23e: mov qword ptr [rsp + 0x40], rax
14045e243: lea rax, [rsp + 0x28]
14045e248: mov qword ptr [rsp + 0x48], rax
14045e24d: lea rax, [rip - 0x3543c4]
14045e254: mov qword ptr [rsp + 0x50], rax
14045e259: lea rax, [rip + 0x25780e8]
14045e260: mov qword ptr [rsp + 0x58], rax
14045e265: mov qword ptr [rsp + 0x60], 2
14045e26e: mov qword ptr [rsp + 0x78], 0
14045e277: lea rax, [rsp + 0x38]
14045e27c: mov qword ptr [rsp + 0x68], rax
14045e281: mov qword ptr [rsp + 0x70], 2
14045e28a: lea rdx, [rip + 0x257813f]
14045e291: lea rcx, [rsp + 0x58]
14045e296: call 0x1428d9390
14045e29b: lea rcx, [rip + 0x257dd6e]
14045e2a2: lea r8, [rip + 0x257de2f]
14045e2a9: mov edx, 0x59
14045e2ae: call 0x1428d9330
14045e2b3: int3 
